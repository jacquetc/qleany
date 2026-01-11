# Qleany

**Architecture scaffolding generator for C++20/Qt6 or Rust 2024 desktop applications and CLI tools.**

Building a desktop app in Qt or Rust? Not sure how to structure it beyond "put code in files"?
Qleany generates a complete architecture: controllers, repositories, DTOs, undo/redo, reactive models, GUI skeletons — organized by feature, ready to extend.

Define your entities and relationships in a YAML manifest. Qleany generates several hundred repetitive files — saving you conservatively several days of tedious, error-prone work. Get a working structure that scales.

Qleany follows Package by Feature (Vertical Slice Architecture) principles. Define your entities and features once, generate consistent scaffolding across Rust and C++/Qt.

## Documentation

| Document | Purpose |
|----------|---------|
| [Quick Start](docs/quick_start.md) | Step-by-step tutorial building a complete application |
| [Manifest Reference](docs/manifest-reference.md) | Entity options, field types, relationships, features and use cases |
| [Design Philosophy](docs/design-philosophy.md) | Clean Architecture background, package by feature, Rust module structure |
| [Regeneration Workflow](docs/regeneration-workflow.md) | How file generation works, what gets overwritten, files that must stay in sync |
| [Undo-Redo Architecture](docs/undo-redo-architecture.md) | Entity tree structure, undoable vs non-undoable, configuration patterns |
| [QML Integration](docs/qml-integration.md) | Reactive models, mocks, and event system for C++/Qt |
| [Generated Infrastructure](docs/generated-code.md) | Database layer, repositories, and file organization details |

New to Qleany? Start with the [Quick Start Guide](Quick_start.md), then return here for reference.

---

## Why Qleany

Writing CRUD operations, DTOs, repositories, undo/redo infrastructure, and reactive UI models is tedious. The patterns are well-understood, but implementing them consistently across a codebase takes time.

Qleany generates this scaffolding so you can focus on business logic. It targets a specific architectural style — Package by Feature with Clean Architecture principles — that works well for desktop applications and CLI tools.

**Generate and disappear:** Qleany generates code, then gets out of your way. The output has no dependency on Qleany itself — no runtime, no base classes to inherit from, no framework to learn. Modify, extend, or delete the generated code freely. The generated code is yours.

**What Qleany is not:**
- A framework (no runtime dependencies)
- A solution for web services or high-throughput systems

## Key Features

- **Complete CRUD scaffolding** — Controllers, DTOs, use cases, repositories per entity
- **GUI skeleton generation** — Ready-to-compile frontend code for QtQuick, QtWidgets, Kirigami, or combinations thereof
- **Undo/redo system** — Command-based with grouping, scopes, and failure strategies
- **Reactive QML models** — Auto-updating list models and single-entity wrappers (C++/Qt)
- **QML mocks** — JavaScript stubs for UI development without backend (C++/Qt)
- **Relationship management** — Junction tables with ordering, caching, cascade deletion
- **Event system** — Decoupled communication between features

---

## Is Qleany the Right Fit?

### When Qleany Makes Sense

**Data-centric desktop applications** that will grow in complexity over time. Think document editors, project management tools, creative applications, or anything where users manipulate structured data and expect undo/redo to work reliably.

**Complex CLI tools in Rust** — tools like `git` that manage structured data, have multiple subcommands, and need consistent internal architecture. Qleany itself is built this way: type `qleany -h` to see a CLI interface backed by the same architecture that powers its Slint GUI.

**Applications needing multiple Qt frontends** — if you need QtQuick, QtWidgets, KDE Widgets, or Kirigami (or any combination of them simultaneously), Qleany generates a ready-to-compile backend architecture that any of these frontends can consume. The generated controllers, repositories, and event system work identically regardless of which UI toolkit you choose.

**Solo developers or small teams** without established architectural patterns. Qt provides excellent widgets and signals, but no guidance on organizing a 30,000-line application. Qleany gives you that structure immediately, with patterns validated through real-world use in Skribisto.

**Projects that will grow incrementally** — the manifest-driven approach means you can define a new entity, regenerate the scaffolding, and immediately have a working controller, repository, DTOs, and use cases. The consistency this brings across your codebase is hard to achieve manually.

### When to Reconsider

