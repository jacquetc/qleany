use super::{
    dtos::{CreateEntityDto, EntityDto},
    units_of_work::{EntityUnitOfWorkFactory, EntityUnitOfWorkROFactory},
    use_cases::{
        create_entity_uc::CreateEntityUseCase, get_entity_uc::GetEntityUseCase,
        remove_entity_uc::RemoveEntityUseCase, update_entity_uc::UpdateEntityUseCase,
    },
};
use anyhow::{Ok, Result};
use common::{database::db_context::DbContext, entities::EntityId, event::EventHub};
use std::rc::Rc;

pub fn create(
    db_context: &DbContext,
    event_hub: &Rc<EventHub>,
    entity: &CreateEntityDto,
) -> Result<EntityDto> {
    let uow_factory = EntityUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut use_case = CreateEntityUseCase::new(Box::new(uow_factory));
    use_case.execute(entity.clone())
}

pub fn get(db_context: &DbContext, id: &EntityId) -> Result<Option<EntityDto>> {
    let uow_factory = EntityUnitOfWorkROFactory::new(&db_context);
    let use_case = GetEntityUseCase::new(Box::new(uow_factory));
    use_case.execute(id)
}

pub fn update(
    db_context: &DbContext,
    event_hub: &Rc<EventHub>,
    entity: &EntityDto,
) -> Result<EntityDto> {
    let uow_factory = EntityUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut use_case = UpdateEntityUseCase::new(Box::new(uow_factory));
    use_case.execute(entity)
}

pub fn remove(db_context: &DbContext, event_hub: &Rc<EventHub>, id: &EntityId) -> Result<()> {
    // delete entity
    let uow_factory = EntityUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut use_case = RemoveEntityUseCase::new(Box::new(uow_factory));
    use_case.execute(id)?;

    Ok(())
}
