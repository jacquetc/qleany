from jinja2 import Environment, FileSystemLoader
import yaml
import os
import sys
import stringcase
import shutil
import uncrustify
from pathlib import Path
import clang_format_runner as clang_format_runner
import generation_dict_tools as tools


def get_other_entities_relation_fields(
    entity_name: str, entities_by_name: dict
) -> list:
    other_entities_relation_fields = []

    entity = entities_by_name.get(entity_name, None)
    if entity is None:
        return []

    for entity, data in entities_by_name.items():
        if entity == entity_name:
            continue

        fields = data["fields"]
        for field in fields:
            if field.get("hidden", False):
                continue
            field_type = field["type"]
            if tools.is_unique_foreign_entity(
                field_type, entities_by_name
            ) or tools.is_list_foreign_entity(field_type, entities_by_name):
                if entity_name != tools.get_entity_from_foreign_field_type(
                    field_type, entities_by_name
                ):
                    continue
                other_entities_relation_fields.append(
                    {
                        "name_snake": stringcase.snakecase(entity),
                        "name_pascal": stringcase.pascalcase(entity),
                        "name_spinal": stringcase.spinalcase(entity),
                        "name_camel": stringcase.camelcase(entity),
                        "field_name_snake": stringcase.snakecase(field["name"]),
                        "field_name_pascal": stringcase.pascalcase(field["name"]),
                        "field_name_spinal": stringcase.spinalcase(field["name"]),
                        "field_name_camel": stringcase.camelcase(field["name"]),
                    }
                )

    return other_entities_relation_fields




