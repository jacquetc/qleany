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
    front_ends = manifest_data.get("front_ends", {})
    front_end_dict = {}

    front_end_dict["kf6_kirigami"] = front_ends.get("kf6_kirigami", {})
    if front_end_dict["kf6_kirigami"]:
        assert front_end_dict["kf6_kirigami"][
            "folder_path"
        ], "kf6_kirigami folder_path is not set in the manifest file"
    folder_path = front_end_dict["kf6_kirigami"]["folder_path"]

    cmakelists_dict["folder_path"] = folder_path
    generation_dict["cmakelists"] = cmakelists_dict

    # main.cpp

    main_cpp_dict = {}
    main_cpp_dict["folder_path"] = folder_path
    main_cpp_dict["application_name"] = application_name
    main_cpp_dict["application_name_lower"] = application_name_lower
    main_cpp_dict["application_name_pascal"] = stringcase.pascalcase(application_name)
    main_cpp_dict["application_cpp_domain_name"] = application_cpp_domain_name
    main_cpp_dict["organisation_domain"] = (
        manifest_data.get("global", {})
        .get("organisation", {})
        .get("domain", "example.com")
    )
    main_cpp_dict["organisation_name"] = (
        manifest_data.get("global", {}).get("organisation", {}).get("name", "example")
    )

    generation_dict["main_cpp"] = main_cpp_dict

    # qml file
    qml_dict = {}
    qml_dict["folder_path"] = folder_path
    qml_dict["application_name"] = application_name
    qml_dict["application_name_lower"] = application_name_lower
    qml_dict["application_cpp_domain_name"] = application_cpp_domain_name

    generation_dict["qml"] = qml_dict

    # placeholderconfig
    placeholderconfig_dict = {}
    placeholderconfig_dict["folder_path"] = folder_path
    placeholderconfig_dict["application_name_lower"] = application_name_lower
    placeholderconfig_dict["application_name_pascal"] = stringcase.pascalcase(application_name)

    generation_dict["placeholderconfig"] = placeholderconfig_dict


    return generation_dict


def _generate_cmakelists_file(
    root_path: str, cmakelists_dict: dict, files_to_be_generated: dict[str, bool]
):
    cmakelists_file = os.path.join(cmakelists_dict["folder_path"], "CMakeLists.txt")

    # Create the jinja2 environment
    template_env = Environment(loader=FileSystemLoader("templates/UIs/kf6_kirigami"))
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
        application_name_spinal=stringcase.spinalcase(
            cmakelists_dict["application_name"]
        ),
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


def _generate_main_cpp_file(
    root_path: str, main_cpp_dict: dict, files_to_be_generated: dict[str, bool]
):
    main_cpp_file = os.path.join(main_cpp_dict["folder_path"], "main.cpp")

    # Create the jinja2 environment
    template_env = Environment(loader=FileSystemLoader("templates/UIs/kf6_kirigami"))
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
        application_name_lower=main_cpp_dict["application_name_lower"],
        application_name_upper=main_cpp_dict["application_name_lower"].upper(),
        application_name_pascal=main_cpp_dict["application_name_pascal"],
        organisation_domain=main_cpp_dict["organisation_domain"],
        organisation_name=main_cpp_dict["organisation_name"],
    )

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Write the output to the file
    with open(output_file, "w") as fh:
        fh.write(output)

    print(f"Successfully wrote file {output_file}")

def _generate_placeholderconfig_kcfgc_file(
    root_path: str, folder_path: str, placeholderconfig: dict, files_to_be_generated: dict[str, bool]
):
    file = os.path.join(folder_path, f"{placeholderconfig['application_name_lower']}config.kcfgc")
    # Create the jinja2 environment
    template_env = Environment(
        loader=FileSystemLoader("templates/UIs/kf6_kirigami")
    )
    # Load the template
    placeholderconfig_template = template_env.get_template("placeholderconfig.kcfgc.jinja2")

    # generate the placeholderconfig.kcfgc file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(file, False):
        return

    placeholderconfig_file = os.path.join(root_path, file)

    # Render the template
    output_h = placeholderconfig_template.render(
        application_name_lower=placeholderconfig["application_name_lower"],
        application_name_pascal=placeholderconfig["application_name_pascal"],
    )
    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(placeholderconfig_file), exist_ok=True)

    # Write the output to the file
    with open(placeholderconfig_file, "w") as fh:
        fh.write(output_h)

    print(f"Successfully wrote file {placeholderconfig_file}")


def _generate_qml_file(
    root_path: str, qml_dict: dict, files_to_be_generated: dict[str, bool]
):
    file = os.path.join(qml_dict["folder_path"], "contents/ui/Main.qml")
    # Create the jinja2 environment
    template_env = Environment(
        loader=FileSystemLoader("templates/UIs/kf6_kirigami/contents/ui")
    )
    # Load the template
    qml_template = template_env.get_template("Main.qml.jinja2")

    # generate the qml file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(file, False):
        return

    qml_file = os.path.join(root_path, file)

    # Render the template
    output_h = qml_template.render(
        application_name=qml_dict["application_name"],
        application_name_lower=qml_dict["application_name_lower"],
        application_pascal_name=stringcase.pascalcase(qml_dict["application_name"]),
        application_cpp_domain_name=qml_dict["application_cpp_domain_name"],
    )
    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(qml_file), exist_ok=True)

    # Write the output to the file
    with open(qml_file, "w") as fh:
        fh.write(output_h)

    print(f"Successfully wrote file {qml_file}")


