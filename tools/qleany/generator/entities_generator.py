from jinja2 import Environment, FileSystemLoader
import yaml
import os
import sys
import stringcase
import shutil
import uncrustify
import clang_format_runner as clang_format_runner
from pathlib import Path

def generate_entities_enum_file(
    root_path: str,
    path: str,
    entities_list: list[dict],
    application_cpp_domain_name: str,
    files_to_be_generated: dict[str, bool],
):
    template_env = Environment(loader=FileSystemLoader("templates/entities"))
    entities_enum_template = template_env.get_template(
        "entities_enum_template.h.jinja2"
    )

    relative_entities_enum_file = os.path.join(path, "entities.h")

    if not files_to_be_generated.get(relative_entities_enum_file, False):
        return

    entities_enum_file = os.path.join(root_path, relative_entities_enum_file)

    rendered_template = entities_enum_template.render(
        entities_list=entities_list,
        application_cpp_domain_name=application_cpp_domain_name,
    )

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(entities_enum_file), exist_ok=True)

    with open(entities_enum_file, "w") as fh:
        fh.write(rendered_template)
        print(f"Successfully wrote file {entities_enum_file}")


def generate_entities_registration_file(
    root_path: str,
    path: str,
    entities_list: list[dict],
    export: str,
    export_header_file: str,
    headers: list[str],
    application_cpp_domain_name: str,
    files_to_be_generated: dict[str, bool],
):
    template_env = Environment(loader=FileSystemLoader("templates/entities"))
    entities_registration_template = template_env.get_template(
        "entities_registration_template.h.jinja2"
    )

    relative_entities_registration_output_file = os.path.join(
        path, "entities_registration.h"
    )
    entities_registration_output_file = os.path.join(
        root_path, relative_entities_registration_output_file
    )

    rendered_template = entities_registration_template.render(
        entities_list=entities_list,
        export=export,
        export_header_file=export_header_file,
        application_cpp_domain_name=application_cpp_domain_name,
        headers=headers,
    )

    if files_to_be_generated.get(relative_entities_registration_output_file, False):
        with open(entities_registration_output_file, "w") as fh:
            fh.write(rendered_template)
            print(f"Successfully wrote file {entities_registration_output_file}")


def generate_cmakelists(
    root_path: str,
    path: str,
    application_name: str,
    files_to_be_generated: dict[str, bool],
):
    # generate the cmakelists.txt

    template_env = Environment(loader=FileSystemLoader("templates/entities"))
    cmakelists_template = template_env.get_template("cmakelists_template.jinja2")

    relative_cmakelists_file = os.path.join(path, "CMakeLists.txt")

    if not files_to_be_generated.get(relative_cmakelists_file, False):
        return

    cmakelists_file = os.path.join(root_path, relative_cmakelists_file)

    rendered_template = cmakelists_template.render(
        application_spinalcase_name=stringcase.spinalcase(application_name),
        application_uppercase_name=stringcase.uppercase(application_name),
        application_snakecase_name=stringcase.snakecase(application_name),
    )

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(cmakelists_file), exist_ok=True)

    with open(cmakelists_file, "w") as fh:
        fh.write(rendered_template)
        print(f"Successfully wrote file {cmakelists_file}")


