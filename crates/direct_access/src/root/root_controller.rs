use super::{
    dtos::{CreateRootDto, RootDto},
    units_of_work::{RootUnitOfWorkFactory, RootUnitOfWorkROFactory},
    use_cases::{
        create_root_uc::CreateRootUseCase, get_root_uc::GetRootUseCase,
        remove_root_uc::RemoveRootUseCase, update_root_uc::UpdateRootUseCase,
    },
};
use anyhow::{Ok, Result};
use common::{database::db_context::DbContext, entities::EntityId};
//use crate::entity::entity_controller;

pub fn create(db_context: &DbContext, root: &CreateRootDto) -> Result<RootDto> {
    let uow_factory = RootUnitOfWorkFactory::new(&db_context);
    let mut use_case = CreateRootUseCase::new(Box::new(uow_factory));
    use_case.execute(root.clone())
}

pub fn get(db_context: &DbContext, id: &EntityId) -> Result<Option<RootDto>> {
    let uow_factory = RootUnitOfWorkROFactory::new(&db_context);
    let use_case = GetRootUseCase::new(Box::new(uow_factory));
    use_case.execute(id)
}

pub fn update(db_context: &DbContext, root: &RootDto) -> Result<RootDto> {
    let uow_factory = RootUnitOfWorkFactory::new(&db_context);
    let mut use_case = UpdateRootUseCase::new(Box::new(uow_factory));
    use_case.execute(root)
}

pub fn remove(db_context: &DbContext, id: &EntityId) -> Result<()> {
    // delete root
    let uow_factory = RootUnitOfWorkFactory::new(&db_context);
    let mut use_case = RemoveRootUseCase::new(Box::new(uow_factory));
    use_case.execute(id)?;

    Ok(())
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use common::database::db_context::DbContext;

    #[test]
    fn test_create_root() {
        let db_context = DbContext::new().unwrap();
        let root = CreateRootDto {
            global: 1,
            entities: vec![1],
            features: vec![1],
        };
        let result = create(&db_context, &root);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_root() {
        // get with invalid id
        let db_context = DbContext::new().unwrap();
        let id = 115;
        let result = get(&db_context, &id);
        assert!(result.is_err());

        // create
        let root = CreateRootDto {
            global: 1,
            entities: vec![1],
            features: vec![1],
        };
        let result = create(&db_context, &root);
        assert!(result.is_ok());

        // get with valid id
        let id = 1;
        let result = get(&db_context, &id);
        assert!(result.is_ok());
        let root = result.unwrap();
        assert!(root.is_some());
        assert_eq!(root.unwrap().global, 1);
    }

    #[test]
    fn test_update_root() {
        // update with invalid id
        let db_context = DbContext::new().unwrap();
        let root = RootDto {
            id: 115,
            global: 1,
            entities: vec![1],
            features: vec![1],
        };
        let result = update(&db_context, &root);
        assert!(result.is_err());

        // create
        let root = CreateRootDto {
            global: 1,
            entities: vec![1],
            features: vec![1],
        };
        let result = create(&db_context, &root);
        assert!(result.is_ok());

        // update with valid id
        let root = RootDto {
            id: 1,
            global: 2,
            entities: vec![2],
            features: vec![2],
        };
        let result = update(&db_context, &root);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().global, 2);
    }
}
