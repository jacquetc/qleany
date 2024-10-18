from typing import Tuple

from qleany.common.direct_access.common.database.db_table_creator import DbTableCreator
from qleany.common.direct_access.common.database.sqlite_db_context import (
    SqliteDbContext,
)
from qleany.common.direct_access.common.repository.repository_factory import (
    RepositoryFactory,
)
from qleany.common.direct_access.common.repository.repository_messenger import Messenger

# from qleany.common.direct_access.feature.feature_repository import FeatureRepository
# from qleany.common.direct_access.feature.feature_db_table_group import FeatureDbTableGroup


def register_persistence() -> Tuple[RepositoryFactory, SqliteDbContext, Messenger]:
    messenger = Messenger()
    factory = RepositoryFactory(messenger)
    db_context = SqliteDbContext()
    table_creator = DbTableCreator(db_context.get_connection())

    # set up root
    from qleany.common.direct_access.root.root_db_table_group import RootDbTableGroup
    from qleany.common.direct_access.root.root_repository import RootRepository
    from qleany.common.entities.root import Root

    factory.register(RootRepository, RootDbTableGroup)
    table_creator.add_tables([Root])

    # set up entity
    from qleany.common.direct_access.entity.entity_db_table_group import (
        EntityDbTableGroup,
    )
    from qleany.common.direct_access.entity.entity_repository import EntityRepository
    from qleany.common.entities.entity import Entity

    factory.register(EntityRepository, EntityDbTableGroup)
    table_creator.add_tables([Entity])

    # set up field
    from qleany.common.direct_access.field.field_db_table_group import FieldDbTableGroup
    from qleany.common.direct_access.field.field_repository import FieldRepository
    from qleany.common.entities.field import Field

    factory.register(FieldRepository, FieldDbTableGroup)
    table_creator.add_tables([Field])

    # set up feature
    from qleany.common.direct_access.feature.feature_db_table_group import (
        FeatureDbTableGroup,
    )
    from qleany.common.direct_access.feature.feature_repository import FeatureRepository
    from qleany.common.entities.feature import Feature

    factory.register(FeatureRepository, FeatureDbTableGroup)
    table_creator.add_tables([Feature])

    # finally, create the internal database
    table_creator.create_empty_database()

    return (factory, db_context, messenger)
