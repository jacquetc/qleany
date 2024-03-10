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

    application_name = manifest_data.get("global", {}).get(
        "application_name", "example"
    )

    cmakelists_dict["application_name"] = application_name
    cmakelists_dict["application_name_pascalcase"] = stringcase.pascalcase(
        application_name
    )
    cmakelists_dict["application_name_uppercase"] = stringcase.uppercase(
        application_name
    )
    cmakelists_dict["application_name_lowercase"] = stringcase.lowercase(
        application_name
    ).strip("-_ ")

    # get the paths from the manifest file
    cmakelists_dict["entities_path"] = manifest_data.get("entities", {}).get(
        "folder_path", "src/entities"
    )
    cmakelists_dict["contracts_path"] = manifest_data.get("contracts", {}).get(
        "folder_path", "src/contracts"
    )
    cmakelists_dict["persistence_path"] = manifest_data.get("repositories", {}).get(
        "base_folder_path", "src/persistence"
    )
    cmakelists_dict["contracts_dto_path"] = manifest_data.get("DTOs", {}).get(
        "common_cmake_folder_path", "src/contracts.dto"
    )
    cmakelists_dict["contracts_cqrs_path"] = manifest_data.get("CQRS", {}).get(
        "common_cmake_folder_path", "src/contracts.cqrs"
    )
    cmakelists_dict["application_path"] = manifest_data.get("application", {}).get(
        "common_cmake_folder_path", "src/application"
    )
    cmakelists_dict["interactor_path"] = manifest_data.get("interactor", {}).get(
        "folder_path", "src/interactor"
    )
    cmakelists_dict["presenter_path"] = manifest_data.get("presenter", {}).get(
        "folder_path", "src/presenters"
    )

    # get the front ends
    front_ends = manifest_data.get("front_ends", {})
    front_end_dict = {}
    front_end_count = 0

    front_end_dict["qt_widgets"] = front_ends.get("qt_widgets", {})
    front_end_dict["qt_widgets"]["enabled"] = (
        True if front_end_dict["qt_widgets"] else False
    )
    if front_end_dict["qt_widgets"]["enabled"]:
        assert front_end_dict["qt_widgets"][
            "folder_path"
        ], "qt_widgets folder_path is not set in the manifest file"
        front_end_count += 1

    front_end_dict["qt_quick"] = front_ends.get("qt_quick", {})
    front_end_dict["qt_quick"]["enabled"] = (
        True if front_end_dict["qt_quick"] else False
    )
    if front_end_dict["qt_quick"]["enabled"]:
        assert front_end_dict["qt_quick"][
            "folder_path"
        ], "qt_quick folder_path is not set in the manifest file"
        front_end_count += 1

    front_end_dict["kf6_kirigami"] = front_ends.get("kf6_kirigami", {})
    front_end_dict["kf6_kirigami"]["enabled"] = (
        True if front_end_dict["kf6_kirigami"] else False
    )
    if front_end_dict["kf6_kirigami"]["enabled"]:
        assert front_end_dict["kf6_kirigami"][
            "folder_path"
        ], "kf6_kirigami folder_path is not set in the manifest file"
        front_end_count += 1

    front_end_dict["kf6_widgets"] = front_ends.get("kf6_widgets", {})
    front_end_dict["kf6_widgets"]["enabled"] = (
        True if front_end_dict["kf6_widgets"] else False
    )
    if front_end_dict["kf6_widgets"]["enabled"]:
        assert front_end_dict["kf6_widgets"][
            "folder_path"
        ], "kf6_widgets folder_path is not set in the manifest file"
        front_end_count += 1

    front_end_dict["qml_imports_integration"] = front_ends.get(
        "qml_imports_integration", {}
    )
    front_end_dict["qml_imports_integration"]["enabled"] = (
        True if front_end_dict["qml_imports_integration"] else False
    )
    if front_end_dict["qml_imports_integration"]["enabled"]:
        assert front_end_dict["qml_imports_integration"][
            "folder_path"
        ], "qml_imports_integration folder_path is not set in the manifest file"
        front_end_count += 1

    front_end_dict["multiple_uis"] = True if front_end_count > 1 else False
    front_end_dict["no_ui"] = True if front_end_count == 0 else False

    cmakelists_dict["front_ends"] = front_end_dict

    generation_dict["cmakelists"] = cmakelists_dict

    return generation_dict


def _generate_cmakelists_file(
    root_path: str,
    template_path: str,
    cmakelists_dict: dict,
    files_to_be_generated: dict[str, bool],
):
    cmakelists_file = "CMakeLists.txt"

    # Create the jinja2 environment
    template_env = Environment(
        loader=FileSystemLoader(os.path.join("templates/root", template_path))
    )
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


