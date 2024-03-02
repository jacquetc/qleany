# Qleany - Clean Architecture Framework for C++/Qt6 Projects

Qleany is a streamlined framework designed to integrate Clean Architecture principles within C++ Qt6 applications. It is built on three core components:

1. **Qleany C++/Qt Library**: Provides a range of common and generic tools and classes essential for implementing Clean Architecture in C++/Qt projects.
2. **Python/Jinja2 Project Structure Generator**: Features a dedicated user interface developed using PySide. This generator facilitates the creation of a structured project environment based on the principles of Clean Architecture.
3. **Examples and Documentation**: A collection of examples to guide users in implementing the framework effectively.

## Important Notices

Please avoid using Qt Design Studio version 4.3 (which utilizes Qt 6.6) due to a known issue that impacts Qt versions 6.5.3 and 6.6. This bug can cause crashes in previews (qml2puppet) when working with QML mocks generated by Qleany. We recommend using Qt Design Studio LTS version 4.1 instead, as it is based on Qt 6.5.1 and does not exhibit this problem. Qt Design Studio 4.4 preview seems to run well with Qleany. 

## Framework's Objective

Qleany's primary goal is to automate the generation of a structured project environment for C++/Qt6 applications. This is achieved by interpreting a simple manifest file, named `qleany.yaml`, located at the root of the project. The framework generates a comprehensive structure including folders, CMakeLists.txt, and essential C++ files. The generated projects support both QWidget and QML GUIs or a combination of both. Upon initial generation, the projects are immediately compilable, requiring developers only to design GUIs and implement custom use cases.

The framework acknowledges the repetitive nature of file creation in Clean Architecture and addresses this by automating the generation of similar files. Additional features include:

- An asynchronous undo-redo system based on the command pattern. A project can ignore the undo part if it is not needed.
- A SQLite-based database layer for data persistence.
- Support for custom use cases with their own signal and with user-defined DTOs (Data Transfer Objects) for inputs and outputs.
- The ability to define both soft and hard relationships between entities, including one-to-one and one-to-many (unordered or ordered) associations. Many-to-many relationships are not supported yet.
- Entities within the framework handle cascade deletion. Additionally, the implementation of soft-deletion (recoverable trash binning) is currently in progress.

## Framework Structure

Many developers are likely familiar with the following depiction of Clean Architecture:

![Alt text](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*D1EvAeK74Gry46JMZM4oOQ.png)

It's important to note that this conceptual representation needs to be tailored to fit the specific requirements of the language and project at hand. Qleany presents a distinct interpretation of Clean Architecture, uniquely adapted and structured to suit its specific use cases and environment.

- **Entities**: Contains entities and is encapsulated in a library named `entities`.
- **Application**: Groups use cases by functionalities, organized within a library called `application`.
- **Persistence**: Manages internal data persistence. It includes a 'repository' wrapper for SQLite database interactions, with each entity having its repository in the `RepositoryProvider` class.
- **Contracts**: A common library for most other components, housing all interfaces from `persistence`, `gateway`, and `infrastructure`. This design minimizes tight coupling and circular dependencies.
- **DTO Libraries**: Each functionality has its DTO library, facilitating communication with the `application` layer. DTOs are used for both input and output in interactions with the outer layers, such as interactors.
- **CQRS Libraries** (Command Query Responsibility Segregation): The `application` layer is designed to support CQRS, with commands and queries being handled separately. This separation is achieved by using the `CommandHandler` and `QueryHandler` classes. Other classes, such as `CommandValidator` and `QueryValidator`, are used to validate commands and queries, respectively. They are stored away in a separate library called `cqrs`.
- **Gateway**: Optional library for handling remote connections and services. It can be manually added by the developer and is used similarly to repositories in use cases.
- **Infrastructure**: Optional. Handles actions like file management, local settings, and system queries. It's injected into use cases similar to repositories and gateways.
- **Interactor**: Acts as an internal API to invoke use cases, streamlining the interaction between the user interface and application logic.
- **Presenter**: Maintains Qt models and representations of unique entities (referred to as `Singles`), enhancing their integration and usage within the GUI.
- **Registration**: Each component (`persistence`, `gateway`, `infrastructure`, `interactor`) initializes its classes in a corresponding *name*_registration.cpp file, typically called together in the main.cpp.

