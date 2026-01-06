use crate::NewReturnDto;
use anyhow::{anyhow, Result};
use common::database::CommandUnitOfWork;
use common::entities::{Entity, Field, FieldType, Global, Root};
use common::types::EntityId;

pub trait NewUnitOfWorkFactoryTrait {
    fn create(&self) -> Box<dyn NewUnitOfWorkTrait>;
}
#[macros::uow_action(entity = "Root", action = "CreateMulti")]
#[macros::uow_action(entity = "Global", action = "CreateMulti")]
#[macros::uow_action(entity = "Entity", action = "CreateMulti")]
#[macros::uow_action(entity = "Field", action = "CreateMulti")]
pub trait NewUnitOfWorkTrait: CommandUnitOfWork {}

pub struct NewUseCase {
    uow_factory: Box<dyn NewUnitOfWorkFactoryTrait>,
}

impl NewUseCase {
    pub fn new(uow_factory: Box<dyn NewUnitOfWorkFactoryTrait>) -> Self {
        NewUseCase { uow_factory }
    }

    pub fn execute(&mut self) -> Result<NewReturnDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        let fields = vec![
            Field {
                id: 0,
                name: "id".to_string(),
                field_type: FieldType::UInteger,
                entity: None,
                relationship: Default::default(),
                required: false,
                single_model: false,
                strong: false,
                list_model: false,
                list_model_displayed_field: None,
                enum_name: None,
                enum_values: None,
            },
            Field {
                id: 0,
                name: "created_at".to_string(),
                field_type: FieldType::DateTime,
                entity: None,
                relationship: Default::default(),
                required: false,
                single_model: false,
                strong: false,
                list_model: false,
                list_model_displayed_field: None,
                enum_name: None,
                enum_values: None,
            },
            Field {
                id: 0,
                name: "updated_at".to_string(),
                field_type: FieldType::DateTime,
                entity: None,
                relationship: Default::default(),
                required: false,
                single_model: false,
                strong: false,
                list_model: false,
                list_model_displayed_field: None,
                enum_name: None,
                enum_values: None,
            },
        ];

        let created_fields = uow.create_field_multi(&fields)?;
        let created_field_ids: Vec<EntityId> = created_fields.iter().map(|f| f.id).collect();

        let entity_base = Entity {
            id: 0,
            name: "EntityBase".to_string(),
            inherits_from: None,
            only_for_heritage: true,
            fields: created_field_ids,
            relationships: vec![],
            allow_direct_access: false,
        };

        let created_entity = uow.create_entity_multi(&vec![entity_base])?;
        let root_entity = Entity {
            id: 0,
            name: "Root".to_string(),
            inherits_from: Some(created_entity[0].id),
            only_for_heritage: false,
            fields: vec![],
            relationships: vec![],
            allow_direct_access: true,
        };

        let created_root_entity = uow.create_entity_multi(&vec![root_entity])?;

        let global = Global {
            id: 0,
            language: "rust".to_string(),
            application_name: "My Application".to_string(),
            organisation_name: "".to_string(),
            organisation_domain: "".to_string(),
            prefix_path: "".to_string(),
        };

        let created_global = uow.create_global_multi(&vec![global])?;

        let root = Root {
            id: 0,
            manifest_absolute_path: "".to_string(),
            global: created_global[0].id,
            entities: vec![ created_entity[0].id, created_root_entity[0].id],
            features: vec![],
            files: vec![],
        };

        let created_root = uow.create_root_multi(&vec![root])?;

        uow.commit()?;
        Ok(NewReturnDto {
            root_id: created_root[0].id,
        })
    }
}
