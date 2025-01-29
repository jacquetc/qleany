mod model_structs;
mod relationship_generator;
mod validation_schema;

use anyhow::{Result};
use common::{
    database::CommandUnitOfWork,
    entities::{Entity, EntityId, Feature, Field, FieldType, Global, Root, UseCase},
};

use crate::LoadDto;

pub trait LoadUnitOfWorkTrait: CommandUnitOfWork {
    fn create_root(&self, root: &Root) -> Result<Root>;
    fn get_root(&self, id: &EntityId) -> Result<Option<Root>>;
    fn update_root(&self, root: &Root) -> Result<Root>;
    fn create_global(&self, global: &Global) -> Result<Global>;
    fn create_feature(&self, feature: &Feature) -> Result<Feature>;
    fn create_use_case(&self, use_case: &UseCase) -> Result<UseCase>;
    fn create_entity(&self, entity: &Entity) -> Result<Entity>;
    fn get_entity(&self, id: &EntityId) -> Result<Option<Entity>>;
    fn update_entity(&self, entity: &Entity) -> Result<Entity>;
    fn create_field(&self, field: &Field) -> Result<Field>;
}

pub struct LoadUseCase<'a> {
    uow: &'a mut dyn LoadUnitOfWorkTrait,
}

impl<'a> LoadUseCase<'a> {
    pub fn new(uow: &'a mut dyn LoadUnitOfWorkTrait) -> Self {
        LoadUseCase { uow }
    }

    pub fn execute(&mut self, dto: &LoadDto) -> Result<()> {
        // load file
        let path = &dto.manifest_path;

        // validate that the file exists
        if !std::path::Path::new(path).exists() {
            return Err(anyhow::anyhow!("File does not exist"));
        }
        // readable ?
        if !std::path::Path::new(path).is_file() {
            return Err(anyhow::anyhow!("File is not a file"));
        }

        // if yaml file, convert to json

        let json_value: serde_json::Value = match path.split('.').last() {
            Some("yaml") => {
                let yaml = std::fs::read_to_string(path)?;
                serde_yml::from_str(&yaml)?
            }
            Some("json") => {
                let json = std::fs::read_to_string(path)?;
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

        self.uow.begin_transaction()?;

        let result: Result<()> = {

            // create global
            let global = self.uow.create_global(&Global {
                id: 0,
                language: manifest.global.language,
                application_name: manifest.global.application_name,
                organisation_name: manifest.global.organisation.name,
                organisation_domain: manifest.global.organisation.domain,
                prefix_path: manifest.global.prefix_path,
            })?;
            let global_id = global.id;

            // create root
            let root = self.uow.create_root(&Root {
                id: 0,
                global: global_id,
                entities: vec![],
                features: vec![],
            })?;
            let root_id = root.id;

            // create entities
            let mut entity_ids = vec![];
            let mut entities = vec![];
            for model_entity in manifest.entities.iter() {
                let entity = self.uow.create_entity(&Entity {
                    id: 0,
                    name: model_entity.name.clone(),
                    only_for_heritage: model_entity.only_for_heritage.unwrap_or_default(),
                    parent: None,          // will be filled in later
                    fields: vec![],        // will be filled in later
                    relationships: vec![], // will be filled in later
                })?;
                entity_ids.push(entity.id);
                entities.push(entity);
            }

            // create fields
            for model_entity in manifest.entities.iter() {
                // find the entity id
                let entity_id = entities
                    .iter()
                    .find(|e| e.name == model_entity.name)
                    .map(|e| e.id)
                    .expect("Entity not found");

                let mut field_ids = vec![];

                for model_field in model_entity.fields.iter() {
                    // determine if model_field.type is another entity name
                    let field_entity_id = entities
                        .iter()
                        .find(|e| e.name == model_field.r#type)
                        .map(|e| e.id);
                    let field_type = match field_entity_id{
                        Some(_id) => FieldType::Entity,
                        None => match model_field.r#type.clone().as_str() {
                            "Boolean"|"Bool" => FieldType::Boolean,
                            "Integer" => FieldType::Integer,
                            "UInteger" => FieldType::UInteger,
                            "Float" => FieldType::Float,
                            "String" => FieldType::String,
                            "Uuid" => FieldType::Uuid,
                            "DateTime" => FieldType::DateTime,
                            _ => FieldType::String,
                        },
                    };
                    

                    // create field
                    let field = self.uow.create_field(&Field {
                        id: 0,
                        name: model_field.name.clone(),
                        field_type: field_type,
                        entity: field_entity_id,
                        is_nullable: model_field.is_nullable.unwrap_or_default(),
                        is_primary_key: model_field.is_primary_key.unwrap_or_default(),
                        is_list: model_field.is_list.unwrap_or_default(),
                        single: model_field.single.unwrap_or_default(),
                        strong: model_field.strong.unwrap_or_default(),
                        ordered: model_field.ordered.unwrap_or_default(),
                        list_model: model_field.list_model.unwrap_or_default(),
                        list_model_displayed_field: model_field.list_model_displayed_field.clone(),
                    })?;
                    field_ids.push(field.id);
                }

                // get entity from repo
                let mut entity = self.uow.get_entity(&entity_id)?.expect("Entity not found");

                // update entity with fields
                entity.fields = field_ids;

                // update entity with parent

                if let Some(parent_name) = model_entity.parent.clone() {
                    let parent_id = entities
                        .iter()
                        .find(|e| e.name == parent_name)
                        .map(|e| e.id)
                        .expect("Parent not found");
                    entity.parent = Some(parent_id);
                }

                // update entity in repo
                self.uow.update_entity(&entity)?;
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

                    let use_case = self.uow.create_use_case(&UseCase {
                        id: 0,
                        name: model_use_case.name.clone(),
                        validator: model_use_case.validator,
                        entities: use_case_entity_ids,
                        undoable: model_use_case.undoable,
                        dto_in: None,
                        dto_out: None,
                    })?;
                    use_case_ids.push(use_case.id);
                }

                let feature = self.uow.create_feature(&Feature {
                    id: 0,
                    name: model_feature.name.clone(),
                    use_cases: use_case_ids,
                })?;
                feature_ids.push(feature.id);
            }

            // update root with entities
            // good practice to get the root again, to make sure it is not stale
            let root = self.uow.get_root(&root_id)?.expect("Root not found");
            let root = Root {
                id: root.id,
                global: global_id,
                entities: entity_ids,
                features: feature_ids,
            };
            self.uow.update_root(&root)?;

            Ok(())
        };
        if let Err(e) = result {
            self.uow.rollback()?;
            return Err(e);
        }

        self.uow.commit()?;

        Ok(())
    }
}
