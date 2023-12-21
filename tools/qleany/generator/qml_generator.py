from jinja2 import Environment, FileSystemLoader
import yaml
import os
import sys
import stringcase
import shutil
import uncrustify
import clang_format
import generation_dict_tools as tools
from pathlib import Path


def _get_generation_dict(
    real_imports_folder_path: str,
    mock_imports_folder_path: str,
    feature_by_name: dict,
    entities_by_name: dict,
    has_undo_redo: bool,
    create_undo_and_redo_singles: bool,
    list_models: list,
    singles: list,
    application_name: str,
    application_cpp_domain_name: str,
) -> dict:
    # generating controller files

    generation_dict = {}
    generation_dict["controllers"] = {}
    for feature_name, feature_data in feature_by_name.items():
        feature_snake_name = stringcase.snakecase(feature_name)
        feature_pascal_name = stringcase.pascalcase(feature_name)
        feature_camel_name = stringcase.camelcase(feature_name)

        crud = feature_data.get("CRUD", {})
        crud_enabled = crud.get("enabled", False)
        get_enabled = crud.get("get", {}).get("enabled", False)
        get_with_details_enabled = crud.get("get_with_details", {}).get(
            "enabled", False
        )
        get_all_enabled = crud.get("get_all", {}).get("enabled", False)
        create_enabled = crud.get("create", {}).get("enabled", False)
        update_enabled = crud.get("update", {}).get("enabled", False)
        remove_enabled = crud.get("remove", {}).get("enabled", False)

        # custom commands
        custom_commands = []
        for command in feature_data.get("commands", []):
            dto_in = command.get("dto", {}).get("in", {})
            custom_commands.append(
                {
                    "camel_name": stringcase.camelcase(command["name"]),
                    "pascal_name": stringcase.pascalcase(command["name"]),
                    "snake_name": stringcase.snakecase(command["name"]),
                    "dto_in_enabled": dto_in.get("enabled", True),
                    "dto_in_pascal_type_prefix": stringcase.pascalcase(
                        dto_in.get("type_prefix", "undefined")
                    ),
                }
            )

        # custom queries
        custom_queries = []
        for query in feature_data.get("queries", []):
            dto_in = query.get("dto", {}).get("in", {})
            custom_queries.append(
                {
                    "camel_name": stringcase.camelcase(query["name"]),
                    "pascal_name": stringcase.pascalcase(query["name"]),
                    "snake_name": stringcase.snakecase(query["name"]),
                    "dto_in_enabled": dto_in.get("enabled", True),
                    "dto_in_pascal_type_prefix": stringcase.pascalcase(
                        dto_in.get("type_prefix", "undefined")
                    ),
                }
            )

        generation_dict["controllers"][feature_pascal_name] = {
            "mock_controller_file": os.path.join(
                mock_imports_folder_path,
                "Controllers",
                f"{feature_pascal_name}Controller.qml",
            ),
            "mock_signals_file": os.path.join(
                mock_imports_folder_path,
                "Controllers",
                f"{feature_pascal_name}Signals.qml",
            ),
            "mock_template_path": "QML/mock_imports/controllers/",
            "mock_template": "controller.qml.jinja2",
            "mock_signals_template": "signals.qml.jinja2",
            "real_controller_file": os.path.join(
                real_imports_folder_path,
                "controllers",
                f"foreign_{feature_snake_name}_controller.h",
            ),
            "real_template_path": "QML/real_imports/controllers/",
            "real_template": "foreign_controller.h.jinja2",
            "feature_pascal_name": feature_pascal_name,
            "feature_camel_name": feature_camel_name,
            "feature_snake_name": feature_snake_name,
            "crud": {
                "enabled": crud_enabled,
                "get": get_enabled,
                "get_with_details": get_with_details_enabled,
                "get_all": get_all_enabled,
                "create": create_enabled,
                "update": update_enabled,
                "remove": remove_enabled,
                "entity_has_relation_fields": tools.does_entity_have_relation_fields(
                    feature_pascal_name, entities_by_name
                ),
            },
            "custom_commands": custom_commands,
            "custom_queries": custom_queries,
        }

    # add mock_custom_functions to the generation_dict["controllers"][feature_pascal_name] dict
    for feature_name, feature_data in feature_by_name.items():
        feature_pascal_name = stringcase.pascalcase(feature_name)
        generation_dict["controllers"][feature_pascal_name][
            "mock_custom_functions"
        ] = []
        commands = feature_data.get("commands", [])
        if commands:
            for command in commands:
                generation_dict["controllers"][feature_pascal_name][
                    "mock_custom_functions"
                ] += [stringcase.camelcase(command["name"])]

        queries = feature_data.get("queries", [])
        if queries:
            for query in queries:
                generation_dict["controllers"][feature_pascal_name][
                    "mock_custom_functions"
                ] += [stringcase.camelcase(query["name"])]

    # add qmldir:
    controller_qmldir_file = os.path.join(
        mock_imports_folder_path, "Controllers", "qmldir"
    )

    generation_dict["controller_qmldir_file"] = controller_qmldir_file

    # add CMakelists.txt:
    controller_cmakelists_file = os.path.join(
        real_imports_folder_path, "controllers", "CMakeLists.txt"
    )
    generation_dict["controller_cmakelists_file"] = controller_cmakelists_file

    # add "mock_controller_file" and "real_controller_file" to the generation_dict["real_controller_files"] list
    generation_dict["real_controller_files"] = []
    generation_dict["mock_controller_files"] = []
    for _, controller in generation_dict["controllers"].items():
        generation_dict["real_controller_files"].append(
            controller["real_controller_file"]
        )
        generation_dict["mock_controller_files"].append(
            controller["mock_controller_file"]
        )
        generation_dict["mock_controller_files"].append(controller["mock_signals_file"])

    # add event dispatcher
    real_event_dispatcher_file = os.path.join(
        real_imports_folder_path,
        "controllers",
        "foreign_event_dispatcher.h",
    )
    generation_dict["real_controller_files"].append(real_event_dispatcher_file)
    generation_dict["real_event_dispatcher_file"] = real_event_dispatcher_file

    mock_event_dispatcher_file = os.path.join(
        mock_imports_folder_path,
        "Controllers",
        "EventDispatcher.qml",
    )
    generation_dict["mock_controller_files"].append(mock_event_dispatcher_file)
    generation_dict["mock_event_dispatcher_file"] = mock_event_dispatcher_file

    # add undo redo
    generation_dict["has_undo_redo"] = has_undo_redo
    if has_undo_redo:
        real_undo_redo_controller_file = os.path.join(
            real_imports_folder_path, "controllers", "foreign_undo_redo_controller.h"
        )
        generation_dict["real_controller_files"].append(real_undo_redo_controller_file)
        generation_dict[
            "real_undo_redo_controller_file"
        ] = real_undo_redo_controller_file

        mock_undo_redo_controller_file = os.path.join(
            mock_imports_folder_path, "Controllers", "UndoRedoController.qml"
        )
        generation_dict["mock_controller_files"].append(mock_undo_redo_controller_file)
        generation_dict[
            "mock_undo_redo_controller_file"
        ] = mock_undo_redo_controller_file

        mock_undo_redo_signals_file = os.path.join(
            mock_imports_folder_path, "Controllers", "UndoRedoSignals.qml"
        )
        generation_dict["mock_controller_files"].append(mock_undo_redo_signals_file)
        generation_dict["mock_undo_redo_signals_file"] = mock_undo_redo_signals_file

    # progress signals
    mock_progress_signals_file = os.path.join(
        mock_imports_folder_path, "Controllers", "ProgressSignals.qml"
    )
    generation_dict["mock_controller_files"].append(mock_progress_signals_file)
    generation_dict["mock_progress_signals_file"] = mock_progress_signals_file

    # error signals
    mock_error_signals_file = os.path.join(
        mock_imports_folder_path, "Controllers", "ErrorSignals.qml"
    )
    generation_dict["mock_controller_files"].append(mock_error_signals_file)
    generation_dict["mock_error_signals_file"] = mock_error_signals_file

    # QCoro::QmlTask mock
    qcoro_qmltask_file = os.path.join(
        mock_imports_folder_path, "Controllers", "QCoroQmlTask.qml"
    )
    generation_dict["mock_controller_files"].append(qcoro_qmltask_file)
    generation_dict["qcoro_qmltask_file"] = qcoro_qmltask_file

    # add models
    generation_dict["models"] = []
    generation_dict["mock_model_files"] = []
    generation_dict["real_model_files"] = []

    generation_dict["create_undo_and_redo_singles"] = create_undo_and_redo_singles

    for list_model in list_models:
        list_model_name = list_model["name"]

        entity_name = list_model["entity"]
        entity_name_snake = stringcase.snakecase(entity_name)
        entity_name_pascal = stringcase.pascalcase(entity_name)
        entity_name_spinal = stringcase.spinalcase(entity_name)
        entity_name_camel = stringcase.camelcase(entity_name)

        # displayed_field
        displayed_field = list_model.get("displayed_field", "id")
        displayed_field_snake = stringcase.snakecase(displayed_field)
        displayed_field_pascal = stringcase.pascalcase(displayed_field)
        displayed_field_spinal = stringcase.spinalcase(displayed_field)
        displayed_field_camel = stringcase.camelcase(displayed_field)

        related_name = list_model.get("in_relation_of", "")
        related_name_snake = stringcase.snakecase(related_name)
        related_name_pascal = stringcase.pascalcase(related_name)
        related_name_spinal = stringcase.spinalcase(related_name)
        related_name_camel = stringcase.camelcase(related_name)

        related_field_name = list_model.get("relation_field_name", "")
        related_field_name_snake = stringcase.snakecase(related_field_name)
        related_field_name_pascal = stringcase.pascalcase(related_field_name)
        related_field_name_spinal = stringcase.spinalcase(related_field_name)
        related_field_name_camel = stringcase.camelcase(related_field_name)
        is_related_list = related_name != "" and related_field_name != ""

        list_model_name = list_model["name"]
        if list_model_name == "auto":
            if is_related_list:
                list_model_name = (
                    entity_name_pascal
                    + "ListModelFrom"
                    + related_name_pascal
                    + related_field_name_pascal
                )
            else:
                list_model_name = entity_name_pascal + "ListModel"

        list_model_pascal_name = stringcase.pascalcase(list_model_name)
        list_model_snake_name = stringcase.snakecase(list_model_name)
        list_model_spinal_name = stringcase.spinalcase(list_model_name)
        list_model_camel_name = stringcase.camelcase(list_model_name)

        mock_model_file = os.path.join(
            mock_imports_folder_path,
            "Models",
            f"{list_model_pascal_name}.qml",
        )
        generation_dict["mock_model_files"].append(mock_model_file)
        real_model_file = os.path.join(
            real_imports_folder_path,
            "models",
            f"foreign_{list_model_snake_name}_model.h",
        )
        generation_dict["real_model_files"].append(real_model_file)

        generation_dict["models"].append(
            {
                "mock_model_file": mock_model_file,
                "mock_template_path": "QML/mock_imports/models/",
                "mock_template": "list_model.qml.jinja2",
                "real_model_file": real_model_file,
                "real_template_path": "QML/real_imports/models/",
                "real_template": "foreign_model.h.jinja2",
                "list_model_pascal_name": list_model_pascal_name,
                "list_model_snake_name": list_model_snake_name,
                "list_model_spinal_name": list_model_spinal_name,
                "list_model_camel_name": list_model_camel_name,
                "relation_name_camel": stringcase.camelcase(
                    list_model.get("in_relation_of", "")
                ),
                "is_in_relation_of_another_entity": bool(
                    list_model.get("in_relation_of", False)
                ),
                "fields": tools.get_fields_without_foreign_entities(
                    entities_by_name[list_model["entity"]]["fields"],
                    entities_by_name,
                    list_model["entity"],
                ),
            }
        )

    # add qmldir:
    model_qmldir_file = os.path.join(mock_imports_folder_path, "Models", "qmldir")

    generation_dict["model_qmldir_file"] = model_qmldir_file

    # add CMakelists.txt:
    model_cmakelists_file = os.path.join(
        real_imports_folder_path, "models", "CMakeLists.txt"
    )
    generation_dict["model_cmakelists_file"] = model_cmakelists_file

    # add application_name
    generation_dict["application_name"] = application_name
    generation_dict["application_cpp_domain_name"] = application_cpp_domain_name

    return generation_dict


