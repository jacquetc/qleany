//! Features Tab module
//!
//! This module contains the logic specific to the Features tab,
//! including event subscriptions and callback handlers for feature, use case, and DTO management.

use std::sync::Arc;

use slint::ComponentHandle;
use common::direct_access::root::RootRelationshipField;
use common::direct_access::feature::FeatureRelationshipField;
use common::direct_access::use_case::UseCaseRelationshipField;
use common::direct_access::dto::DtoRelationshipField;
use common::entities::DtoFieldType;
use common::event::{DirectAccessEntity, EntityEvent, Origin};
use direct_access::{FeatureRelationshipDto, UseCaseRelationshipDto, DtoRelationshipDto};

use crate::app_context::AppContext;
use crate::commands::{feature_commands, root_commands, use_case_commands, dto_commands, dto_field_commands};
use crate::event_hub_client::EventHubClient;
use crate::{App, FeaturesTabState, AppState, ListItem};

/// Subscribe to Root update events to refresh feature_cr_list
fn subscribe_root_updated_event(event_hub_client: &EventHubClient, app: &App, app_context: &Arc<AppContext>) {
    event_hub_client.subscribe(
        Origin::DirectAccess(DirectAccessEntity::Root(EntityEvent::Updated)),
        {
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |event| {
                log::info!("Root updated event received (features tab): {:?}", event);
                let ctx = Arc::clone(&ctx);
                let app_weak = app_weak.clone();

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_weak.upgrade() {
                        fill_feature_list(&app, &ctx);
                        app.global::<AppState>().set_manifest_is_saved(false);
                    }
                });
            }
        },
    );
}

/// Subscribe to Feature update events to refresh feature_cr_list
fn subscribe_feature_updated_event(event_hub_client: &EventHubClient, app: &App, app_context: &Arc<AppContext>) {
    event_hub_client.subscribe(
        Origin::DirectAccess(DirectAccessEntity::Feature(EntityEvent::Updated)),
        {
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |event| {
                log::info!("Feature updated event received: {:?}", event);
                let ctx = Arc::clone(&ctx);
                let app_weak = app_weak.clone();

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_weak.upgrade() {
                        fill_feature_list(&app, &ctx);
                        fill_use_case_list(&app, &ctx);
                        app.global::<AppState>().set_manifest_is_saved(false);
                    }
                });
            }
        }
    )
}

/// Subscribe to UseCase update events to refresh use_case_cr_list
fn subscribe_use_case_updated_event(event_hub_client: &EventHubClient, app: &App, app_context: &Arc<AppContext>) {
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
                    if let Some(app) = app_weak.upgrade() {
                        fill_use_case_list(&app, &ctx);
                        app.global::<AppState>().set_manifest_is_saved(false);
                    }
                });
            }
        }
    )
}

