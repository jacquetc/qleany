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

    application_name = manifest_data.get("global", {}).get("application_name", "example")
    application_cpp_domain_name = manifest_data.get("global", {}).get("application_cpp_domain_name", "example")

    # CMakeLists.txt

    cmakelists_dict["application_name"] = application_name

    front_ends = manifest_data.get("front_ends", {})
    front_end_dict = {}

    front_end_dict["qt_widgets"] = front_ends.get("qt_widgets", {})
    if front_end_dict["qt_widgets"]:
        assert front_end_dict["qt_widgets"]["folder_path"], "qt_widgets folder_path is not set in the manifest file"
    folder_path = front_end_dict["qt_widgets"]["folder_path"]

    cmakelists_dict["folder_path"] = folder_path
    generation_dict["cmakelists"] = cmakelists_dict

    # main.cpp

    main_cpp_dict = {}
    main_cpp_dict["folder_path"] = folder_path
    main_cpp_dict["application_name"] = application_name
    main_cpp_dict["application_cpp_domain_name"] = application_cpp_domain_name
    main_cpp_dict["organisation_domain"] = manifest_data.get("global", {}).get("organisation", {}).get("domain", "example.com")
    main_cpp_dict["organisation_name"] = manifest_data.get("global", {}).get("organisation", {}).get("name", "example")

    generation_dict["main_cpp"] = main_cpp_dict

    # mainwindow files
    mainwindow_dict = {}
    mainwindow_dict["folder_path"] = folder_path
    mainwindow_dict["application_name"] = application_name
    mainwindow_dict["application_cpp_domain_name"] = application_cpp_domain_name

    generation_dict["mainwindow"] = mainwindow_dict

    return generation_dict


def _generate_cmakelists_file(
    root_path: str, cmakelists_dict: dict, files_to_be_generated: dict[str, bool]
):
    cmakelists_file = os.path.join(cmakelists_dict["folder_path"], "CMakeLists.txt")
    
    # Create the jinja2 environment
    template_env = Environment(loader=FileSystemLoader("templates/UIs/qt_widgets"))
    # Load the template
    cmakelists_template = template_env.get_template("CMakeLists.txt.jinja2")

    # generate the real cmakelists file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(cmakelists_file, False):
        return

    output_file = os.path.join(root_path, cmakelists_file)

    # Render the template
    output = cmakelists_template.render(
        application_name=cmakelists_dict["application_name"],
        application_spinal_name=stringcase.spinalcase(cmakelists_dict["application_name"]),
        application_pascal_name=stringcase.pascalcase(cmakelists_dict["application_name"]),
    )

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Write the output to the file
    with open(output_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {output_file}")

def _generate_main_cpp_file(
    root_path: str,
    main_cpp_dict: dict,
    files_to_be_generated: dict[str, bool]
):
    main_cpp_file = os.path.join(main_cpp_dict["folder_path"], "main.cpp")
    
    # Create the jinja2 environment
    template_env = Environment(loader=FileSystemLoader("templates/UIs/qt_widgets"))
    # Load the template
    main_cpp_template = template_env.get_template("main.cpp.jinja2")

    # generate the real main.cpp file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(main_cpp_file, False):
        return

    output_file = os.path.join(root_path, main_cpp_file)

    # Render the template
    output = main_cpp_template.render(
        application_name=main_cpp_dict["application_name"],
        application_cpp_domain_name=main_cpp_dict["application_cpp_domain_name"],
        organisation_domain=main_cpp_dict["organisation_domain"],
        organisation_name=main_cpp_dict["organisation_name"],
    )

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Write the output to the file
    with open(output_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {output_file}")

def _generate_mainwindow_files(
    root_path: str,
    mainwindow_dict: dict,
    files_to_be_generated: dict[str, bool]
):
    mainwindow_h_file = os.path.join(mainwindow_dict["folder_path"], "mainwindow.h")
    mainwindow_cpp_file = os.path.join(mainwindow_dict["folder_path"], "mainwindow.cpp")
    mainwindow_ui_file = os.path.join(mainwindow_dict["folder_path"], "mainwindow.ui")
    
    # Create the jinja2 environment
    template_env = Environment(loader=FileSystemLoader("templates/UIs/qt_widgets"))
    # Load the template
    mainwindow_h_template = template_env.get_template("mainwindow.h.jinja2")
    mainwindow_cpp_template = template_env.get_template("mainwindow.cpp.jinja2")
    mainwindow_ui_template = template_env.get_template("mainwindow.ui.jinja2")

    # generate the real mainwindow files if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(mainwindow_h_file, False):
        return
    if not files_to_be_generated.get(mainwindow_cpp_file, False):
        return
    if not files_to_be_generated.get(mainwindow_ui_file, False):
        return

    output_h_file = os.path.join(root_path, mainwindow_h_file)
    output_cpp_file = os.path.join(root_path, mainwindow_cpp_file)
    output_ui_file = os.path.join(root_path, mainwindow_ui_file)

    # Render the template
    output_h = mainwindow_h_template.render(
        application_name=mainwindow_dict["application_name"],
        application_pascal_name=stringcase.pascalcase(mainwindow_dict["application_name"]),
        application_cpp_domain_name=mainwindow_dict["application_cpp_domain_name"],
    )
    output_cpp = mainwindow_cpp_template.render(
        application_name=mainwindow_dict["application_name"],
        application_pascal_name=stringcase.pascalcase(mainwindow_dict["application_name"]),
        application_cpp_domain_name=mainwindow_dict["application_cpp_domain_name"],
    )
    output_ui = mainwindow_ui_template.render(
        application_name=mainwindow_dict["application_name"],
        application_pascal_name=stringcase.pascalcase(mainwindow_dict["application_name"]),
    )

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_h_file), exist_ok=True)
    os.makedirs(os.path.dirname(output_cpp_file), exist_ok=True)
    os.makedirs(os.path.dirname(output_ui_file), exist_ok=True)

    # Write the output to the file
    with open(output_h_file, "w") as fh:
        fh.write(output_h)
    with open(output_cpp_file, "w") as fh:
        fh.write(output_cpp)
    with open(output_ui_file, "w") as fh:
        fh.write(output_ui)

    print(f"Successfully wrote file {output_h_file}")
    print(f"Successfully wrote file {output_cpp_file}")
    print(f"Successfully wrote file {output_ui_file}")

