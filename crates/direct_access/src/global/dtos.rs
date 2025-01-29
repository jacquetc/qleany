use std::convert::From;

use common::entities::Global;
use common::entities::EntityId;

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalDto {
    pub id: EntityId,
    pub language: String,
    pub application_name: String,
    pub organisation_name: String,
    pub organisation_domain: String,
    pub prefix_path: String,
}

impl From<GlobalDto> for Global {
    fn from(global_dto: GlobalDto) -> Self {
        Global {
            id: global_dto.id,
            language: global_dto.language,
            application_name: global_dto.application_name,
            organisation_name: global_dto.organisation_name,
            organisation_domain: global_dto.organisation_domain,
            prefix_path: global_dto.prefix_path,
        }
    }
}

impl From<&GlobalDto> for Global {
    fn from(global_dto: &GlobalDto) -> Self {
        Global {
            id: global_dto.id,
            language: global_dto.language.clone(),
            application_name: global_dto.application_name.clone(),
            organisation_name: global_dto.organisation_name.clone(),
            organisation_domain: global_dto.organisation_domain.clone(),
            prefix_path: global_dto.prefix_path.clone(),
        }
    }
}

impl From<Global> for GlobalDto {
    fn from(global: Global) -> Self {
        GlobalDto {
            id: global.id,
            language: global.language,
            application_name: global.application_name,
            organisation_name: global.organisation_name,
            organisation_domain: global.organisation_domain,
            prefix_path: global.prefix_path,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateGlobalDto {
    pub language: String,
    pub application_name: String,
    pub organisation_name: String,
    pub organisation_domain: String,
    pub prefix_path: String,
}

impl From<CreateGlobalDto> for Global {
    fn from(create_global_dto: CreateGlobalDto) -> Self {
        Global {
            id: 0,
            language: create_global_dto.language,
            application_name: create_global_dto.application_name,
            organisation_name: create_global_dto.organisation_name,
            organisation_domain: create_global_dto.organisation_domain,
            prefix_path: create_global_dto.prefix_path,
        }
    }
}

impl From<&CreateGlobalDto> for Global {
    fn from(create_global_dto: &CreateGlobalDto) -> Self {
        Global {
            id: 0,
            language: create_global_dto.language.clone(),
            application_name: create_global_dto.application_name.clone(),
            organisation_name: create_global_dto.organisation_name.clone(),
            organisation_domain: create_global_dto.organisation_domain.clone(),
            prefix_path: create_global_dto.prefix_path.clone(),
        }
    }
}

impl From<Global> for CreateGlobalDto {
    fn from(global: Global) -> Self {
        CreateGlobalDto {
            language: global.language,
            application_name: global.application_name,
            organisation_name: global.organisation_name,
            organisation_domain: global.organisation_domain,
            prefix_path: global.prefix_path,
        }
    }
}