def get_generation_dict(
    folder_path: str,
    application_name: str,
    application_cpp_domain_name: str,
    feature_by_name: dict,
    entities_by_name: dict,
    controller_by_name: dict,
    export: str,
    export_header_file: str,
    create_undo_redo_controller: bool,
) -> dict:
    generation_dict = {}

    # add export_header
    generation_dict["export_header"] = export_header_file
    generation_dict["export"] = export
    generation_dict["folder_path"] = folder_path
    generation_dict["all_controller_files"] = []

    # add application name
    generation_dict["application_cpp_domain_name"] = application_cpp_domain_name
    generation_dict["application_snakecase_name"] = stringcase.snakecase(
        application_name
    )
    generation_dict["application_pascalcase_name"] = stringcase.pascalcase(
        application_name
    )
    generation_dict["application_spinalcase_name"] = stringcase.spinalcase(
        application_name
    )
    generation_dict["application_camelcase_name"] = stringcase.camelcase(
        application_name
    )
    generation_dict["application_uppercase_name"] = application_name.upper()

    generation_dict["features"] = []

    for feature_name, feature in feature_by_name.items():
        feature_snake_name = stringcase.snakecase(feature_name)
        feature_pascal_name = stringcase.pascalcase(feature_name)
        feature_spinal_name = stringcase.spinalcase(feature_name)
        feature_camel_name = stringcase.camelcase(feature_name)
        final_feature_dict = {
            "feature_name_snake": feature_snake_name,
            "feature_name_pascal": feature_pascal_name,
            "feature_name_spinal": feature_spinal_name,
            "feature_name_camel": feature_camel_name,
        }
        final_feature_dict["crud"] = {
            "enabled": feature.get("CRUD", {}).get("enabled", False)
        }

        if feature.get("CRUD", {}).get("enabled", False):
            entity_name = feature["CRUD"].get("entity_mappable_with", "Undefined")
            entity = entities_by_name[entity_name]
            entity_snake_name = stringcase.snakecase(entity_name)
            entity_pascal_name = stringcase.pascalcase(entity_name)
            entity_spinal_name = stringcase.spinalcase(entity_name)
            entity_camel_name = stringcase.camelcase(entity_name)
            final_feature_dict["crud"]["entity_name_snake"] = entity_snake_name
            final_feature_dict["crud"]["entity_name_pascal"] = entity_pascal_name
            final_feature_dict["crud"]["entity_name_spinal"] = entity_spinal_name
            final_feature_dict["crud"]["entity_name_camel"] = entity_camel_name
            final_feature_dict["crud"][
                "entity_has_relation_fields"
            ] = tools.does_entity_have_relation_fields(entity_name, entities_by_name, False)

            final_feature_dict["crud"]["get"] = (
                feature["CRUD"].get("get", {}).get("enabled", False)
            )
            final_feature_dict["crud"]["get_with_details"] = (
                feature["CRUD"].get("get_with_details", {}).get("enabled", False)
            )
            final_feature_dict["crud"]["get_all"] = (
                feature["CRUD"].get("get_all", {}).get("enabled", False)
            )
            final_feature_dict["crud"]["create"] = (
                feature["CRUD"].get("create", {}).get("enabled", False)
            )
            final_feature_dict["crud"]["update_"] = (
                feature["CRUD"].get("update", {}).get("enabled", False)
            )
            final_feature_dict["crud"]["remove"] = (
                feature["CRUD"].get("remove", {}).get("enabled", False)
            )
            final_feature_dict["crud"]["insert_relation"] = (
                feature["CRUD"].get("insert_relation", {}).get("enabled", False)
            )

            # has owner ?
            owner_dict = tools.determine_owner(entity_name, entities_by_name)

            final_feature_dict["crud"]["has_owner"] = owner_dict != {}
            owner_name = owner_dict.get("name", "Undefined")
            owner_field_name = owner_dict.get("field", "Undefined")
            final_feature_dict["crud"]["owner_is_list"] = owner_dict.get(
                "is_list", False
            )
            final_feature_dict["crud"]["owner_is_ordered"] = owner_dict.get(
                "ordered", False
            )
            final_feature_dict["crud"]["owner_name_camel"] = stringcase.camelcase(
                owner_name
            )
            final_feature_dict["crud"]["owner_name_snake"] = stringcase.snakecase(
                owner_name
            )
            final_feature_dict["crud"]["owner_name_pascal"] = stringcase.pascalcase(
                owner_name
            )
            final_feature_dict["crud"]["owner_field_name_camel"] = stringcase.camelcase(
                owner_field_name
            )
            final_feature_dict["crud"]["owner_field_name_snake"] = stringcase.snakecase(
                owner_field_name
            )
            final_feature_dict["crud"][
                "owner_field_name_pascal"
            ] = stringcase.pascalcase(owner_field_name)

            # other entities relation fields
            final_feature_dict["crud"][
                "other_entities_relation_fields"
            ] = get_other_entities_relation_fields(entity_name, entities_by_name)

        # files :
        generation_dict["all_controller_files"].append(
            os.path.join(
                folder_path,
                feature_snake_name,
                f"{feature_snake_name}_controller.h",
            )
        )
        generation_dict["all_controller_files"].append(
            os.path.join(
                folder_path,
                feature_snake_name,
                f"{feature_snake_name}_controller.cpp",
            )
        )
        generation_dict["all_controller_files"].append(
            os.path.join(
                folder_path,
                feature_snake_name,
                f"{feature_snake_name}_signals.h",
            )
        )

        # add custom commands
        if feature.get("commands", []):
            final_feature_dict["custom_commands"] = []
            for command in feature["commands"]:
                repositories = []
                for entity in command.get("entities", []):
                    repositories.append(
                        {
                            "name_snake": stringcase.snakecase(entity),
                            "name_pascal": stringcase.pascalcase(entity),
                            "name_spinal": stringcase.spinalcase(entity),
                            "name_camel": stringcase.camelcase(entity),
                        }
                    )
                dto_out_enabled = (
                    command.get("dto", {}).get("out", {}).get("enabled", True)
                )
                dto_out = (
                    command.get("dto", {})
                    .get("out", {})
                    .get("type_prefix", "Undefined")
                    if dto_out_enabled
                    else "Undefined"
                )
                dto_in_enabled = (
                    command.get("dto", {}).get("in", {}).get("enabled", True)
                )
                dto_in = (
                    command.get("dto", {}).get("in", {}).get("type_prefix", "Undefined")
                    if dto_in_enabled
                    else "Undefined"
                )

                final_feature_dict["custom_commands"].append(
                    {
                        "name": command["name"],
                        "name_snake": stringcase.snakecase(command["name"]),
                        "name_camel": stringcase.camelcase(command["name"]),
                        "repositories": repositories,
                        "dto_out_enabled": dto_out_enabled,
                        "dto_out": dto_out,
                        "dto_out_snake": stringcase.snakecase(dto_out),
                        "dto_in_enabled": dto_in_enabled,
                        "dto_in": dto_in,
                        "dto_in_snake": stringcase.snakecase(dto_in),
                    }
                )

        # add custom queries
        if feature.get("queries", []):
            final_feature_dict["custom_queries"] = []
            for query in feature["queries"]:
                repositories = []
                for entity in query.get("entities", []):
                    repositories.append(
                        {
                            "name_snake": stringcase.snakecase(entity),
                            "name_pascal": stringcase.pascalcase(entity),
                            "name_spinal": stringcase.spinalcase(entity),
                            "name_camel": stringcase.camelcase(entity),
                        }
                    )

                dto_out_enabled = (
                    query.get("dto", {}).get("out", {}).get("enabled", True)
                )
                dto_out = (
                    query.get("dto", {}).get("out", {}).get("type_prefix", "Undefined")
                    if dto_out_enabled
                    else "Undefined"
                )
                dto_in_enabled = query.get("dto", {}).get("in", {}).get("enabled", True)
                dto_in = (
                    query.get("dto", {}).get("in", {}).get("type_prefix", "Undefined")
                    if dto_in_enabled
                    else "Undefined"
                )

                final_feature_dict["custom_queries"].append(
                    {
                        "name": query["name"],
                        "name_snake": stringcase.snakecase(query["name"]),
                        "name_camel": stringcase.camelcase(query["name"]),
                        "repositories": repositories,
                        "dto_out_enabled": dto_out_enabled,
                        "dto_out": dto_out,
                        "dto_out_snake": stringcase.snakecase(dto_out),
                        "dto_in_enabled": dto_in_enabled,
                        "dto_in": dto_in,
                        "dto_in_snake": stringcase.snakecase(dto_in),
                    }
                )

        generation_dict["features"].append(final_feature_dict)

    # add undo redo controller
    generation_dict["create_undo_redo_controller"] = create_undo_redo_controller
    if create_undo_redo_controller:
        h_file = os.path.join(
            folder_path,
            "undo_redo",
            f"undo_redo_controller.h",
        )

        cpp_file = os.path.join(
            folder_path,
            "undo_redo",
            f"undo_redo_controller.cpp",
        )

        signals_file = os.path.join(
            folder_path,
            "undo_redo",
            f"undo_redo_signals.h",
        )

        generation_dict["all_controller_files"].append(h_file)

        generation_dict["all_controller_files"].append(cpp_file)
        generation_dict["all_controller_files"].append(signals_file)
        generation_dict["undo_redo_controller_files"] = [
            h_file,
            cpp_file,
            signals_file,
        ]

    return generation_dict


