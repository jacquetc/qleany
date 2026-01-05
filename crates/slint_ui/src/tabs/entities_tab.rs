//! Entities Tab module
//!
//! This module contains the logic specific to the Entities tab,
//! including event subscriptions and callback handlers for entity management.

use std::sync::Arc;

use common::direct_access::entity::EntityRelationshipField;
use common::direct_access::root::RootRelationshipField;
use common::event::{DirectAccessEntity, EntityEvent, Origin};
use direct_access::EntityRelationshipDto;
use direct_access::RootRelationshipDto;
use slint::{ComponentHandle, Model};

use crate::app_context::AppContext;
use crate::commands::{entity_commands, field_commands, root_commands};
use crate::event_hub_client::EventHubClient;
use crate::{App, AppState, EntitiesTabState, ListItem};

/// Subscribe to Root update events to refresh entity_cr_list
fn subscribe_root_updated_event(
    event_hub_client: &EventHubClient,
    app: &App,
    app_context: &Arc<AppContext>,
) {
    event_hub_client.subscribe(
        Origin::DirectAccess(DirectAccessEntity::Root(EntityEvent::Updated)),
        {
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |event| {
                log::info!("Root updated event received: {:?}", event);
                let ctx = Arc::clone(&ctx);
                let app_weak = app_weak.clone();

                // Use invoke_from_event_loop to safely update UI from background thread
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_weak.upgrade() {
                        if app.global::<AppState>().get_manifest_is_open() {
                            fill_entity_list(&app, &ctx);
                            app.global::<AppState>().set_manifest_is_saved(false);
                        }
                    }
                });
            }
        },
    );
}

/// Subscribe to Entity update events to refresh entity_cr_list
fn subscribe_entity_updated_event(
    event_hub_client: &EventHubClient,
    app: &App,
    app_context: &Arc<AppContext>,
) {
    event_hub_client.subscribe(
        Origin::DirectAccess(DirectAccessEntity::Entity(EntityEvent::Updated)),
        {
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |event| {
                log::info!("Entity updated event received: {:?}", event);
                let ctx = Arc::clone(&ctx);
                let app_weak = app_weak.clone();

                // Use invoke_from_event_loop to safely update UI from background thread
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_weak.upgrade() {
                        if app.global::<AppState>().get_manifest_is_open() {
                            fill_entity_list(&app, &ctx);
                            app.global::<AppState>().set_manifest_is_saved(false);
                        }
                    }
                });
            }
        },
    )
}

/// Subscribe to Entity update events to refresh entity_cr_list
fn subscribe_field_updated_event(
    event_hub_client: &EventHubClient,
    app: &App,
    app_context: &Arc<AppContext>,
) {
    event_hub_client.subscribe(
        Origin::DirectAccess(DirectAccessEntity::Field(EntityEvent::Updated)),
        {
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |event| {
                log::info!("Field updated event received: {:?}", event);
                let ctx = Arc::clone(&ctx);
                let app_weak = app_weak.clone();

                // Use invoke_from_event_loop to safely update UI from background thread
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_weak.upgrade() {
                        if app.global::<AppState>().get_manifest_is_open() {
                            fill_field_list(&app, &ctx);
                            app.global::<AppState>().set_manifest_is_saved(false);
                        }
                    }
                });
            }
        },
    )
}

