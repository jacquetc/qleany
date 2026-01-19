//! Use Case handlers module
//!
//! This module contains functions for use case management including
//! event subscriptions, list operations, form handling, and callbacks.

use std::sync::Arc;

use common::direct_access::feature::FeatureRelationshipField;
use common::direct_access::use_case::UseCaseRelationshipField;
use common::direct_access::workspace::WorkspaceRelationshipField;
use common::event::{DirectAccessEntity, EntityEvent, Origin};
use direct_access::{FeatureRelationshipDto, UseCaseRelationshipDto};
use slint::ComponentHandle;

use crate::app_context::AppContext;
use crate::commands::{entity_commands, feature_commands, use_case_commands, workspace_commands};
use crate::event_hub_client::EventHubClient;
use crate::{App, AppState, FeaturesTabState, ListItem};

use super::dto_in_handlers::{clear_dto_in_form, fill_dto_in_field_list, fill_dto_in_form};
use super::dto_out_handlers::{clear_dto_out_form, fill_dto_out_field_list, fill_dto_out_form};

/// Subscribe to UseCase update events to refresh use_case_cr_list
pub fn subscribe_use_case_updated_event(
    event_hub_client: &EventHubClient,
    app: &App,
    app_context: &Arc<AppContext>,
) {
    event_hub_client.subscribe(
        Origin::DirectAccess(DirectAccessEntity::UseCase(EntityEvent::Updated)),
        {
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |event| {
                log::info!("UseCase updated event received: {:?}", event);
                let ctx = Arc::clone(&ctx);
                let app_weak = app_weak.clone();

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_weak.upgrade()
                        && app.global::<AppState>().get_manifest_is_open()
                    {
                        fill_use_case_list(&app, &ctx);
                        app.global::<AppState>().set_manifest_is_saved(false);
                    }
                });
            }
        },
    )
}

/// Subscribe to UseCase deletion events
pub fn subscribe_use_case_deleted_event(
    event_hub_client: &EventHubClient,
    app: &App,
    app_context: &Arc<AppContext>,
) {
    event_hub_client.subscribe(
        Origin::DirectAccess(DirectAccessEntity::UseCase(EntityEvent::Removed)),
        {
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |event| {
                log::info!("UseCase updated event received: {:?}", event);
                let _ctx = Arc::clone(&ctx);
                let app_weak = app_weak.clone();

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_weak.upgrade()
                        && app.global::<AppState>().get_manifest_is_open()
                    {
                        app.global::<AppState>().set_manifest_is_saved(false);
                    }
                });
            }
        },
    )
}

