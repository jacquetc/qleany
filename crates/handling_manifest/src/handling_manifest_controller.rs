use crate::{units_of_work::LoadUnitOfWorkFactory, use_cases::load_uc::LoadUseCase, LoadDto};
use anyhow::Result;
use common::{database::db_context::DbContext, event::EventHub};
use std::rc::Rc;

pub fn load(db_context: &DbContext, event_hub: &Rc<EventHub>, dto: &LoadDto) -> Result<()> {
    let uow_context = LoadUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut laod_uc = LoadUseCase::new(Box::new(uow_context));
    laod_uc.execute(dto)
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use common::database::db_context::DbContext;

    #[test]
    fn test_load_yaml() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Rc::new(EventHub::new());
        let load_dto = LoadDto {
            manifest_path: "../../qleany.yaml".to_string(),
        };
        load(&db_context, &event_hub, &load_dto).unwrap();
    }
}