def _generate_cmakelists_file_for_multiple_uis(
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
        application_name_upper=stringcase.uppercase(
            cmakelists_dict["application_name"]
        ),
        application_name_lower=stringcase.lowercase(
            cmakelists_dict["application_name"]
        ),
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

def _generate_cmakelists_file_for_no_ui(
    root_path: str, cmakelists_dict: dict, files_to_be_generated: dict[str, bool]
):
    cmakelists_file = "CMakeLists.txt"

    # Create the jinja2 environment
    template_env = Environment(loader=FileSystemLoader("templates/root"))
    # Load the template
    cmakelists_template = template_env.get_template("CMakeLists.txt.no_ui.jinja2")

    # generate the real cmakelists file if in the files_to_be_generated dict the value is True
    if not files_to_be_generated.get(cmakelists_file, False):
        return

    output_file = os.path.join(root_path, cmakelists_file)

    # Render the template
    output = cmakelists_template.render(
        application_name=cmakelists_dict["application_name"],
        application_name_upper=stringcase.uppercase(
            cmakelists_dict["application_name"]
        ),
        application_name_lower=stringcase.lowercase(
            cmakelists_dict["application_name"]
        ),
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

def _generate_kf6_kirigami_files(
    root_path: str, cmakelists_dict: dict, files_to_be_generated: dict[str, bool]
):
    application_name = cmakelists_dict["application_name"]
    application_name_lower = cmakelists_dict["application_name_lowercase"]
    application_name_pascal = cmakelists_dict["application_name_pascalcase"]

    # Create the jinja2 environment
    template_env = Environment(
        loader=FileSystemLoader(os.path.join("templates/root", "kf6_kirigami"))
    )

    # generate the real files if in the files_to_be_generated dict the value is True
    if files_to_be_generated.get(f"org.kde.{application_name_lower}.json", False):
        # Load the template
        org_kde_json_template = template_env.get_template("org.kde.placeholder.json.jinja2")

        output_file = os.path.join(root_path, f"org.kde.{application_name_lower}.json")

        # Render the template
        output = org_kde_json_template.render(
            application_name_lower=application_name_lower
        )

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(output_file), exist_ok=True)

        # Write the output to the file
        with open(output_file, "w") as fh:
            fh.write(output)

        print(f"Successfully wrote file {output_file}")

    if files_to_be_generated.get(f"org.kde.{application_name_lower}.desktop", False):
        # Load the template
        desktop_template = template_env.get_template("org.kde.placeholder.desktop.jinja2")

        output_file = os.path.join(
            root_path, f"org.kde.{application_name_lower}.desktop"
        )

        # Render the template
        output = desktop_template.render(
            application_name=cmakelists_dict["application_name"]
        )

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(output_file), exist_ok=True)

        # Write the output to the file
        with open(output_file, "w") as fh:
            fh.write(output)

        print(f"Successfully wrote file {output_file}")

    if files_to_be_generated.get(f"org.kde.{application_name_lower}.metainfo.xml", False):
        # Load the template
        metainfo_template = template_env.get_template("org.kde.placeholder.metainfo.xml.jinja2")

        output_file = os.path.join(
            root_path, f"org.kde.{application_name_lower}.metainfo.xml"
        )

        # Render the template
        output = metainfo_template.render(
            application_name=cmakelists_dict["application_name"],
            application_name_lower=application_name_lower
        )

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(output_file), exist_ok=True)

        # Write the output to the file
        with open(output_file, "w") as fh:
            fh.write(output)

        print(f"Successfully wrote file {output_file}")

    if files_to_be_generated.get(f"{application_name_pascal}.kdev4", False):
        # Load the template
        kdev4_template = template_env.get_template("placeholder.kdev4.jinja2")

        output_file = os.path.join(root_path, f"{application_name_pascal}.kdev4")

        # Render the template
        output = kdev4_template.render(application_name_pascal=application_name_pascal)

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(output_file), exist_ok=True)

        # Write the output to the file
        with open(output_file, "w") as fh:
            fh.write(output)

        print(f"Successfully wrote file {output_file}")

    # Create the jinja2 environment for android files
    template_env = Environment(
        loader=FileSystemLoader(os.path.join("templates/root", "kf6_kirigami", "android"))
    )

    if files_to_be_generated.get(f"android/AndroidManifest.xml", False):
        # Load the template
        android_manifest_template = template_env.get_template("AndroidManifest.xml.jinja2")

        output_file = os.path.join(root_path, "android/AndroidManifest.xml")

        # Render the template
        output = android_manifest_template.render(
            application_name_lower=application_name_lower,
            application_name=application_name
        )

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(output_file), exist_ok=True)

        # Write the output to the file
        with open(output_file, "w") as fh:
            fh.write(output)

        print(f"Successfully wrote file {output_file}")

    # copy the other android and LICENSES files if they are in the files_to_be_generated dict
    if files_to_be_generated.get(f"android/build.gradle", False):
        os.makedirs(os.path.join(root_path, "android"), exist_ok=True)
        shutil.copy(
            os.path.join("templates/root/kf6_kirigami/android", "build.gradle"),
            os.path.join(root_path, "android/build.gradle"),
        )
        print(f"Successfully wrote file {root_path}/android/build.gradle")

    if files_to_be_generated.get(f"android/version.gradle.in", False):
        os.makedirs(os.path.join(root_path, "android"), exist_ok=True)
        shutil.copy(
            os.path.join("templates/root/kf6_kirigami/android", "version.gradle.in"),
            os.path.join(root_path, "android/version.gradle.in"),
        )
        print(f"Successfully wrote file {root_path}/android/version.gradle.in")

    if files_to_be_generated.get(f"android/res/drawable/logo.png", False):
        # Create the directory if it does not exist
        os.makedirs(os.path.join(root_path, "android/res/drawable/"), exist_ok=True)
        shutil.copy(
            os.path.join("templates/root/kf6_kirigami/android/res/drawable", "logo.png"),
            os.path.join(root_path, "android/res/drawable/logo.png"),
        )
        print(f"Successfully wrote file {root_path}/android/res/drawable/logo.png")

    if files_to_be_generated.get(f"android/res/drawable/splash.xml", False):
        # Create the directory if it does not exist
        os.makedirs(os.path.join(root_path, "android/res/drawable/"), exist_ok=True)
        shutil.copy(
            os.path.join("templates/root/kf6_kirigami/android/res/drawable", "splash.xml"),
            os.path.join(root_path, "android/res/drawable/splash.xml"),
        )
        print(f"Successfully wrote file {root_path}/android/res/drawable/splash.xml")

    if files_to_be_generated.get(f"LICENSES/BSD-3-Clause.txt", False):
        # Create the directory if it does not exist
        os.makedirs(os.path.join(root_path, "LICENSES"), exist_ok=True)
        shutil.copy(
            os.path.join("templates/root/kf6_kirigami/LICENSES", "BSD-3-Clause.txt"),
            os.path.join(root_path, "LICENSES/BSD-3-Clause.txt"),
        )
        print(f"Successfully wrote file {root_path}/LICENSES/BSD-3-Clause.txt")

    if files_to_be_generated.get(f"LICENSES/GPL-2.0-or-later.txt", False):
        # Create the directory if it does not exist
        os.makedirs(os.path.join(root_path, "LICENSES"), exist_ok=True)
        shutil.copy(
            os.path.join("templates/root/kf6_kirigami/LICENSES", "GPL-2.0-or-later.txt"),
            os.path.join(root_path, "LICENSES/GPL-2.0-or-later.txt"),
        )
        print(f"Successfully wrote file {root_path}/LICENSES/GPL-2.0-or-later.txt")

    if files_to_be_generated.get(f"LICENSES/CC0-1.0.txt", False):
        # Create the directory if it does not exist
        os.makedirs(os.path.join(root_path, "LICENSES"), exist_ok=True)
        shutil.copy(
            os.path.join("templates/root/kf6_kirigami/LICENSES", "CC0-1.0.txt"),
            os.path.join(root_path, "LICENSES/CC0-1.0.txt"),
        )
        print(f"Successfully wrote file {root_path}/LICENSES/CC0-1.0.txt")

    if files_to_be_generated.get(f"LICENSES/FSFAP.txt", False):
        # Create the directory if it does not exist
        os.makedirs(os.path.join(root_path, "LICENSES"), exist_ok=True)
        shutil.copy(
            os.path.join("templates/root/kf6_kirigami/LICENSES", "FSFAP.txt"),
            os.path.join(root_path, "LICENSES/FSFAP.txt"),
        )
        print(f"Successfully wrote file {root_path}/LICENSES/FSFAP.txt")


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
    if generation_dict["cmakelists"]["front_ends"]["multiple_uis"]:
        _generate_cmakelists_file_for_multiple_uis(
            root_path, generation_dict["cmakelists"], files_to_be_generated
        )
    elif generation_dict["cmakelists"]["front_ends"]["no_ui"]:
        _generate_cmakelists_file_for_no_ui(
            root_path, generation_dict["cmakelists"], files_to_be_generated
        )
    elif generation_dict["cmakelists"]["front_ends"]["qt_widgets"]["enabled"]:
        _generate_cmakelists_file(
            root_path, "qt_widgets", generation_dict["cmakelists"], files_to_be_generated
        )
    elif generation_dict["cmakelists"]["front_ends"]["qt_quick"]["enabled"]:
        _generate_cmakelists_file(
            root_path, "qt_quick", generation_dict["cmakelists"], files_to_be_generated
        )
    # generate the kf6 kirigami files
    elif generation_dict["cmakelists"]["front_ends"]["kf6_kirigami"]["enabled"]:
        _generate_cmakelists_file(
            root_path, "kf6_kirigami", generation_dict["cmakelists"], files_to_be_generated
        )

    if generation_dict["cmakelists"]["front_ends"]["kf6_kirigami"]["enabled"]:
        _generate_kf6_kirigami_files(
            root_path, generation_dict["cmakelists"], files_to_be_generated
        )


