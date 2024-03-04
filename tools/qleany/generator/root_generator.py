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

def _get_generation_dict(manifest_data) -> dict:
    generation_dict = {}

    cmakelists_dict = {}

    cmakelists_dict["application_name"] = manifest_data.get("global", {}).get(
        "application_name", "example"
    )

    # get the paths from the manifest file
    cmakelists_dict["entities_path"] = manifest_data.get("entities", {}).get("folder_path", "src/entities")
    cmakelists_dict["contracts_path"] = manifest_data.get("contracts", {}).get("folder_path", "src/contracts")
    cmakelists_dict["persistence_path"] = manifest_data.get("repositories", {}).get("base_folder_path", "src/persistence")
    cmakelists_dict["contracts_dto_path"] = manifest_data.get("DTOs", {}).get("common_cmake_folder_path", "src/contracts.dto")
    cmakelists_dict["contracts_cqrs_path"] = manifest_data.get("CQRS", {}).get("common_cmake_folder_path", "src/contracts.cqrs")
    cmakelists_dict["application_path"] = manifest_data.get("application", {}).get("common_cmake_folder_path", "src/application")
    cmakelists_dict["interactor_path"] = manifest_data.get("interactor", {}).get("folder_path", "src/interactor")
    cmakelists_dict["presenter_path"] = manifest_data.get("presenter", {}).get("folder_path", "src/presenters")

    # get the front ends
    front_ends = manifest_data.get("front_ends", {})
    front_end_dict = {}

    front_end_dict["qt_widgets"] = front_ends.get("qt_widgets", {})
    front_end_dict["qt_widgets"]["enabled"] = True if front_end_dict["qt_widgets"] else False
    if front_end_dict["qt_widgets"]["enabled"]:
        assert front_end_dict["qt_widgets"]["folder_path"], "qt_widgets folder_path is not set in the manifest file"

    front_end_dict["qt_quick"] = front_ends.get("qt_quick", {})
    front_end_dict["qt_quick"]["enabled"] = True if front_end_dict["qt_quick"] else False
    if front_end_dict["qt_quick"]["enabled"]:
        assert front_end_dict["qt_quick"]["folder_path"], "qt_quick folder_path is not set in the manifest file"

    front_end_dict["kf6_kirigami"] = front_ends.get("kf6_kirigami", {})
    front_end_dict["kf6_kirigami"]["enabled"] = True if front_end_dict["kf6_kirigami"] else False
    if front_end_dict["kf6_kirigami"]["enabled"]:
        assert front_end_dict["kf6_kirigami"]["folder_path"], "kf6_kirigami folder_path is not set in the manifest file"

    front_end_dict["kf6_widgets"] = front_ends.get("kf6_widgets", {})
    front_end_dict["kf6_widgets"]["enabled"] = True if front_end_dict["kf6_widgets"] else False
    if front_end_dict["kf6_widgets"]["enabled"]:
        assert front_end_dict["kf6_widgets"]["folder_path"], "kf6_widgets folder_path is not set in the manifest file"
    
    cmakelists_dict["front_ends"] = front_end_dict

    generation_dict["cmakelists"] = cmakelists_dict

    return generation_dict

def _generate_cmakelists_file(
    root_path: str, cmakelists_dict: dict, files_to_be_generated: dict[str, bool]
):
    cmakelists_file = "CMakeLists.txt"
    
    # Create the jinja2 environment
    template_env = Environment(loader=FileSystemLoader("templates/root"))
    # Load the template
    cmakelists_template = template_env.get_template("CMakeLists.txt.jinja2")

    # generate the real cmakelists file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(cmakelists_file, False):
        return

    output_file = os.path.join(root_path, cmakelists_file)

    # Render the template
    output = cmakelists_template.render(
        application_name=cmakelists_dict["application_name"],
        entities_path=cmakelists_dict["entities_path"],
        contracts_path=cmakelists_dict["contracts_path"],
        persistence_path=cmakelists_dict["persistence_path"],
        contracts_dto_path=cmakelists_dict["contracts_dto_path"],
        contracts_cqrs_path=cmakelists_dict["contracts_cqrs_path"],
        application_path=cmakelists_dict["application_path"],
        interactor_path=cmakelists_dict["interactor_path"],
        presenter_path=cmakelists_dict["presenter_path"],
        front_ends=cmakelists_dict["front_ends"],
    )

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Write the output to the file
    with open(output_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {output_file}")


def generate_root_files(
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

    generation_dict = _get_generation_dict(manifest_data)


    # generate the CMakeLists.txt file
    _generate_cmakelists_file(root_path, generation_dict["cmakelists"], files_to_be_generated)



def get_files_to_be_generated(
    manifest_file: str, files_to_be_generated: dict[str, bool] = {}
) -> list[str]:
    """
    Get the list of files that need to be generated based on the manifest file
    """

    files = []
    files.append("CMakeLists.txt")

    # strip from files if the value in files_to_be_generated is False
    if files_to_be_generated:
        for path, generate in files_to_be_generated.items():
            if not generate and path in files:
                files.remove(path)

    return files


# generate the files into the preview folder
def preview_root_files(
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

    # write the modified manifest file
    with open(manifest_preview_file, "w") as fh:
        yaml.dump(manifest, fh)

    root_path = os.path.join(root_path, "qleany_preview")

    # remove .. from the path
    if files_to_be_generated:
        preview_files_to_be_generated = {}
        for path, value in files_to_be_generated.items():
            preview_files_to_be_generated[path.replace("..", "")] = value

        generate_root_files(
            root_path,
            manifest_preview_file,
            preview_files_to_be_generated,
            uncrustify_config_file,
        )

    else:
        generate_root_files(root_path, manifest_preview_file, {}, uncrustify_config_file)
        

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
                preview_root_files(root_path, manifest_file)
            else:
                generate_root_files(root_path, manifest_file)
        else:
            print("Error: Manifest file must be named 'qleany.yaml' or 'qleany.yml'")
    else:
        print("Error: Please provide the manifest file as an argument")
