from jinja2 import Environment, FileSystemLoader
import yaml
import os
import sys
import stringcase
import shutil
import uncrustify
import clang_format_runner as clang_format_runner
import generation_dict_tools as tools
from pathlib import Path


def _get_generation_dict(
    folder_path,
    feature_by_name: dict,
    entities_by_name: dict,
    has_undo_redo: bool,
    create_undo_and_redo_singles: bool,
    list_models: list,
    singles: list,
    application_name: str,
    application_cpp_domain_name: str,
) -> dict:
    
    real_imports_folder_path = os.path.join(folder_path, "real_imports")
    mock_imports_folder_path = os.path.join(folder_path, "mock_imports")

    # generating interactor files

    generation_dict = {}
    generation_dict["interactors"] = {}
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

        generation_dict["interactors"][feature_pascal_name] = {
            "mock_interactor_file": os.path.join(
                mock_imports_folder_path,
                "Interactors",
                f"{feature_pascal_name}Interactor.qml",
            ),
            "mock_signals_file": os.path.join(
                mock_imports_folder_path,
                "Interactors",
                f"{feature_pascal_name}Signals.qml",
            ),
            "mock_template_path": "QML/mock_imports/interactors/",
            "mock_template": "interactor.qml.jinja2",
            "mock_signals_template": "signals.qml.jinja2",
            "real_interactor_file": os.path.join(
                real_imports_folder_path,
                "interactors",
                f"foreign_{feature_snake_name}_interactor.h",
            ),
            "real_template_path": "QML/real_imports/interactors/",
            "real_template": "foreign_interactor.h.jinja2",
            "feature_pascal_name": feature_pascal_name,
            "feature_camel_name": feature_camel_name,
            "feature_snake_name": feature_snake_name,
            "crud": {
                "enabled": crud_enabled,
                "get": get_enabled,
                "get_with_details": get_with_details_enabled,
                "get_all": get_all_enabled,
                "create": create_enabled,
                "update_": update_enabled,
                "remove": remove_enabled,
                "entity_has_relation_fields": tools.does_entity_have_relation_fields(
                    feature_pascal_name, entities_by_name, False
                ),
            },
            "custom_commands": custom_commands,
            "custom_queries": custom_queries,
        }

    # add mock_custom_functions to the generation_dict["interactors"][feature_pascal_name] dict
    for feature_name, feature_data in feature_by_name.items():
        feature_pascal_name = stringcase.pascalcase(feature_name)
        generation_dict["interactors"][feature_pascal_name][
            "mock_custom_functions"
        ] = []
        commands = feature_data.get("commands", [])
        if commands:
            for command in commands:
                generation_dict["interactors"][feature_pascal_name][
                    "mock_custom_functions"
                ] += [stringcase.camelcase(command["name"])]

        queries = feature_data.get("queries", [])
        if queries:
            for query in queries:
                generation_dict["interactors"][feature_pascal_name][
                    "mock_custom_functions"
                ] += [stringcase.camelcase(query["name"])]

    # add qmldir:
    interactor_qmldir_file = os.path.join(
        mock_imports_folder_path, "Interactors", "qmldir"
    )

    generation_dict["interactor_qmldir_file"] = interactor_qmldir_file

    # add common real CmakeLists.txt file
    common_real_cmakelists_file = os.path.join(
        real_imports_folder_path, "CMakeLists.txt"
    )
    generation_dict["common_real_cmakelists_file"] = common_real_cmakelists_file

    # add realqmlmodules.cmake file
    real_qml_modules_file = os.path.join(
        folder_path, "realqmlmodules.cmake"
    )
    generation_dict["real_qml_modules_file"] = real_qml_modules_file

    # add CMakelists.txt:
    interactor_cmakelists_file = os.path.join(
        real_imports_folder_path, "interactors", "CMakeLists.txt"
    )
    generation_dict["interactor_cmakelists_file"] = interactor_cmakelists_file

    # add "mock_interactor_file" and "real_interactor_file" to the generation_dict["real_interactor_files"] list
    generation_dict["real_interactor_files"] = []
    generation_dict["mock_interactor_files"] = []
    for _, interactor in generation_dict["interactors"].items():
        generation_dict["real_interactor_files"].append(
            interactor["real_interactor_file"]
        )
        generation_dict["mock_interactor_files"].append(
            interactor["mock_interactor_file"]
        )
        generation_dict["mock_interactor_files"].append(interactor["mock_signals_file"])

    # add event dispatcher
    real_event_dispatcher_file = os.path.join(
        real_imports_folder_path,
        "interactors",
        "foreign_event_dispatcher.h",
    )
    generation_dict["real_interactor_files"].append(real_event_dispatcher_file)
    generation_dict["real_event_dispatcher_file"] = real_event_dispatcher_file

    mock_event_dispatcher_file = os.path.join(
        mock_imports_folder_path,
        "Interactors",
        "EventDispatcher.qml",
    )
    generation_dict["mock_interactor_files"].append(mock_event_dispatcher_file)
    generation_dict["mock_event_dispatcher_file"] = mock_event_dispatcher_file

    # add undo redo
    generation_dict["has_undo_redo"] = has_undo_redo
    if has_undo_redo:
        real_undo_redo_interactor_file = os.path.join(
            real_imports_folder_path, "interactors", "foreign_undo_redo_interactor.h"
        )
        generation_dict["real_interactor_files"].append(real_undo_redo_interactor_file)
        generation_dict[
            "real_undo_redo_interactor_file"
        ] = real_undo_redo_interactor_file

        mock_undo_redo_interactor_file = os.path.join(
            mock_imports_folder_path, "Interactors", "UndoRedoInteractor.qml"
        )
        generation_dict["mock_interactor_files"].append(mock_undo_redo_interactor_file)
        generation_dict[
            "mock_undo_redo_interactor_file"
        ] = mock_undo_redo_interactor_file

        mock_undo_redo_signals_file = os.path.join(
            mock_imports_folder_path, "Interactors", "UndoRedoSignals.qml"
        )
        generation_dict["mock_interactor_files"].append(mock_undo_redo_signals_file)
        generation_dict["mock_undo_redo_signals_file"] = mock_undo_redo_signals_file

    # progress signals
    mock_progress_signals_file = os.path.join(
        mock_imports_folder_path, "Interactors", "ProgressSignals.qml"
    )
    generation_dict["mock_interactor_files"].append(mock_progress_signals_file)
    generation_dict["mock_progress_signals_file"] = mock_progress_signals_file

    # error signals
    mock_error_signals_file = os.path.join(
        mock_imports_folder_path, "Interactors", "ErrorSignals.qml"
    )
    generation_dict["mock_interactor_files"].append(mock_error_signals_file)
    generation_dict["mock_error_signals_file"] = mock_error_signals_file

    # QCoro::QmlTask mock
    qcoro_qmltask_file = os.path.join(
        mock_imports_folder_path, "Interactors", "QCoroQmlTask.qml"
    )
    generation_dict["mock_interactor_files"].append(qcoro_qmltask_file)
    generation_dict["qcoro_qmltask_file"] = qcoro_qmltask_file

    # ---- add models
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
            f"foreign_{list_model_snake_name}.h",
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

    # ---- add singles
    generation_dict["singles"] = []
    generation_dict["mock_single_files"] = []
    generation_dict["real_single_files"] = []

    for single in singles:
        single_name = single["name"]
        single_pascal_name = stringcase.pascalcase(single_name)
        single_snake_name = stringcase.snakecase(single_name)
        single_camel_name = stringcase.camelcase(single_name)

        entity_name = single["entity"]
        entity_name_snake = stringcase.snakecase(entity_name)
        entity_name_pascal = stringcase.pascalcase(entity_name)
        entity_name_camel = stringcase.camelcase(entity_name)

        if single_snake_name == "auto":
            single_snake_name = f"single_{entity_name_snake}"
            single_pascal_name = f"Single{entity_name_pascal}"
            single_camel_name = f"single{entity_name_camel}"

        mock_single_file = os.path.join(
            mock_imports_folder_path,
            "Singles",
            f"{single_pascal_name}.qml",
        )
        generation_dict["mock_single_files"].append(mock_single_file)
        real_single_file = os.path.join(
            real_imports_folder_path,
            "singles",
            f"foreign_{single_snake_name}.h",
        )
        generation_dict["real_single_files"].append(real_single_file)

        generation_dict["singles"].append(
            {
                "mock_single_file": mock_single_file,
                "mock_template_path": "QML/mock_imports/singles/",
                "mock_template": "single.qml.jinja2",
                "real_single_file": real_single_file,
                "real_template_path": "QML/real_imports/singles/",
                "real_template": "foreign_single.h.jinja2",
                "single_pascal_name": single_pascal_name,
                "single_snake_name": single_snake_name,
                "single_camel_name": single_camel_name,
                "fields": tools.get_fields_without_foreign_entities(
                    entities_by_name[single["entity"]]["fields"],
                    entities_by_name,
                    single["entity"],
                ),
            }
        )

    # add qmldir:
    single_qmldir_file = os.path.join(mock_imports_folder_path, "Singles", "qmldir")

    generation_dict["single_qmldir_file"] = single_qmldir_file

    # add CMakelists.txt:
    single_cmakelists_file = os.path.join(
        real_imports_folder_path, "singles", "CMakeLists.txt"
    )
    generation_dict["single_cmakelists_file"] = single_cmakelists_file


    # add application_name
    generation_dict["application_name"] = application_name
    generation_dict["application_cpp_domain_name"] = application_cpp_domain_name

    return generation_dict


