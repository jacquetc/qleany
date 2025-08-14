use crate::use_cases::common::rust_code_generator::generate_code;
use crate::{GenerateRustCodeDto, GenerateRustCodeReturnDto};
use anyhow::{Result, anyhow};
use common::entities::DtoField;
use common::entities::Entity;
use common::entities::Feature;
use common::entities::Field;
use common::entities::UseCase;
use common::entities::{Dto, FieldType};
use common::types::EntityId;
use common::{database::QueryUnitOfWork, entities::File};
use std::collections::HashMap;

pub trait GenerateRustCodeUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn GenerateRustCodeUnitOfWorkTrait>;
}

#[macros::uow_action(entity = "Root", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "File", action = "GetRO")]
#[macros::uow_action(entity = "Feature", action = "GetRO")]
#[macros::uow_action(entity = "UseCase", action = "GetRO")]
#[macros::uow_action(entity = "UseCase", action = "GetMultiRO")]
#[macros::uow_action(entity = "Dto", action = "GetRO")]
#[macros::uow_action(entity = "DtoField", action = "GetRO")]
#[macros::uow_action(entity = "DtoField", action = "GetMultiRO")]
#[macros::uow_action(entity = "Entity", action = "GetRO")]
#[macros::uow_action(entity = "Entity", action = "GetMultiRO")]
#[macros::uow_action(entity = "Field", action = "GetRO")]
#[macros::uow_action(entity = "Field", action = "GetMultiRO")]
pub trait GenerateRustCodeUnitOfWorkTrait: QueryUnitOfWork {}

pub struct GenerateRustCodeUseCase {
    uow_factory: Box<dyn GenerateRustCodeUnitOfWorkFactoryTrait>,
}

