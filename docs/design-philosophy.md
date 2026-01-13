# Design Philosophy

This document explains the architectural principles behind Qleany and why it generates code the way it does.

## What is Clean Architecture?

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
- **Features** — Groupings of related use cases and entities
- **Use Cases** — Single-purpose operations encapsulating business logic
- **DTOs** — Data transfer objects crossing layer boundaries
- **Repositories** — Abstractions over data access
- **Dependency Inversion** — High-level modules don't depend on low-level modules

## The Problem with Pure Clean Architecture

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

## Package by Feature (a.k.a. Vertical Slice Architecture)

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
- **Fewer files** — Same 17-entity project produces ~600 files across ~80 folders
- **Easier onboarding** — New developers understand features, not layers

### Why Vertical Slices?

The term comes from visualizing your application as a layered cake. A horizontal slice would be one entire layer (all controllers, or all repositories). A vertical slice cuts through all layers for one feature — from UI down to database, but only for that specific capability.

Each slice is relatively self-contained. You can understand, modify, and test the Binder feature without understanding how Events or Tags work internally. This isolation makes onboarding easier and reduces the blast radius of changes.

### What We Keep from Clean Architecture

- Dependency direction (UI → Controllers → Use Cases → Repositories → Database)
- Use cases as the unit of business logic
- DTOs at boundaries
- Repository pattern for data access
- Testability through clear interfaces

### What We Drop

- Strict layer-per-folder organization
- Separate "domain" module (entities live in `common`)
- Interface-for-everything (only where it aids testing)

## Why This Matters for Desktop Apps

Web frameworks often provide architectural scaffolding (Rails, Django, Spring). Desktop frameworks like Qt provide widgets and signals, but no guidance on organizing a 50,000-line application.

Qleany fills that gap with an architecture that:
- Scales from small tools to large applications
- Integrates naturally with Qt's object model
- Supports undo/redo, a desktop-specific requirement
- Keeps related code together for solo developers and small teams

For the complete file organization, see [Generated Infrastructure](generated-code.md#file-organization).

## Generate and Disappear

Qleany generates code, then gets out of your way. The output has no dependency on Qleany itself. Modify, extend, or delete the generated code freely. The generated code is yours — there's no runtime, no base classes to inherit from, no framework to learn.

## No Framework, No Runtime

Qleany generates plain Rust structs and C++ classes. There's no:
- Base class you must inherit from
- Trait you must implement for Qleany
- Runtime library to link against
- Macro that transforms your code

The generated code uses standard libraries (redb for Rust, Qt for C++) but has no Qleany-specific dependencies. If you decide to stop using Qleany, the generated code continues to work unchanged.

## Manifest as Source of Truth

The `qleany.yaml` manifest defines your architecture. It's:
- **Human-readable** — Edit it directly when the UI is inconvenient
- **Version-controllable** — Diff changes, review in PRs
- **Portable** — Share between team members, regenerate on any machine

The manifest describes *what* you want. Qleany figures out *how* to generate it. When templates improve, regenerate from the same manifest to get updated code.

## Rust Module Structure

Qleany generates Rust code using the modern module naming convention. Instead of:

```
direct_access/
└── car/
    └── mod.rs      # Old style
```

Qleany generates:

```
direct_access/
├── car.rs          # Module file
└── car/            # Submodules folder
    ├── controller.rs
    ├── dtos.rs
    └── use_cases.rs
```

This follows Rust's recommended practice since the 2018 edition, avoiding the proliferation of `mod.rs` files that makes navigation difficult.
