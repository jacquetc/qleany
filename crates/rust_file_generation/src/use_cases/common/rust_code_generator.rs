use anyhow::Result;
use common::database::QueryUnitOfWork;
use common::entities::{Dto, DtoField, Entity, Feature, Field, File, UseCase};
use common::types::EntityId;
use include_dir::{Dir, include_dir};
use indexmap::IndexMap;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::OnceLock;
use tera::{Context, Tera};
use heck::AsSnakeCase;

#[derive(Debug, Serialize, Clone)]
pub struct GenerationSnapshot {
    pub file: FileVM,
    pub entities: IndexMap<EntityId, EntityVM>,
    pub features: IndexMap<EntityId, FeatureVM>,
    pub use_cases: IndexMap<EntityId, UseCaseVM>,
    pub dtos: IndexMap<EntityId, DtoVM>,
}

#[derive(Debug, Serialize, Clone)]
pub struct FileVM {
    pub inner: File,
}

#[derive(Debug, Serialize, Clone)]
pub struct EntityVM {
    pub inner: Entity,
    pub fields: IndexMap<EntityId, Field>,
    pub referenced_entities: IndexMap<EntityId, Entity>,
    pub snake_name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct FeatureVM {
    pub inner: Feature,
}

#[derive(Debug, Serialize, Clone)]
pub struct UseCaseVM {
    pub inner: UseCase,
    pub entities: IndexMap<EntityId, Entity>,
    pub dto_in: Option<DtoVM>,
    pub dto_out: Option<DtoVM>,
}

#[derive(Debug, Serialize, Clone)]
pub struct DtoVM {
    pub inner: Dto,
    pub fields: IndexMap<EntityId, DtoField>,
}

static RUST_TEMPLATES_DIR: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/src/use_cases/common/templates");

static RUST_TERA: OnceLock<Tera> = OnceLock::new();

pub(crate) fn get_rust_tera() -> &'static Tera {
    RUST_TERA.get_or_init(|| {
        let mut tera = Tera::default();
        load_templates_from_dir(&mut tera, &RUST_TEMPLATES_DIR);
        tera
    })
}

fn load_templates_from_dir(tera: &mut Tera, dir: &Dir) {
    for file in dir.files() {
        if let Some(Some(file_stem)) = file.path().file_stem().map(|s| s.to_str()) {
            // remove last ".*" from the file stem
            let file_stem = file_stem
                .rsplit_once('.')
                .map_or(file_stem, |(stem, _)| stem);

            // Add the template to Tera
            // The file contents are expected to be UTF-8 encoded
            let content = file.contents_utf8().expect("Invalid UTF-8 in template");
            tera.add_raw_template(file_stem, content)
                .expect("Failed to add template");
        }
    }
    tera.build_inheritance_chains()
        .expect("Failed to build inheritance");
}

pub(crate) fn generate_code_with_snapshot(snapshot: &GenerationSnapshot) -> Result<String> {
    let tera = get_rust_tera();
    let mut context = Context::new();
    context.insert("s", snapshot);

    let code = match snapshot.file.inner.template_name.as_str() {
        "root_cargo" => tera.render("root_cargo", &context)?,
        "common_cargo" => tera.render("common_cargo", &context)?,
        "common_lib" => tera.render("common_lib", &context)?,
        "undo_redo" => tera.render("undo_redo", &context)?,
        "types" => tera.render("types", &context)?,
        "event" => tera.render("event", &context)?,
        "long_operation" => tera.render("long_operation", &context)?,
        "database" => tera.render("database", &context)?,
        "db_context" => tera.render("db_context", &context)?,
        "transactions" => tera.render("transactions", &context)?,
        "redb_tests" => tera.render("redb_tests", &context)?,
        "undo_redo_tests" => tera.render("undo_redo_tests", &context)?,
        "common_entities" => tera.render("common_entities", &context)?,
        "common_direct_access_mod" => tera.render("common_direct_access_mod", &context)?,
        "direct_access_cargo" => tera.render("direct_access_cargo", &context)?,
        "direct_access_lib" => tera.render("direct_access_lib", &context)?,
        "entity_mod" => tera.render("entity_mod", &context)?, 
        _ => {
            return Err(anyhow::anyhow!(
                "Unknown template name: {}",
                snapshot.file.inner.template_name
            ));
        }
    };

    Ok(code)
}

