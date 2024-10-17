import os
import sys
from pathlib import Path

# Get the current working directory
caller_path = os.getcwd()

# Resolve the full path of the current file's directory
current_file_path = Path(__file__).resolve().parent

# Navigate up to the root directory of the project
root_path = current_file_path.parent

# Convert the Path object to a string
root_path_str = str(root_path)

# Add the root directory to the sys.path so that we can import the generated files
sys.path.append(root_path_str)

from qleany.ui.cli.cli import run_cli

if __name__ == "__main__":
    run_cli()