def generate_cmakelists(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool] = None
):
    template_env = Environment(loader=FileSystemLoader("templates/controller"))
    template = template_env.get_template("cmakelists_template.jinja2")

    folder_path = generation_dict["folder_path"]

    relative_cmakelists_file = os.path.join(folder_path, "CMakeLists.txt")
    cmakelists_file = os.path.join(root_path, relative_cmakelists_file)

    if files_to_be_generated.get(relative_cmakelists_file, False):
        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(cmakelists_file), exist_ok=True)

        with open(cmakelists_file, "w") as f:
            f.write(
                template.render(
                    export_header_file=generation_dict["export_header"],
                    application_spinalcase_name=generation_dict[
                        "application_spinalcase_name"
                    ],
                    application_uppercase_name=generation_dict[
                        "application_uppercase_name"
                    ],
                    features=generation_dict["features"],
                )
            )
            print(f"Successfully wrote file {cmakelists_file}")


def generate_cmake_file(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool] = None
):
    template_env = Environment(loader=FileSystemLoader("templates/controller"))
    template = template_env.get_template("controllers.cmake.jinja2")

    folder_path = generation_dict["folder_path"]
    all_controller_files = generation_dict["all_controller_files"]

    relative_cmake_file = os.path.join(folder_path, "controllers.cmake")
    cmake_file = os.path.join(root_path, relative_cmake_file)

    # write the controller cmake list file

    if files_to_be_generated.get(relative_cmake_file, False):
        controller_files = []
        for controller_file in all_controller_files:
            relative_path = os.path.relpath(
                os.path.join(root_path, controller_file), os.path.dirname(cmake_file)
            )
            controller_files.append(relative_path.replace("\\", "/"))

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(cmake_file), exist_ok=True)

        rendered_template = template.render(
            controller_files=controller_files,
        )

        with open(cmake_file, "w") as fh:
            fh.write(rendered_template)
            print(f"Successfully wrote file {cmake_file}")


