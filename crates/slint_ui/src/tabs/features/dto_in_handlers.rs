//! DTO In handlers module
//!
//! This module contains functions for DTO In (input) management including
//! form handling, field list operations, and callbacks.

use crate::app_context::AppContext;
use crate::commands::{dto_commands, dto_field_commands, use_case_commands};
use crate::event_hub_client::EventHubClient;
use crate::{App, AppState, FeaturesTabState, ListItem};
use common::direct_access::dto::DtoRelationshipField;
use common::direct_access::use_case::UseCaseRelationshipField;
use common::entities::DtoFieldType;
use common::event::{DirectAccessEntity, EntityEvent, Origin};
use direct_access::{DtoRelationshipDto, UseCaseRelationshipDto};
use log::log;
use slint::ComponentHandle;
use std::sync::Arc;

pub fn subscribe_dto_updated_event(
    event_hub_client: &EventHubClient,
    app: &App,
    app_context: &Arc<AppContext>,
) {
    event_hub_client.subscribe(
        Origin::DirectAccess(DirectAccessEntity::Dto(EntityEvent::Updated)),
        {
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |event| {
                log::info!("Dto updated event received: {:?}", event);
                let ctx = Arc::clone(&ctx);
                let app_weak = app_weak.clone();

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_weak.upgrade()
                        && app.global::<AppState>().get_manifest_is_open()
                    {
                        fill_dto_in_field_list(&app, &ctx);
                        app.global::<AppState>().set_manifest_is_saved(false);
                    }
                });
            }
        },
    )
}

pub fn subscribe_use_case_updated_event(
    event_hub_client: &EventHubClient,
    app: &App,
    app_context: &Arc<AppContext>,
) {
    // event_hub_client.subscribe(
    //     Origin::DirectAccess(DirectAccessEntity::UseCase(EntityEvent::Updated)),
    //     {
    //         let ctx = Arc::clone(app_context);
    //         let app_weak = app.as_weak();
    //         move |event| {
    //             log::info!("Dto updated event received: {:?}", event);
    //             let ctx = Arc::clone(&ctx);
    //             let app_weak = app_weak.clone();
    //
    //             let _ = slint::invoke_from_event_loop(move || {
    //                 if let Some(app) = app_weak.upgrade()
    //                     && app.global::<AppState>().get_manifest_is_open()
    //                 {
    //                     let use_case_id =
    //                         app.global::<FeaturesTabState>().get_selected_use_case_id();
    //                     let dto_dto =
    //                         dto_commands::get_dto(&ctx, &(use_case_id as common::types::EntityId));
    //                     fill_dto_in_form(
    //                         &app,
    //                         &dto_dto
    //                             .expect("Failed to get DTO for use case")
    //                             .as_ref()
    //                             .expect("DTO not found"),
    //                     );
    //                     fill_dto_in_field_list(&app, &ctx);
    //                     app.global::<AppState>().set_manifest_is_saved(false);
    //                 }
    //             });
    //         }
    //     },
    // )
}

