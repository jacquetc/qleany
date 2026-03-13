# Qleany Quick Start - C++/Qt

This guide walks you through creating a complete desktop application for a car dealership using Qleany. By the end, you'll have generated architecture with entities, repositories, controllers, and undo/redo infrastructure. After generation, the only code you write is inside the use cases (your business logic) and the UI. Everything else compiles and works out of the box.

For Rust, see [Qleany Quick Start - Rust](quick-start-rust.md). The differences are minor.

The qleany.yaml of this example is available [here](../examples/cpp-qt/quick_start_carlot/qleany.yaml).

**Mandatory step**:

If not already done, create a git repository and commit the initial manifest, and tag in the pattern `vX.X.X`:
`git init && git add . && git commit -m"initial commit" && git tag v0.0.1`

---

## Step 1: Think About Your Domain

Before touching any tool, grab paper or open a diagramming tool. This is the most important step.

**Ask yourself:**
- What are the core "things" in my business? These become entities.
- What actions do users perform? These become use cases.
- Which use cases belong together? These become features.

### Example: CarLot — A Car Dealership App

**Entities** (the nouns):

| Entity     | Purpose                                  | Key Fields                            |
|------------|------------------------------------------|---------------------------------------|
| EntityBase | Base class for all entities              | id, created_at, updated_at            |
| Root       | Application entry point, owns everything | cars, customers, sales                |
| Car        | Vehicle in inventory                     | make, model, year, price, status      |
| Customer   | Potential or actual buyer                | name, email, phone                    |
| Sale       | Completed transaction                    | sale_date, final_price, car, customer |

**Relationships:**

- Root owns many Cars (inventory)
- Root owns many Customers (contacts)
- Root owns many Sales (history)
- Sale references one Car (what was sold)
- Sale references one Customer (who bought it)

**Features and Use Cases** (the verbs):

| Feature              | Use Case         | What it does                          |
|----------------------|------------------|---------------------------------------|
| inventory_management | import_inventory | Parse CSV file, populate Car entities |
| inventory_management | export_inventory | Generate CSV from current inventory   |

### Draw It First

