import sys
import os
import __version__
from pathlib import Path

full_path = Path(__file__).resolve().parent
full_path = f"{full_path}"
# add the current directory to the path so that we can import the generated files
sys.path.append(full_path)

# set the current directory to the generator directory
os.chdir(full_path)

from generator.qleany_generator_gui import main as gui_main


def main():
    # -v or --version
    if len(sys.argv) > 1 and sys.argv[1] in ["-v", "--version"]:
        print("qleany version:")
        print(__version__.__version__)

    elif len(sys.argv) > 1 and sys.argv[1] == "gui":
        gui_main()
    else:
        gui_main()


if __name__ == "__main__":
    main()