pub fn subscribe_dto_deleted_event(
    event_hub_client: &EventHubClient,
    app: &App,
    app_context: &Arc<AppContext>,
) {
    event_hub_client.subscribe(
        Origin::DirectAccess(DirectAccessEntity::Dto(EntityEvent::Removed)),
        {
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |event| {
                log::info!("Dto updated event received: {:?}", event);
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

/// Convert DtoFieldType to ComboBox index
pub fn dto_field_type_to_index(field_type: &DtoFieldType) -> i32 {
    match field_type {
        DtoFieldType::Boolean => 0,
        DtoFieldType::Integer => 1,
        DtoFieldType::UInteger => 2,
        DtoFieldType::Float => 3,
        DtoFieldType::String => 4,
        DtoFieldType::Uuid => 5,
        DtoFieldType::DateTime => 6,
        DtoFieldType::Enum => 7,
    }
}

/// Convert ComboBox index to DtoFieldType
pub fn index_to_dto_field_type(index: i32) -> DtoFieldType {
    match index {
        0 => DtoFieldType::Boolean,
        1 => DtoFieldType::Integer,
        2 => DtoFieldType::UInteger,
        3 => DtoFieldType::Float,
        4 => DtoFieldType::String,
        5 => DtoFieldType::Uuid,
        6 => DtoFieldType::DateTime,
        7 => DtoFieldType::Enum,
        _ => DtoFieldType::String,
    }
}

/// Helper function to fill DTO In form from DtoDto
pub fn fill_dto_in_form(app: &App, dto: &direct_access::DtoDto) {
    let state = app.global::<FeaturesTabState>();
    state.set_dto_in_enabled(true);
    state.set_selected_dto_in_id(dto.id as i32);
    state.set_selected_dto_in_name(dto.name.clone().into());
}

/// Helper function to clear DTO In form
pub fn clear_dto_in_form(app: &App) {
    let state = app.global::<FeaturesTabState>();
    state.set_dto_in_enabled(false);
    state.set_selected_dto_in_id(-1);
    state.set_selected_dto_in_name("".into());
    // Clear DTO In fields
    let empty_model: std::rc::Rc<slint::VecModel<ListItem>> =
        std::rc::Rc::new(slint::VecModel::from(vec![]));
    state.set_dto_in_field_cr_list(empty_model.into());
    clear_dto_in_field_form(app);
}

/// Helper function to fill DTO In field form from DtoFieldDto
pub fn fill_dto_in_field_form(app: &App, dto_field: &direct_access::DtoFieldDto) {
    let state = app.global::<FeaturesTabState>();
    state.set_selected_dto_in_field_id(dto_field.id as i32);
    state.set_selected_dto_in_field_name(dto_field.name.clone().into());
    state.set_selected_dto_in_field_type_index(dto_field_type_to_index(&dto_field.field_type));
    state.set_selected_dto_in_field_optional(dto_field.optional);
    state.set_selected_dto_in_field_is_list(dto_field.is_list);
}

/// Helper function to clear DTO In field form
pub fn clear_dto_in_field_form(app: &App) {
    let state = app.global::<FeaturesTabState>();
    state.set_selected_dto_in_field_id(-1);
    state.set_selected_dto_in_field_name("".into());
    state.set_selected_dto_in_field_type_index(4); // Default to String
    state.set_selected_dto_in_field_optional(false);
    state.set_selected_dto_in_field_is_list(false);
}

/// Helper function to fill DTO In field list
pub fn fill_dto_in_field_list(app: &App, app_context: &Arc<AppContext>) {
    let state = app.global::<FeaturesTabState>();
    let dto_id_i32 = state.get_selected_dto_in_id();

    if dto_id_i32 < 0 {
        let empty_model: std::rc::Rc<slint::VecModel<ListItem>> =
            std::rc::Rc::new(slint::VecModel::from(vec![]));
        state.set_dto_in_field_cr_list(empty_model.into());
        return;
    }

    let dto_id = dto_id_i32 as common::types::EntityId;

    let field_ids_res =
        dto_commands::get_dto_relationship(app_context, &dto_id, &DtoRelationshipField::Fields);

    match field_ids_res {
        Ok(field_ids) => {
            // empty field list if no fields
            if field_ids.is_empty() {
                let model = std::rc::Rc::new(slint::VecModel::from(vec![]));
                state.set_dto_in_field_cr_list(model.into());
                log::info!("DTO In field list cleared (no fields)");
                return;
            }

            // Fetch field details
            match dto_field_commands::get_dto_field_multi(app_context, &field_ids) {
                Ok(fields_opt) => {
                    let mut list: Vec<ListItem> = Vec::new();
                    for f in fields_opt.into_iter().flatten() {
                        list.push(ListItem {
                            id: f.id as i32,
                            text: slint::SharedString::from(f.name),
                            subtitle: slint::SharedString::from(""),
                            checked: false,
                        });
                    }
                    let model = std::rc::Rc::new(slint::VecModel::from(list));
                    state.set_dto_in_field_cr_list(model.into());
                    log::info!("DTO In field list refreshed");
                }
                Err(e) => {
                    log::error!("Failed to fetch DTO In fields: {}", e);
                }
            }
        }
        Err(e) => {
            log::error!("Failed to get DTO In fields relationship: {}", e);
        }
    }
}

/// Helper function to update a DTO with new values
pub fn update_dto_helper<F>(app: &App, app_context: &Arc<AppContext>, dto_id: i32, update_fn: F)
where
    F: FnOnce(&mut direct_access::DtoDto),
{
    if dto_id < 0 {
        return;
    }

    let dto_res = dto_commands::get_dto(app_context, &(dto_id as common::types::EntityId));

    if let Ok(Some(mut dto)) = dto_res {
        update_fn(&mut dto);
        match dto_commands::update_dto(
            app_context,
            Some(
                app.global::<FeaturesTabState>()
                    .get_features_undo_stack_id() as u64,
            ),
            &dto,
        ) {
            Ok(_) => {
                log::info!("DTO updated successfully");
            }
            Err(e) => {
                log::error!("Failed to update DTO: {}", e);
            }
        }
    }
}

/// Helper function to update a DTO field with new values
pub fn update_dto_field_helper<F>(
    app: &App,
    app_context: &Arc<AppContext>,
    dto_field_id: i32,
    update_fn: F,
) where
    F: FnOnce(&mut direct_access::DtoFieldDto),
{
    if dto_field_id < 0 {
        return;
    }

    let dto_field_res =
        dto_field_commands::get_dto_field(app_context, &(dto_field_id as common::types::EntityId));

    if let Ok(Some(mut dto_field)) = dto_field_res {
        update_fn(&mut dto_field);
        match dto_field_commands::update_dto_field(
            app_context,
            Some(
                app.global::<FeaturesTabState>()
                    .get_features_undo_stack_id() as u64,
            ),
            &dto_field,
        ) {
            Ok(_) => {
                log::info!("DTO field updated successfully");
            }
            Err(e) => {
                log::error!("Failed to update DTO field: {}", e);
            }
        }
    }
}

pub fn setup_dto_in_enabled_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_dto_in_enabled_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |enabled| {
            let ctx = Arc::clone(&ctx);
            let app_weak = app_weak.clone();

            if let Some(app) = app_weak.upgrade() {
                let use_case_id = app.global::<FeaturesTabState>().get_selected_use_case_id();
                if use_case_id < 0 {
                    return;
                }

                let stack_id = app
                    .global::<FeaturesTabState>()
                    .get_features_undo_stack_id() as u64;

                if enabled {
                    // Create a new DTO In for this use case
                    let create_dto = direct_access::CreateDtoDto {
                        name: "NewDtoIn".to_string(),
                        fields: vec![],
                    };
                    match dto_commands::create_dto(&ctx, Some(stack_id), &create_dto) {
                        Ok(dto) => {
                            // Set the relationship
                            let relationship_dto = UseCaseRelationshipDto {
                                id: use_case_id as common::types::EntityId,
                                field: UseCaseRelationshipField::DtoIn,
                                right_ids: vec![dto.id],
                            };
                            match use_case_commands::set_use_case_relationship(
                                &ctx,
                                Some(stack_id),
                                &relationship_dto,
                            ) {
                                Ok(()) => {
                                    let app_weak2 = app_weak.clone();
                                    let _ = slint::invoke_from_event_loop(move || {
                                        if let Some(app) = app_weak2.upgrade() {
                                            fill_dto_in_form(&app, &dto);
                                            // New DTO has no fields, set empty list explicitly
                                            let empty_model: std::rc::Rc<
                                                slint::VecModel<ListItem>,
                                            > = std::rc::Rc::new(slint::VecModel::from(vec![]));
                                            app.global::<FeaturesTabState>()
                                                .set_dto_in_field_cr_list(empty_model.into());
                                            clear_dto_in_field_form(&app);
                                        }
                                    });
                                    log::info!("DTO In created and linked successfully");
                                }
                                Err(e) => {
                                    log::error!("Failed to link DTO In: {}", e);
                                    // Clean up the created DTO
                                    let _ = dto_commands::remove_dto(&ctx, Some(stack_id), &dto.id);
                                    let _ = slint::invoke_from_event_loop(move || {
                                        if let Some(app) = app_weak.upgrade() {
                                            app.global::<FeaturesTabState>()
                                                .set_dto_in_enabled(false);
                                        }
                                    });
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to create DTO In: {}", e);
                            let _ = slint::invoke_from_event_loop(move || {
                                if let Some(app) = app_weak.upgrade() {
                                    app.global::<FeaturesTabState>().set_dto_in_enabled(false);
                                }
                            });
                        }
                    }
                } else {
                    // Remove the DTO In relationship and optionally delete the DTO
                    let dto_id = app.global::<FeaturesTabState>().get_selected_dto_in_id();
                    if dto_id >= 0 {
                        // Clear the relationship first
                        let relationship_dto = UseCaseRelationshipDto {
                            id: use_case_id as common::types::EntityId,
                            field: UseCaseRelationshipField::DtoIn,
                            right_ids: vec![],
                        };
                        match use_case_commands::set_use_case_relationship(
                            &ctx,
                            Some(stack_id),
                            &relationship_dto,
                        ) {
                            Ok(()) => {
                                // Delete the DTO
                                let _ = dto_commands::remove_dto(
                                    &ctx,
                                    Some(stack_id),
                                    &(dto_id as common::types::EntityId),
                                );
                                let app_weak2 = app_weak.clone();
                                let _ = slint::invoke_from_event_loop(move || {
                                    if let Some(app) = app_weak2.upgrade() {
                                        clear_dto_in_form(&app);
                                        // Re-set enabled to false since clear_dto_in_form sets it
                                        app.global::<FeaturesTabState>().set_dto_in_enabled(false);
                                    }
                                });
                                log::info!("DTO In removed successfully");
                            }
                            Err(e) => {
                                log::error!("Failed to unlink DTO In: {}", e);
                                let _ = slint::invoke_from_event_loop(move || {
                                    if let Some(app) = app_weak.upgrade() {
                                        app.global::<FeaturesTabState>().set_dto_in_enabled(true);
                                    }
                                });
                            }
                        }
                    }
                }
            }
        }
    });
}

pub fn setup_dto_in_name_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_dto_in_name_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |name| {
            if let Some(app) = app_weak.upgrade() {
                let dto_id = app.global::<FeaturesTabState>().get_selected_dto_in_id();
                update_dto_helper(&app, &ctx, dto_id, |dto| {
                    dto.name = name.to_string();
                });
            }
        }
    });
}

