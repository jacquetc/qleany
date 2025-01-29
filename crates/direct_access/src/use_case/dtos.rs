use std::convert::From;

use common::entities::UseCase;
use common::entities::EntityId;

#[derive(Debug, Clone, PartialEq)]
pub struct UseCaseDto {
    pub id: EntityId,
    pub name: String,
    pub validator: bool,
    pub entities: Vec<EntityId>,
    pub undoable: bool,
    pub dto_in: Option<EntityId>,
    pub dto_out: Option<EntityId>,
}

impl From<UseCaseDto> for UseCase {
    fn from(use_case_dto: UseCaseDto) -> Self {
        UseCase {
            id: use_case_dto.id,
            name: use_case_dto.name,
            validator: use_case_dto.validator,
            entities: use_case_dto.entities,
            undoable: use_case_dto.undoable,
            dto_in: use_case_dto.dto_in,
            dto_out: use_case_dto.dto_out,
        }
    }
}

impl From<&UseCaseDto> for UseCase {
    fn from(use_case_dto: &UseCaseDto) -> Self {
        UseCase {
            id: use_case_dto.id,
            name: use_case_dto.name.clone(),
            validator: use_case_dto.validator,
            entities: use_case_dto.entities.clone(),
            undoable: use_case_dto.undoable,
            dto_in: use_case_dto.dto_in,
            dto_out: use_case_dto.dto_out,
        }
    }
}

impl From<UseCase> for UseCaseDto {
    fn from(use_case: UseCase) -> Self {
        UseCaseDto {
            id: use_case.id,
            name: use_case.name,
            validator: use_case.validator,
            entities: use_case.entities,
            undoable: use_case.undoable,
            dto_in: use_case.dto_in,
            dto_out: use_case.dto_out,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateUseCaseDto {
    pub name: String,
    pub validator: bool,
    pub entities: Vec<EntityId>,
    pub undoable: bool,
    pub dto_in: Option<EntityId>,
    pub dto_out: Option<EntityId>,
}

impl From<CreateUseCaseDto> for UseCase {
    fn from(create_use_case_dto: CreateUseCaseDto) -> Self {
        UseCase {
            id: 0,
            name: create_use_case_dto.name,
            validator: create_use_case_dto.validator,
            entities: create_use_case_dto.entities,
            undoable: create_use_case_dto.undoable,
            dto_in: create_use_case_dto.dto_in,
            dto_out: create_use_case_dto.dto_out,
        }
    }
}

impl From<&CreateUseCaseDto> for UseCase {
    fn from(create_use_case_dto: &CreateUseCaseDto) -> Self {
        UseCase {
            id: 0,
            name: create_use_case_dto.name.clone(),
            validator: create_use_case_dto.validator,
            entities: create_use_case_dto.entities.clone(),
            undoable: create_use_case_dto.undoable,
            dto_in: create_use_case_dto.dto_in,
            dto_out: create_use_case_dto.dto_out,
        }
    }
}

impl From<UseCase> for CreateUseCaseDto {
    fn from(use_case: UseCase) -> Self {
        CreateUseCaseDto {
            name: use_case.name,
            validator: use_case.validator,
            entities: use_case.entities,
            undoable: use_case.undoable,
            dto_in: use_case.dto_in,
            dto_out: use_case.dto_out,
        }
    }
}

pub use common::direct_access::use_case::UseCaseRelationshipField;

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveUseCaseRelationshipsDto {
    pub field: UseCaseRelationshipField,
    pub ids_to_remove: Vec<EntityId>,
}
