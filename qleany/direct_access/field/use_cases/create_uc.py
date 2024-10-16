from typing import Sequence
from qleany.common.entities.field import Field
from qleany.direct_access.field.dtos import CreateFieldsDto, FieldDto
from qleany.direct_access.field.i_field_uow import IFieldUow


class CreateUc():
    def __init__(self, unit_of_work: IFieldUow):
        self._unit_of_work = unit_of_work

    def execute(self, create_dto: CreateFieldsDto) -> list[FieldDto]:
        
        self.validate(create_dto)
        
        with self._unit_of_work as uow:
            entities_to_create = self._convert_dtos_to_entities(create_dto.entities)
            new_entities = uow.field_repository.create(entities_to_create, create_dto.owner_id, create_dto.position)
            return self._convert_entities_to_dtos(new_entities)
        
    def validate(self, create_dto: CreateFieldsDto):
        if not create_dto.entities:
            raise ValueError("No entities to create")
        # verify if exist
        with self._unit_of_work as uow:
            if not uow.entity_repository.exists(create_dto.owner_id):
                raise Exception("Root field does not exist")

        
    def _convert_entity_to_dto(self, field: Field) -> FieldDto:
        return FieldDto(
            id_=field.id_,
            name=field.name,
            type_=field.type_,
            entity= field.entity,
            is_nullable= field.is_nullable,
            is_primary_key= field.is_primary_key,
            is_list= field.is_list,
            is_single= field.is_single,
            strong= field.strong,
            ordered= field.ordered,
            list_model= field.list_model,
            list_model_displayed_field= field.list_model_displayed_field,
        )
        
    def _convert_dto_to_entity(self, dto: FieldDto) -> Field:
        return Field(
            id_= 0,
            name=dto.name,
            type_=dto.type_,
            entity= dto.entity,
            is_nullable= dto.is_nullable,
            is_primary_key= dto.is_primary_key,
            is_list= dto.is_list,
            is_single= dto.is_single,
            strong= dto.strong,
            ordered= dto.ordered,
            list_model= dto.list_model,
            list_model_displayed_field= dto.list_model_displayed_field,
        )
    
    def _convert_entities_to_dtos(self, entities: Sequence[Field]) -> list[FieldDto]:
        return [self._convert_entity_to_dto(field) for field in entities]
    
    def _convert_dtos_to_entities(self, dtos: Sequence[FieldDto]) -> list[Field]:
        return [self._convert_dto_to_entity(dto) for dto in dtos]