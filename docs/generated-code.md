# Generated Infrastructure

This document details the infrastructure Qleany generates for each target language. It's reference material — read it when you need to understand, extend, or debug the generated code, not as a getting-started guide.

## C++/Qt Infrastructure

### Database Layer

**DbContext / DbSubContext**: Connection pool with scoped transactions. Each unit of work owns a `DbSubContext` providing `beginTransaction`, `commit`, `rollback`, and savepoint support.

```cpp
// Usage in a use case
auto subContext = m_dbContext->createSubContext();
subContext->beginTransaction();
try {
    // ... operations ...
    subContext->commit();
} catch (...) {
    subContext->rollback();
    throw;
}
```

**Repository Factory**: Creates repositories bound to a specific `DbSubContext`. Returns owned instances (`std::unique_ptr`) — no cross-thread sharing.

```cpp
auto repo = m_repositoryFactory->createCarRepository(subContext);
auto car = repo->get(carId);
```

**Table Cache / Junction Cache**: Thread-safe, time-expiring (30 minutes), invalidated at write time. Improves performance for repeated queries within a session.

### SQLite Configuration

SQLite with WAL mode, optimized for desktop writing applications:

```sql
PRAGMA journal_mode=WAL;
PRAGMA synchronous=NORMAL;
PRAGMA cache_size=20000;        -- 20MB
PRAGMA mmap_size=268435456;     -- 256MB
```

### Ephemeral Database Pattern

The internal database lives in `/tmp/`, decoupled from user files:

1. **Load**: Transform file → internal database
2. **Work**: All operations against ephemeral database
3. **Save**: Transform internal database → file
4. **Crash Recovery**: Detect orphaned database, offer recovery

This pattern separates the user's file format from internal data structures. Your `.myapp` file can be JSON, XML, SQLite, or any format — the internal database remains consistent.

### Async Undo/Redo with QCoro

Commands execute asynchronously using C++20 coroutines:

```cpp
QCoro::Task<Result> CreateCarCommand::execute() {
    auto result = co_await m_useCase->execute(m_dto);
    if (result.isSuccess()) {
        m_createdId = result.value().id;
    }
    co_return result;
}

QCoro::Task<Result> CreateCarCommand::undo() {
    co_await m_removeUseCase->execute(m_createdId);
    co_return Result::success();
}
```

Features:
- Scoped stacks (per-document undo)
- Command grouping (multiple operations as one undo step)
- Failure strategies (rollback group on failure, or continue)

### Event Registry

QObject-based event dispatch for reactive updates:

```cpp
// Emitting (in repository)
emit m_events->updated(entity.id());

// Subscribing (in model)
connect(m_events, &CarEvents::updated, this, &CarListModel::onCarUpdated);
```

The registry is a singleton accessible from both C++ and QML, providing consistent reactivity throughout the application.

---

## Rust Infrastructure

### redb Backend

Embedded key-value storage with ACID transactions:

```rust
// Table definitions (generated)
const CARS_TABLE: TableDefinition<u64, Car> = TableDefinition::new("cars");

// Usage
let read_txn = db.begin_read()?;
let table = read_txn.open_table(CARS_TABLE)?;
let car = table.get(&car_id)?;
```

The same repository pattern applies — use cases receive repository traits, not concrete implementations.

### Long Operation Manager

Threaded execution for heavy tasks:

```rust
let manager = LongOperationManager::new();
let handle = manager.spawn(
    "Importing inventory",
    |progress| {
        for (i, line) in lines.iter().enumerate() {
            progress.set_percent((i * 100) / lines.len());
            // ... process line ...
        }
        Ok(ImportResult { count: lines.len() })
    }
);

// In UI
handle.on_progress(|p| update_progress_bar(p));
let result = handle.await?;
```

Features:
- Progress callbacks with percentage and message
- Cancellation support
- Result or error on completion

### Synchronous Commands

Unlike C++/Qt's async approach, Rust uses synchronous command execution:

```rust
impl Command for CreateCarCommand {
    fn execute(&mut self, ctx: &mut Context) -> Result<()> {
        let car = self.use_case.execute(&self.dto, ctx)?;
        self.created_id = Some(car.id);
        Ok(())
    }
    
    fn undo(&mut self, ctx: &mut Context) -> Result<()> {
        if let Some(id) = self.created_id {
            self.remove_use_case.execute(id, ctx)?;
        }
        Ok(())
    }
}
```

This simpler model works well for CLI and desktop applications where blocking is acceptable.

### Event Hub

Channel-based event dispatch:

```rust
// Publishing
event_hub.publish(CarEvent::Updated(car.id));

// Subscribing
let rx = event_hub.subscribe::<CarEvent>();
while let Ok(event) = rx.recv() {
    match event {
        CarEvent::Updated(id) => refresh_car_view(id),
        // ...
    }
}
```

---

## Common Patterns

### Repository Pattern

Both languages generate repositories with identical interfaces:

| Method | Purpose |
|--------|---------|
| `get(id)` | Fetch single entity |
| `get_all()` | Fetch all entities |
| `create(entity)` | Insert new entity |
| `update(entity)` | Update existing entity |
| `remove(id)` | Delete entity |
| `exists(id)` | Check existence |

Relationship-specific methods are also generated:

| Method | For |
|--------|-----|
| `get_children(parent_id)` | one_to_many, ordered_one_to_many |
| `add_child(parent_id, child_id)` | one_to_many |
| `set_order(parent_id, child_ids)` | ordered_one_to_many |
| `get_related(entity_id)` | many_to_many |

### Unit of Work

Each use case gets a unit of work encapsulating its database operations:

```
CreateCarUnitOfWork
├── beginTransaction()
├── createCar(dto) → Car
├── commit()
└── rollback()
```

This keeps transaction boundaries explicit and testable.

### DTO Mapping

DTOs are generated for all boundary crossings:

```
Controller ←→ CreateCarDto ←→ UseCase ←→ Car (Entity) ←→ Repository
```

The separation ensures:
- Controllers don't expose entity internals
- Use cases receive validated, typed input
- Entities remain persistence-agnostic

---

## File Organization

### C++/Qt Output

```
src/
├── common/
│   ├── database/
│   │   ├── db_context.h/.cpp
│   │   ├── db_sub_context.h/.cpp
│   │   └── repository_factory.h/.cpp
│   ├── entities/
│   │   ├── car.h/.cpp
│   │   └── ...
│   ├── direct_access/
│   │   └── {entity}/
│   │       ├── {entity}_repository.h/.cpp
│   │       ├── {entity}_table.h/.cpp
│   │       └── {entity}_events.h/.cpp
│   └── undo_redo/
│       ├── undo_redo_system.h/.cpp
│       └── command.h
├── direct_access/
│   └── {entity}/
│       ├── {entity}_controller.h/.cpp
│       ├── dtos.h
│       ├── unit_of_work.h/.cpp
│       └── use_cases/
└── {feature}/
    ├── {feature}_controller.h/.cpp
    └── use_cases/
```

### Rust Output

Qleany uses the modern Rust module naming convention (no `mod.rs` files):

```
src/
├── common.rs                    # Module declaration
├── common/
│   ├── database.rs
│   ├── database/
│   │   ├── db_context.rs
│   │   └── tables.rs
│   ├── entities.rs              # All entity exports
│   ├── entities/
│   │   └── car.rs
│   ├── event.rs                 # All event definitions
│   ├── direct_access.rs         # Module exports
│   ├── direct_access/
│   │   ├── repository_factory.rs
│   │   └── {entity}/
│   │       ├── repository.rs
│   │       └── events.rs
│   └── undo_redo.rs
├── direct_access.rs
├── direct_access/
│   ├── lib.rs                   # Feature exports
│   ├── {entity}.rs
│   └── {entity}/
│       ├── controller.rs
│       ├── dtos.rs
│       ├── unit_of_work.rs
│       └── use_cases.rs
├── {feature}.rs
└── {feature}/
    ├── controller.rs
    └── use_cases.rs
```

This follows Rust's recommended practice since the 2018 edition, making navigation clearer by naming modules after their folders.
