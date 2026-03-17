use crate::use_cases::common::tools;
use crate::{FillCppQtFilesDto, FillCppQtFilesReturnDto};
use anyhow::Result;
use common::direct_access::feature::FeatureRelationshipField;
use common::direct_access::system::SystemRelationshipField;
use common::direct_access::workspace::WorkspaceRelationshipField;
use common::entities::UserInterface;
use common::entities::{Entity, FileNature};
use common::generator::file_list_builder::FileListBuilder;
use common::types::EntityId;
use common::{
    database::CommandUnitOfWork, entities::Feature, entities::Field, entities::File,
    entities::Global, entities::Relationship, entities::Root, entities::UseCase,
};
use heck::ToTitleCase;

pub trait FillCppQtFilesUnitOfWorkFactoryTrait {
    fn create(&self) -> Box<dyn FillCppQtFilesUnitOfWorkTrait>;
}

#[macros::uow_action(entity = "Root", action = "GetAll")]
#[macros::uow_action(entity = "Root", action = "GetRelationship")]
#[macros::uow_action(entity = "Workspace", action = "GetRelationship")]
#[macros::uow_action(entity = "System", action = "GetRelationship")]
#[macros::uow_action(entity = "System", action = "SetRelationship")]
#[macros::uow_action(entity = "Global", action = "Get")]
#[macros::uow_action(entity = "UserInterface", action = "Get")]
#[macros::uow_action(entity = "Entity", action = "GetMulti")]
#[macros::uow_action(entity = "Entity", action = "GetRelationship")]
#[macros::uow_action(entity = "Field", action = "GetMulti")]
#[macros::uow_action(entity = "Relationship", action = "GetMulti")]
#[macros::uow_action(entity = "Feature", action = "GetMulti")]
#[macros::uow_action(entity = "Feature", action = "GetRelationship")]
#[macros::uow_action(entity = "UseCase", action = "GetMulti")]
#[macros::uow_action(entity = "File", action = "CreateOrphan")]
#[macros::uow_action(entity = "File", action = "CreateOrphanMulti")]
#[macros::uow_action(entity = "File", action = "RemoveMulti")]
pub trait FillCppQtFilesUnitOfWorkTrait: CommandUnitOfWork {}

pub struct FillCppQtFilesUseCase {
    uow_factory: Box<dyn FillCppQtFilesUnitOfWorkFactoryTrait>,
}

impl FillCppQtFilesUseCase {
    pub fn new(uow_factory: Box<dyn FillCppQtFilesUnitOfWorkFactoryTrait>) -> Self {
        FillCppQtFilesUseCase { uow_factory }
    }

    pub fn execute(&mut self, dto: &FillCppQtFilesDto) -> Result<FillCppQtFilesReturnDto> {
        let mut b = FileListBuilder::new();

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        use anyhow::anyhow;
        let roots = uow.get_all_root()?;
        let root = roots
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("Root entity not found"))?;

        let all_workspace_ids = uow.get_root_relationship(
            &root.id,
            &common::direct_access::root::RootRelationshipField::Workspace,
        )?;

        let workspace_id = all_workspace_ids
            .first()
            .cloned()
            .ok_or(anyhow!("No workspace found"))?;

        let all_system_ids = uow.get_root_relationship(
            &root.id,
            &common::direct_access::root::RootRelationshipField::System,
        )?;

        let system_id = all_system_ids
            .first()
            .cloned()
            .ok_or(anyhow!("No system found"))?;

        // Get global
        let globals =
            uow.get_workspace_relationship(&workspace_id, &WorkspaceRelationshipField::Global)?;
        let global_id = globals.first().ok_or(anyhow!("No global found"))?;
        let global = uow.get_global(global_id)?;
        let global = global.ok_or(anyhow!("Global not found"))?;
        if global.language != "cpp-qt" {
            return Err(anyhow!("Global language is not cpp-qt"));
        };

        // get prefix path
        let prefix = global.prefix_path.clone();
        // strip it from leading and trailing "/" or "\"
        let prefix = if prefix.trim().is_empty() {
            "src".to_string()
        } else {
            tools::strip_leading_and_trailing_slashes(&prefix)
        };

        // ui
        let user_interfaces = uow.get_workspace_relationship(
            &workspace_id,
            &WorkspaceRelationshipField::UserInterface,
        )?;
        let ui_id = user_interfaces
            .first()
            .ok_or(anyhow!("No user interface found"))?;
        let ui = uow
            .get_user_interface(ui_id)?
            .ok_or(anyhow!("User interface not found"))?;

        // remove all files from system
        let all_previous_files =
            uow.get_system_relationship(&root.id, &SystemRelationshipField::Files)?;
        if !all_previous_files.is_empty() {
            uow.remove_file_multi(&all_previous_files)?;
        }

        b.add(
            "CMakeLists.txt",
            "",
            "base",
            "root_cmake",
            FileNature::Aggregate,
        )
        .all_features = true;
        b.add(
            "CMakeLists.txt",
            format!("{}/common/", prefix),
            "base",
            "common_cmake",
            FileNature::Aggregate,
        );
        b.add(
            "InstallHelpers.cmake",
            "cmake/",
            "base",
            "install_helpers_cmake",
            FileNature::Aggregate,
        );
        b.add(
            "VersionFromGit.cmake",
            "cmake/",
            "base",
            "version_from_git_cmake",
            FileNature::Aggregate,
        );
        b.add(
            "service_locator.h",
            format!("{}/common/", prefix),
            "base",
            "service_locator_h",
            FileNature::Infrastructure,
        );
        b.add(
            "service_locator.cpp",
            format!("{}/common/", prefix),
            "base",
            "service_locator_cpp",
            FileNature::Infrastructure,
        );
        b.add(
            "controller_command_helpers.h",
            format!("{}/common/", prefix),
            "base",
            "controller_command_helpers_h",
            FileNature::Infrastructure,
        );
        b.add(
            "signal_buffer.h",
            format!("{}/common/", prefix),
            "base",
            "signal_buffer_h",
            FileNature::Infrastructure,
        );
        b.add(
            "app_bootstrap.h",
            format!("{}/common/frontend", prefix),
            "base",
            "app_bootstrap_h",
            FileNature::Infrastructure,
        );
        b.add(
            "app_bootstrap.cpp",
            format!("{}/common/frontend", prefix),
            "base",
            "app_bootstrap_cpp",
            FileNature::Infrastructure,
        );
        b.add(
            "CMakeLists.txt",
            format!("{}/common/frontend", prefix),
            "base",
            "common_frontend_cmake",
            FileNature::Aggregate,
        );