def generate_event_dispatcher_files(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool] = None
):
    template_env = Environment(loader=FileSystemLoader("templates/controller"))
    #  event dispatcher h
    template = template_env.get_template("event_dispatcher.h.jinja2")

    folder_path = generation_dict["folder_path"]
    all_controller_files = generation_dict["all_controller_files"]

    relative_event_dispatcher_file = os.path.join(folder_path, "event_dispatcher.h")
    event_dispatcher_file = os.path.join(root_path, relative_event_dispatcher_file)

    # write the event dispatcher header file

    if files_to_be_generated.get(relative_event_dispatcher_file, False):
        controller_files = []
        for controller_file in all_controller_files:
            relative_path = os.path.relpath(
                controller_file, os.path.dirname(event_dispatcher_file)
            )
            controller_files.append(relative_path.replace("\\", "/"))

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(event_dispatcher_file), exist_ok=True)

        rendered_template = template.render(
            export_header_file=generation_dict["export_header"],
            export=generation_dict["export"],
            features=generation_dict["features"],
            application_cpp_domain_name=generation_dict["application_cpp_domain_name"],
            undo_redo_signals=generation_dict["create_undo_redo_controller"],
        )

        with open(event_dispatcher_file, "w") as fh:
            fh.write(rendered_template)
            print(f"Successfully wrote file {event_dispatcher_file}")

    #  event dispatcher cpp
    template = template_env.get_template("event_dispatcher.cpp.jinja2")
    relative_event_dispatcher_file = os.path.join(folder_path, "event_dispatcher.cpp")
    event_dispatcher_file = os.path.join(root_path, relative_event_dispatcher_file)

    # write the event dispatcher cpp file

    if files_to_be_generated.get(relative_event_dispatcher_file, False):
        controller_files = []
        for controller_file in all_controller_files:
            relative_path = os.path.relpath(
                controller_file, os.path.dirname(event_dispatcher_file)
            )
            controller_files.append(relative_path.replace("\\", "/"))

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(event_dispatcher_file), exist_ok=True)

        rendered_template = template.render(
            features=generation_dict["features"],
            application_cpp_domain_name=generation_dict["application_cpp_domain_name"],
            undo_redo_signals=generation_dict["create_undo_redo_controller"],
        )

        with open(event_dispatcher_file, "w") as fh:
            fh.write(rendered_template)
            print(f"Successfully wrote file {event_dispatcher_file}")


