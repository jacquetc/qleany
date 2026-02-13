use crate::use_cases::common::tools;
use crate::{ListCppQtFilesDto, ListCppQtFilesReturnDto};
use anyhow::Result;
use common::direct_access::feature::FeatureRelationshipField;
use common::direct_access::system::SystemRelationshipField;
use common::direct_access::workspace::WorkspaceRelationshipField;
use common::entities::Entity;
use common::entities::UserInterface;
use common::types::EntityId;
use common::{
    database::CommandUnitOfWork, entities::Feature, entities::Field, entities::File,
    entities::Global, entities::Relationship, entities::Root, entities::UseCase,
};

pub trait ListCppQtFilesUnitOfWorkFactoryTrait {
    fn create(&self) -> Box<dyn ListCppQtFilesUnitOfWorkTrait>;
}

#[macros::uow_action(entity = "Root", action = "GetMulti")]
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
#[macros::uow_action(entity = "File", action = "Create")]
#[macros::uow_action(entity = "File", action = "CreateMulti")]
#[macros::uow_action(entity = "File", action = "DeleteMulti")]
pub trait ListCppQtFilesUnitOfWorkTrait: CommandUnitOfWork {}

pub struct ListCppQtFilesUseCase {
    uow_factory: Box<dyn ListCppQtFilesUnitOfWorkFactoryTrait>,
}

impl ListCppQtFilesUseCase {
    pub fn new(uow_factory: Box<dyn ListCppQtFilesUnitOfWorkFactoryTrait>) -> Self {
        ListCppQtFilesUseCase { uow_factory }
    }

    pub fn execute(&mut self, dto: &ListCppQtFilesDto) -> Result<ListCppQtFilesReturnDto> {
        let mut files: Vec<File> = vec![];

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        use anyhow::anyhow;
        let roots = uow.get_root_multi(&[])?;
        let root = roots
            .into_iter()
            .flatten()
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
            uow.delete_file_multi(&all_previous_files)?;
        }