fn fill_entity_list(app: &App, app_context: &Arc<AppContext>) {
    let ctx = Arc::clone(app_context);
    let app_weak = app.as_weak();

    if let Some(app) = app_weak.upgrade() {
        let root_id = app.global::<AppState>().get_root_id() as common::types::EntityId;

        // Only refresh if we have a valid root_id
        if root_id > 0 {
            // Get entities attached to the root
            let entity_ids_res = root_commands::get_root_relationship(
                &ctx,
                &root_id,
                &RootRelationshipField::Entities,
            );

            match entity_ids_res {
                Ok(entity_ids) => {
                    // empty entity list if no entities
                    if entity_ids.is_empty() {
                        let model = std::rc::Rc::new(slint::VecModel::from(Vec::<ListItem>::new()));
                        app.global::<EntitiesTabState>()
                            .set_entity_cr_list(model.into());
                        log::info!("Entity list cleared (no entities)");
                        return;
                    }

                    // Fetch entities details to obtain names
                    match entity_commands::get_entity_multi(&ctx, &entity_ids) {
                        Ok(entities_opt) => {
                            // Map to ListItem (id + text)
                            let mut list: Vec<ListItem> = Vec::new();
                            for maybe_entity in entities_opt.into_iter() {
                                if let Some(e) = maybe_entity {
                                    list.push(ListItem {
                                        id: e.id as i32,
                                        text: slint::SharedString::from(e.name),
                                        subtitle: slint::SharedString::from(""),
                                        checked: false,
                                    });
                                }
                            }

                            // Apply to AppState
                            let model = std::rc::Rc::new(slint::VecModel::from(list));
                            app.global::<EntitiesTabState>()
                                .set_entity_cr_list(model.into());
                            log::info!("Entity list refreshed");
                        }
                        Err(e) => {
                            log::error!("Failed to fetch entities: {}", e);
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to get root entities: {}", e);
                }
            }
        }
    }
}

fn clear_entity_list(app: &App, app_context: &Arc<AppContext>) {
    let _ctx = Arc::clone(app_context);
    let app_weak = app.as_weak();

    if let Some(app) = app_weak.upgrade() {
        // Clear entity list
        let model = std::rc::Rc::new(slint::VecModel::from(Vec::<ListItem>::new()));
        app.global::<EntitiesTabState>()
            .set_entity_cr_list(model.into());
        log::info!("Entity list cleared");
    }
}

fn fill_field_list(app: &App, app_context: &Arc<AppContext>) {
    let ctx = Arc::clone(app_context);
    let app_weak = app.as_weak();

    if let Some(app) = app_weak.upgrade() {
        let entity_id =
            app.global::<EntitiesTabState>().get_selected_entity_id() as common::types::EntityId;

        // Only refresh if we have a valid root_id
        if entity_id > 0 {
            // Get entities attached to the root
            let field_ids_res = entity_commands::get_entity_relationship(
                &ctx,
                &entity_id,
                &EntityRelationshipField::Fields,
            );

            match field_ids_res {
                Ok(field_ids) => {
                    // empty field list if no fields
                    if field_ids.is_empty() {
                        let model = std::rc::Rc::new(slint::VecModel::from(Vec::<ListItem>::new()));
                        app.global::<EntitiesTabState>()
                            .set_field_cr_list(model.into());
                        log::info!("Field list cleared (no fields)");
                        return;
                    }

                    // Fetch entities details to obtain names
                    match field_commands::get_field_multi(&ctx, &field_ids) {
                        Ok(fields_opt) => {
                            // Map to ListItem (id + text)
                            let mut list: Vec<ListItem> = Vec::new();
                            for maybe_field in fields_opt.into_iter() {
                                if let Some(e) = maybe_field {
                                    list.push(ListItem {
                                        id: e.id as i32,
                                        text: slint::SharedString::from(e.name),
                                        subtitle: slint::SharedString::from(""),
                                        checked: false,
                                    });
                                }
                            }

                            // Apply to AppState
                            let model = std::rc::Rc::new(slint::VecModel::from(list));
                            app.global::<EntitiesTabState>()
                                .set_field_cr_list(model.into());
                            log::info!("Field list refreshed");
                        }
                        Err(e) => {
                            log::error!("Failed to fetch fields: {}", e);
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to get root fields: {}", e);
                }
            }
        }
    }
}

fn clear_field_list(app: &App, app_context: &Arc<AppContext>) {
    let _ctx = Arc::clone(app_context);
    let app_weak = app.as_weak();

    if let Some(app) = app_weak.upgrade() {
        // Clear field list
        let model = std::rc::Rc::new(slint::VecModel::from(Vec::<ListItem>::new()));
        app.global::<EntitiesTabState>()
            .set_field_cr_list(model.into());
        log::info!("Field list cleared");
    }
}

/// Wire up the on_request_entities_reorder callback on AppState
fn setup_entities_reorder_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<EntitiesTabState>()
        .on_request_entities_reorder({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |from_index, to_index| {
                let from = from_index as usize;
                let to = to_index as usize;

                if let Some(app) = app_weak.upgrade() {
                    // 1) Get entities attached to the root

                    let root_id = app.global::<AppState>().get_root_id() as common::types::EntityId;
                    let entity_ids_res = root_commands::get_root_relationship(
                        &ctx,
                        &root_id,
                        &RootRelationshipField::Entities,
                    );
                    let mut entity_ids = entity_ids_res.unwrap_or_default();

                    if from == to || from >= entity_ids.iter().count() {
                        return;
                    }

                    let moving_entity_id = entity_ids.remove(from);
                    // Adjust target slot when moving downwards because removing shifts indices left
                    let mut insert_at = if to > from { to - 1 } else { to };
                    if insert_at > entity_ids.iter().count() {
                        insert_at = entity_ids.iter().count();
                    }
                    entity_ids.insert(insert_at, moving_entity_id);

                    let result = root_commands::set_root_relationship(
                        &ctx,
                        &RootRelationshipDto {
                            id: root_id,
                            field: RootRelationshipField::Entities,
                            right_ids: entity_ids,
                        },
                    );

                    match result {
                        Ok(()) => {
                            log::info!("Entities reordered successfully");
                        }
                        Err(e) => {
                            log::error!("Failed to reorder entities: {}", e);
                        }
                    }
                }
            }
        });
}

fn setup_fields_reorder_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<EntitiesTabState>().on_request_fields_reorder({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |from_index, to_index| {
            let from = from_index as usize;
            let to = to_index as usize;

            if let Some(app) = app_weak.upgrade() {
                // 1) Get fields attached to the entity

                let entity_id = app.global::<EntitiesTabState>().get_selected_entity_id()
                    as common::types::EntityId;
                let field_ids_res = entity_commands::get_entity_relationship(
                    &ctx,
                    &entity_id,
                    &EntityRelationshipField::Fields,
                );
                let mut field_ids = field_ids_res.unwrap_or_default();

                if from == to || from >= field_ids.iter().count() {
                    return;
                }

                let moving_field_id = field_ids.remove(from);
                // Adjust target slot when moving downwards because removing shifts indices left
                let mut insert_at = if to > from { to - 1 } else { to };
                if insert_at > field_ids.iter().count() {
                    insert_at = field_ids.iter().count();
                }
                field_ids.insert(insert_at, moving_field_id);
                let result = entity_commands::set_entity_relationship(
                    &ctx,
                    &EntityRelationshipDto {
                        id: entity_id,
                        field: EntityRelationshipField::Fields,
                        right_ids: field_ids,
                    },
                );
                match result {
                    Ok(()) => {
                        log::info!("Fields reordered successfully");
                    }
                    Err(e) => {
                        log::error!("Failed to reorder fields: {}", e);
                    }
                }
            }
        }
    });
}

fn setup_field_deletion_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<EntitiesTabState>().on_request_field_deletion({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |field_id| {
            if let Some(app) = app_weak.upgrade() {
                let result =
                    field_commands::remove_field(&ctx, &(field_id as common::types::EntityId));
                match result {
                    Ok(()) => {
                        log::info!("Field deleted successfully");
                        // Refresh field list
                        fill_field_list(&app, &ctx);
                        // Clear field form
                        clear_field_form(&app);
                    }
                    Err(e) => {
                        log::error!("Failed to delete field: {}", e);
                    }
                }
            }
        }
    });
}

fn setup_entity_deletion_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<EntitiesTabState>()
        .on_request_entity_deletion({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |entity_id| {
                if let Some(app) = app_weak.upgrade() {
                    let result = entity_commands::remove_entity(
                        &ctx,
                        &(entity_id as common::types::EntityId),
                    );
                    match result {
                        Ok(()) => {
                            log::info!("Entity deleted successfully");
                            // Refresh entity list
                            fill_entity_list(&app, &ctx);
                            // Clear field list and form
                            clear_field_list(&app, &ctx);
                            clear_field_form(&app);
                        }
                        Err(e) => {
                            log::error!("Failed to delete entity: {}", e);
                        }
                    }
                }
            }
        });
}

