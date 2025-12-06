

use std::rc::Rc;
slint::include_modules!();

fn main() {
    let app = App::new().unwrap();

    // Initialize global state
    app.set_is_loading(false);
    app.set_error_message(slint::SharedString::from(""));
    app.set_current_tab(0); // Home

    // Project defaults
    app.set_project_language(slint::SharedString::from("Rust"));
    app.set_project_application_name(slint::SharedString::from(""));
    app.set_project_organisation_name(slint::SharedString::from(""));
    app.set_project_organisation_domain(slint::SharedString::from(""));
    app.set_project_prefix_path(slint::SharedString::from(""));
    app.set_project_is_saving(false);

    // Entities/Features demo lists
    let entities = vec!["User", "Order", "Product"]; 
    let fields = vec!["id", "name", "created_at"]; 
    let features = vec!["Auth", "Catalog", "Checkout"]; 
    let use_cases = vec!["Login", "Search", "Purchase"]; 

    let entity_model = Rc::new(slint::VecModel::from(
        entities.into_iter().map(slint::SharedString::from).collect::<Vec<_>>()
    ));
    let field_model = Rc::new(slint::VecModel::from(
        fields.into_iter().map(slint::SharedString::from).collect::<Vec<_>>()
    ));
    let feature_model = Rc::new(slint::VecModel::from(
        features.into_iter().map(slint::SharedString::from).collect::<Vec<_>>()
    ));
    let use_case_model = Rc::new(slint::VecModel::from(
        use_cases.into_iter().map(slint::SharedString::from).collect::<Vec<_>>()
    ));

    app.set_entity_list(slint::ModelRc::from(entity_model));
    app.set_field_list(slint::ModelRc::from(field_model));
    app.set_feature_list(slint::ModelRc::from(feature_model));
    app.set_use_case_list(slint::ModelRc::from(use_case_model));
    app.set_selected_entity_index(-1);
    app.set_selected_field_index(-1);
    app.set_selected_feature_index(-1);
    app.set_selected_use_case_index(-1);

    // Generate defaults
    app.set_generate_in_temp(true);
    app.set_generate_is_running(false);
    app.set_generate_progress(0.0);
    app.set_generate_message(slint::SharedString::from(""));

    let groups = vec!["Core", "Domain", "Infrastructure"]; 
    let files = vec!["mod.rs", "user.rs", "order.rs"]; 
    let group_model = Rc::new(slint::VecModel::from(
        groups.into_iter().map(slint::SharedString::from).collect::<Vec<_>>()
    ));
    let file_model = Rc::new(slint::VecModel::from(
        files.into_iter().map(slint::SharedString::from).collect::<Vec<_>>()
    ));
    app.set_group_list(slint::ModelRc::from(group_model));
    app.set_file_list(slint::ModelRc::from(file_model));
    app.set_selected_group_index(-1);
    app.set_selected_file_index(-1);

    // Wire up Home callbacks (placeholders for now)
    app.on_new_manifest(|| {
        println!("New Manifest clicked");
    });
    app.on_open_manifest(|| {
        println!("Open Manifest clicked");
    });
    app.on_save_manifest(|| {
        println!("Save Manifest clicked");
    });
    app.on_close_manifest(|| {
        println!("Close Manifest clicked");
    });
    app.on_open_qleany_manifest(|| {
        println!("Open Qleany Manifest clicked");
    });
    app.on_exit_app(|| {
        println!("Exit clicked");
        // In a real app, trigger proper shutdown
    });

    // Project callbacks
    app.on_save_project_settings(|| {
        println!("Save Project Settings clicked");
    });

    // Generate callbacks
    app.on_list_rust_files(|| {
        println!("List Rust Files clicked");
    });
    app.on_start_generate_rust_files(|| {
        println!("Start Generate Rust Files clicked");
    });
    app.on_cancel_generate_rust_files(|| {
        println!("Cancel Generate Rust Files clicked");
    });

    // Entities/Features selection callbacks
    app.on_select_entity(|id| {
        println!("Select Entity: {}", id);
    });
    app.on_select_field(|id| {
        println!("Select Field: {}", id);
    });
    app.on_select_feature(|id| {
        println!("Select Feature: {}", id);
    });
    app.on_select_use_case(|id| {
        println!("Select Use Case: {}", id);
    });

    app.run().unwrap();
}