        // database

        b.add(
            "db_builder.h",
            format!("{}/common/database", prefix),
            "common_db",
            "db_builder_h",
            FileNature::Infrastructure,
        )
        .all_entities = true;
        b.add(
            "cache_registry.h",
            format!("{}/common/database", prefix),
            "common_db",
            "cache_registry_h",
            FileNature::Infrastructure,
        );
        b.add(
            "db_context.h",
            format!("{}/common/database", prefix),
            "common_db",
            "db_context_h",
            FileNature::Infrastructure,
        );
        b.add(
            "table_cache.h",
            format!("{}/common/database", prefix),
            "common_db",
            "table_cache_h",
            FileNature::Infrastructure,
        );
        b.add(
            "junction_cache.h",
            format!("{}/common/database/junction_table_ops/", prefix),
            "common_db",
            "junction_cache_h",
            FileNature::Infrastructure,
        );
        b.add(
            "snapshot_types.h",
            format!("{}/common/database", prefix),
            "common_db",
            "snapshot_types_h",
            FileNature::Infrastructure,
        );
        b.add(
            "unordered_many_to_many.h",
            format!("{}/common/database/junction_table_ops/", prefix),
            "common_db",
            "unordered_many_to_many_h",
            FileNature::Infrastructure,
        );
        b.add(
            "unordered_many_to_many.cpp",
            format!("{}/common/database/junction_table_ops/", prefix),
            "common_db",
            "unordered_many_to_many_cpp",
            FileNature::Infrastructure,
        );
        b.add(
            "unordered_one_to_many.h",
            format!("{}/common/database/junction_table_ops/", prefix),
            "common_db",
            "unordered_one_to_many_h",
            FileNature::Infrastructure,
        );
        b.add(
            "unordered_one_to_many.cpp",
            format!("{}/common/database/junction_table_ops/", prefix),
            "common_db",
            "unordered_one_to_many_cpp",
            FileNature::Infrastructure,
        );
        b.add(
            "ordered_one_to_many.h",
            format!("{}/common/database/junction_table_ops/", prefix),
            "common_db",
            "ordered_one_to_many_h",
            FileNature::Infrastructure,
        );
        b.add(
            "ordered_one_to_many.cpp",
            format!("{}/common/database/junction_table_ops/", prefix),
            "common_db",
            "ordered_one_to_many_cpp",
            FileNature::Infrastructure,
        );
        b.add(
            "one_to_one.h",
            format!("{}/common/database/junction_table_ops/", prefix),
            "common_db",
            "one_to_one_h",
            FileNature::Infrastructure,
        );
        b.add(
            "one_to_one.cpp",
            format!("{}/common/database/junction_table_ops/", prefix),
            "common_db",
            "one_to_one_cpp",
            FileNature::Infrastructure,
        );
        b.add(
            "many_to_one.h",
            format!("{}/common/database/junction_table_ops/", prefix),
            "common_db",
            "many_to_one_h",
            FileNature::Infrastructure,
        );
        b.add(
            "many_to_one.cpp",
            format!("{}/common/database/junction_table_ops/", prefix),
            "common_db",
            "many_to_one_cpp",
            FileNature::Infrastructure,
        );

        // undo redo

        b.add(
            "group_command.cpp",
            format!("{}/common/undo_redo/", prefix),
            "common_undo",
            "group_command_cpp",
            FileNature::Infrastructure,
        );
        b.add(
            "group_command.h",
            format!("{}/common/undo_redo/", prefix),
            "common_undo",
            "group_command_h",
            FileNature::Infrastructure,
        );
        b.add(
            "group_command_builder.cpp",
            format!("{}/common/undo_redo/", prefix),
            "common_undo",
            "group_command_builder_cpp",
            FileNature::Infrastructure,
        );
        b.add(
            "group_command_builder.h",
            format!("{}/common/undo_redo/", prefix),
            "common_undo",
            "group_command_builder_h",
            FileNature::Infrastructure,
        );
        b.add(
            "query_handler.cpp",
            format!("{}/common/undo_redo/", prefix),
            "common_undo",
            "query_handler_cpp",
            FileNature::Infrastructure,
        );
        b.add(
            "query_handler.h",
            format!("{}/common/undo_redo/", prefix),
            "common_undo",
            "query_handler_h",
            FileNature::Infrastructure,
        );
        b.add(
            "undo_redo_command.cpp",
            format!("{}/common/undo_redo/", prefix),
            "common_undo",
            "undo_redo_command_cpp",
            FileNature::Infrastructure,
        );
        b.add(
            "undo_redo_command.h",
            format!("{}/common/undo_redo/", prefix),
            "common_undo",
            "undo_redo_command_h",
            FileNature::Infrastructure,
        );
        b.add(
            "undo_redo_manager.cpp",
            format!("{}/common/undo_redo/", prefix),
            "common_undo",
            "undo_redo_manager_cpp",
            FileNature::Infrastructure,
        );
        b.add(
            "undo_redo_manager.h",
            format!("{}/common/undo_redo/", prefix),
            "common_undo",
            "undo_redo_manager_h",
            FileNature::Infrastructure,
        );
        b.add(
            "undo_redo_stack.cpp",
            format!("{}/common/undo_redo/", prefix),
            "common_undo",
            "undo_redo_stack_cpp",
            FileNature::Infrastructure,
        );
        b.add(
            "undo_redo_stack.h",
            format!("{}/common/undo_redo/", prefix),
            "common_undo",
            "undo_redo_stack_h",
            FileNature::Infrastructure,
        );
        b.add(
            "undo_redo_system.cpp",
            format!("{}/common/undo_redo/", prefix),
            "common_undo",
            "undo_redo_system_cpp",
            FileNature::Infrastructure,
        );
        b.add(
            "undo_redo_system.h",
            format!("{}/common/undo_redo/", prefix),
            "common_undo",
            "undo_redo_system_h",
            FileNature::Infrastructure,
        );

