# Mobile Development

This document covers using Qleany-generated code for mobile platforms: Plasma Mobile, Ubuntu Touch, and other Linux-based mobile environments.

Qleany's architecture is platform-agnostic. The generated backend code works identically on desktop and mobile. You write platform-specific UI code on top of the same controllers, repositories, and use cases. Of course, you can also build for mobile using QtQuick or Slint frontends.

All but Slint are Qt-based (QML), so you get the full power of Qt's cross-platform capabilities. Qleany generates C++ code that integrates seamlessly with QtQuick/QML UIs.

**No dependencies, no framework.** The generated code has no Qleany runtime, no base classes to inherit from, no library to link against. You own all the generated code — modify it, extend it, or delete parts you don't need. If you decide to stop using Qleany tomorrow, your application continues to work unchanged.

---

## The Mobile Landscape

If you're building for Plasma Mobile or Ubuntu Touch, you're already using Qt. The architecture Qleany generates works identically on these platforms — there's nothing desktop-specific about the controllers, repositories, or business logic. Only the UI layer needs platform-specific consideration.

**Plasma Mobile** uses Kirigami, KDE's convergent UI framework built on QtQuick. Kirigami components adapt their appearance and behavior based on form factor — the same app can run on desktop and mobile with appropriate layouts.

**Ubuntu Touch** uses Lomiri (formerly Unity8) with its own QML component library. The UI layer is different from Kirigami, but the backend architecture is identical.

**Slint** is positioning itself for embedded and mobile use cases. Qleany generates Slint basic frontends for Rust and the wrapping around the backend. The architecture supports building mobile UIs manually on top of the generated infrastructure.

**QtQuick** applications can also run on mobile platforms. You can build a QtQuick UI that uses the generated backend code, just like you would on desktop.

---

## Same Backend, Different Frontends

The key insight is that Qleany generates platform-agnostic backend code. Your entities, repositories, controllers, DTOs, and use cases don't know or care whether they're running on a desktop, a phone, or a command-line tool.

```
┌─────────────────────────────────────────────────────────────┐
│                        UI Layer                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │  Kirigami   │  │   Lomiri    │  │  QtQuick            │  │
│  │  (Plasma)   │  │  (Ubuntu)   │  │  or QtWidgets       │  │
│  └──────┬──────┘  └──────┬──────┘  └──────────┬──────────┘  │
└─────────┼────────────────┼────────────────────┼─────────────┘
          │                │                    │
          └────────────────┼────────────────────┘
                           │
┌──────────────────────────┴──────────────────────────────────┐
│                  Generated Backend                          │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ Controllers │  │   Events    │  │    List Models      │  │
│  └──────┬──────┘  └──────┬──────┘  └──────────┬──────────┘  │
│         │                │                    │             │
│  ┌──────┴────────────────┴────────────────────┴──────────┐  │
│  │                    Use Cases                          │  │
│  └───────────────────────┬───────────────────────────────┘  │
│                          │                                  │
│  ┌───────────────────────┴───────────────────────────────┐  │
│  │              Repositories / Database                  │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

This means you can build a single application that deploys to multiple platforms with minimal UI code duplication. Your business logic — the hard part — is written once.

---

## Kirigami (Plasma Mobile)

Kirigami applications use the same QML models and controllers that Qleany generates. The reactive list models and single-entity wrappers work without modification.

### Project Setup

Your CMakeLists.txt needs Kirigami in addition to the standard Qt dependencies:

```cmake
find_package(KF6 REQUIRED COMPONENTS Kirigami2)

target_link_libraries(${PROJECT_NAME}
    Qt6::Core
    Qt6::Qml
    Qt6::Quick
    KF6::Kirigami2
    # ... your generated backend libraries
)
```

### Using Generated Models

The generated list models work directly with Kirigami's ListView and other components:

```qml
import QtQuick
import org.kde.kirigami as Kirigami
import MyApp.Models

Kirigami.ScrollablePage {
    title: "Notes"
    
    ListView {
        model: NoteListModelFromWorkspaceNotes {
            workspaceId: 1
        }
        
        delegate: Kirigami.BasicListItem {
            label: model.title
            subtitle: model.updatedAt
            onClicked: pageStack.push(noteDetailPage, { noteId: model.itemId })
        }
    }
    
    actions: [
        Kirigami.Action {
            icon.name: "list-add"
            text: "New Note"
            onTriggered: NoteController.create({ title: "Untitled", content: "" })
        }
    ]
}
```

### Convergent Design

Kirigami's strength is convergent UI — components that work on both desktop and mobile. The generated backend supports this naturally because it doesn't assume any particular screen size or interaction model.

Consider organizing your QML with form-factor awareness:

```qml
Kirigami.ApplicationWindow {
    id: root
    
    pageStack.initialPage: Kirigami.Page {
        // On mobile, show list only; on desktop, show list + detail side by side
        Kirigami.ColumnView.fillWidth: Kirigami.Settings.isMobile
    }
}
```

The generated models emit the same signals regardless of platform. Your detail view updates when the entity changes, whether the user is on a phone or a 27" monitor.

---

## Ubuntu Touch (Lomiri)

Ubuntu Touch uses Lomiri UI Toolkit components instead of Kirigami, but the underlying Qt/QML is the same. Qleany's generated backend works without modification.

### Project Setup

Ubuntu Touch uses Click packaging and has specific CMake requirements:

```cmake
find_package(Qt6 REQUIRED COMPONENTS Core Qml Quick)

