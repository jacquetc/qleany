mod tools;
mod validation_schema;
use crate::use_cases::common::model_structs;
use crate::{LoadDto, LoadReturnDto};
use anyhow::Result;
use common::types::EntityId;
use common::{
    database::CommandUnitOfWork,
    entities::{
        Dto, DtoField, Entity, Feature, Field, FieldRelationshipType, FieldType, Global,
        Relationship, RelationshipType, Root, System, UseCase, UserInterface, Workspace,
    },
};

pub trait LoadUnitOfWorkFactoryTrait {
    fn create(&self) -> Box<dyn LoadUnitOfWorkTrait>;
}

#[macros::uow_action(entity = "Root", action = "Create")]
#[macros::uow_action(entity = "Root", action = "Get")]
#[macros::uow_action(entity = "Root", action = "Update")]
#[macros::uow_action(entity = "Workspace", action = "Create")]
#[macros::uow_action(entity = "Workspace", action = "Get")]
#[macros::uow_action(entity = "Workspace", action = "Update")]
#[macros::uow_action(entity = "System", action = "Create")]
#[macros::uow_action(entity = "System", action = "Get")]
#[macros::uow_action(entity = "System", action = "Update")]
#[macros::uow_action(entity = "Global", action = "Create")]
#[macros::uow_action(entity = "Feature", action = "Create")]
#[macros::uow_action(entity = "UseCase", action = "Create")]
#[macros::uow_action(entity = "Entity", action = "Create")]
#[macros::uow_action(entity = "Entity", action = "Get")]
#[macros::uow_action(entity = "Entity", action = "Update")]
#[macros::uow_action(entity = "Field", action = "Create")]
#[macros::uow_action(entity = "Field", action = "GetMulti")]
#[macros::uow_action(entity = "Dto", action = "Create")]
#[macros::uow_action(entity = "DtoField", action = "Create")]
#[macros::uow_action(entity = "Relationship", action = "CreateMulti")]
#[macros::uow_action(entity = "UserInterface", action = "Create")]
pub trait LoadUnitOfWorkTrait: CommandUnitOfWork {}

pub struct LoadUseCase {
    uow_factory: Box<dyn LoadUnitOfWorkFactoryTrait>,
}

impl LoadUseCase {
    pub fn new(uow_factory: Box<dyn LoadUnitOfWorkFactoryTrait>) -> Self {
        LoadUseCase { uow_factory }
    }