def generate_entity_files(
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

    entities_data = manifest_data.get("entities", [])
    entities_list = entities_data.get("list", [])
    # remove entities that are not to be generated
    entities_list = [entity for entity in entities_list]

    global_data = manifest_data.get("global", [])
    application_name = global_data.get("application_name", "")
    application_cpp_domain_name = global_data.get(
        "application_cpp_domain_name", "Undefined"
    )

    export = f"{stringcase.snakecase(application_name).upper()}_ENTITIES_EXPORT"
    export_header_file = f"{stringcase.snakecase(application_name)}_entities_export.h"
    path = entities_data.get("folder_path", ".")

    # Organize entities by name for easier lookup
    entities_by_name = {entity["name"]: entity for entity in entities_list}

    template_env = Environment(loader=FileSystemLoader("templates/entities"))
    template = template_env.get_template("entity_template.h.jinja2")

    # Default initialization values
    default_init_values = {
        "int": "0",
        "float": "0.0f",
        "double": "0.0",
        "bool": "false",
        "QString": "QString()",
        "QDateTime": "QDateTime()",
        "QUuid": "QUuid()",
        "QObject": "nullptr",
        "QList": "QList<>()",
    }

    def isUniqueForeignEntity(field_type: str) -> bool:
        for entity in entities_list:
            name = entity["name"]
            if name == field_type:
                return True

        return False

    def isListOrSetForeignEntity(field_type: str) -> bool:
        if "<" not in field_type:
            return False

        type = field_type.split("<")[1].split(">")[0].strip()

        for entity in entities_list:
            name = entity["name"]
            if name == type:
                return True

        return False

    def determine_relationships(
        entities_by_name: dict[str, dict], entity_name: str
    ) -> list[dict]:
        """
        Determine the relationships of the entity with the other entities. These relationships are defined in the other entities.
        """

        relationships = []

        # Loop through all the entities
        for entity in entities_by_name.values():
            # Loop through all the fields of the entity
            for field in entity["fields"]:
                # If the field is a foreign entity, check if it is the current entity
                if isListOrSetForeignEntity(field["type"]) or isUniqueForeignEntity(
                    field["type"]
                ):
                    if "<" in field["type"]:
                        other_entity_name = (
                            field["type"].split("<")[1].split(">")[0].strip()
                        )
                    else:
                        other_entity_name = field["type"]

                    if (
                        entity_name != other_entity_name
                        and entity_name != entity["name"]
                    ):
                        continue

                    # Determine the cardinality of the relationship
                    cardinality = "ManyUnordered"
                    if isUniqueForeignEntity(field["type"]):
                        cardinality = "One"
                    elif field["type"].startswith("QList") and field.get(
                        "ordered", False
                    ):
                        cardinality = "ManyOrdered"

                    # Determine the relationship type
                    relationship_type = (
                        "OneToOne" if cardinality == "One" else "OneToMany"
                    )

                    # Determine the direction of the relationship
                    direction = "Backward"
                    if entity_name == entity["name"]:
                        direction = "Forward"

                    # Add the relationship to the list
                    relationships.append(
                        {
                            "left_entity_name": entity["name"],
                            "right_entity_name": other_entity_name,
                            "field_name": field["name"],
                            "type": relationship_type,
                            "strength": "Strong"
                            if field.get("strong", True)
                            else "Weak",
                            "cardinality": cardinality,
                            "direction": direction,
                        }
                    )

        return relationships

    generated_files = []
    all_headers = []

    # add "active" field to entities fields in entities_list if entity activable is set to true

    for entity in entities_list:
        if entity.get("activable", False):
            entity["fields"].append(
                {
                    "name": "active",
                    "type": "bool",
                    "need_lazy_loader": False,
                    "name_pascal": "Active",
                    "is_primary_key": False,
                    "is_linked_to_another_entity": False,
                }
            )

    for entity in entities_list:
        name = entity["name"]
        fields = entity["fields"]
        parent = entity.get("parent", "EntityBase")
        relationships = []

        # add other informations to fields

        has_foreign_and_lazy_fields = False

        for field in fields:
            field["name_pascal"] = stringcase.pascalcase(field["name"])
            field["is_primary_key"] = False

            # add need_lazy_loader to fields
            if isListOrSetForeignEntity(field["type"]) or isUniqueForeignEntity(
                field["type"]
            ):
                has_foreign_and_lazy_fields = True
                field["need_lazy_loader"] = True
                field["is_linked_to_another_entity"] = True
            else:
                field["need_lazy_loader"] = False
                field["is_linked_to_another_entity"] = False

        # Get the headers to include based on the field types
        headers = []
        for field in fields:
            field_type = field["type"]
            if isListOrSetForeignEntity(field_type):
                include_header_file = field_type.split("<")[1].split(">")[0].strip()
                header_file = f'"{stringcase.snakecase(include_header_file)}.h"'
                headers.append(header_file)
            elif isUniqueForeignEntity(field_type):
                header_file = f'"{stringcase.snakecase(field_type)}.h"'
                headers.append(header_file)
            elif field_type in ["QString", "QDateTime", "QUuid"]:
                headers.append(f"<{field_type}>")

        # remove duplicates
        headers = list(set(headers))

        parent_header = f'"{stringcase.snakecase(parent)}.h"'
        if parent == "EntityBase":
            parent_header = '<qleany/entities/entity_base.h>'

        # If the parent is a custom entity defined in the manifest, include its fields as well. Also, it will add the fields parent of the parent recursively, butonly if the parent is defined in the
        # manifest. If the parent is not defined in the manifest, it will only add the fields of the current entity. But it adds the field "id" of type "int" from EntityBase.
        # This is done to avoid having to add the field "id" in every entity defined in the manifest.

        # Initialize an empty list for parent fields
        parent_fields = []

        # Check if the parent of the current entity is a custom entity defined in the manifest
        if parent in entities_by_name:
            parent_fields.append(
                {
                    "name": "id",
                    "type": "int",
                    "need_lazy_loader": False,
                    "name_pascal": "Id",
                    "is_primary_key": True,
                    "is_linked_to_another_entity": False,
                }
            )
            # If it is, then get the fields of the parent entity
            parent_fields += entities_by_name[parent]["fields"]

            # If parent has its own parent, recursively add its fields
            while (
                "parent" in entities_by_name[parent]
                and entities_by_name[parent]["parent"] in entities_by_name
            ):
                parent = entities_by_name[parent]["parent"]

                parent_fields += entities_by_name[parent]["fields"]
        else:
            parent_fields.append(
                {
                    "name": "id",
                    "type": "int",
                    "need_lazy_loader": False,
                    "name_pascal": "Id",
                    "is_primary_key": True,
                    "is_linked_to_another_entity": False,
                }
            )

        # add other informations to parent_fields
        for field in parent_fields:
            field["name_pascal"] = stringcase.pascalcase(field["name"])
            field["is_primary_key"] = True if field["name"] == "id" else False

            # add need_lazy_loader to fields
            if isListOrSetForeignEntity(field["type"]) or isUniqueForeignEntity(
                field["type"]
            ):
                field["need_lazy_loader"] = True
                field["is_linked_to_another_entity"] = True
            else:
                field["need_lazy_loader"] = False
                field["is_linked_to_another_entity"] = False

        # only keep the fields from the current entity for initialization
        fields_init_values = ", ".join(
            [
                f"m_{field['name']}({default_init_values.get(field['type'].split('<')[0], '{}')})"
                for field in fields
                if not field["need_lazy_loader"]
            ]
        )
        fields_init_values = (
            ", " + fields_init_values if fields_init_values else fields_init_values
        )

        # use parent fields to initialize parent class in constructor
        parent_init_values = ", ".join(
            [
                f"{field['name']}({default_init_values.get(field['type'].split('<')[0], '{}')})"
                for field in parent_fields
            ]
        )

        # relationships
        relationships = determine_relationships(entities_by_name, name)

        rendered_template = template.render(
            name=name,
            parent=parent,
            fields=fields,
            parent_fields=parent_fields,
            headers=headers,
            parent_header=parent_header,
            export=export,
            export_header_file=export_header_file,
            fields_init_values=fields_init_values,
            parent_init_values=parent_init_values,
            has_foreign_and_lazy_fields=has_foreign_and_lazy_fields,
            relationships=relationships,
            application_cpp_domain_name=application_cpp_domain_name,
        )

        relative_output_file = os.path.join(path, f"{stringcase.snakecase(name)}.h")
        output_file = os.path.join(root_path, relative_output_file)

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(output_file), exist_ok=True)

        if not files_to_be_generated.get(relative_output_file, False):
            continue

        # Add the generated file to the list
        generated_files.append(output_file)

        with open(output_file, "w") as fh:
            fh.write(rendered_template)
            print(f"Successfully wrote file {output_file}")

        # if uncrustify_config_file:
        #     uncrustify.run_uncrustify(output_file, uncrustify_config_file)
        clang_format_runner.run_clang_format(output_file)

    # generate entities cmake file

    entities_cmake_template = template_env.get_template("entities.cmake.jinja2")

    relative_cmake_output_file = os.path.join(path, "entities.cmake")
    cmake_output_file = os.path.join(root_path, relative_cmake_output_file)

    entities = []
    for entity in entities_list:
        entity_header = f'{stringcase.snakecase(entity["name"])}.h'
        relative_path = os.path.relpath(
            os.path.join(root_path, path, entity_header),
            os.path.dirname(cmake_output_file),
        )
        entities.append(relative_path.replace("\\", "/"))

    rendered_template = entities_cmake_template.render(
        entities=entities,
    )

    if files_to_be_generated.get(relative_cmake_output_file, False):
        with open(cmake_output_file, "w") as fh:
            fh.write(rendered_template)
            print(f"Successfully wrote file {output_file}")

    # generate entities enum file
    generate_entities_enum_file(
        root_path,
        path,
        entities_list,
        application_cpp_domain_name,
        files_to_be_generated,
    )

    # generate domain registration file
    for entity in entities_list:
        name = entity["name"]
        header_file = f'"{stringcase.snakecase(name)}.h"'
        all_headers.append(header_file)

    generate_entities_registration_file(
        root_path,
        path,
        entities_list,
        export,
        export_header_file,
        all_headers,
        application_cpp_domain_name,
        files_to_be_generated,
    )

    generate_cmakelists(
        root_path,
        path,
        manifest_data["global"]["application_name"],
        files_to_be_generated,
    )