        files.push(File {
            id: 0,
            name: "CMakeLists.txt".to_string(),
            relative_path: "".to_string(),
            group: "base".to_string(),
            template_name: "root_cmake".to_string(),
            feature: Some(0),
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "CMakeLists.txt".to_string(),
            relative_path: format!("{}/common/", prefix),
            group: "base".to_string(),
            template_name: "common_cmake".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "InstallHelpers.cmake".to_string(),
            relative_path: "cmake/".to_string(),
            group: "base".to_string(),
            template_name: "install_helpers_cmake".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "VersionFromGit.cmake".to_string(),
            relative_path: "cmake/".to_string(),
            group: "base".to_string(),
            template_name: "version_from_git_cmake".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "service_locator.h".to_string(),
            relative_path: format!("{}/common/", prefix),
            group: "base".to_string(),
            template_name: "service_locator_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "service_locator.cpp".to_string(),
            relative_path: format!("{}/common/", prefix),
            group: "base".to_string(),
            template_name: "service_locator_cpp".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "controller_command_helpers.h".to_string(),
            relative_path: format!("{}/common/", prefix),
            group: "base".to_string(),
            template_name: "controller_command_helpers_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        // database

        files.push(File {
            id: 0,
            name: "db_builder.h".to_string(),
            relative_path: format!("{}/common/database", prefix),
            group: "common_db".to_string(),
            template_name: "db_builder_h".to_string(),
            feature: None,
            entity: Some(0),
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "db_context.h".to_string(),
            relative_path: format!("{}/common/database", prefix),
            group: "common_db".to_string(),
            template_name: "db_context_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "table_cache.h".to_string(),
            relative_path: format!("{}/common/database", prefix),
            group: "common_db".to_string(),
            template_name: "table_cache_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "junction_cache.h".to_string(),
            relative_path: format!("{}/common/database/junction_table_ops/", prefix),
            group: "common_db".to_string(),
            template_name: "junction_cache_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "unordered_many_to_many.h".to_string(),
            relative_path: format!("{}/common/database/junction_table_ops/", prefix),
            group: "common_db".to_string(),
            template_name: "unordered_many_to_many_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "unordered_many_to_many.cpp".to_string(),
            relative_path: format!("{}/common/database/junction_table_ops/", prefix),
            group: "common_db".to_string(),
            template_name: "unordered_many_to_many_cpp".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "unordered_one_to_many.h".to_string(),
            relative_path: format!("{}/common/database/junction_table_ops/", prefix),
            group: "common_db".to_string(),
            template_name: "unordered_one_to_many_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "unordered_one_to_many.cpp".to_string(),
            relative_path: format!("{}/common/database/junction_table_ops/", prefix),
            group: "common_db".to_string(),
            template_name: "unordered_one_to_many_cpp".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "ordered_one_to_many.h".to_string(),
            relative_path: format!("{}/common/database/junction_table_ops/", prefix),
            group: "common_db".to_string(),
            template_name: "ordered_one_to_many_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "ordered_one_to_many.cpp".to_string(),
            relative_path: format!("{}/common/database/junction_table_ops/", prefix),
            group: "common_db".to_string(),
            template_name: "ordered_one_to_many_cpp".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "one_to_one.h".to_string(),
            relative_path: format!("{}/common/database/junction_table_ops/", prefix),
            group: "common_db".to_string(),
            template_name: "one_to_one_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "one_to_one.cpp".to_string(),
            relative_path: format!("{}/common/database/junction_table_ops/", prefix),
            group: "common_db".to_string(),
            template_name: "one_to_one_cpp".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });
        files.push(File {
            id: 0,
            name: "many_to_one.h".to_string(),
            relative_path: format!("{}/common/database/junction_table_ops/", prefix),
            group: "common_db".to_string(),
            template_name: "many_to_one_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "many_to_one.cpp".to_string(),
            relative_path: format!("{}/common/database/junction_table_ops/", prefix),
            group: "common_db".to_string(),
            template_name: "many_to_one_cpp".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        // undo redo

        files.push(File {
            id: 0,
            name: "group_command.cpp".to_string(),
            relative_path: format!("{}/common/undo_redo/", prefix),
            group: "common_undo".to_string(),
            template_name: "group_command_cpp".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "group_command.h".to_string(),
            relative_path: format!("{}/common/undo_redo/", prefix),
            group: "common_undo".to_string(),
            template_name: "group_command_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "group_command_builder.cpp".to_string(),
            relative_path: format!("{}/common/undo_redo/", prefix),
            group: "common_undo".to_string(),
            template_name: "group_command_builder_cpp".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "group_command_builder.h".to_string(),
            relative_path: format!("{}/common/undo_redo/", prefix),
            group: "common_undo".to_string(),
            template_name: "group_command_builder_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "query_handler.cpp".to_string(),
            relative_path: format!("{}/common/undo_redo/", prefix),
            group: "common_undo".to_string(),
            template_name: "query_handler_cpp".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "query_handler.h".to_string(),
            relative_path: format!("{}/common/undo_redo/", prefix),
            group: "common_undo".to_string(),
            template_name: "query_handler_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "undo_redo_command.cpp".to_string(),
            relative_path: format!("{}/common/undo_redo/", prefix),
            group: "common_undo".to_string(),
            template_name: "undo_redo_command_cpp".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "undo_redo_command.h".to_string(),
            relative_path: format!("{}/common/undo_redo/", prefix),
            group: "common_undo".to_string(),
            template_name: "undo_redo_command_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "undo_redo_manager.cpp".to_string(),
            relative_path: format!("{}/common/undo_redo/", prefix),
            group: "common_undo".to_string(),
            template_name: "undo_redo_manager_cpp".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "undo_redo_manager.h".to_string(),
            relative_path: format!("{}/common/undo_redo/", prefix),
            group: "common_undo".to_string(),
            template_name: "undo_redo_manager_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "undo_redo_stack.cpp".to_string(),
            relative_path: format!("{}/common/undo_redo/", prefix),
            group: "common_undo".to_string(),
            template_name: "undo_redo_stack_cpp".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "undo_redo_stack.h".to_string(),
            relative_path: format!("{}/common/undo_redo/", prefix),
            group: "common_undo".to_string(),
            template_name: "undo_redo_stack_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "undo_redo_system.cpp".to_string(),
            relative_path: format!("{}/common/undo_redo/", prefix),
            group: "common_undo".to_string(),
            template_name: "undo_redo_system_cpp".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "undo_redo_system.h".to_string(),
            relative_path: format!("{}/common/undo_redo/", prefix),
            group: "common_undo".to_string(),
            template_name: "undo_redo_system_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        // common unit of work macros
        files.push(File {
            id: 0,
            name: "uow_macros.h".to_string(),
            relative_path: format!("{}/common/unit_of_work/", prefix),
            group: "common_unit_of_work".to_string(),
            template_name: "uow_macros_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "unit_of_work.h".to_string(),
            relative_path: format!("{}/common/unit_of_work/", prefix),
            group: "common_unit_of_work".to_string(),
            template_name: "unit_of_work_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "uow_base.h".to_string(),
            relative_path: format!("{}/common/unit_of_work/", prefix),
            group: "common_unit_of_work".to_string(),
            template_name: "uow_base_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "uow_ops.h".to_string(),
            relative_path: format!("{}/common/unit_of_work/", prefix),
            group: "common_unit_of_work".to_string(),
            template_name: "uow_ops_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        // direct access

        files.push(File {
            id: 0,
            name: "event_registry.h".to_string(),
            relative_path: format!("{}/common/direct_access/", prefix),
            group: "common_direct_access".to_string(),
            template_name: "event_registry_h".to_string(),
            feature: None,
            entity: Some(0),
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "repository_factory.cpp".to_string(),
            relative_path: format!("{}/common/direct_access/", prefix),
            group: "common_direct_access".to_string(),
            template_name: "repository_factory_cpp".to_string(),
            feature: None,
            entity: Some(0),
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "repository_factory.h".to_string(),
            relative_path: format!("{}/common/direct_access/", prefix),
            group: "common_direct_access".to_string(),
            template_name: "repository_factory_h".to_string(),
            feature: None,
            entity: Some(0),
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "mapper_tools.h".to_string(),
            relative_path: format!("{}/common/direct_access/", prefix),
            group: "common_direct_access".to_string(),
            template_name: "mapper_tools_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "converter_registration.h".to_string(),
            relative_path: format!("{}/common/direct_access/", prefix),
            group: "common_direct_access".to_string(),
            template_name: "converter_registration_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "operators.h".to_string(),
            relative_path: format!("{}/common/direct_access/", prefix),
            group: "common_direct_access".to_string(),
            template_name: "operators_h".to_string(),
            feature: None,
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "CMakeLists.txt".to_string(),
            relative_path: format!("{}/common/direct_access/", prefix),
            group: "common_direct_access".to_string(),
            template_name: "common_direct_access_cmake".to_string(),
            feature: None,
            entity: Some(0),
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "CMakeLists.txt".to_string(),
            relative_path: format!("{}/common/entities/", prefix),
            group: "common_direct_access".to_string(),
            template_name: "common_entities_cmake".to_string(),
            feature: None,
            entity: Some(0),
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "CMakeLists.txt".to_string(),
            relative_path: format!("{}/direct_access/", prefix),
            group: "direct_access".to_string(),
            template_name: "direct_access_cmake".to_string(),
            feature: None,
            entity: Some(0),
            use_case: None,
            field: None,
        });

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

            if entity.allow_direct_access {
                // for src/direct_access/{}

                let relative_path = format!(
                    "{}/direct_access/{}/",
                    prefix,
                    heck::AsSnakeCase(&entity.name)
                );

                files.push(File {
                    id: 0,
                    name: "CMakeLists.txt".to_string(),
                    relative_path: relative_path.clone(),
                    group: format!("direct_access: {}", entity_pascal_name),
                    template_name: "direct_access_entity_cmake".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                    field: None,
                });

                files.push(File {
                    id: 0,
                    name: format!("{}_controller.cpp", entity_snake_name),
                    relative_path: relative_path.clone(),
                    group: format!("direct_access: {}", entity_pascal_name),
                    template_name: "entity_controller_cpp".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                    field: None,
                });

                files.push(File {
                    id: 0,
                    name: format!("{}_controller.h", entity_snake_name),
                    relative_path: relative_path.clone(),
                    group: format!("direct_access: {}", entity_pascal_name),
                    template_name: "entity_controller_h".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                    field: None,
                });

                files.push(File {
                    id: 0,
                    name: format!("{}_unit_of_work.h", entity_snake_name),
                    relative_path: relative_path.clone(),
                    group: format!("direct_access: {}", entity_pascal_name),
                    template_name: "entity_unit_of_work_h".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                    field: None,
                });

                files.push(File {
                    id: 0,
                    name: "dtos.h".to_string(),
                    relative_path: relative_path.clone(),
                    group: format!("direct_access: {}", entity_pascal_name),
                    template_name: "dtos_h".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                    field: None,
                });

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

                    files.push(File {
                        id: 0,
                        name: format!("{}_{}_list_model.cpp", entity_snake_name, field_snake_name),
                        relative_path: relative_path.clone(),
                        group: format!("direct_access: {}", entity_pascal_name),
                        template_name: "entity_field_list_model_cpp".to_string(),
                        feature: None,
                        entity: Some(entity.id),
                        use_case: None,
                        field: Some(list_model_field.id),
                    });

                    files.push(File {
                        id: 0,
                        name: format!("{}_{}_list_model.h", entity_snake_name, field_snake_name),
                        relative_path: relative_path.clone(),
                        group: format!("direct_access: {}", entity_pascal_name),
                        template_name: "entity_field_list_model_h".to_string(),
                        feature: None,
                        entity: Some(entity.id),
                        use_case: None,
                        field: Some(list_model_field.id),
                    });
                }

                // single model

                if entity.single_model {
                    files.push(File {
                        id: 0,
                        name: format!("single_{}.h", entity_snake_name),
                        relative_path: relative_path.clone(),
                        group: format!("direct_access: {}", entity_pascal_name),
                        template_name: "single_entity_h".to_string(),
                        feature: None,
                        entity: Some(entity.id),
                        use_case: None,
                        field: None,
                    });

                    files.push(File {
                        id: 0,
                        name: format!("single_{}.cpp", entity_snake_name),
                        relative_path: relative_path.clone(),
                        group: format!("direct_access: {}", entity_pascal_name),
                        template_name: "single_entity_cpp".to_string(),
                        feature: None,
                        entity: Some(entity.id),
                        use_case: None,
                        field: None,
                    });
                }

                // for src/direct_access/{}/use_cases/

                let relative_path = format!(
                    "{}/direct_access/{}/use_cases/",
                    prefix,
                    heck::AsSnakeCase(&entity.name)
                );

                files.push(File {
                    id: 0,
                    name: format!("i_{}_unit_of_work.h", entity_snake_name),
                    relative_path: relative_path.clone(),
                    group: format!("direct_access: {}", entity_pascal_name),
                    template_name: "i_entity_unit_of_work_h".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                    field: None,
                });

                files.push(File {
                    id: 0,
                    name: "create_uc.cpp".to_string(),
                    relative_path: relative_path.clone(),
                    group: "direct_access".to_string(),
                    template_name: "create_uc_cpp".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                    field: None,
                });

                files.push(File {
                    id: 0,
                    name: "create_uc.h".to_string(),
                    relative_path: relative_path.clone(),
                    group: "direct_access".to_string(),
                    template_name: "create_uc_h".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                    field: None,
                });

                files.push(File {
                    id: 0,
                    name: "get_uc.cpp".to_string(),
                    relative_path: relative_path.clone(),
                    group: "direct_access".to_string(),
                    template_name: "get_uc_cpp".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                    field: None,
                });

                files.push(File {
                    id: 0,
                    name: "get_uc.h".to_string(),
                    relative_path: relative_path.clone(),
                    group: "direct_access".to_string(),
                    template_name: "get_uc_h".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                    field: None,
                });

                files.push(File {
                    id: 0,
                    name: "remove_uc.cpp".to_string(),
                    relative_path: relative_path.clone(),
                    group: "direct_access".to_string(),
                    template_name: "remove_uc_cpp".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                    field: None,
                });

                files.push(File {
                    id: 0,
                    name: "remove_uc.h".to_string(),
                    relative_path: relative_path.clone(),
                    group: "direct_access".to_string(),
                    template_name: "remove_uc_h".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                    field: None,
                });

                files.push(File {
                    id: 0,
                    name: "update_uc.cpp".to_string(),
                    relative_path: relative_path.clone(),
                    group: "direct_access".to_string(),
                    template_name: "update_uc_cpp".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                    field: None,
                });

                files.push(File {
                    id: 0,
                    name: "update_uc.h".to_string(),
                    relative_path: relative_path.clone(),
                    group: "direct_access".to_string(),
                    template_name: "update_uc_h".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                    field: None,
                });

                files.push(File {
                    id: 0,
                    name: "dto_mapper.h".to_string(),
                    relative_path: format!("{}{}", relative_path.clone(), "common/"),
                    group: "direct_access".to_string(),
                    template_name: "dto_mapper_h".to_string(),
                    feature: None,
                    entity: Some(entity.id),
                    use_case: None,
                    field: None,
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
                        name: "get_relationship_ids_count_uc.cpp".to_string(),
                        relative_path: relative_path.clone(),
                        group: "direct_access".to_string(),
                        template_name: "get_relationship_ids_count_uc_cpp".to_string(),
                        feature: None,
                        entity: Some(entity.id),
                        use_case: None,
                        field: None,
                    });

                    files.push(File {
                        id: 0,
                        name: "get_relationship_ids_count_uc.h".to_string(),
                        relative_path: relative_path.clone(),
                        group: "direct_access".to_string(),
                        template_name: "get_relationship_ids_count_uc_h".to_string(),
                        feature: None,
                        entity: Some(entity.id),
                        use_case: None,
                        field: None,
                    });

                    files.push(File {
                        id: 0,
                        name: "get_relationship_ids_in_range_uc.cpp".to_string(),
                        relative_path: relative_path.clone(),
                        group: "direct_access".to_string(),
                        template_name: "get_relationship_ids_in_range_uc_cpp".to_string(),
                        feature: None,
                        entity: Some(entity.id),
                        use_case: None,
                        field: None,
                    });

                    files.push(File {
                        id: 0,
                        name: "get_relationship_ids_in_range_uc.h".to_string(),
                        relative_path: relative_path.clone(),
                        group: "direct_access".to_string(),
                        template_name: "get_relationship_ids_in_range_uc_h".to_string(),
                        feature: None,
                        entity: Some(entity.id),
                        use_case: None,
                        field: None,
                    });

                    files.push(File {
                        id: 0,
                        name: "get_relationship_ids_many_uc.cpp".to_string(),
                        relative_path: relative_path.clone(),
                        group: "direct_access".to_string(),
                        template_name: "get_relationship_ids_many_uc_cpp".to_string(),
                        feature: None,
                        entity: Some(entity.id),
                        use_case: None,
                        field: None,
                    });

                    files.push(File {
                        id: 0,
                        name: "get_relationship_ids_many_uc.h".to_string(),
                        relative_path: relative_path.clone(),
                        group: "direct_access".to_string(),
                        template_name: "get_relationship_ids_many_uc_h".to_string(),
                        feature: None,
                        entity: Some(entity.id),
                        use_case: None,
                        field: None,
                    });

                    files.push(File {
                        id: 0,
                        name: "get_relationship_ids_uc.cpp".to_string(),
                        relative_path: relative_path.clone(),
                        group: "direct_access".to_string(),
                        template_name: "get_relationship_ids_uc_cpp".to_string(),
                        feature: None,
                        entity: Some(entity.id),
                        use_case: None,
                        field: None,
                    });

                    files.push(File {
                        id: 0,
                        name: "get_relationship_ids_uc.h".to_string(),
                        relative_path: relative_path.clone(),
                        group: "direct_access".to_string(),
                        template_name: "get_relationship_ids_uc_h".to_string(),
                        feature: None,
                        entity: Some(entity.id),
                        use_case: None,
                        field: None,
                    });

                    files.push(File {
                        id: 0,
                        name: "set_relationship_ids_uc.cpp".to_string(),
                        relative_path: relative_path.clone(),
                        group: "direct_access".to_string(),
                        template_name: "set_relationship_ids_uc_cpp".to_string(),
                        feature: None,
                        entity: Some(entity.id),
                        use_case: None,
                        field: None,
                    });

                    files.push(File {
                        id: 0,
                        name: "set_relationship_ids_uc.h".to_string(),
                        relative_path: relative_path.clone(),
                        group: "direct_access".to_string(),
                        template_name: "set_relationship_ids_uc_h".to_string(),
                        feature: None,
                        entity: Some(entity.id),
                        use_case: None,
                        field: None,
                    });
                } // has forward relationship
            } // allow direct access

            // for common/entities/
            let relative_path = format!("{}/common/entities/", prefix);

            files.push(File {
                id: 0,
                name: format!("{}.h", entity_snake_name),
                relative_path: relative_path.to_string(),
                group: format!("entities: {}", entity_pascal_name),
                template_name: "entity_h".to_string(),
                feature: None,
                entity: Some(entity.id),
                use_case: None,
                field: None,
            });

            // for common/direct_access/{entity_snake_name}/
            let relative_path = format!("{}/common/direct_access/", prefix);

            files.push(File {
                id: 0,
                name: "CMakeLists.txt".to_string(),
                relative_path: format!("{}{}/", relative_path, entity_snake_name),
                group: format!("entities: {}", entity_pascal_name),
                template_name: "common_direct_access_entity_cmake".to_string(),
                feature: None,
                entity: Some(entity.id),
                use_case: None,
                field: None,
            });
            files.push(File {
                id: 0,
                name: format!("i_{}_repository.h", entity_snake_name),
                relative_path: format!("{}{}/", relative_path, entity_snake_name),
                group: format!("entities: {}", entity_pascal_name),
                template_name: "i_entity_repository_h".to_string(),
                feature: None,
                entity: Some(entity.id),
                use_case: None,
                field: None,
            });

            files.push(File {
                id: 0,
                name: "table_definitions.h".to_string(),
                relative_path: format!("{}{}/", relative_path, entity_snake_name),
                group: format!("entities: {}", entity_pascal_name),
                template_name: "table_definitions_h".to_string(),
                feature: None,
                entity: Some(entity.id),
                use_case: None,
                field: None,
            });

            files.push(File {
                id: 0,
                name: format!("{}_events.h", entity_snake_name),
                relative_path: format!("{}{}/", relative_path, entity_snake_name),
                group: format!("entities: {}", entity_pascal_name),
                template_name: "entity_events_h".to_string(),
                feature: None,
                entity: Some(entity.id),
                use_case: None,
                field: None,
            });

            files.push(File {
                id: 0,
                name: format!("{}_repository.cpp", entity_snake_name),
                relative_path: format!("{}{}/", relative_path, entity_snake_name),
                group: format!("entities: {}", entity_pascal_name),
                template_name: "entity_repository_cpp".to_string(),
                feature: None,
                entity: Some(entity.id),
                use_case: None,
                field: None,
            });

            files.push(File {
                id: 0,
                name: format!("{}_repository.h", entity_snake_name),
                relative_path: format!("{}{}/", relative_path, entity_snake_name),
                group: format!("entities: {}", entity_pascal_name),
                template_name: "entity_repository_h".to_string(),
                feature: None,
                entity: Some(entity.id),
                use_case: None,
                field: None,
            });

            files.push(File {
                id: 0,
                name: format!("{}_table.cpp", entity_snake_name),
                relative_path: format!("{}{}/", relative_path, entity_snake_name),
                group: format!("entities: {}", entity_pascal_name),
                template_name: "entity_table_cpp".to_string(),
                feature: None,
                entity: Some(entity.id),
                use_case: None,
                field: None,
            });

            files.push(File {
                id: 0,
                name: format!("{}_table.h", entity_snake_name),
                relative_path: format!("{}{}/", relative_path, entity_snake_name),
                group: format!("entities: {}", entity_pascal_name),
                template_name: "entity_table_h".to_string(),
                feature: None,
                entity: Some(entity.id),
                use_case: None,
                field: None,
            });
        }

        // common features:

        let relative_path = format!("{}/common/features/", prefix);

        files.push(File {
            id: 0,
            name: "CMakeLists.txt".to_string(),
            relative_path: relative_path.clone(),
            group: "features".to_string(),
            template_name: "common_features_cmake".to_string(),
            feature: Some(0),
            entity: None,
            use_case: None,
            field: None,
        });

        files.push(File {
            id: 0,
            name: "feature_event_registry.h".to_string(),
            relative_path: relative_path.clone(),
            group: "features".to_string(),
            template_name: "feature_event_registry_h".to_string(),
            feature: Some(0),
            entity: None,
            use_case: None,
            field: None,
        });

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

            files.push(File {
                id: 0,
                name: format!("{}_events.h", feature_snake_name),
                relative_path: relative_path.clone(),
                group: format!("feature: {}", feature_pascal_name),
                template_name: "feature_events_h".to_string(),
                feature: Some(feature.id),
                entity: None,
                use_case: None,
                field: None,
            });

            let relative_path = format!("{}/{}/", prefix, feature_snake_name);

            files.push(File {
                id: 0,
                name: "CMakeLists.txt".to_string(),
                relative_path: relative_path.clone(),
                group: format!("feature: {}", feature_pascal_name),
                template_name: "feature_cmake".to_string(),
                feature: Some(feature.id),
                entity: None,
                use_case: None,
                field: None,
            });

            files.push(File {
                id: 0,
                name: format!("{}_dtos.h", feature_snake_name),
                relative_path: relative_path.clone(),
                group: format!("feature: {}", feature_pascal_name),
                template_name: "feature_dtos_h".to_string(),
                feature: Some(feature.id),
                entity: None,
                use_case: None,
                field: None,
            });

            files.push(File {
                id: 0,
                name: format!("{}_controller.h", feature_snake_name),
                relative_path: relative_path.clone(),
                group: format!("feature: {}", feature_pascal_name),
                template_name: "feature_controller_h".to_string(),
                feature: Some(feature.id),
                entity: None,
                use_case: None,
                field: None,
            });

            files.push(File {
                id: 0,
                name: format!("{}_controller.cpp", feature_snake_name),
                relative_path: relative_path.clone(),
                group: format!("feature: {}", feature_pascal_name),
                template_name: "feature_controller_cpp".to_string(),
                feature: Some(feature.id),
                entity: None,
                use_case: None,
                field: None,
            });

            let use_cases =
                uow.get_feature_relationship(&feature.id, &FeatureRelationshipField::UseCases)?;
            let use_cases = uow.get_use_case_multi(&use_cases)?;

            for use_case in use_cases {
                let use_case = use_case.ok_or(anyhow!("Use case not found"))?;

                let use_case_snake_name = heck::AsSnakeCase(&use_case.name);
                let relative_path = format!("{}/{}/units_of_work/", prefix, feature_snake_name);

                files.push(File {
                    id: 0,
                    name: format!("{}_uow.h", use_case_snake_name),
                    relative_path: relative_path.clone(),
                    group: format!("feature: {}", feature_pascal_name),
                    template_name: "feature_uow_h".to_string(),
                    feature: Some(feature.id),
                    entity: None,
                    use_case: Some(use_case.id),
                    field: None,
                });

                let relative_path = format!(
                    "{}/{}/use_cases/{}_uc/",
                    prefix, feature_snake_name, use_case_snake_name
                );

                files.push(File {
                    id: 0,
                    name: format!("i_{}_uow.h", use_case_snake_name),
                    relative_path: relative_path.clone(),
                    group: format!("feature: {}", feature_pascal_name),
                    template_name: "i_feature_uow_h".to_string(),
                    feature: Some(feature.id),
                    entity: None,
                    use_case: Some(use_case.id),
                    field: None,
                });

                let relative_path = format!("{}/{}/use_cases/", prefix, feature_snake_name);

                files.push(File {
                    id: 0,
                    name: format!("{}_uc.h", use_case_snake_name),
                    relative_path: relative_path.clone(),
                    group: format!("feature: {}", feature_pascal_name),
                    template_name: "feature_uc_h".to_string(),
                    feature: Some(feature.id),
                    entity: None,
                    use_case: Some(use_case.id),
                    field: None,
                });

                files.push(File {
                    id: 0,
                    name: format!("{}_uc.cpp", use_case_snake_name),
                    relative_path: relative_path.clone(),
                    group: format!("feature: {}", feature_pascal_name),
                    template_name: "feature_uc_cpp".to_string(),
                    feature: Some(feature.id),
                    entity: None,
                    use_case: Some(use_case.id),
                    field: None,
                });
            }
        }

        //----------------------------------------------------------------------
        // QtWidgets GUI
        //----------------------------------------------------------------------

        if ui.cpp_qt_qtwidgets {
            // for common/entities/
            let relative_path = format!("{}/qtwidgets_app/", prefix);

            files.push(File {
                id: 0,
                name: "CMakeLists.txt".to_string(),
                relative_path: relative_path.clone(),
                group: "QtWidgets UI".to_string(),
                template_name: "qt_widgets_cmake".to_string(),
                feature: Some(0),
                entity: None,
                use_case: None,
                field: None,
            });

            files.push(File {
                id: 0,
                name: "main.cpp".to_string(),
                relative_path: relative_path.clone(),
                group: "QtWidgets UI".to_string(),
                template_name: "qt_widgets_main_cpp".to_string(),
                feature: None,
                entity: None,
                use_case: None,
                field: None,
            });

            files.push(File {
                id: 0,
                name: "main_window.cpp".to_string(),
                relative_path: relative_path.clone(),
                group: "QtWidgets UI".to_string(),
                template_name: "qt_widgets_main_window_cpp".to_string(),
                feature: None,
                entity: None,
                use_case: None,
                field: None,
            });

            files.push(File {
                id: 0,
                name: "main_window.h".to_string(),
                relative_path: relative_path.clone(),
                group: "QtWidgets UI".to_string(),
                template_name: "qt_widgets_main_window_h".to_string(),
                feature: None,
                entity: None,
                use_case: None,
                field: None,
            });
        }

        // Keep only the files already existing
        let files = files
            .into_iter()
            .filter(|file| {
                if dto.only_list_already_existing {
                    let full_path = format!("{}{}", file.relative_path, file.name);
                    std::path::Path::new(&full_path).exists()
                } else {
                    true
                }
            })
            .collect::<Vec<File>>();

        // create files in db
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

        Ok(ListCppQtFilesReturnDto {
            file_ids,
            file_names,
            file_groups,
        })
    }
}