fn fill_feature_list(app: &App, app_context: &Arc<AppContext>) {
    let ctx = Arc::clone(app_context);
    let app_weak = app.as_weak();

    if let Some(app) = app_weak.upgrade() {
        let root_id = app.global::<AppState>().get_root_id() as common::types::EntityId;

        if root_id > 0 {
            let feature_ids_res = root_commands::get_root_relationship(
                &ctx,
                &root_id,
                &RootRelationshipField::Features,
            );

            match feature_ids_res {
                Ok(feature_ids) => {
                    // empty field list if no features
                    if feature_ids.is_empty() {
                        let model = std::rc::Rc::new(slint::VecModel::from(Vec::<ListItem>::new()));
                        app.global::<FeaturesTabState>().set_feature_cr_list(model.into());
                        log::info!("Feature list cleared (no features)");
                        return;
                    }
                    
                    match feature_commands::get_feature_multi(&ctx, &feature_ids) {
                        Ok(features_opt) => {
                            let mut list: Vec<ListItem> = Vec::new();
                            for maybe_feature in features_opt.into_iter() {
                                if let Some(f) = maybe_feature {
                                    list.push(ListItem {
                                        id: f.id as i32,
                                        text: slint::SharedString::from(f.name),
                                        subtitle: slint::SharedString::from(""),
                                        checked: false,
                                    });
                                }
                            }

                            let model = std::rc::Rc::new(slint::VecModel::from(list));
                            app.global::<FeaturesTabState>().set_feature_cr_list(model.into());
                            log::info!("Feature list refreshed");
                        }
                        Err(e) => {
                            log::error!("Failed to fetch features: {}", e);
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to get root features: {}", e);
                }
            }
        }
    }
}

fn clear_feature_list(app: &App, app_context: &Arc<AppContext>) {
    let ctx = Arc::clone(app_context);
    let app_weak = app.as_weak();

    if let Some(app) = app_weak.upgrade() {
        // Clear feature list
        let model = std::rc::Rc::new(slint::VecModel::from(Vec::<ListItem>::new()));
        app.global::<FeaturesTabState>().set_feature_cr_list(model.into());
        log::info!("Feature list cleared");
    }
}


fn fill_use_case_list(app: &App, app_context: &Arc<AppContext>) {
    let ctx = Arc::clone(app_context);
    let app_weak = app.as_weak();

    if let Some(app) = app_weak.upgrade() {
        let feature_id = app.global::<FeaturesTabState>().get_selected_feature_id() as common::types::EntityId;

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
                        app.global::<FeaturesTabState>().set_use_case_cr_list(model.into());
                        log::info!("Use case list cleared (no use cases)");
                        return;
                    }
                    
                    match use_case_commands::get_use_case_multi(&ctx, &use_case_ids) {
                        Ok(use_cases_opt) => {
                            let mut list: Vec<ListItem> = Vec::new();
                            for maybe_use_case in use_cases_opt.into_iter() {
                                if let Some(uc) = maybe_use_case {
                                    list.push(ListItem {
                                        id: uc.id as i32,
                                        text: slint::SharedString::from(uc.name),
                                        subtitle: slint::SharedString::from(""),
                                        checked: false,
                                    });
                                }
                            }

                            let model = std::rc::Rc::new(slint::VecModel::from(list));
                            app.global::<FeaturesTabState>().set_use_case_cr_list(model.into());
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

fn clear_use_case_list(app: &App, app_context: &Arc<AppContext>) {
    let ctx = Arc::clone(app_context);
    let app_weak = app.as_weak();

    if let Some(app) = app_weak.upgrade() {
        // Clear use case list
        let model = std::rc::Rc::new(slint::VecModel::from(Vec::<ListItem>::new()));
        app.global::<FeaturesTabState>().set_use_case_cr_list(model.into());
        log::info!("Use case list cleared");
    }
}

/// Helper function to fill feature form from FeatureDto
fn fill_feature_form(app: &App, feature: &direct_access::FeatureDto) {
    let state = app.global::<FeaturesTabState>();
    state.set_selected_feature_id(feature.id as i32);
    state.set_selected_feature_name(feature.name.clone().into());
}

/// Helper function to clear feature form
fn clear_feature_form(app: &App) {
    let state = app.global::<FeaturesTabState>();
    state.set_selected_feature_id(-1);
    state.set_selected_feature_name("".into());
}

/// Helper function to fill use case form from UseCaseDto
fn fill_use_case_form(app: &App, use_case: &direct_access::UseCaseDto) {
    let state = app.global::<FeaturesTabState>();
    state.set_selected_use_case_id(use_case.id as i32);
    state.set_selected_use_case_name(use_case.name.clone().into());
    state.set_selected_use_case_validator(use_case.validator);
    state.set_selected_use_case_undoable(use_case.undoable);
    state.set_selected_use_case_read_only(use_case.read_only);
    state.set_selected_use_case_long_operation(use_case.long_operation);
}

/// Helper function to clear use case form
fn clear_use_case_form(app: &App) {
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

// ============================================================================
// DTO Helper Functions
// ============================================================================

/// Convert DtoFieldType to ComboBox index
fn dto_field_type_to_index(field_type: &DtoFieldType) -> i32 {
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
fn index_to_dto_field_type(index: i32) -> DtoFieldType {
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
fn fill_dto_in_form(app: &App, dto: &direct_access::DtoDto) {
    let state = app.global::<FeaturesTabState>();
    state.set_dto_in_enabled(true);
    state.set_selected_dto_in_id(dto.id as i32);
    state.set_selected_dto_in_name(dto.name.clone().into());
}

/// Helper function to clear DTO In form
fn clear_dto_in_form(app: &App) {
    let state = app.global::<FeaturesTabState>();
    state.set_dto_in_enabled(false);
    state.set_selected_dto_in_id(-1);
    state.set_selected_dto_in_name("".into());
    // Clear DTO In fields
    let empty_model: std::rc::Rc<slint::VecModel<ListItem>> = std::rc::Rc::new(slint::VecModel::from(vec![]));
    state.set_dto_in_field_cr_list(empty_model.into());
    clear_dto_in_field_form(app);
}

/// Helper function to fill DTO Out form from DtoDto
fn fill_dto_out_form(app: &App, dto: &direct_access::DtoDto) {
    let state = app.global::<FeaturesTabState>();
    state.set_dto_out_enabled(true);
    state.set_selected_dto_out_id(dto.id as i32);
    state.set_selected_dto_out_name(dto.name.clone().into());
}

/// Helper function to clear DTO Out form
fn clear_dto_out_form(app: &App) {
    let state = app.global::<FeaturesTabState>();
    state.set_dto_out_enabled(false);
    state.set_selected_dto_out_id(-1);
    state.set_selected_dto_out_name("".into());
    // Clear DTO Out fields
    let empty_model: std::rc::Rc<slint::VecModel<ListItem>> = std::rc::Rc::new(slint::VecModel::from(vec![]));
    state.set_dto_out_field_cr_list(empty_model.into());
    clear_dto_out_field_form(app);
}

/// Helper function to fill DTO In field form from DtoFieldDto
fn fill_dto_in_field_form(app: &App, dto_field: &direct_access::DtoFieldDto) {
    let state = app.global::<FeaturesTabState>();
    state.set_selected_dto_in_field_id(dto_field.id as i32);
    state.set_selected_dto_in_field_name(dto_field.name.clone().into());
    state.set_selected_dto_in_field_type_index(dto_field_type_to_index(&dto_field.field_type));
    state.set_selected_dto_in_field_is_nullable(dto_field.is_nullable);
    state.set_selected_dto_in_field_is_list(dto_field.is_list);
}

/// Helper function to clear DTO In field form
fn clear_dto_in_field_form(app: &App) {
    let state = app.global::<FeaturesTabState>();
    state.set_selected_dto_in_field_id(-1);
    state.set_selected_dto_in_field_name("".into());
    state.set_selected_dto_in_field_type_index(4); // Default to String
    state.set_selected_dto_in_field_is_nullable(false);
    state.set_selected_dto_in_field_is_list(false);
}

/// Helper function to fill DTO Out field form from DtoFieldDto
fn fill_dto_out_field_form(app: &App, dto_field: &direct_access::DtoFieldDto) {
    let state = app.global::<FeaturesTabState>();
    state.set_selected_dto_out_field_id(dto_field.id as i32);
    state.set_selected_dto_out_field_name(dto_field.name.clone().into());
    state.set_selected_dto_out_field_type_index(dto_field_type_to_index(&dto_field.field_type));
    state.set_selected_dto_out_field_is_nullable(dto_field.is_nullable);
    state.set_selected_dto_out_field_is_list(dto_field.is_list);
}

/// Helper function to clear DTO Out field form
fn clear_dto_out_field_form(app: &App) {
    let state = app.global::<FeaturesTabState>();
    state.set_selected_dto_out_field_id(-1);
    state.set_selected_dto_out_field_name("".into());
    state.set_selected_dto_out_field_type_index(4); // Default to String
    state.set_selected_dto_out_field_is_nullable(false);
    state.set_selected_dto_out_field_is_list(false);
}

/// Helper function to fill DTO In field list
fn fill_dto_in_field_list(app: &App, app_context: &Arc<AppContext>) {
    let state = app.global::<FeaturesTabState>();
    let dto_id = state.get_selected_dto_in_id() as common::types::EntityId;

    if dto_id <= 0 {
        let empty_model: std::rc::Rc<slint::VecModel<ListItem>> = std::rc::Rc::new(slint::VecModel::from(vec![]));
        state.set_dto_in_field_cr_list(empty_model.into());
        return;
    }

    let field_ids_res = dto_commands::get_dto_relationship(
        app_context,
        &dto_id,
        &DtoRelationshipField::Fields,
    );

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

/// Helper function to fill DTO Out field list
fn fill_dto_out_field_list(app: &App, app_context: &Arc<AppContext>) {
    let state = app.global::<FeaturesTabState>();
    let dto_id = state.get_selected_dto_out_id() as common::types::EntityId;

    if dto_id <= 0 {
        let empty_model: std::rc::Rc<slint::VecModel<ListItem>> = std::rc::Rc::new(slint::VecModel::from(vec![]));
        state.set_dto_out_field_cr_list(empty_model.into());
        return;
    }

    let field_ids_res = dto_commands::get_dto_relationship(
        app_context,
        &dto_id,
        &DtoRelationshipField::Fields,
    );

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

/// Load DTOs for a use case
fn load_use_case_dtos(app: &App, app_context: &Arc<AppContext>, use_case_id: common::types::EntityId) {
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

/// Helper function to update a DTO with new values
fn update_dto_helper<F>(app_context: &Arc<AppContext>, dto_id: i32, update_fn: F)
where
    F: FnOnce(&mut direct_access::DtoDto),
{
    if dto_id < 0 {
        return;
    }

    let dto_res = dto_commands::get_dto(app_context, &(dto_id as common::types::EntityId));

    if let Ok(Some(mut dto)) = dto_res {
        update_fn(&mut dto);
        match dto_commands::update_dto(app_context, &dto) {
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
fn update_dto_field_helper<F>(app_context: &Arc<AppContext>, dto_field_id: i32, update_fn: F)
where
    F: FnOnce(&mut direct_access::DtoFieldDto),
{
    if dto_field_id < 0 {
        return;
    }

    let dto_field_res = dto_field_commands::get_dto_field(app_context, &(dto_field_id as common::types::EntityId));

    if let Ok(Some(mut dto_field)) = dto_field_res {
        update_fn(&mut dto_field);
        match dto_field_commands::update_dto_field(app_context, &dto_field) {
            Ok(_) => {
                log::info!("DTO field updated successfully");
            }
            Err(e) => {
                log::error!("Failed to update DTO field: {}", e);
            }
        }
    }
}

/// Helper function to update a use case with new values
fn update_use_case_helper<F>(app_context: &Arc<AppContext>, use_case_id: i32, update_fn: F)
where
    F: FnOnce(&mut direct_access::UseCaseDto),
{
    if use_case_id < 0 {
        return;
    }

    let use_case_res = use_case_commands::get_use_case(app_context, &(use_case_id as common::types::EntityId));

    if let Ok(Some(mut use_case)) = use_case_res {
        update_fn(&mut use_case);
        match use_case_commands::update_use_case(app_context, &use_case) {
            Ok(_) => {
                log::info!("Use case updated successfully");
            }
            Err(e) => {
                log::error!("Failed to update use case: {}", e);
            }
        }
    }
}

fn setup_features_reorder_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_request_features_reorder({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |from_index, to_index| {
            let from = from_index as usize;
            let to = to_index as usize;

            if let Some(app) = app_weak.upgrade() {
                let root_id = app.global::<AppState>().get_root_id() as common::types::EntityId;
                let feature_ids_res = root_commands::get_root_relationship(
                    &ctx,
                    &root_id,
                    &RootRelationshipField::Features,
                );
                let mut feature_ids = feature_ids_res.unwrap_or_default();

                if from == to || from >= feature_ids.iter().count() {
                    return;
                }

                let moving_feature_id = feature_ids.remove(from);
                let mut insert_at = if to > from { to - 1 } else { to };
                if insert_at > feature_ids.iter().count() { insert_at = feature_ids.iter().count(); }
                feature_ids.insert(insert_at, moving_feature_id);

                let result = root_commands::set_root_relationship(
                    &ctx,
                    &direct_access::RootRelationshipDto {
                        id: root_id,
                        field: RootRelationshipField::Features,
                        right_ids: feature_ids,
                    }
                );

                match result {
                    Ok(()) => {
                        log::info!("Features reordered successfully");
                    }
                    Err(e) => {
                        log::error!("Failed to reorder features: {}", e);
                    }
                }
            }
        }
    });
}

fn setup_use_cases_reorder_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_request_use_cases_reorder({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |from_index, to_index| {
            let from = from_index as usize;
            let to = to_index as usize;

            if let Some(app) = app_weak.upgrade() {
                let feature_id = app.global::<FeaturesTabState>().get_selected_feature_id() as common::types::EntityId;
                let use_case_ids_res = feature_commands::get_feature_relationship(
                    &ctx,
                    &feature_id,
                    &FeatureRelationshipField::UseCases,
                );
                let mut use_case_ids = use_case_ids_res.unwrap_or_default();

                if from == to || from >= use_case_ids.iter().count() {
                    return;
                }

                let moving_use_case_id = use_case_ids.remove(from);
                let mut insert_at = if to > from { to - 1 } else { to };
                if insert_at > use_case_ids.iter().count() { insert_at = use_case_ids.iter().count(); }
                use_case_ids.insert(insert_at, moving_use_case_id);

                let result = feature_commands::set_feature_relationship(
                    &ctx,
                    &FeatureRelationshipDto {
                        id: feature_id,
                        field: FeatureRelationshipField::UseCases,
                        right_ids: use_case_ids,
                    }
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

fn setup_feature_deletion_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_request_feature_deletion({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |feature_id| {
            if let Some(app) = app_weak.upgrade() {
                let result = feature_commands::remove_feature(
                    &ctx,
                    &(feature_id as common::types::EntityId)
                );
                match result {
                    Ok(()) => {
                        log::info!("Feature deleted successfully");
                        // Refresh feature list
                        fill_feature_list(&app, &ctx);
                        // Clear use case list and form
                        clear_use_case_list(&app, &ctx);
                        clear_use_case_form(&app);
                    }
                    Err(e) => {
                        log::error!("Failed to delete feature: {}", e);
                    }
                }
            }
        }
    });
}

fn setup_use_case_deletion_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_request_use_case_deletion({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |use_case_id| {
            if let Some(app) = app_weak.upgrade() {
                let result = use_case_commands::remove_use_case(
                    &ctx,
                    &(use_case_id as common::types::EntityId)
                );
                match result {
                    Ok(()) => {
                        log::info!("Use case deleted successfully");
                        // Refresh use case list
                        fill_use_case_list(&app, &ctx);
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

fn setup_select_feature_callbacks(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_feature_selected({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |selected_feature_id| {
            if let Some(app) = app_weak.upgrade() {
                if selected_feature_id >= 0 {
                    let feature_res = feature_commands::get_feature(
                        &ctx,
                        &(selected_feature_id as common::types::EntityId)
                    );
                    match feature_res {
                        Ok(Some(feature)) => {
                            fill_feature_form(&app, &feature);
                            fill_use_case_list(&app, &ctx);
                            // Clear use case selection when feature changes
                            clear_use_case_form(&app);
                        }
                        _ => {
                            clear_feature_form(&app);
                        }
                    };
                } else {
                    clear_feature_form(&app);
                }
            };
        }
    });
}

fn setup_select_use_case_callbacks(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_use_case_selected({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |selected_use_case_id| {
            if let Some(app) = app_weak.upgrade() {
                if selected_use_case_id >= 0 {
                    let use_case_res = use_case_commands::get_use_case(
                        &ctx,
                        &(selected_use_case_id as common::types::EntityId)
                    );
                    match use_case_res {
                        Ok(Some(use_case)) => {
                            fill_use_case_form(&app, &use_case);
                            // Load DTOs for the selected use case
                            load_use_case_dtos(&app, &ctx, use_case.id);
                        }
                        _ => {
                            clear_use_case_form(&app);
                        }
                    };
                } else {
                    clear_use_case_form(&app);
                }
            };
        }
    });
}

fn setup_feature_name_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_feature_name_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |new_feature_name| {
            if let Some(app) = app_weak.upgrade() {
                if new_feature_name != "" {
                    let current_feature_id = app.global::<FeaturesTabState>().get_selected_feature_id();
                    let feature_res = feature_commands::get_feature(
                        &ctx,
                        &(current_feature_id as common::types::EntityId)
                    );

                    match feature_res {
                        Ok(Some(mut feature)) => {
                            if feature.name == new_feature_name.to_string() {
                                return;
                            }
                            feature.name = new_feature_name.to_string();

                            let result = feature_commands::update_feature(&ctx, &feature);

                            match result {
                                Ok(_) => {
                                    log::info!("Feature name updated successfully");
                                }
                                Err(e) => {
                                    log::error!("Failed to update feature name: {}", e);
                                }
                            }
                        }
                        _ => {}
                    };
                }
            };
        }
    });
}

fn setup_use_case_name_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_use_case_name_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |new_name| {
            if let Some(app) = app_weak.upgrade() {
                let use_case_id = app.global::<FeaturesTabState>().get_selected_use_case_id();
                let name_str = new_name.to_string();
                if !name_str.is_empty() {
                    update_use_case_helper(&ctx, use_case_id, |use_case| {
                        use_case.name = name_str;
                    });
                }
            }
        }
    });
}

fn setup_use_case_validator_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_use_case_validator_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |value| {
            if let Some(app) = app_weak.upgrade() {
                let use_case_id = app.global::<FeaturesTabState>().get_selected_use_case_id();
                update_use_case_helper(&ctx, use_case_id, |use_case| {
                    use_case.validator = value;
                });
            }
        }
    });
}

fn setup_use_case_undoable_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_use_case_undoable_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |value| {
            if let Some(app) = app_weak.upgrade() {
                let use_case_id = app.global::<FeaturesTabState>().get_selected_use_case_id();
                update_use_case_helper(&ctx, use_case_id, |use_case| {
                    use_case.undoable = value;
                });
            }
        }
    });
}

fn setup_use_case_read_only_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_use_case_read_only_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |value| {
            if let Some(app) = app_weak.upgrade() {
                let use_case_id = app.global::<FeaturesTabState>().get_selected_use_case_id();
                update_use_case_helper(&ctx, use_case_id, |use_case| {
                    use_case.read_only = value;
                });
            }
        }
    });
}

fn setup_use_case_long_operation_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_use_case_long_operation_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |value| {
            if let Some(app) = app_weak.upgrade() {
                let use_case_id = app.global::<FeaturesTabState>().get_selected_use_case_id();
                update_use_case_helper(&ctx, use_case_id, |use_case| {
                    use_case.long_operation = value;
                });
            }
        }
    });
}

// ============================================================================
// DTO Callback Setup Functions
// ============================================================================

fn setup_dto_in_enabled_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_dto_in_enabled_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |enabled| {
            if let Some(app) = app_weak.upgrade() {
                let use_case_id = app.global::<FeaturesTabState>().get_selected_use_case_id();
                if use_case_id < 0 {
                    return;
                }

                if enabled {
                    // Create a new DTO In for this use case
                    let create_dto = direct_access::CreateDtoDto {
                        name: "NewDtoIn".to_string(),
                        fields: vec![],
                    };
                    match dto_commands::create_dto(&ctx, &create_dto) {
                        Ok(dto) => {
                            // Set the relationship
                            let relationship_dto = UseCaseRelationshipDto {
                                id: use_case_id as common::types::EntityId,
                                field: UseCaseRelationshipField::DtoIn,
                                right_ids: vec![dto.id],
                            };
                            match use_case_commands::set_use_case_relationship(&ctx, &relationship_dto) {
                                Ok(()) => {
                                    fill_dto_in_form(&app, &dto);
                                    // New DTO has no fields, set empty list explicitly
                                    let empty_model: std::rc::Rc<slint::VecModel<ListItem>> = std::rc::Rc::new(slint::VecModel::from(vec![]));
                                    app.global::<FeaturesTabState>().set_dto_in_field_cr_list(empty_model.into());
                                    clear_dto_in_field_form(&app);
                                    log::info!("DTO In created and linked successfully");
                                }
                                Err(e) => {
                                    log::error!("Failed to link DTO In: {}", e);
                                    // Clean up the created DTO
                                    let _ = dto_commands::remove_dto(&ctx, &dto.id);
                                    app.global::<FeaturesTabState>().set_dto_in_enabled(false);
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to create DTO In: {}", e);
                            app.global::<FeaturesTabState>().set_dto_in_enabled(false);
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
                        match use_case_commands::set_use_case_relationship(&ctx, &relationship_dto) {
                            Ok(()) => {
                                // Delete the DTO
                                let _ = dto_commands::remove_dto(&ctx, &(dto_id as common::types::EntityId));
                                clear_dto_in_form(&app);
                                // Re-set enabled to false since clear_dto_in_form sets it
                                app.global::<FeaturesTabState>().set_dto_in_enabled(false);
                                log::info!("DTO In removed successfully");
                            }
                            Err(e) => {
                                log::error!("Failed to unlink DTO In: {}", e);
                                app.global::<FeaturesTabState>().set_dto_in_enabled(true);
                            }
                        }
                    }
                }
            }
        }
    });
}

fn setup_dto_out_enabled_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_dto_out_enabled_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |enabled| {
            if let Some(app) = app_weak.upgrade() {
                let use_case_id = app.global::<FeaturesTabState>().get_selected_use_case_id();
                if use_case_id < 0 {
                    return;
                }

                if enabled {
                    // Create a new DTO Out for this use case
                    let create_dto = direct_access::CreateDtoDto {
                        name: "NewDtoOut".to_string(),
                        fields: vec![],
                    };
                    match dto_commands::create_dto(&ctx, &create_dto) {
                        Ok(dto) => {
                            // Set the relationship
                            let relationship_dto = UseCaseRelationshipDto {
                                id: use_case_id as common::types::EntityId,
                                field: UseCaseRelationshipField::DtoOut,
                                right_ids: vec![dto.id],
                            };
                            match use_case_commands::set_use_case_relationship(&ctx, &relationship_dto) {
                                Ok(()) => {
                                    fill_dto_out_form(&app, &dto);
                                    // New DTO has no fields, set empty list explicitly
                                    let empty_model: std::rc::Rc<slint::VecModel<ListItem>> = std::rc::Rc::new(slint::VecModel::from(vec![]));
                                    app.global::<FeaturesTabState>().set_dto_out_field_cr_list(empty_model.into());
                                    clear_dto_out_field_form(&app);
                                    log::info!("DTO Out created and linked successfully");
                                }
                                Err(e) => {
                                    log::error!("Failed to link DTO Out: {}", e);
                                    // Clean up the created DTO
                                    let _ = dto_commands::remove_dto(&ctx, &dto.id);
                                    app.global::<FeaturesTabState>().set_dto_out_enabled(false);
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to create DTO Out: {}", e);
                            app.global::<FeaturesTabState>().set_dto_out_enabled(false);
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
                        match use_case_commands::set_use_case_relationship(&ctx, &relationship_dto) {
                            Ok(()) => {
                                // Delete the DTO
                                let _ = dto_commands::remove_dto(&ctx, &(dto_id as common::types::EntityId));
                                clear_dto_out_form(&app);
                                // Re-set enabled to false since clear_dto_out_form sets it
                                app.global::<FeaturesTabState>().set_dto_out_enabled(false);
                                log::info!("DTO Out removed successfully");
                            }
                            Err(e) => {
                                log::error!("Failed to unlink DTO Out: {}", e);
                                app.global::<FeaturesTabState>().set_dto_out_enabled(true);
                            }
                        }
                    }
                }
            }
        }
    });
}

fn setup_dto_in_name_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_dto_in_name_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |new_name| {
            if let Some(app) = app_weak.upgrade() {
                let dto_id = app.global::<FeaturesTabState>().get_selected_dto_in_id();
                let name_str = new_name.to_string();
                if !name_str.is_empty() {
                    update_dto_helper(&ctx, dto_id, |dto| {
                        dto.name = name_str;
                    });
                }
            }
        }
    });
}

fn setup_dto_out_name_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_dto_out_name_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |new_name| {
            if let Some(app) = app_weak.upgrade() {
                let dto_id = app.global::<FeaturesTabState>().get_selected_dto_out_id();
                let name_str = new_name.to_string();
                if !name_str.is_empty() {
                    update_dto_helper(&ctx, dto_id, |dto| {
                        dto.name = name_str;
                    });
                }
            }
        }
    });
}

