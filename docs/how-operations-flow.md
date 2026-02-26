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
            CalendarRepository::update(event_hub, &entity)
                CalendarTable::update(&entity)
                event_hub.send_event(Calendar(Updated))
                    event pushed to flume channel, queued in EventHub

      3e. uow.commit()
            redb transaction committed

── Return to controller ──────────────────────────────────────────

4.  undo_redo_manager.add_command_to_stack(Box::new(uc), stack_id)
5.  returns CalendarDto
```

In Rust, events flow through a central `EventHub` using flume channels. The event loop runs on a dedicated thread, receiving events and pushing them into a shared `Queue` (an `Arc<Mutex<Vec<Event>>>`). The UI polls this queue to pick up changes.

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

- **Entity events**: Each entity has a dedicated `[Entity]Events` class (e.g., `CalendarEvents`) with signals: `created(QList<int>)`, `updated(QList<int>)`, `removed(QList<int>)`, and `relationshipChanged(int, RelationshipField, QList<int>)`. These are centralized in `EventRegistry`.

- **Feature events**: Each feature group has a `[Feature]Events` class (e.g., `CalendarManagementEvents`) with a signal per use case. Centralized in `FeatureEventRegistry`.

Events are **deferred** via the `SignalBuffer`. The flow:

1. Repository calls `emitUpdated(ids)`.
2. `SignalBuffer::push()` captures the callback (it's a lambda wrapping `QMetaObject::invokeMethod` with `Qt::QueuedConnection`).
3. On `commit()`, `SignalBuffer::flush()` executes all callbacks.
4. On `rollback()`, `SignalBuffer::discard()` drops them all.

The `Qt::QueuedConnection` ensures signals are delivered on the events object's thread (typically the main thread), not the worker thread where the command executed. Cross-thread signal delivery is handled by Qt's meta-object system, with metatypes registered at construction time.

### Rust

All events (entity, feature, undo/redo, long operation) flow through a single `EventHub`:

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

The `EventHub` uses a flume channel internally. Events are sent from any thread via `send_event()`, received by a dedicated event loop thread, and pushed into a shared `Queue` (`Arc<Mutex<Vec<Event>>>`). The UI layer polls this queue to process events.

Events are **not deferred** in Rust. Unlike C++/Qt's `SignalBuffer`, the repository sends the event into the `EventHub` channel immediately, even before the redb transaction commits. If the transaction later fails and rolls back, the events are already in the queue, referencing data that no longer exists. In practice, the UI handles "entity not found" gracefully, but it's not as clean as C++/Qt's approach where events are strictly withheld until commit. Deferred events for Rust are planned but not yet implemented.

## Transaction Boundaries

Both targets use transactions to guarantee atomicity:

- **C++/Qt**: SQLite transactions with WAL mode. The `DbSubContext` manages `BEGIN/COMMIT/ROLLBACK`. Savepoints are available in the API just in case the developer really needs them, but Qleany doesn't use them internally (see below).

- **Rust**: redb write transactions. The `Transaction` struct wraps redb's `WriteTransaction` and provides `begin_write_transaction()`, `commit()`, `rollback()`. Savepoints exist in the API (`create_savepoint()` / `restore_to_savepoint()`), but same story: Qleany doesn't rely on them.

In both cases, the unit of work owns the transaction lifecycle. `beginTransaction()` opens it, `commit()` closes it successfully (and flushes events in C++/Qt), `rollback()` aborts it (and discards events in C++/Qt).

### Why Snapshots, Not Savepoints

Early versions of Qleany used database savepoints to handle undo for destructive operations. This turned out to be a trap: savepoints restore *everything*, including non-undoable data. Now, `create`, `createOrphans`, `remove`, and `setRelationshipIds` use **cascading table-level snapshots** that only touch the affected entities. This is currently **C++/Qt only**. Rust use cases store entity state in their own undo/redo stacks for now, with cascading snapshots coming soon.

For the full story, see the [Undo-Redo Architecture](undo-redo-architecture.md#savepoints) documentation.

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
│   │   │   ├── calendar_repository.rs  # CRUD + event emission via EventHub
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
| Event deferral | SignalBuffer (explicit buffer/flush/discard) | Not yet deferred (coming soon) |
| Event registries | Separate per entity + separate per feature | Single EventHub with Origin enum |
| Long operations | QtConcurrent::run | std::thread::spawn |
| Database | SQLite (WAL mode) | redb (embedded key-value) |
| Cascade snapshots | Yes (table-level snapshot/restore) | Coming soon (per-field undo stacks for now) |
| UoW boilerplate | CRTP templates (entities), macros (feature use cases) | Procedural macros (`#[macros::uow_action]`) |
| Read-only queries | Through undo/redo system (serialization) | Direct call with read-only UoW |