pub fn fill_use_case_list(app: &App, app_context: &Arc<AppContext>) {
    let ctx = Arc::clone(app_context);
    let app_weak = app.as_weak();

    if let Some(app) = app_weak.upgrade() {
        let feature_id =
            app.global::<FeaturesTabState>().get_selected_feature_id() as common::types::EntityId;

        if feature_id > 0 {
            let use_case_ids_res = feature_commands::get_feature_relationship(
                &ctx,
                &feature_id,
                &FeatureRelationshipField::UseCases,
            );

            match use_case_ids_res {
                Ok(use_case_ids) => {
                    // empty field list if no use cases
                    if use_case_ids.is_empty() {
                        let model = std::rc::Rc::new(slint::VecModel::from(Vec::<ListItem>::new()));
                        app.global::<FeaturesTabState>()
                            .set_use_case_cr_list(model.into());
                        log::info!("Use case list cleared (no use cases)");
                        return;
                    }

                    match use_case_commands::get_use_case_multi(&ctx, &use_case_ids) {
                        Ok(use_cases_opt) => {
                            let mut list: Vec<ListItem> = Vec::new();
                            for uc in use_cases_opt.into_iter().flatten() {
                                list.push(ListItem {
                                    id: uc.id as i32,
                                    text: slint::SharedString::from(uc.name),
                                    subtitle: slint::SharedString::from(""),
                                    checked: false,
                                });
                            }

                            let model = std::rc::Rc::new(slint::VecModel::from(list));
                            app.global::<FeaturesTabState>()
                                .set_use_case_cr_list(model.into());
                            log::info!("Use case list refreshed");
                        }
                        Err(e) => {
                            log::error!("Failed to fetch use cases: {}", e);
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to get feature use cases: {}", e);
                }
            }
        }
    }
}

pub fn clear_use_case_list(app: &App, app_context: &Arc<AppContext>) {
    let _ctx = Arc::clone(app_context);
    let app_weak = app.as_weak();

    if let Some(app) = app_weak.upgrade() {
        // Clear use case list
        let model = std::rc::Rc::new(slint::VecModel::from(Vec::<ListItem>::new()));
        app.global::<FeaturesTabState>()
            .set_use_case_cr_list(model.into());
        log::info!("Use case list cleared");
    }
}

/// Helper function to fill use case form from UseCaseDto
pub fn fill_use_case_form(app: &App, use_case: &direct_access::UseCaseDto) {
    let state = app.global::<FeaturesTabState>();
    state.set_selected_use_case_id(use_case.id as i32);
    state.set_selected_use_case_name(use_case.name.clone().into());
    state.set_selected_use_case_validator(use_case.validator);
    state.set_selected_use_case_undoable(use_case.undoable);
    state.set_selected_use_case_read_only(use_case.read_only);
    state.set_selected_use_case_long_operation(use_case.long_operation);
}

/// Helper function to clear use case form
pub fn clear_use_case_form(app: &App) {
    let state = app.global::<FeaturesTabState>();
    state.set_selected_use_case_id(-1);
    state.set_selected_use_case_name("".into());
    state.set_selected_use_case_validator(false);
    state.set_selected_use_case_undoable(false);
    state.set_selected_use_case_read_only(false);
    state.set_selected_use_case_long_operation(false);
    // Also clear DTOs when use case is cleared
    clear_dto_in_form(app);
    clear_dto_out_form(app);
}

/// Helper function to update a use case with new values
pub fn update_use_case_helper<F>(
    app: &App,
    app_context: &Arc<AppContext>,
    use_case_id: i32,
    update_fn: F,
) where
    F: FnOnce(&mut direct_access::UseCaseDto),
{
    if use_case_id < 0 {
        return;
    }

    let use_case_res =
        use_case_commands::get_use_case(app_context, &(use_case_id as common::types::EntityId));

    if let Ok(Some(mut use_case)) = use_case_res {
        update_fn(&mut use_case);
        match use_case_commands::update_use_case(
            app_context,
            Some(
                app.global::<FeaturesTabState>()
                    .get_features_undo_stack_id() as u64,
            ),
            &use_case,
        ) {
            Ok(_) => {
                log::info!("Use case updated successfully");
            }
            Err(e) => {
                log::error!("Failed to update use case: {}", e);
            }
        }
    }
}

/// Load DTOs for a use case
pub fn load_use_case_dtos(
    app: &App,
    app_context: &Arc<AppContext>,
    use_case_id: common::types::EntityId,
) {
    use crate::commands::dto_commands;

    // Load DTO In
    let dto_in_ids_res = use_case_commands::get_use_case_relationship(
        app_context,
        &use_case_id,
        &UseCaseRelationshipField::DtoIn,
    );

    match dto_in_ids_res {
        Ok(dto_in_ids) => {
            if let Some(dto_in_id) = dto_in_ids.first() {
                match dto_commands::get_dto(app_context, dto_in_id) {
                    Ok(Some(dto)) => {
                        fill_dto_in_form(app, &dto);
                        fill_dto_in_field_list(app, app_context);
                    }
                    _ => {
                        clear_dto_in_form(app);
                    }
                }
            } else {
                clear_dto_in_form(app);
            }
        }
        Err(e) => {
            log::error!("Failed to get DTO In relationship: {}", e);
            clear_dto_in_form(app);
        }
    }

    // Load DTO Out
    let dto_out_ids_res = use_case_commands::get_use_case_relationship(
        app_context,
        &use_case_id,
        &UseCaseRelationshipField::DtoOut,
    );

    match dto_out_ids_res {
        Ok(dto_out_ids) => {
            if let Some(dto_out_id) = dto_out_ids.first() {
                match dto_commands::get_dto(app_context, dto_out_id) {
                    Ok(Some(dto)) => {
                        fill_dto_out_form(app, &dto);
                        fill_dto_out_field_list(app, app_context);
                    }
                    _ => {
                        clear_dto_out_form(app);
                    }
                }
            } else {
                clear_dto_out_form(app);
            }
        }
        Err(e) => {
            log::error!("Failed to get DTO Out relationship: {}", e);
            clear_dto_out_form(app);
        }
    }
}

/// Fill the use case entity list with all entities, marking which are associated with the use case
pub fn fill_use_case_entity_list(app: &App, app_context: &Arc<AppContext>) {
    let use_case_id = app.global::<FeaturesTabState>().get_selected_use_case_id();
    if use_case_id < 0 {
        // Clear the list if no use case is selected
        let empty_model = std::rc::Rc::new(slint::VecModel::<ListItem>::default());
        app.global::<FeaturesTabState>()
            .set_use_case_entity_cr_list(empty_model.into());
        return;
    }

    // Get all entity IDs from workspace
    let workspace_id = app.global::<AppState>().get_workspace_id() as common::types::EntityId;
    let all_entity_ids_res = workspace_commands::get_workspace_relationship(
        app_context,
        &workspace_id,
        &WorkspaceRelationshipField::Entities,
    );

    let all_entity_ids = match all_entity_ids_res {
        Ok(ids) => ids,
        Err(e) => {
            log::error!("Failed to get all entities from workspace: {}", e);
            return;
        }
    };

    // Get entity IDs associated with this use case
    let use_case_entity_ids_res = use_case_commands::get_use_case_relationship(
        app_context,
        &(use_case_id as common::types::EntityId),
        &UseCaseRelationshipField::Entities,
    );

    let use_case_entity_ids: Vec<common::types::EntityId> = match use_case_entity_ids_res {
        Ok(ids) => ids,
        Err(e) => {
            log::error!("Failed to get use case entities: {}", e);
            Vec::new()
        }
    };

    // Get all entity details
    match entity_commands::get_entity_multi(app_context, &all_entity_ids) {
        Ok(entities_options) => {
            let items: Vec<ListItem> = entities_options
                .iter()
                .filter_map(|opt| opt.as_ref())
                .map(|entity| {
                    let is_checked = use_case_entity_ids.contains(&entity.id);
                    ListItem {
                        id: entity.id as i32,
                        text: slint::SharedString::from(&entity.name),
                        subtitle: slint::SharedString::from(""),
                        checked: is_checked,
                    }
                })
                .collect();

            let model = std::rc::Rc::new(slint::VecModel::from(items));
            app.global::<FeaturesTabState>()
                .set_use_case_entity_cr_list(model.into());
            log::info!("Use case entity list refreshed");
        }
        Err(e) => {
            log::error!("Failed to get entity details: {}", e);
        }
    }
}

pub fn setup_use_cases_reorder_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_request_use_cases_reorder({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |from_index, to_index| {
                let from = from_index as usize;
                let to = to_index as usize;

                if let Some(app) = app_weak.upgrade() {
                    let feature_id = app.global::<FeaturesTabState>().get_selected_feature_id()
                        as common::types::EntityId;
                    let use_case_ids_res = feature_commands::get_feature_relationship(
                        &ctx,
                        &feature_id,
                        &FeatureRelationshipField::UseCases,
                    );
                    let mut use_case_ids = use_case_ids_res.unwrap_or_default();

                    if from == to || from >= use_case_ids.len() {
                        return;
                    }

                    let moving_use_case_id = use_case_ids.remove(from);
                    let mut insert_at = if to > from { to - 1 } else { to };
                    if insert_at > use_case_ids.len() {
                        insert_at = use_case_ids.len();
                    }
                    use_case_ids.insert(insert_at, moving_use_case_id);

                    let result = feature_commands::set_feature_relationship(
                        &ctx,
                        Some(
                            app.global::<FeaturesTabState>()
                                .get_features_undo_stack_id() as u64,
                        ),
                        &FeatureRelationshipDto {
                            id: feature_id,
                            field: FeatureRelationshipField::UseCases,
                            right_ids: use_case_ids,
                        },
                    );

                    match result {
                        Ok(()) => {
                            log::info!("Use cases reordered successfully");
                        }
                        Err(e) => {
                            log::error!("Failed to reorder use cases: {}", e);
                        }
                    }
                }
            }
        });
}

pub fn setup_use_case_deletion_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_request_use_case_deletion({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |use_case_id| {
                if use_case_id < 0 {
                    return;
                }
                if let Some(app) = app_weak.upgrade() {
                    let result = use_case_commands::remove_use_case(
                        &ctx,
                        Some(
                            app.global::<FeaturesTabState>()
                                .get_features_undo_stack_id() as u64,
                        ),
                        &(use_case_id as common::types::EntityId),
                    );
                    match result {
                        Ok(()) => {
                            log::info!("Use case deleted successfully");
                            // Clear use case form
                            clear_use_case_form(&app);
                        }
                        Err(e) => {
                            log::error!("Failed to delete use case: {}", e);
                        }
                    }
                }
            }
        });
}

