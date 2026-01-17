# Entity Tree and Undo-Redo Architecture

Undo-redo systems are harder than they first appear. A robust implementation must handle complex scenarios like nested entities, cascading changes, and maintaining data integrity during undos/redos. Qleany's undo-redo architecture is designed to simplify these challenges by enforcing clear rules on how entities relate to each other in the context of undo-redo operations. It means rules to follow when designing your entity tree.

Entities in Qleany form a tree structure based on strong (ownership) relationships. This tree organization directly influences how undo-redo works across your application.

## My Recommendations

Do not use a single, linear, undo-redo stack for the entire application (but for basic applications). It's a trap. Think about interactions from the user's perspective: they expect undo/redo to apply to specific contexts (documents, projects) rather than globally. A monolithic stack leads to confusion and unintended consequences.

For example, if a user is editing a document and undoes an action, they don't expect that to also undo changes in unrelated settings or other documents. Instead, each context should have its own undo-redo stack.  Undo can be done by a dumb Ctrl-Z or a toast notification with "Undo" action after a destructive operation. Is the user expecting more granular control over what gets undone? Probably not. Keep it simple, limited to the context they are working in.

Also, for destructive operations (deleting entities), Qleany supports cascading deletions, but not their undoing. If you delete a parent entity, all its strongly-owned children are also deleted.  At first, I used a database savepoint to be restored on undo, but the savepoint impacted the non-undoable data as well, leading to confusion and unexpected behavior.  Consequence: there is no undo for entity deletions, so what to do? 

At least for now, as said previously, **no undoing of deletions**. Instead, consider soft-deleting entities (marking them as inactive) if you want to allow recovery. "emptying the trash bin" manually would be a permanent action, not undoable, with all the undo redo stacks cleared. Thus, 1. Users can recover deleted items from a trash bin, and 2. Undo/redo stacks remain consistent and manageable.

Thi seems like a limitation, but in practice, users rarely need to undo deletions if they have a way to recover deleted items through a trash or archive system.

### Soft Deletion

If you want to implement soft deletion, add a boolean field `activated` to your entities. When "deleting" an entity, set this flag to false instead of removing it from the database. Your UI can then filter out entities where `activated` is false, effectively hiding them from the user. When you want to permanently delete entities (e.g., emptying the trash), you can then remove them from the database. This will clear all the undo-redo stacks, which is acceptable since permanent deletion is a non-undoable action from the user's perspective, too. 

Conretely, you can have a `TrashBin` entity that holds references to soft-deleted entities. Users can restore them from the trash bin or permanently delete them.

| Id | trashed_date | entity_type | entity_id
|----|--------------|-------------|-----------|
| 1  | 2024-01-01   | Document    | 42
| 2  | 2024-01-02   | Car         | 7

It would typically need dedicated logic in use cases to handle restoring entities.

## Entity Properties

Each entity can define these properties:

| Property | Type | Default | Effect |
|----------|------|---------|--------|
| `undoable` | bool | false | Adds undo/redo support to the entity's controller |
| `allow_direct_access` | bool | true | Generates entity files in `direct_access/` for UI access |
| `single_model` | bool | false | Generates `Single{Entity}` wrapper for QML (C++/Qt only) |

## Undo-Redo Rules

The undo-redo system follows strict inheritance rules through the entity tree:

1. **A non-undoable entity cannot have an undoable entity as parent** (strong relationship)
2. **All children of an undoable entity must also be undoable**
3. **Weak relationships (references) can point to any entity** regardless of undo status

These rules ensure that when you undo an operation on a parent entity, all its strongly-owned children can be consistently rolled back.

> **What happens if you violate these rules?** The code will generate, compile, and run — Qleany doesn't enforce these rules at generation time. However, undo/redo stacks will become inconsistent. For example, if you place non-undoable persistent settings as a child of an undoable entity, those settings could be unexpectedly undone by cascade when the user undoes the parent. You don't want app settings disappearing because the user undid an unrelated action.
>
> Follow these rules strictly. If data shouldn't participate in undo (like settings), place it in a separate non-undoable trunk — don't nest it under undoable entities.
>
> *A basic validation system checks some of these rules at generation time. It's being improved to be able to check at load time.*

## Entity Tree Configurations

Depending on your application's complexity, you can organize your entity tree in three ways:

### Configuration 1: No Undo-Redo

For simple applications where undo-redo is not needed. All entities are non-undoable.

```
Root (undoable: false)
├── Settings
├── Project
│   ├── Document
│   └── Asset
└── Cache
```

```yaml
entities:
  - name: Root
    inherits_from: EntityBase
    undoable: false
    fields:
      - name: settings
        type: entity
        entity: Settings
        relationship: one_to_one
        strong: true
      - name: projects
        type: entity
        entity: Project
        relationship: ordered_one_to_many
        strong: true
```

