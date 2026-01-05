use crate::SaveDto;
use crate::use_cases::common::model_structs;
use anyhow::Result;
use common::database::QueryUnitOfWork;
use common::entities::{
    Dto, DtoField, Entity, Feature, Field, Global, RelationshipType, Root, UseCase,
};
use common::types::EntityId;

pub trait SaveUnitOfWorkFactoryTrait {
    fn create(&self) -> Box<dyn SaveUnitOfWorkTrait>;
}

#[macros::uow_action(entity = "Root", action = "GetMultiRO")]
#[macros::uow_action(entity = "Root", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Global", action = "GetRO")]
#[macros::uow_action(entity = "Feature", action = "GetMultiRO")]
#[macros::uow_action(entity = "Feature", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "UseCase", action = "GetMultiRO")]
#[macros::uow_action(entity = "UseCase", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Entity", action = "GetMultiRO")]
#[macros::uow_action(entity = "Entity", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Field", action = "GetMultiRO")]
#[macros::uow_action(entity = "Field", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Dto", action = "GetMultiRO")]
#[macros::uow_action(entity = "Dto", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "DtoField", action = "GetMultiRO")]
pub trait SaveUnitOfWorkTrait: QueryUnitOfWork {}

pub struct SaveUseCase {
    uow_factory: Box<dyn SaveUnitOfWorkFactoryTrait>,
}
//TODO: add Ui
impl SaveUseCase {
    pub fn new(uow_factory: Box<dyn SaveUnitOfWorkFactoryTrait>) -> Self {
        SaveUseCase { uow_factory }
    }

    pub fn execute(&mut self, dto: &SaveDto) -> Result<()> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // Get all roots
        let roots = uow.get_root_multi(&[])?;
        if roots.is_empty() {
            return Err(anyhow::anyhow!("No root found"));
        }
        let root = &roots[0].as_ref().ok_or(anyhow::anyhow!("Root is None"))?;

        // Get global
        let global = uow
            .get_global(&root.global)?
            .ok_or(anyhow::anyhow!("Global not found"))?;

        // Get entities
        let entities = uow.get_entity_multi(&root.entities)?;
        let entities = entities.into_iter().flatten().collect::<Vec<Entity>>();

        // Get features
        let features = uow.get_feature_multi(&root.features)?;
        let features = features.into_iter().flatten().collect::<Vec<Feature>>();

        // Get all fields
        let field_ids = entities
            .iter()
            .flat_map(|e| e.fields.clone())
            .collect::<Vec<EntityId>>();
        let fields = uow.get_field_multi(&field_ids)?;
        let fields = fields.into_iter().flatten().collect::<Vec<Field>>();

        // Get all use cases
        let use_case_ids = features
            .iter()
            .flat_map(|f| f.use_cases.clone())
            .collect::<Vec<EntityId>>();
        let use_cases = uow.get_use_case_multi(&use_case_ids)?;
        let use_cases = use_cases.into_iter().flatten().collect::<Vec<UseCase>>();

        // Get all DTOs
        let dto_ids = use_cases
            .iter()
            .flat_map(|uc| {
                let mut ids = Vec::new();
                if let Some(id) = uc.dto_in {
                    ids.push(id);
                }
                if let Some(id) = uc.dto_out {
                    ids.push(id);
                }
                ids
            })
            .collect::<Vec<EntityId>>();
        let dtos = uow.get_dto_multi(&dto_ids)?;
        let dtos = dtos.into_iter().flatten().collect::<Vec<Dto>>();

        // Get all DTO fields
        let dto_field_ids = dtos
            .iter()
            .flat_map(|d| d.fields.clone())
            .collect::<Vec<EntityId>>();
        let dto_fields = uow.get_dto_field_multi(&dto_field_ids)?;
        let dto_fields = dto_fields.into_iter().flatten().collect::<Vec<DtoField>>();

        uow.end_transaction()?;

        // Create model structs
        let model_global = model_structs::Global {
            language: global.language.clone(),
            application_name: global.application_name.clone(),
            organisation: model_structs::Organisation {
                name: global.organisation_name.clone(),
                domain: global.organisation_domain.clone(),
            },
            prefix_path: global.prefix_path.clone(),
        };

