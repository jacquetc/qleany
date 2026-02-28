# Migration Guide

This document covers breaking changes between manifest schema versions and how to upgrade.

---

## Schema v2 to v3

**Qleany version**: v1.0.29

### What changed

The `allow_direct_access` entity property has been removed. Every entity that isn't heritage-only now always gets its `direct_access/` files generated.

### Reasons for the change

The direct_access/ is an internal API. `allow_direct_access: true` skipped generation of the files for an entity. Yet, this entity could have needed to offer a list_model or a single model, which wouldnt be possible without direct_access/ files.
So, from now on, all non-heritage entities always get their `direct_access/` files generated. At compilation time, unused C++ functions (static libraries) are stripped from the binary. Same for Rust. In shared C++ libraries, C++ unused functions are compiled, yet the overweight is negligible.

### Automatic migration

Qleany auto-migrates v2 manifests on load. When you open a v2 manifest, the migrator strips all `allow_direct_access` fields and bumps the version to 3 before validation. No manual editing is required to load an old manifest.

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