Project dependencies:
![Alt text](doc/qleany_project_dep.drawio.png)

Example of project structure:
![Alt text](doc/qleany_project_structure.drawio.png)

## Installing the Qleany library

### Manually

Prerequisites:
- Qt 6.5 (dev packages) 
- QCoro (dev packages)
- Cmake and extra-cmake-modules

The use of sccache is optional. Also, adapt the -j6 to your number of CPU minus one.

CMake options are:
- QLEANY_BUILD_EXAMPLES (default: on)
- QLEANY_BUILD_TESTS (default: on)
- BUILD_SHARED_LIBS (default: off)
- QLEANY_BUILD_WITH_QT_GUI (default: on)

```bash
git clone https://github.com/jacquetc/qleany.git
cd qleany
mkdir build
cd build
cmake -DCMAKE_INSTALL_PREFIX=/usr/local -DQLEANY_BUILD_WITH_QT_GUI=on -DQLEANY_BUILD_EXAMPLES=off -DQLEANY_BUILD_TESTS -DCMAKE_CXX_COMPILER_LAUNCHER=sccache ..
cmake --build . -- -j6
sudo cmake --install .
```

Qleany is building and examples are running well if you use Qt Creator or Visual Studio Code with the CMake Tools extension.

## Using Qleany

To use Qleany, follow these steps:

1. Write a `qleany.yaml` file for your project. You can use the `examples/simple/qleany.yaml` file as a reference.
2. Run the Qleany GUI interface and select the `qleany.yaml` file.
3. List and select the files you want to generate.
4. To avoid overwriting your current files: Preview the files, it will generate them in a "qleany_preview" folder.
5. If you are sure, generate the files directly. Qleany will generate them in the right place, but will never delete other files.
7. Create CMakelists.txt files to include the generated libraries in your project. You can use the `examples/simple/src/core/CMakeLists.txt` and `examples/simple/src/gui/CMakeLists.txt` files as a reference.
6. For custom commands and queries, you still have to fill the blanks in the generated files. You will find "Q_UNIMPLEMENTED();" in the generated files.


### For QWidgets GUI

7. Create an UI project, not at the root of the project, but in a dedicated sub-folder, like with did with `examples/simple/src/gui/desktop_application`.
8. You can now start to implement your GUI and use cases. A GUI made with QWidgets will only use the interactors and models in presenter. Refer to the example for guidance at `examples/simple/src/gui/desktop_application/main.cpp`

### For QML GUI

*Note*: For now, the QML file generation is tailor-made to be used after a project is created using Qt Design Studio, but only subltle changes are needed to use it with a project created manually. You can use the `examples/simple/src/gui/qml_application` as a reference of what is running fine, this project uses Qt Design Studio's generated CMakeLists.txt. At the minimum, you only have to include the generated `realqmlmodules.cmake` file in your project's CMakeLists.txt file and mofify your main.cpp to register the other libraries.

7. Create a QML project using Qt Design Studio, not at the root of the project, but in a dedicated sub-folder, like with did with `examples/simple/src/gui/qml_application`.
8. You can now start to implement your GUI and use cases. A GUI made with QML will use **not** the interactors and models directly from the interactor and presenter libraries. Wrappers around them all are generated in the QML `real_imports` folder in the QML folder to be made available from QML. Also, QML mocks are generated in `mock_imports`, to be filled by the developer. Refer to the example for guidance at `examples/simple/src/gui/qml_application/src/main.cpp` and `examples/simple/src/gui/qml_application/CMakelists.txt`

