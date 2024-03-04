from jinja2 import Environment, FileSystemLoader
import yaml
import os
import sys
import stringcase
import shutil
import uncrustify
from pathlib import Path
import copy
import clang_format_runner as clang_format_runner
import generation_dict_tools as tools


def _get_generation_dict(
    folder_path: str,
    application_name: str,
    application_cpp_domain_name: str,
    feature_by_name: dict,
    entities_by_name: dict,
    singles_list: list,
    list_models_list: list,
    create_undo_and_redo_singles: bool,
) -> dict:

    generation_dict = {}

    generation_dict["folder_path"] = folder_path
    generation_dict["application_name"] = application_name
    generation_dict["application_spinalcase_name"] = stringcase.spinalcase(
        application_name
    )
    generation_dict["application_uppercase_name"] = application_name.upper()
    generation_dict["application_snakecase_name"] = stringcase.snakecase(
        application_name
    )
    generation_dict["application_cpp_domain_name"] = application_cpp_domain_name
    generation_dict["export_header_file"] = f"{stringcase.snakecase(application_name)}_presenter_export.h"
    generation_dict["export"] = f"{stringcase.snakecase(application_name).upper()}_PRESENTER_EXPORT"

    generation_dict["all_presenter_files"] = []
    generation_dict["singles_list"] = []

    for single in singles_list:
        read_only = single.get("read_only", False)

        entity_name = single["entity"]
        entity_name_snake = stringcase.snakecase(entity_name)
        entity_name_pascal = stringcase.pascalcase(entity_name)
        entity_name_spinal = stringcase.spinalcase(entity_name)
        entity_name_camel = stringcase.camelcase(entity_name)

        class_name = single["name"]
        if class_name == "auto":
            class_name = "Single" + entity_name_pascal

        class_name_snake = stringcase.snakecase(class_name)
        class_name_pascal = stringcase.pascalcase(class_name)
        class_name_spinal = stringcase.spinalcase(class_name)
        class_name_camel = stringcase.camelcase(class_name)

        generation_dict["singles_list"].append(
            {
                "entity_name_snake": entity_name_snake,
                "entity_name_pascal": entity_name_pascal,
                "entity_name_spinal": entity_name_spinal,
                "entity_name_camel": entity_name_camel,
                "class_name_snake": class_name_snake,
                "class_name_pascal": class_name_pascal,
                "class_name_spinal": class_name_spinal,
                "class_name_camel": class_name_camel,
                "fields": tools.get_fields_without_foreign_entities(
                    entities_by_name[single["entity"]]["fields"],
                    entities_by_name,
                    single["entity"],
                ),
                "read_only": read_only,
            }
        )
        generation_dict["all_presenter_files"].append(
            os.path.join(
                folder_path,
                f"{class_name_snake}.h",
            )
        )

        generation_dict["all_presenter_files"].append(
            os.path.join(
                folder_path,
                f"{class_name_snake}.cpp",
            )
        )

    # list models

    generation_dict["list_models"] = []

    for model in list_models_list:
        read_only = model.get("read_only", False)

        entity_name = model["entity"]
        entity_name_snake = stringcase.snakecase(entity_name)
        entity_name_pascal = stringcase.pascalcase(entity_name)
        entity_name_spinal = stringcase.spinalcase(entity_name)
        entity_name_camel = stringcase.camelcase(entity_name)

        # displayed_field
        displayed_field = model.get("displayed_field", "id")
        displayed_field_snake = stringcase.snakecase(displayed_field)
        displayed_field_pascal = stringcase.pascalcase(displayed_field)
        displayed_field_spinal = stringcase.spinalcase(displayed_field)
        displayed_field_camel = stringcase.camelcase(displayed_field)

        related_name = model.get("in_relation_of", "")
        related_name_snake = stringcase.snakecase(related_name)
        related_name_pascal = stringcase.pascalcase(related_name)
        related_name_spinal = stringcase.spinalcase(related_name)
        related_name_camel = stringcase.camelcase(related_name)

        related_field_name = model.get("relation_field_name", "")
        related_field_name_snake = stringcase.snakecase(related_field_name)
        related_field_name_pascal = stringcase.pascalcase(related_field_name)
        related_field_name_spinal = stringcase.spinalcase(related_field_name)
        related_field_name_camel = stringcase.camelcase(related_field_name)

        is_related_list = related_name != "" and related_field_name != ""

        related_fields = (
            tools.get_entity_fields(related_name, entities_by_name) if is_related_list else []
        )
        is_ordered_list = False
        for field in related_fields:
            if field["name"] == related_field_name:
                is_ordered_list = field.get("ordered", False)
                break

        class_name = model["name"]
        if class_name == "auto":
            if is_related_list:
                class_name = (
                    entity_name_pascal
                    + "ListModelFrom"
                    + related_name_pascal
                    + related_field_name_pascal
                )
            else:
                class_name = entity_name_pascal + "ListModel"

        class_name_snake = stringcase.snakecase(class_name)
        class_name_pascal = stringcase.pascalcase(class_name)
        class_name_spinal = stringcase.spinalcase(class_name)
        class_name_camel = stringcase.camelcase(class_name)

        generation_dict["list_models"].append(
            {
                "entity_name_snake": entity_name_snake,
                "entity_name_pascal": entity_name_pascal,
                "entity_name_spinal": entity_name_spinal,
                "entity_name_camel": entity_name_camel,
                "displayed_field_snake": displayed_field_snake,
                "displayed_field_pascal": displayed_field_pascal,
                "displayed_field_spinal": displayed_field_spinal,
                "displayed_field_camel": displayed_field_camel,
                "related_name_snake": related_name_snake,
                "related_name_pascal": related_name_pascal,
                "related_name_spinal": related_name_spinal,
                "related_name_camel": related_name_camel,
                "related_field_name_snake": related_field_name_snake,
                "related_field_name_pascal": related_field_name_pascal,
                "related_field_name_spinal": related_field_name_spinal,
                "related_field_name_camel": related_field_name_camel,
                "class_name_snake": class_name_snake,
                "class_name_pascal": class_name_pascal,
                "class_name_spinal": class_name_spinal,
                "class_name_camel": class_name_camel,
                "fields": tools.get_fields_without_foreign_entities(
                    entities_by_name[model["entity"]]["fields"],
                    entities_by_name,
                    model["entity"],
                ),
                "is_ordered_list": is_ordered_list,
                "is_related_list": is_related_list,
                "read_only": read_only,

            }
        )
        generation_dict["all_presenter_files"].append(
            os.path.join(
                folder_path,
                f"{class_name_snake}.h",
            )
        )

        generation_dict["all_presenter_files"].append(
            os.path.join(
                folder_path,
                f"{class_name_snake}.cpp",
            )
        )

    # add undo redo
    generation_dict["create_undo_and_redo_singles"] = create_undo_and_redo_singles
    if create_undo_and_redo_singles:
        generation_dict["all_presenter_files"].append(
            os.path.join(
                folder_path,
                "single_undo.h",
            )
        )

        generation_dict["all_presenter_files"].append(
            os.path.join(
                folder_path,
                "single_undo.cpp",
            )
        )

        generation_dict["all_presenter_files"].append(
            os.path.join(
                folder_path,
                "single_redo.h",
            )
        )

        generation_dict["all_presenter_files"].append(
            os.path.join(
                folder_path,
                "single_redo.cpp",
            )
        )

    return generation_dict