def _generate_controller_h_and_cpp_files(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool] = None
):
    template_env = Environment(loader=FileSystemLoader("templates/controller"))
    for feature in generation_dict["features"]:
        #  controller h
        template = template_env.get_template("controller.h.jinja2")

        folder_path = generation_dict["folder_path"]

        relative_controller_file = os.path.join(
            folder_path,
            feature["feature_name_snake"],
            f"{feature['feature_name_snake']}_controller.h",
        )
        controller_file = os.path.join(root_path, relative_controller_file)

        # write the controller header file

        if files_to_be_generated.get(relative_controller_file, False):
            # Create the directory if it does not exist
            os.makedirs(os.path.dirname(controller_file), exist_ok=True)

            rendered_template = template.render(
                export_header_file=generation_dict["export_header"],
                export=generation_dict["export"],
                feature=feature,
                application_cpp_domain_name=generation_dict[
                    "application_cpp_domain_name"
                ],
                custom_commands=feature.get("custom_commands", []),
                custom_queries=feature.get("custom_queries", []),
                entity_name_pascal=feature["crud"].get("entity_name_pascal", ""),
                entity_name_snake=feature["crud"].get("entity_name_snake", ""),
                entity_name_spinal=feature["crud"].get("entity_name_spinal", ""),
                entity_name_camel=feature["crud"].get("entity_name_camel", ""),
                feature_name_pascal=feature["feature_name_pascal"],
                feature_name_snake=feature["feature_name_snake"],
                feature_name_spinal=feature["feature_name_spinal"],
                feature_name_camel=feature["feature_name_camel"],
            )

            with open(controller_file, "w") as fh:
                fh.write(rendered_template)
                print(f"Successfully wrote file {controller_file}")

        #  controller cpp
        template = template_env.get_template("controller.cpp.jinja2")
        relative_controller_file = os.path.join(
            folder_path,
            feature["feature_name_snake"],
            f"{feature['feature_name_snake']}_controller.cpp",
        )
        controller_file = os.path.join(root_path, relative_controller_file)

        # write the controller cpp file

        if files_to_be_generated.get(relative_controller_file, False):
            # Create the directory if it does not exist
            os.makedirs(os.path.dirname(controller_file), exist_ok=True)

            rendered_template = template.render(
                feature=feature,
                application_cpp_domain_name=generation_dict[
                    "application_cpp_domain_name"
                ],
                custom_commands=feature.get("custom_commands", []),
                custom_queries=feature.get("custom_queries", []),
                entity_name_pascal=feature["crud"].get("entity_name_pascal", ""),
                entity_name_snake=feature["crud"].get("entity_name_snake", ""),
                entity_name_spinal=feature["crud"].get("entity_name_spinal", ""),
                entity_name_camel=feature["crud"].get("entity_name_camel", ""),
                feature_name_pascal=feature["feature_name_pascal"],
                feature_name_snake=feature["feature_name_snake"],
                feature_name_spinal=feature["feature_name_spinal"],
                feature_name_camel=feature["feature_name_camel"],
            )

            with open(controller_file, "w") as fh:
                fh.write(rendered_template)
                print(f"Successfully wrote file {controller_file}")


def generate_undo_redo_controller_h_and_cpp_files(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool] = None
):
    template_env = Environment(loader=FileSystemLoader("templates/controller"))
    #  controller h
    template = template_env.get_template("undo_redo_controller.h.jinja2")

    folder_path = generation_dict["folder_path"]

    relative_controller_file = os.path.join(
        folder_path,
        "undo_redo",
        f"undo_redo_controller.h",
    )
    controller_file = os.path.join(root_path, relative_controller_file)

    # write the controller header file

    if files_to_be_generated.get(relative_controller_file, False):
        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(controller_file), exist_ok=True)

        rendered_template = template.render(
            export_header_file=generation_dict["export_header"],
            export=generation_dict["export"],
            application_cpp_domain_name=generation_dict["application_cpp_domain_name"],
        )

        with open(controller_file, "w") as fh:
            fh.write(rendered_template)
            print(f"Successfully wrote file {controller_file}")

    #  controller cpp
    template = template_env.get_template("undo_redo_controller.cpp.jinja2")
    relative_controller_file = os.path.join(
        folder_path,
        "undo_redo",
        f"undo_redo_controller.cpp",
    )
    controller_file = os.path.join(root_path, relative_controller_file)

    # write the controller cpp file

    if files_to_be_generated.get(relative_controller_file, False):
        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(controller_file), exist_ok=True)

        rendered_template = template.render(
            application_cpp_domain_name=generation_dict["application_cpp_domain_name"]
        )
        with open(controller_file, "w") as fh:
            fh.write(rendered_template)
            print(f"Successfully wrote file {controller_file}")


