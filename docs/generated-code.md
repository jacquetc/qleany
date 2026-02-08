# Generated Infrastructure

This document details the infrastructure Qleany generates for each target language. It's a reference material — read it when you need to understand, extend, or debug the generated code, not as a getting-started guide.

## C++/Qt Infrastructure

### Database Layer

**DbContext / DbSubContext**: Connection pool with scoped transactions. Each unit of work owns a `DbSubContext` providing `beginTransaction`, `commit`, `rollback`, and savepoint support.

**Repository Factory**: Creates repositories bound to a specific `DbSubContext` and `EventRegistry`. Returns owned instances (`std::unique_ptr`) — no cross-thread sharing.

```cpp
auto repo = RepositoryFactory::createWorkRepository(m_dbSubContext, m_eventRegistry);
auto works = repo->get(QList<int>{workId});
```

**Table Cache / Junction Cache**: Thread-safe, time-expiring (30 minutes), invalidated at write time. Improves performance for repeated queries within a session.

### SQLite Configuration

SQLite with WAL mode, optimized for desktop applications:

```sql
PRAGMA journal_mode=WAL;
PRAGMA synchronous=NORMAL;
PRAGMA cache_size=20000;        -- 20MB
PRAGMA mmap_size=268435456;     -- 256MB
```

### Ephemeral Database Pattern

The internal database lives in `/tmp/`, decoupled from user files:

1. **Load**: Transform file → internal database
2. **Work**: All operations against ephemeral database
3. **Save**: Transform internal database → file

This pattern separates the user's file format from internal data structures. Your `.myapp` file can be JSON, XML, SQLite, or any format — the internal database remains consistent.

### Async Undo/Redo with QCoro

Controllers use C++20 coroutines via QCoro for non-blocking command execution:

```cpp
QCoro::Task<QList<WorkDto>> WorkController::update(const QList<WorkDto> &works, int stackId)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<WorkDto>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IWorkUnitOfWork> uow = std::make_unique<WorkUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<UpdateWorkUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Update Works Command"_L1);
    QList<WorkDto> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<UpdateWorkUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, works, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(works);
        }
    });

    // Prepare lambda for redo - use weak_ptr to avoid circular reference
    command->setRedoFunction([weakUseCase]() -> Common::UndoRedo::Result<void> {
        if (auto useCase = weakUseCase.lock())
        {
            return useCase->redo();
        }
        return Common::UndoRedo::Result<void>("UseCase no longer available"_L1,
                                              Common::UndoRedo::ErrorCategory::ExecutionError);
    });

    // Prepare lambda for undo - use weak_ptr to avoid circular reference
    command->setUndoFunction([weakUseCase]() -> Common::UndoRedo::Result<void> {
        if (auto useCase = weakUseCase.lock())
        {
            return useCase->undo();
        }
        return Common::UndoRedo::Result<void>("UseCase no longer available"_L1,
                                              Common::UndoRedo::ErrorCategory::ExecutionError);
    });

    // Store the useCase in the command to maintain ownership
    // This ensures the useCase stays alive as long as the command exists
    command->setProperty("useCase", QVariant::fromValue(useCase));

    // Execute command asynchronously using QCoro integration
    std::optional<bool> success = co_await m_undoRedoSystem->executeCommandAsync(command, 500, stackId);

    if (!success.has_value())
    {
        qWarning() << "Update work command execution timed out";
        co_return QList<WorkDto>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute update work command";
        co_return QList<WorkDto>();
    }

    co_return result;
}
```

Use cases contain synchronous business logic with state for undo/redo:

```cpp
QList<WorkDto> UpdateWorkUseCase::execute(const QList<WorkDto> &works)
{
    // Store original state for undo
    m_uow->beginTransaction();
    m_originalWorks = DtoMapper::toDtoList(m_uow->getWork(workIds));

    // Perform update
    auto updatedEntities = m_uow->updateWork(DtoMapper::toEntityList(works));
    m_uow->commit();

    m_updatedWorks = DtoMapper::toDtoList(updatedEntities);
    return m_updatedWorks;
}

Result<void> UpdateWorkUseCase::undo()
{
    m_uow->beginTransaction();
    m_uow->updateWork(DtoMapper::toEntityList(m_originalWorks));
    m_uow->commit();
    return {};
}
```

Queries (read-only operations) also execute asynchronously:

```cpp
QCoro::Task<QList<WorkDto>> WorkController::get(const QList<int> &workIds)
{
    auto query = m_undoRedoSystem->createQuery<QList<WorkDto>>("Get Works Query"_L1);
    query->setQueryFunction([this, workIds]() -> QList<WorkDto> {
        auto uow = std::make_unique<WorkUnitOfWork>(*m_dbContext, m_eventRegistry);
        return std::make_unique<GetWorkUseCase>(std::move(uow))->execute(workIds);
    });

    co_return co_await m_undoRedoSystem->executeQueryAsync(query);
}
```

Features:
- Undo stacks (per-document undo)
- Command grouping (multiple operations as one undo step)
- Timeout handling for long operations
- Weak pointer pattern to avoid circular references

### Event Registry

QObject-based event dispatch for reactive updates. Each entity has its own events class:

```cpp
class WorkEvents : public QObject
{
    Q_OBJECT
public:
    explicit WorkEvents(QObject *parent = nullptr) : QObject(parent)
    {
        // Register metatypes for cross-thread signal delivery
        qRegisterMetaType<QList<int>>("QList<int>");
        qRegisterMetaType<WorkRelationshipField>("WorkRelationshipField");
    }

public Q_SLOTS:
    // Invoked from any thread via QMetaObject::invokeMethod
    void publishCreated(const QList<int> &ids) { Q_EMIT created(ids); }
    void publishUpdated(const QList<int> &ids) { Q_EMIT updated(ids); }
    void publishRemoved(const QList<int> &ids) { Q_EMIT removed(ids); }
    void publishRelationshipChanged(int workId, WorkRelationshipField relationship, 
                                    const QList<int> &relatedIds)
    { Q_EMIT relationshipChanged(workId, relationship, relatedIds); }

Q_SIGNALS:
    void created(const QList<int> &ids);
    void updated(const QList<int> &ids);
    void removed(const QList<int> &ids);
    void relationshipChanged(int workId, WorkRelationshipField relationship, 
                             const QList<int> &relatedIds);
};
```

Repositories emit events asynchronously via queued connections to ensure thread safety:

```cpp
// In repository
void WorkRepository::emitUpdated(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishUpdated", 
                              Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

// Subscribing (in model or UI)
connect(m_events, &WorkEvents::updated, this, &WorkListModel::onWorkUpdated);
```

The `EventRegistry` singleton provides access to all entity events from both C++ and QML.

---

## Rust Infrastructure

### redb Backend

Embedded key-value storage with ACID transactions. Qleany generates a trait-based abstraction layer:

```rust
// Table trait (generated) — implemented by redb storage
pub trait WorkspaceTable {
    fn create(&mut self, entity: &Workspace) -> Result<Workspace, Error>;
    fn create_multi(&mut self, entities: &[Workspace]) -> Result<Vec<Workspace>, Error>;
    fn get(&self, id: &EntityId) -> Result<Option<Workspace>, Error>;
    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Workspace>>, Error>;
    fn update(&mut self, entity: &Workspace) -> Result<Workspace, Error>;
    fn update_multi(&mut self, entities: &[Workspace]) -> Result<Vec<Workspace>, Error>;
    fn delete(&mut self, id: &EntityId) -> Result<(), Error>;
    fn delete_multi(&mut self, ids: &[EntityId]) -> Result<(), Error>;

    fn get_relationship(
        &self,
        id: &EntityId,
        field: &WorkspaceRelationshipField,
    ) -> Result<Vec<EntityId>, Error>;
    fn get_relationships_from_right_ids(
        &self,
        field: &WorkspaceRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error>;
    fn set_relationship_multi(
        &mut self,
        field: &WorkspaceRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error>;
    fn set_relationship(
        &mut self,
        id: &EntityId,
        field: &WorkspaceRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error>;
}

// Repository wraps table with event emission
pub struct WorkspaceRepository<'a> {
    redb_table: Box<dyn WorkspaceTable + 'a>,
    transaction: &'a Transaction,
}
```

Read-only operations use a separate `WorkspaceTableRO` trait and `WorkspaceRepositoryRO` struct, enforcing immutability at the type level.

### Long Operation Manager

Threaded execution for heavy tasks:

```rust
pub fn generate_rust_files(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    long_operation_manager: &mut LongOperationManager,
    dto: &GenerateRustFilesDto,
) -> Result<String> {
    let uow_context = GenerateRustFilesUnitOfWorkFactory::new(&db_context);
    let uc = GenerateRustFilesUseCase::new(Box::new(uow_context), dto);
    let operation_id = long_operation_manager.start_operation(uc);
    Ok(operation_id)
}

pub fn get_generate_rust_files_progress(
    long_operation_manager: &LongOperationManager,
    operation_id: &str,
) -> Option<OperationProgress> {
    long_operation_manager.get_operation_progress(operation_id)
}

pub fn get_generate_rust_files_result(
    long_operation_manager: &LongOperationManager,
    operation_id: &str,
) -> Result<Option<GenerateRustFilesReturnDto>> {
    // Get the operation result as a JSON string
    let result_json = long_operation_manager.get_operation_result(operation_id);

    // If there's no result, return None
    if result_json.is_none() {
        return Ok(None);
    }

    // Parse the JSON string into a GenerateRustFilesResultDto
    let result_dto: GenerateRustFilesReturnDto = serde_json::from_str(&result_json.unwrap())?;

    Ok(Some(result_dto))
}
```

Features:
- Progress callbacks with percentage and message
- Cancellation support
- Result or error on completion

### Synchronous Undo/Redo Commands