def _generate_mock_controller_file(
    root_path: str,
    controller: dict,
    generation_dict: dict,
    files_to_be_generated: dict[str, bool],
):
    # generate the mock controller file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(controller["mock_controller_file"], False):
        return

    # Create the jinja2 environment
    template_path = os.path.join("templates", controller["mock_template_path"])
    env = Environment(loader=FileSystemLoader(template_path))
    # Load the template
    template = env.get_template(controller["mock_template"])

    # Render the template
    output = template.render(
        controller=controller,
    )

    output_file = os.path.join(root_path, controller["mock_controller_file"])

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Write the output to the file
    with open(output_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {output_file}")


def _generate_mock_signals_file(
    root_path: str,
    controller: dict,
    generation_dict: dict,
    files_to_be_generated: dict[str, bool],
):
    # generate the mock signals file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(controller["mock_signals_file"], False):
        return

    # Create the jinja2 environment
    template_path = os.path.join("templates", controller["mock_template_path"])
    env = Environment(loader=FileSystemLoader(template_path))
    # Load the template
    template = env.get_template(controller["mock_signals_template"])

    # Render the template
    output = template.render(
        controller=controller,
    )

    output_file = os.path.join(root_path, controller["mock_signals_file"])

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Write the output to the file
    with open(output_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {output_file}")


def _generate_mock_undo_redo_controller_file(
    root_path: str,
    generation_dict: dict,
    files_to_be_generated: dict[str, bool],
):
    # generate the mock undo redo controller file if in the files_to_be_generated dict the value is True
    undo_redo_controller_file = generation_dict["mock_undo_redo_controller_file"]

    if not files_to_be_generated.get(undo_redo_controller_file, False):
        return

    undo_redo_controller_file = os.path.join(
        root_path,
        undo_redo_controller_file,
    )

    # Create the jinja2 environment
    template_path = os.path.join("templates", "QML", "mock_imports", "controllers")
    env = Environment(loader=FileSystemLoader(template_path))
    # Load the template
    template = env.get_template("undo_redo_controller.qml.jinja2")

    # Render the template
    output = template.render()

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(undo_redo_controller_file), exist_ok=True)

    # Write the output to the file
    with open(undo_redo_controller_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {undo_redo_controller_file}")


def _generate_mock_undo_redo_signals_file(
    root_path: str,
    generation_dict: dict,
    files_to_be_generated: dict[str, bool],
):
    # generate the mock undo redo signals file if in the files_to_be_generated dict the value is True
    undo_redo_signals_file = generation_dict["mock_undo_redo_signals_file"]

    if not files_to_be_generated.get(undo_redo_signals_file, False):
        return

    undo_redo_signals_file = os.path.join(
        root_path,
        undo_redo_signals_file,
    )

    # Create the jinja2 environment
    template_path = os.path.join("templates", "QML", "mock_imports", "controllers")
    env = Environment(loader=FileSystemLoader(template_path))
    # Load the template
    template = env.get_template("undo_redo_signals.qml.jinja2")

    # Render the template
    output = template.render()

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(undo_redo_signals_file), exist_ok=True)

    # Write the output to the file
    with open(undo_redo_signals_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {undo_redo_signals_file}")


def _generate_mock_progress_signals_file(
    root_path: str,
    generation_dict: dict,
    files_to_be_generated: dict[str, bool],
):
    # generate the mock progress signals file if in the files_to_be_generated dict the value is True
    progress_signals_file = generation_dict["mock_progress_signals_file"]

    if not files_to_be_generated.get(progress_signals_file, False):
        return

    progress_signals_file = os.path.join(
        root_path,
        progress_signals_file,
    )

    # Create the jinja2 environment
    template_path = os.path.join("templates", "QML", "mock_imports", "controllers")
    env = Environment(loader=FileSystemLoader(template_path))
    # Load the template
    template = env.get_template("progress_signals.qml.jinja2")

    # Render the template
    output = template.render()

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(progress_signals_file), exist_ok=True)

    # Write the output to the file
    with open(progress_signals_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {progress_signals_file}")


def _generate_mock_error_signals_file(
    root_path: str,
    generation_dict: dict,
    files_to_be_generated: dict[str, bool],
):
    # generate the mock error signals file if in the files_to_be_generated dict the value is True
    error_signals_file = generation_dict["mock_error_signals_file"]

    if not files_to_be_generated.get(error_signals_file, False):
        return

    error_signals_file = os.path.join(
        root_path,
        error_signals_file,
    )

    # Create the jinja2 environment
    template_path = os.path.join("templates", "QML", "mock_imports", "controllers")
    env = Environment(loader=FileSystemLoader(template_path))
    # Load the template
    template = env.get_template("error_signals.qml.jinja2")

    # Render the template
    output = template.render()

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(error_signals_file), exist_ok=True)

    # Write the output to the file
    with open(error_signals_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {error_signals_file}")


def _generate_mock_event_dispatcher_file(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool]
):
    # generate the mock event dispatcher file if in the files_to_be_generated dict the value is True
    event_dispatcher_file = generation_dict["mock_event_dispatcher_file"]

    if not files_to_be_generated.get(event_dispatcher_file, False):
        return

    event_dispatcher_file = os.path.join(
        root_path,
        event_dispatcher_file,
    )

    # Create the jinja2 environment
    template_path = os.path.join("templates", "QML", "mock_imports", "controllers")
    env = Environment(loader=FileSystemLoader(template_path))
    # Load the template
    template = env.get_template("event_dispatcher.qml.jinja2")

    # Render the template
    output = template.render(
        controllers=generation_dict["controllers"],
        has_undo_redo=generation_dict["has_undo_redo"],
    )

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(event_dispatcher_file), exist_ok=True)

    # Write the output to the file
    with open(event_dispatcher_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {event_dispatcher_file}")


def _generate_mock_qcoro_qmltask_file(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool]
):
    # generate the mock qcoro qmltask file if in the files_to_be_generated dict the value is True
    qcoro_qmltask_file = generation_dict["qcoro_qmltask_file"]

    if not files_to_be_generated.get(qcoro_qmltask_file, False):
        return

    qcoro_qmltask_file = os.path.join(
        root_path,
        qcoro_qmltask_file,
    )

    # Create the jinja2 environment
    template_path = os.path.join("templates", "QML", "mock_imports", "controllers")
    env = Environment(loader=FileSystemLoader(template_path))
    # Load the template
    template = env.get_template("qcoro_qmltask.qml.jinja2")

    # Render the template
    output = template.render()

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(qcoro_qmltask_file), exist_ok=True)

    # Write the output to the file
    with open(qcoro_qmltask_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {qcoro_qmltask_file}")


def _generate_mock_controllers_qmldir_file(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool]
):
    # generate the mock qmldir file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(generation_dict["controller_qmldir_file"], False):
        return

    # Create the jinja2 environment
    env = Environment(
        loader=FileSystemLoader("templates/QML/mock_imports/controllers/")
    )
    # Load the template
    template = env.get_template("qmldir_template.jinja2")

    singleton_list = []
    for _, controller in generation_dict["controllers"].items():
        name = controller["feature_pascal_name"]
        singleton_list.append(f"singleton {name}Controller 1.0 {name}Controller.qml")
        singleton_list.append(f"singleton {name}Signals 1.0 {name}Signals.qml")

    if generation_dict["has_undo_redo"]:
        singleton_list.append(
            f"singleton UndoRedoController 1.0 UndoRedoController.qml"
        )
        singleton_list.append(f"singleton UndoRedoSignals 1.0 UndoRedoSignals.qml")
    singleton_list.append(f"singleton ProgressSignals 1.0 ProgressSignals.qml")
    singleton_list.append(f"singleton ErrorSignals 1.0 ErrorSignals.qml")
    singleton_list.append(f"singleton EventDispatcher 1.0 EventDispatcher.qml")
    singleton_list.append(f"QCoroQmlTask 1.0 QCoroQmlTask.qml")

    # Render the template
    output = template.render(singleton_list=singleton_list)

    output_file = os.path.join(root_path, generation_dict["controller_qmldir_file"])

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Write the output to the file
    with open(output_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {output_file}")


def _generate_real_controller_file(
    root_path: str,
    controller: dict,
    generation_dict: dict,
    files_to_be_generated: dict[str, bool],
    uncrustify_config_file: str,
):
    # generate the real controller file if in the files_to_be_generated dict the value is True
    real_controller_file = controller["real_controller_file"]

    if not files_to_be_generated.get(real_controller_file, False):
        return

    # Create the jinja2 environment
    template_path = os.path.join("templates", controller["real_template_path"])
    env = Environment(loader=FileSystemLoader(template_path))
    # Load the template
    template = env.get_template(controller["real_template"])

    # Render the template
    output = template.render(
        controller=controller,
        application_cpp_domain_name=generation_dict["application_cpp_domain_name"],
    )

    output_file = os.path.join(root_path, real_controller_file)

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Write the output to the file
    with open(output_file, "w") as fh:
        fh.write(output)

    # if uncrustify_config_file:
    #     uncrustify.run_uncrustify(real_controller_file, uncrustify_config_file)
    clang_format.run_clang_format(output_file)

    print(f"Successfully wrote file {output_file}")


def _generate_real_undo_redo_controller_file(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool]
):
    # generate the real undo redo controller file if in the files_to_be_generated dict the value is True
    undo_redo_controller_file = generation_dict["real_undo_redo_controller_file"]

    if not files_to_be_generated.get(undo_redo_controller_file, False):
        return

    undo_redo_controller_file = os.path.join(
        root_path,
        undo_redo_controller_file,
    )

    # Create the jinja2 environment
    template_path = os.path.join("templates", "QML", "real_imports", "controllers")
    env = Environment(loader=FileSystemLoader(template_path))
    # Load the template
    template = env.get_template("foreign_undo_redo_controller.h.jinja2")

    # Render the template
    output = template.render(
        application_cpp_domain_name=generation_dict["application_cpp_domain_name"],
    )

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(undo_redo_controller_file), exist_ok=True)

    # Write the output to the file
    with open(undo_redo_controller_file, "w") as fh:
        fh.write(output)

    # if uncrustify_config_file:
    #     uncrustify.run_uncrustify(undo_redo_controller_file, uncrustify_config_file)
    clang_format.run_clang_format(undo_redo_controller_file)

    print(f"Successfully wrote file {undo_redo_controller_file}")


