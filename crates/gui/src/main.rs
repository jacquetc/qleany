// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, rc::Rc};

use common::database::db_context::DbContext;
use direct_access::root_controller;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {

    let db_context = Rc::new(DbContext::new()?);



    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let db_context = db_context.clone();
        let root = direct_access::CreateRootDto {
            global: 1,
            entities: vec![1],
            features: vec![1],
        };
        root_controller::create(&db_context, &root)?;

        let root = root_controller::get(&db_context, &1)?.unwrap();


        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(root.global as i32);
        }
    


    });

    ui.run()?;

    Ok(())
}