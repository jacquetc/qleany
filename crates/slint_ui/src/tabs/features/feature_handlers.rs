//! Feature handlers module
//!
//! This module contains functions for feature management including
//! event subscriptions, list operations, form handling, and callbacks.

use std::sync::Arc;

use common::direct_access::feature::FeatureRelationshipField;
use common::direct_access::root::RootRelationshipField;
use common::event::{DirectAccessEntity, EntityEvent, HandlingManifestEvent, Origin};
use slint::{ComponentHandle, Timer};

use crate::app_context::AppContext;
use crate::commands::{feature_commands, root_commands};
use crate::event_hub_client::EventHubClient;
use crate::{App, AppState, FeaturesTabState, ListItem};

use super::use_case_handlers::{clear_use_case_form, clear_use_case_list, fill_use_case_list};

pub fn subscribe_close_manifest_event(
    event_hub_client: &EventHubClient,
    app: &App,
    app_context: &Arc<AppContext>,
) {
    event_hub_client.subscribe(Origin::HandlingManifest(HandlingManifestEvent::Close), {
        let ctx = Arc::clone(&app_context);
        let app_weak = app.as_weak();
        move |event| {
            log::info!("Manifest closed event received: {:?}", event);
            let ctx = Arc::clone(&ctx);
            let app_weak = app_weak.clone();

            // Use invoke_from_event_loop to safely update UI from background thread
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = app_weak.upgrade() {
                    clear_feature_list(&app, &ctx);
                    clear_use_case_list(&app, &ctx);
                }
            });
        }
    });
}

pub fn subscribe_new_manifest_event(
    event_hub_client: &EventHubClient,
    app: &App,
    app_context: &Arc<AppContext>,
) {
    event_hub_client.subscribe(Origin::HandlingManifest(HandlingManifestEvent::New), {
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |_event| {
            log::info!("New manifest created event received");
            let ctx = Arc::clone(&ctx);
            let app_weak = app_weak.clone();

            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = app_weak.upgrade() {
                    if app.global::<AppState>().get_manifest_is_open() {
                        fill_feature_list(&app, &ctx);
                        fill_use_case_list(&app, &ctx);
                    }
                }
            });
        }
    });
}

pub fn subscribe_load_manifest_event(
    event_hub_client: &EventHubClient,
    app: &App,
    app_context: &Arc<AppContext>,
) {
    event_hub_client.subscribe(Origin::HandlingManifest(HandlingManifestEvent::Load), {
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |_event| {
            log::info!("Manifest loaded event received");
            let ctx = Arc::clone(&ctx);
            let app_weak = app_weak.clone();

            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = app_weak.upgrade() {
                    log::info!("Refreshing feature and use case lists after manifest load");
                    if app.global::<AppState>().get_manifest_is_open() {
                        log::info!("Manifest is open, scheduling list refresh");
                        fill_feature_list(&app, &ctx);
                        fill_use_case_list(&app, &ctx);
                        log::info!("Feature and Use Case lists refreshed after manifest load");
                    }
                }
            });
        }
    });
}

/// Subscribe to Root update events to refresh feature_cr_list
pub fn subscribe_root_updated_event(
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
                log::info!("Root updated event received (features tab): {:?}", event);
                let ctx = Arc::clone(&ctx);
                let app_weak = app_weak.clone();

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_weak.upgrade() {
                        if app.global::<AppState>().get_manifest_is_open() {
                            fill_feature_list(&app, &ctx);
                            app.global::<AppState>().set_manifest_is_saved(false);
                        }
                    }
                });
            }
        },
    );
}

/// Subscribe to Feature update events to refresh feature_cr_list
pub fn subscribe_feature_updated_event(
    event_hub_client: &EventHubClient,
    app: &App,
    app_context: &Arc<AppContext>,
) {
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
                        if app.global::<AppState>().get_manifest_is_open() {
                            fill_feature_list(&app, &ctx);
                            fill_use_case_list(&app, &ctx);
                            app.global::<AppState>().set_manifest_is_saved(false);
                        }
                    }
                });
            }
        },
    )
}