fn setup_select_entity_callbacks(app: &App, app_context: &Arc<AppContext>) {
    app.global::<EntitiesTabState>().on_entity_selected({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |selected_entity_id| {
            if let Some(app) = app_weak.upgrade() {
                if selected_entity_id >= 0 {
                    let entity_res = entity_commands::get_entity(
                        &ctx,
                        &(selected_entity_id as common::types::EntityId),
                    );
                    // Update ALL dependent properties here
                    match entity_res {
                        Ok(Some(entity)) => {
                            app.global::<EntitiesTabState>()
                                .set_selected_entity_id(selected_entity_id);
                            app.global::<EntitiesTabState>()
                                .set_selected_entity_name(entity.name.into());
                            fill_field_list(&app, &ctx);
                        }
                        _ => {
                            app.global::<EntitiesTabState>().set_selected_entity_id(-1);
                            app.global::<EntitiesTabState>()
                                .set_selected_entity_name("".into());
                        }
                    };
                }
            };
        }
    });
}

/// Helper function to convert FieldType to string
fn field_type_to_string(field_type: &common::entities::FieldType) -> &'static str {
    match field_type {
        common::entities::FieldType::Boolean => "Boolean",
        common::entities::FieldType::Integer => "Integer",
        common::entities::FieldType::UInteger => "UInteger",
        common::entities::FieldType::Float => "Float",
        common::entities::FieldType::String => "String",
        common::entities::FieldType::Uuid => "Uuid",
        common::entities::FieldType::DateTime => "DateTime",
        common::entities::FieldType::Entity => "Entity",
        common::entities::FieldType::Enum => "Enum",
    }
}

