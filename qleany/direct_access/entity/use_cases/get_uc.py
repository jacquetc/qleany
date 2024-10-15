from typing import Sequence
from qleany.common.entities.entity import Entity
from qleany.direct_access.entity.dtos import EntityDto
from qleany.direct_access.entity.i_entity_uow import IEntityUow


class Get:
    def __init__(self, unit_of_work: IEntityUow):
        self._unit_of_work = unit_of_work

    def execute(self, ids: list[int]) -> list[EntityDto]:
        with self._unit_of_work as uow:
            entities = uow.entity_repository.get(tuple(ids))
            return self._convert_entities_to_dtos(entities)

    def _convert_entity_to_dto(self, entity: Entity) -> EntityDto:
        return EntityDto(
            id_=entity.id_,
            only_for_heritage=entity.only_for_heritage,
            fields=entity.fields,
            #relationships=entity.relationships,
        )
    
    def _convert_entities_to_dtos(self, entities: Sequence[Entity]) -> list[EntityDto]:
        return [self._convert_entity_to_dto(entity) for entity in entities]
    
    def _convert_dtos_to_entities(self, dtos: Sequence[EntityDto]) -> list[Entity]:
        return [self._convert_dto_to_entity(dto) for dto in dtos]
    
    def _convert_dto_to_entity(self, dto: EntityDto) -> Entity:
        return Entity(
            id_=dto.id_,
            only_for_heritage=dto.only_for_heritage,
            fields=dto.fields,
            #relationships=dto.relationships,
        )