pub fn fill_feature_list(app: &App, app_context: &Arc<AppContext>) {
    log::info!("Filling feature list ...");

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
                        app.global::<FeaturesTabState>()
                            .set_feature_cr_list(model.into());
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
                            app.global::<FeaturesTabState>()
                                .set_feature_cr_list(model.into());
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

pub fn clear_feature_list(app: &App, app_context: &Arc<AppContext>) {
    let _ctx = Arc::clone(app_context);
    let app_weak = app.as_weak();

    if let Some(app) = app_weak.upgrade() {
        // Clear feature list
        let model = std::rc::Rc::new(slint::VecModel::from(Vec::<ListItem>::new()));
        app.global::<FeaturesTabState>()
            .set_feature_cr_list(model.into());
        log::info!("Feature list cleared");
    }
}

/// Helper function to fill feature form from FeatureDto
pub fn fill_feature_form(app: &App, feature: &direct_access::FeatureDto) {
    let state = app.global::<FeaturesTabState>();
    state.set_selected_feature_id(feature.id as i32);
    state.set_selected_feature_name(feature.name.clone().into());
}

/// Helper function to clear feature form
pub fn clear_feature_form(app: &App) {
    let state = app.global::<FeaturesTabState>();
    state.set_selected_feature_id(-1);
    state.set_selected_feature_name("".into());
}

pub fn setup_features_reorder_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_request_features_reorder({
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
                    if insert_at > feature_ids.iter().count() {
                        insert_at = feature_ids.iter().count();
                    }
                    feature_ids.insert(insert_at, moving_feature_id);

                    let result = root_commands::set_root_relationship(
                        &ctx,
                        &direct_access::RootRelationshipDto {
                            id: root_id,
                            field: RootRelationshipField::Features,
                            right_ids: feature_ids,
                        },
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

pub fn setup_feature_deletion_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_request_feature_deletion({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move |feature_id| {
                if let Some(app) = app_weak.upgrade() {
                    let result = feature_commands::remove_feature(
                        &ctx,
                        &(feature_id as common::types::EntityId),
                    );
                    match result {
                        Ok(()) => {
                            log::info!("Feature deleted successfully");
                            // Clear feature form
                            clear_feature_form(&app);
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

pub fn setup_select_feature_callbacks(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_feature_selected({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |feature_id| {
            if let Some(app) = app_weak.upgrade() {
                let feature_res =
                    feature_commands::get_feature(&ctx, &(feature_id as common::types::EntityId));
                match feature_res {
                    Ok(Some(feature)) => {
                        fill_feature_form(&app, &feature);
                        fill_use_case_list(&app, &ctx);
                        // Clear use case form when feature changes
                        clear_use_case_form(&app);
                        log::info!("Feature selected: {}", feature.name);
                    }
                    Ok(None) => {
                        log::warn!("Feature not found: {}", feature_id);
                    }
                    Err(e) => {
                        log::error!("Failed to get feature: {}", e);
                    }
                }
            }
        }
    });
}

pub fn setup_feature_name_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>().on_feature_name_changed({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move |name| {
            if let Some(app) = app_weak.upgrade() {
                let feature_id = app.global::<FeaturesTabState>().get_selected_feature_id();
                if feature_id < 0 {
                    return;
                }

                let feature_res =
                    feature_commands::get_feature(&ctx, &(feature_id as common::types::EntityId));

                if let Ok(Some(mut feature)) = feature_res {
                    feature.name = name.to_string();
                    match feature_commands::update_feature(&ctx, &feature) {
                        Ok(_) => {
                            log::info!("Feature name updated successfully");
                        }
                        Err(e) => {
                            log::error!("Failed to update feature name: {}", e);
                        }
                    }
                }
            }
        }
    });
}

pub fn setup_feature_addition_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<FeaturesTabState>()
        .on_request_feature_addition({
            let ctx = Arc::clone(app_context);
            let app_weak = app.as_weak();
            move || {
                if let Some(app) = app_weak.upgrade() {
                    let root_id = app.global::<AppState>().get_root_id();
                    if root_id <= 0 {
                        log::warn!("Cannot add feature: no root loaded");
                        return;
                    }

                    // Create a new feature with default values
                    let create_dto = direct_access::CreateFeatureDto {
                        name: "NewFeature".to_string(),
                        use_cases: vec![],
                    };

                    match feature_commands::create_feature(&ctx, &create_dto) {
                        Ok(new_feature) => {
                            log::info!("Feature created successfully with id: {}", new_feature.id);

                            // Get current feature ids from root
                            let feature_ids_res = root_commands::get_root_relationship(
                                &ctx,
                                &(root_id as common::types::EntityId),
                                &RootRelationshipField::Features,
                            );

                            match feature_ids_res {
                                Ok(mut feature_ids) => {
                                    // Add the new feature id to the list
                                    feature_ids.push(new_feature.id);

                                    // Update the root relationship
                                    let relationship_dto = direct_access::RootRelationshipDto {
                                        id: root_id as common::types::EntityId,
                                        field: RootRelationshipField::Features,
                                        right_ids: feature_ids,
                                    };

                                    if let Err(e) = root_commands::set_root_relationship(
                                        &ctx,
                                        &relationship_dto,
                                    ) {
                                        log::error!(
                                            "Failed to add feature to root relationship: {}",
                                            e
                                        );
                                    }
                                }
                                Err(e) => {
                                    log::error!("Failed to get root features relationship: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to create feature: {}", e);
                        }
                    }
                }
            }
        });
}