fn setup_dto_in_field_selected_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_dto_in_field_selected({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |selected_field_id| {
            if let Some(app) = app_weak.upgrade() {
                if selected_field_id >= 0 {
                    let field_res = dto_field_commands::get_dto_field(
                        &ctx,
                        &(selected_field_id as common::types::EntityId)
                    );
                    match field_res {
                        Ok(Some(field)) => {
                            fill_dto_in_field_form(&app, &field);
                        }
                        _ => {
                            clear_dto_in_field_form(&app);
                        }
                    };
                } else {
                    clear_dto_in_field_form(&app);
                }
            };
        }
    });
}

fn setup_dto_out_field_selected_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_dto_out_field_selected({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |selected_field_id| {
            if let Some(app) = app_weak.upgrade() {
                if selected_field_id >= 0 {
                    let field_res = dto_field_commands::get_dto_field(
                        &ctx,
                        &(selected_field_id as common::types::EntityId)
                    );
                    match field_res {
                        Ok(Some(field)) => {
                            fill_dto_out_field_form(&app, &field);
                        }
                        _ => {
                            clear_dto_out_field_form(&app);
                        }
                    };
                } else {
                    clear_dto_out_field_form(&app);
                }
            };
        }
    });
}

fn setup_dto_in_field_name_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_dto_in_field_name_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |new_name| {
            if let Some(app) = app_weak.upgrade() {
                let field_id = app.global::<FeaturesTabState>().get_selected_dto_in_field_id();
                let name_str = new_name.to_string();
                if !name_str.is_empty() {
                    update_dto_field_helper(&ctx, field_id, |field| {
                        field.name = name_str;
                    });
                }
            }
        }
    });
}

