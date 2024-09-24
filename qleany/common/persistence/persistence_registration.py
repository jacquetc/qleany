from qleany.common.persistence.database.db_context import DbContext
from qleany.common.persistence.repositories.dto_field_repository import (
    DtoFieldRepository,
)
from qleany.common.persistence.repositories.dto_repository import DtoRepository
from qleany.common.persistence.repository_provider import RepositoryProvider
from qleany.common.persistence.repositories.interfaces.i_dto_field_repository import (
    IDtoFieldRepository,
)
from qleany.common.persistence.repositories.interfaces.i_dto_repository import (
    IDtoRepository,
)
from qleany.common.persistence.repositories.interfaces.i_use_case_repository import (
    IUseCaseRepository,
)
from qleany.common.persistence.repositories.interfaces.i_feature_repository import (
    IFeatureRepository,
)
from qleany.common.persistence.repositories.use_case_repository import UseCaseRepository
from qleany.common.persistence.repositories.feature_repository import FeatureRepository
from qleany.common.persistence.repositories.entity_repository import EntityRepository
from qleany.common.persistence.repositories.field_repository import FieldRepository
from qleany.common.persistence.repositories.relationship_repository import (
    RelationshipRepository,
)
from qleany.common.persistence.repositories.interfaces.i_entity_repository import (
    IEntityRepository,
)
from qleany.common.persistence.repositories.interfaces.i_field_repository import (
    IFieldRepository,
)
from qleany.common.persistence.repositories.interfaces.i_relationship_repository import (
    IRelationshipRepository,
)
from qleany.common.persistence.repositories.global_repository import GlobalRepository
from qleany.common.persistence.repositories.interfaces.i_global_repository import (
    IGlobalRepository,
)
from qleany.common.persistence.repositories.root_repository import RootRepository
from qleany.common.persistence.repositories.interfaces.i_root_repository import (
    IRootRepository,
)



def register() -> tuple[RepositoryProvider, DbContext]:
    # initialize database

    db_context = DbContext()

    # assemble repositories
    relationship_repository = RelationshipRepository()
    RepositoryProvider.register(IRelationshipRepository, relationship_repository)
    field_repository = FieldRepository()
    RepositoryProvider.register(IFieldRepository, field_repository)
    entity_repository = EntityRepository(field_repository, relationship_repository)
    RepositoryProvider.register(IEntityRepository, entity_repository)
    dto_field_repository = DtoFieldRepository()
    RepositoryProvider.register(IDtoFieldRepository, dto_field_repository)
    dto_repository = DtoRepository(dto_field_repository)
    RepositoryProvider.register(IDtoRepository, dto_repository)
    use_case_repository = UseCaseRepository(entity_repository, dto_repository)
    RepositoryProvider.register(IUseCaseRepository, use_case_repository)
    feature_repository = FeatureRepository(use_case_repository)
    RepositoryProvider.register(IFeatureRepository, feature_repository)





    global_repository = GlobalRepository()
    RepositoryProvider.register(IGlobalRepository, global_repository)
    root_repository = RootRepository(
        feature_repository, entity_repository, global_repository
    )
    RepositoryProvider.register(IRootRepository, root_repository)

    return (RepositoryProvider(), db_context)