### For both QWidgets and QML GUI

You can use both QWidgets and QML GUIs in the same project. You can use the `examples/simple/` as a reference. The QML and QWidgets GUIs are in their own sub-folders, and the main.cpp file is in the root of the project. The CMakeLists.txt file is in the root of the project and includes the QML and QWidgets GUIs.

### Gateway and Infrastructure

The gateway and infrastructure are not generated by Qleany. You have to create them manually. You can use the `examples/simple/src/core/contracts` and `examples/simple/src/core/persistence` as a reference. The `contracts` folder contains the interfaces for the gateway and infrastructure, similar to what is done with the repositories of `persistence`. 

So, if I wnated to add a `gateway`, I would create a `gateway` folder in the `src/core/contracts` folder, and add the interfaces for the gateway. Then, I would create a `gateway` folder in the `src/core` folder, and add the implementation of the gateway. Each use case (handler) in `application` would have a `gateway` parameter using the interface, like what is already one with the repositories, and the `gateway` would be instanciated and injected into `interactor` in the `main.cpp` file.

Finally, do not forget a `gateway_registration.cpp` file in the `src/core/gateway` folder to register the gateway classes.

In a Gateway, we would find connections to remote services like REST APIs and remote datbases, and in Infrastructure, we would find connections to local services, like file management, local settings, and system queries. A "loadFile" method in a `FileLoader` class would be an example of an infrastructure service. Same for a `Settings` class or "exportToPdf" method in a `PdfExporter` class. 

The names Gateway and Infrastructure are not mandatory, you can use other names, like Remote and Local, or whatever you want.

### Custom Commands and Queries

You can add custom commands and queries for each feature in the `application.features` of the `qleany.yaml`. You can use the `examples/simple/qleany.yaml` and `examples/simple/src/core/application` as references. Search for the Q_UNIMPLEMENTED(); macro in the generated files to find the places to fill with your custom code. Be careful ot not overwrite your custom code when you regenerate the files.

## Installing the Qleany GUI Interface

Qleany tooling can be installed using `pip install qleany`. Alternatively, for an easier installation, you can install it using `pipx run qleany` if you have pipx installed.


## Utilizing the Qleany GUI Interface

To access Qleany's user-friendly graphical interface, run `qleany` in a terminal. This interface allows developers to efficiently manage file generation. This is the recommended way to generate files.

![Alt text](doc/qleany_generator_gui.png)

1. **Run the Qleany GUI**:
   - Launch Qleany's graphical user interface by executing the script `generator/qleany_generator_ui.py`.

2. **Select the `qleany.yaml` File**:
   - Begin by choosing your project's `qleany.yaml` file. This configuration file is essential for the GUI to operate correctly.

3. **List Available Files**:
   - In the GUI, use the "list" button for each component. This will generate a list of files that can be created for that component.

4. **Select Files to Generate**:
   - Choose the files you want to generate from the provided list, depending on your project requirements.

5. **Preview Files**:
   - Opt for the "preview" feature to generate and inspect the selected files in a "preview" folder. The location of this folder is defined in your `qleany.yaml` file.

6. **Generate Files**:
   - After previewing, proceed to generate the files by clicking the "generate" button. This will create the files in their designated locations within your project.

7. **Overwrite Confirmation**:
   - Should the file generation process require overwriting existing files, a warning message will appear. This alert ensures you are informed about and agree to the upcoming changes to your current files.

Alternatively, you can list and generate all the files of the project.


## Qleany YAML Configuration Rules

The `qleany.yaml` file is the core configuration file for the Qleany framework. A working example can be foound in `example/simple/qleany.yaml`. Below are the rules and structure for defining the configuration:

### Global Settings
```yaml
global:
  application_name: SimpleExample
  application_cpp_domain_name: Simple
  organisation:
    name: simpleexample
    domain: qleany.eu
```