fn setup_dto_in_field_type_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_dto_in_field_type_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |_value| {
            if let Some(app) = app_weak.upgrade() {
                let field_id = app.global::<FeaturesTabState>().get_selected_dto_in_field_id();
                let type_index = app.global::<FeaturesTabState>().get_selected_dto_in_field_type_index();
                let field_type = index_to_dto_field_type(type_index);
                update_dto_field_helper(&ctx, field_id, |field| {
                    field.field_type = field_type;
                });
            }
        }
    });
}

fn setup_dto_in_field_is_nullable_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_dto_in_field_is_nullable_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |value| {
            if let Some(app) = app_weak.upgrade() {
                let field_id = app.global::<FeaturesTabState>().get_selected_dto_in_field_id();
                update_dto_field_helper(&ctx, field_id, |field| {
                    field.is_nullable = value;
                });
            }
        }
    });
}

fn setup_dto_in_field_is_list_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_dto_in_field_is_list_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |value| {
            if let Some(app) = app_weak.upgrade() {
                let field_id = app.global::<FeaturesTabState>().get_selected_dto_in_field_id();
                update_dto_field_helper(&ctx, field_id, |field| {
                    field.is_list = value;
                });
            }
        }
    });
}

fn setup_dto_out_field_name_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_dto_out_field_name_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |new_name| {
            if let Some(app) = app_weak.upgrade() {
                let field_id = app.global::<FeaturesTabState>().get_selected_dto_out_field_id();
                let name_str = new_name.to_string();
                if !name_str.is_empty() {
                    update_dto_field_helper(&ctx, field_id, |field| {
                        field.name = name_str;
                    });
                }
            }
        }
    });
}

