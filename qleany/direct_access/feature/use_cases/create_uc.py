from typing import Sequence

from qleany.common.entities.feature import Feature
from qleany.direct_access.feature.dtos import CreateFeaturesDto, FeatureDto
from qleany.direct_access.feature.i_feature_uow import IFeatureUow


class CreateUc:
    def __init__(self, unit_of_work: IFeatureUow):
        self._unit_of_work = unit_of_work

    def execute(self, create_dto: CreateFeaturesDto) -> list[FeatureDto]:
        self.validate(create_dto)

        with self._unit_of_work as uow:
            entities_to_create = self._convert_dtos_to_entities(create_dto.entities)
            new_entities = uow.feature_repository.create(
                entities_to_create, create_dto.owner_id, create_dto.position
            )
            return self._convert_entities_to_dtos(new_entities)

    def validate(self, create_dto: CreateFeaturesDto):
        if not create_dto.entities:
            raise ValueError("No entities to create")
        # verify if exist
        with self._unit_of_work as uow:
            if not uow.root_repository.exists(create_dto.owner_id):
                raise Exception("Root feature does not exist")

    def _convert_entity_to_dto(self, feature: Feature) -> FeatureDto:
        return FeatureDto(
            id_=feature.id_,
            name=feature.name,
            description=feature.description,
            # use_cases=feature.use_cases,
        )

    def _convert_dto_to_entity(self, dto: FeatureDto) -> Feature:
        return Feature(
            id_=0,
            name=dto.name,
            description=dto.description,
            # use_cases=dto.use_cases,
        )

    def _convert_entities_to_dtos(
        self, entities: Sequence[Feature]
    ) -> list[FeatureDto]:
        return [self._convert_entity_to_dto(feature) for feature in entities]

    def _convert_dtos_to_entities(self, dtos: Sequence[FeatureDto]) -> list[Feature]:
        return [self._convert_dto_to_entity(dto) for dto in dtos]
