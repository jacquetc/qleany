//! DTO Out handlers module
//!
//! This module contains functions for DTO Out (output) management including
//! form handling, field list operations, and callbacks.

use std::sync::Arc;

use common::direct_access::dto::DtoRelationshipField;
use common::direct_access::use_case::UseCaseRelationshipField;
use direct_access::{DtoRelationshipDto, UseCaseRelationshipDto};
use slint::ComponentHandle;

use crate::app_context::AppContext;
use crate::commands::{dto_commands, dto_field_commands, use_case_commands};
use crate::{App, FeaturesTabState, ListItem};

use super::dto_in_handlers::{
    dto_field_type_to_index, index_to_dto_field_type, update_dto_field_helper, update_dto_helper,
};

/// Helper function to fill DTO Out form from DtoDto
pub fn fill_dto_out_form(app: &App, dto: &direct_access::DtoDto) {
    let state = app.global::<FeaturesTabState>();
    state.set_dto_out_enabled(true);
    state.set_selected_dto_out_id(dto.id as i32);
    state.set_selected_dto_out_name(dto.name.clone().into());
}

/// Helper function to clear DTO Out form
pub fn clear_dto_out_form(app: &App) {
    let state = app.global::<FeaturesTabState>();
    state.set_dto_out_enabled(false);
    state.set_selected_dto_out_id(-1);
    state.set_selected_dto_out_name("".into());
    // Clear DTO Out fields
    let empty_model: std::rc::Rc<slint::VecModel<ListItem>> =
        std::rc::Rc::new(slint::VecModel::from(vec![]));
    state.set_dto_out_field_cr_list(empty_model.into());
    clear_dto_out_field_form(app);
}

/// Helper function to fill DTO Out field form from DtoFieldDto
pub fn fill_dto_out_field_form(app: &App, dto_field: &direct_access::DtoFieldDto) {
    let state = app.global::<FeaturesTabState>();
    state.set_selected_dto_out_field_id(dto_field.id as i32);
    state.set_selected_dto_out_field_name(dto_field.name.clone().into());
    state.set_selected_dto_out_field_type_index(dto_field_type_to_index(&dto_field.field_type));
    state.set_selected_dto_out_field_is_nullable(dto_field.is_nullable);
    state.set_selected_dto_out_field_is_list(dto_field.is_list);
}

/// Helper function to clear DTO Out field form
pub fn clear_dto_out_field_form(app: &App) {
    let state = app.global::<FeaturesTabState>();
    state.set_selected_dto_out_field_id(-1);
    state.set_selected_dto_out_field_name("".into());
    state.set_selected_dto_out_field_type_index(4); // Default to String
    state.set_selected_dto_out_field_is_nullable(false);
    state.set_selected_dto_out_field_is_list(false);
}

/// Helper function to fill DTO Out field list
pub fn fill_dto_out_field_list(app: &App, app_context: &Arc<AppContext>) {
    let state = app.global::<FeaturesTabState>();
    let dto_id_i32 = state.get_selected_dto_out_id();

    if dto_id_i32 < 0 {
        let empty_model: std::rc::Rc<slint::VecModel<ListItem>> =
            std::rc::Rc::new(slint::VecModel::from(vec![]));
        state.set_dto_out_field_cr_list(empty_model.into());
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
                state.set_dto_out_field_cr_list(model.into());
                log::info!("DTO Out field list cleared (no fields)");
                return;
            }

            // Fetch field details
            match dto_field_commands::get_dto_field_multi(app_context, &field_ids) {
                Ok(fields_opt) => {
                    let mut list: Vec<ListItem> = Vec::new();
                    for maybe_field in fields_opt.into_iter() {
                        if let Some(f) = maybe_field {
                            list.push(ListItem {
                                id: f.id as i32,
                                text: slint::SharedString::from(f.name),
                                subtitle: slint::SharedString::from(""),
                                checked: false,
                            });
                        }
                    }
                    let model = std::rc::Rc::new(slint::VecModel::from(list));
                    state.set_dto_out_field_cr_list(model.into());
                    log::info!("DTO Out field list refreshed");
                }
                Err(e) => {
                    log::error!("Failed to fetch DTO Out fields: {}", e);
                }
            }
        }
        Err(e) => {
            log::error!("Failed to get DTO Out fields relationship: {}", e);
        }
    }
}