pub fn setup_select_use_case_callbacks(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_use_case_selected({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |use_case_id| {
            if use_case_id < 0 {
                return;
            }
            if let Some(app) = app_weak.upgrade() {
                let use_case_res = use_case_commands::get_use_case(
                    &ctx,
                    &(use_case_id as common::types::EntityId),
                );
                match use_case_res {
                    Ok(Some(use_case)) => {
                        fill_use_case_form(&app, &use_case);
                        // Load DTOs for this use case
                        load_use_case_dtos(&app, &ctx, use_case_id as common::types::EntityId);
                        // Load entity list for this use case
                        fill_use_case_entity_list(&app, &ctx);
                        log::info!("Use case selected: {}", use_case.name);
                    }
                    Ok(None) => {
                        log::warn!("Use case not found: {}", use_case_id);
                    }
                    Err(e) => {
                        log::error!("Failed to get use case: {}", e);
                    }
                }
            }
        }
    });
}

pub fn setup_use_case_name_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_use_case_name_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |name| {
            if let Some(app) = app_weak.upgrade() {
                let use_case_id = app.global::<FeaturesTabState>().get_selected_use_case_id();
                update_use_case_helper(&app, &ctx, use_case_id, |uc| {
                    uc.name = name.to_string();
                });
            }
        }
    });
}