impl GenerateRustCodeUseCase {
    pub fn new(uow_factory: Box<dyn GenerateRustCodeUnitOfWorkFactoryTrait>) -> Self {
        GenerateRustCodeUseCase { uow_factory }
    }
}
impl GenerateRustCodeUseCase {
    pub(crate) fn execute(&self, dto: &GenerateRustCodeDto) -> Result<GenerateRustCodeReturnDto> {
        let timestamp = chrono::Utc::now();

        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let file = uow.get_file(&dto.file_id)?;
        if file.is_none() {
            return Err(anyhow!("File not found"));
        }

        let file = file.unwrap();

        let mut dto_fields: HashMap<EntityId, DtoField> = HashMap::new();
        let mut dtos: HashMap<EntityId, Dto> = HashMap::new();
        let mut use_cases: HashMap<EntityId, UseCase> = HashMap::new();
        let mut entities: HashMap<EntityId, Entity> = HashMap::new();
        let mut fields: HashMap<EntityId, Field> = HashMap::new();
        let mut features: HashMap<EntityId, Feature> = HashMap::new();

        // feature

        if let Some(feature) = file.feature {
            let feature = uow.get_feature(&feature)?;
            if feature.is_none() {
                return Err(anyhow!("Feature not found"));
            }
            let feature = feature.unwrap();
            if feature.use_cases.is_empty() {
                return Err(anyhow!("Feature does not have an associated use case"));
            }
            let feature_use_cases: Vec<UseCase> = uow
                .get_use_case_multi(&feature.use_cases)?
                .iter()
                .filter(|uc| uc.is_some())
                .map(|uc| uc.clone().unwrap())
                .collect();

            features.insert(feature.id, feature);

            for use_case in feature_use_cases {
                let use_case_entities: Vec<Entity> = uow
                    .get_entity_multi(&use_case.entities)?
                    .iter()
                    .filter(|e| e.is_some())
                    .map(|e| e.clone().unwrap())
                    .collect();

                if let Some(dto_in) = use_case.dto_in {
                    let dto = uow.get_dto(&dto_in)?;
                    if dto.is_none() {
                        return Err(anyhow!("DTO missing for use case"));
                    }
                    let dto: Dto = dto.unwrap();

                    let fields: Vec<DtoField> = uow
                        .get_dto_field_multi(&dto.fields)?
                        .iter()
                        .filter(|f| f.is_some())
                        .map(|f| f.clone().unwrap())
                        .collect();
                    for field in &fields {
                        dto_fields.insert(field.id, field.clone());
                    }
                    dtos.insert(dto.id, dto);
                }

                if let Some(dto_out) = use_case.dto_out {
                    let dto = uow.get_dto(&dto_out)?;
                    if dto.is_none() {
                        return Err(anyhow!("DTO missing for use case"));
                    }
                    let dto: Dto = dto.unwrap();

                    let fields: Vec<DtoField> = uow
                        .get_dto_field_multi(&dto.fields)?
                        .iter()
                        .filter(|f| f.is_some())
                        .map(|f| f.clone().unwrap())
                        .collect();
                    for field in &fields {
                        dto_fields.insert(field.id, field.clone());
                    }
                    dtos.insert(dto.id, dto);
                }

                use_cases.insert(use_case.id, use_case);
            }
        }

        // use cases
        if let Some(use_case) = file.use_case {
            let use_case = uow.get_use_case(&use_case)?;
            if use_case.is_none() {
                return Err(anyhow!("Use case not found"));
            }
            let use_case = use_case.unwrap();

            let use_case_entities: Vec<Entity> = uow
                .get_entity_multi(&use_case.entities)?
                .iter()
                .filter(|e| e.is_some())
                .map(|e| e.clone().unwrap())
                .collect();

            if let Some(dto_in) = use_case.dto_in {
                let dto = uow.get_dto(&dto_in)?;
                if dto.is_none() {
                    return Err(anyhow!("DTO missing for use case"));
                }
                let dto: Dto = dto.unwrap();

                let fields: Vec<DtoField> = uow
                    .get_dto_field_multi(&dto.fields)?
                    .iter()
                    .filter(|f| f.is_some())
                    .map(|f| f.clone().unwrap())
                    .collect();
                for field in &fields {
                    dto_fields.insert(field.id, field.clone());
                }
                dtos.insert(dto.id, dto);
            }

            if let Some(dto_out) = use_case.dto_out {
                let dto = uow.get_dto(&dto_out)?;
                if dto.is_none() {
                    return Err(anyhow!("DTO missing for use case"));
                }
                let dto: Dto = dto.unwrap();

                let fields: Vec<DtoField> = uow
                    .get_dto_field_multi(&dto.fields)?
                    .iter()
                    .filter(|f| f.is_some())
                    .map(|f| f.clone().unwrap())
                    .collect();
                for field in &fields {
                    dto_fields.insert(field.id, field.clone());
                }
                dtos.insert(dto.id, dto);
            }
            use_cases.insert(use_case.id, use_case);
        }

        if let Some(entity) = file.entity {
            let entity = uow.get_entity(&entity)?;
            if entity.is_none() {
                return Err(anyhow!("Entity not found"));
            }
            let entity = entity.unwrap();

            let entity_fields: Vec<Field> = uow
                .get_field_multi(&entity.fields)?
                .iter()
                .filter(|f| f.is_some())
                .map(|f| f.clone().unwrap())
                .collect();

            entities.insert(entity.id, entity);
            for field in &entity_fields {
                if let Some(entity) = field.entity
                    && field.field_type == FieldType::Entity
                {
                    let entity = uow.get_entity(&entity)?;
                    if entity.is_none() {
                        return Err(anyhow!("Entity not found"));
                    }
                    let entity = entity.unwrap();
                    entities.insert(entity.id, entity);
                }

                fields.insert(field.id, field.clone());
            }
        }

        uow.end_transaction()?;

        let generated_code = generate_code(
            file, entities, fields, features, use_cases, dtos, dto_fields,
        )?;

        Ok(GenerateRustCodeReturnDto {
            generated_code,
            timestamp: timestamp.to_string(),
        })
    }
}
