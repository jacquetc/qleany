# Generated Infrastructure

This document details the infrastructure Qleany generates for each target language. It's reference material — read it when you need to understand, extend, or debug the generated code, not as a getting-started guide.

## C++/Qt Infrastructure

### Database Layer

**DbContext / DbSubContext**: Connection pool with scoped transactions. Each unit of work owns a `DbSubContext` providing `beginTransaction`, `commit`, `rollback`, and savepoint support.

```cpp
// Usage in a use case
auto subContext = m_dbContext->createSubContext();
subContext->beginTransaction();
try {
    // ... operations ...
    subContext->commit();
} catch (...) {
    subContext->rollback();
    throw;
}
```

**Repository Factory**: Creates repositories bound to a specific `DbSubContext` and `EventRegistry`. Returns owned instances (`std::unique_ptr`) — no cross-thread sharing.

```cpp
auto repo = RepositoryFactory::createWorkRepository(m_dbSubContext, m_eventRegistry);
auto works = repo->get(QList<int>{workId});
```

**Table Cache / Junction Cache**: Thread-safe, time-expiring (30 minutes), invalidated at write time. Improves performance for repeated queries within a session.

### SQLite Configuration

SQLite with WAL mode, optimized for desktop writing applications:

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
QCoro::Task<QList<WorkDto>> WorkController::update(const QList<WorkDto> &works)
{
    // Create use case owned by the command
    std::unique_ptr<IWorkUnitOfWork> uow = 
        std::make_unique<WorkUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<UpdateWorkUseCase>(std::move(uow));

    // Create command with execute/undo/redo lambdas
    auto command = std::make_shared<UndoRedoCommand>("Update Works Command"_L1);
    QList<WorkDto> result;

    std::weak_ptr<UpdateWorkUseCase> weakUseCase = useCase;

    command->setExecuteFunction([weakUseCase, works, &result](auto &) {
        if (auto uc = weakUseCase.lock())
            result = uc->execute(works);
    });

    command->setUndoFunction([weakUseCase]() -> Result<void> {
        if (auto uc = weakUseCase.lock())
            return uc->undo();
        return Result<void>("UseCase no longer available"_L1);
    });

    command->setRedoFunction([weakUseCase]() -> Result<void> {
        if (auto uc = weakUseCase.lock())
            return uc->redo();
        return Result<void>("UseCase no longer available"_L1);
    });

    // Store useCase in command to maintain ownership
    command->setProperty("useCase", QVariant::fromValue(useCase));

    // Execute asynchronously with timeout
    std::optional<bool> success = 
        co_await m_undoRedoSystem->executeCommandAsync(command, 500, "work_update"_L1);

    if (!success.has_value() || !success.value())
        co_return QList<WorkDto>();

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
- Scoped stacks (per-document undo)
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
    fn get(&self, id: &EntityId) -> Result<Option<Workspace>, Error>;
    fn update(&mut self, entity: &Workspace) -> Result<Workspace, Error>;
    fn delete(&mut self, id: &EntityId) -> Result<(), Error>;
    // ... multi variants and relationship methods
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
let manager = LongOperationManager::new();
let handle = manager.spawn(
    "Importing inventory",
    |progress| {
        for (i, line) in lines.iter().enumerate() {
            progress.set_percent((i * 100) / lines.len());
            // ... process line ...
        }
        Ok(ImportResult { count: lines.len() })
    }
);

// In UI
handle.on_progress(|p| update_progress_bar(p));
let result = handle.await?;
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

Unlike C++/Qt's async controller layer, Rust uses fully synchronous execution throughout, which works well for CLI and desktop applications where blocking is acceptable.

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

// Publishing (in repository)
event_hub.send_event(Event {
    origin: Origin::DirectAccess(DirectAccessEntity::Workspace(EntityEvent::Updated)),
    ids: vec![entity.id.clone()],
    data: None,
});
```

Relationship changes include field information in the `data` field for targeted UI updates.

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

### Unit of Work

Each use case receives a unit of work factory that creates transaction-scoped operations:

**Rust:**
```rust
pub trait WorkspaceUnitOfWorkFactoryTrait {
    fn create(&self) -> Box<dyn WorkspaceUnitOfWorkTrait>;
}

// Use case creates UoW per operation
let mut uow = self.uow_factory.create();
uow.begin_transaction()?;
let entity = uow.update_workspace(&dto.into())?;
uow.commit()?;
```

**C++/Qt:**
```cpp
// Unit of work encapsulates repository access within a transaction
class WorkUnitOfWork {
    void beginTransaction();
    Work createWork(const CreateWorkDto& dto);
    Work updateWork(const WorkDto& dto);
    void commit();
    void rollback();
};
```

This keeps transaction boundaries explicit and testable, while the factory pattern enables easy mocking for unit tests.

### DTO Mapping

DTOs are generated for all boundary crossings:

```
Controller ←→ CreateCarDto ←→ UseCase ←→ Car (Entity) ←→ Repository
```

The separation ensures:
- Controllers don't expose entity internals
- Use cases receive validated, typed input
- Entities remain persistence-agnostic

---

## File Organization

### C++/Qt Output

```
src/
├── common/
│   ├── database/
│   │   ├── db_context.h/.cpp
│   │   ├── db_sub_context.h/.cpp
│   │   └── repository_factory.h/.cpp
│   ├── entities/
│   │   ├── work.h/.cpp
│   │   └── ...
│   ├── direct_access/
│   │   └── {entity}/
│   │       ├── i_{entity}_repository.h   # Interface with relationship enum
│   │       ├── {entity}_repository.h/.cpp
│   │       ├── i_{entity}_table.h        # Table interface
│   │       ├── {entity}_table.h/.cpp
│   │       └── {entity}_events.h/.cpp
│   └── undo_redo/
│       ├── undo_redo_system.h/.cpp
│       └── command.h
├── direct_access/
│   └── {entity}/
│       ├── {entity}_controller.h/.cpp
│       ├── dtos.h
│       ├── unit_of_work.h/.cpp
│       └── use_cases/
└── {feature}/
    ├── {feature}_controller.h/.cpp
    └── use_cases/
```

### Rust Output

Qleany uses the modern Rust module naming convention (no `mod.rs` files):

```
src/
├── common.rs                    # Module declaration
├── common/
│   ├── database.rs
│   ├── database/
│   │   ├── db_context.rs
│   │   ├── transactions.rs
│   │   └── tables.rs
│   ├── entities.rs              # All entity exports
│   ├── entities/
│   │   └── workspace.rs
│   ├── event.rs                 # Event, Origin, EntityEvent enums
│   ├── types.rs                 # EntityId and other shared types
│   ├── direct_access.rs         # Module exports
│   ├── direct_access/
│   │   ├── repository_factory.rs
│   │   └── {entity}/
│   │       └── repository.rs    # Table traits + Repository wrapper
│   └── undo_redo.rs             # UndoRedoCommand trait, UndoRedoManager
├── direct_access.rs
├── direct_access/
│   ├── lib.rs                   # Feature exports
│   ├── {entity}.rs
│   └── {entity}/
│       ├── controller.rs        # Free functions: create, get, update, remove, etc.
│       ├── dtos.rs              # CreateDto, Dto, RelationshipDto
│       ├── units_of_work.rs     # UnitOfWorkFactory, UnitOfWorkTrait
│       └── use_cases/
│           ├── create_uc.rs
│           ├── get_uc.rs
│           ├── update_uc.rs     # Implements UndoRedoCommand
│           └── remove_uc.rs
├── {feature}.rs
└── {feature}/
    ├── controller.rs
    ├── dtos.rs
    ├── units_of_work.rs
    └── use_cases/
```

This follows Rust's recommended practice since the 2018 edition, making navigation clearer by naming modules after their folders.
