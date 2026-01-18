[![crates.io](https://img.shields.io/crates/v/qleany?style=flat-square&logo=rust)](https://crates.io/crates/qleany)
[![API](https://docs.rs/qleany/badge.svg)](https://docs.rs/qleany)
[![license](https://img.shields.io/badge/license-Mozilla_Publc_License--2.0-blue?style=flat-square)](#license)
![quality](https://img.shields.io/github/actions/workflow/status/jacquetc/qleany/ci.yml)
![Lines of code](https://img.shields.io/tokei/lines/github.com/jacquetc/qleany)
# Qleany

**Architecture scaffolding generator for C++20/Qt6 or Rust 2024 applications — desktop, mobile, and CLI.**

> **No framework. No runtime. No Qleany dependencies in your code.**
> 
> The generated code is yours — plain C++ classes and Rust structs using standard libraries (Qt, QCoro, redb). Modify it, extend it, delete Qleany afterward. You're not adopting a framework that will haunt your codebase for years or burn you when the maintainer moves on.

Building an app in Qt or Rust? Not sure how to structure it beyond "put code in files"?
Qleany generates a complete architecture: controllers, repositories, DTOs, undo/redo, reactive models, GUI skeletons — organized by feature, ready to extend.

Define your entities and relationships in a YAML manifest or in its dedicated UI tool. Qleany generates several hundred repetitive files — saving you conservatively several days of tedious, error-prone work. Get a working structure that scales from a simple CLI tool to a full-featured application with desktop and mobile variants.

Qleany follows Package by Feature (Vertical Slice Architecture) principles. Define your entities and features once, generate consistent scaffolding across Rust and C++/Qt with baking-in (empty) UIs. The generated code aims to be readable, idiomatic, and easy to modify, more than sophisticated and abstract.

Qleany's own Slint-based UI is built using the same patterns it generates.

## Documentation

| Document                                                 | Purpose |
|----------------------------------------------------------|---------|
| [Quick Start](docs/quick_start.md)                       | Step-by-step tutorial building a complete application |
| [Manifest Reference](docs/manifest-reference.md)         | Entity options, field types, relationships, features and use cases |
| [Design Philosophy](docs/design-philosophy.md)           | Clean Architecture background, package by feature, Rust module structure |
| [Regeneration Workflow](docs/regeneration-workflow.md)   | How file generation works, what gets overwritten, files that must stay in sync |
| [Undo-Redo Architecture](docs/undo-redo-architecture.md) | Entity tree structure, undoable vs non-undoable, configuration patterns |
| [QML Integration](docs/qml-integration.md)               | Reactive models, mocks, and event system for C++/Qt |
| [Generated Infrastructure](docs/generated-code.md)       | Database layer, repositories, and file organization details |
| [Troubleshooting](docs/troubleshooting.md)               | Common issues and how to fix them |

New to Qleany? Start with the [Quick Start Guide](docs/quick_start.md), then return here for reference.

## Key Features

- **Complete CRUD scaffolding** — Controllers, DTOs, use cases, repositories per entity
- **GUI skeleton generation** — Ready-to-compile frontend code for QtQuick, QtWidgets, Kirigami, or combinations thereof
- **Undo/redo system** (optional) — Command-based with grouping, scopes, and failure strategies; async execution with QCoro coroutines in C++/Qt, synchronous in Rust
- **Reactive QML models** — Auto-updating list models and single-entity wrappers (C++/Qt)
- **QML mocks** — JavaScript stubs for UI development without backend (C++/Qt)
- **Relationship management** — Junction tables with ordering, caching, cascade deletion
- **Event system** — Decoupled communication between features

---

## Is Qleany the Right Fit?

### When Qleany Makes Sense

**Data-centric applications** that will grow in complexity over time. Think document editors, project management tools, creative applications, or anything where users manipulate structured data and expect undo/redo to work reliably. This applies equally to desktop and mobile — a note-taking app on Plasma Mobile has the same architectural needs as one on desktop Linux.

**Complex CLI tools in Rust** — tools like `git` that manage structured data, have multiple subcommands, and need consistent internal architecture. Qleany itself is built this way: type `qleany -h` to see a CLI interface backed by the same architecture that powers its Slint GUI.

**Applications targeting multiple platforms** — if you're building for desktop Linux and want to support Plasma Mobile or Ubuntu Touch with the same codebase, Qleany's generated backend works identically across all of them. Write your business logic once, swap UI frontends as needed.

**Applications needing multiple Qt frontends** — if you need QtQuick, QtWidgets, KDE Widgets, or Kirigami (or any combination of them simultaneously), Qleany generates a ready-to-compile backend architecture that any of these frontends can consume. The generated controllers, repositories, and event system work identically regardless of which UI toolkit you choose.

**Solo developers or small teams** without established architectural patterns. Qt provides excellent widgets and signals, but little guidance on organizing a 30,000-line application (or I couldn't find it). Qleany gives you that structure immediately, with patterns validated through real-world use in Skribisto.

**Projects that will grow incrementally** — the manifest-driven approach means you can define a new entity, regenerate the scaffolding, and immediately have a working controller, repository, DTOs, and use cases. The consistency this brings across your codebase is hard to achieve manually.

### When to Reconsider

For **simple utilities or single-purpose tools**, Qleany introduces more infrastructure than you need. If your application doesn't have complex entity relationships, doesn't need undo/redo, and won't grow significantly, a hand-written architecture may serve you better.

If you're working with a **team that already has established patterns**, introducing Qleany means everyone needs to learn its conventions. The generated code is readable and follows clear patterns, but it represents a specific way of doing things. Discuss with your team before adopting it. Do not antagonize existing workflows. A better, professional, approach may be to present Qleany's patterns with some open-minded senior devs of your team. Even if they don't want to use Qleany - **which is fairly expected** - they may appreciate some of its ideas and adapt them to their existing architecture. They may even want to use Qleany for prototyping or side projects, or scaffold new subsystems of an existing project without disrupting the main architecture.

Qleany **targets native applications**. If you're building for the web, using Electron, this isn't the right tool. Similarly, if you need high-throughput server-side processing, the patterns here are optimized for user interaction, not request-per-second performance. If you are targeting Android/iOS with Flutter or React Native, maybe the Rust as a backend option is an interesting choice, but the C++/Qt generation is not.

### The Practical Test

If your project matches the profile, start by **generating the scaffolding for a small subset of your entities** and spend time reading through the generated code. Understand how the controllers wire to use cases, how the event system propagates changes, how the undo commands work. This investment of a few hours will tell you whether the patterns feel natural to your way of thinking.

The "generate and disappear" philosophy means you're not locked in. If you decide halfway through that you'd prefer a different approach, the generated code is yours to modify or replace.

---

## Why Qleany

I wrote [Skribisto](https://github.com/jacquetc/skribisto), a novel-writing application in Qt. Four times. In different languages. Each time, I hit the same wall: spaghetti code and structural dead-ends that made adding features painful and eventually impossible without rewriting half the codebase.

After the third rewrite, I studied architecture patterns seriously. Clean Architecture (Robert C. Martin) clicked — the separation of concerns, the dependency rules, the testability. But implementing it by hand meant writing the same boilerplate over and over: repositories, DTOs, use cases, controllers. So I wrote templates. The templates grew into a generator. The generator needed a manifest file.

**Qleany v0** was Python/Jinja2 generating C++/Qt code following pure Clean Architecture. It worked, but the tradeoffs were hard to miss: a 17-entity project produced 1700+ files across 500 folders. Some of my early design choices were dubious in hindsight.

**Qleany v1** is a ground-up rewrite in Rust, aiming to fix those problems while adopting a more robust and easier-to-maintain language. Less sophisticated, more pragmatic, architecture. It adopts Package by Feature (a.k.a. Vertical Slice Architecture) instead of strict layer separation — same Clean Architecture principles, but organized by what the code does rather than what layer it belongs to. The same manifest now generates both C++/Qt and Rust code.

This is the tool I needed when I started Skribisto. If it saves someone else from their fourth rewrite, it's done its job.

---

## Target Platforms

| Language | Standard | internal database | Frontend Options                                       |
|----------|----------|-------------------|--------------------------------------------------------|
| C++ | C++20 / Qt6 | SQLite            | QtQuick, QtWidgets, Kirigami, Ubuntu Touch Components (Lomiri) |
| Rust | Rust 2024 | redb              | CLI, Slint                                             |

**Supported deployment targets for C++/Qt:**
- Desktop Linux (KDE Plasma, GNOME, etc.)
- Plasma Mobile
- Ubuntu Touch
- Windows, macOS (Qt's cross-platform support)

**Supported deployment targets for Rust:**
- All the usual Rust targets (Linux, Windows, macOS, etc.)

The generated backend is platform-agnostic. Your business logic, repositories, and controllers work identically whether you're building a desktop app, a mobile app, or both from the same codebase. Only the UI layer differs.

Also, the internal database choice (SQLite for C++/Qt, redb for Rust) is abstracted behind repositories. You can swap out the database implementation if needed, though SQLite and redb are solid choices for most applications.

**Rust frontend examples (working references):**
- **Slint UI**: [qleany/crates/slint_ui](https://github.com/jacquetc/qleany/tree/generator_in_rust/crates/slint_ui)
- **Tauri/React**: [qleany/crates/qleany-app](https://github.com/jacquetc/qleany/tree/885c3ac6fdf6f115aed2c5a30fd26b81e331b4dd/crates/qleany-app)

I'm no web developer, and Tauri/React is not my forte. But if you want to build a web-based frontend with Rust backend generated by Qleany, this is a starting point.

---

## Where to Get Qleany

| Source | Status |
|--------|--------|
| GitHub Releases | *Coming soon* |
| crates.io | *Coming soon* |
| PyPI (pip) | *Coming soon* |

For now, build from source (see below).

---

## License

Qleany (the generator) is licensed under MPL-2.0. See the [LICENSE](LICENSE) file for details. It is compatible with
both open source and proprietary projects.

**Generated code**: This license does not cover the code generated by Qleany. You are free to use, modify,
and distribute generated code under any license of your choice, including proprietary licenses.

### MPL-2.0 License Summary

- ✅ Commercial use
- ✅ Modification
- ✅ Distribution
- ✅ Patent use
- ✅ Private use
- ❗ License and copyright notice required
- ❗ Disclose source (for modified files)
- ❗ Same license (for modified files)
- ❌ Liability
- ❌ Warranty
- ❌ Trademark use

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

For more details, see the [Quick Start Guide](docs/quick_start.md).

### CLI Usage

```bash

# Show help
qleany -h

# Show an option help
qleany generate -h

# new qleany.yaml manifest
qleany new --language cpp-qt (or rust)

# Generate all files
qleany generate

# Dry run (list files that would be generated without writing)
qleany generate --dry-run

# Dry run (list files that would be generated without writing)
qleany generate --dry-run entity MyEntity

# Generate to temp folder (recommended)
qleany generate --temp

# Generate specific feature
qleany generate feature my_feature_name

# List files that would be generated
qleany list

# List features that would be generated
qleany list features
```

---

## Reference Implementation

[Skribisto](https://github.com/jacquetc/skribisto) (develop branch) is a novel-writing application built with Qleany-generated C++/Qt code. It demonstrates:

- Full entity hierarchy (Root → Work → Binder → BinderItem → Content)
- Complex relationships (ordered children, many-to-many tags)
- Feature orchestration (LoadWork, SaveWork with file format transformation)
- Reactive QML UI with auto-updating models
- Undo/redo across structural and content operations

Skribisto serves as both proof-of-concept and template source for C++/Qt generation.

---

## Migration from v1

Qleany v1 (Python/Jinja2) generated pure Clean Architecture with strict layer separation. A 17-entity project produced 1700+ files across 500 folders.

v2 generates Package by Feature with pragmatic organization. The same project produces ~600 files across ~80 folders with better discoverability.

**Breaking changes:**
- Manifest format changed (schema version 2)
- Output structure reorganized by feature
- Reactive models are new (list models, singles)

Bottom line: from v0 to v1, there is no automated migration path. You must regenerate from your manifest and manually port any custom code.

---

## Contributing

Qleany is developed alongside Skribisto. The architecture is stable, but templates are being extracted and refined.

To contribute:
1. Open an issue to discuss changes
2. Reference Skribisto (c++/Qt) or Qleany (Rust) patterns where applicable
3. Ensure changes work for both Rust and C++/Qt
4. Don't forget to sign off your commits (`commit -s`)

Please read the [CONTRIBUTING.md](CONTRIBUTING.md) file.

## Support

**GitHub Issues** is the only support channel: [github.com/jacquetc/qleany/issues](https://github.com/jacquetc/qleany/issues)

Qleany is a personal project licensed under MPL-2.0. It is actively used in Skribisto's development, so improvements flow from real-world needs. Bug reports and contributions are welcome, though response times vary as this is maintained alongside other projects.

## About

Qleany is developed and maintained by FernTech.

## License

Copyright (c) 2025-2026 FernTech
Licensed under [MPL-2.0](LICENSE)
