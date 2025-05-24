use crate::{units_of_work::LoadUnitOfWorkFactory, use_cases::load_uc::LoadUseCase, LoadDto};
use anyhow::Result;
use common::event::{DirectAccessEntity, EntityEvent, Event, HandlingManifestEvent, Origin};
use common::{database::db_context::DbContext, event::EventHub};
use std::sync::Arc;

pub fn load(db_context: &DbContext, event_hub: &Arc<EventHub>, dto: &LoadDto) -> Result<()> {
    let uow_context = LoadUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut load_uc = LoadUseCase::new(Box::new(uow_context));
    load_uc.execute(dto)?;
    // Notify that the handling manifest has been loaded
    event_hub.send_event(Event {
        origin: Origin::HandlingManifest(HandlingManifestEvent::Loaded),
        ids: vec![],
        data: None,
    });
    Ok(())
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use common::database::db_context::DbContext;

    #[test]
    fn test_load_yaml() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let load_dto = LoadDto {
            manifest_path: "../../qleany.yaml".to_string(),
        };
        load(&db_context, &event_hub, &load_dto).unwrap();
    }
}
