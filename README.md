# Qleany

**Architecture scaffolding generator for desktop applications and CLI tools.**

Qleany generates Package by Feature (Vertical Slice Architecture) code from a YAML manifest. Define your entities and features once, generate consistent scaffolding across Rust and C++/Qt.

## Why Qleany

Writing CRUD operations, DTOs, repositories, undo/redo infrastructure, and reactive UI models is tedious. The patterns are well-understood, but implementing them consistently across a codebase takes time.

Qleany generates this scaffolding so you can focus on business logic. It targets a specific architectural style — Package by Feature with Clean Architecture principles — that works well for desktop applications and CLI tools.

**What Qleany is not:**
- A framework (no runtime dependencies)
- A solution for web services or high-throughput systems

## Key Features

- **Complete CRUD scaffolding** — Controllers, DTOs, use cases, repositories per entity
- **Undo/redo system** — Command-based with grouping, scopes, and failure strategies
- **Reactive QML models** — Auto-updating list models and single-entity wrappers
- **QML mocks** — JavaScript stubs for UI development without backend
- **Relationship management** — Junction tables with ordering, caching, cascade deletion
- **Event system** — Decoupled communication between features

## Target Languages

| Language | Status | Database | Frontend |
|----------|--------|----------|----------|
| C++/Qt6 | ✓ Done | SQLite | QML with reactive models |
| Rust | ✓ Done | redb | Slint |

## Architecture Background

### What is Clean Architecture?

Clean Architecture, introduced by Robert C. Martin, organizes code into concentric layers with strict dependency rules:

```
┌─────────────────────────────────────────┐
│            Frameworks & UI              │  ← Outer: Qt, QML, SQLite
├─────────────────────────────────────────┤
│          Controllers & Gateways         │  ← Interface adapters
├─────────────────────────────────────────┤
│              Use Cases                  │  ← Application business rules
├─────────────────────────────────────────┤
│              Entities                   │  ← Core: Enterprise business rules
└─────────────────────────────────────────┘
```

**The Dependency Rule**: Source code dependencies point inward. Inner layers know nothing about outer layers. Entities don't know about use cases. Use cases don't know about controllers. This makes the core testable without frameworks.

**Key concepts Qleany retains:**
- **Entities** — Domain objects with identity and business rules
- **Use Cases** — Single-purpose operations encapsulating business logic
- **DTOs** — Data transfer objects crossing layer boundaries
- **Repositories** — Abstractions over data access
- **Dependency Inversion** — High-level modules don't depend on low-level modules

### The Problem with Pure Clean Architecture

Strict Clean Architecture organizes code by *layer*:

```
src/
├── domain/
│   └── entities/
│       ├── work.h
│       ├── binder.h
│       └── binder_item.h
├── application/
│   └── use_cases/
│       ├── work/
│       ├── binder/
│       └── binder_item/
├── infrastructure/
│   └── repositories/
│       ├── work_repository.h
│       └── binder_repository.h
└── presentation/
    └── controllers/
        ├── work_controller.h
        └── binder_controller.h
```

To modify "Binder," you touch four directories. For a 17-entity project, Qleany v1 generated **1700+ files across 500 folders**. Technically correct, practically unmaintainable.

### Package by Feature (Vertical Slice Architecture)

Package by Feature groups code by *what it does*, not *what layer it belongs to*:

```
src/
├── common/                      # Truly shared infrastructure
│   ├── entities/
│   ├── database/
│   └── undo_redo/
└── direct_access/
    └── binder/                  # Everything about Binder in one place
        ├── binder_controller.h
        ├── binder_repository.h
        ├── dtos.h
        ├── unit_of_work.h
        └── use_cases/
            ├── create_uc.h
            ├── get_uc.h
            ├── update_uc.h
            └── remove_uc.h
```

**Benefits:**
- **Discoverability** — Find all Binder code in one folder
- **Cohesion** — Related code changes together
- **Fewer files** — Same 17-entity project produces ~200 files
- **Easier onboarding** — New developers understand features, not layers

**What we keep from Clean Architecture:**
- Dependency direction (UI → Controllers → Use Cases → Repositories → Database)
- Use cases as the unit of business logic
- DTOs at boundaries
- Repository pattern for data access
- Testability through clear interfaces

**What we drop:**
- Strict layer-per-folder organization
- Separate "domain" module (entities live in `common`)
- Interface-for-everything (only where it aids testing)

### Why This Matters for Desktop Apps

Web frameworks often provide architectural scaffolding (Rails, Django, Spring). Desktop frameworks like Qt provide widgets and signals, but no guidance on organizing a 50,000-line application.

Qleany fills that gap with an architecture that:
- Scales from small tools to large applications
- Integrates naturally with Qt's object model
- Supports undo/redo, a desktop-specific requirement
- Keeps related code together for solo developers and small teams

## Design Philosophy

