use crate::types::EntityId;
use flume::{unbounded, Receiver, Sender};
use serde::Serialize;
use std::{
    sync::{atomic::AtomicBool, Arc, Mutex},
    thread,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub enum HandlingManifestEvent {
    Loaded,
}
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub enum EntityEvent {
    Created,
    Updated,
    Removed,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub enum AllEvent {
    Reset,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub enum Origin {
    DirectAccess(DirectAccessEntity),
    HandlingManifest(HandlingManifestEvent),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
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

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub enum HandlingManifest {
    Load,
}

/// Event struct with metadata
#[derive(Debug, Clone, Hash, PartialEq, Serialize)]
pub struct Event {
    pub origin: Origin,
    pub ids: Vec<EntityId>,
    pub data: Option<String>,
}

impl Event {
    pub fn origin_string(&self) -> String {
        match &self.origin {
            Origin::DirectAccess(entity) => match entity {
                DirectAccessEntity::All(event) => format!("direct_access_all_{:?}", event),
                DirectAccessEntity::Root(event) => format!("direct_access_root_{:?}", event),
                DirectAccessEntity::Entity(event) => format!("direct_access_entity_{:?}", event),
                DirectAccessEntity::Feature(event) => format!("direct_access_feature_{:?}", event),
                DirectAccessEntity::UseCase(event) => format!("direct_access_use_case_{:?}", event),
                DirectAccessEntity::Field(event) => format!("direct_access_field_{:?}", event),
                DirectAccessEntity::DtoField(event) => {
                    format!("direct_access_dto_field_{:?}", event)
                }
                DirectAccessEntity::Relationship(event) => {
                    format!("direct_access_relationship_{:?}", event)
                }
                DirectAccessEntity::Dto(event) => format!("direct_access_dto_{:?}", event),
                DirectAccessEntity::Global(event) => format!("direct_access_global_{:?}", event),
            },
            Origin::HandlingManifest(event) => format!("handling_manifest_{:?}", event),
        }
        .to_lowercase()
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_origin_string_direct_access_all() {
        let event = Event {
            origin: Origin::DirectAccess(DirectAccessEntity::All(AllEvent::Reset)),
            ids: vec![EntityId::default()],
            data: None,
        };
        assert_eq!(event.origin_string(), "direct_access_all_reset");
    }

    #[test]
    fn test_origin_string_direct_access_root() {
        let event = Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Root(EntityEvent::Created)),
            ids: vec![EntityId::default()],
            data: None,
        };
        assert_eq!(event.origin_string(), "direct_access_root_created");
    }

    #[test]
    fn test_origin_string_handling_manifest() {
        let event = Event {
            origin: Origin::HandlingManifest(HandlingManifestEvent::Loaded),
            ids: vec![EntityId::default()],
            data: None,
        };
        assert_eq!(event.origin_string(), "handling_manifest_loaded");
    }

    #[test]
    fn test_event_hub_send_and_receive() {
        let event_hub = EventHub::new();
        let stop_signal = Arc::new(AtomicBool::new(false));
        event_hub.start_event_loop(stop_signal.clone());

        let event = Event {
            origin: Origin::HandlingManifest(HandlingManifestEvent::Loaded),
            ids: vec![EntityId::default()],
            data: Some("test_data".to_string()),
        };

        event_hub.send_event(event.clone());

        thread::sleep(std::time::Duration::from_millis(100));

        let queue = event_hub.get_queue();
        let queue = queue.lock().unwrap();
        assert_eq!(queue.len(), 1);
        assert_eq!(queue[0], event);

        stop_signal.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