def generate_controller_registration_files(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool] = None
):
    template_env = Environment(loader=FileSystemLoader("templates/controller"))
    #  controller_registration.h
    template = template_env.get_template("controller_registration.h.jinja2")

    folder_path = generation_dict["folder_path"]

    relative_controller_file = os.path.join(
        folder_path,
        "controller_registration.h",
    )
    controller_file = os.path.join(root_path, relative_controller_file)

    # write the controller header file

    if files_to_be_generated.get(relative_controller_file, False):
        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(controller_file), exist_ok=True)

        rendered_template = template.render(
            export_header_file=generation_dict["export_header"],
            export=generation_dict["export"],
            application_cpp_domain_name=generation_dict["application_cpp_domain_name"],
        )

        with open(controller_file, "w") as fh:
            fh.write(rendered_template)
            print(f"Successfully wrote file {controller_file}")

    #  controller_registration.cpp
    template = template_env.get_template("controller_registration.cpp.jinja2")
    relative_controller_file = os.path.join(
        folder_path,
        "controller_registration.cpp",
    )
    controller_file = os.path.join(root_path, relative_controller_file)

    # write the controller cpp file

    if files_to_be_generated.get(relative_controller_file, False):
        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(controller_file), exist_ok=True)

        rendered_template = template.render(
            features=generation_dict["features"],
            application_cpp_domain_name=generation_dict["application_cpp_domain_name"],
        )

        with open(controller_file, "w") as fh:
            fh.write(rendered_template)
            print(f"Successfully wrote file {controller_file}")


def generate_export_header_file(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool] = None
):
    template_env = Environment(loader=FileSystemLoader("templates/controller"))
    template = template_env.get_template("export_template.jinja2")

    folder_path = generation_dict["folder_path"]

    relative_export_header_file = os.path.join(
        folder_path, generation_dict["export_header"]
    )
    export_header_file = os.path.join(root_path, relative_export_header_file)

    if files_to_be_generated.get(relative_export_header_file, False):
        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(export_header_file), exist_ok=True)

        with open(export_header_file, "w") as f:
            f.write(
                template.render(
                    application_uppercase_name=generation_dict[
                        "application_uppercase_name"
                    ],
                    export=generation_dict["export"],
                )
            )
            print(f"Successfully wrote file {export_header_file}")


def generate_error_signals_file(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool] = None
):
    template_env = Environment(loader=FileSystemLoader("templates/controller"))
    template = template_env.get_template("error_signals.h.jinja2")

    folder_path = generation_dict["folder_path"]

    relative_error_signals_header_file = os.path.join(
        folder_path,
        f"error_signals.h",
    )
    error_signals_header_file = os.path.join(
        root_path, relative_error_signals_header_file
    )

    if files_to_be_generated.get(relative_error_signals_header_file, False):
        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(error_signals_header_file), exist_ok=True)

        with open(error_signals_header_file, "w") as f:
            f.write(
                template.render(
                    export_header_file=generation_dict["export_header"],
                    export=generation_dict["export"],
                    application_cpp_domain_name=generation_dict[
                        "application_cpp_domain_name"
                    ],
                )
            )
            print(f"Successfully wrote file {error_signals_header_file}")


def generate_undo_redo_signals_file(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool] = None
):
    template_env = Environment(loader=FileSystemLoader("templates/controller"))
    template = template_env.get_template("undo_redo_signals.h.jinja2")

    folder_path = generation_dict["folder_path"]

    relative_undo_redo_signals_header_file = os.path.join(
        folder_path,
        "undo_redo",
        f"undo_redo_signals.h",
    )
    undo_redo_signals_header_file = os.path.join(
        root_path, relative_undo_redo_signals_header_file
    )

    if files_to_be_generated.get(relative_undo_redo_signals_header_file, False):
        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(undo_redo_signals_header_file), exist_ok=True)

        with open(undo_redo_signals_header_file, "w") as f:
            f.write(
                template.render(
                    export_header_file=generation_dict["export_header"],
                    export=generation_dict["export"],
                    application_cpp_domain_name=generation_dict[
                        "application_cpp_domain_name"
                    ],
                )
            )
            print(f"Successfully wrote file {undo_redo_signals_header_file}")


def generate_progress_signals_file(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool] = None
):
    template_env = Environment(loader=FileSystemLoader("templates/controller"))
    template = template_env.get_template("progress_signals.h.jinja2")

    folder_path = generation_dict["folder_path"]

    relative_progress_signals_header_file = os.path.join(
        folder_path,
        f"progress_signals.h",
    )
    progress_signals_header_file = os.path.join(
        root_path, relative_progress_signals_header_file
    )

    if files_to_be_generated.get(relative_progress_signals_header_file, False):
        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(progress_signals_header_file), exist_ok=True)

        with open(progress_signals_header_file, "w") as f:
            f.write(
                template.render(
                    export_header_file=generation_dict["export_header"],
                    export=generation_dict["export"],
                    application_cpp_domain_name=generation_dict[
                        "application_cpp_domain_name"
                    ],
                )
            )
            print(f"Successfully wrote file {progress_signals_header_file}")