        // common unit of work macros
        b.add(
            "uow_macros.h",
            format!("{}/common/unit_of_work/", prefix),
            "common_unit_of_work",
            "uow_macros_h",
            FileNature::Infrastructure,
        );
        b.add(
            "unit_of_work.h",
            format!("{}/common/unit_of_work/", prefix),
            "common_unit_of_work",
            "unit_of_work_h",
            FileNature::Infrastructure,
        );
        b.add(
            "uow_base.h",
            format!("{}/common/unit_of_work/", prefix),
            "common_unit_of_work",
            "uow_base_h",
            FileNature::Infrastructure,
        );
        b.add(
            "uow_ops.h",
            format!("{}/common/unit_of_work/", prefix),
            "common_unit_of_work",
            "uow_ops_h",
            FileNature::Infrastructure,
        );

        // direct access

        b.add(
            "event_registry.h",
            format!("{}/common/direct_access/", prefix),
            "common_direct_access",
            "event_registry_h",
            FileNature::Aggregate,
        )
        .all_entities = true;
        b.add(
            "repository_factory.cpp",
            format!("{}/common/direct_access/", prefix),
            "common_direct_access",
            "repository_factory_cpp",
            FileNature::Aggregate,
        )
        .all_entities = true;
        b.add(
            "repository_factory.h",
            format!("{}/common/direct_access/", prefix),
            "common_direct_access",
            "repository_factory_h",
            FileNature::Aggregate,
        )
        .all_entities = true;
        b.add(
            "mapper_tools.h",
            format!("{}/common/direct_access/", prefix),
            "common_direct_access",
            "mapper_tools_h",
            FileNature::Infrastructure,
        );
        b.add(
            "converter_registration.h",
            format!("{}/common/direct_access/", prefix),
            "common_direct_access",
            "converter_registration_h",
            FileNature::Infrastructure,
        );
        b.add(
            "operators.h",
            format!("{}/common/direct_access/", prefix),
            "common_direct_access",
            "operators_h",
            FileNature::Infrastructure,
        );
        b.add(
            "CMakeLists.txt",
            format!("{}/common/direct_access/", prefix),
            "common_direct_access",
            "common_direct_access_cmake",
            FileNature::Aggregate,
        )
        .all_entities = true;
        b.add(
            "uc_concepts.h",
            format!("{}/common/direct_access/use_case_helpers/", prefix),
            "common_direct_access",
            "uc_helper_concepts_h",
            FileNature::Infrastructure,
        );
        b.add(
            "create_orphans_uc.h",
            format!("{}/common/direct_access/use_case_helpers/", prefix),
            "common_direct_access",
            "uc_helper_create_orphans_h",
            FileNature::Infrastructure,
        );
        b.add(
            "create_uc.h",
            format!("{}/common/direct_access/use_case_helpers/", prefix),
            "common_direct_access",
            "uc_helper_create_h",
            FileNature::Infrastructure,
        );
        b.add(
            "get_uc.h",
            format!("{}/common/direct_access/use_case_helpers/", prefix),
            "common_direct_access",
            "uc_helper_get_h",
            FileNature::Infrastructure,
        );
        b.add(
            "get_all_uc.h",
            format!("{}/common/direct_access/use_case_helpers/", prefix),
            "common_direct_access",
            "uc_helper_get_all_h",
            FileNature::Infrastructure,
        );
        b.add(
            "update_uc.h",
            format!("{}/common/direct_access/use_case_helpers/", prefix),
            "common_direct_access",
            "uc_helper_update_h",
            FileNature::Infrastructure,
        );
        b.add(
            "remove_uc.h",
            format!("{}/common/direct_access/use_case_helpers/", prefix),
            "common_direct_access",
            "uc_helper_remove_h",
            FileNature::Infrastructure,
        );
        b.add(
            "get_relationship_ids_uc.h",
            format!("{}/common/direct_access/use_case_helpers/", prefix),
            "common_direct_access",
            "uc_helper_get_relationship_ids_h",
            FileNature::Infrastructure,
        );
        b.add(
            "get_relationship_ids_many_uc.h",
            format!("{}/common/direct_access/use_case_helpers/", prefix),
            "common_direct_access",
            "uc_helper_get_relationship_ids_many_h",
            FileNature::Infrastructure,
        );
        b.add(
            "get_relationship_ids_count_uc.h",
            format!("{}/common/direct_access/use_case_helpers/", prefix),
            "common_direct_access",
            "uc_helper_get_relationship_ids_count_h",
            FileNature::Infrastructure,
        );
        b.add(
            "get_relationship_ids_in_range_uc.h",
            format!("{}/common/direct_access/use_case_helpers/", prefix),
            "common_direct_access",
            "uc_helper_get_relationship_ids_in_range_h",
            FileNature::Infrastructure,
        );
        b.add(
            "set_relationship_ids_uc.h",
            format!("{}/common/direct_access/use_case_helpers/", prefix),
            "common_direct_access",
            "uc_helper_set_relationship_ids_h",
            FileNature::Infrastructure,
        );
        b.add(
            "move_relationship_ids_uc.h",
            format!("{}/common/direct_access/use_case_helpers/", prefix),
            "common_direct_access",
            "uc_helper_move_relationship_ids_h",
            FileNature::Infrastructure,
        );
        b.add(
            "CMakeLists.txt",
            format!("{}/common/entities/", prefix),
            "common_direct_access",
            "common_entities_cmake",
            FileNature::Aggregate,
        )
        .all_entities = true;
        b.add(
            "CMakeLists.txt",
            format!("{}/direct_access/", prefix),
            "direct_access",
            "direct_access_cmake",
            FileNature::Aggregate,
        )
        .all_entities = true;