def generate_qt_widgets_files(
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

    # generate the main.cpp file
    _generate_main_cpp_file(root_path, generation_dict["main_cpp"], files_to_be_generated)

    # generate the mainwindow files
    _generate_mainwindow_files(root_path, generation_dict["mainwindow"], files_to_be_generated)



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

    folder_path = generation_dict["cmakelists"]["folder_path"]

    files = []
    files.append(os.path.join(folder_path, "CMakeLists.txt"))
    files.append(os.path.join(folder_path, "main.cpp"))
    files.append(os.path.join(folder_path, "mainwindow.h"))
    files.append(os.path.join(folder_path, "mainwindow.cpp"))
    files.append(os.path.join(folder_path, "mainwindow.ui"))

    # strip from files if the value in files_to_be_generated is False
    if files_to_be_generated:
        for path, generate in files_to_be_generated.items():
            if not generate and path in files:
                files.remove(path)

    return files


# generate the files into the preview folder
def preview_qt_widgets_files(
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

        generate_qt_widgets_files(
            root_path,
            manifest_preview_file,
            preview_files_to_be_generated,
            uncrustify_config_file,
        )

    else:
        generate_qt_widgets_files(root_path, manifest_preview_file, {}, uncrustify_config_file)
        

def is_enabled(manifest_file: str) -> bool:
    with open(manifest_file, "r") as stream:
        try:
            manifest_data = yaml.safe_load(stream)
        except yaml.YAMLError as exc:
            print(exc)
            return False

    front_ends = manifest_data.get("front_ends", {})
    if not front_ends:
        return False

    front_end_dict = {}

    front_end_dict["qt_widgets"] = front_ends.get("qt_widgets", {})
    if front_end_dict["qt_widgets"]:
        assert front_end_dict["qt_widgets"]["folder_path"], "qt_widgets folder_path is not set in the manifest file"
        return True
    else:
        return False

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
                preview_qt_widgets_files(root_path, manifest_file)
            else:
                generate_qt_widgets_files(root_path, manifest_file)
        else:
            print("Error: Manifest file must be named 'qleany.yaml' or 'qleany.yml'")
    else:
        print("Error: Please provide the manifest file as an argument")