def generate_signal_files(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool] = None
):
    template_env = Environment(loader=FileSystemLoader("templates/controller"))
    template = template_env.get_template("signals.h.jinja2")

    folder_path = generation_dict["folder_path"]

    for feature in generation_dict["features"]:
        relative_signal_header_file = os.path.join(
            folder_path,
            feature["feature_name_snake"],
            f"{feature['feature_name_snake']}_signals.h",
        )
        signal_header_file = os.path.join(root_path, relative_signal_header_file)

        if files_to_be_generated.get(relative_signal_header_file, False):
            # Create the directory if it does not exist
            os.makedirs(os.path.dirname(signal_header_file), exist_ok=True)

            with open(signal_header_file, "w") as f:
                f.write(
                    template.render(
                        export_header_file=generation_dict["export_header"],
                        export=generation_dict["export"],
                        feature=feature,
                        application_cpp_domain_name=generation_dict[
                            "application_cpp_domain_name"
                        ],
                    )
                )
                print(f"Successfully wrote file {signal_header_file}")


def generate_controller_files(
    root_path: str,
    manifest_file: str,
    files_to_be_generated: dict[str, bool] = None,
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

    application_data = manifest_data.get("application", {})
    feature_list = application_data.get("features", [])

    # Organize feature_list by name for easier lookup
    feature_by_name = {feature["name"]: feature for feature in feature_list}

    global_data = manifest_data.get("global", {})
    application_cpp_domain_name = global_data.get(
        "application_cpp_domain_name", "Undefined"
    )

    entities_data = manifest_data.get("entities", {})
    entities_list = entities_data.get("list", [])
    # remove entities that are not to be generated
    entities_list = [entity for entity in entities_list]

    # Organize entities by name for easier lookup
    entities_by_name = {entity["name"]: entity for entity in entities_list}

    controller_data = manifest_data.get("controller", {})
    controller_list = controller_data.get("features", [])
    controller_by_name = {
        controller["name"]: controller for controller in controller_list
    }

    folder_path = controller_data.get("folder_path", "Undefined")
    export = controller_data.get("export", "Undefined")
    export_header_file = controller_data.get("export_header_file", "Undefined")
    create_undo_redo_controller = controller_data.get(
        "create_undo_redo_controller", False
    )

    generation_dict = get_generation_dict(
        folder_path,
        application_name,
        application_cpp_domain_name,
        feature_by_name,
        entities_by_name,
        controller_by_name,
        export,
        export_header_file,
        create_undo_redo_controller,
    )

    generate_event_dispatcher_files(root_path, generation_dict, files_to_be_generated)
    generate_cmake_file(root_path, generation_dict, files_to_be_generated)
    generate_cmakelists(root_path, generation_dict, files_to_be_generated)
    generate_export_header_file(root_path, generation_dict, files_to_be_generated)
    generate_signal_files(root_path, generation_dict, files_to_be_generated)
    _generate_controller_h_and_cpp_files(
        root_path, generation_dict, files_to_be_generated
    )
    if create_undo_redo_controller:
        generate_undo_redo_controller_h_and_cpp_files(
            root_path, generation_dict, files_to_be_generated
        )
        generate_undo_redo_signals_file(
            root_path, generation_dict, files_to_be_generated
        )
    generate_controller_registration_files(
        root_path, generation_dict, files_to_be_generated
    )
    generate_error_signals_file(root_path, generation_dict, files_to_be_generated)
    generate_progress_signals_file(root_path, generation_dict, files_to_be_generated)

    # format the files
    for file, to_be_generated in files_to_be_generated.items():
        # if uncrustify_config_file and files_to_be_generated.get(file, False):
        #     uncrustify.run_uncrustify(file, uncrustify_config_file)
        if to_be_generated and file.endswith(".h") or file.endswith(".cpp"):
            clang_format_runner.run_clang_format(os.path.join(root_path, file))


def get_files_to_be_generated(
    manifest_file: str,
    files_to_be_generated: dict[str, bool] = None,
    uncrustify_config_file: str = None,
) -> list[str]:
    """
    Get the list of files that need to be generated based on the manifest file
    """
    # Read the manifest file
    with open(manifest_file, "r") as fh:
        manifest_data = yaml.safe_load(fh)

    controller_data = manifest_data.get("controller", {})
    create_undo_redo_controller = controller_data.get(
        "create_undo_redo_controller", False
    )
    folder_path = controller_data["folder_path"]
    export_header_file = controller_data.get("export_header_file", "Undefined")

    # Get the list of files to be generated
    files = []
    for feature in manifest_data["application"]["features"]:
        feature_name = feature["name"]
        feature_name_snake = stringcase.snakecase(feature_name)
        files.append(
            os.path.join(
                folder_path,
                feature_name_snake,
                f"{feature_name_snake}_controller.h",
            )
        )
        files.append(
            os.path.join(
                folder_path,
                feature_name_snake,
                f"{feature_name_snake}_controller.cpp",
            )
        )
        files.append(
            os.path.join(
                folder_path,
                feature_name_snake,
                f"{feature_name_snake}_signals.h",
            )
        )

    # add undo redo controller
    if create_undo_redo_controller:
        files.append(
            os.path.join(
                folder_path,
                "undo_redo",
                f"undo_redo_controller.h",
            )
        )

        files.append(
            os.path.join(
                folder_path,
                "undo_redo",
                f"undo_redo_controller.cpp",
            )
        )

        files.append(
            os.path.join(
                folder_path,
                "undo_redo",
                f"undo_redo_signals.h",
            )
        )

    # add list_file:
    files.append(
        os.path.join(
            folder_path,
            "controllers.cmake",
        )
    )
    files.append(
        os.path.join(
            folder_path,
            "CMakeLists.txt",
        )
    )

    files.append(
        os.path.join(
            folder_path,
            "event_dispatcher.h",
        )
    )
    files.append(
        os.path.join(
            folder_path,
            "event_dispatcher.cpp",
        )
    )
    files.append(
        os.path.join(
            folder_path,
            "error_signals.h",
        )
    )
    files.append(
        os.path.join(
            folder_path,
            "progress_signals.h",
        )
    )
    files.append(
        os.path.join(
            folder_path,
            export_header_file,
        )
    )
    files.append(
        os.path.join(
            folder_path,
            "controller_registration.h",
        )
    )

    files.append(
        os.path.join(
            folder_path,
            "controller_registration.cpp",
        )
    )

    # strip from files if the value in files_to_be_generated is False
    if files_to_be_generated:
        for path, generate in files_to_be_generated.items():
            if not generate and path in files:
                files.remove(path)

    return files


# generate the files into the preview folder
def preview_controller_files(
    root_path: str,
    manifest_file: str,
    files_to_be_generated: dict[str, bool] = None,
    uncrustify_config_file: str = None,
):
    manifest_preview_file = "temp/manifest_preview.yaml"

    # make a copy of the manifest file into temp/manifest_preview.yaml
    shutil.copy(manifest_file, manifest_preview_file)

    # modify the manifest file to generate the files into the preview folder
    with open(manifest_preview_file, "r") as fh:
        manifest = yaml.safe_load(fh)

    # remove .. from the path and add preview before the folder name
    manifest["controller"]["folder_path"] = manifest["controller"][
        "folder_path"
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

        generate_controller_files(
            root_path,
            manifest_preview_file,
            preview_files_to_be_generated,
            uncrustify_config_file,
        )

    else:
        generate_controller_files(
            root_path, manifest_preview_file, {}, uncrustify_config_file
        )


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
                preview_controller_files(root_path, manifest_file)
            else:
                generate_controller_files(root_path, manifest_file)
        else:
            print("Error: Manifest file must be named 'qleany.yaml' or 'qleany.yml'")
    else:
        print("Error: Please provide the manifest file as an argument")
