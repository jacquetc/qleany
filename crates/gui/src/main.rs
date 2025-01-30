// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod event_hub_client;

use std::{error::Error, rc::Rc};

use common::event::{DirectAccessEntity, EntityEvent, Origin};
use common::{database::db_context::DbContext, event::EventHub};
use direct_access::root_controller;
use slint::ComponentHandle;
slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let db_context = Rc::new(DbContext::new()?);

    let event_hub = Rc::new(EventHub::new());
    let atomic_bool = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    event_hub.start_event_loop(atomic_bool.clone());

    let event_hub_client: event_hub_client::EventHubClient =
        event_hub_client::EventHubClient::new(&event_hub);
    event_hub_client.start();

    let ui = AppWindow::new()?;

    // Slint UI subscribes to a specific origin (e.g., DirectAccess(Entity::Root))
    event_hub_client.subscribe(
        Origin::DirectAccess(DirectAccessEntity::Root(EntityEvent::Created)),
        {
            let ui_handle = ui.as_weak();
            move |event| {
                let ui = ui_handle.upgrade().unwrap();
                ui.set_counter(20);
                println!("Slint UI received event: {:?}", event);
            }
        },
    );

    ui.window().on_close_requested(move || {
        atomic_bool.store(true, std::sync::atomic::Ordering::SeqCst);
        // wait a bit for the event loop to finish
        std::thread::sleep(std::time::Duration::from_millis(100));
        std::process::exit(0);
    });

    ui.on_request_increase_value({
        let db_context = db_context.clone();
        let root = direct_access::CreateRootDto {
            global: 1,
            entities: vec![1],
            features: vec![1],
        };

        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            //ui.set_counter(root.global as i32);
            root_controller::create(&db_context, &event_hub, &root).unwrap();
            let root = root_controller::get(&db_context, &1).unwrap().unwrap();
        }
    });

    ui.run()?;

    Ok(())
}
