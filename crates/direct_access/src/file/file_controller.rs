use super::{
    dtos::{CreateFileDto, FileDto},
    units_of_work::{FileUnitOfWorkFactory, FileUnitOfWorkROFactory},
    use_cases::{
        create_file_multi_uc::CreateFileMultiUseCase, create_file_uc::CreateFileUseCase,
        get_file_multi_uc::GetFileMultiUseCase, get_file_uc::GetFileUseCase,
        remove_file_multi_uc::RemoveFileMultiUseCase, remove_file_uc::RemoveFileUseCase,
        update_file_multi_uc::UpdateFileMultiUseCase, update_file_uc::UpdateFileUseCase,
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
    file: &CreateFileDto,
) -> Result<FileDto> {
    let uow_factory = FileUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut file_uc = CreateFileUseCase::new(Box::new(uow_factory));
    let result = file_uc.execute(file.clone())?;
    undo_redo_manager.add_command(Box::new(file_uc));
    Ok(result)
}

pub fn get(db_context: &DbContext, id: &EntityId) -> Result<Option<FileDto>> {
    let uow_factory = FileUnitOfWorkROFactory::new(&db_context);
    let file_uc = GetFileUseCase::new(Box::new(uow_factory));
    file_uc.execute(id)
}

pub fn update(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    file: &FileDto,
) -> Result<FileDto> {
    let uow_factory = FileUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut file_uc = UpdateFileUseCase::new(Box::new(uow_factory));
    let result = file_uc.execute(file)?;
    undo_redo_manager.add_command(Box::new(file_uc));
    Ok(result)
}

pub fn remove(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    id: &EntityId,
) -> Result<()> {
    // delete file
    let uow_factory = FileUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut file_uc = RemoveFileUseCase::new(Box::new(uow_factory));
    file_uc.execute(id)?;
    undo_redo_manager.add_command(Box::new(file_uc));
    Ok(())
}

pub fn create_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    files: &[CreateFileDto],
) -> Result<Vec<FileDto>> {
    let uow_factory = FileUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut file_uc = CreateFileMultiUseCase::new(Box::new(uow_factory));
    let result = file_uc.execute(files)?;
    undo_redo_manager.add_command(Box::new(file_uc));
    Ok(result)
}

pub fn get_multi(db_context: &DbContext, ids: &[EntityId]) -> Result<Vec<Option<FileDto>>> {
    let uow_factory = FileUnitOfWorkROFactory::new(&db_context);
    let file_uc = GetFileMultiUseCase::new(Box::new(uow_factory));
    file_uc.execute(ids)
}

pub fn update_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    files: &[FileDto],
) -> Result<Vec<FileDto>> {
    let uow_factory = FileUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut file_uc = UpdateFileMultiUseCase::new(Box::new(uow_factory));
    let result = file_uc.execute(files)?;
    undo_redo_manager.add_command(Box::new(file_uc));
    Ok(result)
}

