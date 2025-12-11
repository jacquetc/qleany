# Qleany v2 - Architecture Generator (Rust Branch)

> ⚠️ **This branch is under active development.** This README documents the current state and goals. The main branch contains the stable v1 (Python-based) generator.

## What's Happening Here

Qleany is being rewritten from the ground up:

| Aspect | v1 (main branch) | v2 (this branch) |
|--------|------------------|------------------|
| Generator | Python + Jinja2 | Rust + Tera |
| UI | PySide | Slint |
| Architecture | Pure Clean Architecture | Package by Feature / Vertical Slice |
| Output | 1700+ files for 17 entities | Pragmatic, maintainable structure |

## Why the Rewrite

Pure Clean Architecture generated too many files. A simple project with 17 entities produced 1700 C++ files across 500 folders. Technically correct, practically unmaintainable for solo developers or small teams.

**Package by Feature** (also known as Vertical Slice Architecture) keeps the benefits of Clean Architecture—separation of concerns, testability, clear boundaries—while organizing code by feature rather than by layer. The result: fewer files, better discoverability, easier maintenance.

## Current Status

### Done

- [x] Rust generator core
- [x] Rust code generation (self-hosting: the generator generates its own structure)
- [x] Slint UI data binding proof-of-concept
- [x] C++/Qt proof-of-concept ([Skribisto develop branch](https://github.com/jacquetc/skribisto/tree/develop))

### In Progress

- [ ] Slint UI for manifest editing
- [ ] C++/Qt template extraction from Skribisto PoC

### Planned

- [ ] QML frontend generation (with mocks)
- [ ] Kirigami frontend generation
- [ ] Python code generation
- [ ] Rust → C API generation (for FFI)
- [ ] Rust → Python bindings (PyO3)

## Target Languages

| Language | Backend | Frontend | Status |
|----------|---------|----------|--------|
| C++/Qt | ✓ | QML | Primary target |
| Rust | ✓ | — | Done |
| Python | Planned | — | Future |

## Design Decisions

### No Compilation-Ready Output

v2 does not aim to generate immediately compilable code. Instead, it generates:

- Correct file structure
- Correct architecture (vertical slices)
- Boilerplate and scaffolding
- Placeholders for business logic

The developer fixes includes and types in minutes instead of writing boilerplate for hours. This tradeoff avoids the complexity of predicting every project's include paths, namespace conventions, and error handling preferences.

### Generate and Disappear

Qleany is not a framework. It generates code, then gets out of your way. No runtime dependencies on Qleany itself. You're free to modify, extend, or ignore the generated code.

### Manifest-Driven

Everything is defined in `qleany.yaml`. The Slint UI provides form-based editing of this manifest and selective file generation.

## Architecture Overview

Qleany generates this structure for **C++/Qt6**, **Rust**, and **Python** projects.

Generated projects follow this structure (Rust example):

```
crates/
├── common/                         # Shared across all features
│   └── src/
│       ├── database/               # DB context, transactions
│       ├── direct_access/          # Per-entity repository + table
│       │   ├── {entity}/
│       │   │   ├── {entity}_repository.rs
│       │   │   └── {entity}_table.rs
│       │   └── repository_factory.rs
│       ├── entities.rs
│       ├── event.rs
│       └── undo_redo.rs
├── direct_access/                  # CRUD feature (auto-generated)
│   └── src/
│       └── {entity}/               # Vertical slice per entity
│           ├── {entity}_controller.rs
│           ├── dtos.rs
│           ├── units_of_work.rs
│           └── use_cases/
│               ├── create_{entity}_uc.rs
│               ├── get_{entity}_uc.rs
│               ├── update_{entity}_uc.rs
│               └── remove_{entity}_uc.rs
├── {custom_feature}/               # Your features (manual use cases)
│   └── src/
│       ├── {feature}_controller.rs
│       ├── dtos.rs
│       ├── units_of_work/
│       └── use_cases/
├── macros/                         # Proc macros
├── slint_ui/                       # GUI (or other frontend)
└── cli/                            # CLI entry point
```

### Key Patterns (All Languages)

- **Threaded Undo/Redo System**: All tasks run through a central undo/redo system managing execution and history
- **Units of Work**: Own database transactions and repository lifecycle
- **Repository Factory**: Returns owned instances, no cross-thread sharing
- **Feature Events**: Decoupled communication between features
- **Command Merging**: Consecutive similar commands can merge (e.g., typing)
- **Composite Commands**: Group multiple commands as a single undoable unit

### Key Patterns (Rust)

- **Synchronous Commands**: Undo/redo commands execute synchronously
- **Long Operation Manager**: Threaded execution with progress tracking and cancellation for heavy tasks
- **redb**: Embedded key-value database

### Key Patterns (C++/Qt)

- **Async Undo/Redo with QCoro**: Commands execute asynchronously using C++20 coroutines
- **Scoped Stacks**: Separate undo/redo stacks per scope (Work, Content, Settings, Custom)
- **Query Handler**: Async queries separate from commands
- **Group Commands with Failure Strategies**: `StopOnFailure`, `ContinueOnFailure`, `RollbackAll`, `RollbackPartial`
- **Result<T> with Error Categories**: Typed errors (`ValidationError`, `DatabaseError`, `TimeoutError`, etc.) and severity levels
- **Service Locator**: Composition root wires dependencies (required for QML integration)
- **QML with JavaScript Mocks**: Generated `mock_imports/` folder with JS stubs for UI prototyping without backend (`SKR_BUILD_WITH_MOCKS`)
- **SQLite**: Database backend

## Building the Generator

```bash
cd qleany
cargo build --release
```

## Running the UI

```bash
cargo run --release
```

## Real-World Usage

Qleany v2 architecture is being developed alongside [Skribisto](https://github.com/jacquetc/skribisto), a writing application. The `develop` branch of Skribisto serves as the proof-of-concept and template source for C++/Qt generation.

## Relationship to v1

The v1 generator (Python/PySide) on the main branch remains functional for existing users. v2 is a parallel effort that will eventually replace it. Migration path TBD.

## Contributing

This branch is in flux. If you're interested in contributing, open an issue first to discuss.

## License

MPL-2.0 (unchanged from v1)