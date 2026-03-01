# How Operations Flow

This document outlines the data flow within a Qleany-generated application, detailing how data is processed and transferred between components. Both the **C++/Qt** and **Rust** targets follow the same architecture, with language-appropriate implementations. If you're lost in the generated code, start here.

## The Big Picture

Every operation in a Qleany application, whether creating a `Calendar`, updating a `CalendarEvent`, or running a custom feature use case, follows the same pipeline:

**UI → Controller → Use Case → Unit of Work → Repository → Table → Database**

And on the way back:

**Database → Table → Repository (events queued) → Unit of Work → Use Case (produces return DTO) → Controller → UI receives result. Events are flushed.**

The key invariant: **events are never sent until the transaction commits**. If anything goes wrong, the transaction rolls back, the events are discarded, and the UI never sees a thing. No half-baked state, no confused models, no fun debugging sessions at 2 AM.

## Commands

Commands are operations that modify data. There are two flavors:

- **Undoable commands**: modify entities marked `undoable: true` in the manifest. They are executed through the undo/redo system, which keeps them on a named stack for later undo/redo. Each use case captures enough state to reverse itself.

- **Not-undoable commands**: same machinery, same pipeline, but they live on a dedicated throwaway stack (size 1 in C++/Qt, `stack_id: None` maps to global stack 0 in Rust). After execution, the stack is cleared. You get the transactional safety without the history.

### C++/Qt Command Flow

Let's trace an update of a `Calendar` entity, from button press to UI refresh:

```
── Controller setup ──────────────────────────────────────────────

1.  UI action
2.  CalendarController::update(QList<CalendarDto>)
      2a. creates CalendarUnitOfWork (owns DbSubContext + SignalBuffer)
      2b. creates UpdateCalendarUseCase (owns the UoW)
      2c. wraps the use case in an UndoRedoCommand
      2d. co_awaits UndoRedoSystem::executeCommandAsync()

── Worker thread (inside the undo/redo system) ───────────────────

3.  UseCase::execute(calendars)

      3a. UoW::beginTransaction()
            DbSubContext begins SQLite transaction
            SignalBuffer starts buffering

      3b. UoW::get(ids)
            fetch originals for undo

      3c. UoW::update(entities)
            Repository::update()
                Table::updateMany()
            Repository::emitUpdated(ids)
                SignalBuffer::push(callback)       // queued, not delivered yet

      3d. UoW::commit()
            DbSubContext commits SQLite transaction
            SignalBuffer::flush()                   // NOW the events fire
                CalendarEvents::publishUpdated(ids) via Qt::QueuedConnection

── Return to controller ──────────────────────────────────────────

4.  UseCase returns QList<CalendarDto>
5.  Command pushed onto undo stack
6.  co_return result to UI
```

The use case stores the original entities before updating them. On `undo()`, it replays the originals. On `redo()`, it replays the updated values. Each undo/redo opens its own transaction and flushes its own signal buffer.

The `SignalBuffer` is the mechanism for **deferred events**. It sits between the repository and Qt's signal system. During a transaction, `emitCreated/Updated/Removed` calls don't emit signals directly. Instead, they push callbacks into the buffer. On `commit()`, the buffer flushes all callbacks. On `rollback()`, it discards them. Simple, effective, and prevents the UI from seeing phantom state from a failed transaction.

Commands are **asynchronous** thanks to QCoro coroutines. The controller `co_await`s the undo/redo system, which does the actual work on its thread and signals back when done. The UI thread is never blocked. But coroutines are cooperative, and this matters: if your use case does CPU-intensive work inside `execute()`, the coroutine won't magically make it non-blocking. That's what long operations are for (see below).

### Rust Command Flow

Same architecture, different execution model. Rust is **synchronous**:

```
── Controller setup ──────────────────────────────────────────────

1.  UI action
2.  calendar_controller::update(db_context, event_hub, undo_redo_manager, stack_id, &dto)
      2a. creates CalendarUnitOfWorkFactory
      2b. creates UpdateCalendarUseCase (owns the factory)

── Execution (same thread, synchronous) ──────────────────────────

3.  uc.execute(&dto)

      3a. uow = factory.create()
      3b. uow.begin_transaction()
            redb write transaction begins

      3c. uow.get_calendar(id)
            fetch original for undo

      3d. uow.update_calendar(&entity)
            CalendarRepository::update(event_buffer, &entity)
                CalendarTable::update(&entity)
                event_buffer.push(Calendar(Updated))     // queued, not delivered yet

      3e. uow.commit()
            redb transaction committed
            EventBuffer::flush()                          // NOW the events fire
                event_hub.send_event(Calendar(Updated)) via flume channel

── Return to controller ──────────────────────────────────────────

4.  undo_redo_manager.add_command_to_stack(Box::new(uc), stack_id)
5.  returns CalendarDto
```

