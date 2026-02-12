# QML Integration (C++/Qt)

This document covers QML-based frontends: **QtQuick**, **Kirigami**, **Lomiri**. They use QML, so the generated models and patterns apply equally to each.

---

Qleany generates reactive models ready for QML binding — no manual `QAbstractListModel` boilerplate.

## List Models

`{Parent}{Relationship}ListModel` provides a standard `QAbstractListModel` that:
- Auto-updates when entities change (via EventRegistry subscription)
- Refreshes only affected rows, not the entire model
- Supports inline editing through `setData` with async persistence
- Exposes all entity fields as roles

```qml
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

The model subscribes to two event sources:
- **Entity events** (`RecentWorkEvents.updated`) — refreshes only affected rows
- **Parent events** (`RootEvents.updated`) — full refresh if the relationship changed (items added/removed or reordered)

This means if another part of the application updates a RecentWork's title, the ListView updates automatically. If the Root's recentWorks list changes (item added/removed), the model detects the difference and refreshes.

## Single Entity Models

`Single{Entity}` wraps one entity instance for detail views and editor panels. The naming parallels `{Entity}ListModel`: where list models expose collections, single models expose individual entities. Instead of manually fetching the entity and wiring up change notifications, use `Single{Entity}`

`Single{Entity}` wraps one entity with:
- `itemId` property to select which entity
- Auto-fetch on ID change
- Reactive updates when the entity changes elsewhere in the application
- All fields are exposed as Q_PROPERTYs with change signals
- Relationship IDs available for further queries

```qml
SingleBinderItem {
    id: currentItem
    itemId: selectedItemId
}

Column {
    Text { text: currentItem.title }
    Text { text: currentItem.subTitle }
    Text { text: "Children: " + currentItem.binderItems.length }
    Text { text: "Parent: " + currentItem.parentItem }
}
```

The model subscribes to `BinderItemEvents.updated` — if any part of the application modifies this entity, the properties update automatically and QML bindings refresh.

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
    list_model: true                      # Generates BinderListModelFromWorkBinders
    list_model_displayed_field: name      # Default display role
```

## QML Mocks

Generated JavaScript stubs in `mock_imports/` mirror the real C++ API:

```
mock_imports/
└── Skr/
    ├── Controllers/
    │   ├── qmldir                                      # QML module definition
    │   ├── QCoroQmlTask.qml                            # Mock QCoro integration helper
    │   ├── EventRegistry.qml                           # EventRegistry
    │   ├── RootController.qml                          #
    │   ├── RootEvents.qml                              # Event signals for Root entity
    │   ├── BinderItemController.qml
    │   ├── BinderItemEvents.qml
    │   ├── RecentWorkController.qml
    │   └── RecentWorkEvents.qml
    ├── Models/
    │   ├── qmldir
    │   └── RecentWorkListModelFromRootRecentWorks.qml
    └── Singles/
        ├── qmldir
        └── SingleBinderItem.qml
```

Build with `YOUR_APP_BUILD_WITH_MOCKS` to develop UI without backend compilation:

```cmake
option(YOUR_APP_BUILD_WITH_MOCKS "Build with QML mocks instead of real backend" OFF)
```

UI developers can iterate on screens with mock data. When ready, disable the flag and the real controllers take over with no QML changes required.

Just to be clear: the mocks are only for UI development. They don't implement real business logic or data persistence.

Here is the equivalent real C++ import structure:

```
real_imports/
├── CMakeLists.txt
├── controllers/
│   ├── CMakeLists.txt
│   ├── foreign_root_controller.h
│   ├── foreign_binder_item_controller.h
│   ├── foreign_recent_work_controller.h
│   └── foreign_event_registry.h
├── models/
│   ├── CMakeLists.txt
│   └── foreign_recent_work_list_model_from_root_recent_works.h
└── singles/
    ├── CMakeLists.txt
    └── foreign_single_binder_item.h
```

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
    void relationshipChanged(int id, BinderItemRelationshipField relationship, const QList<int> &relatedIds)
};
```

Models automatically subscribe to relevant events. You can also subscribe directly in QML for custom behavior:

```qml
Connections {
    target: EventRegistry.binderItem()
    function onCreated(id) {
        console.log("New BinderItem created:", id)
    }
}
```

To avoid blocking the UI, it's a common pattern to execute an action from QML, then react to the resulting event. 
It's known that the indirection makes debugging
difficult and can cause race conditions with multiple subscribers. It's a mess, so my recommendation is to avoid this 
antipattern. Instead, let models handle updates reactively when possible.

To access entities directly without going through models, use QCoro to await results from their dedicated entities controllers.

Note: you can't chain ".then(...)" with QCoro calls directly because they return `QCoroQmlTask`, not a JavaScript Promise.

There is no model for custom features and their use cases. Like entities, you can access them through their controllers, using QCoro
to await results directly instead of relying on events:

```qml
import YouApp.Controllers

WorkManagementController {
    id: workManagementController

}

Button {
    id: saveButton

    text: "Save"

    onClicked: {
        console.log("Save button clicked");
        let dto = workManagementController.getSaveWorkDto();
        dto.fileName = "/tmp/mywork.skr";

        workManagementController.saveWork(dto).then(function (result) {
            console.log("Async save result :", result);
        });
    }
}

```


## Best Practices

**Prefer list models over manual fetching.** The generated models handle caching, updates, and memory management. Fetching entity lists manually and storing them in JavaScript arrays loses reactivity.

**Use Single models for detail views.** When displaying one entity's details (an editor panel, a detail page), `Single{Entity}` gives you reactive properties without you having to manage refresh logic.

**Keep model instances alive.** Creating a new model instance on every navigation discards cached data and subscriptions. Declare models at component level.

**Use QCoro for direct commands.** For actions outside of models, like custom features/use cases, use QCoro to await the result instead of relying on events.

**Leverage displayed field for simple lists.** The `list_model_displayed_field` provides a sensible default for list delegates. For complex delegates, access individual roles directly.
