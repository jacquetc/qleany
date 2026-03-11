# QML Integration (C++/Qt)

This document covers QML-based frontends: **QtQuick**. They use QML, so the generated models and patterns apply equally to each.

---

Qleany generates reactive models ready for QML binding -- no manual `QAbstractListModel` boilerplate.

## List Models

`{Entity}{Field}ListModel` provides a standard `QAbstractListModel` that:
- Auto-updates when entities change (via EventRegistry subscription)
- Refreshes only affected rows, not the entire model
- Supports inline editing through `setData` with async persistence
- Exposes all target entity fields as roles
- Handles item additions, removals, and reordering
- `undoRedoStackId` to route undo/redo to a specific stack

```qml
import MyApp.Models

ListView {
    model: RootRecentWorksListModel {
        rootId: 1
    }
    delegate: ItemDelegate {
        text: model.title
        subtitle: model.absolutePath
        onClicked: openWork(model.itemId)
    }
}
```

The `{entity}Id` property selects the parent entity whose relationship is displayed. All fields of the target entity are available as roles, plus `itemId` for the primary key (`id` being a reserved word in QML).

### Event subscriptions

The model subscribes to three event sources:

- **Target entity `updated`** -- refreshes only affected rows (field changes on displayed items)
- **Parent entity `updated`** -- detects relationship changes: additions, removals, and reordering. Only fetches new items; existing items are moved in-place.
- **Parent entity `relationshipChanged`** -- handles direct relationship mutations (same add/remove/reorder logic as above)

This means if another part of the application updates a RecentWork's title, the ListView updates automatically. If the Root's recentWorks list changes (item added, removed, or reordered), the model detects the difference and applies minimal changes (no full reset).

### Inline editing

`setData` persists changes asynchronously through the entity controller. After the backend confirms the update, the local row is refreshed with the returned data:

```qml
ListView {
    model: WorkBindersListModel {
        workId: currentWorkId
    }
    delegate: TextField {
        text: model.name
        onEditingFinished: model.name = text
    }
}
```

## Single Entity Models

`Single{Entity}` wraps one entity instance for detail views and editor panels.

Features:
- `itemId` property to select which entity to display
- Auto-fetch on ID change
- Reactive updates when the entity changes elsewhere in the application
- All fields exposed as writable Q_PROPERTY declarations with change signals
- `dirty` tracking -- marks the model as modified when fields change outside of a refresh
- `save()` method to persist local edits via the entity controller
- `loadingStatus` enum: `Unloaded`, `Loading`, `Loaded`, `Error`
- `errorMessage` property for error reporting
- `undoRedoStackId` to route undo/redo to a specific stack
- Auto-clear when the entity is removed

```qml
import MyApp.Singles

SingleBinderItem {
    id: currentItem
    itemId: selectedItemId
}

Column {
    Text { text: currentItem.title }
    Text { text: currentItem.subTitle }
    Text { text: "Children: " + currentItem.binderItems.length }

    TextField {
        text: currentItem.title
        onEditingFinished: {
            currentItem.title = text
            currentItem.save()
        }
    }

    Text {
        visible: currentItem.loadingStatus === SingleBinderItem.Error
        text: currentItem.errorMessage
    }
}
```

The model subscribes to:
- **Entity `updated`** -- if any part of the application modifies this entity, the properties update automatically and QML bindings refresh
- **Entity `removed`** -- clears all fields and resets to `Unloaded`

Note: Since `id` is a reserved word in QML, the property is named `itemId`. It corresponds to the entity's primary key.

## Enabling Model Generation

To generate models for an entity, configure these options in the manifest:

**At entity level:**
```yaml
- name: Work
  inherits_from: EntityBase
  single_model: true    # Generates SingleWork
```

**At field level (for relationship fields):**
```yaml
fields:
  - name: binders
    type: entity
    entity: Binder
    relationship: ordered_one_to_many
    strong: true
    list_model: true                      # Generates WorkBindersListModel
    list_model_displayed_field: name      # Default display role (Qt::DisplayRole)
```

## QML Modules

Generated code is organized into three QML modules:

| Module | Contents |
|---|---|
| `AppName.Controllers` | Entity controllers, feature controllers, EventRegistry, FeatureEventRegistry, UndoRedoController, ServiceLocator |
| `AppName.Models` | List models (`{Entity}{Field}ListModel`) |
| `AppName.Singles` | Single entity models (`Single{Entity}`) |

Import them in QML:
```qml
import MyApp.Controllers
import MyApp.Models
import MyApp.Singles
```

## QML Mocks

