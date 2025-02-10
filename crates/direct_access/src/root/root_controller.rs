use super::{
    dtos::{CreateRootDto, RootDto},
    units_of_work::{RootUnitOfWorkFactory, RootUnitOfWorkROFactory},
    use_cases::{
        create_root_multi_uc::CreateRootMultiUseCase, create_root_uc::CreateRootUseCase,
        get_root_multi_uc::GetRootMultiUseCase, get_root_uc::GetRootUseCase,
        remove_root_multi_uc::RemoveRootMultiUseCase, remove_root_uc::RemoveRootUseCase,
        update_root_multi_uc::UpdateRootMultiUseCase, update_root_uc::UpdateRootUseCase,
    },
};
use anyhow::{Ok, Result};
use common::undo_redo::UndoRedoManager;
use common::{database::db_context::DbContext, entities::EntityId, event::EventHub};
use std::sync::Arc;


pub fn create(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    root: &CreateRootDto,
) -> Result<RootDto> {
    let uow_factory = RootUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut root_uc = CreateRootUseCase::new(Box::new(uow_factory));
    let result = root_uc.execute(root.clone())?;
    undo_redo_manager.add_command(Box::new(root_uc));
    Ok(result)
}

pub fn get(db_context: &DbContext, id: &EntityId) -> Result<Option<RootDto>> {
    let uow_factory = RootUnitOfWorkROFactory::new(&db_context);
    let root_uc = GetRootUseCase::new(Box::new(uow_factory));
    root_uc.execute(id)
}

pub fn update(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    root: &RootDto,
) -> Result<RootDto> {
    let uow_factory = RootUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut root_uc = UpdateRootUseCase::new(Box::new(uow_factory));
    let result = root_uc.execute(root)?;
    undo_redo_manager.add_command(Box::new(root_uc));
    Ok(result)
}

pub fn remove(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    id: &EntityId,
) -> Result<()> {
    // delete root
    let uow_factory = RootUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut root_uc = RemoveRootUseCase::new(Box::new(uow_factory));
    root_uc.execute(id)?;
    undo_redo_manager.add_command(Box::new(root_uc));
    Ok(())
}

pub fn create_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    roots: &[CreateRootDto],
) -> Result<Vec<RootDto>> {
    let uow_factory = RootUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut root_uc = CreateRootMultiUseCase::new(Box::new(uow_factory));
    let result = root_uc.execute(roots)?;
    undo_redo_manager.add_command(Box::new(root_uc));
    Ok(result)
}

pub fn get_multi(db_context: &DbContext, ids: &[EntityId]) -> Result<Vec<Option<RootDto>>> {
    let uow_factory = RootUnitOfWorkROFactory::new(&db_context);
    let root_uc = GetRootMultiUseCase::new(Box::new(uow_factory));
    root_uc.execute(ids)
}

pub fn update_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    roots: &[RootDto],
) -> Result<Vec<RootDto>> {
    let uow_factory = RootUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut root_uc = UpdateRootMultiUseCase::new(Box::new(uow_factory));
    let result = root_uc.execute(roots)?;
    undo_redo_manager.add_command(Box::new(root_uc));
    Ok(result)
}

pub fn remove_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    ids: &[EntityId],
) -> Result<()> {
    let uow_factory = RootUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut root_uc = RemoveRootMultiUseCase::new(Box::new(uow_factory));
    root_uc.execute(ids)?;
    undo_redo_manager.add_command(Box::new(root_uc));
    Ok(())
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use common::{database::db_context::DbContext, event};

    #[test]
    fn test_create_root() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let root = CreateRootDto {
            global: 1,
            entities: vec![1],
            features: vec![1],
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &root);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_root() {
        // get with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let id = 115;
        let result = get(&db_context, &id);
        assert!(result.is_err());

        // create
        let root = CreateRootDto {
            global: 1,
            entities: vec![1],
            features: vec![1],
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &root);
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
        let event_hub = Arc::new(EventHub::new());
        let root = RootDto {
            id: 115,
            global: 1,
            entities: vec![1],
            features: vec![1],
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = update(&db_context, &event_hub, &mut undo_redo_manager, &root);
        assert!(result.is_err());

        // create
        let root = CreateRootDto {
            global: 1,
            entities: vec![1],
            features: vec![1],
        };
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &root);
        assert!(result.is_ok());

        // update with valid id
        let root = RootDto {
            id: 1,
            global: 2,
            entities: vec![2],
            features: vec![2],
        };
        let result = update(&db_context, &event_hub, &mut undo_redo_manager, &root);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().global, 2);
    }
}