        // Get entities
        let entities =
            uow.get_workspace_relationship(&workspace_id, &WorkspaceRelationshipField::Entities)?;
        let entities = uow.get_entity_multi(&entities)?;

        for entity in &entities {
            let entity = entity.as_ref().ok_or(anyhow!("Entity not found"))?;

            // skip if entity is "heritage"
            if entity.only_for_heritage {
                continue;
            }

            let entity_snake_name = heck::AsSnakeCase(&entity.name);
            let entity_pascal_name = heck::AsPascalCase(&entity.name);

            {
                // for src/direct_access/{}

                let relative_path = format!(
                    "{}/direct_access/{}/",
                    prefix,
                    heck::AsSnakeCase(&entity.name)
                );

                b.add(
                    "CMakeLists.txt",
                    relative_path.clone(),
                    format!("direct_access: {}", entity_pascal_name),
                    "direct_access_entity_cmake",
                    FileNature::Aggregate,
                )
                .entity = Some(entity.id);
                b.add(
                    format!("{}_controller.cpp", entity_snake_name),
                    relative_path.clone(),
                    format!("direct_access: {}", entity_pascal_name),
                    "entity_controller_cpp",
                    FileNature::Infrastructure,
                )
                .entity = Some(entity.id);
                b.add(
                    format!("{}_controller.h", entity_snake_name),
                    relative_path.clone(),
                    format!("direct_access: {}", entity_pascal_name),
                    "entity_controller_h",
                    FileNature::Infrastructure,
                )
                .entity = Some(entity.id);
                b.add(
                    format!("{}_unit_of_work.h", entity_snake_name),
                    relative_path.clone(),
                    format!("direct_access: {}", entity_pascal_name),
                    "entity_unit_of_work_h",
                    FileNature::Infrastructure,
                )
                .entity = Some(entity.id);
                b.add(
                    "dtos.h",
                    relative_path.clone(),
                    format!("direct_access: {}", entity_pascal_name),
                    "dtos_h",
                    FileNature::Infrastructure,
                )
                .entity = Some(entity.id);
                b.add(
                    format!("i_{}_unit_of_work.h", entity_snake_name),
                    relative_path.clone(),
                    format!("direct_access: {}", entity_pascal_name),
                    "i_entity_unit_of_work_h",
                    FileNature::Infrastructure,
                )
                .entity = Some(entity.id);
                b.add(
                    "dto_mapper.h",
                    relative_path.clone(),
                    "direct_access",
                    "dto_mapper_h",
                    FileNature::Infrastructure,
                )
                .entity = Some(entity.id);

                let relative_path = format!(
                    "{}/direct_access/{}/models/",
                    prefix,
                    heck::AsSnakeCase(&entity.name)
                );

                // list models, we must find the Field that has the "list_model" field true.

                let fields = uow.get_entity_relationship(
                    &entity.id,
                    &common::direct_access::entity::EntityRelationshipField::Fields,
                )?;
                let fields = uow.get_field_multi(&fields)?;
                let list_model_fields = fields
                    .into_iter()
                    .flatten()
                    .filter(|f| f.list_model)
                    .collect::<Vec<_>>();

                for list_model_field in list_model_fields {
                    let field_snake_name = heck::AsSnakeCase(&list_model_field.name);

                    let f = b.add(
                        format!("{}_{}_list_model.cpp", entity_snake_name, field_snake_name),
                        relative_path.clone(),
                        format!("direct_access: {}", entity_pascal_name),
                        "entity_field_list_model_cpp",
                        FileNature::Infrastructure,
                    );
                    f.entity = Some(entity.id);
                    f.field = Some(list_model_field.id);

                    let f = b.add(
                        format!("{}_{}_list_model.h", entity_snake_name, field_snake_name),
                        relative_path.clone(),
                        format!("direct_access: {}", entity_pascal_name),
                        "entity_field_list_model_h",
                        FileNature::Infrastructure,
                    );
                    f.entity = Some(entity.id);
                    f.field = Some(list_model_field.id);
                }

                // single model

                if entity.single_model {
                    b.add(
                        format!("single_{}.h", entity_snake_name),
                        relative_path.clone(),
                        format!("direct_access: {}", entity_pascal_name),
                        "single_entity_h",
                        FileNature::Infrastructure,
                    )
                    .entity = Some(entity.id);
                    b.add(
                        format!("single_{}.cpp", entity_snake_name),
                        relative_path.clone(),
                        format!("direct_access: {}", entity_pascal_name),
                        "single_entity_cpp",
                        FileNature::Infrastructure,
                    )
                    .entity = Some(entity.id);
                }
            }

            // for common/entities/
            let relative_path = format!("{}/common/entities/", prefix);

            b.add(
                format!("{}.h", entity_snake_name),
                relative_path.to_string(),
                format!("entities: {}", entity_pascal_name),
                "entity_h",
                FileNature::Infrastructure,
            )
            .entity = Some(entity.id);

            // for common/direct_access/{entity_snake_name}/
            let relative_path = format!("{}/common/direct_access/", prefix);

            b.add(
                "CMakeLists.txt",
                format!("{}{}/", relative_path, entity_snake_name),
                format!("entities: {}", entity_pascal_name),
                "common_direct_access_entity_cmake",
                FileNature::Aggregate,
            )
            .entity = Some(entity.id);
            b.add(
                format!("i_{}_repository.h", entity_snake_name),
                format!("{}{}/", relative_path, entity_snake_name),
                format!("entities: {}", entity_pascal_name),
                "i_entity_repository_h",
                FileNature::Infrastructure,
            )
            .entity = Some(entity.id);
            b.add(
                "table_definitions.h",
                format!("{}{}/", relative_path, entity_snake_name),
                format!("entities: {}", entity_pascal_name),
                "table_definitions_h",
                FileNature::Infrastructure,
            )
            .entity = Some(entity.id);
            b.add(
                format!("{}_events.h", entity_snake_name),
                format!("{}{}/", relative_path, entity_snake_name),
                format!("entities: {}", entity_pascal_name),
                "entity_events_h",
                FileNature::Infrastructure,
            )
            .entity = Some(entity.id);
            b.add(
                format!("{}_repository.cpp", entity_snake_name),
                format!("{}{}/", relative_path, entity_snake_name),
                format!("entities: {}", entity_pascal_name),
                "entity_repository_cpp",
                FileNature::Infrastructure,
            )
            .entity = Some(entity.id);
            b.add(
                format!("{}_repository.h", entity_snake_name),
                format!("{}{}/", relative_path, entity_snake_name),
                format!("entities: {}", entity_pascal_name),
                "entity_repository_h",
                FileNature::Infrastructure,
            )
            .entity = Some(entity.id);
            b.add(
                format!("{}_table.cpp", entity_snake_name),
                format!("{}{}/", relative_path, entity_snake_name),
                format!("entities: {}", entity_pascal_name),
                "entity_table_cpp",
                FileNature::Infrastructure,
            )
            .entity = Some(entity.id);
            b.add(
                format!("{}_table.h", entity_snake_name),
                format!("{}{}/", relative_path, entity_snake_name),
                format!("entities: {}", entity_pascal_name),
                "entity_table_h",
                FileNature::Infrastructure,
            )
            .entity = Some(entity.id);
        }