Generated JavaScript stubs in `mock_imports/` mirror the real C++ API, enabling UI development without backend compilation.

### Mock module structure

```
mock_imports/
+-- controllers/
|   +-- qmldir                          # AppName.Controllers module
|   +-- QCoroQmlTask.qml               # Promise-like async mock
|   +-- EventRegistry.qml              # Singleton, exposes entityNameEvents()
|   +-- FeatureEventRegistry.qml       # Singleton, exposes featureNameEvents()
|   +-- UndoRedoController.qml         # Singleton, mock undo/redo
|   +-- ServiceLocator.qml             # Singleton, errorOccurred signal
|   +-- RootController.qml             # Entity CRUD (get, create, update, remove)
|   +-- RootEvents.qml                 # Singleton signals: created, updated, removed, relationshipChanged
|   +-- BinderItemController.qml
|   +-- BinderItemEvents.qml
|   +-- WorkManagementController.qml   # Feature controller with use case methods
|   ...
+-- models/
|   +-- qmldir                          # AppName.Models module
|   +-- RootRecentWorksListModel.qml    # ListModel with 5 mock entries
|   ...
+-- singles/
    +-- qmldir                          # AppName.Singles module
    +-- SingleBinderItem.qml            # QtObject with mock properties
    ...
```

### Mock entity controllers

Mock entity controllers provide:
- `get(ids)` -- returns mock DTOs with default field values
- `getCreateDto()` -- returns a template DTO for creation
- `create(dtos)` / `createOrphans(dtos)` -- assigns random IDs, emits `created` event
- `update(dtos)` -- emits `updated` and `allRelationsInvalidated` events
- `remove(ids)` -- emits `removed` event
- `getRelationshipIds(id)` / `setRelationshipIds(id, ids)` / `moveRelationshipIds(id, idsToMove, newIndex)` -- per relationship field

All async methods return `QCoroQmlTask`, a mock Promise-like object that resolves after a configurable delay (default 50ms).

### Mock feature controllers

Mock feature controllers provide:
- `getInputDtoName()` -- returns template input DTO (for use cases with DTO input)
- `useCaseName(dto)` -- returns mock QCoroQmlTask

### Mock list models

Mock list models are QML `ListModel` components with 5 pre-populated entries. Each entry has `itemId` and all target entity fields at default values.

### Mock single entity models

Mock single entity models expose all entity fields as properties, plus:
- `status` (int: 0=Unloaded, 1=Loading, 2=Loaded, 3=Error)
- `errorMessage`, `dirty`, `id`
- `save()` method (logs and resets dirty)

### Build flag

Build with `YOUR_APP_BUILD_WITH_MOCKS` to develop UI without backend compilation:

```cmake
option(YOUR_APP_BUILD_WITH_MOCKS "Build with QML mocks instead of real backend" OFF)
```

UI developers can iterate on screens with mock data. When ready, disable the flag and the real controllers take over with no QML changes required.

The mocks are only for UI development. They don't implement real business logic or data persistence.

## Real Imports

The real C++ import structure uses `QML_FOREIGN` and `QML_NAMED_ELEMENT` macros to expose backend classes to QML without wrapper overhead.

### Structure

```
real_imports/
+-- CMakeLists.txt                                           # Adds subdirectories
+-- controllers/
|   +-- CMakeLists.txt                                       # qt6_add_qml_module (AppName.Controllers)
|   +-- foreign_event_registry.h                             # QML_SINGLETON
|   +-- foreign_feature_event_registry.h                     # QML_SINGLETON
|   +-- foreign_undo_redo_controller.h                       # QML_SINGLETON
|   +-- foreign_service_locator.h                            # QML_SINGLETON
|   +-- foreign_root_controller.h                            # QML_NAMED_ELEMENT(RootController)
|   +-- foreign_binder_item_controller.h
|   +-- foreign_work_management_controller.h                 # Feature controller
|   ...
+-- models/
|   +-- CMakeLists.txt                                       # qt6_add_qml_module (AppName.Models)
|   +-- foreign_root_recent_works_list_model.h               # QML_NAMED_ELEMENT(RootRecentWorksListModel)
|   ...
+-- singles/
    +-- CMakeLists.txt                                       # qt6_add_qml_module (AppName.Singles)
    +-- foreign_single_binder_item.h                         # QML_NAMED_ELEMENT(SingleBinderItem)
    ...
```

### Foreign type wrappers

