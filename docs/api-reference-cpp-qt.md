# API Reference - C++/Qt

This document is the API reference for Qleany-generated C++20/Qt6 code. It covers the APIs you interact with as a developer: **Entity Controllers**, **Feature Controllers**, and the **Unit of Work macros** you adapt when implementing custom use cases.

For general architecture and code structure, see [Generated Code - C++/Qt](generated-code-cpp-qt.md).

---

## Entity Controller

**File:** `direct_access/{entity}/{entity}_controller.h/.cpp`

Entity controllers are the public entry point for all CRUD and relationship operations on a single entity type. They are `QObject`-based, async (QCoro coroutines), and integrate with the undo/redo system.

### Construction

```cpp
// Create a controller for Car entities
auto controller = new CarController(parent);

// Optionally bind to a specific undo/redo stack (for per-document undo)
auto controller = new CarController(parent, /*undoRedoStackId=*/ 1);
controller->setUndoRedoStackId(2);  // change later
int stackId = controller->undoRedoStackId();
```

Dependencies (`DbContext`, `EventRegistry`, `UndoRedoSystem`) are resolved automatically from `ServiceLocator` at construction time.

### CRUD Methods

All CRUD methods return `QCoro::Task<T>`.

#### create

```cpp
// Only available if the entity has an owner (defined in the manifest)
QCoro::Task<QList<CarDto>> create(
    const QList<CreateCarDto> &cars,
    int ownerId,
    int index = -1     // insertion position; -1 = append
);
```

Creates entities and attaches them to their owner. For `OneToOne`/`ManyToOne` relationships, existing children are displaced (replaced). For list relationships (`orderedOneToMany`, `oneToMany`, `manyToMany`), new items are appended or inserted at `index`.

#### createOrphans

```cpp
QCoro::Task<QList<CarDto>> createOrphans(const QList<CreateCarDto> &cars);
```

Creates entities without an owner. Useful for root entities or deferred ownership assignment.

#### get

```cpp
QCoro::Task<QList<CarDto>> get(const QList<int> &carIds) const;
```

Fetches entities by their IDs. Returns DTOs in the same order as the input IDs.

#### getAll

```cpp
QCoro::Task<QList<CarDto>> getAll() const;
```

Returns all entities of this type. Use with caution on large tables.

#### update

```cpp
QCoro::Task<QList<CarDto>> update(const QList<UpdateCarDto> &cars);
```

Updates scalar fields only (no relationship changes). Accepts `UpdateCarDto` which contains `id` + scalar fields. If the entity has an `updatedAt` field, it is set to the current UTC time automatically.

#### updateWithRelationships

```cpp
QCoro::Task<QList<CarDto>> updateWithRelationships(const QList<CarDto> &cars);
```

Updates both scalar fields and relationship (junction table) data. Accepts the full `CarDto`. Use this when you need to change relationship fields alongside scalar fields in a single atomic operation.

#### remove

```cpp
QCoro::Task<QList<int>> remove(const QList<int> &carIds);
```

Deletes entities by ID. Strong (owned) children are cascade-deleted. Returns the IDs that were actually removed.

#### getCreateDto (static)

```cpp
static CreateCarDto getCreateDto();
```

Returns a default-constructed creation DTO. Convenience for UI code that needs an empty form.

#### getUpdateDto (static)

```cpp
static UpdateCarDto getUpdateDto();
```

Returns a default-constructed update DTO. Convenience for UI code that needs an empty update form.

#### toUpdateDto (static)

```cpp
static UpdateCarDto toUpdateDto(const CarDto &dto);
```

Converts a full `CarDto` to an `UpdateCarDto`, copying `id` + scalar fields and discarding relationship fields. Useful in QML where you fetch with `get()` and want to pass the result to `update()`:

```qml
controller.get([itemId]).then(function(result) {
    var updateDto = controller.toUpdateDto(result[0]);
    updateDto.title = "new title";
    controller.update([updateDto]);
});
```

`UpdateCarDto` also has an explicit converting constructor from `CarDto` for C++ code:

```cpp
UpdateCarDto updateDto(fullDto); // drops relationship fields
```

### Relationship Methods

Only available if the entity has forward relationships defined in the manifest.

#### getRelationshipIds

```cpp
QCoro::Task<QList<int>> getRelationshipIds(
    int carId,
    CarRelationshipField relationship
) const;
```

