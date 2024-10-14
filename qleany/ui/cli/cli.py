from qleany.common.direct_access.persistence_registration import register_persistence


def register():
    repository_factory = register_persistence()


def run_cli():
    register()


if __name__ == "__main__":
    run_cli()
