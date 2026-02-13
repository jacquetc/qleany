# Generated Infrastructure - C++/Qt

This document details the infrastructure Qleany generates for C++20/Qt6. It's a reference material — read it when you need to understand, extend, or debug the generated code, not as a getting-started guide.

## C++/Qt Infrastructure

### Database Layer

**DbContext / DbSubContext**: Connection pool with scoped transactions. Each unit of work owns a `DbSubContext` providing `beginTransaction`, `commit`, `rollback`, and savepoint support. Savepoint is present just in case the developer really needs it, Qleany doesn't use it internally (see the undo redo documentation if you want to know why)

**Repository Factory**: Creates repositories bound to a specific `DbSubContext` and `EventRegistry`. Returns owned instances (`std::unique_ptr`) — no cross-thread sharing. Every command/query holds its own `DbSubContext`, table, and repository instances.

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

These are tuned for a typical desktop workload: frequent reads with occasional write bursts, and a single user.

**WAL mode** lets reads and writes happen concurrently instead of blocking each other. This keeps the UI responsive while background operations write to the database. The tradeoff is two sidecar files (-wal, -shm) next to the database, harmless for a local application.

**NORMAL synchronous** skips fsync() on every commit. A power failure could lose the last transaction, but the database won't corrupt. For a desktop app where users already expect to lose unsaved work after a crash, this is a good trade for faster writes.

**20 MB page cache** (~10× the default) keeps hot pages in memory so navigation stays snappy. Negligible cost on any modern machine.

**256 MB memory-map** lets the OS map the database file into the process address space, bypassing SQLite's I/O layer for reads. Since most desktop databases stay well under this limit, the entire file benefits. Works well on Linux and macOS; functional on Windows, though less battle-tested there.

Together, these give fast, responsive data access while accepting a durability tradeoff that simply doesn't matter for a single-user desktop application.

### Ephemeral Database Pattern

The internal database lives in `/tmp/`, decoupled from user files:

1. **Load**: Transform file → internal database
2. **Work**: All operations against ephemeral database
3. **Save**: Transform internal database → file

This pattern separates the user's file format from internal data structures. Your `.myapp` file can be JSON, XML, SQLite, or any format. The internal database remains consistent.

The user must implement this pattern in dedicated custom use cases.

Note: the SQLite database can be set to exist in memory by using `:memory:` as the filename.

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

What was written above is the "flattened" version of the real code. The actual code below  uses helper functions to make it more readable and less repetitive (in the file `common/controller_command_helpers.h`). Thus, this same code can be shortened:

```cpp

QCoro::Task<QList<WorkDto>> WorkController::update(const QList<WorkDto> &files, int stackId)
{
    auto uow = std::make_unique<WorkUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<UpdateWorkUseCase>(std::move(uow));

    co_return co_await Helpers::executeUndoableCommand<QList<WorkDto>>(
        m_undoRedoSystem, u"Update works Command"_s, std::move(useCase), stackId, kDefaultCommandTimeoutMs, files);
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
    co_return co_await Helpers::executeReadQuery<QList<WorkDto>>(
        m_undoRedoSystem, u"Get works Query"_s, [this, workIds]() -> QList<WorkDto> {
            auto uow = std::make_unique<WorkUnitOfWork>(*m_dbContext, m_eventRegistry);
            auto useCase = std::make_unique<GetWorkUseCase>(std::move(uow));
            return useCase->execute(workIds);
        });
}
```

Features:
- Undo stacks (per-document undo)
- Command grouping (multiple operations as one undo step)
- Timeout handling for long operations

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

