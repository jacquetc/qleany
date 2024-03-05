from jinja2 import Environment, FileSystemLoader
import yaml
import os
import sys
import stringcase
import shutil
import copy
import uncrustify
import clang_format_runner as clang_format_runner
from pathlib import Path
from collections import OrderedDict
import generation_dict_tools as tools


def get_dto_dict_and_feature_ordered_dict(
    feature_by_name: dict, entities_by_name: dict
) -> tuple[dict, OrderedDict]:

    def determine_dto_dependencies_from_fields(fields: list) -> list:
        dto_dependencies = []

        for field in fields:
            if field["is_foreign"]:
                dto_dependencies.append(field["foreign_dto_type"])

        return dto_dependencies

    dto_dict = {}

    for feature in feature_by_name.values():
        # fetch basic entity DTOs

        feature_dto_data = feature.get("DTO", {})
        dto_identical_to_entity = feature_dto_data.get("dto_identical_to_entity", {})

        entity_mappable_with = dto_identical_to_entity.get("entity_mappable_with", "")
        generate_dto_identical_to_entity = dto_identical_to_entity.get(
            "enabled", False
        ) and dto_identical_to_entity.get("entity_mappable_with", "")

        # create DTO entry without foreign DTOs

        if generate_dto_identical_to_entity:
            dto_type_name = f"{stringcase.pascalcase(entity_mappable_with)}DTO"

            dto_dict[dto_type_name] = {
                "feature_name": stringcase.pascalcase(feature["name"]),
                "entity_mappable_with": entity_mappable_with,
                "file_name": f"{stringcase.snakecase(feature['name'])}_dto.h",
                "is_relationship_dto": False,
                "fields": [],
            }

            # add fields without foreign entities
            fields = tools.get_fields_without_foreign_entities(
                entities_by_name[entity_mappable_with]["fields"],
                entities_by_name,
                entity_mappable_with,
            )
            # remove if hidden
            fields = [field for field in fields if not field["hidden"]]

            dto_dict[dto_type_name]["fields"] = fields

            dto_dict[dto_type_name]["dto_dependencies"] = []

        # create DTO entry with foreign DTOs (details)

        if generate_dto_identical_to_entity and tools.does_entity_have_relation_fields(
            entity_mappable_with, entities_by_name
        ):
            dto_type_name = f"{stringcase.pascalcase(feature['name'])}WithDetailsDTO"
            dto_dict[dto_type_name] = {
                "feature_name": stringcase.pascalcase(feature["name"]),
                "entity_mappable_with": entity_mappable_with,
                "file_name": f"{stringcase.snakecase(feature['name'])}_with_details_dto.h",
                "is_relationship_dto": False,
                "fields": [],
            }

            # add fields with foreign entities
            fields = tools.get_fields_with_foreign_entities(
                entities_by_name[entity_mappable_with]["fields"],
                entities_by_name,
                entity_mappable_with,
            )
            # remove if hidden
            fields = [field for field in fields if not field["hidden"]]

            dto_dict[dto_type_name]["fields"] = fields
            dto_dict[dto_type_name][
                "dto_dependencies"
            ] = determine_dto_dependencies_from_fields(
                dto_dict[dto_type_name]["fields"]
            )

        # fetch CRUD DTOs

        crud_data = feature.get("CRUD", {})
        crud_enabled = feature.get("CRUD", {}).get("enabled", {})
        entity_mappable_with = crud_data.get("entity_mappable_with", "")

        if crud_data and crud_enabled and entity_mappable_with:
            generate_create = False
            generate_update = False

            if crud_data["enabled"]:
                generate_create = crud_data.get("create", {}).get("enabled", False)
                generate_update = crud_data.get("update", {}).get("enabled", False)
                generate_insert_relation = tools.does_entity_have_relation_fields(
                    entity_mappable_with, entities_by_name, False
                )
                generate_move = crud_data.get("move_in_relative", {}).get(
                    "enabled", False
                )

            if generate_create:
                dto_type_name = f"Create{stringcase.pascalcase(feature['name'])}DTO"
                dto_dict[dto_type_name] = {
                    "feature_name": stringcase.pascalcase(feature["name"]),
                    "entity_mappable_with": entity_mappable_with,
                    "file_name": f"create_{stringcase.snakecase(feature['name'])}_dto.h",
                    "is_relationship_dto": False,
                    "fields": [],
                }

                # add fields with foreign entities but without id
                fields = tools.get_fields_with_foreign_entities(
                    entities_by_name[entity_mappable_with]["fields"],
                    entities_by_name,
                    entity_mappable_with,
                )
                # remove hidden fields
                fields = [field for field in fields if not field["hidden"]]
                dto_dict[dto_type_name]["fields"] = fields

                # remove id field
                dto_dict[dto_type_name]["fields"] = [
                    field
                    for field in dto_dict[dto_type_name]["fields"]
                    if field["name"] != "id"
                ]

                # add owner id
                owner_dict = tools.determine_owner(
                    entity_mappable_with, entities_by_name
                )
                if owner_dict:
                    owner_name_pascal = stringcase.pascalcase(
                        owner_dict.get("name", "")
                    )
                    owner_name_camel = stringcase.camelcase(owner_dict.get("name", ""))
                    dto_dict[dto_type_name]["fields"].append(
                        {
                            "name": f"{owner_name_camel}Id",
                            "type": "int",
                            "pascal_name": f"{owner_name_pascal}Id",
                            "is_foreign": False,
                        }
                    )

                    # add "position" if it has an owner and the field of owner entity is a QList
                    owner = owner_dict.get("name", "")
                    owner_field = owner_dict.get("field", "")

                    if owner_dict.get("is_list", False) and owner_dict.get(
                        "ordered", False
                    ):
                        dto_dict[dto_type_name]["fields"].append(
                            {
                                "name": "position",
                                "type": "int",
                                "pascal_name": "Position",
                                "is_foreign": False,
                            }
                        )

                dto_dict[dto_type_name][
                    "dto_dependencies"
                ] = determine_dto_dependencies_from_fields(
                    dto_dict[dto_type_name]["fields"]
                )

            if generate_update:
                dto_type_name = f"Update{stringcase.pascalcase(feature['name'])}DTO"
                dto_dict[dto_type_name] = {
                    "feature_name": stringcase.pascalcase(feature["name"]),
                    "entity_mappable_with": entity_mappable_with,
                    "file_name": f"update_{stringcase.snakecase(feature['name'])}_dto.h",
                    "is_relationship_dto": False,
                    "fields": [],
                }

                # add fields with foreign entities
                fields = tools.get_fields_with_foreign_entities(
                    entities_by_name[entity_mappable_with]["fields"],
                    entities_by_name,
                    entity_mappable_with,
                )
                # remove hidden fields
                fields = [field for field in fields if not field["hidden"]]

                dto_dict[dto_type_name]["fields"] = fields

                dto_dict[dto_type_name][
                    "dto_dependencies"
                ] = determine_dto_dependencies_from_fields(
                    dto_dict[dto_type_name]["fields"]
                )

            if generate_insert_relation:
                # DTO in
                dto_type_name = f"{stringcase.pascalcase(feature['name'])}RelationDTO"
                dto_dict[dto_type_name] = {
                    "feature_name": stringcase.pascalcase(feature["name"]),
                    "entity_mappable_with": "",
                    "file_name": f"{stringcase.snakecase(feature['name'])}_relation_dto.h",
                    "is_relation_dto": True,
                    "fields": [],
                    "relation_fields": [],
                }

                # add fields
                dto_dict[dto_type_name]["fields"].append(
                    {
                        "name": "id",
                        "pascal_name": "Id",
                        "is_foreign": False,
                        "type": "int",
                    }
                )
                dto_dict[dto_type_name]["fields"].append(
                    {
                        "name": "relationField",
                        "pascal_name": "RelationshipField",
                        "is_foreign": False,
                        "type": "RelationField",
                    }
                )
                dto_dict[dto_type_name]["fields"].append(
                    {
                        "name": "relatedIds",
                        "pascal_name": "RelatedIds",
                        "is_foreign": False,
                        "type": "QList<int>",
                    }
                )
                dto_dict[dto_type_name]["fields"].append(
                    {
                        "name": "position",
                        "pascal_name": "Position",
                        "is_foreign": False,
                        "type": "int",
                    }
                )       

                # add relationship fields

                dto_dict[dto_type_name][
                    "relation_fields"
                ] = tools.get_only_fields_with_foreign_entities(
                    entities_by_name[entity_mappable_with]["fields"],
                    entities_by_name,
                    entity_mappable_with,
                )

                dto_dict[dto_type_name]["dto_dependencies"] = []

        # fetch command DTOs
        for command in feature.get("commands", []):
            command_dto_data = command.get("dto", {})
            dto_in = command_dto_data.get("in", {})
            if dto_in and dto_in.get("enabled", True):
                dto_type_name = f"{dto_in['type_prefix']}DTO"
                dto_dict[dto_type_name] = {
                    "feature_name": stringcase.pascalcase(feature["name"]),
                    "entity_mappable_with": "",
                    "file_name": f"{stringcase.snakecase(dto_in['type_prefix'])}_dto.h",
                    "is_relationship_dto": False,
                    "fields": [],
                }

                # add fields with foreign entities
                fields = tools.get_fields_with_foreign_entities(
                    dto_in["fields"], entities_by_name
                )
                # remove hidden fields
                fields = [field for field in fields if not field["hidden"]]
                dto_dict[dto_type_name]["fields"] = fields

                dto_dict[dto_type_name][
                    "dto_dependencies"
                ] = determine_dto_dependencies_from_fields(
                    dto_dict[dto_type_name]["fields"]
                )

            dto_out = command_dto_data.get("out", {})
            if dto_out and dto_out.get("enabled", True):
                dto_type_name = f"{dto_out['type_prefix']}DTO"
                dto_dict[dto_type_name] = {
                    "feature_name": stringcase.pascalcase(feature["name"]),
                    "entity_mappable_with": "",
                    "file_name": f"{stringcase.snakecase(dto_out['type_prefix'])}_dto.h",
                    "is_relationship_dto": False,
                    "fields": [],
                }

                # add fields with foreign entities
                fields = tools.get_fields_with_foreign_entities(
                    dto_out["fields"], entities_by_name
                )
                # remove hidden fields
                fields = [field for field in fields if not field["hidden"]]
                dto_dict[dto_type_name]["fields"] = fields
                dto_dict[dto_type_name][
                    "dto_dependencies"
                ] = determine_dto_dependencies_from_fields(
                    dto_dict[dto_type_name]["fields"]
                )

        # fetch query DTOs
        for query in feature.get("queries", []):
            query_dto_data = query.get("dto", {})
            dto_in = query_dto_data.get("in", {})
            if dto_in and dto_in.get("enabled", True):
                dto_type_name = f"{dto_in['type_prefix']}DTO"
                dto_dict[dto_type_name] = {
                    "feature_name": stringcase.pascalcase(feature["name"]),
                    "entity_mappable_with": "",
                    "file_name": f"{stringcase.snakecase(dto_in['type_prefix'])}_dto.h",
                    "is_relationship_dto": False,
                    "fields": [],
                }

                # add fields with foreign entities
                fields = tools.get_fields_with_foreign_entities(
                    dto_in["fields"], entities_by_name
                )
                # remove hidden fields
                fields = [field for field in fields if not field["hidden"]]
                dto_dict[dto_type_name]["fields"] = fields

                dto_dict[dto_type_name][
                    "dto_dependencies"
                ] = determine_dto_dependencies_from_fields(
                    dto_dict[dto_type_name]["fields"]
                )

            dto_out = query_dto_data.get("out", {})
            if dto_out and dto_out.get("enabled", True):
                dto_type_name = f"{dto_out['type_prefix']}DTO"
                dto_dict[dto_type_name] = {
                    "feature_name": stringcase.pascalcase(feature["name"]),
                    "entity_mappable_with": "",
                    "file_name": f"{stringcase.snakecase(dto_out['type_prefix'])}_dto.h",
                    "is_relationship_dto": False,
                    "fields": [],
                }

                # add fields with foreign entities
                fields = tools.get_fields_with_foreign_entities(
                    dto_out["fields"], entities_by_name
                )
                # remove hidden fields
                fields = [field for field in fields if not field["hidden"]]
                dto_dict[dto_type_name]["fields"] = fields
                dto_dict[dto_type_name][
                    "dto_dependencies"
                ] = determine_dto_dependencies_from_fields(
                    dto_dict[dto_type_name]["fields"]
                )

    # create header files for DTOs
    for dto_type_name, dto_data in dto_dict.items():
        header_list = []
        for field in dto_data["fields"]:
            if field["type"] in ["QString", "QDateTime", "QUuid", "QUrl"]:
                header_list.append(f"<{field['type']}>")

        dto_dependencies = dto_data.get("dto_dependencies", [])
        for dto_dependency in dto_dependencies:
            if dto_dependency in dto_dict:
                dto_dependency_file_name = dto_dict[dto_dependency]["file_name"]
                header_list.append(
                    f'"{stringcase.snakecase(dto_dependency.replace("DTO", ""))}/{dto_dependency_file_name}"'
                )

        header_list = list(set(header_list))
        header_list.sort()

        dto_data["header_list"] = header_list

    # add "foreign_feature_name" to foreign fields in DTOs
    for dto_type_name, dto_data in dto_dict.items():
        for field in dto_data["fields"]:
            if not field["is_foreign"]:
                continue
            try:
                foreign_dto_type = field["foreign_dto_type"]
                field["foreign_feature_name"] = dto_dict[foreign_dto_type][
                    "feature_name"
                ]
            except KeyError:
                print(
                    f"ERROR: DTO \"{dto_type_name}\" has a foreign field \"{field['name']}\" with foreign DTO type \"{foreign_dto_type}\" that does not exist. Maybe you forgot to create the corresponding feature ?"
                )
            # dto_data["fields"][i] = field

    # create a new dict "feature_dto_dict" with only the DTOs that are needed for the feature. Each key is a feature name and each value is a dict.
    # The dict contains "dtos" with the list of DTOs. The dict also contains a "feature_dependencies" list with the features that the feature depends on.

    feature_dto_dict = {}
    for dto_type_name, dto_data in dto_dict.items():
        feature_name = dto_data["feature_name"]
        if feature_name not in feature_dto_dict:
            feature_dto_dict[feature_name] = {
                "feature_dependencies": [],
                "dtos": [],
            }

        if not feature_dto_dict[feature_name].get("dtos", {}):
            feature_dto_dict[feature_name]["dtos"] = {}
        feature_dto_dict[feature_name]["dtos"][dto_type_name] = dto_data

        if dto_data["dto_dependencies"]:
            feature_dto_dict[feature_name]["feature_dependencies"].extend(
                dto_data["dto_dependencies"]
            )

    # replace the DTOs in "feature_dependencies" with the feature name of each of these DTOs
    for feature_name, feature_data in feature_dto_dict.items():
        for i, feature_dependency in enumerate(feature_data["feature_dependencies"]):
            feature_data["feature_dependencies"][i] = dto_dict[feature_dependency][
                "feature_name"
            ]

    # remove duplicates from feature_dependencies
    for feature_name, feature_data in feature_dto_dict.items():
        feature_data["feature_dependencies"] = list(
            set(feature_data["feature_dependencies"])
        )

    # order the features by their dependencies, so that the features that depend on other features are generated after the features they depend on

    dto_ordered_dict = OrderedDict()

    for feature_name, feature_data in feature_dto_dict.items():
        # case if a dto depends on another dto in the same feature
        if feature_name in feature_data["feature_dependencies"]:
            feature_data["feature_dependencies"].remove(feature_name)

    original_feature_dto_dict = copy.deepcopy(feature_dto_dict)

    while len(feature_dto_dict) > 0:
        for feature_name, feature_data in feature_dto_dict.items():
            # case if a dto depends on another dto in another feature
            if len(feature_data["feature_dependencies"]) == 0:
                dto_ordered_dict[feature_name] = {}
                dto_ordered_dict[feature_name]["dtos"] = feature_data["dtos"]
                dto_ordered_dict[feature_name][
                    "feature_dependencies"
                ] = original_feature_dto_dict[feature_name]["feature_dependencies"]

                del feature_dto_dict[feature_name]
                for feature_name2, feature_data2 in feature_dto_dict.items():
                    if feature_name in feature_data2["feature_dependencies"]:
                        feature_data2["feature_dependencies"].remove(feature_name)
                break

    return dto_dict, dto_ordered_dict