        let model_entities = entities
            .iter()
            .map(|entity| {
                let parent = entity.parent.and_then(|parent_id| {
                    entities
                        .iter()
                        .find(|e| e.id == parent_id)
                        .map(|e| e.name.clone())
                });

                let entity_fields = entity
                    .fields
                    .iter()
                    .filter_map(|field_id| fields.iter().find(|f| f.id == *field_id))
                    .map(|field| {
                        let field_type = format!("{:?}", field.field_type).to_lowercase();
                        let entity = field
                            .entity
                            .and_then(|entity_id| entities.iter().find(|e| e.id == entity_id))
                            .map(|e| e.name.clone());

                        // Convert RelationshipType enum to string
                        let relationship_str = match field.relationship {
                            RelationshipType::OneToOne => "one_to_one",
                            RelationshipType::OneToMany => "one_to_many",
                            RelationshipType::OrderedOneToMany => "ordered_one_to_many",
                            RelationshipType::ManyToOne => "many_to_one",
                            RelationshipType::ManyToMany => "many_to_many",
                        };

                        model_structs::Field {
                            name: field.name.clone(),
                            r#type: field_type,
                            entity,
                            relationship: Some(relationship_str.to_string()),
                            required: if field.required { Some(true) } else { None },
                            strong: if field.strong { Some(true) } else { None },
                            list_model: if field.list_model { Some(true) } else { None },
                            list_model_displayed_field: field.list_model_displayed_field.clone(),
                            is_primary_key: if field.is_primary_key {
                                Some(true)
                            } else {
                                None
                            },
                            single_model: if field.single_model { Some(true) } else { None },
                            enum_name: field.enum_name.clone(),
                            enum_values: field.enum_values.clone(),
                        }
                    })
                    .collect::<Vec<model_structs::Field>>();

                model_structs::Entity {
                    name: entity.name.clone(),
                    only_for_heritage: Some(entity.only_for_heritage),
                    parent,
                    allow_direct_access: entity.allow_direct_access,
                    fields: entity_fields,
                }
            })
            .collect::<Vec<model_structs::Entity>>();

        let model_features = features
            .iter()
            .map(|feature| {
                let feature_use_cases = feature
                    .use_cases
                    .iter()
                    .filter_map(|uc_id| use_cases.iter().find(|uc| uc.id == *uc_id))
                    .map(|use_case| {
                        let entity_names = use_case
                            .entities
                            .iter()
                            .filter_map(|entity_id| {
                                entities
                                    .iter()
                                    .find(|e| e.id == *entity_id)
                                    .map(|e| e.name.clone())
                            })
                            .collect::<Vec<String>>();

                        let dto_in = use_case.dto_in.and_then(|dto_id| {
                            dtos.iter().find(|d| d.id == dto_id).map(|dto| {
                                let dto_fields = dto
                                    .fields
                                    .iter()
                                    .filter_map(|field_id| {
                                        dto_fields.iter().find(|f| f.id == *field_id)
                                    })
                                    .map(|field| {
                                        let field_type =
                                            format!("{:?}", field.field_type).to_lowercase();
                                        model_structs::DtoField {
                                            name: field.name.clone(),
                                            r#type: field_type,
                                            is_nullable: if field.is_nullable {
                                                Some(true)
                                            } else {
                                                None
                                            },
                                            is_list: if field.is_list { Some(true) } else { None },
                                            enum_name: field.enum_name.clone(),
                                            enum_values: field.enum_values.clone(),
                                        }
                                    })
                                    .collect::<Vec<model_structs::DtoField>>();

                                model_structs::Dto {
                                    name: dto.name.clone(),
                                    fields: dto_fields,
                                }
                            })
                        });

                        let dto_out = use_case.dto_out.and_then(|dto_id| {
                            dtos.iter().find(|d| d.id == dto_id).map(|dto| {
                                let dto_fields = dto
                                    .fields
                                    .iter()
                                    .filter_map(|field_id| {
                                        dto_fields.iter().find(|f| f.id == *field_id)
                                    })
                                    .map(|field| {
                                        let field_type =
                                            format!("{:?}", field.field_type).to_lowercase();
                                        model_structs::DtoField {
                                            name: field.name.clone(),
                                            r#type: field_type,
                                            is_nullable: if field.is_nullable {
                                                Some(true)
                                            } else {
                                                None
                                            },
                                            is_list: if field.is_list { Some(true) } else { None },
                                            enum_name: field.enum_name.clone(),
                                            enum_values: field.enum_values.clone(),
                                        }
                                    })
                                    .collect::<Vec<model_structs::DtoField>>();

                                model_structs::Dto {
                                    name: dto.name.clone(),
                                    fields: dto_fields,
                                }
                            })
                        });

                        model_structs::UseCase {
                            name: use_case.name.clone(),
                            validator: if use_case.validator { Some(true) } else { None },
                            entities: if entity_names.is_empty() {
                                None
                            } else {
                                Some(entity_names)
                            },
                            undoable: if (use_case.undoable) {
                                Some(true)
                            } else {
                                None
                            },
                            read_only: if (use_case.read_only) {
                                Some(true)
                            } else {
                                None
                            },
                            long_operation: if (use_case.long_operation) {
                                Some(true)
                            } else {
                                None
                            },
                            dto_in,
                            dto_out,
                        }
                    })
                    .collect::<Vec<model_structs::UseCase>>();

                model_structs::Feature {
                    name: feature.name.clone(),
                    use_cases: feature_use_cases,
                }
            })
            .collect::<Vec<model_structs::Feature>>();

        // Create the manifest
        let manifest = model_structs::Manifest {
            schema: model_structs::Schema { version: 2 },
            global: model_global,
            entities: model_entities,
            features: model_features,
            ui: model_structs::Ui { cli: true },
        };

        // Serialize to YAML
        let yaml_content = serde_yml::to_string(&manifest)?;

        // add "---\n" to the beginning of the YAML content
        let yaml_content = format!("---\n{}", yaml_content);

        // Write to file
        std::fs::write(&dto.manifest_path, yaml_content)?;

        Ok(())
    }
}