In Rust, events are deferred via an `EventBuffer` owned by each write unit of work. Repositories push events into the buffer during a transaction. On `commit()`, the buffer flushes all events to the central `EventHub` (a flume channel). On `rollback()`, the buffer is discarded. The event loop runs on a dedicated thread, receiving events from the hub and pushing them into a shared `Queue` (`Arc<Mutex<Vec<Event>>>`). The UI polls this queue to pick up changes.

The `UndoRedoManager` is simpler than the C++/Qt version: no async, no worker thread. Commands implement the `UndoRedoCommand` trait (`undo()`, `redo()`, `as_any()`), and the manager maintains multiple stacks with `HashMap<u64, StackData>`. Each stack has an undo and redo `Vec`. The manager also supports composite commands for grouping multiple operations as one undoable unit (via `begin_composite()` / `end_composite()`), and command merging for operations like continuous typing.

The key difference: in C++/Qt, the undo/redo system *executes* the command. In Rust, the use case executes first, then the resulting command object is *pushed* to the undo/redo stack. Same result, different choreography.

## Queries

Queries only read data. They never modify state, so they don't need undo/redo history.

### C++/Qt

Queries still go through the undo/redo system, not for undo, but for **serialization**. The system guarantees that queries execute between commands, never concurrently with one. This prevents dirty reads.

```cpp
QCoro::Task<QList<CalendarDto>> CalendarController::get(const QList<int> &calendarIds) const
{
    co_return co_await Helpers::executeReadQuery<QList<CalendarDto>>(
        m_undoRedoSystem,
        u"Get calendars Query"_s,
        [this, calendarIds]() -> QList<CalendarDto> {
            auto uow = std::make_unique<CalendarUnitOfWork>(*m_dbContext, m_eventRegistry);
            auto useCase = std::make_unique<GetCalendarUseCase>(std::move(uow));
            return useCase->execute(calendarIds);
        });
}
```

The query lambda creates its own UoW and use case, executes synchronously inside the undo/redo system's thread, and returns the result. No events, no signal buffer, no undo stack.

### Rust

Queries use a **read-only unit of work** (`CalendarUnitOfWorkRO`) that opens a read transaction on the redb database. No event hub is needed, no undo manager involved.

```rust
pub fn get(db_context: &DbContext, id: &EntityId) -> Result<Option<CalendarDto>> {
    let uow_factory = CalendarUnitOfWorkROFactory::new(db_context);
    let uc = GetCalendarUseCase::new(Box::new(uow_factory));
    uc.execute(id)
}
```

Straightforward. The read transaction provides a consistent snapshot of the data. redb guarantees isolation.

## Feature Use Cases

Feature use cases are the custom business logic defined in the `features:` section of the manifest. They look like entity CRUD use cases, but with one important difference: **each feature use case gets its own unit of work**. Entity CRUD use cases within `direct_access` share a unit of work per entity. Feature use cases don't share. Each one is self-contained with access to whichever repositories it needs.

### C++/Qt

Feature use cases that are not long operations follow the same command or query patterns as entity CRUD. For example, `get_upcoming_reminders` (which is `read_only: true` and not a long operation) executes as a read query through the undo/redo system:

```cpp
QCoro::Task<UpcomingRemindersDto> CalendarManagementController::getUpcomingReminders(
    const GetUpcomingRemindersDto &dto)
{
    co_return co_await Common::ControllerHelpers::executeReadQuery<UpcomingRemindersDto>(
        m_undoRedoSystem,
        u"get_upcoming_reminders Query"_s,
        [this, dto]() -> UpcomingRemindersDto {
            auto uow = std::make_unique<GetUpcomingRemindersUnitOfWork>(
                *m_dbContext, m_eventRegistry, m_featureEventRegistry);
            auto useCase = std::make_shared<GetUpcomingRemindersUseCase>(std::move(uow));
            return useCase->execute(dto);
        });
}
```

Feature use cases also have their own **event registry** (`FeatureEventRegistry` / `CalendarManagementEvents`), separate from the entity event registry. This keeps entity-level events (Calendar created, updated, removed) distinct from feature-level events (GetEventsInRange completed). The UI can subscribe to exactly what it cares about.

