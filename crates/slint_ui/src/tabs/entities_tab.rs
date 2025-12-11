//! Entities Tab module
//!
//! This module contains the logic specific to the Entities tab,
//! including event subscriptions and callback handlers for entity management.

use std::sync::Arc;

use slint::ComponentHandle;
use common::direct_access::root::RootRelationshipField;
use common::event::{DirectAccessEntity, EntityEvent, Origin};
use direct_access::RootRelationshipDto;

use crate::app_context::AppContext;
use crate::commands::{entity_commands, root_commands};
use crate::event_hub_client::EventHubClient;
use crate::{App, AppState, ListItem};

/// Subscribe to Root update events to refresh entity_cr_list
pub fn subscribe_root_updated_event(event_hub_client: &EventHubClient, app: &App, app_context: &Arc<AppContext>) {
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
                                            app.global::<AppState>().set_entity_cr_list(model.into());
                                            log::info!("Entity list refreshed after root update");
                                        }
                                        Err(e) => {
                                            log::error!("Failed to fetch entities after root update: {}", e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    log::error!("Failed to get root entities after root update: {}", e);
                                }
                            }
                        }
                    }
                });
            }
        },
    );
}

/// Wire up the on_request_entities_reorder callback on AppState
pub fn setup_entities_reorder_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<AppState>().on_request_entities_reorder({
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

/// Initialize all entities tab related subscriptions and callbacks
pub fn init(event_hub_client: &EventHubClient, app: &App, app_context: &Arc<AppContext>) {
    subscribe_root_updated_event(event_hub_client, app, app_context);
    setup_entities_reorder_callback(app, app_context);
}
