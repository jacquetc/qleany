# Migration Guide

This document covers breaking changes between manifest schema versions and how to upgrade.

---

## v1.5.0 to v1.5.3 — Error handling and robustness improvements

**Qleany version**: v1.5.1 through v1.5.3

### What changed

No manifest schema changes. These are generated code improvements that affect regenerated projects.

### Error handling (v1.5.1–v1.5.2)

- **Transactions**: `get_read_transaction()` and `get_write_transaction()` now return `Result` instead of panicking on wrong transaction type or consumed state. `commit()`, `rollback()`, `create_savepoint()`, and `restore_to_savepoint()` return descriptive errors instead of panicking on double-commit or missing `begin_transaction()`.
- **Repository factory**: Factory functions return `Result`, so all unit of work call sites must use `?` to propagate errors. If you have custom UoW implementations, update repository creation calls from `repository_factory::write::create_*_repository(transaction)` to `repository_factory::write::create_*_repository(transaction)?`.
- **Undo/redo**: `begin_composite()` now returns `Result<()>` instead of panicking on mismatched stack IDs. `cancel_composite()` now undoes any already-executed sub-commands before clearing state. Failed `undo()` and `redo()` operations re-push the command to its original stack instead of dropping it.
- **Table constraints**: One-to-one constraint violations return `RepositoryError::ConstraintViolation` instead of panicking.
- **New error variants**: `RepositoryError` gains `ConstraintViolation(String)` and `Other(anyhow::Error)`.
- **Proc macros**: `#[macros::uow_action]` with missing arguments now emits a compile error instead of panicking.
- **DTO enums**: Enum imports in generated DTO files are now `pub use` instead of `use`, making them accessible to external crates.

### Event loop and long operations (v1.5.3)

- **Event loop**: `start_event_loop` now returns `thread::JoinHandle<()>` and uses `recv_timeout(100ms)` so the stop signal is checked even when no events arrive. This fixes unresponsive shutdown.
- **Long operations**: A `lock_or_recover` helper handles mutex poisoning gracefully in `LongOperationManager` and `OperationHandle`, replacing all `.lock().unwrap()` calls.

### Mobile bridge (v1.5.1)

- **Feature method naming**: Feature use case methods now include the feature prefix (e.g., `handling_manifest_save()` instead of `save()`). Swift/Kotlin async wrappers follow suit (`handlingManifestSaveAsync()`).
- **Cross-module types**: A `mobile_types` module re-exports entity types across command modules.
- **Entity conversions**: `From<Entity> for MobileEntityDto` and reverse conversions are now generated.

### How to upgrade

1. Regenerate your project's infrastructure files (nature: Infra) to pick up the new error handling patterns.
2. If you have **custom UoW implementations** (feature use cases), update:
   - Replace `.take().unwrap()` on transaction `Option`s with `.take().ok_or_else(|| anyhow!("No active transaction"))?`
   - Add `?` after `repository_factory::write::create_*_repository(...)` and `repository_factory::read::create_*_repository(...)` calls
   - Update `begin_composite()` call sites to handle the new `Result<()>` return type
3. If you use the **mobile bridge**, update Swift/Kotlin call sites to use the new feature-prefixed method names.

### Cargo workspace dependencies

Generated `Cargo.toml` templates now use workspace-level dependency declarations. Regenerate your Cargo files to pick up this change.

---

## Schema v4 to v5 — `is_list` for entity fields

**Qleany version**: v1.4.0

### What changed

Entity fields now support `is_list: true`, the same way DTO fields already did. This allows declaring list/array fields of primitive types (string, integer, uinteger, float, boolean, uuid, datetime) directly on entities.

### Constraints

- `is_list` cannot be used with `entity` or `enum` field types.
- `is_list` and `optional` are mutually exclusive on the same field.

### Example

```yaml
entities:
  - name: Project
    inherits_from: EntityBase
    fields:
      - name: title
        type: string
      - name: labels
        type: string
        is_list: true
      - name: scores
        type: float
        is_list: true
```