def _generate_cmakelists(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool] = None
):
    template_env = Environment(loader=FileSystemLoader("templates/presenter"))
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
                    export_header_file=generation_dict["export_header_file"],
                    application_spinalcase_name=generation_dict[
                        "application_spinalcase_name"
                    ],
                    application_uppercase_name=generation_dict[
                        "application_uppercase_name"
                    ],
                    application_snakecase_name=generation_dict[
                        "application_snakecase_name"
                    ]

                )
            )
            print(f"Successfully wrote file {cmakelists_file}")


def _generate_cmake_file(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool] = None
):
    template_env = Environment(loader=FileSystemLoader("templates/presenter"))
    template = template_env.get_template("presenters.cmake.jinja2")

    folder_path = generation_dict["folder_path"]
    all_presenter_files = generation_dict["all_presenter_files"]

    relative_cmake_file = os.path.join(folder_path, "presenters.cmake")
    cmake_file = os.path.join(root_path, relative_cmake_file)

    # write the presenter cmake list file

    if files_to_be_generated.get(relative_cmake_file, False):
        presenter_files = []
        for presenter_file in all_presenter_files:
            relative_path = os.path.relpath(
                os.path.join(root_path, presenter_file), os.path.dirname(cmake_file)
            )
            presenter_files.append(relative_path.replace("\\", "/"))

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(cmake_file), exist_ok=True)

        rendered_template = template.render(
            presenter_files=presenter_files,
        )

        with open(cmake_file, "w") as fh:
            fh.write(rendered_template)
            print(f"Successfully wrote file {cmake_file}")


