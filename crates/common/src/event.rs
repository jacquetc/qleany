use std::{
    collections::HashMap,
    sync::{atomic::AtomicBool, Arc, Mutex},
    thread,
};

use flume::{unbounded, Receiver, Sender};

use crate::entities::EntityId;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum HandlingManifestEvent {
    Loaded,
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum EntityEvent {
    Created,
    Updated,
    Removed,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum AllEvent {
    Reset,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Origin {
    DirectAccess(DirectAccessEntity),
    HandlingManifest(HandlingManifestEvent),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum DirectAccessEntity {
    All(AllEvent),
    Root(EntityEvent),
    Entity(EntityEvent),
    Feature(EntityEvent),
    UseCase(EntityEvent),
    Field(EntityEvent),
    DtoField(EntityEvent),
    Relationship(EntityEvent),
    Dto(EntityEvent),
    Global(EntityEvent),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum HandlingManifest {
    Load,
}

/// Event struct with metadata
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Event {
    pub origin: Origin,
    pub ids: Vec<EntityId>,
    pub data: Option<String>,
}

pub type Queue = Arc<Mutex<Vec<Event>>>;

/// Central event hub for managing subscriptions and dispatching events
pub struct EventHub {
    sender: Sender<Event>,
    receiver: Receiver<Event>,
    queue: Queue,
}

impl EventHub {
    /// Create a new event hub
    pub fn new() -> Self {
        let (sender, receiver) = unbounded();
        EventHub {
            sender,
            receiver,
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Start the event processing loop
    pub fn start_event_loop(&self, stop_signal: Arc<AtomicBool>) {
        let receiver = self.receiver.clone();
        let queue = self.queue.clone();
        thread::spawn(move || loop {
            if stop_signal.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }

            let event = receiver.recv().unwrap();
            let mut queue = queue.lock().unwrap();
            queue.push(event.clone());
        });
    }

    /// Send an event to the queue
    pub fn send_event(&self, event: Event) {
        self.sender.send(event).unwrap();
    }

    pub fn get_queue(&self) -> Queue {
        self.queue.clone()
    }
}