def _generate_mock_interactor_file(
    root_path: str,
    interactor: dict,
    generation_dict: dict,
    files_to_be_generated: dict[str, bool],
):
    # generate the mock interactor file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(interactor["mock_interactor_file"], False):
        return

    # Create the jinja2 environment
    template_path = os.path.join("templates", interactor["mock_template_path"])
    env = Environment(loader=FileSystemLoader(template_path))
    # Load the template
    template = env.get_template(interactor["mock_template"])

    # Render the template
    output = template.render(
        interactor=interactor,
    )

    output_file = os.path.join(root_path, interactor["mock_interactor_file"])

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Write the output to the file
    with open(output_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {output_file}")


def _generate_mock_signals_file(
    root_path: str,
    interactor: dict,
    generation_dict: dict,
    files_to_be_generated: dict[str, bool],
):
    # generate the mock signals file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(interactor["mock_signals_file"], False):
        return

    # Create the jinja2 environment
    template_path = os.path.join("templates", interactor["mock_template_path"])
    env = Environment(loader=FileSystemLoader(template_path))
    # Load the template
    template = env.get_template(interactor["mock_signals_template"])

    # Render the template
    output = template.render(
        interactor=interactor,
    )

    output_file = os.path.join(root_path, interactor["mock_signals_file"])

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Write the output to the file
    with open(output_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {output_file}")


def _generate_mock_undo_redo_interactor_file(
    root_path: str,
    generation_dict: dict,
    files_to_be_generated: dict[str, bool],
):
    # generate the mock undo redo interactor file if in the files_to_be_generated dict the value is True
    undo_redo_interactor_file = generation_dict["mock_undo_redo_interactor_file"]

    if not files_to_be_generated.get(undo_redo_interactor_file, False):
        return

    undo_redo_interactor_file = os.path.join(
        root_path,
        undo_redo_interactor_file,
    )

    # Create the jinja2 environment
    template_path = os.path.join("templates", "QML", "mock_imports", "interactors")
    env = Environment(loader=FileSystemLoader(template_path))
    # Load the template
    template = env.get_template("undo_redo_interactor.qml.jinja2")

    # Render the template
    output = template.render()

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(undo_redo_interactor_file), exist_ok=True)

    # Write the output to the file
    with open(undo_redo_interactor_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {undo_redo_interactor_file}")


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
    template_path = os.path.join("templates", "QML", "mock_imports", "interactors")
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
    template_path = os.path.join("templates", "QML", "mock_imports", "interactors")
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
    template_path = os.path.join("templates", "QML", "mock_imports", "interactors")
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
    template_path = os.path.join("templates", "QML", "mock_imports", "interactors")
    env = Environment(loader=FileSystemLoader(template_path))
    # Load the template
    template = env.get_template("event_dispatcher.qml.jinja2")

    # Render the template
    output = template.render(
        interactors=generation_dict["interactors"],
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
    template_path = os.path.join("templates", "QML", "mock_imports", "interactors")
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


def _generate_mock_interactors_qmldir_file(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool]
):
    # generate the mock qmldir file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(generation_dict["interactor_qmldir_file"], False):
        return

    # Create the jinja2 environment
    env = Environment(
        loader=FileSystemLoader("templates/QML/mock_imports/interactors/")
    )
    # Load the template
    template = env.get_template("qmldir_template.jinja2")

    singleton_list = []
    for _, interactor in generation_dict["interactors"].items():
        name = interactor["feature_pascal_name"]
        singleton_list.append(f"singleton {name}Interactor 1.0 {name}Interactor.qml")
        singleton_list.append(f"singleton {name}Signals 1.0 {name}Signals.qml")

    if generation_dict["has_undo_redo"]:
        singleton_list.append(
            f"singleton UndoRedoInteractor 1.0 UndoRedoInteractor.qml"
        )
        singleton_list.append(f"singleton UndoRedoSignals 1.0 UndoRedoSignals.qml")
    singleton_list.append(f"singleton ProgressSignals 1.0 ProgressSignals.qml")
    singleton_list.append(f"singleton ErrorSignals 1.0 ErrorSignals.qml")
    singleton_list.append(f"singleton EventDispatcher 1.0 EventDispatcher.qml")
    singleton_list.append(f"QCoroQmlTask 1.0 QCoroQmlTask.qml")

    # Render the template
    output = template.render(singleton_list=singleton_list)

    output_file = os.path.join(root_path, generation_dict["interactor_qmldir_file"])

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Write the output to the file
    with open(output_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {output_file}")


def _generate_real_interactor_file(
    root_path: str,
    interactor: dict,
    generation_dict: dict,
    files_to_be_generated: dict[str, bool],
    uncrustify_config_file: str,
):
    # generate the real interactor file if in the files_to_be_generated dict the value is True
    real_interactor_file = interactor["real_interactor_file"]

    if not files_to_be_generated.get(real_interactor_file, False):
        return

    # Create the jinja2 environment
    template_path = os.path.join("templates", interactor["real_template_path"])
    env = Environment(loader=FileSystemLoader(template_path))
    # Load the template
    template = env.get_template(interactor["real_template"])

    # Render the template
    output = template.render(
        interactor=interactor,
        application_cpp_domain_name=generation_dict["application_cpp_domain_name"],
    )

    output_file = os.path.join(root_path, real_interactor_file)

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Write the output to the file
    with open(output_file, "w") as fh:
        fh.write(output)

    # if uncrustify_config_file:
    #     uncrustify.run_uncrustify(real_interactor_file, uncrustify_config_file)
    clang_format_runner.run_clang_format(output_file)

    print(f"Successfully wrote file {output_file}")


def _generate_real_undo_redo_interactor_file(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool]
):
    # generate the real undo redo interactor file if in the files_to_be_generated dict the value is True
    undo_redo_interactor_file = generation_dict["real_undo_redo_interactor_file"]

    if not files_to_be_generated.get(undo_redo_interactor_file, False):
        return

    undo_redo_interactor_file = os.path.join(
        root_path,
        undo_redo_interactor_file,
    )

    # Create the jinja2 environment
    template_path = os.path.join("templates", "QML", "real_imports", "interactors")
    env = Environment(loader=FileSystemLoader(template_path))
    # Load the template
    template = env.get_template("foreign_undo_redo_interactor.h.jinja2")

    # Render the template
    output = template.render(
        application_cpp_domain_name=generation_dict["application_cpp_domain_name"],
    )

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(undo_redo_interactor_file), exist_ok=True)

    # Write the output to the file
    with open(undo_redo_interactor_file, "w") as fh:
        fh.write(output)

    # if uncrustify_config_file:
    #     uncrustify.run_uncrustify(undo_redo_interactor_file, uncrustify_config_file)
    clang_format_runner.run_clang_format(undo_redo_interactor_file)

    print(f"Successfully wrote file {undo_redo_interactor_file}")


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
    template_path = os.path.join("templates", "QML", "real_imports", "interactors")
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
    clang_format_runner.run_clang_format(event_dispatcher_file)

    print(f"Successfully wrote file {event_dispatcher_file}")

def _generate_real_common_cmakelists_file(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool]
):
    common_real_cmakelists_file = generation_dict["common_real_cmakelists_file"]

    # generate the real cmakelists file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(common_real_cmakelists_file, False):
        return

    output_file = os.path.join(root_path, common_real_cmakelists_file)

    # Create the jinja2 environment
    env = Environment(
        loader=FileSystemLoader("templates/QML/real_imports/")
    )
    # Load the template
    template = env.get_template("cmakelists.txt.jinja2")

    # Render the template
    output = template.render()

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Write the output to the file
    with open(output_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {output_file}")


def _generate_real_qml_modules_file(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool]
):
    real_qml_modules_file = generation_dict["real_qml_modules_file"]

    # generate the real cmakelists file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(real_qml_modules_file, False):
        return

    output_file = os.path.join(root_path, real_qml_modules_file)

    # Create the jinja2 environment
    env = Environment(
        loader=FileSystemLoader("templates/QML/")
    )
    # Load the template
    template = env.get_template("realqmlmodules.cmake.jinja2")

    # Render the template
    output = template.render(
        application_name=generation_dict["application_name"],
    )

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Write the output to the file
    with open(output_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {output_file}")


def _generate_real_interactors_cmakelists_file(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool]
):
    interactor_cmakelists_file = generation_dict["interactor_cmakelists_file"]

    # generate the real cmakelists file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(interactor_cmakelists_file, False):
        return

    output_file = os.path.join(root_path, interactor_cmakelists_file)

    # Create the jinja2 environment
    env = Environment(
        loader=FileSystemLoader("templates/QML/real_imports/interactors/")
    )
    # Load the template
    template = env.get_template("cmakelists.txt.jinja2")

    files = generation_dict["real_interactor_files"]
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
    clang_format_runner.run_clang_format(output_file)

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
    interactor_cmakelists_file = generation_dict["model_cmakelists_file"]

    # generate the real cmakelists file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(interactor_cmakelists_file, False):
        return

    output_file = os.path.join(root_path, interactor_cmakelists_file)

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