        // common features:

        let relative_path = format!("{}/common/features/", prefix);

        b.add(
            "CMakeLists.txt",
            relative_path.clone(),
            "features",
            "common_features_cmake",
            FileNature::Aggregate,
        )
        .all_features = true;
        b.add(
            "feature_event_registry.h",
            relative_path.clone(),
            "features",
            "feature_event_registry_h",
            FileNature::Aggregate,
        )
        .all_features = true;

        //----------------------------------------------------------------------
        // Features
        //----------------------------------------------------------------------

        let features =
            uow.get_workspace_relationship(&workspace_id, &WorkspaceRelationshipField::Features)?;

        let features = uow.get_feature_multi(&features)?;

        for feature in &features {
            let feature = feature.as_ref().ok_or(anyhow!("Feature not found"))?;

            let relative_path = format!("{}/common/features/", prefix);

            let feature_snake_name = heck::AsSnakeCase(&feature.name);
            let feature_pascal_name = heck::AsPascalCase(&feature.name);

            b.add(
                format!("{}_events.h", feature_snake_name),
                relative_path.clone(),
                format!("feature: {}", feature_pascal_name),
                "feature_events_h",
                FileNature::Infrastructure,
            )
            .feature = Some(feature.id);

            let relative_path = format!("{}/{}/", prefix, feature_snake_name);

            b.add(
                "CMakeLists.txt",
                relative_path.clone(),
                format!("feature: {}", feature_pascal_name),
                "feature_cmake",
                FileNature::Aggregate,
            )
            .feature = Some(feature.id);
            b.add(
                format!("{}_dtos.h", feature_snake_name),
                relative_path.clone(),
                format!("feature: {}", feature_pascal_name),
                "feature_dtos_h",
                FileNature::Infrastructure,
            )
            .feature = Some(feature.id);
            b.add(
                format!("{}_controller.h", feature_snake_name),
                relative_path.clone(),
                format!("feature: {}", feature_pascal_name),
                "feature_controller_h",
                FileNature::Infrastructure,
            )
            .feature = Some(feature.id);
            b.add(
                format!("{}_controller.cpp", feature_snake_name),
                relative_path.clone(),
                format!("feature: {}", feature_pascal_name),
                "feature_controller_cpp",
                FileNature::Infrastructure,
            )
            .feature = Some(feature.id);

            let use_cases =
                uow.get_feature_relationship(&feature.id, &FeatureRelationshipField::UseCases)?;
            let use_cases = uow.get_use_case_multi(&use_cases)?;

            for use_case in use_cases {
                let use_case = use_case.ok_or(anyhow!("Use case not found"))?;

                let use_case_snake_name = heck::AsSnakeCase(&use_case.name);
                let relative_path = format!("{}/{}/units_of_work/", prefix, feature_snake_name);

                let f = b.add(
                    format!("{}_uow.h", use_case_snake_name),
                    relative_path.clone(),
                    format!("feature: {}", feature_pascal_name),
                    "feature_uow_h",
                    FileNature::Scaffold,
                );
                f.feature = Some(feature.id);
                f.use_case = Some(use_case.id);

                let relative_path = format!(
                    "{}/{}/use_cases/{}_uc/",
                    prefix, feature_snake_name, use_case_snake_name
                );

                let f = b.add(
                    format!("i_{}_uow.h", use_case_snake_name),
                    relative_path.clone(),
                    format!("feature: {}", feature_pascal_name),
                    "i_feature_uow_h",
                    FileNature::Scaffold,
                );
                f.feature = Some(feature.id);
                f.use_case = Some(use_case.id);

                let relative_path = format!("{}/{}/use_cases/", prefix, feature_snake_name);

                let f = b.add(
                    format!("{}_uc.h", use_case_snake_name),
                    relative_path.clone(),
                    format!("feature: {}", feature_pascal_name),
                    "feature_uc_h",
                    FileNature::Scaffold,
                );
                f.feature = Some(feature.id);
                f.use_case = Some(use_case.id);

                let f = b.add(
                    format!("{}_uc.cpp", use_case_snake_name),
                    relative_path.clone(),
                    format!("feature: {}", feature_pascal_name),
                    "feature_uc_cpp",
                    FileNature::Scaffold,
                );
                f.feature = Some(feature.id);
                f.use_case = Some(use_case.id);
            }
        }

