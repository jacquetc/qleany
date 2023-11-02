from jinja2 import Environment, FileSystemLoader
import yaml
import os
import sys
import stringcase
import shutil
import uncrustify
from pathlib import Path



def get_generation_dict(
    folder_path: str,
    application_name: str,
    application_cpp_domain_name: str,
    feature_by_name: dict,
    entities_by_name: dict,
    singles_list: list,
    list_models_list: list,
    export: str,
    export_header_file: str,
) -> dict:
    generation_dict = {}
    for feature_name, feature in feature_by_name.items():
        feature_snake_name = stringcase.snakecase(feature_name)
        feature_pascal_name = stringcase.pascalcase(feature_name)
        feature_spinal_name = stringcase.spinalcase(feature_name)
        feature_camel_name = stringcase.camelcase(feature_name)
        generation_dict[feature_pascal_name] = {
            "feature_snake_name": feature_snake_name,
            "feature_pascal_name": feature_pascal_name,
            "feature_spinal_name": feature_spinal_name,
            "feature_camel_name": feature_camel_name,
        }
        # add export_header
        export_header = f"application_{feature_snake_name}_export.h"
        generation_dict[feature_pascal_name]["export_header"] = export_header
        generation_dict[feature_pascal_name]["export_header_file"] = os.path.join(
            common_cmake_folder_patholder_path,
            feature_snake_name + "_feature",
            export_header,
        )
        generation_dict[feature_pascal_name][
            "export"
        ] = f"{stringcase.uppercase(application_name)}_APPLICATION_{stringcase.uppercase(feature_snake_name)}_EXPORT"


def generate_cmakelists(
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

    folder_path = presenter_data.get("folder_path", "Undefined")
    export = presenter_data.get("export", "Undefined")
    export_header_file = presenter_data.get("export_header_file", "Undefined")

    generation_dict = get_generation_dict(
        folder_path,
        application_name,
        application_cpp_domain_name,
        feature_by_name,
        entities_by_name,
        singles_presenter_list,
        list_models_list,
        export,
        export_header_file,
    )


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
        manifest = yaml.safe_load(fh)

    folder_path = manifest["presenter"]["folder_path"]

    # Get the list of files to be generated
    files = []
    for presenter in manifest["presenter"]["list"]:
        presenter_name = presenter["name"]
        files.append(
            os.path.join(
                folder_path,
                f"{stringcase.snakecase(presenter_name)}_presenter.h",
            )
        )
        files.append(
            os.path.join(
                folder_path,
                f"{stringcase.snakecase(presenter_name)}_presenter.cpp",
            )
        )

    # add list_file:
    files.append(
        os.path.join(
            folder_path,
            "presenters.cmake",
        )
    )

    # strip from files if the value in files_to_be_generated is False
    if files_to_be_generated:
        for path, generate in files_to_be_generated.items():
            if not generate:
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
    manifest["presenter"]["folder_path"] = manifest["presenter"][
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
