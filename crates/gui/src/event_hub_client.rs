use std::{
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
    time::Duration,
};

use common::event::{Event, EventHub, Origin, Queue};
use slint::{Timer, TimerMode};

#[derive(Clone)]
pub struct EventHubClient {
    subscribers: Arc<Mutex<HashMap<Origin, Vec<Box<dyn Fn(Event) + Send>>>>>,
    queue: Queue,
    timer: Rc<Timer>,
}

impl EventHubClient {
    /// Create a new event hub client
    pub fn new(event_hub: &EventHub) -> Self {
        EventHubClient {
            subscribers: Arc::new(Mutex::new(HashMap::new())),
            queue: event_hub.get_queue().clone(),
            timer: Rc::new(Timer::default()),
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

    pub fn start(&self) {
        // Start the timer in repeated mode with a 200ms interval
        self.timer
            .start(TimerMode::Repeated, Duration::from_millis(50), {
                let queue = self.queue.clone();
                let subscribers = Arc::clone(&self.subscribers);

                move || {
                    let _ = slint::run_event_loop();
                    while let Some(event) = queue.lock().unwrap().pop() {
                        print!("EventHubClient received event: {:?}", event);
                        let subs = subscribers.lock().unwrap();
                        if let Some(callbacks) = subs.get(&event.origin) {
                            for callback in callbacks {
                                callback(event.clone());
                            }
                        }
                    }
                }
            });
    }
}