pub fn remove_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    ids: &[EntityId],
) -> Result<()> {
    let uow_factory = FileUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut file_uc = RemoveFileMultiUseCase::new(Box::new(uow_factory));
    file_uc.execute(ids)?;
    undo_redo_manager.add_command(Box::new(file_uc));
    Ok(())
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use common::database::db_context::DbContext;

    #[test]
    fn test_create_file() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let file = CreateFileDto {
            ..Default::default()
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &file);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_file() {
        // get with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let id = 115;
        let result = get(&db_context, &id);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());

        // create
        let file = CreateFileDto {
            name: "test".to_string(),
            ..Default::default()
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &file);
        assert!(result.is_ok());

        // get with valid id
        let id = 1;
        let result = get(&db_context, &id);
        assert!(result.is_ok());
        let file = result.unwrap();
        assert!(file.is_some());
        assert_eq!(file.unwrap().name, "test");
    }

    #[test]
    fn test_update_file() {
        // update with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let file = FileDto {
            id: 115,
            ..Default::default()
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = update(&db_context, &event_hub, &mut undo_redo_manager, &file);
        assert!(result.is_err());

        // create
        let file = CreateFileDto {
            ..Default::default()
        };
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &file);
        assert!(result.is_ok());

        // update with valid id
        let file = FileDto {
            id: 1,
            name: "test".to_string(),
            ..Default::default()
        };
        let result = update(&db_context, &event_hub, &mut undo_redo_manager, &file);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "test");
    }

    #[test]
    fn test_remove_file() {
        // remove with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let id = 115;
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = remove(&db_context, &event_hub, &mut undo_redo_manager, &id);
        assert!(result.is_err());

        // create
        let file = CreateFileDto {
            ..Default::default()
        };
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &file);
        assert!(result.is_ok());

        // remove with valid id
        let id = result.unwrap().id;
        let result = remove(&db_context, &event_hub, &mut undo_redo_manager, &id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_file_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let files = vec![
            CreateFileDto {
                ..Default::default()
            },
            CreateFileDto {
                ..Default::default()
            },
            CreateFileDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &files);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());

        // set up
        let files = vec![
            CreateFileDto {
                ..Default::default()
            },
            CreateFileDto {
                ..Default::default()
            },
            CreateFileDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &files);
        assert!(result.is_ok());

        let ids = vec![1, 2, 3];
        let result = get_multi(&db_context, &ids);
        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 3);
    }

    #[test]
    fn test_update_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        // set up
        let files = vec![
            CreateFileDto {
                ..Default::default()
            },
            CreateFileDto {
                ..Default::default()
            },
            CreateFileDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &files);
        assert!(result.is_ok());

        // test update_multi
        let files = vec![
            FileDto {
                id: 1,
                ..Default::default()
            },
            FileDto {
                id: 2,
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = update_multi(&db_context, &event_hub, &mut undo_redo_manager, &files);
        assert!(result.is_ok());
    }

    #[test]
    fn test_remove_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());

        // set up
        let files = vec![
            CreateFileDto {
                ..Default::default()
            },
            CreateFileDto {
                ..Default::default()
            },
            CreateFileDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &files);
        assert!(result.is_ok());

        // test remove_multi
        let ids = vec![1, 2, 3];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = remove_multi(&db_context, &event_hub, &mut undo_redo_manager, &ids);
        assert!(result.is_ok());
    }

    #[test]
    fn test_remove_file_uc_with_undo_redo() {
        // Setup: Create a new database context and event hub
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let mut undo_redo_manager = UndoRedoManager::new();

        // Step 1: Create an file
        let file = CreateFileDto {
            name: "test file".to_string(),
            ..Default::default()
        };
        let create_result = create(&db_context, &event_hub, &mut undo_redo_manager, &file);
        assert!(create_result.is_ok());
        let created_file = create_result.unwrap();
        let file_id = created_file.id;

        // Verify file exists
        let get_result = get(&db_context, &file_id);
        assert!(get_result.is_ok());
        assert!(get_result.unwrap().is_some());

        // Step 2: Remove the file
        let remove_result = remove(&db_context, &event_hub, &mut undo_redo_manager, &file_id);
        assert!(remove_result.is_ok());

        // Verify file is removed
        let get_result = get(&db_context, &file_id);
        assert!(get_result.is_ok() && get_result.unwrap().is_none());

        // Step 3: Undo the removal
        let undo_result = undo_redo_manager.undo();
        assert!(undo_result.is_ok());

        // Verify file exists again after undo
        let get_result = get(&db_context, &file_id);
        assert!(get_result.is_ok());
        let file_option = get_result.unwrap();
        assert!(file_option.is_some());
        let file = file_option.unwrap();
        assert_eq!(file.id, file_id);
        assert_eq!(file.name, "test file");

        // Step 4: Redo the removal
        let redo_result = undo_redo_manager.redo();
        assert!(redo_result.is_ok());

        // Verify file is removed again after redo
        let get_result = get(&db_context, &file_id);
        assert!(get_result.is_ok() && get_result.unwrap().is_none());
    }
}
