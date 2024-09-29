from jinja2 import Environment, FileSystemLoader
import yaml
import os
import sys
import stringcase
import shutil
import uncrustify
import clang_format_runner as clang_format_runner
import generation_dict_tools as tools
import qml_imports_generator
from pathlib import Path


def _get_generation_dict(manifest_data) -> dict:
    generation_dict = {}

    cmakelists_dict = {}

    application_name = manifest_data.get("global", {}).get(
        "application_name", "example"
    )
    application_name_lower = application_name.lower().strip("_- ")
    application_cpp_domain_name = manifest_data.get("global", {}).get(
        "application_cpp_domain_name", "example"
    )

    # CMakeLists.txt

    cmakelists_dict["application_name"] = application_name
    cmakelists_dict["application_name_lower"] = application_name_lower
    cmakelists_dict["application_name_spinal"] = stringcase.spinalcase(application_name)
    common_data = manifest_data.get("common", {})

    folder_path = common_data["folder_path"]

    cmakelists_dict["folder_path"] = folder_path
    generation_dict["cmakelists"] = cmakelists_dict

    # common files dict

    common_files_dict = {}
    common_files_dict["folder_path"] = folder_path
    common_files_dict["application_name"] = application_name
    common_files_dict["application_name_lower"] = application_name_lower
    common_files_dict["application_name_pascal"] = stringcase.pascalcase(
        application_name
    )
    common_files_dict["application_cpp_domain_name"] = application_cpp_domain_name

    generation_dict["common_files"] = common_files_dict

    return generation_dict


def _generate_common_files(
    root_path: str,
    common_files_dict: dict,
    files_to_be_generated: dict[str, bool],
):
    common_files = ["result.h", "error.h"]

    # Create the jinja2 environment
    template_env = Environment(loader=FileSystemLoader("templates/common"))

    for file in common_files:
        # Load the template
        template = template_env.get_template(file + ".jinja2")

        # generate the real file if in the files_to_be_generated dict the value is True
        if not files_to_be_generated.get(
            os.path.join(common_files_dict["folder_path"], file), False
        ):
            continue

        output_file = os.path.join(root_path, common_files_dict["folder_path"], file)

        # Render the template
        output = template.render(
            application_cpp_domain_name=common_files_dict["application_cpp_domain_name"]
        )

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(output_file), exist_ok=True)

        # Write the output to the file
        with open(output_file, "w") as fh:
            fh.write(output)

        print(f"Successfully wrote file {output_file}")


def _generate_automapper_files(
    root_path: str,
    common_files_dict: dict,
    files_to_be_generated: dict[str, bool],
):
    common_files = ["automapper.h", "automapper.cpp"]

    # Create the jinja2 environment
    template_env = Environment(loader=FileSystemLoader("templates/common/tools"))
    tools_path = os.path.join(common_files_dict["folder_path"], "tools")

    for file in common_files:
        # Load the template
        template = template_env.get_template(file + ".jinja2")

        # generate the real file if in the files_to_be_generated dict the value is True
        if not files_to_be_generated.get(os.path.join(tools_path, file), False):
            continue

        output_file = os.path.join(root_path, tools_path, file)

        # Render the template
        output = template.render(
            application_cpp_domain_name=common_files_dict["application_cpp_domain_name"]
        )

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(output_file), exist_ok=True)

        # Write the output to the file
        with open(output_file, "w") as fh:
            fh.write(output)

        print(f"Successfully wrote file {output_file}")


def _generate_cmakelists_file(
    root_path: str, cmakelists_dict: dict, files_to_be_generated: dict[str, bool]
):
    cmakelists_file = os.path.join(cmakelists_dict["folder_path"], "CMakeLists.txt")

    # Create the jinja2 environment
    template_env = Environment(loader=FileSystemLoader("templates/common"))
    # Load the template
    cmakelists_template = template_env.get_template("CMakeLists.txt.jinja2")

    # generate the real cmakelists file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(cmakelists_file, False):
        return

    output_file = os.path.join(root_path, cmakelists_file)

    # Render the template
    output = cmakelists_template.render(
        application_name=cmakelists_dict["application_name"],
        application_name_lower=cmakelists_dict["application_name_lower"],
        application_name_spinal=cmakelists_dict["application_name_spinal"],
        application_name_pascal=stringcase.pascalcase(
            cmakelists_dict["application_name"]
        ),
    )

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Write the output to the file
    with open(output_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {output_file}")


def generate_common_files(
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
    _generate_cmakelists_file(
        root_path, generation_dict["cmakelists"], files_to_be_generated
    )

    # generate the common files
    _generate_common_files(
        root_path, generation_dict["common_files"], files_to_be_generated
    )

    # generate the automapper files
    _generate_automapper_files(
        root_path, generation_dict["common_files"], files_to_be_generated
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

    generation_dict = _get_generation_dict(manifest_data)

    files = []
    folder_path = generation_dict["cmakelists"]["folder_path"]
    files.append(os.path.join(folder_path, "CMakeLists.txt"))
    folder_path = generation_dict["common_files"]["folder_path"]
    files.append(os.path.join(folder_path, "result.h"))
    files.append(os.path.join(folder_path, "error.h"))
    files.append(os.path.join(folder_path, "tools/automapper.h"))
    files.append(os.path.join(folder_path, "tools/automapper.cpp"))

    # strip from files if the value in files_to_be_generated is False
    if files_to_be_generated:
        for path, generate in files_to_be_generated.items():
            if not generate and path in files:
                files.remove(path)

    return files


# generate the files into the preview folder
def preview_common_files(
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

        generate_common_files(
            root_path,
            manifest_preview_file,
            preview_files_to_be_generated,
            uncrustify_config_file,
        )

    else:
        generate_common_files(
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
                preview_common_files(root_path, manifest_file)
            else:
                generate_common_files(root_path, manifest_file)
        else:
            print("Error: Manifest file must be named 'qleany.yaml' or 'qleany.yml'")
    else:
        print("Error: Please provide the manifest file as an argument")