def _generate_other_files(
    root_path: str,
    folder_path: str,
    generation_dict: dict, files_to_be_generated: dict[str, bool],
):
    application_name_lower = generation_dict["cmakelists"]["application_name_lower"]

    # copy the placeholderconfig.kcfg file
    source = os.path.join("templates/UIs/kf6_kirigami", "placeholderconfig.kcfg")
    destination = os.path.join(folder_path, f"{application_name_lower}config.kcfg")

    # generate the placeholderconfig.kcfg file if in the files_to_be_generated dict the value is True
    if files_to_be_generated.get(destination, False):

        destination = os.path.join(root_path, destination)

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(destination), exist_ok=True)

        shutil.copy(source, destination)

        print(f"Successfully wrote file {destination}")

    # copy the contents/ui/About.qml file
    source = os.path.join("templates/UIs/kf6_kirigami/contents/ui", "About.qml")
    destination = os.path.join(folder_path, "contents/ui/About.qml")

    # generate the contents/ui/About.qml file if in the files_to_be_generated dict the value is True
    if files_to_be_generated.get(destination, False):

        destination = os.path.join(root_path, destination)

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(destination), exist_ok=True)

        shutil.copy(source, destination)

        print(f"Successfully wrote file {destination}")

    # copy the app.cpp file
    source = os.path.join("templates/UIs/kf6_kirigami", "app.cpp")
    destination = os.path.join(folder_path, "app.cpp")

    # generate the app.cpp file if in the files_to_be_generated dict the value is True
    if files_to_be_generated.get(destination, False):

        destination = os.path.join(root_path, destination)

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(destination), exist_ok=True)

        shutil.copy(source, destination)

        print(f"Successfully wrote file {destination}")

    # copy the app.h file
    source = os.path.join("templates/UIs/kf6_kirigami", "app.h")
    destination = os.path.join(folder_path, "app.h")

    # generate the app.h file if in the files_to_be_generated dict the value is True
    if files_to_be_generated.get(destination, False):

        destination = os.path.join(root_path, destination)

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(destination), exist_ok=True)

        shutil.copy(source, destination)

        print(f"Successfully wrote file {destination}")

def generate_kf6_kirigami_files(
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
    folder_path = generation_dict["cmakelists"]["folder_path"]

    # generate the CMakeLists.txt file
    _generate_cmakelists_file(
        root_path, generation_dict["cmakelists"], files_to_be_generated
    )

    # generate the main.cpp file
    _generate_main_cpp_file(
        root_path, generation_dict["main_cpp"], files_to_be_generated
    )

    # generate the qml file
    _generate_qml_file(root_path, generation_dict["qml"], files_to_be_generated)

    # generate the placeholderconfig.kcfgc file
    _generate_placeholderconfig_kcfgc_file(
        root_path, folder_path, generation_dict["placeholderconfig"], files_to_be_generated
    )

    _generate_other_files(root_path, folder_path, generation_dict, files_to_be_generated)

    qml_imports_generator.generate_qml_imports_files(
        root_path, folder_path, manifest_file, files_to_be_generated
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
    application_name_lower = generation_dict["cmakelists"]["application_name_lower"]

    folder_path = generation_dict["cmakelists"]["folder_path"]

    files = []
    files.append(os.path.join(folder_path, "CMakeLists.txt"))
    files.append(os.path.join(folder_path, "main.cpp"))
    files.append(os.path.join(folder_path, "contents/ui/Main.qml"))
    files.append(os.path.join(folder_path, "contents/ui/About.qml"))
    files.append(os.path.join(folder_path, "app.cpp"))
    files.append(os.path.join(folder_path, "app.h"))
    files.append(os.path.join(folder_path, f"{application_name_lower}config.kcfg"))
    files.append(os.path.join(folder_path, f"{application_name_lower}config.kcfgc"))

    # strip from files if the value in files_to_be_generated is False
    if files_to_be_generated:
        for path, generate in files_to_be_generated.items():
            if not generate and path in files:
                files.remove(path)

    # get the files from the qml_imports_generator
    files.extend(
        qml_imports_generator.get_files_to_be_generated(
            manifest_file, files_to_be_generated, folder_path
        )
    )

    return files


# generate the files into the preview folder
def preview_kf6_kirigami_files(
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

        generate_kf6_kirigami_files(
            root_path,
            manifest_preview_file,
            preview_files_to_be_generated,
            uncrustify_config_file,
        )

    else:
        generate_kf6_kirigami_files(
            root_path, manifest_preview_file, {}, uncrustify_config_file
        )


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

    front_end_dict["kf6_kirigami"] = front_ends.get("kf6_kirigami", {})
    if front_end_dict["kf6_kirigami"]:
        assert front_end_dict["kf6_kirigami"][
            "folder_path"
        ], "kf6_kirigami folder_path is not set in the manifest file"
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
                preview_kf6_kirigami_files(root_path, manifest_file)
            else:
                generate_kf6_kirigami_files(root_path, manifest_file)
        else:
            print("Error: Manifest file must be named 'qleany.yaml' or 'qleany.yml'")
    else:
        print("Error: Please provide the manifest file as an argument")
