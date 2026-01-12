mod direct_access_lib_tests;
mod rust_code_generator_tests;

use common::entities::Workspace;
use anyhow::Result;
use common::database::QueryUnitOfWork;
use common::entities::{
    Dto, DtoField, DtoFieldType, Entity, Feature, Field, FieldRelationshipType, FieldType, File,
    Global, Relationship, RelationshipType, Root, UseCase,
};
use common::types::EntityId;
use include_dir::{Dir, include_dir};
use indexmap::IndexMap;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::OnceLock;
use tera::{Context, Tera};
use common::entities::Strength::Weak;
use crate::use_cases::common::tools;

// Shared read-API for snapshot building across code and files generation
#[macros::uow_action(entity = "Root", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Root", action = "GetMultiRO")]
#[macros::uow_action(entity = "Workspace", action = "GetRO")]
#[macros::uow_action(entity = "Workspace", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "File", action = "GetRO")]
#[macros::uow_action(entity = "Global", action = "GetRO")]
#[macros::uow_action(entity = "Feature", action = "GetRO")]
#[macros::uow_action(entity = "Feature", action = "GetMultiRO")]
#[macros::uow_action(entity = "UseCase", action = "GetRO")]
#[macros::uow_action(entity = "UseCase", action = "GetMultiRO")]
#[macros::uow_action(entity = "Dto", action = "GetRO")]
#[macros::uow_action(entity = "DtoField", action = "GetRO")]
#[macros::uow_action(entity = "DtoField", action = "GetMultiRO")]
#[macros::uow_action(entity = "Entity", action = "GetRO")]
#[macros::uow_action(entity = "Entity", action = "GetMultiRO")]
#[macros::uow_action(entity = "Field", action = "GetRO")]
#[macros::uow_action(entity = "Field", action = "GetMultiRO")]
#[macros::uow_action(entity = "Relationship", action = "GetRO")]
#[macros::uow_action(entity = "Relationship", action = "GetMultiRO")]
pub(crate) trait GenerationReadOps: QueryUnitOfWork {}

#[derive(Debug, Serialize, Clone)]
pub(crate) struct GenerationSnapshot {
    file: FileVM,
    global: GlobalVM,
    entities: IndexMap<EntityId, EntityVM>,
    features: IndexMap<EntityId, FeatureVM>,
    use_cases: IndexMap<EntityId, UseCaseVM>,
    dtos: IndexMap<EntityId, DtoVM>,
}

#[derive(Debug, Serialize, Clone)]
struct FileVM {
    pub inner: File,
}

#[derive(Debug, Serialize, Clone)]
struct GlobalVM {
    pub inner: Global,
    pub application_kebab_name: String,
}

#[derive(Debug, Serialize, Clone)]
struct EntityVM {
    pub inner: Entity,
    pub relationships: IndexMap<EntityId, RelationshipVM>,
    pub forward_relationships: IndexMap<EntityId, RelationshipVM>,
    pub backward_relationships: IndexMap<EntityId, RelationshipVM>,
    pub snake_name: String,
    pub pascal_name: String,
    pub fields: Vec<FieldVM>,
}

#[derive(Debug, Serialize, Clone)]
struct FeatureVM {
    pub inner: Feature,
    pub use_cases: IndexMap<EntityId, UseCaseVM>,
    pub snake_name: String,
    pub pascal_name: String,
}

#[derive(Debug, Serialize, Clone)]
struct UseCaseVM {
    pub inner: UseCase,
    pub entities: IndexMap<EntityId, EntityVM>,
    pub dto_in: Option<DtoVM>,
    pub dto_out: Option<DtoVM>,
    pub snake_name: String,
    pub pascal_name: String,
}

#[derive(Debug, Serialize, Clone)]
struct DtoVM {
    pub inner: Dto,
    pub fields: Vec<DtoFieldVM>,
    pub pascal_name: String,
}

#[derive(Debug, Serialize, Clone)]
struct FieldVM {
    pub inner: Field,
    pub pascal_name: String,
    pub snake_name: String,
    pub relationship: String,
    pub required: bool,
    pub rust_base_type: String,
    pub rust_type: String,
}

#[derive(Debug, Serialize, Clone)]
struct DtoFieldVM {
    pub inner: DtoField,
    pub pascal_name: String,
    pub snake_name: String,
    pub rust_base_type: String,
    pub rust_type: String,
}

#[derive(Debug, Serialize, Clone)]
struct RelationshipVM {
    pub inner: Relationship,
    pub field_snake_name: String,
    pub field_pascal_name: String,
}

static RUST_TEMPLATES_DIR: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/src/use_cases/common/templates");

static RUST_TERA: OnceLock<Tera> = OnceLock::new();

fn get_rust_tera() -> &'static Tera {
    RUST_TERA.get_or_init(|| {
        let mut tera = Tera::default();
        load_templates_from_dir(&mut tera, &RUST_TEMPLATES_DIR);
        tera
    })
}