def _get_files_to_be_generated_for_qt_widgets(generation_dict: dict) -> list[str]:
    return []


def _get_files_to_be_generated_for_qt_quick(generation_dict: dict) -> list[str]:
    return []


def _get_files_to_be_generated_for_kf6_widgets(generation_dict: dict) -> list[str]:
    application_name = generation_dict["cmakelists"]["application_name"]
    
    # trow an exception
    raise Exception("kf6_widgets is not implemented yet")
    
    return []


def _get_files_to_be_generated_for_kf6_kirigami(generation_dict: dict) -> list[str]:
    application_name_lower = generation_dict["cmakelists"]["application_name_lowercase"]
    application_name_pascal = generation_dict["cmakelists"][
        "application_name_pascalcase"
    ]

    file_list = [
        f"org.kde.{application_name_lower}.json",
        f"{application_name_lower}.desktop",
        f"org.kde.{application_name_lower}.metainfo.xml",
        f"{application_name_pascal}.kdev4",
        f"android/AndroidManifest.xml",
        f"android/build.gradle",
        f"android/version.gradle.in",
        f"android/res/drawable/logo.png",
        f"android/res/drawable/splash.xml",
        f"LICENSES/BSD-3-Clause.txt",
        f"LICENSES/GPL-2.0-or-later.txt",
        f"LICENSES/CC0-1.0.txt",
        f"LICENSES/FSFAP.txt",
    ]
    return file_list


