from typing import Sequence
from qleany.common.entities.entity import Entity
from qleany.direct_access.entity.dtos import CreateEntitiesDto, EntityDto
from qleany.direct_access.entity.i_entity_uow import IEntityUow


class CreateUc():
    def __init__(self, unit_of_work: IEntityUow):
        self._unit_of_work = unit_of_work

    def execute(self, create_dto: CreateEntitiesDto) -> list[EntityDto]:
        
        self.validate(create_dto)
        
        with self._unit_of_work as uow:
            entities_to_create = self._convert_dtos_to_entities(create_dto.entities)
            new_entities = uow.entity_repository.create(entities_to_create, create_dto.owner_id, create_dto.position)
            return self._convert_entities_to_dtos(new_entities)
        
    def validate(self, create_dto: CreateEntitiesDto):
        # verify if exist
        with self._unit_of_work as uow:
            if not uow.root_repository.exists([create_dto.owner_id]):
                raise Exception("Root entity does not exist")

        
    def _convert_entity_to_dto(self, entity: Entity) -> EntityDto:
        return EntityDto(
            id_=entity.id_,
            only_for_heritage=entity.only_for_heritage,
            fields=entity.fields,
            #relationships=entity.relationships,
        )
        
    def _convert_dto_to_entity(self, dto: EntityDto) -> Entity:
        return Entity(
            id_= 0,
            only_for_heritage=dto.only_for_heritage,
            fields=dto.fields,
            #relationships=dto.relationships,
        )
    
    def _convert_entities_to_dtos(self, entities: Sequence[Entity]) -> list[EntityDto]:
        return [self._convert_entity_to_dto(entity) for entity in entities]
    
    def _convert_dtos_to_entities(self, dtos: Sequence[EntityDto]) -> list[Entity]:
        return [self._convert_dto_to_entity(dto) for dto in dtos]