# Lomiri components are typically available system-wide on Ubuntu Touch
# Include paths may vary by Ubuntu Touch version
```

### Using Generated Models

Lomiri components have different names but similar patterns to Kirigami:

```qml
import QtQuick
import Lomiri.Components
import MyApp.Models

Page {
    header: PageHeader {
        title: "Notes"
        trailingActionBar.actions: [
            Action {
                iconName: "add"
                onTriggered: NoteController.create({ title: "Untitled", content: "" })
            }
        ]
    }
    
    ListView {
        anchors.fill: parent
        
        model: NoteListModelFromWorkspaceNotes {
            workspaceId: 1
        }
        
        delegate: ListItem {
            ListItemLayout {
                title.text: model.title
                subtitle.text: model.updatedAt
            }
            onClicked: pageStack.push(Qt.resolvedUrl("NoteDetail.qml"), { noteId: model.itemId })
        }
    }
}
```

### Ubuntu Touch Specifics

Ubuntu Touch applications run in a sandboxed environment with specific lifecycle events:

```qml
Connections {
    target: Qt.application
    
    function onStateChanged() {
        if (Qt.application.state === Qt.ApplicationSuspended) {
            // App is being suspended — save state
            WorkController.saveWork(currentWorkId)
        }
    }
}
```

The generated undo/redo system persists command history, so users don't lose their undo stack when the app is suspended and resumed.

---

## Mobile-Specific Considerations

### App Lifecycle

Mobile platforms suspend and resume applications more aggressively than desktop. The generated architecture handles this well because the internal database persists across suspend/resume cycles. However, you should save user data to files more frequently than you might on desktop.

Consider auto-saving on these events:
- Application suspension
- Significant user actions (creating, deleting entities)
- Periodic timer (every few minutes of activity)

### Memory Constraints

Mobile devices have less RAM than desktops. The generated table caches help performance but consume memory. If you're seeing memory pressure, you can reduce cache sizes in the generated database configuration code:

```cpp
// In db_context.cpp or similar
// Reduce cache from default values
// PRAGMA cache_size=5000;  // 5MB instead of 20MB
```

### Offline Operation

Mobile apps frequently operate offline. The generated architecture supports this naturally — all data lives in the local SQLite database, and network operations (if any) are handled in your custom use cases, not in the generated infrastructure.

Design your use cases with offline-first in mind. The generated CRUD operations never assume network connectivity.

### Touch-Friendly Undo

On mobile, dedicated undo/redo buttons may not fit your UI. Consider alternative patterns:

- Swipe gestures on list items to undo deletions
- Toast notifications with "Undo" action after destructive operations
- Shake-to-undo (though this is falling out of favor)

The generated undo system supports all of these — it provides the infrastructure, and your UI decides how to expose it.

---

## Building for Multiple Platforms

A typical multi-platform project structure:

```
myapp/
├── qleany.yaml                    # Single manifest
├── src/
│   ├── common/                    # Generated backend (shared)
│   ├── direct_access/             # Generated CRUD (shared)
│   ├─── my_feature/                # Your business logic (shared)
│   ├── desktop/                   # Desktop QtQuick UI
│   │   └── main.qml
│   ├── kirigami/                  # Plasma Mobile UI
│   │   └── main.qml
│   └── lomiri/                    # Ubuntu Touch UI
│       └── main.qml
├── CMakeLists.txt                 # Conditional compilation
└── packaging/
    ├── flatpak/                   # Desktop Linux
    ├── click/                     # Ubuntu Touch
    └── ...
```

Your CMakeLists.txt can select which UI to build:

```cmake
option(BUILD_DESKTOP_UI "Build desktop QtQuick UI" ON)
option(BUILD_KIRIGAMI_UI "Build Kirigami UI for Plasma Mobile" OFF)
option(BUILD_LOMIRI_UI "Build Lomiri UI for Ubuntu Touch" OFF)

# Backend is always built
add_subdirectory(src)

# Select UI
if(BUILD_DESKTOP_UI)
    add_subdirectory(ui/desktop)
elseif(BUILD_KIRIGAMI_UI)
    add_subdirectory(ui/kirigami)
elseif(BUILD_LOMIRI_UI)
    add_subdirectory(ui/lomiri)
endif()
```

The generated backend compiles once and links to whichever UI you're building.

---

## Slint on Mobile

Slint is designed for embedded and resource-constrained environments, making it potentially suitable for mobile. Q Rust backend architecture works the same way — you write the Slint UI manually on top of the generated controllers and repositories.

leany generates a basic Slint frontend and wrappers to access the frontend.

As Slint's mobile story matures, Qleany may add Slint frontend generation. For now, the backend generation provides the foundation.

---

## Performance on Mobile

Mobile processors are less powerful than desktop CPUs. The generated architecture includes several performance-conscious patterns:

**Lazy loading:** List models fetch data on demand, not all at once. Scrolling a list of 10,000 items doesn't load all 10,000 into memory.

**Efficient updates:** When an entity changes, only affected list rows refresh — not the entire model. This reduces UI redraw overhead.

**Cached queries:** The table cache prevents repeated database queries for the same data. This matters more on mobile where I/O can be slower.

**WAL mode SQLite:** The generated database configuration uses Write-Ahead Logging, which improves concurrent read performance — important when the UI thread is reading while a background operation is writing.

If you profile and find performance issues, the generated code is yours to optimize. Common improvements include adding database indices for frequently-queried fields or reducing cache expiration times to save memory at the cost of more database queries.