pub fn setup_dto_in_field_selected_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_dto_in_field_selected({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |field_id| {
            // Ignore invalid or zero ids which may be emitted by the UI.
            if field_id <= 0 {
                log::warn!("Ignored invalid DTO In field selection id: {}", field_id);
                return;
            }
            if let Some(app) = app_weak.upgrade() {
                let field_res =
                    dto_field_commands::get_dto_field(&ctx, &(field_id as common::types::EntityId));
                match field_res {
                    Ok(Some(field)) => {
                        fill_dto_in_field_form(&app, &field);
                        log::info!("DTO In field selected: {}", field.name);
                    }
                    Ok(None) => {
                        log::warn!("DTO In field not found: {}", field_id);
                    }
                    Err(e) => {
                        log::error!("Failed to get DTO In field: {}", e);
                    }
                }
            }
        }
    });
}

pub fn setup_dto_in_field_name_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_dto_in_field_name_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |name| {
                if let Some(app) = app_weak.upgrade() {
                    let field_id = app
                        .global::<FeaturesTabState>()
                        .get_selected_dto_in_field_id();
                    update_dto_field_helper(&app, &ctx, field_id, |field| {
                        field.name = name.to_string();
                    });
                    // Refresh field list to show updated name
                    fill_dto_in_field_list(&app, &ctx);
                }
            }
        });
}

