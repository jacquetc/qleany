from qleany.direct_access.direct_access_registration import register_controllers
from qleany.common.direct_access.persistence_registration import register_persistence
import argparse


def register():
    repository_factory, db_context, messenger = register_persistence()
    register_controllers(db_context, repository_factory, messenger)


def run_cli():
    register()

    # Parse command line arguments
    parser = argparse.ArgumentParser(description="Qleany CLI")
    subparsers = parser.add_subparsers(dest="command")

    # Subparser for the "list" command
    parser_list = subparsers.add_parser("list", help="List existing items")
    parser_list.add_argument("--existing", action="store_true", help="List existing items")
    parser_list.set_defaults(func=parse_list_command)

    # Subparser for the "generate" command
    parser_generate = subparsers.add_parser("generate", help="Generate a file")
    parser_generate.add_argument("--file", type=str, required=True, help="Specify the file to generate")
    parser_generate.set_defaults(func=parse_generate_command)

    parser.parse_args()



def parse_list_command(args):
    print("Listing existing items")

def parse_generate_command(args):
    print(f"Generating file: {args.file}")

if __name__ == "__main__":
    run_cli()