### Automatic migration

Qleany auto-migrates v2+ manifests on load. When you open a v4 manifest, the migrator bumps the version to 5 before validation. No manual editing is required.

### Manual migration

Change the schema version:

```yaml
schema:
  version: 5    # was 4
```

No other manifest changes are needed — `is_list` defaults to `false` when omitted.

### Storage

- **Rust**: list fields are stored as `Vec<T>` in the entity struct, serialized via postcard in redb.
- **C++/Qt**: list fields are stored as `QList<T>` in the entity struct, serialized as JSON arrays in SQLite TEXT columns.

---

## Schema v3 to v4

**Qleany version**: v1.0.31

### What changed

The `validator` use case property has been removed.

### Reasons for the change

Validation is the responsibility of the developer.

### Automatic migration

Qleany auto-migrates v2+ manifests on load. When you open a v3 manifest, the migrator strips all `validator` fields and bumps the version to 4 before validation. No manual editing is required to load an old manifest.

If you save the manifest afterwards (from the UI), the file is written as v4.

From the CLI, it's the same: if you run `qleany generate` on a v3 manifest, it will be auto-migrated to v4 before generation. To only migrate the manifest, use `qleany migrate` instead.


### Manual migration

If you prefer to update the file yourself:

1. Change the schema version:

```yaml
schema:
  version: 4    # was 3
```

2. Remove every `validator:` line from your entities:

```diff
 feature:
   - name : my_feature
     use_cases:
       - name: my_use_case
-        validator: true
```

No other manifest changes are needed.

### Behavioral differences

None

### Code generation templates

Never used.

---

## Schema v2 to v3

**Qleany version**: v1.0.29

### What changed

The `allow_direct_access` entity property has been removed. Every entity that isn't heritage-only now always gets its `direct_access/` files generated.

### Reasons for the change

The direct_access/ is an internal API. `allow_direct_access: true` skipped generation of the files for an entity. Yet, this entity could have needed to offer a list_model or a single model, which wouldnt be possible without direct_access/ files.
So, from now on, all non-heritage entities always get their `direct_access/` files generated. At compilation time, unused C++ functions (static libraries) are stripped from the binary. Same for Rust. In shared C++ libraries, C++ unused functions are compiled, yet the overweight is negligible.

### Automatic migration

Qleany auto-migrates v2+ manifests on load. When you open a v2 manifest, the migrator strips all `allow_direct_access` fields and bumps the version to 3 before validation. No manual editing is required to load an old manifest.

If you save the manifest afterwards (from the UI), the file is written as v3.

From the CLI, it's the same: if you run `qleany generate` on a v2 manifest, it will be auto-migrated to v3 before generation. To only migrate the manifest, use `qleany migrate` instead.

### Manual migration

If you prefer to update the file yourself:

1. Change the schema version:

```yaml
schema:
  version: 3    # was 2
```

2. Remove every `allow_direct_access:` line from your entities:

```diff
 entities:
   - name: EntityBase
     only_for_heritage: true
-    allow_direct_access: false
     fields:
       ...

   - name: Car
     inherits_from: EntityBase
-    allow_direct_access: true
     fields:
       ...
```

That's it. No other manifest changes are needed.

### Behavioral differences

| Before (v2) | After (v3) |
|---|---|
| `allow_direct_access: false` hid an entity from `direct_access/` generation | Use `only_for_heritage: true` instead (which also skips generation) |
| `allow_direct_access: true` (the default) generated files | All non-heritage entities always generate files |

If you had entities with `allow_direct_access: false` that were **not** `only_for_heritage: true`, those entities will now generate `direct_access/` files. If you don't want that, mark them `only_for_heritage: true`.

### Code generation templates

Tera templates that referenced `ent.inner.allow_direct_access` now use `not ent.inner.only_for_heritage`. If you've written custom templates that check this field, update them accordingly.