def _generate_single_file(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool] = None
):
    template_env = Environment(loader=FileSystemLoader("templates/presenter"))
    h_template = template_env.get_template("single.h.jinja2")
    cpp_template = template_env.get_template("single.cpp.jinja2")

    folder_path = generation_dict["folder_path"]
    singles_list = generation_dict["singles_list"]

    for single in singles_list:
        h_relative_single_file = os.path.join(
            folder_path, f"{single['class_name_snake']}.h"
        )
        h_single_file = os.path.join(root_path, h_relative_single_file)

        if files_to_be_generated.get(h_relative_single_file, False):
            # Create the directory if it does not exist
            os.makedirs(os.path.dirname(h_single_file), exist_ok=True)

            with open(h_single_file, "w") as f:
                f.write(
                    h_template.render(
                        single=single,
                        application_cpp_domain_name=generation_dict[
                            "application_cpp_domain_name"
                        ],
                        export_header_file=generation_dict["export_header_file"],
                        export=generation_dict["export"],
                    )
                )
                print(f"Successfully wrote file {h_single_file}")

        cpp_relative_single_file = os.path.join(
            folder_path, f"{single['class_name_snake']}.cpp"
        )
        cpp_single_file = os.path.join(root_path, cpp_relative_single_file)

        if files_to_be_generated.get(cpp_relative_single_file, False):
            # Create the directory if it does not exist
            os.makedirs(os.path.dirname(cpp_single_file), exist_ok=True)

            with open(cpp_single_file, "w") as f:
                f.write(
                    cpp_template.render(
                        single=single,
                        application_cpp_domain_name=generation_dict[
                            "application_cpp_domain_name"
                        ],
                    )
                )
                print(f"Successfully wrote file {cpp_single_file}")


def _generate_undo_single_files(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool] = None
):
    template_env = Environment(loader=FileSystemLoader("templates/presenter"))
    h_template = template_env.get_template("single_undo.h.jinja2")
    cpp_template = template_env.get_template("single_undo.cpp.jinja2")

    folder_path = generation_dict["folder_path"]

    h_relative_single_file = os.path.join(folder_path, "single_undo.h")
    h_single_file = os.path.join(root_path, h_relative_single_file)

    if files_to_be_generated.get(h_relative_single_file, False):
        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(h_single_file), exist_ok=True)

        with open(h_single_file, "w") as f:
            f.write(
                h_template.render(
                    application_cpp_domain_name=generation_dict[
                        "application_cpp_domain_name"
                    ],
                    export_header_file=generation_dict["export_header_file"],
                    export=generation_dict["export"],
                )
            )
            print(f"Successfully wrote file {h_single_file}")

    cpp_relative_single_file = os.path.join(folder_path, "single_undo.cpp")
    cpp_single_file = os.path.join(root_path, cpp_relative_single_file)

    if files_to_be_generated.get(cpp_relative_single_file, False):
        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(cpp_single_file), exist_ok=True)

        with open(cpp_single_file, "w") as f:
            f.write(
                cpp_template.render(
                    application_cpp_domain_name=generation_dict[
                        "application_cpp_domain_name"
                    ],
                )
            )
            print(f"Successfully wrote file {cpp_single_file}")