def _generate_mock_single_file(
    root_path: str,
    single: dict,
    generation_dict: dict,
    files_to_be_generated: dict[str, bool],
):
    # generate the mock single file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(single["mock_single_file"], False):
        return

    # Create the jinja2 environment
    template_path = os.path.join("templates", single["mock_template_path"])
    env = Environment(loader=FileSystemLoader(template_path))
    # Load the template
    template = env.get_template(single["mock_template"])

    # Render the template
    output = template.render(
        single=single,
    )

    output_file = os.path.join(root_path, single["mock_single_file"])

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Write the output to the file
    with open(output_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {output_file}")


def _generate_real_single_file(
    root_path: str,
    single: dict,
    generation_dict: dict,
    files_to_be_generated: dict[str, bool],
):
    # generate the real single file if in the files_to_be_generated dict the value is True
    real_single_file = single["real_single_file"]

    if not files_to_be_generated.get(real_single_file, False):
        return

    # Create the jinja2 environment
    template_path = os.path.join("templates", single["real_template_path"])
    env = Environment(loader=FileSystemLoader(template_path))
    # Load the template
    template = env.get_template(single["real_template"])

    # Render the template
    output = template.render(
        single=single,
        application_cpp_domain_name=generation_dict["application_cpp_domain_name"],
    )

    output_file = os.path.join(root_path, real_single_file)

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Write the output to the file
    with open(output_file, "w") as fh:
        fh.write(output)

    # if uncrustify_config_file:
    #     uncrustify.run_uncrustify(real_single_file, uncrustify_config_file)
    clang_format_runner.run_clang_format(output_file)

    print(f"Successfully wrote file {output_file}")
        

def _generate_mock_singles_qmldir_file(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool]
):
    # generate the mock qmldir file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(generation_dict["single_qmldir_file"], False):
        return

    # Create the jinja2 environment
    env = Environment(loader=FileSystemLoader("templates/QML/mock_imports/singles/"))
    # Load the template
    template = env.get_template("qmldir_template.jinja2")

    singles_to_declare_list = []
    for single in generation_dict["singles"]:
        name = single["single_pascal_name"]
        singles_to_declare_list.append(f"{name} 1.0 {name}.qml")

    # Render the template
    output = template.render(singles_to_declare_list=singles_to_declare_list)

    output_file = os.path.join(root_path, generation_dict["single_qmldir_file"])

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Write the output to the file
    with open(output_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {output_file}")


def _generate_real_singles_cmakelists_file(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool]
):
    interactor_cmakelists_file = generation_dict["single_cmakelists_file"]

    # generate the real cmakelists file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(interactor_cmakelists_file, False):
        return

    output_file = os.path.join(root_path, interactor_cmakelists_file)

    # Create the jinja2 environment
    env = Environment(loader=FileSystemLoader("templates/QML/real_imports/singles/"))
    # Load the template
    template = env.get_template("cmakelists.txt.jinja2")

    files = generation_dict["real_single_files"]
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

