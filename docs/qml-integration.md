# QML Integration (C++/Qt)

This document covers QML-based frontends: **QtQuick** and **Kirigami**. Both use QML, so the generated models and patterns apply equally to either.

For **QtWidgets** and **KDE KF6 Widgets** integration, see the dedicated document (coming soon).

---

Qleany generates reactive models ready for QML binding — no manual `QAbstractListModel` boilerplate.

## List Models

`{Entity}ListModelFrom{Parent}{Relationship}` provides a standard `QAbstractListModel` that:
- Auto-updates when entities change (via EventRegistry subscription)
- Refreshes only affected rows, not the entire model
- Supports inline editing through `setData` with async persistence
- Exposes all entity fields as roles

```qml
ListView {
    model: RecentWorkListModelFromRootRecentWorks {
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
- **Parent events** (`RootEvents.updated`) — full refresh if the relationship changed

This means if another part of the application updates a RecentWork's title, the ListView updates automatically. If the Root's recentWorks list changes (item added/removed), the model detects the difference and refreshes.

## Single Entity Models

`Single{Entity}` wraps one entity with:
- `itemId` property to select which entity
- Auto-fetch on ID change
- Reactive updates when the entity changes elsewhere in the application
- All fields exposed as Q_PROPERTYs with change signals
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
    │   ├── RootController.qml
    │   ├── BinderItemController.qml
    │   ├── RecentWorkController.qml
    │   └── EventRegistry.qml
    ├── Models/
    │   └── RecentWorkListModelFromRootRecentWorks.qml
    └── Singles/
        └── SingleBinderItem.qml
```

Build with `YOUR_APP_BUILD_WITH_MOCKS` to develop UI without backend compilation:

```cmake
option(YOUR_APP_BUILD_WITH_MOCKS "Build with QML mocks instead of real backend" OFF)
```

UI developers can iterate on screens with mock data. When ready, disable the flag and the real controllers take over with no QML changes required.

## Event System

The EventRegistry provides decoupled communication between the backend and QML:

```cpp
// Generated in common/direct_access/{entity}/{entity}_events.h
class BinderItemEvents : public QObject {
    Q_OBJECT
signals:
    void created(int id);
    void updated(int id);
    void removed(int id);
};
```

Models automatically subscribe to relevant events. You can also subscribe directly in QML for custom behavior:

```qml
Connections {
    target: EventRegistry.binderItemEvents
    function onCreated(id) {
        console.log("New BinderItem created:", id)
    }
}
```

## Best Practices

**Prefer list models over manual fetching.** The generated models handle caching, updates, and memory management. Fetching entity lists manually and storing them in JavaScript arrays loses reactivity.

**Use Single models for detail views.** When displaying one entity's details (an editor panel, a detail page), `Single{Entity}` gives you reactive properties without managing refresh logic.

**Keep model instances alive.** Creating a new model instance on every navigation discards cached data and subscriptions. Declare models at component level, not inside functions.

**Leverage displayed field for simple lists.** The `list_model_displayed_field` provides a sensible default for list delegates. For complex delegates, access individual roles directly.