// Subscribing (C++):
connect(s_serviceLocator->eventRegistry()->workEvents(), &WorkEvents::updated, this, &Whatever::onWorkUpdated);
```

```qml
// Subscribing (QML)
Connections {
    target: EventRegistry.workEvents()
    function onWorkUpdated(ids) { doSomething(ids) }
}
```

The `EventRegistry`  provides access to all entity events from both C++ and QML.

A similar pattern is used for `FeatureEventRegistry` in `common/features/feature_event_registry.h`. Each feature (= group of use cases) has its own events class.

```cpp
// Subscribing (C++):
connect(s_serviceLocator->eventFeatureRegistry()->handlingAppLifecycleEvents(), &HandlingAppLifecycleEvents::initializeAppSignal, this, &Whatever::onInitializeAppSignal);
```
```qml
// Subscribing (QML)
Connections {
target: EventFeatureRegistry.handlingAppLifecycleEvents()
function onInitializeAppSignal() { doSomething() }
}
```
---

### Repository

Generated repositories are batch-capable interfaces. One repository for each entity type.

| Method                  | Purpose                                            |
|-------------------------|----------------------------------------------------|
| `create(QList<Entity>)` | Insert new entities                                |
| `get(QList<int>)`       | Fetch entities by IDs                              |
| `update(QList<Entity>)` | Update existing entities                           |
| `remove(QList<int>)`    | Delete entities (cascade for strong relationships) |

Relationship-specific methods:

| Method                                                | Purpose                         |
|-------------------------------------------------------|---------------------------------|
| `getRelationshipIds(id, field)`                       | Get related IDs for one entity  |
| `getRelationshipIdsMany(ids, field)`                  | Batch lookup                    |
| `setRelationshipIds(id, field, ids)`                  | Set relationship for one entity |
| `getRelationshipIdsCount(id, field)`                  | Count related items             |
| `getRelationshipIdsInRange(id, field, offset, limit)` | Paginated access                |

C++/Qt offers additional pagination and counting methods for UI scenarios. The generated QAbstractListModels aren't using these yet but can be extended to do so.


### Unit of Work

In C++/Qt, the units of work are helped by macros and inherited classes to generate all the boilerplate for transaction management and repository access. This can be a debatable design choice, since all is already generated by Qleany. The reality is: not all can be generated. The user (developer) has the responsibility to adapt the units of work for each custom use case. The macros are here to ease this task.

**I repeat**: the user is to adapt the macros in custom use cases.

No factory for the unit of work here, the controller creates a new unit of work per use case.

The examples are from Qleany's own code. The C++ parts are generated from the same manifest file.

Here is the unit of work used by all CRUD use cases of the `Workspace` entity. You can see SCUoW::EntityFullImpl
inheritance, which refactors all the boilerplate of all entities units of work into a single base template class (in `common/unit_of_work/uow_ops.h` file).

```cpp
// Unit of work encapsulates repository access 
class WorkspaceUnitOfWork final
    : public SCUoW::UnitOfWorkBase,
      public SCUoW::EntityFullImpl<WorkspaceUnitOfWork, SCE::Workspace, SCDWorkspace::WorkspaceRelationshipField>
{
    friend class SCUoW::EntityFullImpl<WorkspaceUnitOfWork, SCE::Workspace, SCDWorkspace::WorkspaceRelationshipField>;

  public:
    WorkspaceUnitOfWork(SCDatabase::DbContext &dbContext, const QPointer<SCD::EventRegistry> &eventRegistry)
        : UnitOfWorkBase(dbContext, eventRegistry)
    {
    }
    ~WorkspaceUnitOfWork() override = default;

    /// Called by CRTP base to get a repository for Workspace.
    [[nodiscard]] auto makeRepository()
    {
        return SCD::RepositoryFactory::createWorkspaceRepository(m_dbSubContext, m_eventRegistry);
    }
};

// In controller :

QCoro::Task<QList<WorkDto>> WorkController::create(const QList<CreateWorkDto> &works)
{
...
    auto uow = std::make_unique<WorkUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<CreateWorkUseCase>(std::move(uow));
...
}
```

This keeps transaction boundaries explicit and testable.

For the custom use cases, things are done a bit differently to make adaptation easier. We are using dumb variadic macros here:

```cpp