        //----------------------------------------------------------------------
        // Long Operation support
        //----------------------------------------------------------------------

        b.add(
            "i_long_operation.h",
            format!("{}/common/long_operation/", prefix),
            "base",
            "i_long_operation_h",
            FileNature::Infrastructure,
        );
        b.add(
            "long_operation_manager.h",
            format!("{}/common/long_operation/", prefix),
            "base",
            "long_operation_manager_h",
            FileNature::Infrastructure,
        );
        b.add(
            "long_operation_manager.cpp",
            format!("{}/common/long_operation/", prefix),
            "base",
            "long_operation_manager_cpp",
            FileNature::Infrastructure,
        );

        //----------------------------------------------------------------------
        // QtWidgets GUI
        //----------------------------------------------------------------------

        if ui.cpp_qt_qtwidgets {
            // for common/entities/
            let relative_path = format!("{}/qtwidgets_app/", prefix);

            b.add(
                "CMakeLists.txt",
                relative_path.clone(),
                "QtWidgets UI",
                "qt_widgets_cmake",
                FileNature::Aggregate,
            )
            .all_features = true;
            b.add(
                "main.cpp",
                relative_path.clone(),
                "QtWidgets UI",
                "qt_widgets_main_cpp",
                FileNature::Scaffold,
            );
            b.add(
                "main_window.cpp",
                relative_path.clone(),
                "QtWidgets UI",
                "qt_widgets_main_window_cpp",
                FileNature::Scaffold,
            )
            .all_entities = true;
            b.add(
                "main_window.h",
                relative_path.clone(),
                "QtWidgets UI",
                "qt_widgets_main_window_h",
                FileNature::Scaffold,
            )
            .all_entities = true;
        }

        //----------------------------------------------------------------------
        // Presentation
        //----------------------------------------------------------------------

        // 3 first letters
        let application_short_name = global
            .application_name
            .chars()
            .take(3)
            .collect::<String>()
            .to_lowercase()
            .to_title_case();

        let relative_path = format!("{}/presentation/", prefix);

        let qml_enabled = ui.cpp_qt_qtquick;