/// Helper function to convert string to FieldType
fn string_to_field_type(s: &str) -> common::entities::FieldType {
    match s {
        "Boolean" => common::entities::FieldType::Boolean,
        "Integer" => common::entities::FieldType::Integer,
        "UInteger" => common::entities::FieldType::UInteger,
        "Float" => common::entities::FieldType::Float,
        "Uuid" => common::entities::FieldType::Uuid,
        "DateTime" => common::entities::FieldType::DateTime,
        "Entity" => common::entities::FieldType::Entity,
        "Enum" => common::entities::FieldType::Enum,
        _ => common::entities::FieldType::String,
    }
}

/// Helper function to convert RelationshipType to string for UI
fn relationship_type_to_string(rel_type: &common::entities::RelationshipType) -> &'static str {
    match rel_type {
        common::entities::RelationshipType::OneToOne => "one_to_one",
        common::entities::RelationshipType::OneToMany => "one_to_many",
        common::entities::RelationshipType::OrderedOneToMany => "ordered_one_to_many",
        common::entities::RelationshipType::ManyToOne => "many_to_one",
        common::entities::RelationshipType::ManyToMany => "many_to_many",
    }
}

/// Helper function to convert string to RelationshipType
fn string_to_relationship_type(s: &str) -> common::entities::RelationshipType {
    match s {
        "one_to_one" => common::entities::RelationshipType::OneToOne,
        "one_to_many" => common::entities::RelationshipType::OneToMany,
        "ordered_one_to_many" => common::entities::RelationshipType::OrderedOneToMany,
        "many_to_one" => common::entities::RelationshipType::ManyToOne,
        "many_to_many" => common::entities::RelationshipType::ManyToMany,
        _ => common::entities::RelationshipType::OneToOne,
    }
}

