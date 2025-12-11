use crate::LoadDto;
use crate::LoadReturnDto;
use crate::SaveDto;
use crate::units_of_work::load_uow::LoadUnitOfWorkFactory;
use crate::units_of_work::save_uow::SaveUnitOfWorkFactory;
use crate::use_cases::load_uc::LoadUseCase;
use crate::use_cases::save_uc::SaveUseCase;
use anyhow::Result;
use common::event::{Event, HandlingManifestEvent, Origin};

use common::event::HandlingManifestEvent::Loaded;
use common::event::HandlingManifestEvent::Saved;

use common::{database::db_context::DbContext, event::EventHub};
use std::sync::Arc;
use crate::units_of_work::close_uow::CloseUnitOfWorkFactory;

pub fn load(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    dto: &LoadDto,
) -> Result<LoadReturnDto> {
    let uow_context = LoadUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut uc = LoadUseCase::new(Box::new(uow_context));
    let return_dto = uc.execute(dto)?;
    // Notify that the handling manifest has been loaded
    event_hub.send_event(Event {
        origin: Origin::HandlingManifest(Loaded),
        ids: vec![],
        data: None,
    });
    Ok(return_dto)
}

pub fn save(db_context: &DbContext, event_hub: &Arc<EventHub>, dto: &SaveDto) -> Result<()> {
    let uow_context = SaveUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut uc = SaveUseCase::new(Box::new(uow_context));
    let return_dto = uc.execute(dto)?;
    // Notify that the handling manifest has been loaded
    event_hub.send_event(Event {
        origin: Origin::HandlingManifest(Saved),
        ids: vec![],
        data: None,
    });
    Ok(return_dto)
}

pub fn close(_db_context: &DbContext, event_hub: &Arc<EventHub>) -> Result<()> {
    let uow_context = CloseUnitOfWorkFactory::new(&_db_context, &event_hub);
    let mut uc = crate::use_cases::close_uc::CloseUseCase::new(Box::new(uow_context));
    uc.execute()?;
    event_hub.send_event(Event {
        origin: Origin::HandlingManifest(HandlingManifestEvent::Closed),
        ids: vec![],
        data: None,
    });
    Ok(())

}