// Transitional adapter to keep existing callers working
pub(crate) fn generate_code(
    file: File,
    entities: HashMap<EntityId, Entity>,
    fields: HashMap<EntityId, Field>,
    features: HashMap<EntityId, Feature>,
    use_cases: HashMap<EntityId, UseCase>,
    dtos: HashMap<EntityId, Dto>,
    dto_fields: HashMap<EntityId, DtoField>,
) -> Result<String> {
    // Wrap into a minimal snapshot
    let mut entities_vm: IndexMap<EntityId, EntityVM> = IndexMap::new();
    for (eid, e) in entities.into_iter() {
        // gather fields for this entity
        let mut e_fields: IndexMap<EntityId, Field> = IndexMap::new();
        for (fid, f) in &fields {
            if f.entity == Some(e.id) {
                e_fields.insert(*fid, f.clone());
            }
        }
        entities_vm.insert(
            eid,
            EntityVM {
                inner: e.clone(),
                fields: e_fields,
                referenced_entities: IndexMap::new(),
                snake_name: heck::AsSnakeCase(&e.name).to_string(),
            },
        );
    }

    let mut dtos_vm: IndexMap<EntityId, DtoVM> = IndexMap::new();
    for (did, d) in dtos.into_iter() {
        let mut df_map: IndexMap<EntityId, DtoField> = IndexMap::new();
        // Dto has field ids, match from dto_fields map
        for (dfid, df) in &dto_fields {
            if d.fields.contains(dfid) {
                df_map.insert(*dfid, df.clone());
            }
        }
        dtos_vm.insert(
            did,
            DtoVM {
                inner: d,
                fields: df_map,
            },
        );
    }

    let features_vm: IndexMap<EntityId, FeatureVM> = features
        .into_iter()
        .map(|(k, v)| (k, FeatureVM { inner: v }))
        .collect();

    let use_cases_vm: IndexMap<EntityId, UseCaseVM> = use_cases
        .into_iter()
        .map(|(k, uc)| {
            (
                k,
                UseCaseVM {
                    inner: uc,
                    entities: IndexMap::new(),
                    dto_in: None,
                    dto_out: None,
                },
            )
        })
        .collect();

    let snapshot = GenerationSnapshot {
        file: FileVM { inner: file },
        entities: entities_vm,
        features: features_vm,
        use_cases: use_cases_vm,
        dtos: dtos_vm,
    };

    generate_code_with_snapshot(&snapshot)
}

// Snapshot builder to compose consistent data for templates
pub struct SnapshotBuilder;