Sketch your entities and relationships before using Qleany. Use paper, whiteboard, or [Mermaid](https://mermaid.live/).

Deeper explanations about relationships are available in the [Manifest Reference](manifest-reference.md#understanding-relationships).

```mermaid
erDiagram
    EntityBase {
        EntityId id
        datetime created_at
        datetime updated_at
    }

    Root {
        EntityId id
        datetime created_at
        datetime updated_at
        # relationships:
        QList<EntityId> cars
        QList<EntityId> customers
        QList<EntityId> sales
    }
    
    Car {
        EntityId id
        datetime created_at
        datetime updated_at
        string make
        string model
        int year
        float price
        enum status
    }
    
    Customer {
        EntityId id
        datetime created_at
        datetime updated_at
        string name
        string email
        string phone
    }
    
    Sale {
        EntityId id
        datetime created_at
        datetime updated_at
        datetime sale_date
        float final_price
        int car_id
        int customer_id
        # relationships:
        EntityId car
        EntityId customer
    }

    EntityBase ||--o{ Root : "inherits"
    EntityBase ||--o{ Car : "inherits"
    EntityBase ||--o{ Customer : "inherits"
    EntityBase ||--o{ Sale : "inherits"
    Root ||--o{ Car : "owns (strong)"
    Root ||--o{ Customer : "owns (strong)"
    Root ||--o{ Sale : "owns (strong)"
    Sale }o--|| Car : "optionally references"  # Many-to-One (a sale may exist without a car, e.g., if the car was deleted)
    Sale }o--|| Customer : "optionally references" # Many-to-One 
```

**Why draw first?** Changing a diagram is free. Changing generated code is work. Get the model right before generating.

`EntityBase` is a common pattern: it provides shared fields like `id`, `created_at`, and `updated_at`, like an inheritance. Other entities can explicitly inherit from it. This is not an entity. It will never be generated. All your entities can inherit from it to avoid repetition.

> Note: You can note the relationships on the diagram too. Qleany supports various relationship types (one-to-one, one-to-many, many-to-one, many-to-many) and cascade delete (strong relationships). Defining these upfront helps you configure them correctly in the manifest. Unlike typical ER diagrams, the relationships appear as fields. Forget the notion of foreign keys here. Qleany's relationships are directional and can be configured with additional options (e.g., ordered vs unordered, strong vs weak, optional or not (only for some relationship types)). Plan these carefully to ensure the generated code matches your intended data model.

**WRONG**: I only need a few entities without any "owner" relationships. I can just create them in Qleany and skip the Root entity.

**RIGHT**: I want a clear ownership structure. Root owns all Cars, Customers, and Sales. This makes it easy to manage the lifecycle of entities. It avoids orphan entities and simplifies the generated code. Even if Root has few fields, it provides a clear parent-child structure. Think like a tree: Root is the trunk, Cars/Customers/Sales are branches. This is a common pattern in Qleany projects.

---

## Step 2: Create a New Manifest

### Using the GUI

Launch Qleany. You'll land on the **Home** tab.

1. Click **New Manifest** — a creation wizard opens
2. **Step 1 — Language**: Select **C++/Qt**
3. **Step 2 — Project**: Enter your application name (PascalCase, e.g. `CarLot`) and organisation name (e.g. `MyCompany`)
4. **Step 3 — Template**: Choose a starting template:
   - **Blank** — EntityBase + empty Root (start from scratch)
   - **Minimal** — Root with one entity (Item). Hello world equivalent
   - **Document Editor** — Documents > Sections with load/save use cases
   - **Data Management** — Items, Categories, Tags with import/export use cases
5. **Step 4 — UI Options**: Enable **Qt Quick (QML)** and/or **Qt Widgets**
6. Click **Create**, then choose where to save `qleany.yaml` (your project root)

### Using the CLI

```bash
qleany new /path/to/project \
  --language cpp-qt \
  --name CarLot \
  --org-name MyCompany \
  --template blank \
  --options cpp_qt_qtquick
```

All flags are optional — if omitted, the CLI prompts interactively. Use `--force` to overwrite an existing manifest.

### What gets created

Qleany creates a manifest pre-configured with:
- Your chosen language, application name, and organisation
- `EntityBase` (provides id, created_at, updated_at)
- `Root` entity inheriting from EntityBase (plus more entities if you chose a template other than Blank)
- Your selected UI options

---

## Step 3: Configure Project Settings

Click **Project** in the sidebar to review and adjust settings. The wizard already filled in the language, application name, and organisation name. You can still change:

| Field               | Value         |
|---------------------|---------------|
| Organisation Domain | com.mycompany |
| Prefix Path         | src           |

Organisation Domain is used for some installed file names, like the icon name.

Changes save. The header shows "Save Manifest" when there are unsaved changes.

---

## Step 4: Define Entities

Click **Entities** in the sidebar. You'll see a three-column layout.

### 4.1 Create the Car Entity

1. Click the **+** button next to "Entities"
2. A new entity appears — click it to select
3. In the details panel:
    - **Name**: `Car`
    - **Inherits from**: `EntityBase`

> You can also enable the **Single Model** checkbox to generate a helper class for the entity and its QML wrapper.

Now add fields. In the "Fields" section:

1. Click **+** to add a field
2. Select the new field, then configure:

| Name   | Type    | Notes                                                                          |
|--------|---------|--------------------------------------------------------------------------------|
| make   | String  | —                                                                              |
| model  | String  | —                                                                              |
| year   | Integer | —                                                                              |
| price  | Float   | —                                                                              |
| status | Enum    | Enum Name: `CarStatus`, Values: `Available`, `Reserved`, `Sold` (one per line) |

### 4.2 Create the Customer Entity

1. Click **+** next to "Entities"
2. **Name**: `Customer`
3. **Inherits from**: `EntityBase`
4. Add fields:

| Name  | Type   |
|-------|--------|
| name  | String |
| email | String |
| phone | String |

### 4.3 Create the Sale Entity

1. Click **+** next to "Entities"
2. **Name**: `Sale`
3. **Inherits from**: `EntityBase`
4. Add fields:

| Name        | Type     | Configuration                                              |
|-------------|----------|------------------------------------------------------------|
| sale_date   | DateTime | —                                                          |
| final_price | Float    | —                                                          |
| car         | Entity   | Referenced Entity: `Car`, Relationship: `many_to_one`      |
| customer    | Entity   | Referenced Entity: `Customer`, Relationship: `many_to_one` |

### 4.4 Configure Root Relationships

Select the **Root** entity. Add relationship fields:

| Name      | Type   | Configuration                                                                 |
|-----------|--------|-------------------------------------------------------------------------------|
| cars      | Entity | Referenced Entity: `Car`, Relationship: `ordered_one_to_many`, Strong: ✓      |
| customers | Entity | Referenced Entity: `Customer`, Relationship: `ordered_one_to_many`, Strong: ✓ |
| sales     | Entity | Referenced Entity: `Sale`, Relationship: `ordered_one_to_many`, Strong: ✓     |

> You can also enable the **List Model** checkbox to generate reactive QAbstractListModel and its QML wrappers. Set **Displayed Field** to specify which field appears in list views (e.g., `make` for cars, `name` for customers).

**Key concepts:**
- **Strong relationship**: Deleting Root cascades to delete all Cars, Customers, Sales

---

## Step 5: Define Features and Use Cases

Click **Features** in the sidebar. You'll see a four-column layout.

### 5.1 Create the Feature

1. Click **+** next to "Features"
2. Select it and set **Name**: `inventory_management`

### 5.2 Create the Import Use Case

1. Click **+** next to "Use Cases"
2. Configure:

| Field          | Value                                        |
|----------------|----------------------------------------------|
| Name           | import_inventory                             |
| Undoable       | ✗ *(file imports typically aren't undoable)* |
| Read Only      | ✗ *(it will update the internal database)*   |
| Long Operation | ✗                                |

3. Switch to the **DTO In** tab:
    - Enable the checkbox
    - **Name**: `ImportInventoryDto`
    - Add field: `file_path` (String)

4. Switch to the **DTO Out** tab:
    - Enable the checkbox
    - **Name**: `ImportReturnDto`
    - Add fields: `imported_count` (Integer), `error_messages` (String, List: ✓)

5. Switch to the **Entities** tab:
    - Check: `Root`, `Car`

### 5.3 Create the Export Use Case

1. Click **+** next to "Use Cases"
2. Configure:

| Field          | Value                            |
|----------------|----------------------------------|
| Name           | export_inventory                 |
| Undoable       | ✗                                |
| Read Only      | ✓ *(just reading internal data)* |
| Long Operation | ✗                                |

3. **DTO In**:
    - **Name**: `ExportInventoryDto`
    - Field: `output_path` (String)

4. **DTO Out**:
    - **Name**: `ExportReturnDto`
    - Field: `exported_count` (Integer)

5. **Entities**: Check `Root`, `Car`

### 5.4 UI Options

You already chose your UI frontends (Qt Quick, Qt Widgets, or both) during manifest creation. You can change these later in the **User Interface** tab.

For C++/Qt, the controllers, models, and "singles" (like in "Single model") C++ wrappers for integration with QML are generated for you. Also, mock implementations for each of these files are generated for you to allow developing the UI without the backend.

### 5.5 Save the Manifest

Click **Save Manifest** in the header (or Ctrl+S).

### 5.6 Take a break, drink a coffee, sleep a bit

I mean it. A fresher mind sees things more clearly. You already saved a lot of time by using Qleany instead of writing all the boilerplate yourself. Don't rush the design phase, it's where you get the most value from Qleany.

Designing your domain and use cases is the most important part. The generated code is a complete architecture, not mere scaffolding. If the model is wrong, the code won't help much. Take your time to get it right before generating.

Yes, you can change the manifest and regenerate later. But it's better to get a solid design upfront. The more you change the model after generating, the more work you create for yourself. It's not a problem to evolve your design, but try to avoid major changes that require rewriting large parts of the generated code.

---

## Step 6: Generate

### Commit to Git

Before generating, commit your current state to Git. This isn't optional advice — it's how Qleany is meant to be used. If you accidentally overwrite files you've modified, you can restore them.

For a C++/Qt project, the generated CMakeLists.txt needs a git tag "vX.X.X". It is mandatory (or modify yourself the CMakeLists.txt to remove the tag system)

```bash
git add .
git commit -m "Before Qleany generation"
```

### Generate Code

1. Click **Generate** in the sidebar
2. Review the groups and files. Use the **status** filters (Modified, New, Unchanged) and **nature** filters (Infra, Aggregate, Scaffold) to narrow the list
3. (Optional) Check **in temp/** to generate to a temporary folder first
4. Click a file to preview the generated code
5. Click **Generate (N)** where N is the number of selected files

The progress modal shows generation status. Files are written to your project.

The files are formatted with clang-format (Microsoft style).

---

## Step 7: What You Get

After a generation, your project contains:

```
├── cmake
│   ├── InstallHelpers.cmake
│   └── VersionFromGit.cmake
├── CMakeLists.txt
└── src
    ├── common
    │   ├── CMakeLists.txt
    │   ├── controller_command_helpers.h
    │   ├── service_locator.h/.cpp     
    │   ├── controller_command_helpers.h  
    │   ├── signal_buffer.h                
    │   ├── database
    │   │   ├── db_builder.h
    │   │   ├── db_context.h
    │   │   ├── junction_table_ops
    │   │   │   ├── junction_cache.h
    │   │   │   ├── many_to_one.cpp
    │   │   │   ├── many_to_one.h
    │   │   │   ├── one_to_one.cpp
    │   │   │   ├── one_to_one.h
    │   │   │   ├── ordered_one_to_many.cpp
    │   │   │   ├── ordered_one_to_many.h
    │   │   │   ├── unordered_many_to_many.cpp
    │   │   │   ├── unordered_many_to_many.h
    │   │   │   ├── unordered_one_to_many.cpp
    │   │   │   └── unordered_one_to_many.h
    │   │   └── table_cache.h
    │   ├── direct_access                    # Holds the repositories and table implementations
    │   │   ├── use_case_helpers/...          # Template classes for direct access use cases
    │   │   ├── car
    │   │   │   ├── car_events.h
    │   │   │   ├── car_repository.cpp
    │   │   │   ├── car_repository.h
    │   │   │   ├── car_table.cpp
    │   │   │   ├── car_table.h
    │   │   │   ├── CMakeLists.txt
    │   │   │   ├── i_car_repository.h
    │   │   │   └── table_definitions.h
    │   │   ├── CMakeLists.txt
    │   │   ├── converter_registration.h
    │   │   ├── customer
    │   │   │   ├── CMakeLists.txt
    │   │   │   ├── customer_events.h
    │   │   │   ├── customer_repository.cpp
    │   │   │   ├── customer_repository.h
    │   │   │   ├── customer_table.cpp
    │   │   │   ├── customer_table.h
    │   │   │   ├── i_customer_repository.h
    │   │   │   └── table_definitions.h
    │   │   ├── event_registry.h                # event system for reactive updates
    │   │   ├── mapper_tools.h
    │   │   ├── operators.h
    │   │   ├── repository_factory.cpp
    │   │   ├── repository_factory.h
    │   │   ├── root
    │   │   │   ├── CMakeLists.txt
    │   │   │   ├── i_root_repository.h
    │   │   │   ├── root_events.h
    │   │   │   ├── root_repository.cpp
    │   │   │   ├── root_repository.h
    │   │   │   ├── root_table.cpp
    │   │   │   ├── root_table.h
    │   │   │   └── table_definitions.h
    │   │   └── sale
    │   │       ├── CMakeLists.txt
    │   │       ├── i_sale_repository.h
    │   │       ├── sale_events.h
    │   │       ├── sale_repository.cpp
    │   │       ├── sale_repository.h
    │   │       ├── sale_table.cpp
    │   │       ├── sale_table.h
    │   │       └── table_definitions.h
    │   ├── entities
    │   │   ├── car.h
    │   │   ├── CMakeLists.txt
    │   │   ├── customer.h
    │   │   ├── root.h
    │   │   └── sale.h
    │   ├── features
    │   │   ├── CMakeLists.txt
    │   │   ├── feature_event_registry.h           # event system for reactive updates
    │   │   └── inventory_management_events.h
    │   ├── service_locator.cpp
    │   ├── service_locator.h
    │   ├── undo_redo                              
    │   │   ├── group_command_builder.cpp
    │   │   ├── group_command_builder.h
    │   │   ├── group_command.cpp
    │   │   ├── group_command.h
    │   │   ├── query_handler.cpp
    │   │   ├── query_handler.h
    │   │   ├── undo_redo_command.cpp
    │   │   ├── undo_redo_command.h
    │   │   ├── undo_redo_manager.cpp
    │   │   ├── undo_redo_manager.h
    │   │   ├── undo_redo_stack.cpp
    │   │   ├── undo_redo_stack.h
    │   │   ├── undo_redo_system.cpp
    │   │   └── undo_redo_system.h
    │   └── unit_of_work
    │       ├── unit_of_work.h
    │       ├── uow_base.h
    │       ├── uow_macros.h
    │       └── uow_ops.h
    ├── direct_access
    │   ├── car
    │   │   ├── car_controller.cpp        # Exposes CRUD operations to UI
    │   │   ├── car_controller.h
    │   │   ├── car_unit_of_work.h
    │   │   ├── CMakeLists.txt
    │   │   ├── dtos.h
    │   │   ├── dto_mapper.h
    │   │   └── i_car_unit_of_work.h
    │   ├── CMakeLists.txt
    │   ├── customer
    │   │   └── ...
    │   ├── root
    │   │   └── ...
    │   └── sale
    │       ├── CMakeLists.txt
    │       ├── dtos.h
    │       ├── sale_controller.cpp
    │       ├── sale_controller.h
    │       ├── sale_unit_of_work.h
    │       ├── dto_mapper.h
    │       └── i_sale_unit_of_work.h
    ├── inventory_management
    │   ├── CMakeLists.txt
    │   ├── inventory_management_controller.cpp    # Exposes operations to UI
    │   ├── inventory_management_controller.h
    │   ├── inventory_management_dtos.h
    │   ├── units_of_work                 # adapt the macros here 
    │   │   ├── export_inventory_uow.h
    │   │   └── import_inventory_uow.h
    │   └── use_cases
    │       ├── export_inventory_uc          # adapt the macros here 
    │       │   └── i_export_inventory_uow.h
    │       ├── export_inventory_uc.cpp      # You implement the logic here
    │       ├── export_inventory_uc.h
    │       ├── import_inventory_uc          # adapt the macros here 
    │       │   └── i_import_inventory_uow.h
    │       ├── import_inventory_uc.cpp      # You implement the logic here
    │       └── import_inventory_uc.h
    ├── tests
    │   ├── CMakeLists.txt
    │   ├── database
    │   │   ├── CMakeLists.txt
    │   │   ├── tst_many_to_one_junction.cpp
    │   │   ├── tst_one_to_one_junction.cpp
    │   │   ├── tst_ordered_one_to_many_junction.cpp
    │   │   ├── tst_unordered_many_to_many_junction.cpp
    │   │   └── tst_unordered_one_to_many_junction.cpp
    │   └── undo_redo
    │       ├── CMakeLists.txt
    │       ├── tst_enhanced_undo_redo.cpp
    │       ├── tst_qcoro_integration.cpp
    │       ├── tst_root_undo_redo.cpp
    │       └── tst_undo_redo.cpp
    │
    └── qtwidgets_ui
        ├── CMakeLists.txt
        ├── main.cpp
        ├── main_window.cpp                                  # ← write your UI here
        └── main_window.h
    
And/Or
    
    ├── presentation                                        # generated for all QML-based UIs
    │   ├── CMakeLists.txt
    │   ├── mock_imports                                    # QML mocks
    │   │   └── Car
    │   │       ├── Controllers
    │   │       │   ├── CarController.qml
    │   │       │   ├── CarEvents.qml
    │   │       │   ├── CustomerController.qml
    │   │       │   ├── CustomerEvents.qml
    │   │       │   ├── EventRegistry.qml
    │   │       │   ├── InventoryManagementController.qml
    │   │       │   ├── QCoroQmlTask.qml
    │   │       │   ├── qmldir
    │   │       │   ├── RootController.qml
    │   │       │   ├── RootEvents.qml
    │   │       │   ├── SaleController.qml
    │   │       │   ├── SaleEvents.qml
    │   │       │   └── UndoRedoController.qml
    │   │       ├── Models
    │   │       │   ├── qmldir
    │   │       │   └── RootCustomersListModel.qml
    │   │       └── Singles
    │   │           ├── qmldir
    │   │           ├── SingleCar.qml
    │   │           ├── SingleCustomer.qml
    │   │           ├── SingleRoot.qml
    │   │           └── SingleSale.qml
    │   └── real_imports                                 # QML real imports
    │       ├── controllers
    │       │   ├── CMakeLists.txt
    │       │   ├── foreign_car_controller.h
    │       │   ├── foreign_customer_controller.h
    │       │   ├── foreign_event_registry.h
    │       │   ├── foreign_feature_event_registry.h
    │       │   ├── foreign_inventory_management_controller.h
    │       │   ├── foreign_root_controller.h
    │       │   ├── foreign_sale_controller.h
    │       │   └── foreign_undo_redo_controller.h
    │       ├── models
    │       │   ├── CMakeLists.txt
    │       │   └── foreign_root_customers_list_model.h
    │       └── singles
    │           ├── CMakeLists.txt
    │           ├── foreign_single_car.h
    │           ├── foreign_single_customer.h
    │           ├── foreign_single_root.h
    │           └── foreign_single_sale.h
    └── qtquick_app
        ├── Car                                    # Car: 3 first letters of CarLot.   ← write your UI here
        │   └── CMakeLists.txt
        ├── CMakeLists.txt
        ├── content                                 # ← write your UI here
        │   ├── App.qml
        │   └── CMakeLists.txt
        ├── main.cpp
        ├── main.qml
        └── qtquickcontrols2.conf


```

**What's generated:**
- Complete CRUD for all entities (create, get, update, remove, ...)
- Controllers exposing operations
- DTOs for data transfer
- Repository pattern for database access
- Undo/redo infrastructure for undoable operations
- Tests suites for the database and undo redo infrastructure
- Macros for unit of work
- Event system for reactive updates
- Basic CLI (if selected during project setup)
- Basic empty UI (if selected during project setup)

**What you implement:**
- Your custom use case logic (import_inventory, export_inventory)
- Your UI or CLI on top of the controllers or their adapters.

---

## Step 8: Run the Generated Code

Let's assume that you have Qt6 dev libs and QCoro-qt6 dev libs installed in the system. Also, install cmake and extra-cmake-modules. 

You need the project to sit on a Git repository to generate code. The CMakeLists.txt checks for the latest version tag (vX.Y.Z) and fails if it's not found. So, if you need a new repository:
```bash
git init
git add .
git commit -m "Initial commit"
git tag v0.1.0
```

You can use an IDE like Qt Creator or VS Code and build/run the project from there.

Or in a terminal,
```
mkdir build && cd build
cmake ..
cmake --build . --target all -j$(nproc)
```

Run the app (in case of QtWidgets):
```
./src/qtwidgets_app/CarLot
```

---

## Next Steps

1. Run the generated code — it compiles and provides working CRUD
2. Implement your custom use cases (`import_inventory`, `export_inventory`)
3. Build your UI on top of the controllers
4. Add more features as your application grows

---

## Tips

### Understanding the Internal Database

Entities are stored in an internal database (SQLite). This database is **internal**, users and UI devs don't interact with it directly.

**Typical pattern:**

1. User opens a file (e.g., `.carlot` project file)
2. Your `load_project` use case parses the file and populates entities
3. User works — all changes go to the internal database
4. User saves — your `save_project` use case serializes entities back to file

The internal database is ephemeral. It enables fast operations, undo/redo. The user's file is the permanent storage.

### Undo/Redo

Every generated CRUD operation supports undo/redo automatically. You don't have to display undo/redo controls in your UI if you don't want to, but the infrastructure is there when you need it.

If you mark a use case as **Undoable**, Qleany generates the command pattern scaffolding. You fill in what "undo" means for your specific operation.

For more information, see [Undo-Redo Architecture](undo-redo-architecture.md).

### Relationships

| Relationship | Use When |
|--------------|----------|
| one_to_one | Exclusive 1:1 (User → Profile) |
| many_to_one | Child references parent (Sale → Car) |
| one_to_many | Parent owns unordered children |
| ordered_one_to_many | Parent owns ordered children (chapters in a book) |
| many_to_many | Shared references (Items ↔ Tags) |

**Strong** means cascade delete — deleting the parent deletes children.

For more details, see [Manifest Reference](manifest-reference.md#relationship-fields).

### Regenerating

Made a mistake? The manifest is just YAML. You can:
- Edit it directly in a text editor or from the GUI tool
- Delete entities/features in the UI and recreate them
- Generate to a temp folder, review, then regenerate to the real location

For more details, see [Regeneration Workflow](regeneration-workflow.md).

---

The generated code is yours. Modify it, extend it, or regenerate when you add new entities. Qleany gets out of your way.

---

## Further Reading

- [README](../README.md) — Overview, building and running, reference implementation
- [Manifest Reference](manifest-reference.md) — Entity options, field types, relationships, features
- [Design Philosophy](design-philosophy.md) — Clean Architecture background, package by feature
- [Regeneration Workflow](regeneration-workflow.md) — How file generation works, what gets overwritten
- [Undo-Redo Architecture](undo-redo-architecture.md) — Entity tree structure, undoable vs non-undoable
- [QML Integration](qml-integration.md) — Reactive models and mocks for C++/Qt
- [Generated Infrastructure - C++/Qt](generated-code-cpp-qt.md) — Database layer, event system, file organization
- [Generated Infrastructure - Rust](generated-code-rust.md) — Database layer, event system, file organization
- [Troubleshooting](troubleshooting.md) — Common issues and how to fix them
