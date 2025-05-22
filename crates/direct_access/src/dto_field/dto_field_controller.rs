use super::{
    dtos::{CreateDtoFieldDto, DtoFieldDto},
    units_of_work::{DtoFieldUnitOfWorkFactory, DtoFieldUnitOfWorkROFactory},
    use_cases::{
        create_dto_field_multi_uc::CreateDtoFieldMultiUseCase,
        create_dto_field_uc::CreateDtoFieldUseCase,
        get_dto_field_multi_uc::GetDtoFieldMultiUseCase, get_dto_field_uc::GetDtoFieldUseCase,
        remove_dto_field_multi_uc::RemoveDtoFieldMultiUseCase,
        remove_dto_field_uc::RemoveDtoFieldUseCase,
        update_dto_field_multi_uc::UpdateDtoFieldMultiUseCase,
        update_dto_field_uc::UpdateDtoFieldUseCase,
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
    dto_field: &CreateDtoFieldDto,
) -> Result<DtoFieldDto> {
    let uow_factory = DtoFieldUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut dto_field_uc = CreateDtoFieldUseCase::new(Box::new(uow_factory));
    let result = dto_field_uc.execute(dto_field.clone())?;
    undo_redo_manager.add_command(Box::new(dto_field_uc));
    Ok(result)
}

pub fn get(db_context: &DbContext, id: &EntityId) -> Result<Option<DtoFieldDto>> {
    let uow_factory = DtoFieldUnitOfWorkROFactory::new(&db_context);
    let dto_field_uc = GetDtoFieldUseCase::new(Box::new(uow_factory));
    dto_field_uc.execute(id)
}

pub fn update(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    dto_field: &DtoFieldDto,
) -> Result<DtoFieldDto> {
    let uow_factory = DtoFieldUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut dto_field_uc = UpdateDtoFieldUseCase::new(Box::new(uow_factory));
    let result = dto_field_uc.execute(dto_field)?;
    undo_redo_manager.add_command(Box::new(dto_field_uc));
    Ok(result)
}

pub fn remove(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    id: &EntityId,
) -> Result<()> {
    // delete dto_field
    let uow_factory = DtoFieldUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut dto_field_uc = RemoveDtoFieldUseCase::new(Box::new(uow_factory));
    dto_field_uc.execute(id)?;
    undo_redo_manager.add_command(Box::new(dto_field_uc));
    Ok(())
}

pub fn create_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    dto_fields: &[CreateDtoFieldDto],
) -> Result<Vec<DtoFieldDto>> {
    let uow_factory = DtoFieldUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut dto_field_uc = CreateDtoFieldMultiUseCase::new(Box::new(uow_factory));
    let result = dto_field_uc.execute(dto_fields)?;
    undo_redo_manager.add_command(Box::new(dto_field_uc));
    Ok(result)
}

pub fn get_multi(db_context: &DbContext, ids: &[EntityId]) -> Result<Vec<Option<DtoFieldDto>>> {
    let uow_factory = DtoFieldUnitOfWorkROFactory::new(&db_context);
    let dto_field_uc = GetDtoFieldMultiUseCase::new(Box::new(uow_factory));
    dto_field_uc.execute(ids)
}

pub fn update_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    dto_fields: &[DtoFieldDto],
) -> Result<Vec<DtoFieldDto>> {
    let uow_factory = DtoFieldUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut dto_field_uc = UpdateDtoFieldMultiUseCase::new(Box::new(uow_factory));
    let result = dto_field_uc.execute(dto_fields)?;
    undo_redo_manager.add_command(Box::new(dto_field_uc));
    Ok(result)
}