**Entity controllers** (`ForeignEntityNameController : QObject`) wrap the backend controller and expose:
- `get(ids)`, `create(dtos, ownerId, index)`, `createOrphans(dtos)`, `update(dtos)`, `remove(ids)` -- all return `QCoro::QmlTask`
- `getCreateDto()` -- static, returns template DTO
- `getRelationshipIds(id, field)`, `setRelationshipIds(id, field, ids)`, `moveRelationshipIds(id, field, idsToMove, newIndex)` -- relationship access
- `getRelationshipIdsCount(id, field)`, `getRelationshipIdsInRange(id, field, offset, limit)` -- for paginated relationships
- `undoRedoStackId` property

**Feature controllers** (`ForeignFeatureNameController : QObject`) wrap feature controllers and expose:
- Per use case: `useCaseName(inputDto)` returning `QCoro::QmlTask`
- Long operations: `useCaseName(inputDto)` returns operation ID string, with `getUseCaseNameProgress(opId)`, `hasUseCaseNameResult(opId)`, `getUseCaseNameResult(opId)` for polling
- `getInputDtoName()` -- static, returns template input DTO

**Singletons** (EventRegistry, FeatureEventRegistry, UndoRedoController, ServiceLocator) use `QML_FOREIGN` + `QML_SINGLETON` with a static `create()` method that retrieves the instance from ServiceLocator.

**List models and singles** use `QML_FOREIGN` + `QML_NAMED_ELEMENT` to directly expose the C++ class without additional wrapping.

## Event System

The EventRegistry provides decoupled communication between the backend and QML:

```cpp
// Generated in common/direct_access/{entity}/{entity}_events.h
class BinderItemEvents : public QObject {
    Q_OBJECT
signals:
    void created(QList<int> ids);
    void updated(QList<int> ids);
    void removed(QList<int> ids);
    void relationshipChanged(int id, BinderItemRelationshipField relationship, const QList<int> &relatedIds);
    void allRelationsInvalidated(int id);
};
```

Models automatically subscribe to relevant events. You can also subscribe directly in QML for custom behavior:

```qml
import MyApp.Controllers

Connections {
    target: EventRegistry.binderItemEvents()
    function onCreated(ids) {
        console.log("New BinderItems created:", ids)
    }
}
```

To avoid blocking the UI, it's a common pattern to execute an action from QML, then react to the resulting event.
It's known that the indirection makes debugging
difficult and can cause race conditions with multiple subscribers. It's a mess, so my recommendation is to avoid this
antipattern. Instead, let models handle updates reactively when possible.

To access entities directly without going through models, use QCoro to await results from their dedicated entity controllers.

Note: you can't chain ".then(...)" with QCoro calls directly because they return `QCoro::QmlTask`, not a JavaScript Promise.

There is no model for custom features and their use cases. Like entities, you can access them through their controllers, using QCoro to await results directly instead of relying on events:

```qml
import MyApp.Controllers

WorkManagementController {
    id: workManagementController
}

Button {
    text: "Save"
    onClicked: {
        let dto = workManagementController.getSaveWorkDto();
        dto.fileName = "/tmp/mywork.skr";

        workManagementController.saveWork(dto).then(function (result) {
            console.log("Async save result:", result);
        });
    }
}
```

## Undo/Redo in QML

The `UndoRedoController` singleton exposes the undo/redo system to QML:

```qml
import MyApp.Controllers

Button {
    text: "Undo: " + UndoRedoController.undoText()
    enabled: UndoRedoController.canUndo()
    onClicked: UndoRedoController.undo()
}

Button {
    text: "Redo: " + UndoRedoController.redoText()
    enabled: UndoRedoController.canRedo()
    onClicked: UndoRedoController.redo()
}
```

Both entity controllers and single entity models expose `undoRedoStackId` to route operations to a specific undo/redo stack.

## Best Practices

**Prefer list models over manual fetching.** The generated models handle caching, updates, and memory management. Fetching entity lists manually and storing them in JavaScript arrays loses reactivity.

**Use Single models for detail views.** When displaying one entity's details (an editor panel, a detail page), `Single{Entity}` gives you reactive properties with dirty tracking and save support.

**Keep model instances alive.** Creating a new model instance on every navigation discards cached data and subscriptions. Declare models at component level.

**Use QCoro for direct commands.** For actions outside of models, like custom features/use cases, use QCoro to await the result instead of relying on events.

**Leverage displayed field for simple lists.** The `list_model_displayed_field` provides a sensible default for list delegates (`Qt::DisplayRole`). For complex delegates, access individual roles directly.

**Use dirty + save for editable forms.** Bind fields to `Single{Entity}` properties, check `dirty` to enable a save button, then call `save()`. The model handles the async update and resets dirty on success.