def _generate_real_event_dispatcher_file(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool]
):
    # generate the real event dispatcher file if in the files_to_be_generated dict the value is True
    event_dispatcher_file = generation_dict["real_event_dispatcher_file"]

    if not files_to_be_generated.get(event_dispatcher_file, False):
        return

    event_dispatcher_file = os.path.join(
        root_path,
        event_dispatcher_file,
    )

    # Create the jinja2 environment
    template_path = os.path.join("templates", "QML", "real_imports", "controllers")
    env = Environment(loader=FileSystemLoader(template_path))
    # Load the template
    template = env.get_template("foreign_event_dispatcher.h.jinja2")

    # Render the template
    output = template.render(
        application_cpp_domain_name=generation_dict["application_cpp_domain_name"],
    )

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(event_dispatcher_file), exist_ok=True)

    # Write the output to the file
    with open(event_dispatcher_file, "w") as fh:
        fh.write(output)

    # if uncrustify_config_file:
    #     uncrustify.run_uncrustify(event_dispatcher_file, uncrustify_config_file)
    clang_format.run_clang_format(event_dispatcher_file)

    print(f"Successfully wrote file {event_dispatcher_file}")


def _generate_real_controllers_cmakelists_file(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool]
):
    controller_cmakelists_file = generation_dict["controller_cmakelists_file"]

    # generate the real cmakelists file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(controller_cmakelists_file, False):
        return

    output_file = os.path.join(root_path, controller_cmakelists_file)

    # Create the jinja2 environment
    env = Environment(
        loader=FileSystemLoader("templates/QML/real_imports/controllers/")
    )
    # Load the template
    template = env.get_template("cmakelists.txt.jinja2")

    files = generation_dict["real_controller_files"]
    relative_files = []
    for file in files:
        relative_files.append(
            os.path.relpath(
                os.path.join(root_path, file), os.path.dirname(output_file)
            ).replace("\\", "/")
        )

    # Render the template
    output = template.render(
        files=relative_files,
        application_name=generation_dict["application_name"],
    )

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Write the output to the file
    with open(output_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {output_file}")


def _generate_mock_model_file(
    root_path: str,
    model: dict,
    generation_dict: dict,
    files_to_be_generated: dict[str, bool],
):
    # generate the mock model file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(model["mock_model_file"], False):
        return

    # Create the jinja2 environment
    template_path = os.path.join("templates", model["mock_template_path"])
    env = Environment(loader=FileSystemLoader(template_path))
    # Load the template
    template = env.get_template(model["mock_template"])

    # Render the template
    output = template.render(
        model=model,
    )

    output_file = os.path.join(root_path, model["mock_model_file"])

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Write the output to the file
    with open(output_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {output_file}")


def _generate_real_model_file(
    root_path: str,
    model: dict,
    generation_dict: dict,
    files_to_be_generated: dict[str, bool],
):
    # generate the real model file if in the files_to_be_generated dict the value is True
    real_model_file = model["real_model_file"]

    if not files_to_be_generated.get(real_model_file, False):
        return

    # Create the jinja2 environment
    template_path = os.path.join("templates", model["real_template_path"])
    env = Environment(loader=FileSystemLoader(template_path))
    # Load the template
    template = env.get_template(model["real_template"])

    # Render the template
    output = template.render(
        model=model,
        application_cpp_domain_name=generation_dict["application_cpp_domain_name"],
    )

    output_file = os.path.join(root_path, real_model_file)

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Write the output to the file
    with open(output_file, "w") as fh:
        fh.write(output)

    # if uncrustify_config_file:
    #     uncrustify.run_uncrustify(real_model_file, uncrustify_config_file)
    clang_format.run_clang_format(output_file)

    print(f"Successfully wrote file {output_file}")


def _generate_mock_models_qmldir_file(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool]
):
    # generate the mock qmldir file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(generation_dict["model_qmldir_file"], False):
        return

    # Create the jinja2 environment
    env = Environment(loader=FileSystemLoader("templates/QML/mock_imports/models/"))
    # Load the template
    template = env.get_template("qmldir_template.jinja2")

    models_to_declare_list = []
    for model in generation_dict["models"]:
        name = model["list_model_pascal_name"]
        models_to_declare_list.append(f"{name} 1.0 {name}.qml")

    # Render the template
    output = template.render(models_to_declare_list=models_to_declare_list)

    output_file = os.path.join(root_path, generation_dict["model_qmldir_file"])

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Write the output to the file
    with open(output_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {output_file}")


def _generate_real_models_cmakelists_file(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool]
):
    controller_cmakelists_file = generation_dict["model_cmakelists_file"]

    # generate the real cmakelists file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(controller_cmakelists_file, False):
        return

    output_file = os.path.join(root_path, controller_cmakelists_file)

    # Create the jinja2 environment
    env = Environment(loader=FileSystemLoader("templates/QML/real_imports/models/"))
    # Load the template
    template = env.get_template("cmakelists.txt.jinja2")

    files = generation_dict["real_model_files"]
    relative_files = []
    for file in files:
        relative_files.append(
            os.path.relpath(
                os.path.join(root_path, file), os.path.dirname(output_file)
            ).replace("\\", "/")
        )

    # Render the template
    output = template.render(
        files=relative_files,
        application_name=generation_dict["application_name"],
    )

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Write the output to the file
    with open(output_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {output_file}")


def generate_qml_files(
    root_path: str,
    manifest_file: str,
    files_to_be_generated: dict[str, bool] = {},
    uncrustify_config_file: str = None,
):
    with open(manifest_file, "r") as stream:
        try:
            manifest_data = yaml.safe_load(stream)
        except yaml.YAMLError as exc:
            print(exc)
            return

    application_name = manifest_data.get("global", {}).get(
        "application_name", "example"
    )
    application_name = stringcase.spinalcase(application_name)

    application_cpp_domain_name = manifest_data.get("global", {}).get(
        "application_cpp_domain_name", "Example"
    )

    entities_data = manifest_data.get("entities", {})
    entities_list = entities_data.get("list", [])
    # remove entities that are not to be generated
    entities_list = [entity for entity in entities_list]

    # Organize entities by name for easier lookup
    entities_by_name = {entity["name"]: entity for entity in entities_list}

    application_data = manifest_data.get("application", {})
    feature_list = application_data.get("features", [])

    # Organize feature_list by name for easier lookup
    feature_by_name = {feature["name"]: feature for feature in feature_list}

    controller_data = manifest_data.get("controller", {})
    has_undo_redo = controller_data.get("create_undo_redo_controller", False)

    presenter_data = manifest_data.get("presenter", {})
    create_undo_and_redo_singles = presenter_data.get(
        "create_undo_and_redo_singles", False
    )
    list_models = presenter_data.get("list_models", [])
    singles = presenter_data.get("singles", [])

    qml_data = manifest_data.get("qml", [])

    generation_dict = _get_generation_dict(
        qml_data["real_imports_folder_path"],
        qml_data["mock_imports_folder_path"],
        feature_by_name,
        entities_by_name,
        has_undo_redo,
        create_undo_and_redo_singles,
        list_models,
        singles,
        application_name,
        application_cpp_domain_name,
    )

    # generate mock files
    for _, controller in generation_dict["controllers"].items():
        _generate_mock_controller_file(
            root_path, controller, generation_dict, files_to_be_generated
        )
        _generate_mock_signals_file(
            root_path, controller, generation_dict, files_to_be_generated
        )

    _generate_mock_event_dispatcher_file(
        root_path, generation_dict, files_to_be_generated
    )
    if has_undo_redo:
        _generate_mock_undo_redo_controller_file(
            root_path, generation_dict, files_to_be_generated
        )
        _generate_mock_undo_redo_signals_file(
            root_path, generation_dict, files_to_be_generated
        )

    _generate_mock_progress_signals_file(
        root_path, generation_dict, files_to_be_generated
    )
    _generate_mock_error_signals_file(root_path, generation_dict, files_to_be_generated)

    _generate_mock_qcoro_qmltask_file(root_path, generation_dict, files_to_be_generated)

    # generate mock qmldir file
    _generate_mock_controllers_qmldir_file(
        root_path, generation_dict, files_to_be_generated
    )

    # generate real files
    for _, controller in generation_dict["controllers"].items():
        _generate_real_controller_file(
            root_path,
            controller,
            generation_dict,
            files_to_be_generated,
            uncrustify_config_file,
        )

    _generate_real_event_dispatcher_file(
        root_path, generation_dict, files_to_be_generated
    )

    if has_undo_redo:
        _generate_real_undo_redo_controller_file(
            root_path, generation_dict, files_to_be_generated
        )

    # generate real CMakeLists.txt file
    _generate_real_controllers_cmakelists_file(
        root_path, generation_dict, files_to_be_generated
    )

    # models
    for model in generation_dict["models"]:
        _generate_mock_model_file(
            root_path, model, generation_dict, files_to_be_generated
        )
        _generate_real_model_file(
            root_path, model, generation_dict, files_to_be_generated
        )

    # generate mock qmldir file
    _generate_mock_models_qmldir_file(root_path, generation_dict, files_to_be_generated)
    _generate_real_models_cmakelists_file(
        root_path, generation_dict, files_to_be_generated
    )


def get_files_to_be_generated(
    manifest_file: str, files_to_be_generated: dict[str, bool] = {}
) -> list[str]:
    """
    Get the list of files that need to be generated based on the manifest file
    """
    # Read the manifest file
    with open(manifest_file, "r") as stream:
        try:
            manifest_data = yaml.safe_load(stream)
        except yaml.YAMLError as exc:
            print(exc)
            return

    application_name = manifest_data.get("global", {}).get(
        "application_name", "example"
    )
    application_name = stringcase.spinalcase(application_name)

    application_cpp_domain_name = manifest_data.get("global", {}).get(
        "application_cpp_domain_name", "Example"
    )

    entities_data = manifest_data.get("entities", {})
    entities_list = entities_data.get("list", [])
    # remove entities that are not to be generated
    entities_list = [entity for entity in entities_list]

    # Organize entities by name for easier lookup
    entities_by_name = {entity["name"]: entity for entity in entities_list}

    application_data = manifest_data.get("application", {})

    controller_data = manifest_data.get("controller", {})
    has_undo_redo = controller_data.get("create_undo_redo_controller", False)

    feature_list = application_data.get("features", [])

    presenter_data = manifest_data.get("presenter", {})
    create_undo_and_redo_singles = presenter_data.get(
        "create_undo_and_redo_singles", False
    )

    list_models = presenter_data.get("list_models", [])
    singles = presenter_data.get("singles", [])

    # Organize feature_list by name for easier lookup
    feature_by_name = {feature["name"]: feature for feature in feature_list}

    qml_data = manifest_data.get("qml", [])

    files = []
    generation_dict = _get_generation_dict(
        qml_data["real_imports_folder_path"],
        qml_data["mock_imports_folder_path"],
        feature_by_name,
        entities_by_name,
        has_undo_redo,
        create_undo_and_redo_singles,
        list_models,
        singles,
        application_name,
        application_cpp_domain_name,
    )
    files += generation_dict["real_controller_files"]
    files += generation_dict["mock_controller_files"]

    # for _, controller in generation_dict["controllers"].items():
    #     files += feature["real_controller_files"]

    # # add CMakelists.txt:
    controller_cmakelists_file = generation_dict["controller_cmakelists_file"]
    files.append(controller_cmakelists_file)

    # # add qmldir:
    controller_qmldir_file = generation_dict["controller_qmldir_file"]
    files.append(controller_qmldir_file)

    # list models 
    files += generation_dict["real_model_files"]
    files += generation_dict["mock_model_files"]

    # # add CMakelists.txt:
    model_cmakelists_file = generation_dict["model_cmakelists_file"]
    files.append(model_cmakelists_file)

    # # add qmldir:
    model_qmldir_file = generation_dict["model_qmldir_file"]
    files.append(model_qmldir_file)

    # strip from files if the value in files_to_be_generated is False
    if files_to_be_generated:
        for path, generate in files_to_be_generated.items():
            if not generate:
                files.remove(path)

    return files


# generate the files into the preview folder
def preview_qml_files(
    root_path: str,
    manifest_file: str,
    files_to_be_generated: dict[str, bool] = {},
    uncrustify_config_file: str = None,
):
    manifest_preview_file = "temp/manifest_preview.yaml"

    # make a copy of the manifest file into temp/manifest_preview.yaml
    shutil.copy(manifest_file, manifest_preview_file)

    # modify the manifest file to generate the files into the preview folder
    with open(manifest_preview_file, "r") as fh:
        manifest = yaml.safe_load(fh)

    # remove .. from the path
    manifest["qml"]["mock_imports_folder_path"] = manifest["qml"][
        "mock_imports_folder_path"
    ].replace("..", "")
    manifest["qml"]["real_imports_folder_path"] = manifest["qml"][
        "real_imports_folder_path"
    ].replace("..", "")

    # write the modified manifest file
    with open(manifest_preview_file, "w") as fh:
        yaml.dump(manifest, fh)

    root_path = os.path.join(root_path, "qleany_preview")

    # remove .. from the path
    if files_to_be_generated:
        preview_files_to_be_generated = {}
        for path, value in files_to_be_generated.items():
            preview_files_to_be_generated[path.replace("..", "")] = value

        generate_qml_files(
            root_path,
            manifest_preview_file,
            preview_files_to_be_generated,
            uncrustify_config_file,
        )

    else:
        generate_qml_files(root_path, manifest_preview_file, {}, uncrustify_config_file)


# Main execution
if __name__ == "__main__":
    full_path = Path(__file__).resolve().parent

    # Add the current directory to the path so that we can import the generated files
    sys.path.append(full_path)

    # Set the current directory to the generator directory
    os.chdir(full_path)

    if len(sys.argv) > 1:
        manifest_arg = sys.argv[1]
        if manifest_arg.endswith(".yaml") or manifest_arg.endswith(".yml"):
            manifest_file = manifest_arg
            root_path = Path(manifest_file).parent

            if len(sys.argv) > 2 and sys.argv[2] == "--preview":
                preview_qml_files(root_path, manifest_file)
            else:
                generate_qml_files(root_path, manifest_file)
        else:
            print("Error: Manifest file must be named 'qleany.yaml' or 'qleany.yml'")
    else:
        print("Error: Please provide the manifest file as an argument")