/// Helper function to fill all field form properties from a FieldDto
fn fill_field_form(app: &App, field: &direct_access::FieldDto) {
    let state = app.global::<EntitiesTabState>();
    state.set_selected_field_id(field.id as i32);
    state.set_selected_field_name(field.name.clone().into());
    state.set_selected_field_type(field_type_to_string(&field.field_type).into());
    state.set_selected_field_entity(field.entity.map(|e| e as i32).unwrap_or(-1));
    state.set_selected_field_is_primary_key(field.is_primary_key);
    state.set_selected_field_relationship(relationship_type_to_string(&field.relationship).into());
    state.set_selected_field_required(field.required);
    state.set_selected_field_single_model(field.single_model);
    state.set_selected_field_strong(field.strong);
    state.set_selected_field_list_model(field.list_model);
    state.set_selected_field_list_model_displayed_field(
        field
            .list_model_displayed_field
            .clone()
            .unwrap_or_default()
            .into(),
    );
    state.set_selected_field_enum_name(field.enum_name.clone().unwrap_or_default().into());
    state.set_selected_field_enum_values(
        field
            .enum_values
            .clone()
            .map(|v| v.join("\n"))
            .unwrap_or_default()
            .into(),
    );
}

/// Helper function to clear field form
fn clear_field_form(app: &App) {
    let state = app.global::<EntitiesTabState>();
    state.set_selected_field_id(-1);
    state.set_selected_field_name("".into());
    state.set_selected_field_type("String".into());
    state.set_selected_field_entity(-1);
    state.set_selected_field_is_primary_key(false);
    state.set_selected_field_relationship("one_to_one".into());
    state.set_selected_field_required(false);
    state.set_selected_field_single_model(true);
    state.set_selected_field_strong(true);
    state.set_selected_field_list_model(false);
    state.set_selected_field_list_model_displayed_field("".into());
    state.set_selected_field_enum_name("".into());
    state.set_selected_field_enum_values("".into());
}

/// Helper function to populate entity options for the Referenced Entity ComboBox
fn fill_entity_options(app: &App, app_context: &Arc<AppContext>) {
    let root_id = app.global::<AppState>().get_root_id() as common::types::EntityId;
    if root_id > 0 {
        let entity_ids_res = root_commands::get_root_relationship(
            app_context,
            &root_id,
            &RootRelationshipField::Entities,
        );

        if let Ok(entity_ids) = entity_ids_res {
            if let Ok(entities_opt) = entity_commands::get_entity_multi(app_context, &entity_ids) {
                let mut names: Vec<slint::SharedString> = Vec::new();
                let mut ids: Vec<i32> = Vec::new();
                for maybe_entity in entities_opt.into_iter() {
                    if let Some(e) = maybe_entity {
                        names.push(e.name.into());
                        ids.push(e.id as i32);
                    }
                }
                let names_model = std::rc::Rc::new(slint::VecModel::from(names));
                let ids_model = std::rc::Rc::new(slint::VecModel::from(ids));
                app.global::<EntitiesTabState>()
                    .set_entity_options(names_model.into());
                app.global::<EntitiesTabState>()
                    .set_entity_option_ids(ids_model.into());
            }
        }
    }
}

/// Helper function to update a field with new values
fn update_field_helper<F>(app_context: &Arc<AppContext>, field_id: i32, update_fn: F)
where
    F: FnOnce(&mut direct_access::FieldDto),
{
    if field_id < 0 {
        return;
    }

    let field_res = field_commands::get_field(app_context, &(field_id as common::types::EntityId));

    if let Ok(Some(mut field)) = field_res {
        update_fn(&mut field);
        match field_commands::update_field(app_context, &field) {
            Ok(_) => {
                log::info!("Field updated successfully");
            }
            Err(e) => {
                log::error!("Failed to update field: {}", e);
            }
        }
    }
}

fn setup_select_field_callbacks(app: &App, app_context: &Arc<AppContext>) {
    app.global::<EntitiesTabState>().on_field_selected({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |selected_field_id| {
            if let Some(app) = app_weak.upgrade() {
                if selected_field_id >= 0 {
                    let field_res = field_commands::get_field(
                        &ctx,
                        &(selected_field_id as common::types::EntityId),
                    );

                    if let Ok(Some(field)) = field_res {
                        fill_field_form(&app, &field);
                        fill_entity_options(&app, &ctx);
                    } else {
                        clear_field_form(&app);
                    }
                } else {
                    clear_field_form(&app);
                }
            }
        }
    })
}