Returns the IDs of related entities for a single entity.

#### setRelationshipIds

```cpp
QCoro::Task<void> setRelationshipIds(
    int carId,
    CarRelationshipField relationship,
    const QList<int> &relatedIds
);
```

Replaces the relationship for a single entity. If the entity has an `updatedAt` field, it is touched.

#### getRelationshipIdsMany

```cpp
QCoro::Task<QHash<int, QList<int>>> getRelationshipIdsMany(
    const QList<int> &carIds,
    CarRelationshipField relationship
) const;
```

Batch lookup: returns a map from entity ID to its related IDs.

#### getRelationshipIdsCount

```cpp
QCoro::Task<int> getRelationshipIdsCount(
    int carId,
    CarRelationshipField relationship
) const;
```

Returns the count of related entities without fetching them.

#### getRelationshipIdsInRange

```cpp
QCoro::Task<QList<int>> getRelationshipIdsInRange(
    int carId,
    CarRelationshipField relationship,
    int offset,
    int limit
) const;
```

Paginated access to related entity IDs.

#### moveRelationshipIds

```cpp
QCoro::Task<QList<int>> moveRelationshipIds(
    int carId,
    CarRelationshipField relationship,
    const QList<int> &idsToMove,
    int newIndex     // -1 = append at end
);
```

Moves specific related IDs to a new position within an ordered relationship. Returns the reordered list of IDs. Supports undo/redo.

### Usage Examples

All controller methods return `QCoro::Task<T>`. Use `QCoro::connect()` to handle the result from non-coroutine code (slots, UI handlers), or `.then()` to chain dependent operations.