def getEntityFromForeignFieldType(field_type: str, entities_by_name: dict) -> str:
    if "<" not in field_type:
        return field_type

    type = field_type.split("<")[1].split(">")[0].strip()

    for entity_name in entities_by_name:
        if entity_name == type:
            return entity_name

    return ""


def is_unique_foreign_dto(dto_list: list, field_type: str) -> bool:
    for dto_type in dto_list:
        if dto_type == field_type:
            return True

    return False


def is_list_foreign_dto(dto_list: list, field_type: str) -> bool:
    if "<" not in field_type:
        return False

    type = field_type.split("<")[1].split(">")[0].strip()

    for dto_type in dto_list:
        if dto_type == type:
            return True

    return False


def generate_dto(
    root_path: str,
    template,
    dto_type,
    dto_data,
    dto_common_cmake_folder_path,
    default_init_values,
    application_cpp_domain_name,
    files_to_be_generated,
):
    fields = dto_data.get("fields", [])

    # only keep the fields from the current entity for initialization
    fields_init_values = ", ".join(
        [
            f"m_{field['name']}({default_init_values.get(field['type'], '{}')})"
            for field in fields
            if not field["is_foreign"]
        ]
    )

    # Get the headers to include based on the field types
    headers = dto_data["header_list"]

    dto_file_path = os.path.join(
        dto_common_cmake_folder_path,
        stringcase.snakecase(dto_data["feature_name"] + "_dto"),
        stringcase.snakecase(dto_data["feature_name"]),
        dto_data["file_name"],
    )

    if not files_to_be_generated.get(dto_file_path, False):
        return None

    dto_file_path = os.path.join(root_path, dto_file_path)

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(dto_file_path), exist_ok=True)

    with open(dto_file_path, "w") as f:
        f.write(
            template.render(
                feature_pascal_name=dto_data["feature_name"],
                dto_pascal_type=dto_type,
                fields=fields,
                is_relation_dto=dto_data.get("is_relation_dto", False),
                relation_fields=dto_data.get("relation_fields", []),
                headers=headers,
                fields_init_values=fields_init_values,
                application_cpp_domain_name=application_cpp_domain_name,
            )
        )
        print(f"Successfully wrote file {dto_file_path}")

    return dto_file_path


