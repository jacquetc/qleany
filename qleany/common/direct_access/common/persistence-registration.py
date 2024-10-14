

from qleany.common.direct_access.common.repository.repository_factory import RepositoryFactory


def register_persistence() -> RepositoryFactory:
    factory = RepositoryFactory()

    # entity
    from qleany.common.direct_access.entity.entity_db_table_group import EntityDbTableGroup
    from qleany.common.direct_access.entity.entity_repository import EntityRepository
    factory.register(EntityRepository, EntityDbTableGroup)



    return factory