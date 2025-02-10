use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use common::event::{Event, EventHub, Origin, Queue};
use tauri::Emitter;

#[derive(Clone)]
pub struct EventHubClient {
    subscribers: Arc<Mutex<HashMap<Origin, Vec<Box<dyn Fn(Event) + Send>>>>>,
    queue: Queue,
}

impl EventHubClient {
    /// Create a new event hub client
    pub fn new(event_hub: &EventHub) -> Self {
        EventHubClient {
            subscribers: Arc::new(Mutex::new(HashMap::new())),
            queue: event_hub.get_queue().clone(),
        }
    }

    /// Subscribe a callback to an origin
    pub fn subscribe<F>(&self, origin: Origin, callback: F)
    where
        F: Fn(Event) + Send + 'static,
    {
        let mut subs = self.subscribers.lock().unwrap();
        subs.entry(origin)
            .or_insert_with(Vec::new)
            .push(Box::new(callback));
    }

    /// Start

    pub fn start(
        &self,
        app_handle: tauri::AppHandle,
        quit_signal: Arc<std::sync::atomic::AtomicBool>,
    ) {
        // Start the timer in repeated mode with a 200ms interval

        let queue = self.queue.clone();
        let subscribers = Arc::clone(&self.subscribers);
        let quit_signal = Arc::clone(&quit_signal);
        println!("EventHubClient starting event loop");
        tauri::async_runtime::spawn(async move {
            println!("EventHubClient starting event loop async");
            loop {
                while let Some(event) = queue.lock().unwrap().pop() {
                    println!("EventHubClient received event: {:?}", event);
                    let subs = subscribers.lock().unwrap();
                    if let Some(callbacks) = subs.get(&event.origin) {
                        for callback in callbacks {
                            callback(event.clone());
                        }
                    }
                    // Emit Tauri event with origin string
                    app_handle.emit(&event.origin_string(), event).unwrap();
                }
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                if quit_signal.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }
            }
        });

    }
}