fn setup_field_name_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<EntitiesTabState>().on_field_name_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |new_name| {
            if let Some(app) = app_weak.upgrade() {
                let field_id = app.global::<EntitiesTabState>().get_selected_field_id();
                let name_str = new_name.to_string();
                if !name_str.is_empty() {
                    update_field_helper(&ctx, field_id, |field| {
                        field.name = name_str;
                    });
                }
            }
        }
    });
}

fn setup_field_type_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<EntitiesTabState>().on_field_type_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |new_type| {
            if let Some(app) = app_weak.upgrade() {
                let field_id = app.global::<EntitiesTabState>().get_selected_field_id();
                let type_str = new_type.to_string();
                update_field_helper(&ctx, field_id, |field| {
                    field.field_type = string_to_field_type(&type_str);
                    // Clear entity reference if not Entity type
                    if field.field_type != common::entities::FieldType::Entity {
                        field.entity = None;
                    }
                });
            }
        }
    });
}

fn setup_field_entity_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<EntitiesTabState>().on_field_entity_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |entity_index| {
            if let Some(app) = app_weak.upgrade() {
                let field_id = app.global::<EntitiesTabState>().get_selected_field_id();
                // Get the entity id from the index
                let entity_option_ids = app.global::<EntitiesTabState>().get_entity_option_ids();
                let entity_id = if entity_index >= 0
                    && (entity_index as usize) < entity_option_ids.row_count()
                {
                    Some(
                        entity_option_ids
                            .row_data(entity_index as usize)
                            .unwrap_or(-1) as common::types::EntityId,
                    )
                } else {
                    None
                };
                update_field_helper(&ctx, field_id, |field| {
                    field.entity = entity_id;
                });
            }
        }
    });
}

fn setup_field_relationship_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<EntitiesTabState>()
        .on_field_relationship_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |value| {
                if let Some(app) = app_weak.upgrade() {
                    let field_id = app.global::<EntitiesTabState>().get_selected_field_id();
                    let relationship_type = string_to_relationship_type(value.as_str());
                    update_field_helper(&ctx, field_id, |field| {
                        field.relationship = relationship_type.clone();
                    });
                }
            }
        });
}

fn setup_field_required_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<EntitiesTabState>().on_field_required_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |value| {
            if let Some(app) = app_weak.upgrade() {
                let field_id = app.global::<EntitiesTabState>().get_selected_field_id();
                update_field_helper(&ctx, field_id, |field| {
                    field.required = value;
                });
            }
        }
    });
}

fn setup_field_is_primary_key_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<EntitiesTabState>()
        .on_field_is_primary_key_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |value| {
                if let Some(app) = app_weak.upgrade() {
                    let field_id = app.global::<EntitiesTabState>().get_selected_field_id();
                    update_field_helper(&ctx, field_id, |field| {
                        field.is_primary_key = value;
                    });
                }
            }
        });
}

fn setup_field_strong_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<EntitiesTabState>().on_field_strong_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |value| {
            if let Some(app) = app_weak.upgrade() {
                let field_id = app.global::<EntitiesTabState>().get_selected_field_id();
                update_field_helper(&ctx, field_id, |field| {
                    field.strong = value;
                });
            }
        }
    });
}

fn setup_field_single_model_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<EntitiesTabState>()
        .on_field_single_model_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |value| {
                if let Some(app) = app_weak.upgrade() {
                    let field_id = app.global::<EntitiesTabState>().get_selected_field_id();
                    update_field_helper(&ctx, field_id, |field| {
                        field.single_model = value;
                    });
                }
            }
        });
}

fn setup_field_list_model_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<EntitiesTabState>()
        .on_field_list_model_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |value| {
                if let Some(app) = app_weak.upgrade() {
                    let field_id = app.global::<EntitiesTabState>().get_selected_field_id();
                    update_field_helper(&ctx, field_id, |field| {
                        field.list_model = value;
                    });
                }
            }
        });
}