Rust uses synchronous command execution (unlike C++/Qt's async controller layer). Each use case implements `UndoRedoCommand` and maintains its own undo/redo stacks using `VecDeque`:

```rust
pub struct UpdateWorkspaceUseCase {
    uow_factory: Box<dyn WorkspaceUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Workspace>,
    redo_stack: VecDeque<Workspace>,
}

impl UndoRedoCommand for UpdateWorkspaceUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_entity) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_workspace(&last_entity)?;
            uow.commit()?;
            self.redo_stack.push_back(last_entity);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(entity) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_workspace(&entity)?;
            uow.commit()?;
            self.undo_stack.push_back(entity);
        }
        Ok(())
    }
}
```

Controllers manage the `UndoRedoManager` and optional scoped stacks:

```rust
pub fn update(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    stack_id: Option<u64>,
    entity: &WorkspaceDto,
) -> Result<WorkspaceDto> {
    let uow_factory = WorkspaceUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut uc = UpdateWorkspaceUseCase::new(Box::new(uow_factory));
    let result = uc.execute(entity)?;
    undo_redo_manager.add_command_to_stack(Box::new(uc), stack_id)?;
    Ok(result)
}
```

Unlike C++/Qt's async controller layer, Rust uses fully synchronous execution throughout, which works well for CLI where blocking is acceptable. I choose to avoid async/await complexity here.

### Event Hub

Channel-based event dispatch using a unified `Event` struct:

```rust
// Event structure (generated)
pub struct Event {
    pub origin: Origin,
    pub ids: Vec<EntityId>,
    pub data: Option<String>,
}

pub enum Origin {
    DirectAccess(DirectAccessEntity),
    Feature(FeatureEntity),
}

pub enum DirectAccessEntity {
    Workspace(EntityEvent),
    Entity(EntityEvent),
    // ... other entities
}

pub enum EntityEvent {
    Created,
    Updated,
    Removed,
}
...

// Publishing (from the repositories)
event_hub.send_event(Event {
    origin: Origin::DirectAccess(DirectAccessEntity::Workspace(EntityEvent::Updated)),
    ids: vec![entity.id.clone()],
    data: None,
});
```

---

## Common Patterns

### Repository Pattern

Both languages generate repositories with batch-capable interfaces:

**Rust:**
| Method | Purpose |
|--------|---------|
| `create(entity)` / `create_multi(entities)` | Insert new entities |
| `get(id)` / `get_multi(ids)` | Fetch entities |
| `update(entity)` / `update_multi(entities)` | Update existing entities |
| `delete(id)` / `delete_multi(ids)` | Delete entities (cascade for strong relationships) |

**C++/Qt:**
| Method | Purpose |
|--------|---------|
| `create(QList<Entity>)` | Insert new entities |
| `get(QList<int>)` | Fetch entities by IDs |
| `update(QList<Entity>)` | Update existing entities |
| `remove(QList<int>)` | Delete entities (cascade for strong relationships) |

Relationship-specific methods:

**Rust:**
| Method | Purpose |
|--------|---------|
| `get_relationship(id, field)` | Get related IDs for one entity |
| `get_relationships_from_right_ids(field, ids)` | Reverse lookup |
| `set_relationship(id, field, ids)` | Set relationship for one entity |
| `set_relationship_multi(field, relationships)` | Batch relationship updates |

**C++/Qt:**
| Method | Purpose |
|--------|---------|
| `getRelationshipIds(id, field)` | Get related IDs for one entity |
| `getRelationshipIdsMany(ids, field)` | Batch lookup |
| `setRelationshipIds(id, field, ids)` | Set relationship for one entity |
| `getRelationshipIdsCount(id, field)` | Count related items |
| `getRelationshipIdsInRange(id, field, offset, limit)` | Paginated access |

C++/Qt offers additional pagination and counting methods for UI scenarios. The generated QAbstractListModels aren't using these yet but can be extended to do so.

### Unit of Work

In C++//Qt and Rust, the unit of works are helped by macros to generate all the boilerplate for transaction management and repository access. This can be a debatable design choice, since all is already generated by Qleany. The reality is : not all can be generated. The user (developer) have the responsability to adapt the units of work for each custom use case. The macros are here to ease this task.

I repeat : the user is to adapt the macros in custom use cases.

**Rust:**

Each use case receives a unit of work factory which handles the unit of work creation that allow transaction-scoped operations:

```rust
// In the controller, we create the use case with a factory for the unit of work

pub fn create(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    stack_id: Option<u64>,
    entity: &CreateWorkspaceDto,
) -> Result<WorkspaceDto> {
    let uow_factory = WorkspaceUnitOfWorkFactory::new(db_context, event_hub);
    let mut uc = CreateWorkspaceUseCase::new(Box::new(uow_factory));
    let result = uc.execute(entity.clone())?;
    undo_redo_manager.add_command_to_stack(Box::new(uc), stack_id)?;
    Ok(result)
}

// In the unit of work, you see a bit of macro magic to generate all the boilerplate:

#[macros::uow_action(entity = "Workspace", action = "Create")]
#[macros::uow_action(entity = "Workspace", action = "CreateMulti")]
#[macros::uow_action(entity = "Workspace", action = "Get")]
#[macros::uow_action(entity = "Workspace", action = "GetMulti")]
#[macros::uow_action(entity = "Workspace", action = "Update")]
#[macros::uow_action(entity = "Workspace", action = "UpdateMulti")]
#[macros::uow_action(entity = "Workspace", action = "Delete")]
#[macros::uow_action(entity = "Workspace", action = "DeleteMulti")]
#[macros::uow_action(entity = "Workspace", action = "GetRelationship")]
#[macros::uow_action(entity = "Workspace", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "Workspace", action = "SetRelationship")]
#[macros::uow_action(entity = "Workspace", action = "SetRelationshipMulti")]
impl WorkspaceUnitOfWorkTrait for WorkspaceUnitOfWork {}
```

**C++/Qt:**

No factory for the unit of work here, the controller creates a new unit of work per use case.

Yet 

```cpp
// Unit of work encapsulates repository access 
class GlobalUnitOfWork final : public SCUoW::UnitOfWorkBase, public IGlobalUnitOfWork
{

  public:
    GlobalUnitOfWork(SCDatabase::DbContext &dbContext, const QPointer<SCD::EventRegistry> &eventRegistry)
        : UnitOfWorkBase(dbContext, eventRegistry)
    {
    }
    ~GlobalUnitOfWork() override = default;

    // Transaction interface delegation
    UOW_TRANSACTION_INTERFACE
    // Full CRUD operations for Global
    UOW_ENTITY_CRUD(Global)
};

// In controller :

QCoro::Task<QList<WorkDto>> WorkController::create(const QList<CreateWorkDto> &works)
{
...
    std::unique_ptr<IWorkUnitOfWork> uow = std::make_unique<WorkUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<CreateWorkUseCase>(std::move(uow));
...
}
```

This keeps transaction boundaries explicit and testable, while the factory pattern enables easy mocking for unit tests.

### DTO Mapping

DTOs are generated for all boundary crossings:

```
Controller ←→ CreateCarDto ←→ UseCase ←→ Car (Entity) ←→ Repository
```

The separation ensures:
- Controllers don't expose entity internals
- You control what data flows in/out of each layer

---

## File Organization

### C++/Qt Output

```
src/
├── common/
│   ├── service_locator.h/.cpp
│   ├── database/                           # database infrastructure
│   │   ├── junction_table_ops/...
│   │   ├── db_builder.h
│   │   ├── db_context.h
│   │   └── table_cache.h
│   ├── entities/                      # Generated entities
│   │   ├── my_entity.h
│   │   └── ...
│   ├── unit_of_work/                  # unit of work macros and base class
│   │   ├── unit_of_work.h
│   │   ├── uow_macros.h 
│   │   └── ...
│   ├── features/
│   │   ├── feature_event_registry.h   # Event registry for feature events
│   │   └── ...
│   ├── direct_access/                     # Holds the repositoryies and tables
│   │   ├── repository_factory.h/.cpp
│   │   ├── event_registry.h
│   │   └── {entity}/
│   │       ├── i_{entity}_repository.h   # Interface with relationship enum
│   │       ├── table_definitions.h       #  Table schema definitions
│   │       ├── {entity}_repository.h/.cpp
│   │       ├── {entity}_table.h/.cpp
│   │       └── {entity}_events.h
│   └── undo_redo/ ...                      # undo/redo infrastructure
├── direct_access/                          # Direct access to entity controllers and use cases
│   └── {entity}/
│       ├── {entity}_controller.h/.cpp
│       ├── dtos.h
│       ├── unit_of_work.h
│       └── use_cases/
└── {feature}/                              # Custom controllers and use cases
    ├── {feature}_controller.h/.cpp
    ├── {feature}_dtos.h
    ├── units_of_work/                  # ← adapt the macros here
    │   └── ...
    └── use_cases/                      # ← You implement the logic here
        └── ...
```

### Rust Output

```
Cargo.toml
crates/
├── cli/
│   ├── src/
│   │   ├── main.rs    
│   └── Cargo.toml
├── common/
│   ├── src/
│   │   ├── entities.rs             # Generated entities
│   │   ├── database.rs
│   │   ├── database/
│   │   │   ├── db_context.rs
│   │   │   ├── db_helpers.rs
│   │   │   └── transactions.rs
│   │   ├── direct_access.rs
│   │   ├── direct_access/         # Holds the repository and table implementations for each entity
│   │   │   ├── car.rs
│   │   │   ├── car/
│   │   │   │   ├── car_repository.rs
│   │   │   │   └── car_table.rs
│   │   │   ├── customer.rs
│   │   │   ├── customer/
│   │   │   │   ├── customer_repository.rs
│   │   │   │   └── customer_table.rs
│   │   │   ├── sale.rs
│   │   │   ├── sale/
│   │   │   │   ├── sale_repository.rs
│   │   │   │   └── sale_table.rs
│   │   │   ├── root.rs
│   │   │   ├── root/
│   │   │   │   ├── root_repository.rs
│   │   │   │   └── root_table.rs
│   │   │   ├── repository_factory.rs
│   │   │   └── setup.rs
│   │   ├── event.rs             # event system for reactive updates
│   │   ├── lib.rs
│   │   ├── long_operation.rs    # infrastructure for long operations
│   │   ├── types.rs         
│   │   └── undo_redo.rs        # undo/redo infrastructure
│   └── Cargo.toml
├── direct_access/                   # a direct access point for UI or CLI to interact with entities
│   ├── src/
│   │   ├── car.rs
│   │   ├── car/
│   │   │   ├── car_controller.rs   # Exposes CRUD operations to UI or CLI
│   │   │   ├── dtos.rs
│   │   │   ├── units_of_work.rs
│   │   │   ├── use_cases.rs
│   │   │   └── use_cases/          # The logic here is auto-generated
│   │   │       ├── create_car_uc.rs
│   │   │       ├── get_car_uc.rs
│   │   │       ├── update_car_uc.rs
│   │   │       ├── remove_car_uc.rs
│   │   │       └── ...
│   │   ├── customer.rs
│   │   ├── customer/
│   │   │   └── ...
│   │   ├── sale.rs
│   │   ├── sale/
│   │   │   └── ...
│   │   ├── root.rs
│   │   ├── root/
│   │   │   └── ...
│   │   └── lib.rs
│   └── Cargo.toml
└── inventory_management/
    ├── src/
    │   ├── inventory_management_controller.rs
    │   ├── dtos.rs
    │   ├── units_of_work.rs
    │   ├── units_of_work/          # ← adapt the macros here
    │   │   └── ...
    │   ├── use_cases.rs
    │   ├── use_cases/              # ← You implement the logic here
    │   │   └── ...
    │   └── lib.rs
    └── Cargo.toml

```