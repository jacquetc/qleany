//! Entities Tab module
//!
//! This module contains the logic specific to the Entities tab,
//! including event subscriptions and callback handlers for entity management.

use std::sync::Arc;

use slint::ComponentHandle;
use common::direct_access::root::RootRelationshipField;
use common::direct_access::entity::EntityRelationshipField;
use common::event::{DirectAccessEntity, EntityEvent, Origin};
use direct_access::RootRelationshipDto;
use direct_access::EntityRelationshipDto;

use crate::app_context::AppContext;
use crate::commands::{entity_commands, root_commands, field_commands};
use crate::event_hub_client::EventHubClient;
use crate::{App, EntitiesTabState, AppState, ListItem};

/// Subscribe to Root update events to refresh entity_cr_list
fn subscribe_root_updated_event(event_hub_client: &EventHubClient, app: &App, app_context: &Arc<AppContext>) {
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
                        fill_entity_list(&app, &ctx);
                    }
                });
            }
        },
    );
}

/// Subscribe to Entity update events to refresh entity_cr_list
fn subscribe_entity_updated_event(event_hub_client: &EventHubClient, app: &App, app_context: &Arc<AppContext>) {
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
                        fill_entity_list(&app, &ctx);
                    }
                });

            }
        }
    )
}


/// Subscribe to Entity update events to refresh entity_cr_list
fn subscribe_field_updated_event(event_hub_client: &EventHubClient, app: &App, app_context: &Arc<AppContext>) {
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
                        fill_field_list(&app, &ctx);
                    }
                });
            }
        }
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
                            app.global::<EntitiesTabState>().set_entity_cr_list(model.into());
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

fn fill_field_list(app: &App, app_context: &Arc<AppContext>) {

    let ctx = Arc::clone(app_context);
    let app_weak = app.as_weak();

    if let Some(app) = app_weak.upgrade() {
        let entity_id = app.global::<EntitiesTabState>().get_selected_entity_id() as common::types::EntityId;

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
                            app.global::<EntitiesTabState>().set_field_cr_list(model.into());
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


/// Wire up the on_request_entities_reorder callback on AppState
fn setup_entities_reorder_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<EntitiesTabState>().on_request_entities_reorder({
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
                if insert_at > entity_ids.iter().count() { insert_at = entity_ids.iter().count(); }
                entity_ids.insert(insert_at, moving_entity_id);


                let result = root_commands::set_root_relationship(
                    &ctx,
                    &RootRelationshipDto {
                        id: root_id,
                        field: RootRelationshipField::Entities,
                        right_ids: entity_ids,
                    }
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

                 let entity_id = app.global::<EntitiesTabState>().get_selected_entity_id() as common::types::EntityId;
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
                 if insert_at > field_ids.iter().count() { insert_at = field_ids.iter().count(); }
                 field_ids.insert(insert_at, moving_field_id);
                    let result = entity_commands::set_entity_relationship(
                        &ctx,
                        &EntityRelationshipDto {
                            id: entity_id,
                            field: EntityRelationshipField::Fields,
                            right_ids: field_ids,
                        }
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


fn setup_select_entity_callbacks(app: &App, app_context: &Arc<AppContext>) {
    app.global::<EntitiesTabState>().on_entity_selected({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |selected_entity_id| {
            if let Some(app) = app_weak.upgrade() {
                if selected_entity_id >= 0 {
                    let entity_res = entity_commands::get_entity(
                        &ctx,
                        &(selected_entity_id as common::types::EntityId)
                    );
                    // Update ALL dependent properties here
                    match entity_res {
                        Ok(Some(entity)) => {
                            app.global::<EntitiesTabState>().set_selected_entity_id(selected_entity_id);
                            app.global::<EntitiesTabState>().set_selected_entity_name(entity.name.into());
                            fill_field_list(&app, &ctx);

                        }
                        _ => {
                            app.global::<EntitiesTabState>().set_selected_entity_id(-1);
                            app.global::<EntitiesTabState>().set_selected_entity_name("".into());
                        }
                    };
                }
            };
        }
    });
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
                        &(selected_field_id as common::types::EntityId)
                    );

                    if let Ok(Some(field)) = field_res {
                        app.global::<EntitiesTabState>().set_selected_field_id(field.id as i32);
                        app.global::<EntitiesTabState>().set_selected_field_name(field.name.into());
                    }
                }
            }
        }
    })
}


fn setup_entity_name_callbacks(app: &App, app_context: &Arc<AppContext>) {
    app.global::<EntitiesTabState>().on_entity_name_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |new_entity_name| {
            if let Some(app) = app_weak.upgrade() {
                if new_entity_name != "" {

                    let current_entity_id = app.global::<EntitiesTabState>().get_selected_entity_id();
                    let entity_res = entity_commands::get_entity(
                        &ctx,
                        &(current_entity_id as common::types::EntityId)
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
                        _ => {
                        }
                    };
                }
            };
        }
    });
}

/// Initialize all entities tab related subscriptions and callbacks
pub fn init(event_hub_client: &EventHubClient, app: &App, app_context: &Arc<AppContext>) {
    subscribe_root_updated_event(event_hub_client, app, app_context);
    subscribe_entity_updated_event(event_hub_client, app, app_context);
    subscribe_field_updated_event(event_hub_client, app, app_context);
    setup_entities_reorder_callback(app, app_context);
    setup_select_entity_callbacks(app, app_context);
    setup_entity_name_callbacks(app, app_context);
    setup_fields_reorder_callback(app, app_context);
    setup_select_field_callbacks(app, app_context);
}
