mod model_structs;
mod tools;
mod validation_schema;

use anyhow::Result;
use common::types::EntityId;
use common::{
    database::CommandUnitOfWork,
    entities::{
        Dto, DtoField, Entity, Feature, Field, FieldType, Global, Relationship, Root, UseCase,
    },
};

use crate::LoadDto;

pub trait LoadUnitOfWorkFactoryTrait {
    fn create(&self) -> Box<dyn LoadUnitOfWorkTrait>;
}

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
    fn get_fields(&self, ids: &[EntityId]) -> Result<Vec<Option<Field>>>;
    fn create_dto(&self, dto: &Dto) -> Result<Dto>;
    fn create_dto_field(&self, dto_field: &DtoField) -> Result<DtoField>;
    fn create_relationships(&self, relationships: &[Relationship]) -> Result<Vec<Relationship>>;
}

pub struct LoadUseCase {
    uow_factory: Box<dyn LoadUnitOfWorkFactoryTrait>,
}

impl LoadUseCase {
    pub fn new(uow_factory: Box<dyn LoadUnitOfWorkFactoryTrait>) -> Self {
        LoadUseCase { uow_factory }
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

        let mut uow = self.uow_factory.create();

        uow.begin_transaction()?;

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

        // create root
        let root = uow.create_root(&Root {
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
            let entity = uow.create_entity(&Entity {
                id: 0,
                name: model_entity.name.clone(),
                only_for_heritage: model_entity.only_for_heritage.unwrap_or_default(),
                parent: None, // will be filled in later
                allow_direct_access: model_entity.only_for_heritage.unwrap_or_else(|| true),
                fields: vec![],        // will be filled in later
                relationships: vec![], // will be filled in later
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

            let mut field_ids = vec![];

            for model_field in model_entity.fields.iter() {
                // determine if model_field.type is another entity name
                let field_entity_id = entities
                    .iter()
                    .find(|e| e.name == model_field.r#type)
                    .map(|e| e.id);
                let field_type = match field_entity_id {
                    Some(_id) => FieldType::Entity,
                    None => tools::str_to_field_type(&model_field.r#type),
                };

                // create field
                let field = uow.create_field(&Field {
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
                all_field_ids.push(field.id);
            }

            // get entity from repo
            let mut entity = uow
                .get_entity(&entity_id)?
                .ok_or(anyhow::anyhow!("Entity not found"))?;

            // update entity with fields
            entity.fields = field_ids;

            // update entity with parent

            if let Some(parent_name) = model_entity.parent.clone() {
                let parent_id = entities
                    .iter()
                    .find(|e| e.name == parent_name)
                    .map(|e| e.id)
                    .ok_or(anyhow::anyhow!("Parent not found"))?;
                entity.parent = Some(parent_id);
            }

            // update entity in repo
            uow.update_entity(&entity)?;
        }

        // create relationships
        let all_fields = uow.get_fields(&all_field_ids)?;
        let all_fields = all_fields.into_iter().flatten().collect::<Vec<Field>>();
        let all_relationships = tools::generate_relationships(&entities, &all_fields);

        for (entity_id, relationships) in all_relationships.iter() {
            let new_relationship_ids = uow
                .create_relationships(relationships)?
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
                            field_type: field_type,
                            is_nullable: model_dto_field.is_nullable.unwrap_or_default(),
                            is_list: model_dto_field.is_list.unwrap_or_default(),
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
                            field_type: field_type,
                            is_nullable: model_dto_field.is_nullable.unwrap_or_default(),
                            is_list: model_dto_field.is_list.unwrap_or_default(),
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
                    validator: model_use_case.validator,
                    entities: use_case_entity_ids,
                    undoable: model_use_case.undoable,
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

        // update root with all ids
        // good practice to get the root again, to make sure it is not stale
        let root = uow
            .get_root(&root_id)?
            .ok_or(anyhow::anyhow!("Root not found"))?;
        let root = Root {
            id: root.id,
            global: global_id,
            entities: entity_ids,
            features: feature_ids,
        };
        uow.update_root(&root)?;

        uow.commit()?;

        Ok(())
    }
}