For **simple utilities or single-purpose tools**, Qleany introduces more infrastructure than you need. If your application doesn't have complex entity relationships, doesn't need undo/redo, and won't grow significantly, a hand-written architecture may serve you better.

If you're working with a **team that already has established patterns**, introducing Qleany means everyone needs to learn its conventions. The generated code is readable and follows clear patterns, but it represents a specific way of doing things.

Qleany **targets desktop and CLI applications**. If you're building for the web, using Electron, or targeting mobile with Flutter, this isn't the right tool. Similarly, if you need high-throughput server-side processing, the patterns here are optimized for user interaction, not request-per-second performance.

### The Practical Test

If your project matches the profile, start by **generating the scaffolding for a small subset of your entities** and spend time reading through the generated code. Understand how the controllers wire to use cases, how the event system propagates changes, how the undo commands work. This investment of a few hours will tell you whether the patterns feel natural to your way of thinking.

The "generate and disappear" philosophy means you're not locked in. If you decide halfway through that you'd prefer a different approach, the generated code is yours to modify or replace.

---

## Target Languages

| Language | Standard | Database | Frontend |
|----------|----------|----------|----------|
| C++ | C++20 / Qt6 | SQLite | QtQuick, QtWidgets, Kirigami |
| Rust | Rust 2024 | redb | CLI, Slint (reference implementation) |

**Rust frontend examples (not generated, but working references):**
- **Slint UI**: [qleany/crates/slint_ui](https://github.com/jacquetc/qleany/tree/generator_in_rust/crates/slint_ui)
- **Tauri/React**: [qleany/crates/qleany-app](https://github.com/jacquetc/qleany/tree/885c3ac6fdf6f115aed2c5a30fd26b81e331b4dd/crates/qleany-app)

---

## Where to Get Qleany

| Source | Status |
|--------|--------|
| GitHub Releases | *Coming soon* |
| crates.io | *Coming soon* |
| PyPI (pip) | *Coming soon* |

For now, build from source (see below).

---

## Building and Running

### Prerequisites

- Rust (install via [rustup](https://rustup.rs/))

### Building Qleany

```bash
git clone https://github.com/jacquetc/qleany
cd qleany
cargo build --release
```

### Running the UI

```bash
cargo run --release
```

The Slint-based UI provides:
- Form-based manifest editing
- Entity and relationship management
- Selective file generation
- Code preview before writing

### CLI Usage

```bash
# Generate all files
qleany generate --manifest qleany.yaml --output ./src

# Generate to temp folder (recommended)
qleany generate --manifest qleany.yaml --output ./tmp/qleany-output

# Generate specific feature
qleany generate --manifest qleany.yaml --output ./src --feature work_management

# List files that would be generated
qleany list --manifest qleany.yaml
```

---

## Reference Implementation

[Skribisto](https://github.com/jacquetc/skribisto) (develop branch) is a novel-writing application built with Qleany-generated C++/Qt code. It demonstrates:

- Full entity hierarchy (Root → Work → Binder → BinderItem → Content)
- Complex relationships (ordered children, many-to-many tags)
- Feature orchestration (LoadWork, SaveWork with file format transformation)
- Reactive QML UI with auto-updating models
- Undo/redo across structural and content operations
- Crash recovery with ephemeral database

Skribisto serves as both proof-of-concept and template source for C++/Qt generation.

---

## Migration from v1

Qleany v1 (Python/Jinja2) generated pure Clean Architecture with strict layer separation. A 17-entity project produced 1700+ files across 500 folders.

v2 generates Package by Feature with pragmatic organization. The same project produces ~200 files with better discoverability.

**Breaking changes:**
- Manifest format changed (schema version 2)
- Output structure reorganized by feature
- Reactive models are new (list models, singles)

---

## Contributing

Qleany is developed alongside Skribisto. The architecture is stable, but templates are being extracted and refined.

To contribute:
1. Open an issue to discuss changes
2. Reference Skribisto patterns where applicable
3. Ensure changes work for both Rust and C++/Qt

## Support

**GitHub Issues** is the only support channel: [github.com/jacquetc/qleany/issues](https://github.com/jacquetc/qleany/issues)

Qleany is a personal project licensed under MPL-2.0. It's provided as-is, with no obligation to maintain, fix bugs, or add features. That said, thoughtful bug reports and contributions are welcome — just understand this is maintained in spare time alongside other projects.

## License

MPL-2.0
