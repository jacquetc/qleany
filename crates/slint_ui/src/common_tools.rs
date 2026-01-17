use crate::app_context::AppContext;
use crate::{App, CommonTools};
use slint::ComponentHandle;
use std::sync::Arc;

/// Wire up the on_check_is_pascal_case callback
pub fn setup_check_is_pascal_case_callback(app: &App) {
    app.global::<CommonTools>().on_check_is_pascal_case({
        move |name| {
            log::info!("Checking if naming is PascalCase");
            heck::AsPascalCase(&name).to_string() == name.to_string()
        }
    });
}

pub fn setup_check_is_snake_case_callback(app: &App) {
    app.global::<CommonTools>().on_check_is_snake_case({
        move |name| {
            log::info!("Checking if naming is snake_case");
            heck::AsSnakeCase(&name).to_string() == name.to_string()
        }
    });
}

pub fn init_developer_mode(app: &App) {
    // check if ../../ has a .git folder
    let binary_path = std::env::current_exe().unwrap();
    let git_path = binary_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(".git");
    let developer_mode = git_path.try_exists().unwrap_or(false);
    log::info!("Developer mode: {}", developer_mode);

    app.global::<CommonTools>()
        .set_developer_mode(developer_mode);
}

pub fn init(app: &App, _app_context: &Arc<AppContext>) {
    setup_check_is_pascal_case_callback(app);
    setup_check_is_snake_case_callback(app);
    init_developer_mode(app);
}
