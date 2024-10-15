from typing import Sequence
from qleany.common.entities.feature import Feature
from qleany.direct_access.feature.dtos import FeatureDto
from qleany.direct_access.feature.i_feature_uow import IFeatureUow

class UpdateUc():
    def __init__(self, unit_of_work: IFeatureUow):
        self._unit_of_work = unit_of_work

    def execute(self, update_dtos: Sequence[FeatureDto]) -> Sequence[FeatureDto]:
        
        self._validate(update_dtos)
        
        with self._unit_of_work as uow:
            entities_to_update = self._convert_dtos_to_entities(update_dtos)
            updated_entities = uow.feature_repository.update(entities_to_update)
            return self._convert_entities_to_dtos(updated_entities)
    
    def _validate(self, update_dtos: Sequence[FeatureDto]):
        entity_ids = [dto.id_ for dto in update_dtos]
        with self._unit_of_work as uow:
            entities = uow.feature_repository.get(entity_ids)
            if len(entities) != len(entity_ids):
                raise Exception("Some entities do not exist")
        
    def _convert_entity_to_dto(self, entity: Feature) -> FeatureDto:
        return FeatureDto(
            id_=entity.id_,
            name=entity.name,
            description=entity.description,
            #use_cases=entity.use_cases,
        )
        
    def _convert_dto_to_entity(self, dto: FeatureDto) -> Feature:
        return Feature(
            id_= dto.id_,
            name=dto.name,
            description=dto.description,
            #use_cases=dto.use_cases,
        )
    
    def _convert_entities_to_dtos(self, entities: Sequence[Feature]) -> list[FeatureDto]:
        return [self._convert_entity_to_dto(entity) for entity in entities]
    
    def _convert_dtos_to_entities(self, dtos: Sequence[FeatureDto]) -> list[Feature]:
        return [self._convert_dto_to_entity(dto) for dto in dtos]