def generate_dto_files(
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

    dto_data = manifest_data.get("DTOs", [])
    application_name = manifest_data.get("global", {}).get(
        "application_name", "example"
    )
    application_name = stringcase.spinalcase(application_name)
    dto_common_cmake_folder_path = dto_data.get("common_cmake_folder_path", "")

    application_data = manifest_data.get("application", [])
    feature_list = application_data.get("features", [])

    global_data = manifest_data.get("global", [])
    application_cpp_domain_name = global_data.get(
        "application_cpp_domain_name", "Undefined"
    )

    # Organize feature_list by name for easier lookup
    feature_by_name = {feature["name"]: feature for feature in feature_list}

    template_env = Environment(loader=FileSystemLoader("templates/DTOs"))
    template = template_env.get_template("dto_template.h.jinja2")

    entities_data = manifest_data.get("entities", [])
    entities_list = entities_data.get("list", [])

    # Organize entities by name for easier lookup
    entities_by_name = {entity["name"]: entity for entity in entities_list}

    # Default initialization values
    default_init_values = {
        "int": "0",
        "float": "0.0f",
        "double": "0.0",
        "bool": "false",
        "QString": "QString()",
        "QDateTime": "QDateTime()",
        "QUuid": "QUuid()",
        "QUrl": "QUrl()",
        "QObject": "nullptr",
        "QList": "QList<>()",
        "RelationField": "RelationField::Undefined",
    }

    dto_dict, feature_ordered_dict = get_dto_dict_and_feature_ordered_dict(
        feature_by_name, entities_by_name
    )

    for feature_name, feature_data in feature_ordered_dict.items():
        generated_files = []
        feature_snake_name = stringcase.snakecase(feature_name)

        for dto_type, dto_data in feature_data["dtos"].items():
            generated_files.append(
                generate_dto(
                    root_path,
                    template,
                    dto_type,
                    dto_data,
                    dto_common_cmake_folder_path,
                    default_init_values,
                    application_cpp_domain_name,
                    files_to_be_generated,
                )
            )

        # strip generated files of None values
        generated_files = [file for file in generated_files if file]

        # generate these DTO's cmakelists.txt
        dto_cmakelists_template = template_env.get_template(
            "cmakelists_template.jinja2"
        )

        relative_dto_cmakelists_file = os.path.join(
            dto_common_cmake_folder_path,
            feature_snake_name + "_dto",
            "CMakeLists.txt",
        )
        dto_cmakelists_file = os.path.join(
            root_path,
            dto_common_cmake_folder_path,
            feature_snake_name + "_dto",
            "CMakeLists.txt",
        )

        # get all dto files, even those not generated
        dto_files = []
        for dto_type, dto_data in feature_data["dtos"].items():
            dto_files.append(
                os.path.join(
                    root_path,
                    dto_common_cmake_folder_path,
                    feature_snake_name + "_dto",
                    feature_snake_name,
                    dto_data["file_name"],
                )
            )

        ## Convert the file path to be relative to the directory of the cmakelists
        relative_dto_files = []
        for file_path in dto_files:
            relative_generated_file = os.path.relpath(
                file_path, os.path.dirname(dto_cmakelists_file)
            )
            relative_dto_files.append(relative_generated_file.replace("\\", "/"))

        # Get the feature dependencies and make them spinal case
        spinal_case_feature_dependencies = []
        for feature_dependency in feature_data["feature_dependencies"]:
            spinal_case_feature_dependencies.append(
                stringcase.spinalcase(feature_dependency)
            )

        rendered_template = dto_cmakelists_template.render(
            feature_spinal_name=stringcase.spinalcase(feature_snake_name),
            files=relative_dto_files,
            application_name=application_name,
            feature_dependencies=spinal_case_feature_dependencies,
        )

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(dto_cmakelists_file), exist_ok=True)

        if files_to_be_generated.get(relative_dto_cmakelists_file, False):
            with open(dto_cmakelists_file, "w") as fh:
                fh.write(rendered_template)
                print(f"Successfully wrote file {dto_cmakelists_file}")

    # generate common cmakelists.txt

    relative_dto_common_cmakelists_file = os.path.join(
        dto_common_cmake_folder_path, "CMakeLists.txt"
    )
    dto_common_cmakelists_file = os.path.join(
        root_path, relative_dto_common_cmakelists_file
    )
    if files_to_be_generated.get(relative_dto_common_cmakelists_file, False):
        ## After the loop, write the list of features folders to the common cmakelists.txt
        with open(dto_common_cmakelists_file, "w") as fh:
            for feature_name, _ in feature_ordered_dict.items():
                fh.write(
                    f"add_subdirectory({stringcase.snakecase(feature_name)}_dto)" + "\n"
                )
            print(f"Successfully wrote file {dto_common_cmakelists_file}")

    # format the files
    for file, to_be_generated in files_to_be_generated.items():
        # if uncrustify_config_file and files_to_be_generated.get(file, False):
        #     uncrustify.run_uncrustify(file, uncrustify_config_file)
        if to_be_generated and file.endswith(".h") or file.endswith(".cpp"):
            clang_format_runner.run_clang_format(os.path.join(root_path, file))