pub fn setup_dto_in_field_type_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_dto_in_field_type_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |_value| {
                if let Some(app) = app_weak.upgrade() {
                    let field_id = app
                        .global::<FeaturesTabState>()
                        .get_selected_dto_in_field_id();
                    let type_index = app
                        .global::<FeaturesTabState>()
                        .get_selected_dto_in_field_type_index();
                    let field_type = index_to_dto_field_type(type_index);
                    update_dto_field_helper(&app, &ctx, field_id, |field| {
                        field.field_type = field_type;
                    });
                }
            }
        });
}

pub fn setup_dto_in_field_optional_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_dto_in_field_optional_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |optional| {
                if let Some(app) = app_weak.upgrade() {
                    let field_id = app
                        .global::<FeaturesTabState>()
                        .get_selected_dto_in_field_id();
                    update_dto_field_helper(&app, &ctx, field_id, |field| {
                        field.optional = optional;
                    });
                }
            }
        });
}

pub fn setup_dto_in_field_is_list_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_dto_in_field_is_list_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |is_list| {
                if let Some(app) = app_weak.upgrade() {
                    let field_id = app
                        .global::<FeaturesTabState>()
                        .get_selected_dto_in_field_id();
                    update_dto_field_helper(&app, &ctx, field_id, |field| {
                        field.is_list = is_list;
                    });
                }
            }
        });
}

