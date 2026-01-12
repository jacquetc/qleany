use crate::{ListRustFilesDto, ListRustFilesReturnDto};
use anyhow::{Result, anyhow};
use common::direct_access::feature::FeatureRelationshipField;
use common::direct_access::root::RootRelationshipField;
use common::entities::Entity;
use common::types::EntityId;
use common::{
    database::CommandUnitOfWork, entities::Feature, entities::File, entities::Global,
    entities::Relationship, entities::Root, entities::UseCase,
};
use common::direct_access::system::SystemRelationshipField;
use common::direct_access::workspace::WorkspaceRelationshipField;
use crate::use_cases::common::rust_code_generator::GenerationReadOps;
use crate::use_cases::common::tools;

pub trait ListRustFilesUnitOfWorkFactoryTrait {
    fn create(&self) -> Box<dyn ListRustFilesUnitOfWorkTrait>;
}

#[macros::uow_action(entity = "Root", action = "GetMulti")]
#[macros::uow_action(entity = "Root", action = "GetRelationship")]
#[macros::uow_action(entity = "Workspace", action = "GetRelationship")]
#[macros::uow_action(entity = "System", action = "GetRelationship")]
#[macros::uow_action(entity = "System", action = "SetRelationship")]
#[macros::uow_action(entity = "Global", action = "Get")]
#[macros::uow_action(entity = "Entity", action = "GetMulti")]
#[macros::uow_action(entity = "Entity", action = "GetRelationship")]
#[macros::uow_action(entity = "Relationship", action = "GetMulti")]
#[macros::uow_action(entity = "Feature", action = "GetMulti")]
#[macros::uow_action(entity = "Feature", action = "GetRelationship")]
#[macros::uow_action(entity = "UseCase", action = "GetMulti")]
#[macros::uow_action(entity = "File", action = "Create")]
#[macros::uow_action(entity = "File", action = "CreateMulti")]
#[macros::uow_action(entity = "File", action = "DeleteMulti")]
pub trait ListRustFilesUnitOfWorkTrait: CommandUnitOfWork {}

pub struct ListRustFilesUseCase {
    uow_factory: Box<dyn ListRustFilesUnitOfWorkFactoryTrait>,
}

impl ListRustFilesUseCase {
    pub fn new(uow_factory: Box<dyn ListRustFilesUnitOfWorkFactoryTrait>) -> Self {
        ListRustFilesUseCase { uow_factory }
    }

