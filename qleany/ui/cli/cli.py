from qleany.common.feature_registration import register_features
from qleany.direct_access.direct_access_registration import register_controllers
from qleany.common.direct_access.persistence_registration import register_persistence

import argparse

from qleany.manifest_handling_feature.dtos import LoadManifestDto
from qleany.manifest_handling_feature.manifest_handling_controller import ManifestHandlingController
from qleany.python_file_listing_feature.dtos import PythonFileListingDto
from qleany.python_file_listing_feature.python_file_listing_controller import PythonFileListingController



def register():
    repository_factory, db_context, messenger = register_persistence()
    register_controllers(db_context, repository_factory, messenger)
    register_features(db_context, repository_factory, messenger)


def run_cli():
    register()

    # Parse command line arguments
    parser = argparse.ArgumentParser(description="Qleany CLI")
    subparsers = parser.add_subparsers(dest="command", required=True)
    # Subparser for the "list" command
    parser_list = subparsers.add_parser("list", help="List existing items")
    parser_list.add_argument("--existing", action="store_true", help="List existing items")
    parser_list.add_argument("--group", type=str, help="Select a group")
    parser_list.add_argument("manifest", type=str, help="Path to the manifest file", default="./qleany.yaml") 
    parser_list.set_defaults(func=parse_list_command)

    # Subparser for the "generate" command
    parser_generate = subparsers.add_parser("generate", help="Generate a file")
    parser_generate.add_argument("--file", type=str, required=True, action='append', help="Specify the file to generate")
    parser_generate.set_defaults(func=parse_generate_command)
    
    args = parser.parse_args()
    args.func(args)
    
def parse_list_command(args):
    
    group = args.group
    existing = args.existing
    manifest = args.manifest
    
    ManifestHandlingController.get_instance().load_manifest(LoadManifestDto(file_path=manifest))
    
    if group == "direct_access":
        print("Listing files for direct access")
        dto = PythonFileListingDto()
        response_dto = PythonFileListingController.get_instance().list_direct_access_files(dto)
        print(file for file in response_dto.files)
        
    if group == "entities" or group == "entity":
        print("Listing files for entities")
        
    elif group == "features" or group == "feature":
        print("Listing files for features")
        
    elif group == "persistence":
        print("Listing files for persistence")
        
    elif group == "common":
        print("Listing common files")
        
    elif group == "ui":
        print("Listing UI files")
        
    elif group == "all":
        print("Listing all files")
    else:
        print("Invalid group")
    

def parse_generate_command(args):
    print(f"Generating file: {args.file}")

if __name__ == "__main__":
    run_cli()