pub fn setup_dto_out_enabled_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_dto_out_enabled_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |enabled| {
                let ctx = Arc::clone(&ctx);
                let app_weak = app_weak.clone();

                // Run the potentially heavy operation in a background thread to avoid freezing the UI
                std::thread::spawn(move || {
                    if let Some(app) = app_weak.upgrade() {
                        let use_case_id = app.global::<FeaturesTabState>().get_selected_use_case_id();
                        if use_case_id < 0 {
                            return;
                        }

                        let stack_id = app
                            .global::<FeaturesTabState>()
                            .get_features_undo_stack_id() as u64;

                        if enabled {
                            // Create a new DTO Out for this use case
                            let create_dto = direct_access::CreateDtoDto {
                                name: "NewDtoOut".to_string(),
                                fields: vec![],
                            };
                            match dto_commands::create_dto(&ctx, Some(stack_id), &create_dto) {
                                Ok(dto) => {
                                    // Set the relationship
                                    let relationship_dto = UseCaseRelationshipDto {
                                        id: use_case_id as common::types::EntityId,
                                        field: UseCaseRelationshipField::DtoOut,
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
                                                    fill_dto_out_form(&app, &dto);
                                                    // New DTO has no fields, set empty list explicitly
                                                    let empty_model: std::rc::Rc<slint::VecModel<ListItem>> =
                                                        std::rc::Rc::new(slint::VecModel::from(vec![]));
                                                    app.global::<FeaturesTabState>()
                                                        .set_dto_out_field_cr_list(empty_model.into());
                                                    clear_dto_out_field_form(&app);
                                                }
                                            });
                                            log::info!("DTO Out created and linked successfully");
                                        }
                                        Err(e) => {
                                            log::error!("Failed to link DTO Out: {}", e);
                                            // Clean up the created DTO
                                            let _ =
                                                dto_commands::remove_dto(&ctx, Some(stack_id), &dto.id);
                                            let _ = slint::invoke_from_event_loop(move || {
                                                if let Some(app) = app_weak.upgrade() {
                                                    app.global::<FeaturesTabState>().set_dto_out_enabled(false);
                                                }
                                            });
                                        }
                                    }
                                }
                                Err(e) => {
                                    log::error!("Failed to create DTO Out: {}", e);
                                    let _ = slint::invoke_from_event_loop(move || {
                                        if let Some(app) = app_weak.upgrade() {
                                            app.global::<FeaturesTabState>().set_dto_out_enabled(false);
                                        }
                                    });
                                }
                            }
                        } else {
                            // Remove the DTO Out relationship and optionally delete the DTO
                            let dto_id = app.global::<FeaturesTabState>().get_selected_dto_out_id();
                            if dto_id >= 0 {
                                // Clear the relationship first
                                let relationship_dto = UseCaseRelationshipDto {
                                    id: use_case_id as common::types::EntityId,
                                    field: UseCaseRelationshipField::DtoOut,
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
                                                clear_dto_out_form(&app);
                                                // Re-set enabled to false since clear_dto_out_form sets it
                                                app.global::<FeaturesTabState>().set_dto_out_enabled(false);
                                            }
                                        });
                                        log::info!("DTO Out removed successfully");
                                    }
                                    Err(e) => {
                                        log::error!("Failed to unlink DTO Out: {}", e);
                                        let _ = slint::invoke_from_event_loop(move || {
                                            if let Some(app) = app_weak.upgrade() {
                                                app.global::<FeaturesTabState>().set_dto_out_enabled(true);
                                            }
                                        });
                                    }
                                }
                            }
                        }
                    }
                });
            }
        });
}

pub fn setup_dto_out_name_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_dto_out_name_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |name| {
            if let Some(app) = app_weak.upgrade() {
                let dto_id = app.global::<FeaturesTabState>().get_selected_dto_out_id();
                update_dto_helper(&app, &ctx, dto_id, |dto| {
                    dto.name = name.to_string();
                });
            }
        }
    });
}

pub fn setup_dto_out_field_selected_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_dto_out_field_selected({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |field_id| {
            if field_id < 0 {
                return;
            }
            if let Some(app) = app_weak.upgrade() {
                let field_res =
                    dto_field_commands::get_dto_field(&ctx, &(field_id as common::types::EntityId));
                match field_res {
                    Ok(Some(field)) => {
                        fill_dto_out_field_form(&app, &field);
                        log::info!("DTO Out field selected: {}", field.name);
                    }
                    Ok(None) => {
                        log::warn!("DTO Out field not found: {}", field_id);
                    }
                    Err(e) => {
                        log::error!("Failed to get DTO Out field: {}", e);
                    }
                }
            }
        }
    });
}

