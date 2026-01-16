use crate::{App, CommonTools};
use crate::app_context::AppContext;
use std::sync::Arc;
use slint::ComponentHandle;

/// Wire up the on_check_is_pascal_case callback
pub fn setup_check_is_pascal_case_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<CommonTools>().on_check_is_pascal_case({
        move |name| {
            log::info!("Checking if naming is PascalCase");
            heck::AsPascalCase(&name).to_string() == name.to_string()
        }
    });
}

pub fn setup_check_is_snake_case_callback(app: &App, app_context: &Arc<AppContext>) {
    app.global::<CommonTools>().on_check_is_snake_case({
        move |name| {
            log::info!("Checking if naming is snake_case");
            heck::AsSnakeCase(&name).to_string() == name.to_string()
        }
    });
}

pub fn init(app: &App, app_context: &Arc<AppContext>) {
    setup_check_is_pascal_case_callback(app, app_context);
    setup_check_is_snake_case_callback(app, app_context);
}
