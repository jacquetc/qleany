

from qleany.common.direct_access.common.database.interfaces.i_db_context import IDbContext
from qleany.common.direct_access.common.repository.repository_factory import IRepositoryFactory
from qleany.direct_access.entity.dtos import CreateEntitiesDto, EntityDto
from qleany.direct_access.entity.entity_uow import EntityUow
from qleany.direct_access.entity.use_cases.create_uc import CreateUc


class EntityController:

    def __init__(self, db_context: IDbContext, repository_factory: IRepositoryFactory):
        self._db_context = db_context
        self._repository_factory = repository_factory
        
    def create(self, dto: CreateEntitiesDto) -> list[EntityDto]:
        unit_of_work = EntityUow(self._db_context, self._repository_factory)
        return CreateUc(unit_of_work).execute(dto)
        
        