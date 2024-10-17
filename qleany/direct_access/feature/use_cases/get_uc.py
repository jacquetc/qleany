from typing import Sequence

from qleany.common.entities.feature import Feature
from qleany.direct_access.feature.dtos import FeatureDto
from qleany.direct_access.feature.i_feature_uow import IFeatureUow


class GetUc:
    def __init__(self, unit_of_work: IFeatureUow):
        self._unit_of_work = unit_of_work

    def execute(self, ids: Sequence[int]) -> Sequence[FeatureDto]:
        with self._unit_of_work as uow:
            entities = uow.feature_repository.get(tuple(ids))
            return self._convert_entities_to_dtos(entities)

    def _convert_entity_to_dto(self, entity: Feature) -> FeatureDto:
        return FeatureDto(
            id_=entity.id_,
            name=entity.name,
            description=entity.description,
            # use_cases=entity.use_cases,
        )

    def _convert_entities_to_dtos(
        self, entities: Sequence[Feature]
    ) -> list[FeatureDto]:
        return [self._convert_entity_to_dto(entity) for entity in entities]

    def _convert_dtos_to_entities(self, dtos: Sequence[FeatureDto]) -> list[Feature]:
        return [self._convert_dto_to_entity(dto) for dto in dtos]

    def _convert_dto_to_entity(self, dto: FeatureDto) -> Feature:
        return Feature(
            id_=0,
            name=dto.name,
            description=dto.description,
            # use_cases=dto.use_cases,
        )