def get_files_to_be_generated(
    manifest_file: str, files_to_be_generated: dict[str, bool] = None
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

    dto_data = manifest_data.get("DTOs", [])
    dto_common_cmake_folder_path = dto_data.get("common_cmake_folder_path", "")

    application_data = manifest_data.get("application", [])
    feature_list = application_data.get("features", [])

    # Organize feature_list by name for easier lookup
    feature_by_name = {feature["name"]: feature for feature in feature_list}

    entities_data = manifest_data.get("entities", [])
    entities_list = entities_data.get("list", [])

    # Organize entities by name for easier lookup
    entities_by_name = {entity["name"]: entity for entity in entities_list}

    dto_dict, feature_ordered_dict = get_dto_dict_and_feature_ordered_dict(
        feature_by_name, entities_by_name
    )

    files = []
    for feature_name, feature_data in feature_ordered_dict.items():
        feature_snake_name = stringcase.snakecase(feature_name)

        for dto_type, dto_data in feature_data["dtos"].items():
            files.append(
                os.path.join(
                    dto_common_cmake_folder_path,
                    feature_snake_name + "_dto",
                    feature_snake_name,
                    dto_data["file_name"],
                )
            )

        dto_cmakelists_file = os.path.join(
            dto_common_cmake_folder_path,
            feature_snake_name + "_dto",
            "CMakeLists.txt",
        )

        files.append(dto_cmakelists_file)

    # generate common cmakelists.txt
    dto_cmakelists_file = os.path.join(dto_common_cmake_folder_path, "CMakeLists.txt")

    files.append(dto_cmakelists_file)

    # strip from files if the value in files_to_be_generated is False
    if files_to_be_generated:
        for path, generate in files_to_be_generated.items():
            if not generate and path in files:
                files.remove(path)

    return files


# generate the files into the preview folder
def preview_dto_files(
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

    # remove .. from the path
    manifest["DTOs"]["common_cmake_folder_path"] = manifest["DTOs"][
        "common_cmake_folder_path"
    ].replace("..", "")

    # write the modified manifest file
    with open(manifest_preview_file, "w") as fh:
        yaml.dump(manifest, fh)

    root_path = os.path.join(root_path, "qleany_preview")

    #  remove .. from the path
    if files_to_be_generated:
        preview_files_to_be_generated = {}
        for path, value in files_to_be_generated.items():
            preview_files_to_be_generated[path.replace("..", "")] = value

        generate_dto_files(
            root_path,
            manifest_preview_file,
            preview_files_to_be_generated,
            uncrustify_config_file,
        )

    else:
        generate_dto_files(root_path, manifest_preview_file, {}, uncrustify_config_file)


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
                preview_dto_files(root_path, manifest_file)
            else:
                # generate the files
                generate_dto_files(root_path, manifest_file)

        else:
            print("Error: Manifest file must be named 'qleany.yaml' or 'qleany.yml'")
    else:
        print("Error: Please provide the manifest file as an argument")
