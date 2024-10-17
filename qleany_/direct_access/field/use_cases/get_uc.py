from typing import Sequence

from qleany_.common.entities.field import Field
from qleany_.direct_access.field.dtos import FieldDto
from qleany_.direct_access.field.i_field_uow import IFieldUow


class GetUc:
    def __init__(self, unit_of_work: IFieldUow):
        self._unit_of_work = unit_of_work

    def execute(self, ids: Sequence[int]) -> Sequence[FieldDto]:
        with self._unit_of_work as uow:
            entities = uow.field_repository.get(tuple(ids))
            return self._convert_entities_to_dtos(entities)

    def _convert_entity_to_dto(self, field: Field) -> FieldDto:
        return FieldDto(
            id_=field.id_,
            name=field.name,
            type_=field.type_,
            entity=field.entity,
            is_nullable=field.is_nullable,
            is_primary_key=field.is_primary_key,
            is_list=field.is_list,
            is_single=field.is_single,
            strong=field.strong,
            ordered=field.ordered,
            list_model=field.list_model,
            list_model_displayed_field=field.list_model_displayed_field,
        )

    def _convert_entities_to_dtos(self, entities: Sequence[Field]) -> list[FieldDto]:
        return [self._convert_entity_to_dto(entity) for entity in entities]

    def _convert_dtos_to_entities(self, dtos: Sequence[FieldDto]) -> list[Field]:
        return [self._convert_dto_to_entity(dto) for dto in dtos]

    def _convert_dto_to_entity(self, dto: FieldDto) -> Field:
        return Field(
            id_=dto.id_,
            name=dto.name,
            type_=dto.type_,
            entity=dto.entity,
            is_nullable=dto.is_nullable,
            is_primary_key=dto.is_primary_key,
            is_list=dto.is_list,
            is_single=dto.is_single,
            strong=dto.strong,
            ordered=dto.ordered,
            list_model=dto.list_model,
            list_model_displayed_field=dto.list_model_displayed_field,
        )