pub fn setup_dto_out_field_name_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_dto_out_field_name_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |name| {
                if let Some(app) = app_weak.upgrade() {
                    let field_id = app
                        .global::<FeaturesTabState>()
                        .get_selected_dto_out_field_id();
                    update_dto_field_helper(&app, &ctx, field_id, |field| {
                        field.name = name.to_string();
                    });
                    // Refresh field list to show updated name
                    fill_dto_out_field_list(&app, &ctx);
                }
            }
        });
}

pub fn setup_dto_out_field_type_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_dto_out_field_type_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |_value| {
                if let Some(app) = app_weak.upgrade() {
                    let field_id = app
                        .global::<FeaturesTabState>()
                        .get_selected_dto_out_field_id();
                    let type_index = app
                        .global::<FeaturesTabState>()
                        .get_selected_dto_out_field_type_index();
                    let field_type = index_to_dto_field_type(type_index);
                    update_dto_field_helper(&app, &ctx, field_id, |field| {
                        field.field_type = field_type;
                    });
                }
            }
        });
}

pub fn setup_dto_out_field_is_nullable_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_dto_out_field_is_nullable_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |is_nullable| {
                if let Some(app) = app_weak.upgrade() {
                    let field_id = app
                        .global::<FeaturesTabState>()
                        .get_selected_dto_out_field_id();
                    update_dto_field_helper(&app, &ctx, field_id, |field| {
                        field.is_nullable = is_nullable;
                    });
                }
            }
        });
}

pub fn setup_dto_out_field_is_list_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_dto_out_field_is_list_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |is_list| {
                if let Some(app) = app_weak.upgrade() {
                    let field_id = app
                        .global::<FeaturesTabState>()
                        .get_selected_dto_out_field_id();
                    update_dto_field_helper(&app, &ctx, field_id, |field| {
                        field.is_list = is_list;
                    });
                }
            }
        });
}

pub fn setup_dto_out_fields_reorder_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_request_dto_out_fields_reorder({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |from_index, to_index| {
                if from_index < 0 || to_index < 0 {
                    return;
                }
                let from = from_index as usize;
                let to = to_index as usize;

                if let Some(app) = app_weak.upgrade() {
                    let dto_id_i32 = app.global::<FeaturesTabState>().get_selected_dto_out_id();
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

                    if from == to || from >= field_ids.iter().count() {
                        return;
                    }

                    let moving_field_id = field_ids.remove(from);
                    let mut insert_at = if to > from { to - 1 } else { to };
                    if insert_at > field_ids.iter().count() {
                        insert_at = field_ids.iter().count();
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
                            log::info!("DTO Out fields reordered successfully");
                        }
                        Err(e) => {
                            log::error!("Failed to reorder DTO Out fields: {}", e);
                        }
                    }
                }
            }
        });
}

pub fn setup_dto_out_field_deletion_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_request_dto_out_field_deletion({
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
                            log::info!("DTO Out field deleted successfully");
                            // Refresh DTO Out field list
                            fill_dto_out_field_list(&app, &ctx);
                            // Clear DTO Out field form
                            clear_dto_out_field_form(&app);
                        }
                        Err(e) => {
                            log::error!("Failed to delete DTO Out field: {}", e);
                        }
                    }
                }
            }
        });
}

pub fn setup_dto_out_field_addition_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_request_dto_out_field_addition({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move || {
                let ctx = Arc::clone(&ctx);
                let app_weak = app_weak.clone();

                std::thread::spawn(move || {
                    if let Some(app) = app_weak.upgrade() {
                        let dto_id = app.global::<FeaturesTabState>().get_selected_dto_out_id();
                        if dto_id < 0 {
                            log::warn!("Cannot add DTO Out field: no DTO Out selected");
                            return;
                        }

                        // Create a new DTO field with default values
                        let create_dto = direct_access::CreateDtoFieldDto {
                            name: "new_field".to_string(),
                            field_type: common::entities::DtoFieldType::String,
                            is_nullable: false,
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
                                    "DTO Out field created successfully with id: {}",
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
                                                "Failed to add field to DTO Out relationship: {}",
                                                e
                                            );
                                        } else {
                                            // Refresh the field list
                                            let _ = slint::invoke_from_event_loop(move || {
                                                if let Some(app) = app_weak.upgrade() {
                                                    fill_dto_out_field_list(&app, &ctx);
                                                }
                                            });
                                        }
                                    }
                                    Err(e) => {
                                        log::error!("Failed to get DTO Out fields relationship: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("Failed to create DTO Out field: {}", e);
                            }
                        }
                    }
                });
            }
        });
}