pub fn remove_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    ids: &[EntityId],
) -> Result<()> {
    let uow_factory = DtoFieldUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut dto_field_uc = RemoveDtoFieldMultiUseCase::new(Box::new(uow_factory));
    dto_field_uc.execute(ids)?;
    undo_redo_manager.add_command(Box::new(dto_field_uc));
    Ok(())
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use common::database::db_context::DbContext;

    #[test]
    fn test_create_dto_field() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let dto_field = CreateDtoFieldDto {
            ..Default::default()
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &dto_field);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_dto_field() {
        // get with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let id = 115;
        let result = get(&db_context, &id);
        assert!(result.is_err());

        // create
        let dto_field = CreateDtoFieldDto {
            name: "test".to_string(),
            field_type: common::entities::DtoFieldType::String,
            is_nullable: false,
            is_list: false,
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &dto_field);
        assert!(result.is_ok());

        // get with valid id
        let id = 1;
        let result = get(&db_context, &id);
        assert!(result.is_ok());
        let dto_field = result.unwrap();
        assert!(dto_field.is_some());
        assert_eq!(dto_field.unwrap().name, "test");
    }

    #[test]
    fn test_update_dto_field() {
        // update with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let dto_field = DtoFieldDto {
            id: 115,
            ..Default::default()
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = update(&db_context, &event_hub, &mut undo_redo_manager, &dto_field);
        assert!(result.is_err());

        // create
        let dto_field = CreateDtoFieldDto {
            name: "initial".to_string(),
            field_type: common::entities::DtoFieldType::String,
            is_nullable: false,
            is_list: false,
        };
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &dto_field);
        assert!(result.is_ok());

        // update with valid id
        let dto_field = DtoFieldDto {
            id: 1,
            name: "test".to_string(),
            field_type: common::entities::DtoFieldType::String,
            is_nullable: false,
            is_list: false,
        };
        let result = update(&db_context, &event_hub, &mut undo_redo_manager, &dto_field);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "test");
    }

    #[test]
    fn test_remove_dto_field() {
        // remove with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let id = 115;
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = remove(&db_context, &event_hub, &mut undo_redo_manager, &id);
        assert!(result.is_err());

        // create
        let dto_field = CreateDtoFieldDto {
            ..Default::default()
        };
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &dto_field);
        assert!(result.is_ok());

        // remove with valid id
        let id = result.unwrap().id;
        let result = remove(&db_context, &event_hub, &mut undo_redo_manager, &id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_dto_field_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let dto_fields = vec![
            CreateDtoFieldDto {
                ..Default::default()
            },
            CreateDtoFieldDto {
                ..Default::default()
            },
            CreateDtoFieldDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &dto_fields);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());

        // set up
        let dto_fields = vec![
            CreateDtoFieldDto {
                ..Default::default()
            },
            CreateDtoFieldDto {
                ..Default::default()
            },
            CreateDtoFieldDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &dto_fields);
        assert!(result.is_ok());

        let ids = vec![1, 2, 3];
        let result = get_multi(&db_context, &ids);
        assert!(result.is_ok());
        let dto_fields = result.unwrap();
        assert_eq!(dto_fields.len(), 3);
    }

    #[test]
    fn test_update_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        // set up
        let dto_fields = vec![
            CreateDtoFieldDto {
                ..Default::default()
            },
            CreateDtoFieldDto {
                ..Default::default()
            },
            CreateDtoFieldDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &dto_fields);
        assert!(result.is_ok());

        // test update_multi
        let dto_fields = vec![
            DtoFieldDto {
                id: 1,
                ..Default::default()
            },
            DtoFieldDto {
                id: 2,
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = update_multi(&db_context, &event_hub, &mut undo_redo_manager, &dto_fields);
        assert!(result.is_ok());
    }

    #[test]
    fn test_remove_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());

        // set up
        let dto_fields = vec![
            CreateDtoFieldDto {
                ..Default::default()
            },
            CreateDtoFieldDto {
                ..Default::default()
            },
            CreateDtoFieldDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &dto_fields);
        assert!(result.is_ok());

        // test remove_multi
        let ids = vec![1, 2, 3];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = remove_multi(&db_context, &event_hub, &mut undo_redo_manager, &ids);
        assert!(result.is_ok());
    }
}
