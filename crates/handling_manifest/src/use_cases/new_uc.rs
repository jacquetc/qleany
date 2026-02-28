use crate::NewReturnDto;
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::root::RootRelationshipField;
use common::entities::{Entity, Field, FieldType, Global, Root, UserInterface, Workspace};
use common::types::EntityId;

pub trait NewUnitOfWorkFactoryTrait {
    fn create(&self) -> Box<dyn NewUnitOfWorkTrait>;
}
#[macros::uow_action(entity = "Root", action = "Get")]
#[macros::uow_action(entity = "Root", action = "SetRelationship")]
#[macros::uow_action(entity = "Workspace", action = "CreateMulti")]
#[macros::uow_action(entity = "Global", action = "CreateMulti")]
#[macros::uow_action(entity = "Entity", action = "CreateMulti")]
#[macros::uow_action(entity = "Field", action = "CreateMulti")]
#[macros::uow_action(entity = "UserInterface", action = "Create")]
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
                optional: false,
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
                optional: false,
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
                optional: false,
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
            single_model: false,
            only_for_heritage: true,
            fields: created_field_ids,
            relationships: vec![],
            undoable: false,
        };

        let created_entity = uow.create_entity_multi(&[entity_base])?;
        let root_entity = Entity {
            id: 0,
            name: "Root".to_string(),
            inherits_from: Some(created_entity[0].id),
            single_model: false,
            only_for_heritage: false,
            fields: vec![],
            relationships: vec![],
            undoable: false,
        };

        let created_root_entity = uow.create_entity_multi(&[root_entity])?;

        // create global
        let global = Global {
            id: 0,
            language: "rust".to_string(),
            application_name: "My Application".to_string(),
            organisation_name: "MyCompany".to_string(),
            organisation_domain: "eu.mycompany".to_string(),
            prefix_path: "".to_string(),
        };

        let created_global = uow.create_global_multi(&[global])?;

        // create user interface
        let ui = uow.create_user_interface(&UserInterface {
            id: 0,
            rust_cli: false,
            rust_slint: false,
            cpp_qt_qtwidgets: false,
            cpp_qt_qtquick: false,
        })?;

        let workspace = Workspace {
            id: 0,
            manifest_absolute_path: "".to_string(),
            global: created_global[0].id,
            entities: vec![created_entity[0].id, created_root_entity[0].id],
            features: vec![],
            user_interface: ui.id,
        };

        let created_workspace = uow.create_workspace_multi(&[workspace])?;

        let root = uow.get_root(&1)?.ok_or(anyhow!("Root entity not found"))?;

        uow.set_root_relationship(
            &root.id,
            &RootRelationshipField::Workspace,
            &[created_workspace[0].id],
        )?;

        uow.commit()?;
        Ok(NewReturnDto {
            workspace_id: created_workspace[0].id,
        })
    }
}