        if qml_enabled {
            b.add(
                "CMakeLists.txt",
                relative_path.clone(),
                "QML Presentation",
                "foreign_presentation_cmake",
                FileNature::Aggregate,
            );

            // Real controllers

            let relative_path = format!("{}/presentation/real_imports/controllers/", prefix);

            let f = b.add(
                "CMakeLists.txt",
                relative_path.clone(),
                "QML Presentation",
                "foreign_controllers_cmake",
                FileNature::Aggregate,
            );
            f.all_features = true;
            f.all_entities = true;

            b.add(
                "foreign_event_registry.h",
                relative_path.clone(),
                "QML Presentation",
                "foreign_event_registry_h",
                FileNature::Aggregate,
            )
            .all_entities = true;
            b.add(
                "foreign_feature_event_registry.h",
                relative_path.clone(),
                "QML Presentation",
                "foreign_feature_event_registry_h",
                FileNature::Aggregate,
            )
            .all_features = true;
            b.add(
                "foreign_undo_redo_controller.h",
                relative_path.clone(),
                "QML Presentation",
                "foreign_undo_redo_controller_h",
                FileNature::Infrastructure,
            );
            b.add(
                "foreign_service_locator.h",
                relative_path.clone(),
                "QML Presentation",
                "foreign_service_locator_h",
                FileNature::Infrastructure,
            );

            // mock controllers

            let relative_path = format!(
                "{}/presentation/mock_imports/{}/Controllers/",
                prefix, application_short_name
            );

            b.add(
                "EventRegistry.qml",
                relative_path.clone(),
                "QML Presentation",
                "event_registry_qml",
                FileNature::Infrastructure,
            )
            .all_entities = true;
            b.add(
                "QCoroQmlTask.qml",
                relative_path.clone(),
                "QML Presentation",
                "qcoro_qml_task_qml",
                FileNature::Infrastructure,
            );
            b.add(
                "UndoRedoController.qml",
                relative_path.clone(),
                "QML Presentation",
                "undo_redo_controller_qml",
                FileNature::Infrastructure,
            );
            b.add(
                "ServiceLocator.qml",
                relative_path.clone(),
                "QML Presentation",
                "service_locator_qml",
                FileNature::Infrastructure,
            );
            b.add(
                "FeatureEventRegistry.qml",
                relative_path.clone(),
                "QML Presentation",
                "feature_event_registry_qml",
                FileNature::Infrastructure,
            )
            .all_features = true;

            let f = b.add(
                "qmldir",
                relative_path.clone(),
                "QML Presentation",
                "mock_controllers_qmldir",
                FileNature::Aggregate,
            );
            f.all_features = true;
            f.all_entities = true;

            // Real models

            let relative_path = format!("{}/presentation/real_imports/models/", prefix);

            b.add(
                "CMakeLists.txt",
                relative_path.clone(),
                "QML Presentation",
                "foreign_models_cmake",
                FileNature::Aggregate,
            )
            .all_entities = true;

            // mock models

            let relative_path = format!(
                "{}/presentation/mock_imports/{}/Models/",
                prefix, application_short_name
            );

            b.add(
                "qmldir",
                relative_path.clone(),
                "QML Presentation",
                "mock_models_qmldir",
                FileNature::Aggregate,
            )
            .all_entities = true;

            // Real singles

            let relative_path = format!("{}/presentation/real_imports/singles/", prefix);

            b.add(
                "CMakeLists.txt",
                relative_path.clone(),
                "QML Presentation",
                "foreign_singles_cmake",
                FileNature::Aggregate,
            )
            .all_entities = true;

            // mock singles

            let relative_path = format!(
                "{}/presentation/mock_imports/{}/Singles/",
                prefix, application_short_name
            );

            b.add(
                "qmldir",
                relative_path.clone(),
                "QML Presentation",
                "mock_singles_qmldir",
                FileNature::Aggregate,
            )
            .all_entities = true;

            // Get entities
            let entities = uow
                .get_workspace_relationship(&workspace_id, &WorkspaceRelationshipField::Entities)?;
            let entities = uow.get_entity_multi(&entities)?;

            for entity in &entities {
                let entity = entity.as_ref().ok_or(anyhow!("Entity not found"))?;

                // skip if entity is "heritage"
                if entity.only_for_heritage {
                    continue;
                }

                let entity_snake_name = heck::AsSnakeCase(&entity.name);
                let entity_pascal_name = heck::AsPascalCase(&entity.name);

                // Real controllers

                let relative_path = format!("{}/presentation/real_imports/controllers/", prefix);

                b.add(
                    format!("foreign_{}_controller.h", entity_snake_name),
                    relative_path.clone(),
                    "QML Presentation",
                    "foreign_entity_controller_h",
                    FileNature::Infrastructure,
                )
                .entity = Some(entity.id);

                // mock controllers

                let relative_path = format!(
                    "{}/presentation/mock_imports/{}/Controllers/",
                    prefix, application_short_name
                );

                b.add(
                    format!("{}Controller.qml", entity_pascal_name),
                    relative_path.clone(),
                    "QML Presentation",
                    "entity_controller_qml",
                    FileNature::Infrastructure,
                )
                .entity = Some(entity.id);
                b.add(
                    format!("{}Events.qml", entity_pascal_name),
                    relative_path.clone(),
                    "QML Presentation",
                    "entity_events_qml",
                    FileNature::Infrastructure,
                )
                .entity = Some(entity.id);

                let fields = uow.get_entity_relationship(
                    &entity.id,
                    &common::direct_access::entity::EntityRelationshipField::Fields,
                )?;
                let fields = uow.get_field_multi(&fields)?;
                let list_model_fields = fields
                    .into_iter()
                    .flatten()
                    .filter(|f| f.list_model)
                    .collect::<Vec<_>>();

                for list_model_field in list_model_fields {
                    let field_snake_name = heck::AsSnakeCase(&list_model_field.name);
                    let field_pascal_name = heck::AsPascalCase(&list_model_field.name);

                    // real models

                    let relative_path = format!("{}/presentation/real_imports/models/", prefix);

                    let f = b.add(
                        format!(
                            "foreign_{}_{}_list_model.h",
                            entity_snake_name, field_snake_name
                        ),
                        relative_path.clone(),
                        "QML Presentation",
                        "foreign_list_model_h",
                        FileNature::Infrastructure,
                    );
                    f.entity = Some(entity.id);
                    f.field = Some(list_model_field.id);

                    // mock models

                    let relative_path = format!(
                        "{}/presentation/mock_imports/{}/Models/",
                        prefix, application_short_name
                    );

                    let f = b.add(
                        format!("{}{}ListModel.qml", entity_pascal_name, field_pascal_name),
                        relative_path.clone(),
                        "QML Presentation",
                        "list_model_qml",
                        FileNature::Infrastructure,
                    );
                    f.entity = Some(entity.id);
                    f.field = Some(list_model_field.id);
                }

                if entity.single_model {
                    // real singles
                    let relative_path = format!("{}/presentation/real_imports/singles/", prefix);

                    b.add(
                        format!("foreign_single_{}.h", entity_snake_name),
                        relative_path.clone(),
                        "QML Presentation",
                        "foreign_single_h",
                        FileNature::Infrastructure,
                    )
                    .entity = Some(entity.id);

                    // mock singles

                    let relative_path = format!(
                        "{}/presentation/mock_imports/{}/Singles/",
                        prefix, application_short_name
                    );

                    b.add(
                        format!("Single{}.qml", entity_pascal_name),
                        relative_path.clone(),
                        "QML Presentation",
                        "single_entity_qml",
                        FileNature::Infrastructure,
                    )
                    .entity = Some(entity.id);
                }
            }

            let features = uow
                .get_workspace_relationship(&workspace_id, &WorkspaceRelationshipField::Features)?;

            let features = uow.get_feature_multi(&features)?;

            for feature in &features {
                let feature = feature.as_ref().ok_or(anyhow!("Feature not found"))?;

                let feature_snake_name = heck::AsSnakeCase(&feature.name);
                let feature_pascal_name = heck::AsPascalCase(&feature.name);

                let relative_path = format!("{}/presentation/real_imports/controllers/", prefix);

                b.add(
                    format!("foreign_{}_controller.h", feature_snake_name),
                    relative_path.clone(),
                    "QML Presentation",
                    "foreign_feature_controller_h",
                    FileNature::Infrastructure,
                )
                .feature = Some(feature.id);

                // mock controllers feature

                let relative_path = format!(
                    "{}/presentation/mock_imports/{}/Controllers/",
                    prefix, application_short_name
                );

                b.add(
                    format!("{}Controller.qml", feature_pascal_name),
                    relative_path.clone(),
                    "QML Presentation",
                    "feature_controller_qml",
                    FileNature::Infrastructure,
                )
                .feature = Some(feature.id);
                b.add(
                    format!("{}Events.qml", feature_pascal_name),
                    relative_path.clone(),
                    "QML Presentation",
                    "feature_events_qml",
                    FileNature::Infrastructure,
                )
                .feature = Some(feature.id);
            }
        }