impl SnapshotBuilder {
    pub fn for_file(
        uow: &dyn GenerationReadOps,
        file_id: EntityId,
    ) -> anyhow::Result<GenerationSnapshot> {
        use anyhow::anyhow;
        // Load file
        let file = uow
            .get_file(&file_id)?
            .ok_or_else(|| anyhow!("File not found"))?;

        // Working flat maps, then wrap into VMs
        let mut dto_fields: HashMap<EntityId, DtoField> = HashMap::new();
        let mut dtos: HashMap<EntityId, Dto> = HashMap::new();
        let mut use_cases: HashMap<EntityId, UseCase> = HashMap::new();
        let mut entities: HashMap<EntityId, Entity> = HashMap::new();
        let mut fields: HashMap<EntityId, Field> = HashMap::new();
        let mut features: HashMap<EntityId, Feature> = HashMap::new();

        // Feature scope
        if let Some(feature_id) = file.feature {
            let feature = uow
                .get_feature(&feature_id)?
                .ok_or_else(|| anyhow!("Feature not found"))?;
            if feature.use_cases.is_empty() {
                return Err(anyhow!("Feature does not have an associated use case"));
            }
            let feature_use_cases: Vec<UseCase> = uow
                .get_use_case_multi(&feature.use_cases)?
                .into_iter()
                .filter_map(|uc| uc)
                .collect();

            features.insert(feature.id, feature);

            for use_case in feature_use_cases {
                // Entities for use case
                let use_case_entities: Vec<Entity> = uow
                    .get_entity_multi(&use_case.entities)?
                    .into_iter()
                    .filter_map(|e| e)
                    .collect();
                for e in use_case_entities {
                    entities.insert(e.id, e);
                }

                // DTOs
                if let Some(dto_in) = use_case.dto_in {
                    let dto = uow
                        .get_dto(&dto_in)?
                        .ok_or_else(|| anyhow!("DTO missing for use case"))?;
                    let fields_vec: Vec<DtoField> = uow
                        .get_dto_field_multi(&dto.fields)?
                        .into_iter()
                        .filter_map(|f| f)
                        .collect();
                    for f in fields_vec {
                        dto_fields.insert(f.id, f);
                    }
                    dtos.insert(dto.id, dto);
                }
                if let Some(dto_out) = use_case.dto_out {
                    let dto = uow
                        .get_dto(&dto_out)?
                        .ok_or_else(|| anyhow!("DTO missing for use case"))?;
                    let fields_vec: Vec<DtoField> = uow
                        .get_dto_field_multi(&dto.fields)?
                        .into_iter()
                        .filter_map(|f| f)
                        .collect();
                    for f in fields_vec {
                        dto_fields.insert(f.id, f);
                    }
                    dtos.insert(dto.id, dto);
                }

                use_cases.insert(use_case.id, use_case);
            }
        }

        // Single use case scope
        if let Some(use_case_id) = file.use_case {
            let use_case = uow
                .get_use_case(&use_case_id)?
                .ok_or_else(|| anyhow!("Use case not found"))?;
            let use_case_entities: Vec<Entity> = uow
                .get_entity_multi(&use_case.entities)?
                .into_iter()
                .filter_map(|e| e)
                .collect();
            for e in use_case_entities {
                entities.insert(e.id, e);
            }

            if let Some(dto_in) = use_case.dto_in {
                let dto = uow
                    .get_dto(&dto_in)?
                    .ok_or_else(|| anyhow!("DTO missing for use case"))?;
                let fields_vec: Vec<DtoField> = uow
                    .get_dto_field_multi(&dto.fields)?
                    .into_iter()
                    .filter_map(|f| f)
                    .collect();
                for f in fields_vec {
                    dto_fields.insert(f.id, f);
                }
                dtos.insert(dto.id, dto);
            }
            if let Some(dto_out) = use_case.dto_out {
                let dto = uow
                    .get_dto(&dto_out)?
                    .ok_or_else(|| anyhow!("DTO missing for use case"))?;
                let fields_vec: Vec<DtoField> = uow
                    .get_dto_field_multi(&dto.fields)?
                    .into_iter()
                    .filter_map(|f| f)
                    .collect();
                for f in fields_vec {
                    dto_fields.insert(f.id, f);
                }
                dtos.insert(dto.id, dto);
            }
            use_cases.insert(use_case.id, use_case);
        }

        // Entity scope
        if let Some(entity_id) = file.entity {
            let entity = uow
                .get_entity(&entity_id)?
                .ok_or_else(|| anyhow!("Entity not found"))?;
            let entity_fields: Vec<Field> = uow
                .get_field_multi(&entity.fields)?
                .into_iter()
                .filter_map(|f| f)
                .collect();
            for field in &entity_fields {
                if let Some(eid) = field.entity {
                    if field.field_type == common::entities::FieldType::Entity {
                        let ent = uow
                            .get_entity(&eid)?
                            .ok_or_else(|| anyhow!("Entity not found"))?;
                        entities.insert(ent.id, ent);
                    }
                }
                fields.insert(field.id, field.clone());
            }
            entities.insert(entity.id, entity);
        }

        // Now wrap into snapshot similarly to adapter
        let mut entities_vm: IndexMap<EntityId, EntityVM> = IndexMap::new();
        for (eid, e) in &entities {
            let mut e_fields: IndexMap<EntityId, Field> = IndexMap::new();
            for (fid, f) in &fields {
                if f.entity == Some(*eid) {
                    e_fields.insert(*fid, f.clone());
                }
            }
            entities_vm.insert(
                *eid,
                EntityVM {
                    inner: e.clone(),
                    fields: e_fields,
                    referenced_entities: IndexMap::new(),
                    snake_name: heck::AsSnakeCase(&e.name).to_string(),
                },
            );
        }

        let mut dtos_vm: IndexMap<EntityId, DtoVM> = IndexMap::new();
        for (did, d) in &dtos {
            let mut df_map: IndexMap<EntityId, DtoField> = IndexMap::new();
            for (dfid, df) in &dto_fields {
                if d.fields.contains(dfid) {
                    df_map.insert(*dfid, df.clone());
                }
            }
            dtos_vm.insert(
                *did,
                DtoVM {
                    inner: d.clone(),
                    fields: df_map,
                },
            );
        }

        let features_vm: IndexMap<EntityId, FeatureVM> = features
            .into_iter()
            .map(|(k, v)| (k, FeatureVM { inner: v }))
            .collect();
        let use_cases_vm: IndexMap<EntityId, UseCaseVM> = use_cases
            .into_iter()
            .map(|(k, uc)| {
                (
                    k,
                    UseCaseVM {
                        inner: uc.clone(),
                        entities: IndexMap::new(),
                        dto_in: uc.dto_in.and_then(|dto_id| {
                            dtos.get(&dto_id).map(|d| DtoVM {
                                inner: d.clone(),
                                fields: {
                                    let mut df_map = IndexMap::new();
                                    for (dfid, df) in &dto_fields {
                                        if d.fields.contains(dfid) {
                                            df_map.insert(*dfid, df.clone());
                                        }
                                    }
                                    df_map
                                },
                            })
                        }),
                        dto_out: uc.dto_out.and_then(|dto_id| {
                            dtos.get(&dto_id).map(|d| DtoVM {
                                inner: d.clone(),
                                fields: {
                                    let mut df_map = IndexMap::new();
                                    for (dfid, df) in &dto_fields {
                                        if d.fields.contains(dfid) {
                                            df_map.insert(*dfid, df.clone());
                                        }
                                    }
                                    df_map
                                },
                            })
                        }),
                    },
                )
            })
            .collect();

        // compute entity_snake if entity scope
        Ok(GenerationSnapshot {
            file: FileVM { inner: file },
            entities: entities_vm,
            features: features_vm,
            use_cases: use_cases_vm,
            dtos: dtos_vm,
        })
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_root_cargo_tera_template() {
        let tera = get_rust_tera();
        let mut context = Context::new();
        let code = tera.render("root_cargo", &context).unwrap();
        println!("{}", code);
    }
}

// Shared read-API for snapshot building across code and files generation
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
pub trait GenerationReadOps: QueryUnitOfWork {}