> **Note:** Even without user-facing undo-redo, the undo system must be initialized internally as it's used for transaction management.

### Configuration 2: Simple App with Undo-Redo

For applications where all user data should support undo-redo. Root is non-undoable, with a single undoable trunk.

```
Root (undoable: false)
└── Workspace (undoable: true)     ← All user data under this trunk
    ├── Project (undoable: true)
    │   ├── Document (undoable: true)
    │   └── Asset (undoable: true)
    └── Tag (undoable: true)
```

```yaml
entities:
  - name: Root
    inherits_from: EntityBase
    undoable: false
    fields:
      - name: workspace
        type: entity
        entity: Workspace
        relationship: one_to_one
        strong: true

  - name: Workspace
    inherits_from: EntityBase
    undoable: true
    fields:
      - name: projects
        type: entity
        entity: Project
        relationship: ordered_one_to_many
        strong: true
      - name: tags
        type: entity
        entity: Tag
        relationship: one_to_many
        strong: true

  - name: Project
    inherits_from: EntityBase
    undoable: true
    fields:
      - name: documents
        type: entity
        entity: Document
        relationship: ordered_one_to_many
        strong: true
```

### Configuration 3: Complex App with Undo-Redo

For applications that need both undoable user data and non-undoable system data (configurations, search results, temporary state). Root has two trunks: one undoable, one non-undoable.

```
Root (undoable: false)
├── System (undoable: false)       ← Non-undoable trunk
│   ├── Settings (undoable: false)
│   ├── RecentFiles (undoable: false)
│   └── SearchResults (undoable: false)
│
└── Workspace (undoable: true)     ← Undoable trunk
    ├── Event (undoable: true)
    │   └── Attendee (undoable: true)
    └── Calendar (undoable: true)
```

```yaml
entities:
  - name: Root
    inherits_from: EntityBase
    undoable: false
    fields:
      - name: system
        type: entity
        entity: System
        relationship: one_to_one
        strong: true
      - name: workspace
        type: entity
        entity: Workspace
        relationship: one_to_one
        strong: true

  # Non-undoable trunk
  - name: System
    inherits_from: EntityBase
    undoable: false
    allow_direct_access: true
    fields:
      - name: settings
        type: entity
        entity: Settings
        relationship: one_to_one
        strong: true
      - name: recentFiles
        type: entity
        entity: RecentFile
        relationship: ordered_one_to_many
        strong: true
      - name: searchResults
        type: entity
        entity: SearchResult
        relationship: one_to_many
        strong: true

  - name: Settings
    inherits_from: EntityBase
    undoable: false
    fields:
      - name: theme
        type: string
      - name: language
        type: string

  - name: SearchResult
    inherits_from: EntityBase
    undoable: false
    allow_direct_access: false   # Temporary, no UI access needed
    fields:
      - name: query
        type: string
      - name: matchedItem
        type: entity
        entity: Event
        relationship: many_to_one  # Weak reference to undoable entity

  # Undoable trunk
  - name: Workspace
    inherits_from: EntityBase
    undoable: true
    fields:
      - name: events
        type: entity
        entity: Event
        relationship: ordered_one_to_many
        strong: true
      - name: calendars
        type: entity
        entity: Calendar
        relationship: one_to_many
        strong: true

  - name: Event
    inherits_from: EntityBase
    undoable: true
    single_model: true
    fields:
      - name: title
        type: string
      - name: attendees
        type: entity
        entity: Attendee
        relationship: one_to_many
        strong: true
        list_model: true
```

## Cross-Trunk References

Non-undoable entities can hold **weak references** (many_to_one, many_to_many) to undoable entities. This is useful for search results, recent items, or bookmarks that point to user data without owning it.

```yaml
- name: SearchResult
  undoable: false
  fields:
    - name: matchedEvent
      type: entity
      entity: Event           # Event is undoable
      relationship: many_to_one  # Weak reference — allowed
```

The reverse is also true: undoable entities can reference non-undoable entities (like referencing a Settings entity for default values).

## Choosing Your Configuration

| Application Type | Configuration | Example |
|------------------|---------------|---------|
| Simple utility | No undo-redo | File converter, system tool |
| Document editor | Single undoable trunk | Text editor, drawing app |
| Complex workspace | Two trunks | IDE, creative suite, calendar app |

The key question: **Do you have data that shouldn't participate in undo?**

- Settings and preferences → Non-undoable
- Search results and caches → Non-undoable
- User-created content → Undoable
- Temporary UI state → Non-undoable

If everything is user content, use a single undoable trunk. If you have a mix, split into two trunks.

---

For implementation details of the undo/redo system (command infrastructure, async execution, scoped stacks), see [Generated Infrastructure](generated-code.md).