fn load_templates_from_dir(tera: &mut Tera, dir: &Dir) {
    fn add_raw_template_in_dir(tera: &mut Tera, dir: &Dir) {
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

    add_raw_template_in_dir(tera, dir);
    for subdir in dir.dirs() {
        add_raw_template_in_dir(tera, subdir);
    }
}

pub(crate) fn generate_code_with_snapshot(snapshot: &GenerationSnapshot) -> Result<String> {
    let tera = get_rust_tera();
    let mut context = Context::new();
    context.insert("s", snapshot);

    let code = match snapshot.file.inner.template_name.as_str() {
        // root and common
        "root_cargo" => tera.render("root_cargo", &context)?,
        "common_cargo" => tera.render("common_cargo", &context)?,
        "common_lib" => tera.render("common_lib", &context)?,
        // common modules
        "undo_redo" => tera.render("undo_redo", &context)?,
        "types" => tera.render("types", &context)?,
        "long_operation" => tera.render("long_operation", &context)?,
        "database" => tera.render("database", &context)?,
        "db_context" => tera.render("db_context", &context)?,
        "db_helpers" => tera.render("db_helpers", &context)?,
        "transactions" => tera.render("transactions", &context)?,
        "redb_tests" => tera.render("redb_tests", &context)?,
        "undo_redo_tests" => tera.render("undo_redo_tests", &context)?,
        "repository_factory" => tera.render("repository_factory", &context)?,
        "common_setup" => tera.render("common_setup", &context)?,
        // common direct access entities registry
        "common_entities" => tera.render("common_entities", &context)?,
        "common_event" => tera.render("common_event", &context)?,
        "common_direct_access_mod" => tera.render("common_direct_access_mod", &context)?,
        // direct_access crate
        "direct_access_cargo" => tera.render("direct_access_cargo", &context)?,
        "direct_access_lib" => tera.render("direct_access_lib", &context)?,
        // per-entity files inside direct_access
        "entity_mod" => tera.render("entity_mod", &context)?,
        "entity_dtos" => tera.render("entity_dtos", &context)?,
        "entity_use_cases_mod" => tera.render("entity_use_cases_mod", &context)?,
        "entity_units_of_work" => tera.render("entity_units_of_work", &context)?,
        "entity_controller" => tera.render("entity_controller", &context)?,
        "entity_get_use_case" => tera.render("entity_get_use_case", &context)?,
        "entity_get_multi_use_case" => tera.render("entity_get_multi_use_case", &context)?,
        "entity_create_use_case" => tera.render("entity_create_use_case", &context)?,
        "entity_create_multi_use_case" => tera.render("entity_create_multi_use_case", &context)?,
        "entity_update_multi_use_case" => tera.render("entity_update_multi_use_case", &context)?,
        "entity_update_use_case" => tera.render("entity_update_use_case", &context)?,
        "entity_remove_multi_use_case" => tera.render("entity_remove_multi_use_case", &context)?,
        "entity_remove_use_case" => tera.render("entity_remove_use_case", &context)?,
        "entity_get_relationship_use_case" => {
            tera.render("entity_get_relationship_use_case", &context)?
        }
        "entity_set_relationship_use_case" => {
            tera.render("entity_set_relationship_use_case", &context)?
        }
        // common direct_access per-entity
        "common_entity_mod" => tera.render("common_entity_mod", &context)?,
        "common_entity_repository" => tera.render("common_entity_repository", &context)?,
        "common_entity_table" => tera.render("common_entity_table", &context)?,
        // feature crates
        "feature_cargo" => tera.render("feature_cargo", &context)?,
        "feature_lib" => tera.render("feature_lib", &context)?,
        "feature_use_cases_mod" => tera.render("feature_use_cases_mod", &context)?,
        "feature_dtos" => tera.render("feature_dtos", &context)?,
        "feature_units_of_work_mod" => tera.render("feature_units_of_work_mod", &context)?,
        "feature_controller" => tera.render("feature_controller", &context)?,
        "feature_use_case" => tera.render("feature_use_case", &context)?,
        "feature_use_case_uow" => tera.render("feature_use_case_uow", &context)?,
        // macros crate
        "macros_cargo" => tera.render("macros_cargo", &context)?,
        "macros_lib" => tera.render("macros_lib", &context)?,
        "macros_direct_access" => tera.render("macros_direct_access", &context)?,
        // cli
        "cli_cargo" => tera.render("cli_cargo", &context)?,
        "cli_main" => tera.render("cli_main", &context)?,
        _ => {
            return Err(anyhow::anyhow!(
                "Unknown template name: {}",
                snapshot.file.inner.template_name
            ));
        }
    };

    Ok(code)
}

// Snapshot builder to compose consistent data for templates
pub(crate) struct SnapshotBuilder;

impl SnapshotBuilder {

    fn get_entity_vm(
        uow: &dyn GenerationReadOps,
        entity_id: &EntityId,
    ) -> anyhow::Result<EntityVM> {
        use anyhow::anyhow;
        // Load the entity
        let entity = uow
            .get_entity(entity_id)?
            .ok_or_else(|| anyhow!("Entity not found"))?;

        let mut fields_vec: Vec<Field> = Vec::new();

        // Load fields from the inherited entity first, if any
        if let Some(parent_entity_id) = entity.inherits_from {
            let parent_entity_vm = Self::get_entity_vm(uow, &parent_entity_id)?;
            for parent_field_vm in &parent_entity_vm.fields {
                fields_vec.push(parent_field_vm.inner.clone());
            }
        }

        // Load fields belonging to the entity
        fields_vec.extend(
            uow.get_field_multi(&entity.fields)?
                .into_iter()
                .filter_map(|f| f),
        );

        // Build FieldVMs with the same Rust type mapping as used elsewhere
        let mut fields_vm_vec: Vec<FieldVM> = Vec::new();
        for f in &fields_vec {
            if f.name == "id" {
                continue; // skip id field
            }

            let rust_base_type = match f.field_type {
                FieldType::Boolean => "bool".to_string(),
                FieldType::Integer => "i64".to_string(),
                FieldType::UInteger => "u64".to_string(),
                FieldType::Float => "f64".to_string(),
                FieldType::String => "String".to_string(),
                FieldType::Uuid => "uuid::Uuid".to_string(),
                FieldType::DateTime => "chrono::DateTime<chrono::Utc>".to_string(),
                FieldType::Entity => "EntityId".to_string(),
                FieldType::Enum => f
                    .enum_name
                    .clone()
                    .or(Some("enum_name not set".to_string()))
                    .unwrap(),
            };
            //
            let relationship = match f.relationship {
                FieldRelationshipType::OneToOne => "OneToOne".to_string(),
                FieldRelationshipType::OrderedOneToMany => "OrderedOneToMany".to_string(),
                FieldRelationshipType::OneToMany => "OneToMany".to_string(),
                FieldRelationshipType::ManyToOne => "ManyToOne".to_string(),
                FieldRelationshipType::ManyToMany => "ManyToMany".to_string(),
            };

            let rust_type = match f.relationship {
                FieldRelationshipType::OneToOne | FieldRelationshipType::ManyToOne => {
                    if f.required {
                        rust_base_type.clone()
                    } else {
                        format!("Option<{}>", &rust_base_type)
                    }
                }
                FieldRelationshipType::OrderedOneToMany
                | FieldRelationshipType::OneToMany
                | FieldRelationshipType::ManyToMany => format!("Vec<{}>", rust_base_type),
            };
            fields_vm_vec.push(FieldVM {
                inner: f.clone(),
                pascal_name: heck::AsPascalCase(&f.name).to_string(),
                snake_name: heck::AsSnakeCase(&f.name).to_string(),
                relationship,
                required: f.required,
                rust_base_type,
                rust_type,
            });
        }

        // Build relationships maps.
        // We want to include both forward and backward relationships where this entity is involved.
        use std::collections::HashSet;
        let mut relationships_map: IndexMap<EntityId, Relationship> = IndexMap::new();

        // Start with relationships explicitly listed on this entity
        let rel_ids_direct = entity.relationships.clone();
        for rel in uow
            .get_relationship_multi(&rel_ids_direct)?
            .into_iter()
            .filter_map(|r| r)
        {
            relationships_map.insert(rel.id, rel);
        }


        // Additionally, scan all entities from workspace to discover relationships that reference this
        // entity as the right side (backward relationships) but may not be listed on this entity.
        // This mirrors broader snapshot building behavior where backward rels are gathered from peers.
        let all_entity_ids = uow.get_workspace_relationship(
            &tools::get_workspace_id(uow)?,
            &common::direct_access::workspace::WorkspaceRelationshipField::Entities,
        )?;

        if !all_entity_ids.is_empty() {
            let all_entities = uow.get_entity_multi(&all_entity_ids)?;
            let mut extra_rel_ids: HashSet<EntityId> = HashSet::new();
            for e_opt in all_entities.into_iter().filter_map(|e| e) {
                for rid in e_opt.relationships {
                    extra_rel_ids.insert(rid);
                }
            }
            if !extra_rel_ids.is_empty() {
                let extra_rels =
                    uow.get_relationship_multi(&extra_rel_ids.iter().copied().collect::<Vec<_>>())?;
                for rel_opt in extra_rels.into_iter().filter_map(|r| r) {
                    if rel_opt.left_entity == entity.id || rel_opt.right_entity == entity.id {
                        relationships_map.entry(rel_opt.id).or_insert(rel_opt);
                    }
                }
            }
        }

        // Now split into all/forward/backward with deduplication strategy used in for_file
        let mut rel_all: IndexMap<EntityId, RelationshipVM> = IndexMap::new();
        let mut rel_fwd: IndexMap<EntityId, RelationshipVM> = IndexMap::new();
        let mut rel_bwd: IndexMap<EntityId, RelationshipVM> = IndexMap::new();
        let mut fwd_seen: HashSet<String> = HashSet::new();
        let mut bwd_seen: HashSet<(EntityId, String)> = HashSet::new();

        for (rid, rel) in &relationships_map {
            if rel.left_entity == entity.id || rel.right_entity == entity.id {
                rel_all.entry(*rid).or_insert_with(|| RelationshipVM {
                    inner: rel.clone(),
                    field_snake_name: heck::AsSnakeCase(rel.field_name.clone()).to_string(),
                    field_pascal_name: heck::AsPascalCase(rel.field_name.clone()).to_string(),
                });
                if rel.left_entity == entity.id {
                    if fwd_seen.insert(rel.field_name.clone()) {
                        rel_fwd.entry(*rid).or_insert_with(|| RelationshipVM {
                            inner: rel.clone(),
                            field_snake_name: heck::AsSnakeCase(rel.field_name.clone()).to_string(),
                            field_pascal_name: heck::AsPascalCase(rel.field_name.clone())
                                .to_string(),
                        });
                    }
                }
                if rel.right_entity == entity.id {
                    let key = (rel.left_entity, rel.field_name.clone());
                    if bwd_seen.insert(key) {
                        rel_bwd.entry(*rid).or_insert_with(|| RelationshipVM {
                            inner: rel.clone(),
                            field_snake_name: heck::AsSnakeCase(rel.field_name.clone()).to_string(),
                            field_pascal_name: heck::AsPascalCase(rel.field_name.clone())
                                .to_string(),
                        });
                    }
                }
            }
        }

        Ok(EntityVM {
            inner: entity.clone(),
            fields: fields_vm_vec,
            relationships: rel_all,
            forward_relationships: rel_fwd,
            backward_relationships: rel_bwd,
            snake_name: heck::AsSnakeCase(&entity.name).to_string(),
            pascal_name: heck::AsPascalCase(&entity.name).to_string(),
        })
    }

    pub(crate) fn for_file(
        uow: &dyn GenerationReadOps,
        file_id: EntityId,
        generation_snapshot_cache: &Vec<GenerationSnapshot>,
    ) -> anyhow::Result<(GenerationSnapshot, bool)> {
        use anyhow::anyhow;
        // Load file
        let file = uow
            .get_file(&file_id)?
            .ok_or_else(|| anyhow!("File not found"))?;

        // compare with cache
        for cached_snapshot in generation_snapshot_cache {
            let cached_file_vm = &cached_snapshot.file;
            if file.entity == cached_file_vm.inner.entity
                && file.feature == cached_file_vm.inner.feature
                && file.use_case == cached_file_vm.inner.use_case
            {
                // cache hit

                let new_file_vm = FileVM {
                    inner: file.clone(),
                };

                let new_snapshot = GenerationSnapshot {
                    file: new_file_vm,
                    global: cached_snapshot.global.clone(),
                    entities: cached_snapshot.entities.clone(),
                    features: cached_snapshot.features.clone(),
                    use_cases: cached_snapshot.use_cases.clone(),
                    dtos: cached_snapshot.dtos.clone(),
                };

                println!("Snapshot cache hit for file id {}", file_id);

                return Ok((new_snapshot, true));
            }
        }

        let workspace_id = tools::get_workspace_id(uow)?;

        let global_ids = uow.get_workspace_relationship(
            &workspace_id,
            &common::direct_access::workspace::WorkspaceRelationshipField::Global,
        )?;

        let global = uow
            .get_global(&global_ids.first().expect("Root must have a global entity"))?
            .expect("Root must have a global entity");

        let global_vm = GlobalVM {
            inner: global.clone(),
            application_kebab_name: heck::AsKebabCase(&global.application_name).to_string(),
        };

        // Working flat maps, then wrap into VMs
        let mut dto_fields: HashMap<EntityId, DtoField> = HashMap::new();
        let mut dtos: HashMap<EntityId, Dto> = HashMap::new();
        let mut use_cases: HashMap<EntityId, UseCase> = HashMap::new();
        let mut entities: HashMap<EntityId, Entity> = HashMap::new();
        let mut fields: HashMap<EntityId, Field> = HashMap::new();
        let mut features: HashMap<EntityId, Feature> = HashMap::new();

        // Feature scope
        if let Some(feature_id) = file.feature {
            if feature_id == 0 {
                // Load all features via Root -> Features
                let feature_ids = uow.get_workspace_relationship(
                    &workspace_id,
                    &common::direct_access::workspace::WorkspaceRelationshipField::Features,
                )?;
                let feats = uow.get_feature_multi(&feature_ids)?;
                for feat_opt in feats.into_iter().filter_map(|f| f) {
                    if feat_opt.use_cases.is_empty() {
                        continue;
                    }
                    let feature_use_cases: Vec<UseCase> = uow
                        .get_use_case_multi(&feat_opt.use_cases)?
                        .into_iter()
                        .filter_map(|uc| uc)
                        .collect();

                    features.insert(feat_opt.id, feat_opt);

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
            } else {
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

        // Entity scope (including special case Some(0) meaning all entities)
        if let Some(entity_id) = file.entity {
            if entity_id == 0 {
                // Load all entities via Root -> Entities
                let entity_ids = uow.get_workspace_relationship(
                    &workspace_id,
                    &common::direct_access::workspace::WorkspaceRelationshipField::Entities,
                )?;
                let ents = uow.get_entity_multi(&entity_ids)?;
                for ent_opt in ents.into_iter().filter_map(|e| e) {
                    // skip heritage entities; include only those allowed for direct access when generating direct access code
                    if ent_opt.only_for_heritage {
                        continue;
                    }

                    // load fields
                    let entity_fields: Vec<Field> = uow
                        .get_field_multi(&ent_opt.fields)?
                        .into_iter()
                        .filter_map(|f| f)
                        .collect();
                    for field in &entity_fields {
                        if let Some(eid) = field.entity {
                            if field.field_type == FieldType::Entity {
                                if let Some(ent_dep) = uow.get_entity(&eid)? {
                                    entities.insert(ent_dep.id, ent_dep);
                                }
                            }
                        }
                        fields.insert(field.id, field.clone());
                    }
                    entities.insert(ent_opt.id, ent_opt);
                }
            } else {
                let entity = uow
                    .get_entity(&entity_id)?
                    .ok_or_else(|| anyhow!("Entity not found"))?;
                let entity_fields: Vec<Field> = uow
                    .get_field_multi(&entity.fields)?
                    .into_iter()
                    .filter_map(|f| f)
                    .collect();
                // load fields ao as to list entity dependencies
                for field in &entity_fields {
                    if let Some(eid) = field.entity {
                        if field.field_type == FieldType::Entity {
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
        }

        // Load relationships for all collected entities
        let mut relationships_map: IndexMap<EntityId, Relationship> = IndexMap::new();
        {
            let mut rel_ids: Vec<EntityId> = entities
                .values()
                .flat_map(|e| e.relationships.clone())
                .collect();
            rel_ids.sort();
            rel_ids.dedup();
            if !rel_ids.is_empty() {
                let rels = uow.get_relationship_multi(&rel_ids)?;
                for rel_opt in rels.into_iter().filter_map(|r| r) {
                    // Ensure both sides entities are available for templates if referenced
                    if !entities.contains_key(&rel_opt.left_entity) {
                        if let Some(le) = uow.get_entity(&rel_opt.left_entity)? {
                            entities.insert(le.id, le);
                        }
                    }
                    if !entities.contains_key(&rel_opt.right_entity) {
                        if let Some(re) = uow.get_entity(&rel_opt.right_entity)? {
                            entities.insert(re.id, re);
                        }
                    }
                    relationships_map.insert(rel_opt.id, rel_opt);
                }
            }
        }

        // Now wrap into snapshot similarly to adapter
        let mut entities_vm: IndexMap<EntityId, EntityVM> = IndexMap::new();
        for (eid, _e) in &entities {
            // Use the unified entity view model builder
            let evm = SnapshotBuilder::get_entity_vm(uow, eid)?;
            entities_vm.insert(*eid, evm);
        }

        let mut dtos_vm: IndexMap<EntityId, DtoVM> = IndexMap::new();
        for (did, d) in &dtos {
            let mut df_vec: Vec<DtoFieldVM> = Vec::new();
            for (dfid, df) in &dto_fields {
                if d.fields.contains(dfid) {
                    let rust_base_type = match df.field_type {
                        DtoFieldType::Boolean => "bool".to_string(),
                        DtoFieldType::Integer => "i64".to_string(),
                        DtoFieldType::UInteger => "u64".to_string(),
                        DtoFieldType::Float => "f64".to_string(),
                        DtoFieldType::String => "String".to_string(),
                        DtoFieldType::Uuid => "uuid::Uuid".to_string(),
                        DtoFieldType::DateTime => "chrono::DateTime<chrono::Utc>".to_string(),
                        DtoFieldType::Enum => df
                            .enum_name
                            .clone()
                            .or(Some("enum_name not set".to_string()))
                            .unwrap(),
                    };
                    let rust_type = if df.is_list {
                        format!("Vec<{}>", &rust_base_type)
                    } else {
                        rust_base_type.clone()
                    };
                    df_vec.push(DtoFieldVM {
                        inner: df.clone(),
                        pascal_name: heck::AsPascalCase(&df.name).to_string(),
                        snake_name: heck::AsSnakeCase(&df.name).to_string(),
                        rust_base_type,
                        rust_type,
                    });
                }
            }
            dtos_vm.insert(
                *did,
                DtoVM {
                    inner: d.clone(),
                    fields: df_vec,
                    pascal_name: heck::AsPascalCase(&d.name).to_string(),
                },
            );
        }

        let features_vm: IndexMap<EntityId, FeatureVM> = features
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    FeatureVM {
                        inner: v.clone(),
                        use_cases: {
                            let use_cases_ids = v.use_cases;
                            let use_cases: IndexMap<EntityId, UseCase> = uow
                                .get_use_case_multi(&use_cases_ids)
                                .unwrap_or_default()
                                .into_iter()
                                .filter_map(|uc| uc)
                                .map(|uc| (uc.id, uc))
                                .collect();

                            let use_case_vms: IndexMap<EntityId, UseCaseVM> = use_cases
                                .into_iter()
                                .map(|(k, uc)| {
                                    (
                                        k,
                                        UseCaseVM {
                                            inner: uc.clone(),
                                            entities: {
                                                let entities_id = uc.entities;
                                                let entity_vm: IndexMap<EntityId, EntityVM> = uow
                                                    .get_entity_multi(&entities_id)
                                                    .unwrap_or_default()
                                                    .into_iter()
                                                    .filter_map(|e| e)
                                                    .map(|e| {
                                                        (
                                                            e.id,
                                                            SnapshotBuilder::get_entity_vm(
                                                                uow, &e.id,
                                                            )
                                                            .unwrap_or(EntityVM {
                                                                inner: e,
                                                                relationships: Default::default(),
                                                                forward_relationships:
                                                                    Default::default(),
                                                                backward_relationships:
                                                                    Default::default(),
                                                                snake_name: "".to_string(),
                                                                pascal_name: "".to_string(),
                                                                fields: vec![],
                                                            }),
                                                        )
                                                    })
                                                    .collect();
                                                entity_vm
                                            },
                                            dto_in: uc.dto_in.and_then(|dto_id| {
                                                dtos.get(&dto_id).map(|d| DtoVM {
                                                    inner: d.clone(),
                                                    fields: {
                                                        let mut df_vec: Vec<DtoFieldVM> = Vec::new();
                                                        for (dfid, df) in &dto_fields {
                                                            if d.fields.contains(dfid) {                    let rust_base_type = match df.field_type {
                                                                DtoFieldType::Boolean => "bool".to_string(),
                                                                DtoFieldType::Integer => "i64".to_string(),
                                                                DtoFieldType::UInteger => "u64".to_string(),
                                                                DtoFieldType::Float => "f64".to_string(),
                                                                DtoFieldType::String => "String".to_string(),
                                                                DtoFieldType::Uuid => "uuid::Uuid".to_string(),
                                                                DtoFieldType::DateTime => "chrono::DateTime<chrono::Utc>".to_string(),
                                                                DtoFieldType::Enum => df.enum_name.clone().or(Some("enum_name not set".to_string())).unwrap(),
                                                            };
                                                                let rust_type = if df.is_list {
                                                                    format!("Vec<{}>", &rust_base_type)
                                                                } else {
                                                                    rust_base_type.clone()
                                                                };
                                                                df_vec.push(DtoFieldVM {
                                                                    inner: df.clone(),
                                                                    pascal_name: heck::AsPascalCase(&df.name).to_string(),
                                                                    snake_name: heck::AsSnakeCase(&df.name).to_string(),
                                                                    rust_base_type,
                                                                    rust_type,
                                                                });
                                                            }
                                                        }
                                                        df_vec
                                                    },
                                                    pascal_name: heck::AsPascalCase(&d.name)
                                                        .to_string(),
                                                })
                                            }),
                                            dto_out: uc.dto_out.and_then(|dto_id| {
                                                dtos.get(&dto_id).map(|d| DtoVM {
                                                    inner: d.clone(),
                                                    fields: {
                                                        let mut df_vec: Vec<DtoFieldVM> = Vec::new();
                                                        for (dfid, df) in &dto_fields {
                                                            if d.fields.contains(dfid) {                    let rust_base_type = match df.field_type {
                                                                DtoFieldType::Boolean => "bool".to_string(),
                                                                DtoFieldType::Integer => "i64".to_string(),
                                                                DtoFieldType::UInteger => "u64".to_string(),
                                                                DtoFieldType::Float => "f64".to_string(),
                                                                DtoFieldType::String => "String".to_string(),
                                                                DtoFieldType::Uuid => "uuid::Uuid".to_string(),
                                                                DtoFieldType::DateTime => "chrono::DateTime<chrono::Utc>".to_string(),
                                                                DtoFieldType::Enum => df.enum_name.clone().or(Some("enum_name not set".to_string())).unwrap(),
                                                            };
                                                                let rust_type = if df.is_list {
                                                                    format!("Vec<{}>", &rust_base_type)
                                                                } else {
                                                                    rust_base_type.clone()
                                                                };
                                                                df_vec.push(DtoFieldVM {
                                                                    inner: df.clone(),
                                                                    pascal_name: heck::AsPascalCase(&df.name).to_string(),
                                                                    snake_name: heck::AsSnakeCase(&df.name).to_string(),
                                                                    rust_base_type,
                                                                    rust_type,
                                                                });
                                                            }
                                                        }
                                                        df_vec
                                                    },
                                                    pascal_name: heck::AsPascalCase(&d.name)
                                                        .to_string(),
                                                })
                                            }),
                                            pascal_name: heck::AsPascalCase(&uc.name).to_string(),
                                            snake_name: heck::AsSnakeCase(&uc.name).to_string(),
                                        },
                                    )
                                })
                                .collect();
                            use_case_vms
                        },
                        snake_name: heck::AsSnakeCase(&v.name).to_string(),
                        pascal_name: heck::AsPascalCase(&v.name).to_string(),
                    },
                )
            })
            .collect();
        let use_cases_vm: IndexMap<EntityId, UseCaseVM> = use_cases
            .into_iter()
            .map(|(k, uc)| {
                (
                    k,
                    UseCaseVM {
                        inner: uc.clone(),
                        entities: {
                            let entities_id = uc.entities;
                            uow.get_entity_multi(&entities_id)
                                .unwrap_or_default()
                                .into_iter()
                                .filter_map(|e| e)
                                .map(|e| {
                                    (
                                        e.id,
                                        SnapshotBuilder::get_entity_vm(uow, &e.id).unwrap_or(
                                            EntityVM {
                                                inner: e,
                                                relationships: Default::default(),
                                                forward_relationships: Default::default(),
                                                backward_relationships: Default::default(),
                                                snake_name: "".to_string(),
                                                pascal_name: "".to_string(),
                                                fields: vec![],
                                            },
                                        ),
                                    )
                                })
                                .collect()
                        },
                        dto_in: uc.dto_in.and_then(|dto_id| {
                            dtos.get(&dto_id).map(|d| DtoVM {
                                inner: d.clone(),
                                fields: {
                                    let mut df_vec: Vec<DtoFieldVM> = Vec::new();
                                    for (dfid, df) in &dto_fields {
                                        if d.fields.contains(dfid) {
                                            let rust_base_type = match df.field_type {
                                                DtoFieldType::Boolean => "bool".to_string(),
                                                DtoFieldType::Integer => "i64".to_string(),
                                                DtoFieldType::UInteger => "u64".to_string(),
                                                DtoFieldType::Float => "f64".to_string(),
                                                DtoFieldType::String => "String".to_string(),
                                                DtoFieldType::Uuid => "uuid::Uuid".to_string(),
                                                DtoFieldType::DateTime => {
                                                    "chrono::DateTime<chrono::Utc>".to_string()
                                                }
                                                DtoFieldType::Enum => df
                                                    .enum_name
                                                    .clone()
                                                    .or(Some("enum_name not set".to_string()))
                                                    .unwrap(),
                                            };
                                            let rust_type = if df.is_list {
                                                format!("Vec<{}>", &rust_base_type)
                                            } else {
                                                rust_base_type.clone()
                                            };
                                            df_vec.push(DtoFieldVM {
                                                inner: df.clone(),
                                                pascal_name: heck::AsPascalCase(&df.name)
                                                    .to_string(),
                                                snake_name: heck::AsSnakeCase(&df.name).to_string(),
                                                rust_base_type,
                                                rust_type,
                                            });
                                        }
                                    }
                                    df_vec
                                },
                                pascal_name: heck::AsPascalCase(&d.name).to_string(),
                            })
                        }),
                        dto_out: uc.dto_out.and_then(|dto_id| {
                            dtos.get(&dto_id).map(|d| DtoVM {
                                inner: d.clone(),
                                fields: {
                                    let mut df_vec: Vec<DtoFieldVM> = Vec::new();
                                    for (dfid, df) in &dto_fields {
                                        if d.fields.contains(dfid) {
                                            let rust_base_type = match df.field_type {
                                                DtoFieldType::Boolean => "bool".to_string(),
                                                DtoFieldType::Integer => "i64".to_string(),
                                                DtoFieldType::UInteger => "u64".to_string(),
                                                DtoFieldType::Float => "f64".to_string(),
                                                DtoFieldType::String => "String".to_string(),
                                                DtoFieldType::Uuid => "uuid::Uuid".to_string(),
                                                DtoFieldType::DateTime => {
                                                    "chrono::DateTime<chrono::Utc>".to_string()
                                                }
                                                DtoFieldType::Enum => df
                                                    .enum_name
                                                    .clone()
                                                    .or(Some("enum_name not set".to_string()))
                                                    .unwrap(),
                                            };
                                            let rust_type = if df.is_list {
                                                format!("Vec<{}>", &rust_base_type)
                                            } else {
                                                rust_base_type.clone()
                                            };
                                            df_vec.push(DtoFieldVM {
                                                inner: df.clone(),
                                                pascal_name: heck::AsPascalCase(&df.name)
                                                    .to_string(),
                                                snake_name: heck::AsSnakeCase(&df.name).to_string(),
                                                rust_base_type,
                                                rust_type,
                                            });
                                        }
                                    }
                                    df_vec
                                },
                                pascal_name: heck::AsPascalCase(&d.name).to_string(),
                            })
                        }),
                        pascal_name: heck::AsPascalCase(&uc.name).to_string(),
                        snake_name: heck::AsSnakeCase(&uc.name).to_string(),
                    },
                )
            })
            .collect();

        // compute entity_snake if entity scope
        Ok((
            GenerationSnapshot {
                file: FileVM { inner: file },
                global: global_vm,
                entities: entities_vm,
                features: features_vm,
                use_cases: use_cases_vm,
                dtos: dtos_vm,
            },
            false,
        ))
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_root_cargo_tera_template() {
        let tera = get_rust_tera();
        let mut context = Context::new();
        let snapshot = GenerationSnapshot {
            file: FileVM {
                inner: File {
                    id: 1,
                    name: "Cargo.toml".into(),
                    relative_path: "".into(),
                    group: "root".into(),
                    template_name: "root_cargo".into(),
                    feature: None,
                    entity: None,
                    use_case: None,
                },
            },
            global: GlobalVM {
                inner: Global {
                    id: 1,
                    language: "rust".into(),
                    application_name: "test".into(),
                    organisation_name: "test".into(),
                    organisation_domain: "test".into(),
                    prefix_path: "".into(),
                },
                application_kebab_name: "test".into(),
            },
            entities: IndexMap::new(),
            features: IndexMap::new(),
            use_cases: IndexMap::new(),
            dtos: IndexMap::new(),
        };
        context.insert("s", &snapshot);
        let code = tera.render("root_cargo", &context).unwrap();
        println!("{}", code);
    }

    #[test]
    fn test_entity_dtos_template_generation() {
        // Build a minimal snapshot with an entity bound to the file and a couple of fields
        use common::entities::FieldType;
        let entity_id: EntityId = 1;
        let file = File {
            id: 10,
            name: "User DTOs".to_string(),
            relative_path: "src/user_dtos.rs".to_string(),
            group: "entities".to_string(),
            template_name: "entity_dtos".to_string(),
            feature: None,
            entity: Some(entity_id),
            use_case: None,
        };
        let global = Global {
            id: 50,
            language: "".to_string(),
            application_name: "".to_string(),
            organisation_name: "".to_string(),
            organisation_domain: "".to_string(),
            prefix_path: "".to_string(),
        };
        let entity = Entity {
            id: entity_id,
            name: "User".to_string(),
            only_for_heritage: false,
            inherits_from: None,
            single_model: true,
            allow_direct_access: true,
            fields: vec![100, 101],
            relationships: vec![],
            undoable: true,
        };
        let field_relationship = Field {
            id: 100,
            name: "name".to_string(),
            field_type: FieldType::Entity,
            entity: Some(entity_id),
            relationship: FieldRelationshipType::OneToOne,
            required: false, // nullable
            strong: true,
            list_model: false,
            list_model_displayed_field: None,
            enum_name: None,
            enum_values: None,
        };
        let field_tags = Field {
            id: 101,
            name: "tags".to_string(),
            field_type: FieldType::String,
            entity: None,
            relationship: FieldRelationshipType::OneToMany,
            required: false,
            strong: true,
            list_model: false,
            list_model_displayed_field: None,
            enum_name: None,
            enum_values: None,
        };

        let snapshot = GenerationSnapshot {
            file: FileVM { inner: file },
            global: GlobalVM {
                inner: global,
                application_kebab_name: "".to_string(),
            },
            entities: {
                let mut m = IndexMap::new();
                // Build fields VM
                let fields_vm = vec![
                    FieldVM {
                        inner: field_relationship.clone(),
                        pascal_name: heck::AsPascalCase(&field_relationship.name).to_string(),
                        snake_name: heck::AsSnakeCase(&field_relationship.name).to_string(),
                        relationship: "OneToOne".to_string(),
                        required: false,
                        rust_base_type: "String".to_string(),
                        rust_type: "String".to_string(),
                    },
                    FieldVM {
                        inner: field_tags.clone(),
                        pascal_name: heck::AsPascalCase(&field_tags.name).to_string(),
                        snake_name: heck::AsSnakeCase(&field_tags.name).to_string(),
                        relationship: "OneToMany".to_string(),
                        required: false,
                        rust_base_type: "String".to_string(),
                        rust_type: "Vec<String>".to_string(),
                    },
                ];
                m.insert(
                    entity_id,
                    EntityVM {
                        inner: entity,
                        relationships: IndexMap::new(),
                        forward_relationships: IndexMap::new(),
                        backward_relationships: IndexMap::new(),
                        snake_name: "user".to_string(),
                        pascal_name: "User".to_string(),
                        fields: fields_vm,
                    },
                );
                m
            },
            features: IndexMap::new(),
            use_cases: IndexMap::new(),
            dtos: IndexMap::new(),
        };

        // Render directly with Tera to allow adding a workaround for `null` literal in template
        let tera = get_rust_tera();
        let mut context = Context::new();
        context.insert("s", &snapshot);
        // Workaround: the template compares against `null`, which Tera treats as an identifier; provide it explicitly
        context.insert("null", &serde_json::Value::Null);
        let code = tera
            .render("entity_dtos", &context)
            .expect("rendering entity_dtos");
        println!("{}", code);

        // Basic assertion: when file has no bound entity, template emits a clear comment
        assert!(!code.contains("No entity bound to this file"));
    }
}
