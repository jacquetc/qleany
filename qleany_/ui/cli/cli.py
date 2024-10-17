import argparse
import pathlib

from qleany_.__version__ import __version__
from qleany_.common.direct_access.persistence_registration import register_persistence
from qleany_.common.feature_registration import register_features
from qleany_.direct_access.direct_access_registration import register_controllers
from qleany_.manifest_handling_feature.dtos import LoadManifestDto
from qleany_.manifest_handling_feature.manifest_handling_controller import (
    ManifestHandlingController,
)
from qleany_.python_file_listing_feature.dtos import PythonFileListingDto
from qleany_.python_file_listing_feature.python_file_listing_controller import (
    PythonFileListingController,
)


def register():
    repository_factory, db_context, messenger = register_persistence()
    register_controllers(db_context, repository_factory, messenger)
    register_features(db_context, repository_factory, messenger)


def run_cli():
    register()

    # Parse command line arguments
    parser = argparse.ArgumentParser(
        description=f"Qleany CLI v{__version__}", epilog=""
    )
    subparsers = parser.add_subparsers(dest="command", required=True)
    # Subparser for the "list" command
    parser_list = subparsers.add_parser("list", help="List existing items")
    parser_list.add_argument(
        "--existing", action="store_true", help="List existing items"
    )
    parser_list.add_argument("--group", type=str, help="Select a group")
    parser_list.add_argument(
        "--manifest-path",
        type=pathlib.Path,
        help="Path to the manifest file",
        default="./qleany.yaml",
    )
    parser_list.set_defaults(func=parse_list_command)

    # Subparser for the "generate" command
    parser_generate = subparsers.add_parser("generate", help="Generate a file")
    parser_generate.add_argument(
        "--file",
        type=str,
        required=True,
        action="append",
        help="Specify the file to generate",
    )
    parser_generate.add_argument(
        "--manifest-path",
        type=pathlib.Path,
        help="Path to the manifest file",
        default="./qleany.yaml",
    )
    parser_generate.set_defaults(func=parse_generate_command)

    args = parser.parse_args()
    args.func(args)


def parse_list_command(args):
    group = args.group
    existing = args.existing
    manifest_path = args.manifest_path

    ManifestHandlingController.get_instance().load_manifest(
        LoadManifestDto(file_path=manifest_path)
    )

    if group == "direct_access" or group == "direct" or group == "da":
        print("Listing files for direct access")
        dto = PythonFileListingDto(existing=existing)
        response_dto = (
            PythonFileListingController.get_instance().list_direct_access_files(dto)
        )
        for file in response_dto.files:
            print(file)

    elif group == "entities" or group == "entity" or group == "e":
        print("Listing files for entities")
        dto = PythonFileListingDto(existing=existing)
        response_dto = PythonFileListingController.get_instance().list_entity_files(dto)
        for file in response_dto.files:
            print(file)

    elif group == "features" or group == "feature" or group == "f":
        print("Listing files for features")
        dto = PythonFileListingDto(existing=existing)
        response_dto = PythonFileListingController.get_instance().list_feature_files(
            dto
        )
        for file in response_dto.files:
            print(file)

    elif group == "persistence" or group == "persist" or group == "p":
        print("Listing files for persistence")
        dto = PythonFileListingDto(existing=existing)
        response_dto = (
            PythonFileListingController.get_instance().list_persistence_files(dto)
        )
        for file in response_dto.files:
            print(file)

    elif group == "common" or group == "c":
        print("Listing common files")
        dto = PythonFileListingDto(existing=existing)
        response_dto = (
            PythonFileListingController.get_instance().list_common_base_files(dto)
        )
        for file in response_dto.files:
            print(file)

    elif group == "ui":
        print("Listing UI files")
        dto = PythonFileListingDto(existing=existing)
        response_dto = PythonFileListingController.get_instance().list_ui_files(dto)
        for file in response_dto.files:
            print(file)

    elif group == "all":
        print("Listing all files")
        dto = PythonFileListingDto(existing=existing)
        files = []
        response_dto = (
            PythonFileListingController.get_instance().list_direct_access_files(dto)
        )
        files.extend(response_dto.files)
        response_dto = PythonFileListingController.get_instance().list_entity_files(dto)
        files.extend(response_dto.files)
        response_dto = PythonFileListingController.get_instance().list_feature_files(
            dto
        )
        files.extend(response_dto.files)
        response_dto = (
            PythonFileListingController.get_instance().list_persistence_files(dto)
        )
        files.extend(response_dto.files)
        response_dto = (
            PythonFileListingController.get_instance().list_common_base_files(dto)
        )
        files.extend(response_dto.files)
        response_dto = PythonFileListingController.get_instance().list_ui_files(dto)
        files.extend(response_dto.files)

        for file in files:
            print(file)

    else:
        print("Invalid group")


def parse_generate_command(args):
    print(f"Generating file: {args.file}")


if __name__ == "__main__":
    run_cli()