class SaveUnitOfWork : public Common::UnitOfWork::UnitOfWorkBase, public ISaveUnitOfWork
{
  public:
    SaveUnitOfWork(SCDatabase::DbContext &db, QPointer<SCD::EventRegistry> eventRegistry,
                   QPointer<SCF::FeatureEventRegistry> featureEventRegistry)
        : UnitOfWorkBase(db, eventRegistry), m_featureEventRegistry(featureEventRegistry)
    {
    }

    /* TODO: adapt entities to real use :
     * Available Atomic Macros (uow_macros.h — for custom UoWs):
     *   Interface:    UOW_ENTITY_{CREATE,GET,UPDATE,REMOVE,CRUD}(Name)
     *                 UOW_ENTITY_RELATIONSHIPS(Name, Rel)
     *
     * The equivalent macros (with the DECLARE_ prefix) must be set in the use case's unit of work interface file
     * in use_cases/i_save_uow.h
     */
    UOW_ENTITY_UPDATE(Dto);
    UOW_ENTITY_UPDATE(DtoField);
    UOW_ENTITY_UPDATE(Global);
    UOW_ENTITY_UPDATE(Relationship);
    UOW_ENTITY_UPDATE(Root);
    UOW_ENTITY_UPDATE(Entity);
    UOW_ENTITY_UPDATE(Field);
    UOW_ENTITY_UPDATE(Feature);
    UOW_ENTITY_UPDATE(UseCase);

    void publishSaveSignal() override;

  private:
    QPointer<SCF::FeatureEventRegistry> m_featureEventRegistry;
};

inline void SaveUnitOfWork::publishSaveSignal()
{
    m_featureEventRegistry->handlingManifestEvents()->publishSaveSignal();
}

```

The interface in `handling_manifest/use_cases/save_uc/i_save_uow.h`:

```cpp

class ISaveUnitOfWork : public virtual Common::UnitOfWork::ITransactional
{
  public:
    ~ISaveUnitOfWork() override = default;

    /* TODO: adapt entities to real use :
     * Available Atomic Macros (uow_macros.h — for custom UoWs):
     *   Interface:    DECLARE_UOW_ENTITY_{CREATE,GET,UPDATE,REMOVE,CRUD}(Name)
     *                 DECLARE_UOW_ENTITY_RELATIONSHIPS(Name, Rel)
     *
     * The equivalent macros (without the DECLARE_ prefix) must be set in the use case's unit of work file
     * in units_of_work/save_uow.h
     */
    DECLARE_UOW_ENTITY_UPDATE(Dto);
    DECLARE_UOW_ENTITY_UPDATE(DtoField);
    DECLARE_UOW_ENTITY_UPDATE(Global);
    DECLARE_UOW_ENTITY_UPDATE(Relationship);
    DECLARE_UOW_ENTITY_UPDATE(Root);
    DECLARE_UOW_ENTITY_UPDATE(Entity);
    DECLARE_UOW_ENTITY_UPDATE(Field);
    DECLARE_UOW_ENTITY_UPDATE(Feature);
    DECLARE_UOW_ENTITY_UPDATE(UseCase);

    virtual void publishSaveSignal() = 0;
};
```

### DTO Mapping

DTOs are generated for all boundary crossings:

```
Controller ←→ CreateCarDto ←→ UseCase ←→ Car (Entity) ←→ Repository
```

The separation ensures:
- Controllers don't expose entity internals
- You control what data flows in/out of each layer

The DTOs are C++20 struct aggregates that are enhanced by Q_GADGET macro. They are usable in QML.

---

## File Organization

CMakeLists.txt are disseminated across the project and are not shown here.

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
│   ├── direct_access/                     # Holds the repositories and tables
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
    ├── units_of_work/                    
    │   └── {use case}_uow.h                # ← adapt the macros here
    └── use_cases/              
        ├── {use case}_uc/                  # store here the use case's companion modules
        │   └── i_{use case}_uow.h          # ← adapt the macros here too
        └── {use case}_uc.h/.cpp            # ← You implement the logic here
```