pub fn setup_use_case_validator_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_use_case_validator_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |validator| {
                if let Some(app) = app_weak.upgrade() {
                    let use_case_id = app.global::<FeaturesTabState>().get_selected_use_case_id();
                    update_use_case_helper(&app, &ctx, use_case_id, |uc| {
                        uc.validator = validator;
                    });
                }
            }
        });
}

pub fn setup_use_case_undoable_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_use_case_undoable_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |undoable| {
                if let Some(app) = app_weak.upgrade() {
                    let use_case_id = app.global::<FeaturesTabState>().get_selected_use_case_id();
                    update_use_case_helper(&app, &ctx, use_case_id, |uc| {
                        uc.undoable = undoable;
                    });
                }
            }
        });
}

pub fn setup_use_case_read_only_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_use_case_read_only_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |read_only| {
                if let Some(app) = app_weak.upgrade() {
                    let use_case_id = app.global::<FeaturesTabState>().get_selected_use_case_id();
                    update_use_case_helper(&app, &ctx, use_case_id, |uc| {
                        uc.read_only = read_only;
                    });
                }
            }
        });
}

pub fn setup_use_case_long_operation_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_use_case_long_operation_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |long_operation| {
                if let Some(app) = app_weak.upgrade() {
                    let use_case_id = app.global::<FeaturesTabState>().get_selected_use_case_id();
                    update_use_case_helper(&app, &ctx, use_case_id, |uc| {
                        uc.long_operation = long_operation;
                    });
                }
            }
        });
}

