# Manifest Reference

Everything in Qleany is defined in `qleany.yaml`. This document covers all manifest options.

## Example Manifests

Real-world manifests you can reference:

| Project | Language | Frontend | Link |
|---------|----------|----------|------|
| Skribisto | C++20 / Qt6 | QtQuick | [qleany.yaml](https://github.com/jacquetc/skribisto/blob/master/qleany.yaml) |
| Qleany | Rust 2024 | Slint + CLI | [qleany.yaml](https://github.com/jacquetc/qleany/blob/main/qleany.yaml) |

## Basic Structure

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
  - name: EntityBase
    # ...

features:
  - name: my_feature
    # ...
```

## Required Base Entity

All entities must have `id`, `created_at`, and `updated_at` fields. These are essential for identity, caching, and change tracking.

To simplify this, Qleany provides `EntityBase` — a heritable entity with these three fields pre-defined. When you create a new manifest, Qleany automatically generates:
- `EntityBase` with `id`, `created_at`, `updated_at`
- An empty `Root` entity inheriting from `EntityBase`

All your entities should inherit from `EntityBase` using the `inherits_from` field.

```yaml
entities:
  # EntityBase provides the necessary id, created_at, updated_at
  - name: EntityBase
    only_for_heritage: true
    allow_direct_access: false
    fields:
      - name: id
        type: integer
      - name: created_at
        type: datetime
      - name: updated_at
        type: datetime
        
  - name: Root
    inherits_from: EntityBase
    undoable: false
    fields:
      # Your root-level relationships here
```

## Complete Example

```yaml
schema:
  version: 2

global:
  language: cpp
  application_name: MyApp
  organisation:
    name: myorg
    domain: myorg.com
  prefix_path: src

entities:
  - name: EntityBase
    only_for_heritage: true
    allow_direct_access: false
    fields:
      - name: id
        type: integer
      - name: created_at
        type: datetime
      - name: updated_at
        type: datetime
        
  - name: Root
    inherits_from: EntityBase
    undoable: false
    fields:
      - name: works
        type: entity
        entity: Work
        relationship: ordered_one_to_many
        strong: true
        list_model: true
        list_model_displayed_field: title

  - name: Work
    inherits_from: EntityBase
    undoable: true
    single_model: true
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
    inherits_from: EntityBase
    undoable: true
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
    inherits_from: EntityBase
    undoable: true
    fields:
      - name: title
        type: string
      - name: parentItem
        type: entity
        entity: BinderItem
        relationship: many_to_one
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

---

## Entity Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `name` | string | required | Entity name (PascalCase) |
| `inherits_from` | string | none | Parent entity for inheritance |
| `only_for_heritage` | bool | false | Entity used only as base class |
| `undoable` | bool | false | Enable undo/redo for this entity's controller |
| `allow_direct_access` | bool | true | Generate files in `direct_access/` for UI access |
| `single_model` | bool | false | Generate `Single{Entity}` QML wrapper (C++/Qt only) |

---

## Field Types

| Type | Description | Example |
|------|-------------|---------|
| `boolean` | True/false value | `is_active: true` |
| `integer` | Whole number | `count: 42` |
| `float` | Decimal number | `price: 19.99` |
| `string` | Text | `name: "Alice"` |
| `uuid` | Unique identifier | `id: "550e8400-..."` |
| `datetime` | Date and time | `created_at: "2024-01-15T10:30:00"` |
| `entity` | Relationship to another entity | See relationship section |
| `enum` | Enumerated value | See enum section |

### Enum Fields

```yaml
- name: status
  type: enum
  enum_name: CarStatus
  enum_values:
    - Available
    - Reserved
    - Sold
```

---

## Relationship Fields

When `type: entity`, additional options define the relationship:

### Relationship Types

| Relationship | Junction Type | Return Type |
|--------------|---------------|-------------|
| `one_to_one` | OneToOne | `std::optional<int>` / `Option<i64>` |
| `many_to_one` | ManyToOne | `std::optional<int>` / `Option<i64>` |
| `one_to_many` | UnorderedOneToMany | `QList<int>` / `Vec<i64>` |
| `ordered_one_to_many` | OrderedOneToMany | `QList<int>` / `Vec<i64>` |
| `many_to_many` | UnorderedManyToMany | `QList<int>` / `Vec<i64>` |

### Relationship Flags

| Flag | Valid for | Effect |
|------|-----------|--------|
| `required` | `one_to_one`, `many_to_one` | Validated on create/update (1..1 instead of 0..1) |
| `strong` | `one_to_one`, `one_to_many`, `ordered_one_to_many` | Cascade deletion — removing parent removes children |

### QML Generation Flags (C++/Qt only)

| Flag | Effect |
|------|--------|
| `list_model` | Generate `{Entity}ListModelFrom{Parent}{Relationship}` |
| `list_model_displayed_field` | Default display role for the list model |

### Validation Rules

| Flag | `one_to_one`   | `many_to_one` | `one_to_many` | `ordered_one_to_many` | `many_to_many` |
|------|----------------|---------------|---------------|-----------------------|----------------|
| `required` | ✓/✗            | N.A.          | N.A.          | N.A.                  |  N.A. |
| `strong` | ✓/✗ (see note) | ✗             | ✓/✗           | ✓/✗                    | ✗ |

N.A.: Not applicable for this relationship type. Mark them whichever way you want it to be, there will be no change in generated code, or don't use them. When you write code, an empty list will show the want to hold no relationship.

Note : If one_to_one holds a weak relationship (`strong: false`), it couldn't be required (must be `required: false`). There is the risk of a dangling reference if the entity targeted by the reference is deleted.

Invalid combinations are rejected at manifest parsing.

---

## Understanding Relationships

Database relationships describe how entities connect. Two concepts matter:

**Cardinality** — How many entities can participate on each side?
- **One** — At most one entity (0..1 or exactly 1)
- **Many** — Zero or more entities (0..*)

**Direction** — Which side "owns" the relationship?
- The **parent** side holds the list of children
- The **child** side holds a reference back to its parent

```
┌─────────────────────────────────────────────────────────────┐
│                     RELATIONSHIP TYPES                      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   ONE-TO-ONE (1:1)                                          │
│   ┌───────┐         ┌───────┐                               │
│   │ User  │─────────│Profile│   Each user has one profile   │
│   └───────┘         └───────┘   Each profile belongs to     │
│                                 one user                    │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   ONE-TO-MANY (1:N)                                         │
│   ┌───────┐         ┌───────┐                               │
│   │Group  │────────<│ Item  │   One group  has many items   │
│   └───────┘         └───────┘   Binder.items: [1, 2, 3]     │
│                                                             │
│   MANY-TO-ONE (N:1)                                         │
│   ┌───────┐         ┌───────┐                               │
│   │ Car   │>────────│Brand  │   Many items belong to one    │
│   └───────┘         └───────┘   brand.  Car.brand: 5        │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   MANY-TO-MANY (N:M)                                        │
│   ┌───────┐         ┌───────┐                               │
│   │ Item  │>───────<│  Tag  │   Items have many tags        │
│   └───────┘         └───────┘   Tags apply to many items    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**When to use each:**

| Relationship | Use when...                           | Example |
|--------------|---------------------------------------|---------|
| `one_to_one` | Exactly one related entity, exclusive | User → Profile |
| `many_to_one` | Many entities reference one child     | Car → Brand, Comment → Post |
| `one_to_many` | Parent owns a collection of children  | Binder → Items, Post → Comments |
| `ordered_one_to_many` | Same as above, but order matters      | Book → Chapters, Playlist → Songs |
| `many_to_many` | Entities share references both ways   | Items ↔ Tags, Students ↔ Courses |

### Relationship Examples

```yaml
# Exclusive single reference (0..1) — each side has at most one
- name: profile
  type: entity
  entity: UserProfile
  relationship: one_to_one
  strong: true

# Back-reference to parent (N:1) — many children point to one parent
- name: parentItem
  type: entity
  entity: BinderItem
  relationship: many_to_one

# Required back-reference
- name: binder
  type: entity
  entity: Binder
  relationship: many_to_one
  required: true

# Unordered children with cascade delete (1:N)
- name: tags
  type: entity
  entity: BinderTag
  relationship: one_to_many
  strong: true

# Ordered children (1:N with order)
- name: chapters
  type: entity
  entity: BinderItem
  relationship: ordered_one_to_many
  strong: true

# Shared references (N:M)
- name: tags
  type: entity
  entity: BinderTag
  relationship: many_to_many
```

### Weak Relationships

Both `many_to_one` and `many_to_many` are always weak — they reference entities owned elsewhere. They cannot have `strong: true` because cascade deletion is controlled by the owning side.

```yaml
entities:
  - name: Work
    fields:
      - name: tags                        # Owns the tags (strong one-to-many)
        type: entity
        entity: BinderTag
        relationship: one_to_many
        strong: true

  - name: Binder
    fields:
      - name: items                       # Owns the items (strong ordered)
        type: entity
        entity: BinderItem
        relationship: ordered_one_to_many
        strong: true

  - name: BinderItem
    fields:
      - name: binder                      # Back-reference (weak many-to-one)
        type: entity
        entity: Binder
        relationship: many_to_one
        required: true

      - name: tags                        # Shared reference (weak many-to-many)
        type: entity
        entity: BinderTag
        relationship: many_to_many
```

> **Rule of thumb:** Every entity referenced by a weak relationship (`many_to_one` or `many_to_many`) must be strongly owned somewhere else in your entity graph. Without strong ownership, entities become orphans with no lifecycle management.

---

## Features and Use Cases

Features group related use cases together.

```yaml
features:
  - name: work_management
    use_cases:
      - name: load_work
        # ...
      - name: save_work
        # ...
```

### Use Case Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `name` | string | required | Use case name (snake_case) |
| `validator` | bool | false | Generate input validation |
| `undoable` | bool | false | Generate undo/redo command scaffolding |
| `read_only` | bool | false | No data modification (affects generated code) |
| `long_operation` | bool | false | Async execution with progress (Rust only) |
| `entities` | list | [] | Entities this use case works with |

### DTOs

Each use case can have input and output DTOs:

```yaml
use_cases:
  - name: import_inventory
    dto_in:
      name: ImportInventoryDto
      fields:
        - name: file_path
          type: string
        - name: skip_header
          type: boolean
    dto_out:
      name: ImportResultDto
      fields:
        - name: imported_count
          type: integer
        - name: error_messages
          type: string
          list: true
```

### DTO Field Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `name` | string | required | Field name (snake_case) |
| `type` | string | required | Field type (boolean, integer, float, string, uuid, datetime) |
| `list` | bool | false | Field is a list/array |
| `nullable` | bool | false | Field can be null/None |
