from qleany.common.direct_access.common.repository.repository_factory import RepositoryFactory

from qleany.common.direct_access.entity.entity_repository import EntityRepository
from qleany.common.direct_access.entity.entity_db_table_group import EntityDbTableGroup
from qleany.common.direct_access.field.field_repository import FieldRepository
from qleany.common.direct_access.field.field_db_table_group import FieldDbTableGroup
# from qleany.common.direct_access.feature.feature_repository import FeatureRepository
# from qleany.common.direct_access.feature.feature_db_table_group import FeatureDbTableGroup
from qleany.common.direct_access.root.root_repository import RootRepository
from qleany.common.direct_access.root.root_db_table_group import RootDbTableGroup


def register_persistence() -> RepositoryFactory:
    
    factory = RepositoryFactory()

    factory.register(RootRepository, RootDbTableGroup)
    factory.register(EntityRepository, EntityDbTableGroup)
    factory.register(FieldRepository, FieldDbTableGroup)
    # factory.register(FeatureRepository, FeatureDbTableGroup)
    
    return factory
