from typing import Sequence
from qleany.common.entities.root import Root
from qleany.direct_access.root.dtos import RootDto
from qleany.direct_access.root.i_root_uow import IRootUow

class UpdateUc():
    def __init__(self, unit_of_work: IRootUow):
        self._unit_of_work = unit_of_work

    def execute(self, update_dtos: Sequence[RootDto]) -> Sequence[RootDto]:
        
        self._validate(update_dtos)
        
        with self._unit_of_work as uow:
            entities_to_update = self._convert_dtos_to_entities(update_dtos)
            updated_entities = uow.root_repository.update(entities_to_update)
            return self._convert_entities_to_dtos(updated_entities)
    
    def _validate(self, update_dtos: Sequence[RootDto]):
        entity_ids = [dto.id_ for dto in update_dtos]
        with self._unit_of_work as uow:
            entities = uow.root_repository.get(entity_ids)
            if len(entities) != len(entity_ids):
                raise Exception("Some entities do not exist")
        
    def _convert_entity_to_dto(self, entity: Root) -> RootDto:
        return RootDto(
            id_=entity.id_,
            # global_id=entity.global_id,
            entities=entity.entities,
            features=entity.features,
        )
        
    def _convert_dto_to_entity(self, dto: RootDto) -> Root:
        return Root(
            id_= dto.id_,
            # global_id=dto.global_id,
            entities=dto.entities,
            features=dto.features,
        )
    
    def _convert_entities_to_dtos(self, entities: Sequence[Root]) -> list[RootDto]:
        return [self._convert_entity_to_dto(entity) for entity in entities]
    
    def _convert_dtos_to_entities(self, dtos: Sequence[RootDto]) -> list[Root]:
        return [self._convert_dto_to_entity(dto) for dto in dtos]