    pub fn execute(&mut self, dto: &ListRustFilesDto) -> Result<ListRustFilesReturnDto> {
        // TODO: implement only_existing
        let only_existing = dto.only_list_already_existing;
        let mut files: Vec<File> = vec![];

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        use anyhow::anyhow;
        let roots = uow.get_root_multi(&vec![])?;
        let root = roots
            .into_iter()
            .filter_map(|r| r)
            .next()
            .ok_or_else(|| anyhow!("Root entity not found"))?;

        let all_workspace_ids = uow.get_root_relationship(
            &root.id,
            &common::direct_access::root::RootRelationshipField::Workspace
        )?;

        let workspace_id = all_workspace_ids.first().cloned().ok_or(anyhow!("No workspace found"))?;
        
        let all_system_ids = uow.get_root_relationship(
            &root.id,
            &common::direct_access::root::RootRelationshipField::System
        )?;
        
        let system_id = all_system_ids.first().cloned().ok_or(anyhow!("No system found"))?;

        // Get global
        let globals = uow.get_workspace_relationship(&workspace_id, &WorkspaceRelationshipField::Global)?;
        let global_id = globals.first().ok_or(anyhow!("No global found"))?;
        let global = uow.get_global(&global_id)?;
        let global = global.ok_or(anyhow!("Global not found"))?;
        if global.language != "rust" {
            return Err(anyhow!("Global language is not rust"));
        }

        // remove all files from system
        let all_previous_files =
            uow.get_system_relationship(&root.id, &SystemRelationshipField::Files)?;
        if !all_previous_files.is_empty() {
            uow.delete_file_multi(&all_previous_files)?;
        }

        files.push(File {
            id: 0,
            name: "Cargo.toml".to_string(),
            relative_path: "".to_string(),
            group: "base".to_string(),
            template_name: "root_cargo".to_string(),
            feature: Some(0),
            entity: None,
            use_case: None,
        });

        files.push(File {
            id: 0,
            name: "Cargo.toml".to_string(),
            relative_path: "crates/common/".to_string(),
            group: "base".to_string(),
            template_name: "common_cargo".to_string(),
            feature: None,
            entity: None,
            use_case: None,
        });

        files.push(File {
            id: 0,
            name: "lib.rs".to_string(),
            relative_path: "crates/common/src/".to_string(),
            group: "base".to_string(),
            template_name: "common_lib".to_string(),
            feature: None,
            entity: None,
            use_case: None,
        });

        files.push(File {
            id: 0,
            name: "undo_redo.rs".to_string(),
            relative_path: "crates/common/src/".to_string(),
            group: "base".to_string(),
            template_name: "undo_redo".to_string(),
            feature: None,
            entity: None,
            use_case: None,
        });

        files.push(File {
            id: 0,
            name: "long_operation.rs".to_string(),
            relative_path: "crates/common/src/".to_string(),
            group: "base".to_string(),
            template_name: "long_operation".to_string(),
            feature: None,
            entity: None,
            use_case: None,
        });

        files.push(File {
            id: 0,
            name: "repository_factory.rs".to_string(),
            relative_path: "crates/common/src/direct_access/".to_string(),
            group: "base".to_string(),
            template_name: "repository_factory".to_string(),
            feature: None,
            entity: Some(0),
            use_case: None,
        });

        files.push(File {
            id: 0,
            name: "setup.rs".to_string(),
            relative_path: "crates/common/src/direct_access/".to_string(),
            group: "base".to_string(),
            template_name: "common_setup".to_string(),
            feature: None,
            entity: Some(0),
            use_case: None,
        });

        files.push(File {
            id: 0,
            name: "types.rs".to_string(),
            relative_path: "crates/common/src/".to_string(),
            group: "base".to_string(),
            template_name: "types".to_string(),
            feature: None,
            entity: None,
            use_case: None,
        });

        files.push(File {
            id: 0,
            name: "database.rs".to_string(),
            relative_path: "crates/common/src/".to_string(),
            group: "base".to_string(),
            template_name: "database".to_string(),
            feature: None,
            entity: None,
            use_case: None,
        });

        files.push(File {
            id: 0,
            name: "db_context.rs".to_string(),
            relative_path: "crates/common/src/database/".to_string(),
            group: "base".to_string(),
            template_name: "db_context".to_string(),
            feature: None,
            entity: None,
            use_case: None,
        });

        files.push(File {
            id: 0,
            name: "db_helpers.rs".to_string(),
            relative_path: "crates/common/src/database/".to_string(),
            group: "base".to_string(),
            template_name: "db_helpers".to_string(),
            feature: None,
            entity: None,
            use_case: None,
        });

        files.push(File {
            id: 0,
            name: "transactions.rs".to_string(),
            relative_path: "crates/common/src/database/".to_string(),
            group: "base".to_string(),
            template_name: "transactions".to_string(),
            feature: None,
            entity: None,
            use_case: None,
        });

        files.push(File {
            id: 0,
            name: "redb_tests.rs".to_string(),
            relative_path: "crates/common/tests/".to_string(),
            group: "base".to_string(),
            template_name: "redb_tests".to_string(),
            feature: None,
            entity: None,
            use_case: None,
        });

        files.push(File {
            id: 0,
            name: "undo_redo_tests.rs".to_string(),
            relative_path: "crates/common/tests/".to_string(),
            group: "base".to_string(),
            template_name: "undo_redo_tests".to_string(),
            feature: None,
            entity: None,
            use_case: None,
        });

        // direct access entities

        files.push(File {
            id: 0,
            name: "entities.rs".to_string(),
            relative_path: "crates/common/src/".to_string(),
            group: "entities".to_string(),
            template_name: "common_entities".to_string(),
            feature: None,
            entity: Some(0), // 0 means all
            use_case: None,
        });

        files.push(File {
            id: 0,
            name: "event.rs".to_string(),
            relative_path: "crates/common/src/".to_string(),
            group: "base".to_string(),
            template_name: "common_event".to_string(),
            feature: Some(0), // 0 means all
            entity: Some(0),  // 0 means all
            use_case: None,
        });

        files.push(File {
            id: 0,
            name: "direct_access.rs".to_string(),
            relative_path: "crates/common/src/".to_string(),
            group: "entities".to_string(),
            template_name: "common_direct_access_mod".to_string(),
            feature: None,
            entity: Some(0), // 0 means all
            use_case: None,
        });

        files.push(File {
            id: 0,
            name: "Cargo.toml".to_string(),
            relative_path: "crates/direct_access/".to_string(),
            group: "entities".to_string(),
            template_name: "direct_access_cargo".to_string(),
            feature: None,
            entity: None,
            use_case: None,
        });

        files.push(File {
            id: 0,
            name: "lib.rs".to_string(),
            relative_path: "crates/direct_access/src/".to_string(),
            group: "entities".to_string(),
            template_name: "direct_access_lib".to_string(),
            feature: None,
            entity: Some(0), // 0 means all
            use_case: None,
        });

        // Get entities
        let entities = uow.get_workspace_relationship(&workspace_id, &WorkspaceRelationshipField::Entities)?;
        let entities = uow.get_entity_multi(&entities)?;

        for entity in entities {
            let entity = entity.ok_or(anyhow!("Entity not found"))?;

            // continue if entity is "heritage"
            if entity.only_for_heritage {
                continue;
            }

            if entity.allow_direct_access {
                // for crates/direct_access/src/

                files.push(File {
                    id: 0,
                    name: format!("{}.rs", heck::AsSnakeCase(&entity.name)),
                    relative_path: "crates/direct_access/src/".to_string(),
                    group: "entities".to_string(),
                    template_name: "entity_mod".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                });

                let relative_path = format!(
                    "crates/direct_access/src/{}/",
                    heck::AsSnakeCase(&entity.name)
                );

                files.push(File {
                    id: 0,
                    name: "dtos.rs".to_string(),
                    relative_path: relative_path.clone(),
                    group: "entities".to_string(),
                    template_name: "entity_dtos".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                });

                files.push(File {
                    id: 0,
                    name: "use_cases.rs".to_string(),
                    relative_path: relative_path.clone(),
                    group: "entities".to_string(),
                    template_name: "entity_use_cases_mod".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                });

                files.push(File {
                    id: 0,
                    name: "units_of_work.rs".to_string(),
                    relative_path: relative_path.clone(),
                    group: "entities".to_string(),
                    template_name: "entity_units_of_work".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                });

                files.push(File {
                    id: 0,
                    name: format!("{}_controller.rs", heck::AsSnakeCase(&entity.name)),
                    relative_path: relative_path.clone(),
                    group: "entities".to_string(),
                    template_name: "entity_controller".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                });

                // for crates/direct_access/src/{}/use_cases/

                let relative_path = format!(
                    "crates/direct_access/src/{}/use_cases/",
                    heck::AsSnakeCase(&entity.name)
                );

                files.push(File {
                    id: 0,
                    name: format!("get_{}_uc.rs", heck::AsSnakeCase(&entity.name)),
                    relative_path: relative_path.clone(),
                    group: "entities".to_string(),
                    template_name: "entity_get_use_case".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                });

                files.push(File {
                    id: 0,
                    name: format!("get_{}_multi_uc.rs", heck::AsSnakeCase(&entity.name)),
                    relative_path: relative_path.clone(),
                    group: "entities".to_string(),
                    template_name: "entity_get_multi_use_case".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                });

                files.push(File {
                    id: 0,
                    name: format!("create_{}_uc.rs", heck::AsSnakeCase(&entity.name)),
                    relative_path: relative_path.clone(),
                    group: "entities".to_string(),
                    template_name: "entity_create_use_case".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                });

                files.push(File {
                    id: 0,
                    name: format!("create_{}_multi_uc.rs", heck::AsSnakeCase(&entity.name)),
                    relative_path: relative_path.clone(),
                    group: "entities".to_string(),
                    template_name: "entity_create_multi_use_case".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                });

                files.push(File {
                    id: 0,
                    name: format!("update_{}_multi_uc.rs", heck::AsSnakeCase(&entity.name)),
                    relative_path: relative_path.clone(),
                    group: "entities".to_string(),
                    template_name: "entity_update_multi_use_case".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                });

                files.push(File {
                    id: 0,
                    name: format!("update_{}_uc.rs", heck::AsSnakeCase(&entity.name)),
                    relative_path: relative_path.clone(),
                    group: "entities".to_string(),
                    template_name: "entity_update_use_case".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                });

                files.push(File {
                    id: 0,
                    name: format!("remove_{}_multi_uc.rs", heck::AsSnakeCase(&entity.name)),
                    relative_path: relative_path.clone(),
                    group: "entities".to_string(),
                    template_name: "entity_remove_multi_use_case".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                });

                files.push(File {
                    id: 0,
                    name: format!("remove_{}_uc.rs", heck::AsSnakeCase(&entity.name)),
                    relative_path: relative_path.clone(),
                    group: "entities".to_string(),
                    template_name: "entity_remove_use_case".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                });

                // only if there is a forward relationship
                let relationships = uow.get_entity_relationship(
                    &entity.id,
                    &common::direct_access::entity::EntityRelationshipField::Relationships,
                )?;
                let relationships = uow.get_relationship_multi(&relationships)?;
                let has_forward_relationship = relationships.iter().any(|r| {
                    if let Some(r) = r {
                        r.direction == common::entities::Direction::Forward
                    } else {
                        false
                    }
                });

                if has_forward_relationship {
                    files.push(File {
                        id: 0,
                        name: format!("get_{}_relationship_uc.rs", heck::AsSnakeCase(&entity.name)),
                        relative_path: relative_path.clone(),
                        group: "entities".to_string(),
                        template_name: "entity_get_relationship_use_case".to_string(),
                        feature: None,
                        entity: Some(entity.id),
                        use_case: None,
                    });

                    files.push(File {
                        id: 0,
                        name: format!("set_{}_relationship_uc.rs", heck::AsSnakeCase(&entity.name)),
                        relative_path: relative_path.clone(),
                        group: "entities".to_string(),
                        template_name: "entity_set_relationship_use_case".to_string(),
                        feature: None,
                        entity: Some(entity.id),
                        use_case: None,
                    });
                }
            }

            // for crates/common/src/direct_access/
            let relative_path = "crates/common/src/direct_access/";

            files.push(File {
                id: 0,
                name: format!("{}.rs", heck::AsSnakeCase(&entity.name)),
                relative_path: relative_path.to_string(),
                group: "entities".to_string(),
                template_name: "common_entity_mod".to_string(),
                feature: None,
                entity: Some(entity.id),
                use_case: None,
            });

            files.push(File {
                id: 0,
                name: format!("{}_repository.rs", heck::AsSnakeCase(&entity.name)),
                relative_path: format!("{}{}/", relative_path, heck::AsSnakeCase(&entity.name)),
                group: "entities".to_string(),
                template_name: "common_entity_repository".to_string(),
                feature: None,
                entity: Some(entity.id),
                use_case: None,
            });

            files.push(File {
                id: 0,
                name: format!("{}_table.rs", heck::AsSnakeCase(&entity.name)),
                relative_path: format!("{}{}/", relative_path, heck::AsSnakeCase(&entity.name)),
                group: "entities".to_string(),
                template_name: "common_entity_table".to_string(),
                feature: None,
                entity: Some(entity.id),
                use_case: None,
            })
        }

        // features:
        let features = uow.get_workspace_relationship(&workspace_id, &WorkspaceRelationshipField::Features)?;

        let features = uow.get_feature_multi(&features)?;

        for feature in features {
            let feature = feature.ok_or(anyhow!("Feature not found"))?;

            let relative_path = format!("crates/{}/", heck::AsSnakeCase(&feature.name));

            files.push(File {
                id: 0,
                name: "Cargo.toml".to_string(),
                relative_path: relative_path.clone(),
                group: "features".to_string(),
                template_name: "feature_cargo".to_string(),
                feature: Some(feature.id),
                entity: None,
                use_case: None,
            });

            let relative_path = format!("crates/{}/src/", heck::AsSnakeCase(&feature.name));

            files.push(File {
                id: 0,
                name: "lib.rs".to_string(),
                relative_path: relative_path.clone(),
                group: "features".to_string(),
                template_name: "feature_lib".to_string(),
                feature: Some(feature.id),
                entity: None,
                use_case: None,
            });

            files.push(File {
                id: 0,
                name: "use_cases.rs".to_string(),
                relative_path: relative_path.clone(),
                group: "features".to_string(),
                template_name: "feature_use_cases_mod".to_string(),
                feature: Some(feature.id),
                entity: None,
                use_case: None,
            });

            files.push(File {
                id: 0,
                name: "dtos.rs".to_string(),
                relative_path: relative_path.clone(),
                group: "features".to_string(),
                template_name: "feature_dtos".to_string(),
                feature: Some(feature.id),
                entity: None,
                use_case: None,
            });

            files.push(File {
                id: 0,
                name: "units_of_work.rs".to_string(),
                relative_path: relative_path.clone(),
                group: "features".to_string(),
                template_name: "feature_units_of_work_mod".to_string(),
                feature: Some(feature.id),
                entity: None,
                use_case: None,
            });

            files.push(File {
                id: 0,
                name: format!("{}_controller.rs", heck::AsSnakeCase(&feature.name)),
                relative_path: relative_path.clone(),
                group: "features".to_string(),
                template_name: "feature_controller".to_string(),
                feature: Some(feature.id),
                entity: None,
                use_case: None,
            });

            // for crates/{}/src/use_cases/
            let relative_path =
                format!("crates/{}/src/use_cases/", heck::AsSnakeCase(&feature.name));

            let use_cases =
                uow.get_feature_relationship(&feature.id, &FeatureRelationshipField::UseCases)?;
            let use_cases = uow.get_use_case_multi(&use_cases)?;

            for use_case in &use_cases {
                let use_case = use_case.clone().ok_or(anyhow!("Use case not found"))?;

                files.push(File {
                    id: 0,
                    name: format!("{}_uc.rs", heck::AsSnakeCase(&use_case.name)),
                    relative_path: relative_path.clone(),
                    group: "features".to_string(),
                    template_name: "feature_use_case".to_string(),
                    feature: Some(feature.id),
                    entity: None,
                    use_case: Some(use_case.id),
                });
            }

            // for crates/{}/src/units_of_work/
            let relative_path = format!(
                "crates/{}/src/units_of_work/",
                heck::AsSnakeCase(&feature.name)
            );

            for use_case in use_cases {
                let use_case = use_case.ok_or(anyhow!("Use case not found"))?;

                files.push(File {
                    id: 0,
                    name: format!("{}_uow.rs", heck::AsSnakeCase(&use_case.name)),
                    relative_path: relative_path.clone(),
                    group: "features".to_string(),
                    template_name: "feature_use_case_uow".to_string(),
                    feature: Some(feature.id),
                    entity: None,
                    use_case: Some(use_case.id),
                });
            }
        }

        // macros in crates/macros/

        files.push(File {
            id: 0,
            name: "Cargo.toml".to_string(),
            relative_path: "crates/macros/".to_string(),
            group: "base".to_string(),
            template_name: "macros_cargo".to_string(),
            feature: None,
            entity: None,
            use_case: None,
        });

        files.push(File {
            id: 0,
            name: "lib.rs".to_string(),
            relative_path: "crates/macros/src/".to_string(),
            group: "base".to_string(),
            template_name: "macros_lib".to_string(),
            feature: None,
            entity: None,
            use_case: None,
        });

        files.push(File {
            id: 0,
            name: "direct_access.rs".to_string(),
            relative_path: "crates/macros/src/".to_string(),
            group: "base".to_string(),
            template_name: "macros_direct_access".to_string(),
            feature: None,
            entity: None,
            use_case: None,
        });

        files.push(File {
            id: 0,
            name: "Cargo.toml".to_string(),
            relative_path: "crates/cli/".to_string(),
            group: "cli".to_string(),
            template_name: "cli_cargo".to_string(),
            feature: None,
            entity: None,
            use_case: None,
        });

        files.push(File {
            id: 0,
            name: "main.rs".to_string(),
            relative_path: "crates/cli/src/".to_string(),
            group: "cli".to_string(),
            template_name: "cli_main".to_string(),
            feature: None,
            entity: None,
            use_case: None,
        });

        //TODO: add files for UIs

        let created_files = uow.create_file_multi(&files)?;
        uow.set_system_relationship(
            &system_id,
            &SystemRelationshipField::Files,
            &created_files
                .iter()
                .map(|f| f.id)
                .collect::<Vec<EntityId>>(),
        )?;

        uow.commit()?;

        let mut file_ids = vec![];
        let mut file_names = vec![];
        let mut file_groups = vec![];

        for file in created_files {
            file_ids.push(file.id);
            file_names.push(format!("{}{}", file.relative_path, file.name));
            file_groups.push(file.group.clone());
        }

        Ok(ListRustFilesReturnDto {
            file_ids,
            file_names,
            file_groups,
        })
    }
}