pub fn setup_dto_in_fields_reorder_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_request_dto_in_fields_reorder({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |from_index, to_index| {
                if from_index < 0 || to_index < 0 {
                    return;
                }
                let from = from_index as usize;
                let to = to_index as usize;

                if let Some(app) = app_weak.upgrade() {
                    let dto_id_i32 = app.global::<FeaturesTabState>().get_selected_dto_in_id();
                    if dto_id_i32 < 0 {
                        return;
                    }
                    let dto_id = dto_id_i32 as common::types::EntityId;
                    let field_ids_res = dto_commands::get_dto_relationship(
                        &ctx,
                        &dto_id,
                        &DtoRelationshipField::Fields,
                    );
                    let mut field_ids = field_ids_res.unwrap_or_default();

                    if from == to || from >= field_ids.len() {
                        return;
                    }

                    let moving_field_id = field_ids.remove(from);
                    let mut insert_at = if to > from { to - 1 } else { to };
                    if insert_at > field_ids.len() {
                        insert_at = field_ids.len();
                    }
                    field_ids.insert(insert_at, moving_field_id);

                    let result = dto_commands::set_dto_relationship(
                        &ctx,
                        Some(
                            app.global::<FeaturesTabState>()
                                .get_features_undo_stack_id() as u64,
                        ),
                        &DtoRelationshipDto {
                            id: dto_id,
                            field: DtoRelationshipField::Fields,
                            right_ids: field_ids,
                        },
                    );

                    match result {
                        Ok(()) => {
                            log::info!("DTO In fields reordered successfully");
                        }
                        Err(e) => {
                            log::error!("Failed to reorder DTO In fields: {}", e);
                        }
                    }
                }
            }
        });
}

pub fn setup_dto_in_field_deletion_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_request_dto_in_field_deletion({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |field_id| {
                if field_id < 0 {
                    return;
                }
                if let Some(app) = app_weak.upgrade() {
                    let result = dto_field_commands::remove_dto_field(
                        &ctx,
                        Some(
                            app.global::<FeaturesTabState>()
                                .get_features_undo_stack_id() as u64,
                        ),
                        &(field_id as common::types::EntityId),
                    );
                    match result {
                        Ok(()) => {
                            log::info!("DTO In field deleted successfully");
                            // Refresh DTO In field list
                            fill_dto_in_field_list(&app, &ctx);
                            // Clear DTO In field form
                            clear_dto_in_field_form(&app);
                        }
                        Err(e) => {
                            log::error!("Failed to delete DTO In field: {}", e);
                        }
                    }
                }
            }
        });
}

pub fn setup_dto_in_field_addition_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_request_dto_in_field_addition({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move || {
                let ctx = Arc::clone(&ctx);
                let app_weak = app_weak.clone();

                if let Some(app) = app_weak.upgrade() {
                    let dto_id = app.global::<FeaturesTabState>().get_selected_dto_in_id();
                    if dto_id < 0 {
                        log::warn!("Cannot add DTO In field: no DTO In selected");
                        return;
                    }

                    // Create a new DTO field with default values
                    let create_dto = direct_access::CreateDtoFieldDto {
                        name: "new_field".to_string(),
                        field_type: DtoFieldType::String,
                        optional: false,
                        is_list: false,
                        enum_name: None,
                        enum_values: None,
                    };

                    match dto_field_commands::create_dto_field(
                        &ctx,
                        Some(
                            app.global::<FeaturesTabState>()
                                .get_features_undo_stack_id() as u64,
                        ),
                        &create_dto,
                    ) {
                        Ok(new_field) => {
                            log::info!(
                                "DTO In field created successfully with id: {}",
                                new_field.id
                            );

                            // Get current field ids from DTO
                            let field_ids_res = dto_commands::get_dto_relationship(
                                &ctx,
                                &(dto_id as common::types::EntityId),
                                &DtoRelationshipField::Fields,
                            );

                            match field_ids_res {
                                Ok(mut field_ids) => {
                                    // Add the new field id to the list
                                    field_ids.push(new_field.id);

                                    // Update the DTO relationship
                                    let relationship_dto = DtoRelationshipDto {
                                        id: dto_id as common::types::EntityId,
                                        field: DtoRelationshipField::Fields,
                                        right_ids: field_ids,
                                    };

                                    if let Err(e) = dto_commands::set_dto_relationship(
                                        &ctx,
                                        Some(
                                            app.global::<FeaturesTabState>()
                                                .get_features_undo_stack_id()
                                                as u64,
                                        ),
                                        &relationship_dto,
                                    ) {
                                        log::error!(
                                            "Failed to add field to DTO In relationship: {}",
                                            e
                                        );
                                    } else {
                                        // Refresh the field list
                                        let _ = slint::invoke_from_event_loop(move || {
                                            if let Some(app) = app_weak.upgrade() {
                                                fill_dto_in_field_list(&app, &ctx);
                                            }
                                        });
                                    }
                                }
                                Err(e) => {
                                    log::error!("Failed to get DTO In fields relationship: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to create DTO In field: {}", e);
                        }
                    }
                }
            }
        });
}