        //----------------------------------------------------------------------
        // QtQuick GUI
        //----------------------------------------------------------------------

        if ui.cpp_qt_qtquick {
            let relative_path = format!("{}/qtquick_app/", prefix);

            b.add(
                "CMakeLists.txt",
                relative_path.clone(),
                "QtQuick UI",
                "qt_quick_cmake",
                FileNature::Aggregate,
            );
            b.add(
                "main.qml",
                relative_path.clone(),
                "QtQuick UI",
                "qt_quick_main_qml",
                FileNature::Scaffold,
            );
            b.add(
                "main.cpp",
                relative_path.clone(),
                "QtQuick UI",
                "qt_quick_main_cpp",
                FileNature::Scaffold,
            );
            b.add(
                "qtquickcontrols2.conf",
                relative_path.clone(),
                "QtQuick UI",
                "qt_quick_qtquickcontrols2_conf",
                FileNature::Scaffold,
            );

            let relative_path = format!("{}/qtquick_app/content/", prefix);

            b.add(
                "CMakeLists.txt",
                relative_path.clone(),
                "QtQuick UI",
                "qt_quick_content_cmake",
                FileNature::Aggregate,
            );
            b.add(
                "App.qml",
                relative_path.clone(),
                "QtQuick UI",
                "qt_quick_app_qml",
                FileNature::Scaffold,
            )
            .all_entities = true;

            let relative_path = format!("{}/qtquick_app/{}/", prefix, application_short_name);

            b.add(
                "CMakeLists.txt",
                relative_path.clone(),
                "QtQuick UI",
                "qt_quick_app_cmake",
                FileNature::Aggregate,
            );
        }

        //----------------------------------------------------------------------
        // Tests
        //----------------------------------------------------------------------

        b.add(
            "CMakeLists.txt",
            "tests/",
            "tests",
            "tests_cmake",
            FileNature::Aggregate,
        );
        b.add(
            "CMakeLists.txt",
            "tests/database/",
            "tests",
            "tests_database_cmake",
            FileNature::Aggregate,
        );
        b.add(
            "tst_many_to_one_junction.cpp",
            "tests/database/",
            "tests",
            "tst_many_to_one_junction_cpp",
            FileNature::Infrastructure,
        );
        b.add(
            "tst_one_to_one_junction.cpp",
            "tests/database/",
            "tests",
            "tst_one_to_one_junction_cpp",
            FileNature::Infrastructure,
        );
        b.add(
            "tst_ordered_one_to_many_junction.cpp",
            "tests/database/",
            "tests",
            "tst_ordered_one_to_many_junction_cpp",
            FileNature::Infrastructure,
        );
        b.add(
            "tst_unordered_many_to_many_junction.cpp",
            "tests/database/",
            "tests",
            "tst_unordered_many_to_many_junction_cpp",
            FileNature::Infrastructure,
        );
        b.add(
            "tst_unordered_one_to_many_junction.cpp",
            "tests/database/",
            "tests",
            "tst_unordered_one_to_many_junction_cpp",
            FileNature::Infrastructure,
        );
        b.add(
            "CMakeLists.txt",
            "tests/undo_redo/",
            "tests",
            "tests_undo_redo_cmake",
            FileNature::Aggregate,
        );
        b.add(
            "tst_enhanced_undo_redo.cpp",
            "tests/undo_redo/",
            "tests",
            "tst_enhanced_undo_redo_cpp",
            FileNature::Infrastructure,
        );
        b.add(
            "tst_qcoro_integration.cpp",
            "tests/undo_redo/",
            "tests",
            "tst_qcoro_integration_cpp",
            FileNature::Infrastructure,
        );
        b.add(
            "tst_root_undo_redo.cpp",
            "tests/undo_redo/",
            "tests",
            "tst_root_undo_redo_cpp",
            FileNature::Infrastructure,
        )
        .all_entities = true;
        b.add(
            "tst_undo_redo.cpp",
            "tests/undo_redo/",
            "tests",
            "tst_undo_redo_cpp",
            FileNature::Infrastructure,
        );

        let files = if dto.only_list_already_existing {
            b.build_filtered(|file| {
                let full_path = format!("{}{}", file.relative_path, file.name);
                std::path::Path::new(&full_path).exists()
            })
        } else {
            b.build()
        };

        // create files in db
        let created_files = uow.create_orphan_file_multi(&files)?;
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

        Ok(FillCppQtFilesReturnDto {
            file_ids,
            file_names,
            file_groups,
        })
    }
}