def generate_qml_imports_files(
    root_path: str,
    relative_folder_path: str,
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

    interactor_data = manifest_data.get("interactor", {})
    has_undo_redo = interactor_data.get("create_undo_redo_interactor", False)

    presenter_data = manifest_data.get("presenter", {})
    create_undo_and_redo_singles = presenter_data.get(
        "create_undo_and_redo_singles", False
    )
    list_models = presenter_data.get("list_models", [])
    singles = presenter_data.get("singles", [])


    generation_dict = _get_generation_dict(
        relative_folder_path,
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
    for _, interactor in generation_dict["interactors"].items():
        _generate_mock_interactor_file(
            root_path, interactor, generation_dict, files_to_be_generated
        )
        _generate_mock_signals_file(
            root_path, interactor, generation_dict, files_to_be_generated
        )

    _generate_mock_event_dispatcher_file(
        root_path, generation_dict, files_to_be_generated
    )
    if has_undo_redo:
        _generate_mock_undo_redo_interactor_file(
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
    _generate_mock_interactors_qmldir_file(
        root_path, generation_dict, files_to_be_generated
    )

    # generate real files
    for _, interactor in generation_dict["interactors"].items():
        _generate_real_interactor_file(
            root_path,
            interactor,
            generation_dict,
            files_to_be_generated,
            uncrustify_config_file,
        )

    _generate_real_event_dispatcher_file(
        root_path, generation_dict, files_to_be_generated
    )

    if has_undo_redo:
        _generate_real_undo_redo_interactor_file(
            root_path, generation_dict, files_to_be_generated
        )

    # generate real common CMakeLists.txt file
    _generate_real_common_cmakelists_file(
        root_path, generation_dict, files_to_be_generated
    )

    # generate real qmlmodules.cmake file
    _generate_real_qml_modules_file(
        root_path, generation_dict, files_to_be_generated
    )

    # generate real CMakeLists.txt file
    _generate_real_interactors_cmakelists_file(
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

    # singles
    for single in generation_dict["singles"]:
        _generate_mock_single_file(
            root_path, single, generation_dict, files_to_be_generated
        )
        _generate_real_single_file(
            root_path, single, generation_dict, files_to_be_generated
        )

    # generate mock qmldir file
    _generate_mock_singles_qmldir_file(root_path, generation_dict, files_to_be_generated)
    _generate_real_singles_cmakelists_file(
        root_path, generation_dict, files_to_be_generated
    )


def get_files_to_be_generated(
    manifest_file: str, files_to_be_generated: dict[str, bool] = {}, folder_path: str = ""
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

    interactor_data = manifest_data.get("interactor", {})
    has_undo_redo = interactor_data.get("create_undo_redo_interactor", False)

    feature_list = application_data.get("features", [])

    presenter_data = manifest_data.get("presenter", {})
    create_undo_and_redo_singles = presenter_data.get(
        "create_undo_and_redo_singles", False
    )

    list_models = presenter_data.get("list_models", [])
    singles = presenter_data.get("singles", [])

    # Organize feature_list by name for easier lookup
    feature_by_name = {feature["name"]: feature for feature in feature_list}

    files = []
    generation_dict = _get_generation_dict(
        folder_path,
        feature_by_name,
        entities_by_name,
        has_undo_redo,
        create_undo_and_redo_singles,
        list_models,
        singles,
        application_name,
        application_cpp_domain_name,
    )

    files += generation_dict["real_interactor_files"]
    files += generation_dict["mock_interactor_files"]

    # for _, interactor in generation_dict["interactors"].items():
    #     files += feature["real_interactor_files"]

    # # add CMakelists.txt:
    interactor_cmakelists_file = generation_dict["interactor_cmakelists_file"]
    files.append(interactor_cmakelists_file)

    # # add qmldir:
    interactor_qmldir_file = generation_dict["interactor_qmldir_file"]
    files.append(interactor_qmldir_file)

    # list models 
    files += generation_dict["real_model_files"]
    files += generation_dict["mock_model_files"]

    # # add model CMakelists.txt:
    model_cmakelists_file = generation_dict["model_cmakelists_file"]
    files.append(model_cmakelists_file)

    # # add model qmldir:
    model_qmldir_file = generation_dict["model_qmldir_file"]
    files.append(model_qmldir_file)

    # list singles
    files += generation_dict["real_single_files"]
    files += generation_dict["mock_single_files"]

    # # add single CMakelists.txt:
    single_cmakelists_file = generation_dict["single_cmakelists_file"]
    files.append(single_cmakelists_file)

    # # add single qmldir:
    single_qmldir_file = generation_dict["single_qmldir_file"]
    files.append(single_qmldir_file)

    # add common real CMakeLists.txt file
    common_real_cmakelists_file = generation_dict["common_real_cmakelists_file"]
    files.append(common_real_cmakelists_file)

    # add realqmlmodules.cmake file
    real_qml_modules_file = generation_dict["real_qml_modules_file"]
    files.append(real_qml_modules_file)

    # strip from files if the value in files_to_be_generated is False
    if files_to_be_generated:
        for path, generate in files_to_be_generated.items():
            if not generate and path in files:
                files.remove(path)

    return files

def is_qml_imports_integration_enabled(manifest_file: str) -> bool:
    """
    Check if the QML imports integration is enabled
    """
    # Read the manifest file
    with open(manifest_file, "r") as stream:
        try:
            manifest_data = yaml.safe_load(stream)
        except yaml.YAMLError as exc:
            print(exc)
            return

    qml_imports_integration_dict = manifest_data.get("front_ends", {}).get(
        "qml_imports_integration", {})

    if not qml_imports_integration_dict:
        return False

    return True

def get_qml_imports_integration_folder_path(manifest_file: str) -> str:
    """
    Get the folder path where the QML imports files will be integrated
    """
    # Read the manifest file
    with open(manifest_file, "r") as stream:
        try:
            manifest_data = yaml.safe_load(stream)
        except yaml.YAMLError as exc:
            print(exc)
            return
        
    integration_folder_path = manifest_data.get("front_ends", {}).get(
        "qml_imports_integration", {}).get("folder_path", "qml_imports")

    return integration_folder_path


# generate the files into the preview folder
def preview_qml_imports_files(
    root_path: str,
    relative_folder_path: str,
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

    # write the modified manifest file
    with open(manifest_preview_file, "w") as fh:
        yaml.dump(manifest, fh)

    root_path = os.path.join(root_path, "qleany_preview")

    # remove .. from the path
    if files_to_be_generated:
        preview_files_to_be_generated = {}
        for path, value in files_to_be_generated.items():
            preview_files_to_be_generated[path.replace("..", "")] = value

        generate_qml_imports_files(
            root_path,
            relative_folder_path,
            manifest_preview_file,
            preview_files_to_be_generated,
            uncrustify_config_file,
        )

    else:
        generate_qml_imports_files(root_path, relative_folder_path, manifest_preview_file, {}, uncrustify_config_file)


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
                preview_qml_imports_files(root_path, "", manifest_file)
            else:
                generate_qml_imports_files(root_path, "", manifest_file)
        else:
            print("Error: Manifest file must be named 'qleany.yaml' or 'qleany.yml'")
    else:
        print("Error: Please provide the manifest file as an argument")