### Generate and Disappear

Qleany generates code, then gets out of your way. The output has no dependency on Qleany itself. Modify, extend, or delete the generated code freely. Run the generator again when you add entities or features — it will regenerate the scaffolding without touching your business logic.

### Package by Feature

Code is organized by feature (vertical slices) rather than by layer (horizontal slices). A feature contains its controller, DTOs, use cases, and units of work in one place. Cross-cutting concerns live in a shared `common` module.

```
src/
├── common/                      # Shared infrastructure
│   ├── database/                # DbContext, transactions, caching
│   ├── direct_access/           # Per-entity repository + table + events
│   ├── entities/                # Domain entities
│   └── undo_redo/               # Command infrastructure
├── direct_access/               # CRUD feature (auto-generated)
│   └── {entity}/                # Vertical slice per entity
│       ├── {entity}_controller
│       ├── {entity}_list_model_from_{parent}_{relationship}
│       ├── single_{entity}
│       ├── dtos
│       ├── unit_of_work
│       └── use_cases/
│           ├── create_uc
│           ├── get_uc
│           ├── update_uc
│           └── remove_uc
└── {custom_feature}/            # Your features
    ├── {feature}_controller
    ├── dtos
    ├── units_of_work/
    └── use_cases/
```

## QML Integration (C++/Qt)

Qleany generates reactive models ready for QML binding — no manual `QAbstractListModel` boilerplate.

### List Models

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

### Single Entity Models

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

### QML Mocks

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

Build with `SKR_BUILD_WITH_MOCKS` to develop UI without backend compilation:

```cmake
option(SKR_BUILD_WITH_MOCKS "Build with QML mocks instead of real backend" OFF)
```

UI developers can iterate on screens with mock data. When ready, disable the flag and the real controllers take over with no QML changes required.

## The Manifest

Everything is defined in `qleany.yaml`:

```yaml
schema:
  version: 2

global:
  language: cpp          # rust, cpp
  application_name: MyApp
  organisation:
    name: myorg
    domain: myorg.com
  prefix_path: src

entities:
  - name: Root
    parent: EntityBase
    fields:
      - name: works
        type: entity
        entity: Work
        relationship: ordered_one_to_many
        strong: true
        list_model: true
        list_model_displayed_field: title

  - name: Work
    parent: EntityBase
    fields:
      - name: title
        type: string
      - name: binders
        type: entity
        entity: Binder
        relationship: ordered_one_to_many
        strong: true
        list_model: true
        list_model_displayed_field: name
      - name: tags
        type: entity
        entity: BinderTag
        relationship: one_to_many
        strong: true

  - name: Binder
    parent: EntityBase
    fields:
      - name: name
        type: string
      - name: items
        type: entity
        entity: BinderItem
        relationship: ordered_one_to_many
        strong: true
        list_model: true
        list_model_displayed_field: title

  - name: BinderItem
    parent: EntityBase
    fields:
      - name: title
        type: string
      - name: parentItem
        type: entity
        entity: BinderItem
        relationship: one_to_one
      - name: tags
        type: entity
        entity: BinderTag
        relationship: many_to_many

features:
  - name: work_management
    use_cases:
      - name: load_work
        validator: true
        undoable: false
        entities: [Root, Work, Binder, BinderItem]
        dto_in:
          name: LoadWorkDto
          fields:
            - name: file_path
              type: string
        dto_out:
          name: LoadWorkResultDto
          fields:
            - name: work_id
              type: integer
```

### Entity Field Options

**Relationship type** (required for `type: entity`):

| Relationship | Junction Type | Return Type |
|--------------|---------------|-------------|
| `one_to_one` | OneToOne | `std::optional<int>` |
| `one_to_many` | UnorderedOneToMany | `QList<int>` |
| `ordered_one_to_many` | OrderedOneToMany | `QList<int>` |
| `many_to_many` | UnorderedManyToMany | `QList<int>` |

**Relationship flags:**

| Flag | Valid for | Effect |
|------|-----------|--------|
| `required` | `one_to_one` | Validated on create/update (1..1 instead of 0..1) |
| `strong` | `one_to_one`, `one_to_many`, `ordered_one_to_many` | Cascade deletion — removing parent removes children |

**QML generation flags:**

| Flag | Effect |
|------|--------|
| `list_model` | Generate `{Entity}ListModelFrom{Parent}{Relationship}` |
| `list_model_displayed_field` | Default display role for the list model |
| `single` | Generate `Single{Entity}` wrapper |

### Relationship Types

Four relationship types mapping directly to four junction table implementations:

```yaml
# Optional single reference (0..1)
- name: parentItem
  type: entity
  entity: BinderItem
  relationship: one_to_one

# Required single reference (1..1)
- name: owner
  type: entity
  entity: User
  relationship: one_to_one
  required: true

# Unordered children with cascade delete
- name: tags
  type: entity
  entity: BinderTag
  relationship: one_to_many
  strong: true

# Ordered children (chapters, items)
- name: chapters
  type: entity
  entity: BinderItem
  relationship: ordered_one_to_many
  strong: true

# Shared references (tags on items)
- name: tags
  type: entity
  entity: BinderTag
  relationship: many_to_many
```

**Validation rules:**

| Flag | `one_to_one` | `one_to_many` | `ordered_one_to_many` | `many_to_many` |
|------|--------------|---------------|----------------------|----------------|
| `required` | ✓ | ✗ | ✗ | ✗ |
| `strong` | ✓ | ✓ | ✓ | ✗ |

Invalid combinations are rejected at manifest parsing.

**Ownership pattern for many-to-many:**

Many-to-many relationships are always weak — the entities must be owned elsewhere. For tags:

```yaml
entities:
  - name: Work
    fields:
      - name: tags                        # Owns the tags
        type: entity
        entity: BinderTag
        relationship: one_to_many
        strong: true

  - name: BinderItem
    fields:
      - name: tags                        # References tags owned by Work
        type: entity
        entity: BinderTag
        relationship: many_to_many
```

`Work` owns `BinderTag` entities. `BinderItem` references them. Deleting a `Work` deletes its tags. Deleting a `BinderItem` only removes the references.

## Generated Infrastructure

### Database Layer

**DbContext / DbSubContext**: Connection pool with scoped transactions. Each unit of work owns a `DbSubContext` providing `beginTransaction`, `commit`, `rollback`, and savepoint support.

**Repository Factory**: Creates repositories bound to a specific `DbSubContext`. Returns owned instances (`std::unique_ptr` in C++) — no cross-thread sharing.

**Table Cache / Junction Cache**: Thread-safe, time-expiring (30 minutes), invalidated at write time. Improves performance for repeated queries within a session.

### C++/Qt Specific

**SQLite with WAL mode**: Optimized for desktop writing applications:
```sql
PRAGMA journal_mode=WAL;
PRAGMA synchronous=NORMAL;
PRAGMA cache_size=20000;        -- 20MB
PRAGMA mmap_size=268435456;     -- 256MB
```

**Ephemeral Database Pattern**: The internal database lives in `/tmp/`, decoupled from user files:
- **Load**: Transform file → internal database
- **Work**: All operations against ephemeral database
- **Save**: Transform internal database → file
- **Crash Recovery**: Detect orphaned database, offer recovery

**Async Undo/Redo with QCoro**: Commands execute asynchronously using C++20 coroutines. Supports scoped stacks, command grouping, and multiple failure strategies.

**Event Registry**: QObject-based event dispatch. Repositories emit `created`, `updated`, `removed` signals. Models subscribe for reactive updates.

### Rust Specific

**redb Backend**: Embedded key-value storage with the same patterns as SQLite.

**Long Operation Manager**: Threaded execution for heavy tasks with progress callbacks and cancellation.

**Synchronous Commands**: Simpler execution model without async complexity.

## Building Qleany

```bash
git clone https://github.com/jacquetc/qleany
cd qleany
cargo build --release
```

## Running the UI

```bash
cargo run --release
```

The Slint-based UI provides:
- Form-based manifest editing
- Entity and relationship management
- Selective file generation
- Code preview before writing

## CLI Usage

```bash
# Generate all files
qleany generate --manifest qleany.yaml --output ./src

# Generate specific feature
qleany generate --manifest qleany.yaml --output ./src --feature work_management

# List files that would be generated
qleany list --manifest qleany.yaml
```

## Reference Implementation

[Skribisto](https://github.com/jacquetc/skribisto) (develop branch) is a novel-writing application built with Qleany-generated C++/Qt code. It demonstrates:

- Full entity hierarchy (Root → Work → Binder → BinderItem → Content)
- Complex relationships (ordered children, many-to-many tags)
- Feature orchestration (LoadWork, SaveWork with file format transformation)
- Reactive QML UI with auto-updating models
- Undo/redo across structural and content operations
- Crash recovery with ephemeral database

Skribisto serves as both proof-of-concept and template source for C++/Qt generation.

## Migration from v1

Qleany v1 (Python/Jinja2) generated pure Clean Architecture with strict layer separation. A 17-entity project produced 1700+ files across 500 folders.

v2 generates Package by Feature with pragmatic organization. The same project produces ~200 files with better discoverability.

**Breaking changes:**
- Manifest format changed (schema version 2)
- Output structure reorganized by feature
- Reactive models are new (list models, singles)

v1 remains on the main branch for existing users.

## Contributing

Qleany is developed alongside Skribisto. The architecture is stable, but templates are being extracted and refined.

To contribute:
1. Open an issue to discuss changes
2. Reference Skribisto patterns where applicable
3. Ensure changes work for both Rust and C++/Qt

## License

MPL-2.0