fn setup_dto_out_field_type_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_dto_out_field_type_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |_value| {
            if let Some(app) = app_weak.upgrade() {
                let field_id = app.global::<FeaturesTabState>().get_selected_dto_out_field_id();
                let type_index = app.global::<FeaturesTabState>().get_selected_dto_out_field_type_index();
                let field_type = index_to_dto_field_type(type_index);
                update_dto_field_helper(&ctx, field_id, |field| {
                    field.field_type = field_type;
                });
            }
        }
    });
}

fn setup_dto_out_field_is_nullable_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_dto_out_field_is_nullable_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |value| {
            if let Some(app) = app_weak.upgrade() {
                let field_id = app.global::<FeaturesTabState>().get_selected_dto_out_field_id();
                update_dto_field_helper(&ctx, field_id, |field| {
                    field.is_nullable = value;
                });
            }
        }
    });
}

fn setup_dto_out_field_is_list_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_dto_out_field_is_list_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |value| {
            if let Some(app) = app_weak.upgrade() {
                let field_id = app.global::<FeaturesTabState>().get_selected_dto_out_field_id();
                update_dto_field_helper(&ctx, field_id, |field| {
                    field.is_list = value;
                });
            }
        }
    });
}

fn setup_dto_in_fields_reorder_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_request_dto_in_fields_reorder({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |from_index, to_index| {
            let from = from_index as usize;
            let to = to_index as usize;

            if let Some(app) = app_weak.upgrade() {
                let dto_id = app.global::<FeaturesTabState>().get_selected_dto_in_id() as common::types::EntityId;
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
                if insert_at > field_ids.len() { insert_at = field_ids.len(); }
                field_ids.insert(insert_at, moving_field_id);

                let result = dto_commands::set_dto_relationship(
                    &ctx,
                    &DtoRelationshipDto {
                        id: dto_id,
                        field: DtoRelationshipField::Fields,
                        right_ids: field_ids,
                    }
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

fn setup_dto_out_fields_reorder_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_request_dto_out_fields_reorder({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |from_index, to_index| {
            let from = from_index as usize;
            let to = to_index as usize;

            if let Some(app) = app_weak.upgrade() {
                let dto_id = app.global::<FeaturesTabState>().get_selected_dto_out_id() as common::types::EntityId;
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
                if insert_at > field_ids.len() { insert_at = field_ids.len(); }
                field_ids.insert(insert_at, moving_field_id);

                let result = dto_commands::set_dto_relationship(
                    &ctx,
                    &DtoRelationshipDto {
                        id: dto_id,
                        field: DtoRelationshipField::Fields,
                        right_ids: field_ids,
                    }
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

fn setup_dto_in_field_deletion_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_request_dto_in_field_deletion({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |field_id| {
            if let Some(app) = app_weak.upgrade() {
                let result = dto_field_commands::remove_dto_field(
                    &ctx,
                    &(field_id as common::types::EntityId)
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

fn setup_dto_out_field_deletion_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_request_dto_out_field_deletion({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |field_id| {
            if let Some(app) = app_weak.upgrade() {
                let result = dto_field_commands::remove_dto_field(
                    &ctx,
                    &(field_id as common::types::EntityId)
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

/// Initialize all features tab related subscriptions and callbacks
pub fn init(event_hub_client: &EventHubClient, app: &App, app_context: &Arc<AppContext>) {
    // Event subscriptions
    subscribe_root_updated_event(event_hub_client, app, app_context);
    subscribe_feature_updated_event(event_hub_client, app, app_context);
    subscribe_use_case_updated_event(event_hub_client, app, app_context);

    // Feature callbacks
    setup_features_reorder_callback(app, app_context);
    setup_select_feature_callbacks(app, app_context);
    setup_feature_name_callback(app, app_context);
    setup_feature_deletion_callback(app, app_context);

    // Use case list callbacks
    setup_use_cases_reorder_callback(app, app_context);
    setup_select_use_case_callbacks(app, app_context);
    setup_use_case_deletion_callback(app, app_context);

    // Use case detail callbacks
    setup_use_case_name_callback(app, app_context);
    setup_use_case_validator_callback(app, app_context);
    setup_use_case_undoable_callback(app, app_context);
    setup_use_case_read_only_callback(app, app_context);
    setup_use_case_long_operation_callback(app, app_context);

    // DTO In callbacks
    setup_dto_in_enabled_callback(app, app_context);
    setup_dto_in_name_callback(app, app_context);
    setup_dto_in_field_selected_callback(app, app_context);
    setup_dto_in_field_name_callback(app, app_context);
    setup_dto_in_field_type_callback(app, app_context);
    setup_dto_in_field_is_nullable_callback(app, app_context);
    setup_dto_in_field_is_list_callback(app, app_context);
    setup_dto_in_fields_reorder_callback(app, app_context);
    setup_dto_in_field_deletion_callback(app, app_context);

    // DTO Out callbacks
    setup_dto_out_enabled_callback(app, app_context);
    setup_dto_out_name_callback(app, app_context);
    setup_dto_out_field_selected_callback(app, app_context);
    setup_dto_out_field_name_callback(app, app_context);
    setup_dto_out_field_type_callback(app, app_context);
    setup_dto_out_field_is_nullable_callback(app, app_context);
    setup_dto_out_field_is_list_callback(app, app_context);
    setup_dto_out_fields_reorder_callback(app, app_context);
    setup_dto_out_field_deletion_callback(app, app_context);
}