### Entities Definition

Defines entities and their properties. Setting parent to EntityBase (provided by Qleany) offers the "id" field of type "int". It's mandatory to use EntityBase as heritage.
```yaml
entities:
  list:
    - name: EntityName
      parent: ParentEntity
      only_for_heritage: true/false
      fields:
        # basic:
        - type: DataType
          name: fieldName
          hidden: true/false (default: false)
        # one-to-one relationship:
        - type: OtherEntityName
          name: fieldName
          strong: true/false
          hidden: true/false (default: false)
        # one-to-many relationship:
        - type: QList<OtherEntityName>
          name: fieldName
          strong: true/false
          ordered: true/false
          hidden: true/false (default: false)
        # other fields ...
    # other entities ...
  folder_path: path/to/entity/folder
```

### Repositories Configuration

Specifies settings for entity repositories.

```yaml
repositories:
  list:
    - entity_name: EntityName
      lazy_loaders: true/false
    # other repositories, typically one for each entity
  repository_folder_path: path/to/repository/folder
  base_folder_path: path/to/base/folder
```

### Interactor Settings

Configures interactor-specific settings.

```yaml
interactor: 
  folder_path: path/to/interactor/folder
  create_undo_redo_interactor: true/false
```
### Application Layer Configuration

Defines application-specific settings and CRUD operations.

```yaml
application:
  common_cmake_folder_path: path/to/application/folder
  features:
    - name: FeatureName
      DTO:
        dto_identical_to_entity:
          enabled: true/false
          entity_mappable_with: EntityName
      CRUD:
        enabled: true/false (default: false)
        entity_mappable_with: EntityName
        get:
          enabled: true/false
        get_all:
          enabled: true/false
        get_with_details:
          enabled: true/false
        create: 
          enabled: true/false
        remove: 
          enabled: true/false
        update: 
          enabled: true/false       
        insert_relation: 
          enabled: true/false        
        remove_relation: 
          enabled: true/false 
      commands:
        - name: CommandName
          entities:
            - EntityName
          validator: 
            enabled: true/false 
          undo: true/false 
          dto:
            in:
              enabled: true/false (default: true)
              type_prefix: CommandName
              fields:
                - type: DataType
                  name: fieldName
            out:
              enabled: true/false (default: true)
              type_prefix: CommandNameReply
              fields:
                - type: DataType
                  name: fieldName
      queries:
        - name: QueryName
          entities:
            - EntityName
          validator: 
            enabled: true/false 
          undo: false (useless for queries)
          dto:
            in:
              enabled: true/false (default: true)
              type_prefix: QueryName
              fields:
                - type: DataType
                  name: fieldName
            out:
              type_prefix: QueryNameReply
              fields:
                - type: DataType
                  name: fieldName
              
```

### DTOs (Data Transfer Objects) Configuration

```yaml
DTOs:
  common_cmake_folder_path: path/to/dtos/folder
```

### Contracts Configuration

Defines settings for contracts in the application.

```yaml
contracts:
  inverted_app_domain: domain.identifier
  folder_path: path/to/contracts/folder
```

### Presenter Settings

Configures presenter-specific settings. Note: the `name` can be set to `auto`

```yaml
presenter:
  folder_path: path/to/presenter/folder
  create_undo_and_redo_singles: true/false (default false)
  singles:
    - name: SingleName (or "auto")
      entity: EntityName
      read_only: true/false (default: false)
    # Additional singles...
  list_models:
    - name: ListModelName (or auto)
      entity: EntityName
      displayed_field: fieldName
      in_relation_of: RelationEntity
      relation_field_name: relationFieldName
      read_only: true/false (default: false)
    # Additional list models...

```

### QML Configuration

Specifies paths for QML folder. The folders mock_imports and real_imports will be created in it.

```yaml
qml:
  folder_path: path/to/qml/folder

```