def _generate_redo_single_files(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool] = None
):
    template_env = Environment(loader=FileSystemLoader("templates/presenter"))
    h_template = template_env.get_template("single_redo.h.jinja2")
    cpp_template = template_env.get_template("single_redo.cpp.jinja2")

    folder_path = generation_dict["folder_path"]

    h_relative_single_file = os.path.join(folder_path, "single_redo.h")
    h_single_file = os.path.join(root_path, h_relative_single_file)

    if files_to_be_generated.get(h_relative_single_file, False):
        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(h_single_file), exist_ok=True)

        with open(h_single_file, "w") as f:
            f.write(
                h_template.render(
                    application_cpp_domain_name=generation_dict[
                        "application_cpp_domain_name"
                    ],
                    export_header_file=generation_dict["export_header_file"],
                    export=generation_dict["export"],
                )
            )
            print(f"Successfully wrote file {h_single_file}")

    cpp_relative_single_file = os.path.join(folder_path, "single_redo.cpp")
    cpp_single_file = os.path.join(root_path, cpp_relative_single_file)

    if files_to_be_generated.get(cpp_relative_single_file, False):
        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(cpp_single_file), exist_ok=True)

        with open(cpp_single_file, "w") as f:
            f.write(
                cpp_template.render(
                    application_cpp_domain_name=generation_dict[
                        "application_cpp_domain_name"
                    ],
                )
            )
            print(f"Successfully wrote file {cpp_single_file}")


def _generate_list_model_file(
    root_path: str, generation_dict: dict, files_to_be_generated: dict[str, bool] = None
):
    template_env = Environment(loader=FileSystemLoader("templates/presenter"))
    not_ordered_h_template = template_env.get_template("list_model.h.jinja2")
    not_ordered_cpp_template = template_env.get_template("list_model.cpp.jinja2")
    ordered_h_template = template_env.get_template("ordered_list_model.h.jinja2")
    ordered_cpp_template = template_env.get_template("ordered_list_model.cpp.jinja2")
    not_related_h_template = template_env.get_template("entity_list_model.h.jinja2")
    not_related_cpp_template = template_env.get_template("entity_list_model.cpp.jinja2")

    folder_path = generation_dict["folder_path"]
    list_models = generation_dict["list_models"]

    for list_model in list_models:
        h_template = (
            ordered_h_template
            if list_model["is_ordered_list"]
            else not_ordered_h_template
        )
        cpp_template = (
            ordered_cpp_template
            if list_model["is_ordered_list"]
            else not_ordered_cpp_template
        )

        h_template = (
            h_template if list_model["is_related_list"] else not_related_h_template
        )
        cpp_template = (
            cpp_template if list_model["is_related_list"] else not_related_cpp_template
        )

        h_relative_list_model_file = os.path.join(
            folder_path, f"{list_model['class_name_snake']}.h"
        )
        h_list_model_file = os.path.join(root_path, h_relative_list_model_file)

        if files_to_be_generated.get(h_relative_list_model_file, False):
            # Create the directory if it does not exist
            os.makedirs(os.path.dirname(h_list_model_file), exist_ok=True)

            with open(h_list_model_file, "w") as f:
                f.write(
                    h_template.render(
                        model=list_model,
                        application_cpp_domain_name=generation_dict[
                            "application_cpp_domain_name"
                        ],
                        export_header_file=generation_dict["export_header_file"],
                        export=generation_dict["export"],
                    )
                )
                print(f"Successfully wrote file {h_list_model_file}")

        cpp_relative_list_model_file = os.path.join(
            folder_path, f"{list_model['class_name_snake']}.cpp"
        )
        cpp_list_model_file = os.path.join(root_path, cpp_relative_list_model_file)

        if files_to_be_generated.get(cpp_relative_list_model_file, False):
            # Create the directory if it does not exist
            os.makedirs(os.path.dirname(cpp_list_model_file), exist_ok=True)

            with open(cpp_list_model_file, "w") as f:
                f.write(
                    cpp_template.render(
                        model=list_model,
                        application_cpp_domain_name=generation_dict[
                            "application_cpp_domain_name"
                        ],
                    )
                )
                print(f"Successfully wrote file {cpp_list_model_file}")


