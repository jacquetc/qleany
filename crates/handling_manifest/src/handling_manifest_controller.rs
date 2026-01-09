use crate::LoadDto;
use crate::LoadReturnDto;
use crate::NewReturnDto;
use crate::SaveDto;
use crate::units_of_work::close_uow::CloseUnitOfWorkFactory;
use crate::units_of_work::load_uow::LoadUnitOfWorkFactory;
use crate::units_of_work::new_uow::NewUnitOfWorkFactory;
use crate::units_of_work::save_uow::SaveUnitOfWorkFactory;
use crate::use_cases::close_uc::CloseUseCase;
use crate::use_cases::load_uc::LoadUseCase;
use crate::use_cases::new_uc::NewUseCase;
use crate::use_cases::save_uc::SaveUseCase;
use anyhow::Result;
use common::event::{Event, Origin};

use common::event::HandlingManifestEvent::Close;
use common::event::HandlingManifestEvent::Load;
use common::event::HandlingManifestEvent::New;
use common::event::HandlingManifestEvent::Save;

use common::{database::db_context::DbContext, event::EventHub};
use std::sync::Arc;

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
        origin: Origin::HandlingManifest(Load),
        ids: vec![return_dto.root_id],
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
        origin: Origin::HandlingManifest(Save),
        ids: vec![],
        data: None,
    });
    Ok(return_dto)
}

pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Result<NewReturnDto> {
    let uow_context = NewUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut uc = NewUseCase::new(Box::new(uow_context));
    let return_dto = uc.execute()?;
    // Notify that the handling manifest has been loaded
    event_hub.send_event(Event {
        origin: Origin::HandlingManifest(New),
        ids: vec![return_dto.root_id],
        data: None,
    });
    Ok(return_dto)
}

pub fn close(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Result<()> {
    let uow_context = CloseUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut uc = CloseUseCase::new(Box::new(uow_context));
    let return_dto = uc.execute()?;
    // Notify that the handling manifest has been loaded
    event_hub.send_event(Event {
        origin: Origin::HandlingManifest(Close),
        ids: vec![],
        data: None,
    });
    Ok(return_dto)
}