### Rust

Same pattern. Non-long-operation feature use cases execute directly and fire an event through the shared `EventHub`:

```rust
pub fn get_upcoming_reminders(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    dto: &GetUpcomingRemindersDto,
) -> Result<UpcomingRemindersDto> {
    let uow_context = GetUpcomingRemindersUnitOfWorkFactory::new(db_context);
    let mut uc = GetUpcomingRemindersUseCase::new(Box::new(uow_context));
    let return_dto = uc.execute(dto)?;
    event_hub.send_event(Event {
        origin: Origin::CalendarManagement(GetUpcomingReminders),
        ids: vec![],
        data: None,
    });
    Ok(return_dto)
}
```

In Rust, there's no separate feature event registry. Entity events and feature events all flow through the same `EventHub` with an `Origin` enum that discriminates between `DirectAccess(Calendar(Updated))` and `CalendarManagement(GetUpcomingReminders)`. One hub, one queue, one subscription point.

## Long Operations

Long operations are those that take a long time to complete. Yes, the name is self-explanatory. I'm proud of this.

Typical examples include big database operations, heavy network requests, file generation (like Qleany's own code generation), or any task where you want a progress bar and a cancel button.

A use case marked `long_operation: true` in the manifest gets a completely different controller API:

```
run_[use_case_name](DtoIn)      → returns an operation ID (string)
get_[use_case_name]_progress(id) → returns progress (percentage + message)
get_[use_case_name]_result(id)   → returns the output DTO
```

To cancel, call `cancel_operation(id)` on the long operation manager.

### C++/Qt

Long operations bypass the coroutine pipeline entirely. The controller creates the use case, hands it to the `LongOperationManager`, which runs it on a background thread via `QtConcurrent::run`:

```cpp
QString CalendarManagementController::getEventsInRange(const GetEventsInRangeDto &dto)
{
    auto uow = std::make_unique<GetEventsInRangeUnitOfWork>(
        *m_dbContext, m_eventRegistry, m_featureEventRegistry);
    auto operation = std::make_shared<GetEventsInRangeUseCase>(std::move(uow), dto);
    return m_longOperationManager->startOperation(std::move(operation));
}
```

The controller returns the operation ID **synchronously** (no `co_await`). The UI then polls `getGetEventsInRangeProgress(operationId)` to update a progress bar, and calls `getGetEventsInRangeResult(operationId)` when done to retrieve the result DTO (deserialized from JSON internally).

The `LongOperationManager` emits Qt signals (`progressChanged`, `operationCompleted`, `operationFailed`, `operationCancelled`) so the UI can also use signal/slot connections instead of polling.

### Rust

Since Rust is synchronous, long operations run on a **spawned thread**. The operation implements the `LongOperation` trait:

```rust
pub trait LongOperation: Send + 'static {
    type Output: Send + Sync + 'static + serde::Serialize;

    fn execute(
        &self,
        progress_callback: Box<dyn Fn(OperationProgress) + Send>,
        cancel_flag: Arc<AtomicBool>,
    ) -> Result<Self::Output>;
}
```

The `LongOperationManager` spawns a thread, passes in a progress callback and a cancel flag, and manages status tracking through `Arc<Mutex<...>>` shared state. The result is serialized to JSON and stored for later retrieval.

Progress events flow through the `EventHub` with `Origin::LongOperation(Progress/Completed/Failed/Cancelled)`, carrying the operation ID and progress data as serialized JSON in the `data` field.

### Scenarios

Long operations are not undoable. What happens around them depends on what they touch:

**The operation modifies undoable entities** (e.g., bulk-importing events into a calendar): clear the impacted undo stacks after the operation completes, or all of them if you're feeling cautious. The entity events fire on success, the UI refreshes, and the user starts with a clean undo history. Trying to interleave a long operation with existing undo history is asking for trouble.

**The operation modifies non-undoable entities** (e.g., updating cache or search indices): nothing special. Entity events fire on success, the UI picks them up.

**The operation only reads entities and produces output** (`read_only: true`): this is the "generate files" pattern. Qleany's own file generation is a long operation that reads entities from the internal database, writes files to disk, and reports progress. It never modifies the database. "Read-only" means read-only *with respect to entities*. It can write files, call APIs, whatever it needs.

**The operation crunches data and returns results for user approval** (`read_only: true`): think "search & replace across 2000 files." The long operation finds all matches and returns a preview. The user reviews, then a second use case applies the accepted changes. That second step can be a regular undoable use case if you want the user to be able to revert it.

## Events

Events are the backbone of UI reactivity. When a `Calendar` is updated, the UI needs to know. Not "eventually," not "when it feels like it," but precisely when the transaction commits and never before.

### C++/Qt

Entity events and feature events live in separate registries:

- **Entity events**: Each entity has a dedicated `[Entity]Events` class (e.g., `CalendarEvents`) with signals: `created(QList<int>)`, `updated(QList<int>)`, `removed(QList<int>)`, and `relationshipChanged(int, RelationshipField, QList<int>)`. These are centralized in `EventRegistry`, which also provides `errorOccurred(commandName, errorMessage)` for command failures.

- **Feature events**: Each feature group has a `[Feature]Events` class (e.g., `CalendarManagementEvents`) with a signal per use case. Centralized in `FeatureEventRegistry`, which also provides `errorOccurred(commandName, errorMessage)`.

Both registries forward their `errorOccurred` signal to `ServiceLocator::errorOccurred`, giving the UI a single subscription point for all command errors.

Events are **deferred** via the `SignalBuffer`. The flow:

1. Repository calls `emitUpdated(ids)`.
2. `SignalBuffer::push()` captures the callback (it's a lambda wrapping `QMetaObject::invokeMethod` with `Qt::QueuedConnection`).
3. On `commit()`, `SignalBuffer::flush()` executes all callbacks.
4. On `rollback()`, `SignalBuffer::discard()` drops them all.

The `Qt::QueuedConnection` ensures signals are delivered on the events object's thread (typically the main thread), not the worker thread where the command executed. Cross-thread signal delivery is handled by Qt's meta-object system, with metatypes registered at construction time.

### Rust

No separate registries here. Entity events, feature events, undo/redo events, long operation events,they all flow through a single `EventHub`:

```rust
pub struct Event {
    pub origin: Origin,      // which subsystem produced this
    pub ids: Vec<EntityId>,  // affected entity IDs
    pub data: Option<String>, // optional JSON payload
}

pub enum Origin {
    DirectAccess(DirectAccessEntity),  // Calendar(Created), Tag(Updated), ...
    UndoRedo(UndoRedoEvent),           // Undone, Redone, ...
    LongOperation(LongOperationEvent), // Started, Progress, Completed, ...
    CalendarManagement(CalendarManagementEvent), // per-feature events
}
```

The `EventHub` uses a flume channel internally. Events are sent from any thread via `send_event()`, received by a dedicated event loop thread, and pushed into a shared `Queue` (`Arc<Mutex<Vec<Event>>>`). The UI polls this queue to pick up changes. One hub, one queue, one subscription point. The `Origin` enum tells you who sent what.

Events are **deferred** via the `EventBuffer`, the Rust equivalent of the C++/Qt `SignalBuffer`. Each write unit of work owns one (wrapped in `RefCell` for single-threaded UoWs, `Mutex` for long-operation UoWs). The flow:

1. Repository calls `event_buffer.push(event)`.
2. The buffer holds it in a `Vec<Event>`. Not delivered yet.
3. On `commit()`, the UoW calls `event_buffer.flush()`, drains all pending events, and sends each one to the `EventHub`.
4. On `rollback()`, the UoW calls `event_buffer.discard()`. Gone. The UI never knows.

```rust
pub struct EventBuffer {
    buffering: bool,
    pending: Vec<Event>,
}
```

Deliberately simple. `begin_buffering()` arms it and clears stale events from a previous cycle. `push()` queues an event (silently dropped if not buffering). `flush()` drains via `std::mem::take()` and hands you back the `Vec`. `discard()` clears everything and stops buffering.

One edge case worth knowing: `restore_to_savepoint()` discards the buffer (the database state it described is gone), then sends a `Reset` event **directly** to the `EventHub`, bypassing the buffer entirely. The UI must refresh immediately, that Reset cannot sit around waiting for a future `commit()`.

Thread safety lives at the UoW level, not the repository level. The repositories just take `&mut EventBuffer`.

## Transaction Boundaries

Both targets use transactions to guarantee atomicity:

- **C++/Qt**: SQLite transactions with WAL mode. The `DbSubContext` manages `BEGIN/COMMIT/ROLLBACK`. Savepoints are available in the API just in case the developer really needs them, but Qleany doesn't use them internally (see below). Snaphots are better.

- **Rust**: redb write transactions. The `Transaction` struct wraps redb's `WriteTransaction` and provides `begin_write_transaction()`, `commit()`, `rollback()`. Savepoints exist in the API (`create_savepoint()` / `restore_to_savepoint()`), but same story: Qleany doesn't rely on them, as snaphots are better.

In both cases, the unit of work owns the transaction lifecycle. `beginTransaction()` opens it (and arms the event buffer), `commit()` closes it successfully (and flushes buffered events), `rollback()` aborts it (and discards buffered events).

### Why Snapshots, Not Savepoints

Early versions of Qleany used database savepoints to handle undo for destructive operations. This turned out to be a trap: savepoints restore *everything*, including non-undoable data. Now, `create`, `createOrphans`, `remove`, and `setRelationshipIds` use **cascading table-level snapshots** that only touch the affected entities.

For the full story, see the [Undo-Redo Architecture](undo-redo-architecture.md#savepoints) documentation.

## Error Control Flow

This section describes what happens when things go wrong: a repository call throws, a transaction fails to commit, an undo operation blows up. Both targets follow the same principle -- **failed operations must leave no observable trace** (no events emitted, no stale undo history, no half-committed data) -- but the mechanics differ.

### C++/Qt

#### Use case level: try/catch + explicit rollback

Every generated use case method (`execute()`, `undo()`, `redo()`) wraps its work in the same pattern:

```cpp
try
{
    if (!m_uow->beginTransaction())
        throw std::runtime_error("Failed to begin transaction");

    // ... repository calls ...

    if (!m_uow->commit())
        throw std::runtime_error("Failed to commit transaction");
}
catch (...)
{
    m_uow->rollback();
    throw;
}
```

`beginTransaction()` and `commit()` return `bool`. A `false` return is promoted to an exception so it enters the `catch(...)` block. In the catch block, `rollback()` calls `SignalBuffer::discard()`, which drops all queued entity events. Then the exception is re-thrown.

On successful `commit()`, the `UnitOfWorkBase` calls `SignalBuffer::flush()`, which delivers all queued events. If `commit()` returns `false`, the UoW itself calls `discard()` before returning. Either way, the invariant holds: **events fire if and only if the transaction commits**.

```cpp
// UnitOfWorkBase (uow_base.h)
bool commit() override
{
    bool ok = m_dbSubContext.commit();
    if (ok)
        m_signalBuffer->flush();     // success: deliver all events
    else
        m_signalBuffer->discard();   // commit failed: drop all events
    return ok;
}
bool rollback() override
{
    m_dbSubContext.rollback();
    m_signalBuffer->discard();       // rollback: drop all events
    return true;
}
```

#### UndoRedoCommand: exceptions become Result values

Use cases execute on a background thread via `QtConcurrent::run`. The `UndoRedoCommand` wraps each call in a try/catch:

```cpp
auto future = QtConcurrent::run([safeThis, executeFunction]() -> Result<void> {
    try
    {
        QPromise<Result<void>> promise;
        executeFunction(promise);     // calls useCase->execute()
        return Result<void>();
    }
    catch (const std::exception &e)
    {
        return Result<void>(QString::fromStdString(e.what()), ErrorCategory::ExecutionError);
    }
    catch (...)
    {
        return Result<void>("Unknown exception"_L1, ErrorCategory::UnknownError);
    }
});
```

The exception thrown by the use case (after it has already rolled back its own transaction) is caught here and converted to a `Result<void>`. When the future completes, `onExecuteFinished()` (or `onUndoFinished()` / `onRedoFinished()`) checks the result and emits `finished(bool success)`. This signal is what the undo/redo stack and the controller coroutine both listen to.

#### Undo/redo stack: failure recovery

The `UndoRedoStack` moves commands between stacks *before* the async operation runs. On failure, `onCommandFinished(false)` restores the stacks:

- **Execute fails:** The command was left at the top of `m_undoStack` (it was already pushed there). On failure, the stack **pops and drops it**. The command is gone, a failed execute should leave no trace.

- **Undo fails:** The command was moved from `m_undoStack` to `m_redoStack` before `asyncUndo()`. On failure, the command is **moved back from redo to undo**, restoring the stack to its pre-undo state. The use case's catch block already rolled back the transaction, so the database is unchanged. The user can retry the undo.

- **Redo fails:** The command was moved from `m_redoStack` to `m_undoStack` before `asyncRedo()`. On failure, the stack **pops and drops it**. Same as execute failure.

```cpp
void UndoRedoStack::onCommandFinished(bool success)
{
    if (!success)
    {
        if (!m_redoStack.isEmpty() && m_redoStack.top() == m_currentCommand)
        {
            // Undo failed: move command back from redo to undo stack
            auto cmd = m_redoStack.pop();
            m_undoStack.push(cmd);
        }
        else if (!m_undoStack.isEmpty() && m_undoStack.top() == m_currentCommand)
        {
            // Execute or redo failed: drop command from undo stack
            m_undoStack.pop();
        }
    }
    m_currentCommand.reset();
    updateState();
    Q_EMIT commandFinished(success);
}
```

#### Controller level: defaults on failure + error signals

The controller coroutine `co_await`s the undo/redo system with a timeout. Each command helper takes an `onError` callback that the controller wires to the appropriate event registry:

```cpp
std::optional<bool> success = co_await undoRedoSystem->executeCommandAsync(
    command, timeoutMs, undoRedoStackId);

if (!success.has_value()) [[unlikely]]        // timeout
{
    QString msg = commandName + " timed out"_L1;
    qWarning() << msg;
    if (onError) onError(commandName, msg);   // signal-based error reporting
    co_return ResultT{};                      // default-constructed result
}
if (!success.value()) [[unlikely]]            // execution failed
{
    QString msg = "Failed to execute "_L1 + commandName;
    qWarning() << msg;
    if (onError) onError(commandName, msg);   // signal-based error reporting
    co_return ResultT{};                      // default-constructed result
}
co_return result;                             // success
```

On timeout or failure, the controller returns a default-constructed result (empty list, default DTO) *and* invokes the `onError` callback. The callback is a lambda that calls `publishError()` on the appropriate event registry (`EventRegistry` for entity controllers, `FeatureEventRegistry` for feature controllers), which emits `errorOccurred(commandName, errorMessage)`. Both registries forward this signal to `ServiceLocator::errorOccurred`, giving the UI a single subscription point for all command errors:

```
Controller onError lambda
    → EventRegistry::publishError() / FeatureEventRegistry::publishError()
        → errorOccurred signal
            → ServiceLocator::errorOccurred signal  (connected in setters)
```

The return value stays simple (default-constructed) so the controller API remains easy to use from QML. The `errorOccurred` signal provides the structured error details for UIs that need to display error messages, show toasts, or log failures.

#### Long operations: failure via signals

Long operations run on a `QtConcurrent::run` thread. If `ILongOperation::execute()` throws, the `QFutureWatcher::finished` handler catches it:

```cpp
try {
    const QJsonObject result = watcher->result();  // re-throws if execute() threw
    m_completedResults.insert(operationId, result);
    Q_EMIT operationCompleted(operationId, result);
}
catch (const std::exception &e) {
    Q_EMIT operationFailed(operationId, QString::fromUtf8(e.what()));
}
```

On failure, `operationFailed(operationId, errorMessage)` is emitted. No result is stored. The controller's `get_*_result()` returns `std::nullopt` in that case. On QML, it will be seen, as an invalid QVariant. The UI must listen to the `operationFailed` signal or poll `getResult()` and handle `nullopt`.

### Rust

#### Use case level: `?` operator + implicit rollback via Drop

Rust use cases use the `?` operator throughout. Any failure causes an immediate `Err` return:

```rust
pub fn execute(&mut self, dto: &CalendarDto) -> Result<CalendarDto> {
    let mut uow = self.uow_factory.create();
    uow.begin_transaction()?;               // fails? Err returned, uow dropped
    if uow.get_calendar(&dto.id)?.is_none() {
        return Err(anyhow!("..."));         // uow dropped without commit
    }
    let old_entity = uow.get_calendar(&dto.id)?.unwrap();
    let entity = uow.update_calendar(&dto.into())?;
    uow.commit()?;                          // fails? Err returned, but transaction
                                            //   was already consumed by commit()

    // only reached on full success:
    self.undo_stack.push_back(old_entity);
    Ok(entity.into())
}
```

There is **no explicit rollback** in the error path. The safety net is redb's transaction semantics: a `WriteTransaction` that is dropped without calling `commit()` automatically aborts. The `EventBuffer` is similarly safe: if it is dropped without `flush()`, the buffered events are simply freed. No events are ever delivered for uncommitted work.

The undo stack push happens **after** `commit()`. If any step before commit fails, the old entity is never stored in the undo stack and is simply dropped with the local variable. This prevents stale entries from accumulating on failed operations.

#### Controller level: `?` propagation

The controller uses the `?` operator to chain the use case execution and the undo/redo registration:

```rust
pub fn update(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    stack_id: Option<u64>,
    entity: &CalendarDto,
) -> Result<CalendarDto> {
    let uow_factory = CalendarUnitOfWorkFactory::new(db_context, event_hub);
    let mut uc = UpdateCalendarUseCase::new(Box::new(uow_factory));
    let result = uc.execute(entity)?;                                    // fails? uc dropped
    undo_redo_manager.add_command_to_stack(Box::new(uc), stack_id)?;     // fails? mutation committed
                                                                         //   but not in undo history
    Ok(result)
}
```

If `execute()` fails, the use case is dropped and never added to the undo stack. If `execute()` succeeds but `add_command_to_stack()` fails (e.g., invalid stack ID), the mutation is committed to the database but the command is not tracked. The caller gets an `Err`, which is misleading since the data change persisted. In practice this edge case does not arise because stack IDs are set up at initialization time.

#### Undo/redo manager: pop-then-try, drop on failure

The `UndoRedoManager` pops the command from the stack *before* running `undo()` or `redo()`. If the operation fails, the command is intentionally dropped:

```rust
pub fn undo(&mut self, stack_id: Option<u64>) -> Result<()> {
    // ...
    if let Some(mut command) = stack.undo_stack.pop() {
        if let Err(e) = command.undo() {
            log::error!("Undo failed, dropping command: {e}");
            // command dropped intentionally — goes out of scope
            return Err(e);
        }
        stack.redo_stack.push(command);   // only on success
    }
    Ok(())
}
```

This is different from C++/Qt, where a failed undo moves the command back to the undo stack for retry. In Rust, the command is gone permanently. The rationale: a failed undo means the use case's `undo()` method failed mid-transaction. redb aborted the transaction on drop, so the database is in the pre-undo state. But the command's internal state (its undo/redo stacks, cached entities) may be inconsistent. Re-attempting could make things worse, so the conservative choice is to discard it.

The `redo()` path is identical: pop, try, drop on failure.

#### Composite commands: no partial rollback

`CompositeCommand::undo()` iterates sub-commands in reverse with `?`:

```rust
fn undo(&mut self) -> Result<()> {
    for command in self.commands.iter_mut().rev() {
        command.undo()?;   // short-circuits on first failure
    }
    Ok(())
}
```

If commands [A, B, C] are being undone in order C, B, A and B fails: C's undo has already committed (each sub-command opens its own transaction). A's undo never runs. The composite is in a **partially undone state**. Since the `UndoRedoManager` then drops the entire composite, all sub-commands and their undo/redo history are lost.

This is a known limitation. Fixing it would require either nested transactions (not supported by redb) or a two-phase protocol where sub-commands speculatively undo and then commit together. In practice, composites group closely related operations (e.g., create entity + set relationship) where failure of one implies the other would also fail.

#### Long operations: status enum + event

When a long operation's `execute()` returns `Err`, the manager sets the status and emits an event:

```rust
match &operation_result {
    Ok(result) => {
        results.insert(id.clone(), serde_json::to_string(result)?);
        OperationStatus::Completed
    }
    Err(e) => OperationStatus::Failed(e.to_string()),
};
// ...
event_hub.send_event(Event {
    origin: Origin::LongOperation(LongOperationEvent::Failed),
    data: Some(json!({"id": id, "error": error_string}).to_string()),
    ..
});
```

On failure, no result is stored. The controller's `get_*_result()` returns `Ok(None)`, which is **ambiguous**: it also returns `Ok(None)` when the operation is still running. Callers must check the operation status separately via `get_operation_status()` to distinguish "not finished yet" from "failed." The error message is available through the status enum and through the event's JSON payload.

### Summary

| Scenario | C++/Qt | Rust |
|----------|--------|------|
| Repository call fails mid-transaction | `catch(...)` calls `rollback()` + `SignalBuffer::discard()` | `?` returns `Err`; UoW dropped; redb aborts on drop; `EventBuffer` freed |
| `beginTransaction()` fails | Throws `std::runtime_error`, caught by same `catch(...)` | `?` returns `Err`; no transaction was opened |
| `commit()` fails | Throws `std::runtime_error`; `UnitOfWorkBase` already discards the signal buffer | `?` returns `Err`; redb transaction was consumed by the failed commit attempt |
| `execute()` fails at controller level | Command dropped from undo stack; default result returned to UI; `errorOccurred` signal emitted via registry → `ServiceLocator` | Use case dropped; never added to undo stack; `Err` propagated |
| Undo fails | Command moved back from redo to undo stack (retryable) | Command popped and dropped permanently |
| Redo fails | Command dropped from undo stack | Command popped and dropped permanently |
| Composite undo partially fails | N/A (composites are Rust-only) | Short-circuits; already-undone sub-commands stay committed; composite dropped |
| Long operation fails | `operationFailed` signal emitted; no result stored | `Failed` status set; `Failed` event emitted; `get_*_result()` returns `None` |

## Where the Code Lives

### C++/Qt (251 files for this manifest)

```
src/
├── direct_access/
│   └── calendar/                    # per-entity package
│       ├── calendar_controller.cpp  # entry point for UI
│       ├── calendar_unit_of_work.h  # UoW with transaction + signal buffer
│       ├── dtos.h                   # CalendarDto, CreateCalendarDto
│       ├── models/                  # reactive QML list models
│       └── use_cases/               # CRUD use cases with undo/redo
├── calendar_management/             # feature package
│   ├── calendar_management_controller.cpp
│   ├── calendar_management_dtos.h
│   ├── units_of_work/               # feature-specific UoWs
│   └── use_cases/                   # feature use cases
└── common/
    ├── direct_access/               # repositories, tables, events per entity
    │   ├── event_registry.h         # centralizes all entity event objects
    │   └── calendar/
    │       ├── calendar_repository.cpp  # CRUD + event emission via SignalBuffer
    │       ├── calendar_events.h        # Qt signals for created/updated/removed
    │       └── calendar_table.cpp       # SQLite operations + cache
    ├── features/
    │   ├── feature_event_registry.h     # centralizes feature event objects
    │   └── calendar_management_events.h # Qt signals per feature use case
    ├── undo_redo/                    # command pattern + async execution
    ├── unit_of_work/                 # base classes, CRTP helpers
    ├── long_operation/               # threaded execution with progress
    ├── signal_buffer.h               # deferred event delivery
    └── database/                     # DbContext, junction tables, caches
```

### Rust (120 files for this manifest)

```
src/
├── direct_access/src/
│   └── calendar/                    # per-entity package
│       ├── calendar_controller.rs   # free functions, entry point
│       ├── dtos.rs                  # CalendarDto, CreateCalendarDto
│       ├── units_of_work.rs         # UoW + UoWRO with redb transactions
│       └── use_cases/               # CRUD use cases with UndoRedoCommand trait
├── calendar_management/src/
│   ├── calendar_management_controller.rs  # feature controller
│   ├── dtos.rs
│   ├── units_of_work/               # feature-specific UoWs
│   └── use_cases/                   # feature use cases
├── common/src/
│   ├── direct_access/               # repositories, tables per entity
│   │   ├── calendar/
│   │   │   ├── calendar_repository.rs  # CRUD + event emission via EventBuffer
│   │   │   └── calendar_table.rs       # redb operations
│   │   └── repository_factory.rs       # creates repositories within transactions
│   ├── event.rs                     # EventHub, Event, Origin enums (all events)
│   ├── undo_redo.rs                 # UndoRedoManager, multi-stack, composites
│   ├── long_operation.rs            # threaded execution with progress
│   └── database/                    # DbContext, transactions
└── macros/src/                      # procedural macros for UoW boilerplate
```

## Summary of Differences

| Aspect | C++/Qt | Rust |
|--------|--------|------|
| Execution model | Async (QCoro coroutines) | Synchronous |
| Command execution | Undo/redo system executes the command | Use case executes, then pushed to stack |
| Event deferral | SignalBuffer (explicit buffer/flush/discard) | EventBuffer (explicit buffer/flush/discard) |
| Event registries | Separate per entity + separate per feature | Single EventHub with Origin enum |
| Long operations | QtConcurrent::run | std::thread::spawn |
| Database | SQLite (WAL mode) | redb (embedded key-value) |
| Cascade snapshots | Yes (table-level snapshot/restore) | Yes (table-level snapshot/restore) |
| UoW boilerplate | CRTP templates (entities), macros (feature use cases) | Procedural macros (`#[macros::uow_action]`) |
| Read-only queries | Through undo/redo system (serialization) | Direct call with read-only UoW |