fn setup_field_list_model_displayed_field_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<EntitiesTabState>()
        .on_field_list_model_displayed_field_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |new_value| {
                if let Some(app) = app_weak.upgrade() {
                    let field_id = app.global::<EntitiesTabState>().get_selected_field_id();
                    let value_str = new_value.to_string();
                    update_field_helper(&ctx, field_id, |field| {
                        field.list_model_displayed_field = if value_str.is_empty() {
                            None
                        } else {
                            Some(value_str)
                        };
                    });
                }
            }
        });
}

fn setup_field_enum_name_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<EntitiesTabState>()
        .on_field_enum_name_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |new_value| {
                if let Some(app) = app_weak.upgrade() {
                    let field_id = app.global::<EntitiesTabState>().get_selected_field_id();
                    let value_str = new_value.to_string();
                    update_field_helper(&ctx, field_id, |field| {
                        field.enum_name = if value_str.is_empty() {
                            None
                        } else {
                            Some(value_str)
                        };
                    });
                }
            }
        });
}

fn setup_field_enum_values_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<EntitiesTabState>()
        .on_field_enum_values_changed({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |new_value| {
                if let Some(app) = app_weak.upgrade() {
                    let field_id = app.global::<EntitiesTabState>().get_selected_field_id();
                    let value_str = new_value.to_string();
                    update_field_helper(&ctx, field_id, |field| {
                        if value_str.is_empty() {
                            field.enum_values = None;
                        } else {
                            // Split by newlines or commas
                            let values: Vec<String> = value_str
                                .split(|c| c == '\n' || c == ',')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect();
                            field.enum_values = if values.is_empty() {
                                None
                            } else {
                                Some(values)
                            };
                        }
                    });
                }
            }
        });
}

fn setup_entity_name_callbacks(app: &App, app_context: &Arc<AppContext>) {
    app.global::<EntitiesTabState>().on_entity_name_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |new_entity_name| {
            if let Some(app) = app_weak.upgrade() {
                if new_entity_name != "" {
                    let current_entity_id =
                        app.global::<EntitiesTabState>().get_selected_entity_id();
                    let entity_res = entity_commands::get_entity(
                        &ctx,
                        &(current_entity_id as common::types::EntityId),
                    );

                    // Update
                    match entity_res {
                        Ok(Some(mut entity)) => {
                            if entity.name == new_entity_name.to_string() {
                                return;
                            }
                            entity.name = new_entity_name.to_string();

                            let result = entity_commands::update_entity(&ctx, &entity);

                            match result {
                                Ok(_) => {
                                    log::info!("Entity name updated successfully");
                                }
                                Err(e) => {
                                    log::error!("Failed to update entity name: {}", e);
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

/// Initialize all entities tab related subscriptions and callbacks
pub fn init(event_hub_client: &EventHubClient, app: &App, app_context: &Arc<AppContext>) {
    // Event subscriptions
    subscribe_root_updated_event(event_hub_client, app, app_context);
    subscribe_entity_updated_event(event_hub_client, app, app_context);
    subscribe_field_updated_event(event_hub_client, app, app_context);

    // Entity callbacks
    setup_entities_reorder_callback(app, app_context);
    setup_select_entity_callbacks(app, app_context);
    setup_entity_name_callbacks(app, app_context);
    setup_entity_deletion_callback(app, app_context);

    // Field list callbacks
    setup_fields_reorder_callback(app, app_context);
    setup_select_field_callbacks(app, app_context);
    setup_field_deletion_callback(app, app_context);

    // Field detail callbacks
    setup_field_name_callback(app, app_context);
    setup_field_type_callback(app, app_context);
    setup_field_entity_callback(app, app_context);
    setup_field_is_primary_key_callback(app, app_context);
    setup_field_relationship_callback(app, app_context);
    setup_field_required_callback(app, app_context);
    setup_field_single_model_callback(app, app_context);
    setup_field_strong_callback(app, app_context);
    setup_field_list_model_callback(app, app_context);
    setup_field_list_model_displayed_field_callback(app, app_context);
    setup_field_enum_name_callback(app, app_context);
    setup_field_enum_values_callback(app, app_context);
}
