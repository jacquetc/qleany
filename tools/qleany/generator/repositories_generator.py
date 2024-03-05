from jinja2 import Environment, FileSystemLoader
import yaml
import os
import sys
import stringcase
import shutil
import uncrustify
import clang_format_runner as clang_format_runner
from pathlib import Path
from copy import deepcopy
import generation_dict_tools as tools


def _generate_cmakelists(
    root_path: str,
    path: str,
    application_name: str,
    files_to_be_generated: dict[str, bool],
):
    # generate the cmakelists.txt

    template_env = Environment(loader=FileSystemLoader("templates/repositories"))
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


def _generate_contracts_cmakelists(
    root_path: str,
    path: str,
    application_name: str,
    files_to_be_generated: dict[str, bool],
):
    # generate the cmakelists.txt

    template_env = Environment(loader=FileSystemLoader("templates/repositories"))
    cmakelists_template = template_env.get_template(
        "contracts_cmakelists_template.jinja2"
    )

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


def generate_repository_files(
    root_path: str,
    manifest_file,
    files_to_be_generated: dict[str, bool] = None,
    uncrustify_config_file: str = None,
):
    with open(manifest_file, "r") as stream:
        try:
            manifest_data = yaml.safe_load(stream)
        except yaml.YAMLError as exc:
            print(exc)
            return

    repositories_data = manifest_data.get("repositories", {})
    repositories_list = repositories_data.get("list", [])
    repository_path = repositories_data.get("repository_folder_path", ".")
    base_path = repositories_data.get("base_folder_path", ".")

    global_data = manifest_data.get("global", {})
    application_name = global_data.get("application_name", "example")
    application_cpp_domain_name = global_data.get(
        "application_cpp_domain_name", "Undefined"
    )

    template_env = Environment(loader=FileSystemLoader("templates/repositories"))
    repo_template = template_env.get_template("repository_template.h.jinja2")
    repo_cpp_template = template_env.get_template("repository_template.cpp.jinja2")
    interface_template = template_env.get_template(
        "interface_repository_template.h.jinja2"
    )

    contracts_data = manifest_data.get("contracts", {})
    inverted_app_domain = contracts_data.get("inverted_app_domain", "com.example")
    contracts_folder_path = contracts_data.get("folder_path", ".")

    interface_path = os.path.join(contracts_folder_path, "repository")

    entities_data = manifest_data.get("entities", {})
    entities_list = entities_data.get("list", [])

    # exports:

    export = f"{stringcase.snakecase(application_name).upper()}_PERSISTENCE_EXPORT"
    export_header_file = f"{stringcase.snakecase(application_name)}_persistence_export.h"
    contracts_export = f"{stringcase.snakecase(application_name).upper()}_CONTRACTS_EXPORT"
    contracts_export_header_file = f"{stringcase.snakecase(application_name)}_contracts_export.h"

    # Organize entities by name for easier lookup
    entities_by_name = {entity["name"]: entity for entity in entities_list}

    generated_files = []
    interface_generated_files = []

    all_repository_files = []
    all_interface_files = []

    for repository in repositories_list:
        name = repository["entity_name"]
        pascal_name = stringcase.pascalcase(name)
        snake_name = stringcase.snakecase(name)
        camel_name = stringcase.camelcase(name)
        generate_lazy_loaders = repository.get("lazy_loaders", True)

        # create a dict of foreign entities for this repository
        foreign_entities = {}
        for field in entities_by_name[name]["fields"]:
            field_type = field["type"]
            field_name = field["name"]
            if tools.is_unique_foreign_entity(field_type, entities_by_name):
                foreign_entities[f"{field_type}__{field_name}"] = deepcopy(
                    entities_by_name[field_type]
                )
            elif tools.is_list_foreign_entity(field_type, entities_by_name):
                foreign_entities[f"{field_type}__{field_name}"] = deepcopy(
                    entities_by_name[field_type.split("<")[1].split(">")[0].strip()]
                )

        foreign_entities = dict(sorted(foreign_entities.items()))

        # add helpful keys to foreign entities
        for key, value in foreign_entities.items():
            field_type = key.split("__")[0]
            field_name = key.split("__")[1]
            value["is_list"] = field_type.split("<")[
                0
            ] == "QList" and tools.is_list_foreign_entity(field_type, entities_by_name)
            if value["is_list"]:
                value["type_camel_name"] = stringcase.camelcase(
                    field_type.split("<")[1].split(">")[0].strip()
                )
            else:
                value["type_camel_name"] = stringcase.camelcase(field_type)

            if value["is_list"]:
                value["type_name_only"] = stringcase.pascalcase(
                    field_type.split("<")[1].split(">")[0].strip()
                )
            else:
                value["type_name_only"] = stringcase.pascalcase(field_type)

            if value["is_list"]:
                value["type_pascal_name"] = stringcase.pascalcase(
                    field_type.split("<")[1].split(">")[0].strip()
                )
            else:
                value["type_pascal_name"] = stringcase.pascalcase(field_type)

            if value["is_list"]:
                value["type_snake_name"] = stringcase.snakecase(
                    field_type.split("<")[1].split(">")[0].strip()
                )
            else:
                value["type_snake_name"] = stringcase.snakecase(field_type)

            value["related_field_name"] = field_name
            value["related_field_pascal_name"] = stringcase.pascalcase(field_name)

        foreign_repository_constructor_arguments = []
        for key, value in foreign_entities.items():
            new_constructor_argument = f"Interface{value['type_pascal_name']}Repository *{value['type_camel_name']}Repository"
            if (
                new_constructor_argument
                not in foreign_repository_constructor_arguments
            ):
                foreign_repository_constructor_arguments.append(
                    new_constructor_argument
                )
        foreign_repository_constructor_arguments.sort()

        foreign_repository_constructor_arguments_string = ", ".join(
            foreign_repository_constructor_arguments
        )
        foreign_repository_constructor_arguments_string = (
            ", " + foreign_repository_constructor_arguments_string
            if foreign_repository_constructor_arguments
            else foreign_repository_constructor_arguments_string
        )

        foreign_entities_private_member_list = []
        for key, value in foreign_entities.items():
            foreign_entities_private_member_list.append(
                f"Interface{value['type_pascal_name']}Repository *m_{value['type_camel_name']}Repository;"
            )
        # remove duplicates :
        foreign_entities_private_member_list = list(dict.fromkeys(foreign_entities_private_member_list))

        foreign_repository_header_list = []
        for key, value in foreign_entities.items():
            foreign_repository_header_list.append(
                f"\"repository/interface_{value['type_snake_name']}_repository.h\""
            )
        # remove duplicates :
        foreign_repository_header_list = list(
            dict.fromkeys(foreign_repository_header_list)
        )

        # loader functions like     Entities::Book::ChaptersLoader fetchChaptersLoader();

        loader_function_list = []
        if generate_lazy_loaders:
            for key, value in foreign_entities.items():
                loader_function_list.append(
                    f"{application_cpp_domain_name}::Entities::{name}::{value['related_field_pascal_name']}Loader fetch{value['related_field_pascal_name']}Loader() override;"
                )

        # Create .h file
        rendered_template = repo_template.render(
            name=name,
            pascal_name=pascal_name,
            snake_name=snake_name,
            camel_name=camel_name,
            generate_lazy_loaders=generate_lazy_loaders,
            foreign_entities=foreign_entities,
            foreign_repository_constructor_arguments_string=foreign_repository_constructor_arguments_string,
            foreign_entities_private_member_list=foreign_entities_private_member_list,
            loader_function_list=loader_function_list,
            export=export,
            export_header_file=export_header_file,
            foreign_repository_header_list=foreign_repository_header_list,
            application_cpp_domain_name=application_cpp_domain_name,
        )
        output_file = os.path.join(repository_path, f"{snake_name}_repository.h")
        all_repository_files.append(os.path.join(root_path, output_file))

        if files_to_be_generated.get(output_file, False):
            output_file = os.path.join(root_path, output_file)

            os.makedirs(os.path.dirname(output_file), exist_ok=True)
            with open(output_file, "w") as fh:
                fh.write(rendered_template)
                print(f"Successfully wrote file {output_file}")
            generated_files.append(output_file)

            # if uncrustify_config_file:
            #     uncrustify.run_uncrustify(output_file, uncrustify_config_file)
            clang_format_runner.run_clang_format(output_file)

        # prepare the fields init values

        fields_init_values = [
            f"m_{value['type_camel_name']}Repository({value['type_camel_name']}Repository)"
            for key, value in foreign_entities.items()
        ]
        # remove duplicates :
        fields_init_values = list(dict.fromkeys(fields_init_values))

        fields_init_values = ", ".join(fields_init_values)
        fields_init_values = (
            ", " + fields_init_values if fields_init_values else fields_init_values
        )

        # Create .cpp file
        rendered_template = repo_cpp_template.render(
            name=name,
            pascal_name=pascal_name,
            snake_name=snake_name,
            camel_name=camel_name,
            generate_lazy_loaders=generate_lazy_loaders,
            foreign_entities=foreign_entities,
            foreign_repository_constructor_arguments_string=foreign_repository_constructor_arguments_string,
            fields_init_values=fields_init_values,
            application_cpp_domain_name=application_cpp_domain_name,
        )
        output_file = os.path.join(repository_path, f"{snake_name}_repository.cpp")
        all_repository_files.append(os.path.join(root_path, output_file))

        if files_to_be_generated.get(output_file, False):
            output_file = os.path.join(root_path, output_file)

            os.makedirs(os.path.dirname(output_file), exist_ok=True)
            with open(output_file, "w") as fh:
                fh.write(rendered_template)
                print(f"Successfully wrote file {output_file}")
            generated_files.append(output_file)

            # if uncrustify_config_file:
            #     uncrustify.run_uncrustify(output_file, uncrustify_config_file)
            clang_format_runner.run_clang_format(output_file)

        # loader functions like     Entities::Book::ChaptersLoader fetchChaptersLoader();

        loader_function_list_for_interface = []
        if generate_lazy_loaders:
            for key, value in foreign_entities.items():
                loader_function_list_for_interface.append(
                    f"virtual {application_cpp_domain_name}::Entities::{name}::{value['related_field_pascal_name']}Loader fetch{value['related_field_pascal_name']}Loader() = 0;"
                )

        # Create interface .h file
        rendered_template = interface_template.render(
            name=name,
            snake_name=snake_name,
            camel_name=camel_name,
            foreign_entities=foreign_entities,
            inverted_app_domain=inverted_app_domain,
            contracts_export=contracts_export,
            contracts_export_header_file=contracts_export_header_file,
            loader_function_list_for_interface=loader_function_list_for_interface,
            application_cpp_domain_name=application_cpp_domain_name,
        )
        output_file = os.path.join(
            interface_path, f"interface_{snake_name}_repository.h"
        )
        all_interface_files.append(os.path.join(root_path, output_file))

        if files_to_be_generated.get(output_file, False):
            output_file = os.path.join(root_path, output_file)

            os.makedirs(os.path.dirname(output_file), exist_ok=True)
            with open(output_file, "w") as fh:
                fh.write(rendered_template)
                print(f"Successfully wrote file {output_file}")
            interface_generated_files.append(output_file)

            # if uncrustify_config_file:
            #     uncrustify.run_uncrustify(output_file, uncrustify_config_file)
            clang_format_runner.run_clang_format(output_file)

    # write the repository cmake list file

    repository_cmake_template = template_env.get_template("repositories.cmake.jinja2")
    relative_cmake_output_file = os.path.join(base_path, "repositories.cmake")
    cmake_output_file = os.path.join(root_path, relative_cmake_output_file)

    if files_to_be_generated.get(relative_cmake_output_file, False):
        repositories = []
        for repository in all_repository_files:
            repository_header = f"{repository}"
            relative_path = os.path.relpath(
                repository_header, os.path.dirname(cmake_output_file)
            )
            repositories.append(relative_path.replace("\\", "/"))

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(cmake_output_file), exist_ok=True)

        rendered_template = repository_cmake_template.render(
            repositories=repositories,
        )

        with open(cmake_output_file, "w") as fh:
            fh.write(rendered_template)
            print(f"Successfully wrote file {cmake_output_file}")

    # write the interface list file

    repository_interface_cmake_template = template_env.get_template(
        "repositories.cmake.jinja2"
    )
    relative_repository_interface_cmake_output_file = os.path.join(
        contracts_folder_path, "repository_interfaces.cmake"
    )
    repository_interface_cmake_output_file = os.path.join(
        root_path, relative_repository_interface_cmake_output_file
    )

    if files_to_be_generated.get(
        relative_repository_interface_cmake_output_file, False
    ):
        repositories = []
        for repository_interface in all_interface_files:
            repository_interface_header = f"{repository_interface}"
            relative_path = os.path.relpath(
                repository_interface_header,
                os.path.dirname(repository_interface_cmake_output_file),
            )
            repositories.append(relative_path.replace("\\", "/"))

        # Create the directory if it does not exist
        os.makedirs(
            os.path.dirname(repository_interface_cmake_output_file), exist_ok=True
        )

        rendered_template = repository_interface_cmake_template.render(
            repositories=repositories,
        )

        with open(repository_interface_cmake_output_file, "w") as fh:
            fh.write(rendered_template)
            print(f"Successfully wrote file {repository_interface_cmake_output_file}")

    def determine_direct_children(entity: dict) -> list:
        """
        Determine the direct children of an entity
        """
        direct_children = []
        for field in entity["fields"]:
            field_type = field["type"]
            if tools.is_unique_foreign_entity(field_type, entities_by_name):
                direct_children.append(field_type)
            elif tools.is_list_foreign_entity(field_type, entities_by_name):
                direct_children.append(field_type.split("<")[1].split(">")[0].strip())

        # alphabetize the list
        direct_children.sort()

        return direct_children

    def determine_order_of_registering_of_entities(entities_list: list) -> list:
        """
        Determine the children for each entity and determine an order of entities so the result is a list of entities in the order they should be registered.
        """

        # Remove entities that are only for heritage
        entities_list = [
            entity
            for entity in entities_list
            if not entity.get("only_for_heritage", False)
        ]

        # Get a dict of children for each entity
        children_for_each_entity = {}
        for entity in entities_list:
            children_for_each_entity[entity["name"]] = determine_direct_children(entity)

        root_entities = []
        for entity in entities_list:
            is_root_entity = True
            for children in children_for_each_entity.values():
                if entity["name"] in children:
                    is_root_entity = False
                    break
            if is_root_entity:
                root_entities.append(entity["name"])

        # alphabetize the list
        root_entities.sort()

        class EntityNode:
            def __init__(self, name: str, children: [str]):
                self.name = name
                self.children = children
                self.parents = []
                self.registered = False
                self.is_root = False

            def is_common_child(self):
                return len(self.other_parents) > 0

            def add_child(self, child: str):
                self.children.append(child)

            def add_parent(self, parent: str):
                self.parents.append(parent)

        # create a list of Entity objects
        entities = []

        # add
        for entity in entities_list:
            entities.append(
                EntityNode(entity["name"], children_for_each_entity[entity["name"]])
            )

        # add the parents to each entity
        for entity in entities:
            for child in entity.children:
                for other_entity in entities:
                    if child == other_entity.name:
                        other_entity.add_parent(entity.name)

        # if the entity has no parent, it is a root entity
        for entity in entities:
            if not entity.parents:
                entity.is_root = True

        # create a list of root entities
        root_entities = []
        for entity in entities:
            if entity.is_root:
                root_entities.append(entity.name)

        # add the root entities to the ordered_entities list
        ordered_entities = []
        while len(ordered_entities) < len(root_entities):
            for entity in entities:
                if not entity.registered:
                    if entity.is_root:
                        ordered_entities.append(entity.name)
                        entity.registered = True

        # order the entities from the most root entity to the most leaf entity. Rule : a entity must be registered before its parents. And append only in parents are already appended.
        while len(ordered_entities) < len(entities):
            for entity in entities:
                if not entity.registered:
                    # check if all parents are already registered. entity.parents giving string instead of EntityNode
                    if all([parent in ordered_entities for parent in entity.parents]):
                        ordered_entities.append(entity.name)
                        entity.registered = True

        # reverse the order
        ordered_entities.reverse()

        return ordered_entities

    # write persistence_registration.h file

    persistence_registration_template = template_env.get_template(
        "persistence_registration.h.jinja2"
    )
    relative_output_file = os.path.join(base_path, "persistence_registration.h")
    output_file = os.path.join(root_path, relative_output_file)

    if files_to_be_generated.get(relative_output_file, False):
        repositories = []
        for repository in all_repository_files:
            repository_header = f"{repository}"
            relative_path = os.path.relpath(
                repository_header, os.path.dirname(output_file)
            )
            repositories.append(relative_path.replace("\\", "/"))

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(output_file), exist_ok=True)

        rendered_template = persistence_registration_template.render(
            repositories=repositories,
            export=export,
            export_header_file=export_header_file,
            application_cpp_domain_name=application_cpp_domain_name,
        )

        with open(output_file, "w") as fh:
            fh.write(rendered_template)
            print(f"Successfully wrote file {output_file}")

        clang_format_runner.run_clang_format(output_file)

    # write persistence_registration.cpp file

    persistence_registration_template = template_env.get_template(
        "persistence_registration.cpp.jinja2"
    )
    relative_output_file = os.path.join(base_path, "persistence_registration.cpp")
    output_file = os.path.join(root_path, relative_output_file)

    if files_to_be_generated.get(relative_output_file, False):
        ordered_entities = determine_order_of_registering_of_entities(entities_list)

        # using the order given by ordered_entities, create a list of entities dict with entity_pascal_name, entity_camel_name and a list of children_entities, each having a child_camel_name
        entities = []
        for entity in ordered_entities:
            entity_pascal_name = stringcase.pascalcase(entity)
            entity_camel_name = stringcase.camelcase(entity)
            pre_children_entities = []
            for child in determine_direct_children(entities_by_name[entity]):
                if child not in pre_children_entities:
                    pre_children_entities.append(child)

            children_entities = []
            for child in pre_children_entities:
                children_entities.append(
                    {
                        "child_camel_name": stringcase.camelcase(child),
                    }
                )
            entities.append(
                {
                    "entity_pascal_name": entity_pascal_name,
                    "entity_camel_name": entity_camel_name,
                    "children_entities": children_entities,
                }
            )
        # sort entities by entity_pascal_name
        entities = sorted(entities, key=lambda k: k["entity_pascal_name"])

        repositories = []
        repository_headers = []
        for repository in all_repository_files:
            repository_header = f"{repository}"
            relative_path = os.path.relpath(
                repository_header, os.path.dirname(output_file)
            )
            repositories.append(relative_path.replace("\\", "/"))
            if relative_path.endswith(".h"):
                repository_headers.append(relative_path.replace("\\", "/"))

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(output_file), exist_ok=True)

        rendered_template = persistence_registration_template.render(
            repository_headers=repository_headers,
            entities=entities,
            application_cpp_domain_name=application_cpp_domain_name,
        )

        with open(output_file, "w") as fh:
            fh.write(rendered_template)
            print(f"Successfully wrote file {output_file}")

        clang_format_runner.run_clang_format(output_file)

    # write the CMakeLists.txt file

    _generate_cmakelists(
        root_path,
        base_path,
        application_name,
        files_to_be_generated,
    )

    # write the interface CMakeLists.txt file

    _generate_contracts_cmakelists(
        root_path,
        contracts_folder_path,
        application_name,
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

    base_folder_path = manifest["repositories"]["base_folder_path"]
    repository_folder_path = manifest["repositories"]["repository_folder_path"]
    contracts_folder_path = manifest["contracts"]["folder_path"]
    interface_folder_path = os.path.join(contracts_folder_path, "repository")
    global_data = manifest.get("global", {})

    # Get the list of files to be generated
    files = []
    for entity in manifest["repositories"]["list"]:
        entity_name = entity["entity_name"]
        files.append(
            os.path.join(
                repository_folder_path,
                f"{stringcase.snakecase(entity_name)}_repository.h",
            )
        )
        files.append(
            os.path.join(
                repository_folder_path,
                f"{stringcase.snakecase(entity_name)}_repository.cpp",
            )
        )
        files.append(
            os.path.join(
                interface_folder_path,
                f"interface_{stringcase.snakecase(entity_name)}_repository.h",
            )
        )

    # add repository cmake file:
    files.append(os.path.join(base_folder_path, "repositories.cmake"))

    # add persistence_registration file:
    files.append(os.path.join(base_folder_path, "persistence_registration.h"))
    files.append(os.path.join(base_folder_path, "persistence_registration.cpp"))

    # add CMakeLists.txt file:
    files.append(os.path.join(base_folder_path, "CMakeLists.txt"))

    # add contracts CMakeLists.txt file:
    files.append(os.path.join(contracts_folder_path, "CMakeLists.txt"))

    # add interface cmake file:
    files.append(
        os.path.join(
            contracts_folder_path,
            "repository_interfaces.cmake",
        )
    )

    # strip from files if the value in files_to_be_generated is False
    if files_to_be_generated:
        for path, generate in files_to_be_generated.items():
            if not generate and path in files:
                files.remove(path)

    return files


# generate the files into the preview folder
def preview_repository_files(
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
    manifest["repositories"]["base_folder_path"] = manifest["repositories"][
        "base_folder_path"
    ].replace("..", "")
    manifest["repositories"]["repository_folder_path"] = manifest["repositories"][
        "repository_folder_path"
    ].replace("..", "")

    # write the modified manifest file
    with open(manifest_preview_file, "w") as fh:
        yaml.dump(manifest, fh)

    root_path = os.path.join(root_path, "qleany_preview")

    # preprend preview/ to the file names in the dict files_to_be_generated and remove .. from the path
    if files_to_be_generated:
        preview_files_to_be_generated = {}
        for path, value in files_to_be_generated.items():
            preview_files_to_be_generated[path.replace("..", "")] = value

        generate_repository_files(
            root_path,
            manifest_preview_file,
            preview_files_to_be_generated,
            uncrustify_config_file,
        )
    else:
        generate_repository_files(
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
                preview_repository_files(root_path, manifest_file)
            else:
                generate_repository_files(root_path, manifest_file)
        else:
            print("Error: Manifest file must be named 'qleany.yaml' or 'qleany.yml'")
    else:
        print("Error: Please provide the manifest file as an argument")
