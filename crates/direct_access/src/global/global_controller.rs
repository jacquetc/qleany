use super::{
    dtos::{CreateGlobalDto, GlobalDto},
    units_of_work::{GlobalUnitOfWorkFactory, GlobalUnitOfWorkROFactory},
    use_cases::{
        create_global_multi_uc::CreateGlobalMultiUseCase, create_global_uc::CreateGlobalUseCase,
        get_global_multi_uc::GetGlobalMultiUseCase, get_global_uc::GetGlobalUseCase,
        remove_global_multi_uc::RemoveGlobalMultiUseCase, remove_global_uc::RemoveGlobalUseCase,
        update_global_multi_uc::UpdateGlobalMultiUseCase, update_global_uc::UpdateGlobalUseCase,
    },
};
use anyhow::{Ok, Result};
use common::undo_redo::UndoRedoManager;
use common::{database::db_context::DbContext, event::EventHub, types::EntityId};
use std::sync::Arc;

pub fn create(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    global: &CreateGlobalDto,
) -> Result<GlobalDto> {
    let uow_factory = GlobalUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut global_uc = CreateGlobalUseCase::new(Box::new(uow_factory));
    let result = global_uc.execute(global.clone())?;
    undo_redo_manager.add_command(Box::new(global_uc));
    Ok(result)
}

pub fn get(db_context: &DbContext, id: &EntityId) -> Result<Option<GlobalDto>> {
    let uow_factory = GlobalUnitOfWorkROFactory::new(&db_context);
    let global_uc = GetGlobalUseCase::new(Box::new(uow_factory));
    global_uc.execute(id)
}

pub fn update(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    global: &GlobalDto,
) -> Result<GlobalDto> {
    let uow_factory = GlobalUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut global_uc = UpdateGlobalUseCase::new(Box::new(uow_factory));
    let result = global_uc.execute(global)?;
    undo_redo_manager.add_command(Box::new(global_uc));
    Ok(result)
}

pub fn remove(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    id: &EntityId,
) -> Result<()> {
    // delete global
    let uow_factory = GlobalUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut global_uc = RemoveGlobalUseCase::new(Box::new(uow_factory));
    global_uc.execute(id)?;
    undo_redo_manager.add_command(Box::new(global_uc));
    Ok(())
}

pub fn create_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    globals: &[CreateGlobalDto],
) -> Result<Vec<GlobalDto>> {
    let uow_factory = GlobalUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut global_uc = CreateGlobalMultiUseCase::new(Box::new(uow_factory));
    let result = global_uc.execute(globals)?;
    undo_redo_manager.add_command(Box::new(global_uc));
    Ok(result)
}

pub fn get_multi(db_context: &DbContext, ids: &[EntityId]) -> Result<Vec<Option<GlobalDto>>> {
    let uow_factory = GlobalUnitOfWorkROFactory::new(&db_context);
    let global_uc = GetGlobalMultiUseCase::new(Box::new(uow_factory));
    global_uc.execute(ids)
}

pub fn update_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    globals: &[GlobalDto],
) -> Result<Vec<GlobalDto>> {
    let uow_factory = GlobalUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut global_uc = UpdateGlobalMultiUseCase::new(Box::new(uow_factory));
    let result = global_uc.execute(globals)?;
    undo_redo_manager.add_command(Box::new(global_uc));
    Ok(result)
}

pub fn remove_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    ids: &[EntityId],
) -> Result<()> {
    let uow_factory = GlobalUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut global_uc = RemoveGlobalMultiUseCase::new(Box::new(uow_factory));
    global_uc.execute(ids)?;
    undo_redo_manager.add_command(Box::new(global_uc));
    Ok(())
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use common::database::db_context::DbContext;

    #[test]
    fn test_create_global() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let global = CreateGlobalDto {
            ..Default::default()
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &global);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_global() {
        // get with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let id = 115;
        let result = get(&db_context, &id);
        assert!(result.is_err());

        // create
        let global = CreateGlobalDto {
            language: "test".to_string(),
            ..Default::default()
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &global);
        assert!(result.is_ok());

        // get with valid id
        let id = 1;
        let result = get(&db_context, &id);
        assert!(result.is_ok());
        let global = result.unwrap();
        assert!(global.is_some());
        assert_eq!(global.unwrap().language, "test");
    }

    #[test]
    fn test_update_global() {
        // update with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let global = GlobalDto {
            id: 115,
            ..Default::default()
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = update(&db_context, &event_hub, &mut undo_redo_manager, &global);
        assert!(result.is_err());

        // create
        let global = CreateGlobalDto {
            ..Default::default()
        };
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &global);
        assert!(result.is_ok());

        // update with valid id
        let global = GlobalDto {
            id: 1,
            language: "test".to_string(),
            ..Default::default()
        };
        let result = update(&db_context, &event_hub, &mut undo_redo_manager, &global);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().language, "test");
    }

    #[test]
    fn test_remove_global() {
        // remove with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let id = 115;
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = remove(&db_context, &event_hub, &mut undo_redo_manager, &id);
        assert!(result.is_err());

        // create
        let global = CreateGlobalDto {
            ..Default::default()
        };
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &global);
        assert!(result.is_ok());

        // remove with valid id
        let id = result.unwrap().id;
        let result = remove(&db_context, &event_hub, &mut undo_redo_manager, &id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_global_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let globals = vec![
            CreateGlobalDto {
                ..Default::default()
            },
            CreateGlobalDto {
                ..Default::default()
            },
            CreateGlobalDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &globals);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());

        // set up
        let globals = vec![
            CreateGlobalDto {
                ..Default::default()
            },
            CreateGlobalDto {
                ..Default::default()
            },
            CreateGlobalDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &globals);
        assert!(result.is_ok());

        let ids = vec![1, 2, 3];
        let result = get_multi(&db_context, &ids);
        assert!(result.is_ok());
        let globals = result.unwrap();
        assert_eq!(globals.len(), 3);
    }

    #[test]
    fn test_update_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        // set up
        let globals = vec![
            CreateGlobalDto {
                ..Default::default()
            },
            CreateGlobalDto {
                ..Default::default()
            },
            CreateGlobalDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &globals);
        assert!(result.is_ok());

        // test update_multi
        let globals = vec![
            GlobalDto {
                id: 1,
                ..Default::default()
            },
            GlobalDto {
                id: 2,
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = update_multi(&db_context, &event_hub, &mut undo_redo_manager, &globals);
        assert!(result.is_ok());
    }

    #[test]
    fn test_remove_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());

        // set up
        let globals = vec![
            CreateGlobalDto {
                ..Default::default()
            },
            CreateGlobalDto {
                ..Default::default()
            },
            CreateGlobalDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &globals);
        assert!(result.is_ok());

        // test remove_multi
        let ids = vec![1, 2, 3];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = remove_multi(&db_context, &event_hub, &mut undo_redo_manager, &ids);
        assert!(result.is_ok());
    }
}