def get_files_to_be_generated(
    manifest_file: str, files_to_be_generated: dict[str, bool] = None
) -> list[str]:
    """
    Get the list of files that need to be generated based on the manifest file
    """
    # Read the manifest file
    with open(manifest_file, "r") as fh:
        manifest = yaml.safe_load(fh)

    folder_path = manifest["entities"]["folder_path"]

    global_data = manifest.get("global", {})
    application_name = global_data.get("application_name", "example")

    # Get the list of files to be generated
    files = []
    for entity in manifest["entities"]["list"]:
        entity_name = entity["name"]
        files.append(
            os.path.join(folder_path, f"{stringcase.snakecase(entity_name)}.h")
        )

    # add list cmake file:
    list_file = os.path.join(folder_path, "entities.cmake")
    files.append(list_file)

    files.append(os.path.join(folder_path, "entities.h"))
    files.append(os.path.join(folder_path, "entities_registration.h"))
    files.append(os.path.join(folder_path, "CMakeLists.txt"))

    # strip from files if the value in files_to_be_generated is False
    if files_to_be_generated:
        for path, generate in files_to_be_generated.items():
            if not generate and path in files:
                files.remove(path)

    return files


# generate the files into the preview folder
def preview_entity_files(
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
    manifest["entities"]["folder_path"] = manifest["entities"]["folder_path"].replace(
        "..", ""
    )

    # write the modified manifest file
    with open(manifest_preview_file, "w") as fh:
        yaml.dump(manifest, fh)

    root_path = os.path.join(root_path, "qleany_preview")

    # remove .. from the path
    if files_to_be_generated:
        preview_files_to_be_generated = {}
        for path, value in files_to_be_generated.items():
            preview_files_to_be_generated[path.replace("..", "")] = value

        generate_entity_files(
            root_path,
            manifest_preview_file,
            preview_files_to_be_generated,
            uncrustify_config_file,
        )
    else:
        generate_entity_files(
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
                preview_entity_files(root_path, manifest_file)
            else:
                # generate the files
                generate_entity_files(root_path, manifest_file)
        else:
            print("Error: Manifest file must be named 'qleany.yaml' or 'qleany.yml'")
    else:
        print("Error: Please provide the manifest file as an argument")