def get_files_to_be_generated(
    manifest_file: str, files_to_be_generated: dict[str, bool] = {}
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

    generation_dict = _get_generation_dict(manifest_data)

    files = []
    # there is always a root CMakeLists.txt file
    files.append("CMakeLists.txt")

    # if multiple UIs:
    if generation_dict["cmakelists"]["front_ends"]["multiple_uis"]:
        if generation_dict["cmakelists"]["front_ends"]["qt_widgets"]["enabled"]:
            files.extend(_get_files_to_be_generated_for_qt_widgets(generation_dict))
        if generation_dict["cmakelists"]["front_ends"]["qt_quick"]["enabled"]:
            files.extend(_get_files_to_be_generated_for_qt_quick(generation_dict))
        if generation_dict["cmakelists"]["front_ends"]["kf6_widgets"]["enabled"]:
            files.extend(_get_files_to_be_generated_for_kf6_widgets(generation_dict))
        if generation_dict["cmakelists"]["front_ends"]["kf6_kirigami"]["enabled"]:
            files.extend(_get_files_to_be_generated_for_kf6_kirigami(generation_dict))

        # remove duplicates
        files = list(dict.fromkeys(files))

    # if only one UI:
    elif generation_dict["cmakelists"]["front_ends"]["qt_widgets"]["enabled"]:
        pass
    elif generation_dict["cmakelists"]["front_ends"]["qt_quick"]["enabled"]:
        pass
    elif generation_dict["cmakelists"]["front_ends"]["kf6_widgets"]["enabled"]:
        files.extend(_get_files_to_be_generated_for_kf6_widgets(generation_dict))
    elif generation_dict["cmakelists"]["front_ends"]["kf6_kirigami"]["enabled"]:
        files.extend(_get_files_to_be_generated_for_kf6_kirigami(generation_dict))

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
        generate_root_files(
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
                preview_root_files(root_path, manifest_file)
            else:
                generate_root_files(root_path, manifest_file)
        else:
            print("Error: Manifest file must be named 'qleany.yaml' or 'qleany.yml'")
    else:
        print("Error: Please provide the manifest file as an argument")
