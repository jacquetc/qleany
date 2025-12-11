use common::event::{Event, EventHub, Origin, Queue};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

/// Event callback type for Slint
pub type EventCallback = Box<dyn Fn(Event) + Send>;

/// EventHubClient for Slint - handles event passing from backend to UI
/// Unlike Tauri which uses app_handle.emit(), Slint uses callbacks
#[derive(Clone)]
pub struct EventHubClient {
    subscribers: Arc<Mutex<HashMap<Origin, Vec<EventCallback>>>>,
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

    /// Start the event loop in a background thread
    /// This polls the event queue and dispatches events to subscribers
    pub fn start(&self, quit_signal: Arc<std::sync::atomic::AtomicBool>) {
        let queue = self.queue.clone();
        let subscribers = Arc::clone(&self.subscribers);
        let quit_signal = Arc::clone(&quit_signal);

        log::info!("EventHubClient starting event loop");

        thread::spawn(move || {
            log::info!("EventHubClient event loop started");
            loop {
                // Process all pending events
                while let Some(event) = queue.lock().unwrap().pop() {
                    log::debug!("EventHubClient received event: {:?}", event);
                    let subs = subscribers.lock().unwrap();
                    if let Some(callbacks) = subs.get(&event.origin) {
                        for callback in callbacks {
                            callback(event.clone());
                        }
                    }
                }

                // Sleep briefly to avoid busy-waiting
                thread::sleep(std::time::Duration::from_millis(50));

                // Check for quit signal
                if quit_signal.load(std::sync::atomic::Ordering::Relaxed) {
                    log::info!("EventHubClient quitting event loop");
                    break;
                }
            }
        });
    }
}