    pub fn execute(&mut self, dto: &LoadDto) -> Result<LoadReturnDto> {
        // load file
        let path = &dto.manifest_path;
        let filename = path.clone();

        // validate that the file exists
        if !std::path::Path::new(path).exists() {
            return Err(anyhow::anyhow!("File does not exist"));
        }
        // readable ?
        if !std::path::Path::new(path).is_file() {
            return Err(anyhow::anyhow!("File is not a file"));
        }

        // ensure that the path is absolute
        let path = if std::path::Path::new(path).is_absolute() {
            std::path::Path::new(path)
                .parent()
                .map_or(
                    Err(anyhow::anyhow!(
                        "Failed to get parent directory of the manifest path"
                    )),
                    |p| Ok(p),
                )?
                .to_string_lossy()
                .to_string()
        } else {
            std::env::current_dir()
                .map_err(|e| anyhow::anyhow!("Failed to get current directory: {}", e))?
                .join(path)
                .parent()
                .map_or(
                    Err(anyhow::anyhow!(
                        "Failed to get parent directory of the manifest path"
                    )),
                    |p| Ok(p.to_path_buf()),
                )?
                .to_string_lossy()
                .to_string()
        };

        // if yaml file, convert to json

        let json_value: serde_json::Value = match filename.split('.').last() {
            Some("yaml") => {
                let yaml = std::fs::read_to_string(&filename)?;
                serde_yml::from_str(&yaml)?
            }
            Some("json") => {
                let json = std::fs::read_to_string(&filename)?;
                serde_json::from_str(&json)?
            }
            _ => return Err(anyhow::anyhow!("File extension not supported")),
        };

        //let json = serde_json::to_string_pretty(&json_value);

        // validate Json schema

        let validation_schema = validation_schema::json_validation_schema();
        let validator = jsonschema::draft7::new(&validation_schema)?;
        let validation_result = validator.validate(&json_value);
        if let Err(e) = validation_result {
            return Err(anyhow::anyhow!("Json schema validation failed: {}", e));
        }

        // apply the json to the model
        let manifest: model_structs::Manifest = serde_json::from_value(json_value)?;

        let mut uow = self.uow_factory.create();

        uow.begin_transaction()?;

        let root_id = 1;

        // create global
        let global = uow.create_global(&Global {
            id: 0,
            language: manifest.global.language,
            application_name: manifest.global.application_name,
            organisation_name: manifest.global.organisation.name,
            organisation_domain: manifest.global.organisation.domain,
            prefix_path: manifest.global.prefix_path,
        })?;
        let global_id = global.id;

        // create user interface
        let ui = uow.create_user_interface(&UserInterface {
            id: 0,
            rust_cli: manifest.ui.rust_cli,
            rust_slint: manifest.ui.rust_slint,
            cpp_qt_qtwidgets: manifest.ui.cpp_qt_qtwidgets,
            cpp_qt_qtquick: manifest.ui.cpp_qt_qtquick,
            cpp_qt_kirigami: manifest.ui.cpp_qt_kirigami,
        })?;

        // create workspace
        let workspace = uow.create_workspace(&Workspace {
            id: 0,
            manifest_absolute_path: path,
            global: global_id,
            entities: vec![],
            features: vec![],
            user_interface: ui.id,
        })?;
        let workspace_id = workspace.id;

        // create entities
        let mut entity_ids: Vec<EntityId> = vec![];
        let mut entities = vec![];
        for model_entity in manifest.entities.iter() {
            let entity = uow.create_entity(&Entity {
                id: 0,
                name: model_entity.name.clone(),
                only_for_heritage: model_entity.only_for_heritage.unwrap_or_default(),
                single_model: model_entity.single_model.unwrap_or_default(),
                inherits_from: None, // will be filled in later
                allow_direct_access: model_entity.allow_direct_access,
                fields: vec![],        // will be filled in later
                relationships: vec![], // will be filled in later
                undoable: model_entity.undoable,
            })?;
            entity_ids.push(entity.id);
            entities.push(entity);
        }

        // create fields
        let mut all_field_ids = vec![];

        for model_entity in manifest.entities.iter() {
            // find the entity id
            let entity_id = entities
                .iter()
                .find(|e| e.name == model_entity.name)
                .map(|e| e.id)
                .ok_or(anyhow::anyhow!("Entity not found"))?;

            let mut field_ids: Vec<EntityId> = vec![];

            for model_field in model_entity.fields.iter() {
                let field_type = tools::str_to_field_type(&model_field.r#type);

                // validate field type
                if (field_type == FieldType::Entity) && model_field.entity.is_none() {
                    return Err(anyhow::anyhow!(
                        "Field {} is of type Entity but no entity name is provided",
                        model_field.name
                    ));
                }
                // validate field type against existing entities
                if field_type == FieldType::Entity {
                    let entity_name = model_field.entity.as_ref().unwrap();
                    if !entities.iter().any(|e| e.name == *entity_name) {
                        return Err(anyhow::anyhow!(
                            "Entity {} not found for field {}",
                            entity_name,
                            model_field.name
                        ));
                    }
                }

                // parse relationship type from string
                let relationship = match model_field.relationship.as_deref() {
                    Some("one_to_one") => FieldRelationshipType::OneToOne,
                    Some("many_to_one") => FieldRelationshipType::ManyToOne,
                    Some("one_to_many") => FieldRelationshipType::OneToMany,
                    Some("ordered_one_to_many") => FieldRelationshipType::OrderedOneToMany,
                    Some("many_to_many") => FieldRelationshipType::ManyToMany,
                    Some(other) => {
                        return Err(anyhow::anyhow!("Unknown relationship type: {}", other));
                    }
                    None => FieldRelationshipType::OneToOne, // default for entity fields
                };

                // create field
                let field = uow.create_field(&Field {
                    id: 0,
                    name: model_field.name.clone(),
                    field_type,
                    entity: model_field
                        .entity
                        .clone()
                        .map(|e| {
                            entities
                                .iter()
                                .find(|entity| entity.name == e)
                                .map(|entity| entity.id)
                                .ok_or(anyhow::anyhow!("Entity not found"))
                        })
                        .transpose()?,
                    relationship,
                    required: model_field.required.unwrap_or_default(),
                    strong: model_field.strong.unwrap_or_default(),
                    list_model: model_field.list_model.unwrap_or_default(),
                    list_model_displayed_field: model_field.list_model_displayed_field.clone(),
                    enum_name: model_field.enum_name.clone(),
                    enum_values: model_field.enum_values.clone(),
                })?;
                field_ids.push(field.id);
                all_field_ids.push(field.id);
            }

            // get entity from repo
            let mut entity = uow
                .get_entity(&entity_id)?
                .ok_or(anyhow::anyhow!("Entity not found"))?;

            // update entity with fields
            entity.fields = field_ids;

            // update entity with parent

            if let Some(parent_name) = model_entity.inherits_from.clone() {
                let parent_id = entities
                    .iter()
                    .find(|e| e.name == parent_name)
                    .map(|e| e.id)
                    .ok_or(anyhow::anyhow!("Parent not found"))?;
                entity.inherits_from = Some(parent_id);
            }

            // update entity in entities
            let entity_index = entities
                .iter()
                .position(|e| e.id == entity_id)
                .ok_or(anyhow::anyhow!("Entity not found in entities"))?;
            entities[entity_index] = entity.clone();

            // update entity in repo
            uow.update_entity(&entity)?;
        }

        // create relationships
        let all_fields = uow.get_field_multi(&all_field_ids)?;
        let all_fields = all_fields.into_iter().flatten().collect::<Vec<Field>>();
        let all_relationships = tools::generate_relationships(&entities, &all_fields);

        for (entity_id, relationships) in all_relationships.iter() {
            let new_relationship_ids = uow
                .create_relationship_multi(relationships)?
                .iter()
                .map(|new_relationship| new_relationship.id)
                .collect::<Vec<EntityId>>();

            let mut entity = uow
                .get_entity(entity_id)?
                .ok_or(anyhow::anyhow!("Entity not found"))?;

            entity.relationships = new_relationship_ids.clone();

            uow.update_entity(&entity)?;
        }

        // create features
        let mut feature_ids = vec![];
        for model_feature in manifest.features.iter() {
            // create use cases
            let mut use_case_ids = vec![];
            for model_use_case in model_feature.use_cases.iter() {
                // match entity names to ids
                let use_case_entity_names = model_use_case.entities.clone();
                let mut use_case_entity_ids = vec![];
                if let Some(use_case_entity_names) = use_case_entity_names {
                    for entity in entities.iter() {
                        if use_case_entity_names.contains(&entity.name) {
                            use_case_entity_ids.push(entity.id);
                        }
                    }
                    // error if not all entities found
                    if use_case_entity_ids.len() != use_case_entity_names.len() {
                        return Err(anyhow::anyhow!(
                            "Not all entities found in use case {}",
                            model_use_case.name
                        ));
                    }
                }

                // create dto_in
                let mut dto_in_id = None;
                if let Some(dto_in) = &model_use_case.dto_in {
                    // create DtoFields
                    let mut dto_field_ids = vec![];
                    for model_dto_field in dto_in.fields.iter() {
                        let field_type = tools::str_to_dto_field_type(&model_dto_field.r#type);

                        let dto_field = uow.create_dto_field(&DtoField {
                            id: 0,
                            name: model_dto_field.name.clone(),
                            field_type,
                            is_nullable: model_dto_field.is_nullable.unwrap_or_default(),
                            is_list: model_dto_field.is_list.unwrap_or_default(),
                            enum_name: model_dto_field.enum_name.clone(),
                            enum_values: model_dto_field.enum_values.clone(),
                        })?;
                        dto_field_ids.push(dto_field.id);
                    }

                    let dto_in = uow.create_dto(&Dto {
                        id: 0,
                        name: dto_in.name.clone(),
                        fields: dto_field_ids,
                    })?;
                    dto_in_id = Some(dto_in.id);
                }

                // create dto_out
                let mut dto_out_id = None;
                if let Some(dto_out) = &model_use_case.dto_out {
                    // create DtoFields
                    let mut dto_field_ids = vec![];
                    for model_dto_field in dto_out.fields.iter() {
                        let field_type = tools::str_to_dto_field_type(&model_dto_field.r#type);

                        let dto_field = uow.create_dto_field(&DtoField {
                            id: 0,
                            name: model_dto_field.name.clone(),
                            field_type,
                            is_nullable: model_dto_field.is_nullable.unwrap_or_default(),
                            is_list: model_dto_field.is_list.unwrap_or_default(),
                            enum_name: model_dto_field.enum_name.clone(),
                            enum_values: model_dto_field.enum_values.clone(),
                        })?;
                        dto_field_ids.push(dto_field.id);
                    }

                    let dto_out = uow.create_dto(&Dto {
                        id: 0,
                        name: dto_out.name.clone(),
                        fields: dto_field_ids,
                    })?;
                    dto_out_id = Some(dto_out.id);
                }

                let use_case = uow.create_use_case(&UseCase {
                    id: 0,
                    name: model_use_case.name.clone(),
                    validator: model_use_case.validator.unwrap_or_default(),
                    entities: use_case_entity_ids,
                    undoable: model_use_case.undoable.unwrap_or_default(),
                    read_only: model_use_case.read_only.unwrap_or_default(),
                    long_operation: model_use_case.long_operation.unwrap_or_default(),
                    dto_in: dto_in_id,
                    dto_out: dto_out_id,
                })?;
                use_case_ids.push(use_case.id);
            }

            let feature = uow.create_feature(&Feature {
                id: 0,
                name: model_feature.name.clone(),
                use_cases: use_case_ids,
            })?;
            feature_ids.push(feature.id);
        }

        // update workspace with all ids
        // good practice to get the workspace again, to make sure it is not stale
        let workspace = uow
            .get_workspace(&workspace_id)?
            .ok_or(anyhow::anyhow!("Workspace not found"))?;
        let workspace = common::entities::Workspace {
            id: workspace.id,
            entities: entity_ids,
            features: feature_ids.clone(),
            ..workspace
        };
        uow.update_workspace(&workspace)?;

        // update root with all ids
        // good practice to get the root again, to make sure it is not stale
        let root = uow
            .get_root(&root_id)?
            .ok_or(anyhow::anyhow!("Root not found"))?;
        let root = Root {
            id: root.id,
            workspace: Some(workspace_id),
            system: root.system,
        };
        uow.update_root(&root)?;

        uow.commit()?;

        Ok(LoadReturnDto { workspace_id, manifest_path: filename } )
    }
}
