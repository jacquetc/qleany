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

To modify "Binder," you touch four directories. For a 17-entity project, Qleany v0 generated **1700+ c++ files across 500 folders**. Technically correct, practically unmaintainable.

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

To modify "Binder," you only touch one folder. It's easier to find code, understand features, and make changes. For the same 17-entity project, Qleany now generates **600 files across 80 folders**. Roughly, 33 files per entity instead of 90.

**Benefits:**
- **Discoverability** — Find all Binder code in one place
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

Web frameworks often provide architectural scaffolding (Rails, Django, Spring). Desktop frameworks like Qt provide widgets and signals, but few guidance on organizing a 50,000-line application.

Qleany fills that gap with an architecture that:
- Scales from small tools to large applications
- Integrates naturally with Qt's object model
- Supports undo/redo, a desktop-specific requirement
- Keeps related code together for solo developers and small teams
- Multiple UIs (Qt Widgets, QML, CLI) sharing the same core logic

For the complete file organization, see [Generated Infrastructure](generated-code.md#file-organization).

## Why this Matters for Mobile Apps

Mobile apps share many characteristics with desktop apps (see above), but have additional constraints:
- Rich UIs with complex interactions
- Need for offline functionality
- Local data storage with sync capabilities
- Performance constraints requiring efficient architecture

For the performance, since Qleany generates C++ and Rust, I think that it can be called performant enough for mobile apps. Mobile apps often require efficient memory usage and responsiveness, which C++ and Rust can provide.

Possibly, a Rust backend could be plugged into a mobile app developed with native technologies (Swift for iOS, Kotlin for Android) or cross-platform frameworks (Flutter, React Native). This way, the core logic benefits from Rust's performance and safety, while the UI is built with tools optimized for mobile platforms. 

> Since I don't develop for mobile platforms myself, I base my thoughts on common knowledge about mobile app development. If you have experience developing for mobile apps, please share your insights!

## Generate and Disappear

Qleany generates code, then gets out of your way. The output has no dependency on Qleany itself. Modify, extend, or delete the generated code freely. The generated code is yours — there's no runtime, no base classes to inherit from, no framework to learn.

## No Framework, No Runtime

Qleany generates plain Rust structs and C++ classes. There's no:
- Base class you must inherit from
- Trait you must implement for Qleany
- Runtime library to link against

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

## Code quality and "purity"

Some advanced developers may argue that the code is not very efficient, not “state-of-the-art”, be it in Rust or C++. Like someone called Steve said: “It’s not a bug, it’s a feature.” I choose to avoid writing anything too hard to wrap your head around. A developer with only a few years of experience in C++ or Rust would be able to understand the generated code.

It means for Rust:
- lifetimes only where the compiler requires them (no complex multi-lifetime scenarios), mostly deep inside the infrastructure
- no async/await
- generics only from standard library types (Result, Option, Vec) — no custom generic abstractions
- no unsafe code
- there is a bit too much of cloning around
- generated traits stay simple
- the only macro exists to help the developer with custom units of work

For C++/Qt:
- some C++20 aggregates and std::optional
- exceptions used for error handling
- async operations handled through QCoro where the event loop requires it
- no raw pointers, only smart pointers
- no multi-level inheritance, be it virtual or polymorphic
- a bit too much of copying around, but std::move is used deeper inside the infrastructure

I know how to use complex lifetimes, generics, async Rust, diamond inheritance with virtual, etc. I did it in professional projects, and I saw younger developers struggling with it. The borrow checker is brilliant, but watching someone spend three hours fighting lifetime annotations on code that just needs to clone a string taught me something.

It’s a trade-off between code approachability and performance. Qleany prioritizes code that intermediate developers can confidently modify over code that squeezes every last microsecond from the CPU. The generated code is clean, readable, and maintainable. Yes, there’s cloning where a senior developer would use borrowing.

And be real: you are writing C++ and Rust, which are among the fastest languages in the world. And you are not writing a game engine. Your application spends most of its time waiting for the user to click something or fetching from the database. The few microseconds lost to cloning a DTO are not your bottleneck. Your bottleneck is the junior developer who can’t figure out how to add a field to an entity because the code is too clever.

If you need every optimization, write your hot paths by hand. Profile first, then optimize what matters. The generator gives you a solid baseline that works and that your team can maintain. That’s the deal.

## Plugins

I add this little section about plugins too while I'm at it. Qt plugins especially. To paraphrase Uncle Bob: "UI is a detail, database is a detail", ... and plugins are details too. They can change without affecting the core business rules. The entities, use cases, don't care whether you're using a SQLite database or a JSON file. They don't care whether the UI is QML or something else. This is the same idea with plugins. Plugin realm is **outside** the core (entities and business rules).

In concrete terms, this means that the plugin system is implemented in the outermost layer (Frameworks & UI). The core application logic doesn't depend on plugins. Instead, plugins depend on the core application logic. This way, you can add, remove, or change plugins without affecting the core functionality of your application.

If I had to create an application using plugins, I would design entities dedicated to managing plugins and their data, a feature dedicated to plugins. Maybe a feature by plugin type to be compartmentalized. Consider these features/use cases as the API for plugins to interact with the core application. The core application would provide services and data to the plugins through these use cases, ensuring that plugins can operate independently of the core logic.

Also, I'd separate the plugins extending the UI from the plugins extending the backend logic. The UI plugins would be loaded and managed by the UI layer, while the backend plugins would exist in their own section, always in the outermost layer, separate from the UI. And all plugins can have access to the features/use cases dedicated to plugins.

## User settings and Configuration

This part may be obvious to most developers. Does the user settings/configuration belong to the core application logic? No, it doesn't. It belongs to the outermost layer (Frameworks & UI). The core application logic should be agnostic of how settings are stored or managed. The settings/configuration system should be implemented in the outer layer, allowing the core logic to remain unaffected by changes in how settings are handled.

You don't want the window geometry to be held in entities. Its place is in the UI layer. You don't want the theme preference to be held in use cases. Its place is in the UI layer too. The core application logic should focus on business rules and data management, while settings and configuration are handled separately in the outer layer.

Use cases must stay pure and repeatable. They should not depend on user-specific settings or configurations. If a use case needs to behave differently based on user settings, it should receive those settings as input parameters, rather than accessing them directly.: This keeps the use cases decoupled from the settings system, maintains their reusability, and keeps them testable.