For more information about QCoro `.then()`, see the [QCoro documentation](https://qcoro.dev/reference/coro/task/#then-continuation) and `connect()` [here](https://qcoro.dev/reference/coro/task/#interfacing-with-synchronous-functions).

For QCoro on QML: [https://qcoro.dev/qml/qmltask/](https://qcoro.dev/qml/qmltask/)

#### From C++ (QCoro::connect)

```cpp
CarController *controller = new CarController(this);

// Create orphans
QCoro::connect(std::move(controller->createOrphans({CarController::getCreateDto()})),
    this, [](auto &&created) {
        qDebug() << "Created" << created.size() << "cars";
    });

// Get by IDs
QCoro::connect(std::move(controller->get({1, 2, 3})),
    this, [](auto &&cars) {
        for (const auto &car : cars)
            qDebug() << car.name;
    });

// Update
CarDto car = /* ... */;
car.name = u"Updated Name"_s;
QCoro::connect(std::move(controller->update({car})),
    this, [](auto &&updated) {
        qDebug() << "Updated:" << updated.first().name;
    });

// Remove
QCoro::connect(std::move(controller->remove({carId})),
    this, [](auto &&removedIds) {
        qDebug() << "Removed" << removedIds.size() << "cars";
    });
```

#### Chaining dependent operations with .then()

```cpp
// Get relationship IDs, then fetch the related entities
auto task = controller->getRelationshipIds(carId, CarRelationshipField::Passengers)
    .then([passengerController](auto &&passengerIds) {
        return passengerController->get(passengerIds);
    });

QCoro::connect(std::move(task), this, [](auto &&passengers) {
    for (const auto &p : passengers)
        qDebug() << p.name;
});
```

#### From QML

Be careful with QML and async: you must use QCoroQMLTask's `then()` to handle results, as QML does not support coroutines directly. This is not a Javascript async function, **you can't chain** several `.then()`. Only one `.then()`, that's all. See [https://qcoro.dev/qml/qmltask/](https://qcoro.dev/qml/qmltask/).

```qml
carController.createOrphans([dto]).then(function(result) {
    console.log("Created:", JSON.stringify(result));
});

carController.get([carId]).then(function(cars) {
    console.log("Fetched:", cars.length, "cars");
});
```

#### From another coroutine (co_await)

```cpp
QCoro::Task<void> MyClass::doWork()
{
    auto cars = co_await controller->getAll(zzz);
    
    // Process cars
    ...
    
    auto updatedCars = co_await controller->update(cars);
}
```

---

## Feature Controller

**File:** `{feature}/{feature}_controller.h/.cpp`

Feature controllers are the entry point for custom use cases grouped by feature. Like entity controllers, they are `QObject`-based and async. The controller is generated; **you implement the use case logic**.

### Construction

Same pattern as entity controllers:

```cpp
auto controller = new HandlingFileController(parent);
controller->setUndoRedoStackId(1);
```

Dependencies (`DbContext`, `EventRegistry`, `FeatureEventRegistry`, `UndoRedoSystem`, and optionally `LongOperationManager`) are resolved from `ServiceLocator`.

### Generated Methods

For each use case defined in the manifest, the controller generates a method. The shape depends on the use case configuration:

#### Standard use case (with input DTO, with output DTO)

```cpp
QCoro::Task<SaveResultDto> save(const SaveDto &saveDto);

// Convenience: get an empty input DTO
static SaveDto getSaveDto();
```

#### Standard use case (no input DTO, with output DTO)

```cpp
QCoro::Task<ExportResultDto> exportData();
```

#### Standard use case (with input DTO, no output DTO)

```cpp
QCoro::Task<bool> importData(const ImportDto &importDto);
```

#### Long operation use case

Long operations run on a background thread with progress tracking. They return synchronously (no `co_await`):

```cpp
// Start the operation, returns an operation ID
QString generateCode(const GenerateCodeDto &generateCodeDto);

// Poll progress
std::optional<Common::LongOperation::OperationProgress> getGenerateCodeProgress(
    const QString &operationId) const;

// Get result (if use case has an output DTO)
std::optional<GenerateCodeResultDto> getGenerateCodeResult(
    const QString &operationId) const;
```

### Usage Examples

#### Standard use case from C++

```cpp
HandlingFileController *controller = new HandlingFileController(this);

QCoro::connect(std::move(controller->save(saveDto)),
    this, [](auto &&result) {
        qDebug() << "Save result:" << result.success;
    });
```

#### Long operation from C++

```cpp
QString opId = controller->generateCode(dto);

// Check progress (e.g., from a timer)
auto progress = controller->getGenerateCodeProgress(opId);
if (progress)
    qDebug() << progress->message << progress->percentage() << "%";

// Get result when done
auto result = controller->getGenerateCodeResult(opId);
```

---

## Custom Unit of Work (Macros)

**Files you edit:**
- Interface: `{feature}/use_cases/{use_case}_uc/i_{use_case}_uow.h`
- Implementation: `{feature}/units_of_work/{use_case}_uow.h`

When Qleany generates a custom feature use case, it scaffolds a UoW interface and implementation with `TODO` comments. Your job is to **adapt the macros** to expose only the entity operations your use case needs.

### How It Works

The generated UoW inherits `UnitOfWorkBase` which provides transaction management (`beginTransaction`, `commit`, `rollback`). You pick which entity operations to expose using matching pairs of macros:

1. **Interface file** (`i_{use_case}_uow.h`): use `DECLARE_UOW_ENTITY_*` macros
2. **Implementation file** (`{use_case}_uow.h`): use the matching `UOW_ENTITY_*` macros

Each macro expands to a method named after the entity. For example, `DECLARE_UOW_ENTITY_UPDATE(Work)` declares `virtual QList<SCE::Work> updateWork(const QList<SCE::Work> &items) = 0`.

### Interface Declaration Macros

Use in `i_{use_case}_uow.h`. All declared methods are pure virtual.

**Individual operations:**

| Macro                                           | Declares method                                                    |
|-------------------------------------------------|--------------------------------------------------------------------|
| `DECLARE_UOW_ENTITY_CREATE(Name)`               | `createName(items, ownerId, index) -> QList<SCE::Name>`           |
| `DECLARE_UOW_ENTITY_CREATE_ORPHANS(Name)`        | `createOrphanName(items) -> QList<SCE::Name>`                     |
| `DECLARE_UOW_ENTITY_GET(Name)`                   | `getName(ids) -> QList<SCE::Name>`                                |
| `DECLARE_UOW_ENTITY_GET_ALL(Name)`               | `getAllName() -> QList<SCE::Name>`                                |
| `DECLARE_UOW_ENTITY_UPDATE(Name)`                | `updateName(items) -> QList<SCE::Name>` (scalar fields only)      |
| `DECLARE_UOW_ENTITY_UPDATE_WITH_RELATIONSHIPS(Name)` | `updateWithRelationshipsName(items) -> QList<SCE::Name>` (scalars + relationships) |
| `DECLARE_UOW_ENTITY_REMOVE(Name)`                | `removeName(ids) -> QList<int>`                                   |
| `DECLARE_UOW_ENTITY_SNAPSHOT(Name)`              | `snapshotName(ids)` + `restoreName(snap)`                         |
| `DECLARE_UOW_ENTITY_GET_REL_FROM_OWNER(Name)`    | `getNameRelationshipsFromOwner(ownerId) -> QList<int>`            |
| `DECLARE_UOW_ENTITY_SET_REL_IN_OWNER(Name)`      | `setNameRelationshipsInOwner(itemIds, ownerId)`                   |

**Composite macros** (shorthand for common combinations):

| Macro                                                   | Includes                                                                                |
|---------------------------------------------------------|-----------------------------------------------------------------------------------------|
| `DECLARE_UOW_ENTITY_CRUD(Name)`                         | CREATE + CREATE_ORPHANS + GET_REL_FROM_OWNER + SET_REL_IN_OWNER + GET + GET_ALL + UPDATE + UPDATE_WITH_RELATIONSHIPS + REMOVE + SNAPSHOT |
| `DECLARE_UOW_ORPHAN_ENTITY_CRUD(Name)`                   | CREATE_ORPHANS + GET + GET_ALL + UPDATE + UPDATE_WITH_RELATIONSHIPS + REMOVE + SNAPSHOT  |
| `DECLARE_UOW_ENTITY_RELATIONSHIPS(Name, RelFieldEnum)`   | getNameRelationship, setNameRelationship, moveNameRelationship, getNameRelationshipMany, getNameRelationshipCount, getNameRelationshipInRange |

### Implementation Macros

Use in `{use_case}_uow.h`. Each macro must match a declaration in the interface.

**Individual operations:**

| Macro                                | Implements               |
|--------------------------------------|--------------------------|
| `UOW_ENTITY_CREATE(Name)`            | `createName()`           |
| `UOW_ENTITY_CREATE_ORPHANS(Name)`    | `createOrphanName()`     |
| `UOW_ENTITY_GET(Name)`               | `getName()`              |
| `UOW_ENTITY_GET_ALL(Name)`           | `getAllName()`           |
| `UOW_ENTITY_UPDATE(Name)`            | `updateName()` (scalar fields only) |
| `UOW_ENTITY_UPDATE_WITH_RELATIONSHIPS(Name)` | `updateWithRelationshipsName()` (scalars + relationships) |
| `UOW_ENTITY_REMOVE(Name)`            | `removeName()`           |
| `UOW_ENTITY_SNAPSHOT(Name)`          | `snapshotName()` + `restoreName()` |
| `UOW_ENTITY_GET_REL_FROM_OWNER(Name)` | `getNameRelationshipsFromOwner()` |
| `UOW_ENTITY_SET_REL_IN_OWNER(Name)`  | `setNameRelationshipsInOwner()` |

**Composite macros:**

| Macro                                          | Includes                                             |
|------------------------------------------------|------------------------------------------------------|
| `UOW_ENTITY_CRUD(Name)`                        | All CRUD + owner relationship + snapshot              |
| `UOW_ORPHAN_ENTITY_CRUD(Name)`                  | All CRUD + snapshot (no owner ops)                   |
| `UOW_ENTITY_RELATIONSHIPS(Name, RelFieldEnum)`  | All relationship operations                          |

All implementation macros internally create a repository via `RepositoryFactory::createNameRepository(m_dbSubContext, m_eventRegistry, m_signalBuffer)`.

### Full Example

Given a "Save" use case in the "HandlingManifest" feature that needs to read and update `Work` and `Setting` entities:

**Interface** (`use_cases/save_uc/i_save_uow.h`):

```cpp
class ISaveUnitOfWork : public virtual Common::UnitOfWork::ITransactional
{
  public:
    ~ISaveUnitOfWork() override = default;

    DECLARE_UOW_ENTITY_GET(Work);
    DECLARE_UOW_ENTITY_UPDATE(Work);
    DECLARE_UOW_ENTITY_GET_ALL(Setting);
    DECLARE_UOW_ENTITY_UPDATE(Setting);
    DECLARE_UOW_ENTITY_RELATIONSHIPS(Work, WorkRelationshipField);

    virtual void publishSaveSignal() = 0;
};
```

**Implementation** (`units_of_work/save_uow.h`):

```cpp
class SaveUnitOfWork : public Common::UnitOfWork::UnitOfWorkBase,
                       public ISaveUnitOfWork
{
  public:
    SaveUnitOfWork(SCDatabase::DbContext &db,
                   QPointer<SCD::EventRegistry> er,
                   QPointer<SCF::FeatureEventRegistry> fer)
        : UnitOfWorkBase(db, er), m_featureEventRegistry(fer) {}

    UOW_ENTITY_GET(Work)
    UOW_ENTITY_UPDATE(Work)
    UOW_ENTITY_GET_ALL(Setting)
    UOW_ENTITY_UPDATE(Setting)
    UOW_ENTITY_RELATIONSHIPS(Work, WorkRelationshipField)

    void publishSaveSignal() override
    {
        m_featureEventRegistry->handlingManifestEvents()->publishSaveSignal();
    }

  private:
    QPointer<SCF::FeatureEventRegistry> m_featureEventRegistry;
};
```

**Use case** (`use_cases/save_uc.cpp`) -- this is where you write your logic:

```cpp
SaveResultDto SaveUseCase::execute(const SaveDto &saveDto) const
{
    try
    {
        if (!m_uow->beginTransaction())
            throw std::runtime_error("Failed to begin transaction");

        // Use the UoW methods you declared:
        auto works = m_uow->getWork({saveDto.workId});
        auto settings = m_uow->getAllSetting();

        // ... your business logic ...

        auto updated = m_uow->updateWork(works);

        if (!m_uow->commit())
            throw std::runtime_error("Failed to commit transaction");
    }
    catch (...)
    {
        m_uow->rollback();
        throw;
    }

    m_uow->publishSaveSignal();

    return SaveResultDto{/* ... */};
}
```

### ITransactional Methods

These are available on every UoW via `UnitOfWorkBase`. You call them in your use case `execute()`:

| Method                  | Purpose                                                                 |
|-------------------------|-------------------------------------------------------------------------|
| `beginTransaction()`    | Start a DB transaction and begin signal buffering                       |
| `commit()`              | Commit; flush buffered signals on success, discard on failure           |
| `rollback()`            | Roll back the transaction and discard buffered signals                  |
| `createSavepoint()`     | Create a named savepoint within the current transaction                 |
| `rollbackToSavepoint()` | Roll back to the last savepoint                                        |
| `releaseSavepoint()`    | Release the last savepoint                                             |

The signal buffering ensures that if a transaction fails, no events are emitted and the UI stays consistent.

Do not use savepoint without understanding the implications: please read [Undo-Redo Architecture # savepoints](undo-redo-architecture.md#savepoints)

### Undoable Custom Use Cases

If a custom use case is marked `undoable: true` in the manifest, the controller calls `executeUndoableCommand` which expects `undo()` and `redo()` methods on the use case. The generated scaffold only has `execute()` — **you must add `undo()` and `redo()` yourself**.

Both methods return `UndoRedo::Result<void>`. A default-constructed `Result<void>` means success; construct with an error message to signal failure.

```cpp
class SaveUseCase
{
  public:
    explicit SaveUseCase(std::unique_ptr<ISaveUnitOfWork> uow);
    [[nodiscard]] SaveResultDto execute(const SaveDto &saveDto) const;

    // Add these for undoable use cases:
    UndoRedo::Result<void> undo();
    UndoRedo::Result<void> redo();

  private:
    std::unique_ptr<ISaveUnitOfWork> m_uow;

    // Store whatever state you need to undo/redo
    // (e.g., snapshots taken during execute)
};
```

```cpp
UndoRedo::Result<void> SaveUseCase::undo()
{
    // TODO: restore previous state (e.g., via m_uow->restoreWork(m_snapshot))
    return {};  // success
}

UndoRedo::Result<void> SaveUseCase::redo()
{
    // TODO: re-apply the operation
    return {};  // success
}
```

The SNAPSHOT macro pair (`DECLARE_UOW_ENTITY_SNAPSHOT` / `UOW_ENTITY_SNAPSHOT`) is typically used to capture entity state during `execute()` and restore it in `undo()`.

### publish*Signal()

Every custom UoW has a `publishSignalName()` method that emits a feature-level event via the `FeatureEventRegistry`. Call it **after** a successful commit so subscribers (UI, other features) are notified. The generated scaffold calls it automatically in the use case template.
