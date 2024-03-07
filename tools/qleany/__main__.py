import sys
import os
from pathlib import Path

caller_path = os.getcwd()

full_path = Path(__file__).resolve().parent
full_path = f"{full_path}"
# add the current directory to the path so that we can import the generated files
sys.path.append(full_path)

# set the current directory to the generator directory
os.chdir(full_path)

import __version__

from generator.qleany_generator_gui import main as gui_main
from generator.qleany_init import generate_blank_qleany_yaml
from generator.manifest_validator import validate_manifest

def _print_version():
    print("qleany version:")
    print(__version__.__version__)
    print("")
    print("License:")
    print("MPL License")
    print("https://www.mozilla.org/en-US/MPL/2.0/")

def print_help():
    print("qleany version:")
    print(__version__.__version__)
    print("Written by: Cyril Jacquet")
    print("Github: https://github.com/jacquetc/qleany")
    print("")
    print("Usage:")
    print("qleany init  # write a new qleany.yaml file in the current directory")
    print("qleany gui   # start the GUI")
    print("qleany check # check the qleany.yaml file in the current directory")
    print("qleany       # Show this screen")
    print("")
    print("Options:")
    print("-v, --version  Show version")
    print("-h, --help     Show this screen")
    print("")
    print("Description:")
    print("Qleany is a tool to generate a project structure and files from a qleany.yaml schema file.")
    print("It is a code generator that can be used to generate use cases, DTOs, models, services, and repositories")
    print("for a Qt/C++ project using the Clean Architecture.")
    print("")
    print("License:")
    print("MPL License")
    print("https://www.mozilla.org/en-US/MPL/2.0/")


def main():
    # -v or --version
    if len(sys.argv) > 1 and sys.argv[1] in ["-v", "--version"]:
        _print_version()

    if len(sys.argv) > 1 and sys.argv[1] in ["-h", "--help"]:
        print_help()

    elif len(sys.argv) > 1 and sys.argv[1] == "gui":
        gui_main()

    elif len(sys.argv) > 1 and sys.argv[1] == "init":
        print("Writing a new qleany.yaml file in the current directory")
        generate_blank_qleany_yaml(caller_path)

    elif len(sys.argv) > 1 and sys.argv[1] == "check":
        print("Checking the qleany.yaml file in the current directory")
        validate_manifest(os.path.join(caller_path, "qleany.yaml"))
        print("Done")
    else:
        _print_version()
        gui_main()


if __name__ == "__main__":
    main()
