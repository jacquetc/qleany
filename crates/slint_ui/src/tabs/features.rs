//! Features Tab module
//!
//! This module contains the logic specific to the Features tab,
//! including event subscriptions and callback handlers for feature, use case, and DTO management.
//!
//! The module is organized into submodules:
//! - `feature_handlers`: Feature list and form management
//! - `use_case_handlers`: Use case list, form, and entity management
//! - `dto_in_handlers`: DTO In (input) form and field management
//! - `dto_out_handlers`: DTO Out (output) form and field management

mod dto_in_handlers;
mod dto_out_handlers;
mod feature_handlers;
mod use_case_handlers;

use std::sync::Arc;

use crate::App;
use crate::app_context::AppContext;
use crate::event_hub_client::EventHubClient;

/// Initialize all features tab related subscriptions and callbacks
pub fn init(event_hub_client: &EventHubClient, app: &App, app_context: &Arc<AppContext>) {
    // Event subscriptions
    feature_handlers::subscribe_new_manifest_event(event_hub_client, app, app_context);
    feature_handlers::subscribe_close_manifest_event(event_hub_client, app, app_context);
    feature_handlers::subscribe_load_manifest_event(event_hub_client, app, app_context);
    feature_handlers::subscribe_root_updated_event(event_hub_client, app, app_context);
    feature_handlers::subscribe_feature_updated_event(event_hub_client, app, app_context);
    use_case_handlers::subscribe_use_case_updated_event(event_hub_client, app, app_context);
    dto_in_handlers::subscribe_dto_updated_event(event_hub_client, app, app_context);

    // Feature callbacks
    feature_handlers::setup_features_reorder_callback(app, app_context);
    feature_handlers::setup_select_feature_callbacks(app, app_context);
    feature_handlers::setup_feature_name_callback(app, app_context);
    feature_handlers::setup_feature_deletion_callback(app, app_context);
    feature_handlers::setup_feature_addition_callback(app, app_context);

    // Use case list callbacks
    use_case_handlers::setup_use_cases_reorder_callback(app, app_context);
    use_case_handlers::setup_select_use_case_callbacks(app, app_context);
    use_case_handlers::setup_use_case_deletion_callback(app, app_context);
    use_case_handlers::setup_use_case_addition_callback(app, app_context);

    // Use case detail callbacks
    use_case_handlers::setup_use_case_name_callback(app, app_context);
    use_case_handlers::setup_use_case_validator_callback(app, app_context);
    use_case_handlers::setup_use_case_undoable_callback(app, app_context);
    use_case_handlers::setup_use_case_read_only_callback(app, app_context);
    use_case_handlers::setup_use_case_long_operation_callback(app, app_context);

    // Use case entities callback
    use_case_handlers::setup_use_case_entity_check_callback(app, app_context);

    // DTO In callbacks
    dto_in_handlers::setup_dto_in_enabled_callback(app, app_context);
    dto_in_handlers::setup_dto_in_name_callback(app, app_context);
    dto_in_handlers::setup_dto_in_field_selected_callback(app, app_context);
    dto_in_handlers::setup_dto_in_field_name_callback(app, app_context);
    dto_in_handlers::setup_dto_in_field_type_callback(app, app_context);
    dto_in_handlers::setup_dto_in_field_is_nullable_callback(app, app_context);
    dto_in_handlers::setup_dto_in_field_is_list_callback(app, app_context);
    dto_in_handlers::setup_dto_in_fields_reorder_callback(app, app_context);
    dto_in_handlers::setup_dto_in_field_deletion_callback(app, app_context);
    dto_in_handlers::setup_dto_in_field_addition_callback(app, app_context);

    // DTO Out callbacks
    dto_out_handlers::setup_dto_out_enabled_callback(app, app_context);
    dto_out_handlers::setup_dto_out_name_callback(app, app_context);
    dto_out_handlers::setup_dto_out_field_selected_callback(app, app_context);
    dto_out_handlers::setup_dto_out_field_name_callback(app, app_context);
    dto_out_handlers::setup_dto_out_field_type_callback(app, app_context);
    dto_out_handlers::setup_dto_out_field_is_nullable_callback(app, app_context);
    dto_out_handlers::setup_dto_out_field_is_list_callback(app, app_context);
    dto_out_handlers::setup_dto_out_fields_reorder_callback(app, app_context);
    dto_out_handlers::setup_dto_out_field_deletion_callback(app, app_context);
    dto_out_handlers::setup_dto_out_field_addition_callback(app, app_context);
}
