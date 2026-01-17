# Regeneration Workflow

This document explains how Qleany handles file generation and what happens when you regenerate code.

## The Golden Rule

**Generated files are overwritten when you regenerate them.** Qleany does not merge changes or preserve modifications.

This is intentional. The workflow assumes you control what gets regenerated.

## Before You Generate

**Commit to Git first.** This isn't optional advice — it's how the tool is meant to be used. If something goes wrong, you can recover. If you accidentally overwrite modified files, you can restore them.

## Controlling What Gets Generated

### In the UI

The Generate tab shows all files that would be generated. You select which ones to actually write:

1. Click **List Files** to populate the file list
2. Use group checkboxes to select/deselect categories
3. Uncheck any files you've modified and want to keep
4. Click **Generate (N)** to write only selected files

### In the CLI

Inside the project folder, run:
```bash
# Generate all files (dangerous if you've modified any)
qleany generate

# Generate to temp folder first (safe)
qleany generate --temp

# Then compare and merge manually
diff -r ./temp/crates ./crates
```

## What Happens When You Regenerate

- **Selected files are overwritten** — Your modifications are lost
- **Unselected files are untouched** — Even if the manifest changed
- **No files are deleted** — If you rename an entity, the old files remain; clean them up manually

## Files That Must Stay in Sync

When you add or remove an entity, certain files reference all entities and must be regenerated together. If you've modified one of these files, you'll need to manually merge the changes.

### Rust

These files contain references to all entities:

| File                                         | Contains |
|----------------------------------------------|----------|
| `common/event.rs`                            | Event enum variants for all entities |
| `common/entities.rs`                         | Re-exports all entity structs |
| `common/direct_access/repository_factory.rs` | Factory methods for all repositories |
| `common/direct_access/setup.rs`              | Factory methods for all repositories |
| `common/direct_access.rs`                    | Module declarations for all entity repositories |
| `direct_access/lib.rs`                       | Module declarations for all entity features |

### C++/Qt

| File                                             | Contains |
|--------------------------------------------------|----------|
| `common/direct_access/repository_factory.h/.cpp` | Factory methods for all repositories |
| `common/direct_access/event_registry.h/.cpp`     | Event objects for all entities |
| `common/CMakeLists.txt`                          | Adds all entity source files to build |
| `direct_access/CMakeLists.txt`                   | Adds all entity source files to build |

If you modify one of these files and later add a new entity, you'll need to either:
- Regenerate the file and re-apply your modifications, or
- Manually add the new entity references yourself

## Using the Temp Folder

The safest workflow when you've modified generated files:

1. Check **in temp/** checkbox in the UI (or use `--temp` or ```--output ./whatever/` in CLI)
2. Generate all files to the temp location
3. Compare temp output against your current files:
   ```bash
   diff -r ./temp/crates ./crates
   ```
4. Manually merge changes you want to keep
5. Delete the temp folder

This manual merge is the cost of customization. For files you modify heavily, consider whether the customization belongs in a separate file that won't conflict with generation.

## Practical Guidelines

### Files you'll typically regenerate freely

These are pure scaffolding with no business logic:

- Entity structs (`common/entities/`)
- DTOs (`dtos.rs`, `dtos.h`)
- Repository implementations
- Table/cache definitions
- Event classes

### Files you'll typically modify and protect

These contain your custom code:

- Use case implementations (your business logic)
- Controllers (if you add custom endpoints)
- Main entry point (`main.rs`, `main.cpp`)

### Files that aggregate others

These need careful handling — regenerate them when adding entities, but be aware they may need manual merging:

- Module declarations (`lib.rs`, feature exports)
- Factory classes
- Event registries

## When You Rename an Entity

Qleany doesn't track renames. If you rename `Car` to `Vehicle`:

1. Update the manifest with the new name
2. Generate the new `Vehicle` files
3. **Manually delete** the old `Car` files
4. Update any code that referenced `Car`

The old files won't be removed automatically because Qleany never deletes files.

## When Templates Improve

When Qleany's templates are updated (new features, bug fixes, better patterns):

1. Generate to temp folder with the new version
2. Compare against your existing generated files
3. Decide which improvements to adopt
4. For files you haven't modified: regenerate directly
5. For files you've modified: merge manually or regenerate and re-apply your changes

The manifest remains your source of truth. The same manifest with improved templates produces better output.