pub fn setup_use_case_entity_check_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_use_case_entity_check_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |entity_id, checked| {
                let ctx = Arc::clone(&ctx);
                let app_weak = app_weak.clone();

                if let Some(app) = app_weak.upgrade() {
                    let use_case_id = app.global::<FeaturesTabState>().get_selected_use_case_id();
                    if use_case_id < 0 {
                        return;
                    }

                    // Get current entity IDs for this use case
                    let entity_ids_res = use_case_commands::get_use_case_relationship(
                        &ctx,
                        &(use_case_id as common::types::EntityId),
                        &UseCaseRelationshipField::Entities,
                    );

                    let mut entity_ids = match entity_ids_res {
                        Ok(ids) => ids,
                        Err(e) => {
                            log::error!("Failed to get use case entities: {}", e);
                            return;
                        }
                    };

                    let entity_id_u64 = entity_id as common::types::EntityId;

                    if checked {
                        // Add entity if not already present
                        if !entity_ids.contains(&entity_id_u64) {
                            entity_ids.push(entity_id_u64);
                        }
                    } else {
                        // Remove entity
                        entity_ids.retain(|&id| id != entity_id_u64);
                    }

                    // Update the relationship
                    let relationship_dto = UseCaseRelationshipDto {
                        id: use_case_id as common::types::EntityId,
                        field: UseCaseRelationshipField::Entities,
                        right_ids: entity_ids,
                    };

                    match use_case_commands::set_use_case_relationship(
                        &ctx,
                        Some(
                            app.global::<FeaturesTabState>()
                                .get_features_undo_stack_id() as u64,
                        ),
                        &relationship_dto,
                    ) {
                        Ok(()) => {
                            log::info!(
                                "Use case entity {} {}",
                                entity_id,
                                if checked { "added" } else { "removed" }
                            );
                            // Refresh the entity list to reflect the change
                            let _ = slint::invoke_from_event_loop(move || {
                                if let Some(app) = app_weak.upgrade() {
                                    fill_use_case_entity_list(&app, &ctx);
                                }
                            });
                        }
                        Err(e) => {
                            log::error!("Failed to update use case entities: {}", e);
                            // Refresh to revert UI state
                            let _ = slint::invoke_from_event_loop(move || {
                                if let Some(app) = app_weak.upgrade() {
                                    fill_use_case_entity_list(&app, &ctx);
                                }
                            });
                        }
                    }
                }
            }
        });
}

pub fn setup_use_case_addition_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_request_use_case_addition({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move || {
                if let Some(app) = app_weak.upgrade() {
                    let feature_id = app.global::<FeaturesTabState>().get_selected_feature_id();
                    if feature_id < 0 {
                        log::warn!("Cannot add use case: no feature selected");
                        return;
                    }

                    // Create a new use case with default values
                    let create_dto = direct_access::CreateUseCaseDto {
                        name: "NewUseCase".to_string(),
                        validator: false,
                        undoable: false,
                        read_only: false,
                        long_operation: false,
                        dto_in: None,
                        dto_out: None,
                        entities: vec![],
                    };

                    match use_case_commands::create_use_case(
                        &ctx,
                        Some(
                            app.global::<FeaturesTabState>()
                                .get_features_undo_stack_id() as u64,
                        ),
                        &create_dto,
                    ) {
                        Ok(new_use_case) => {
                            log::info!(
                                "Use case created successfully with id: {}",
                                new_use_case.id
                            );

                            // Get current use case ids from feature
                            let use_case_ids_res = feature_commands::get_feature_relationship(
                                &ctx,
                                &(feature_id as common::types::EntityId),
                                &FeatureRelationshipField::UseCases,
                            );

                            match use_case_ids_res {
                                Ok(mut use_case_ids) => {
                                    // Add the new use case id to the list
                                    use_case_ids.push(new_use_case.id);

                                    // Update the feature relationship
                                    let relationship_dto = FeatureRelationshipDto {
                                        id: feature_id as common::types::EntityId,
                                        field: FeatureRelationshipField::UseCases,
                                        right_ids: use_case_ids,
                                    };

                                    if let Err(e) = feature_commands::set_feature_relationship(
                                        &ctx,
                                        Some(
                                            app.global::<FeaturesTabState>()
                                                .get_features_undo_stack_id()
                                                as u64,
                                        ),
                                        &relationship_dto,
                                    ) {
                                        log::error!(
                                            "Failed to add use case to feature relationship: {}",
                                            e
                                        );
                                    }
                                }
                                Err(e) => {
                                    log::error!(
                                        "Failed to get feature use cases relationship: {}",
                                        e
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to create use case: {}", e);
                        }
                    }
                }
            }
        });
}
