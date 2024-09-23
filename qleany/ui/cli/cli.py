from qleany.common.persistence.persistence_registration import (
    register as register_persistence,
)


def register():
    provider = register_persistence()


def run_cli():
    register()


if __name__ == "__main__":
    run_cli()