def generate_presenter_files(
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

    application_data = manifest_data.get("application", [])
    feature_list = application_data.get("features", [])

    # Organize feature_list by name for easier lookup
    feature_by_name = {feature["name"]: feature for feature in feature_list}

    global_data = manifest_data.get("global", [])
    application_cpp_domain_name = global_data.get(
        "application_cpp_domain_name", "Undefined"
    )

    entities_data = manifest_data.get("entities", [])
    entities_list = entities_data.get("list", [])
    # remove entities that are not to be generated
    entities_list = [entity for entity in entities_list]

    # Organize entities by name for easier lookup
    entities_by_name = {entity["name"]: entity for entity in entities_list}

    presenter_data = manifest_data.get("presenter", {})
    singles_presenter_list = presenter_data.get("singles", [])
    list_models_list = presenter_data.get("list_models", [])
    create_undo_and_redo_singles = presenter_data.get(
        "create_undo_and_redo_singles", False
    )

    folder_path = presenter_data.get("folder_path", "Undefined")

    generation_dict = _get_generation_dict(
        folder_path,
        application_name,
        application_cpp_domain_name,
        feature_by_name,
        entities_by_name,
        singles_presenter_list,
        list_models_list,
        create_undo_and_redo_singles,
    )

    _generate_single_file(root_path, generation_dict, files_to_be_generated)
    _generate_list_model_file(root_path, generation_dict, files_to_be_generated)
    if create_undo_and_redo_singles:
        _generate_undo_single_files(root_path, generation_dict, files_to_be_generated)
        _generate_redo_single_files(root_path, generation_dict, files_to_be_generated)
    _generate_cmake_file(root_path, generation_dict, files_to_be_generated)
    _generate_cmakelists(root_path, generation_dict, files_to_be_generated)

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
    with open(manifest_file, "r") as stream:
        try:
            manifest_data = yaml.safe_load(stream)
        except yaml.YAMLError as exc:
            print(exc)
            return

    application_name = manifest_data.get("global", {}).get(
        "application_name", "example"
    )

    application_data = manifest_data.get("application", [])
    feature_list = application_data.get("features", [])

    # Organize feature_list by name for easier lookup
    feature_by_name = {feature["name"]: feature for feature in feature_list}

    global_data = manifest_data.get("global", [])
    application_cpp_domain_name = global_data.get(
        "application_cpp_domain_name", "Undefined"
    )

    entities_data = manifest_data.get("entities", [])
    entities_list = entities_data.get("list", [])
    # remove entities that are not to be generated
    entities_list = [entity for entity in entities_list]

    # Organize entities by name for easier lookup
    entities_by_name = {entity["name"]: entity for entity in entities_list}

    presenter_data = manifest_data.get("presenter", {})
    singles_presenter_list = presenter_data.get("singles", [])
    list_models_list = presenter_data.get("list_models", [])
    create_undo_and_redo_singles = presenter_data.get(
        "create_undo_and_redo_singles", False
    )

    folder_path = presenter_data.get("folder_path", "Undefined")

    generation_dict = _get_generation_dict(
        folder_path,
        application_name,
        application_cpp_domain_name,
        feature_by_name,
        entities_by_name,
        singles_presenter_list,
        list_models_list,
        create_undo_and_redo_singles,
    )

    files = []

    # add presenter files:
    for presenter_file in generation_dict["all_presenter_files"]:
        files.append(presenter_file)

    # add list_file:
    files.append(
        os.path.join(
            folder_path,
            "presenters.cmake",
        )
    )

    # add CMakeLists.txt
    files.append(
        os.path.join(
            folder_path,
            "CMakeLists.txt",
        )
    )

    # strip from files if the value in files_to_be_generated is False
    if files_to_be_generated:
        for path, generate in files_to_be_generated.items():
            if not generate and path in files:
                files.remove(path)

    return files


# generate the files into the preview folder
def preview_presenter_files(
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
    manifest["presenter"]["folder_path"] = manifest["presenter"]["folder_path"].replace(
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

        generate_presenter_files(
            root_path,
            manifest_preview_file,
            preview_files_to_be_generated,
            uncrustify_config_file,
        )

    else:
        generate_presenter_files(
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
                preview_presenter_files(root_path, manifest_file)
            else:
                generate_presenter_files(root_path, manifest_file)
        else:
            print("Error: Manifest file must be named 'qleany.yaml' or 'qleany.yml'")
    else:
        print("Error: Please provide the manifest file as an argument")
