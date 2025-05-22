use super::{
    dtos::{CreateDtoDto, DtoDto},
    units_of_work::{DtoUnitOfWorkFactory, DtoUnitOfWorkROFactory},
    use_cases::{
        create_dto_multi_uc::CreateDtoMultiUseCase, create_dto_uc::CreateDtoUseCase,
        get_dto_multi_uc::GetDtoMultiUseCase, get_dto_uc::GetDtoUseCase,
        remove_dto_multi_uc::RemoveDtoMultiUseCase, remove_dto_uc::RemoveDtoUseCase,
        update_dto_multi_uc::UpdateDtoMultiUseCase, update_dto_uc::UpdateDtoUseCase,
    },
};
use crate::dto::use_cases::get_dto_relationship_uc::GetDtoRelationshipUseCase;
use crate::dto::use_cases::set_dto_relationship_uc::SetDtoRelationshipUseCase;
use crate::DtoRelationshipDto;
use anyhow::{Ok, Result};
use common::direct_access::dto::DtoRelationshipField;
use common::undo_redo::UndoRedoManager;
use common::{database::db_context::DbContext, event::EventHub, types::EntityId};
use std::sync::Arc;

pub fn create(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    dto: &CreateDtoDto,
) -> Result<DtoDto> {
    let uow_factory = DtoUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut dto_uc = CreateDtoUseCase::new(Box::new(uow_factory));
    let result = dto_uc.execute(dto.clone())?;
    undo_redo_manager.add_command(Box::new(dto_uc));
    Ok(result)
}

pub fn get(db_context: &DbContext, id: &EntityId) -> Result<Option<DtoDto>> {
    let uow_factory = DtoUnitOfWorkROFactory::new(&db_context);
    let dto_uc = GetDtoUseCase::new(Box::new(uow_factory));
    dto_uc.execute(id)
}

pub fn update(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    dto: &DtoDto,
) -> Result<DtoDto> {
    let uow_factory = DtoUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut dto_uc = UpdateDtoUseCase::new(Box::new(uow_factory));
    let result = dto_uc.execute(dto)?;
    undo_redo_manager.add_command(Box::new(dto_uc));
    Ok(result)
}

pub fn remove(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    id: &EntityId,
) -> Result<()> {
    // delete dto
    let uow_factory = DtoUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut dto_uc = RemoveDtoUseCase::new(Box::new(uow_factory));
    dto_uc.execute(id)?;
    undo_redo_manager.add_command(Box::new(dto_uc));
    Ok(())
}

pub fn create_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    dtos: &[CreateDtoDto],
) -> Result<Vec<DtoDto>> {
    let uow_factory = DtoUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut dto_uc = CreateDtoMultiUseCase::new(Box::new(uow_factory));
    let result = dto_uc.execute(dtos)?;
    undo_redo_manager.add_command(Box::new(dto_uc));
    Ok(result)
}

pub fn get_multi(db_context: &DbContext, ids: &[EntityId]) -> Result<Vec<Option<DtoDto>>> {
    let uow_factory = DtoUnitOfWorkROFactory::new(&db_context);
    let dto_uc = GetDtoMultiUseCase::new(Box::new(uow_factory));
    dto_uc.execute(ids)
}

pub fn update_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    dtos: &[DtoDto],
) -> Result<Vec<DtoDto>> {
    let uow_factory = DtoUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut dto_uc = UpdateDtoMultiUseCase::new(Box::new(uow_factory));
    let result = dto_uc.execute(dtos)?;
    undo_redo_manager.add_command(Box::new(dto_uc));
    Ok(result)
}

pub fn remove_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    ids: &[EntityId],
) -> Result<()> {
    let uow_factory = DtoUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut dto_uc = RemoveDtoMultiUseCase::new(Box::new(uow_factory));
    dto_uc.execute(ids)?;
    undo_redo_manager.add_command(Box::new(dto_uc));
    Ok(())
}

pub fn get_relationship(
    db_context: &DbContext,
    id: &EntityId,
    field: &DtoRelationshipField,
) -> Result<Vec<EntityId>> {
    let uow_factory = DtoUnitOfWorkROFactory::new(&db_context);
    let dto_uc = GetDtoRelationshipUseCase::new(Box::new(uow_factory));
    dto_uc.execute(id, field)
}

pub fn set_relationship(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    dto: &DtoRelationshipDto,
) -> Result<()> {
    let uow_factory = DtoUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut dto_uc = SetDtoRelationshipUseCase::new(Box::new(uow_factory));
    dto_uc.execute(dto)?;
    undo_redo_manager.add_command(Box::new(dto_uc));
    Ok(())
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use common::database::db_context::DbContext;

    #[test]
    fn test_create_dto() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let dto = CreateDtoDto {
            ..Default::default()
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &dto);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_dto() {
        // get with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let id = 115;
        let result = get(&db_context, &id);
        assert!(result.is_err());

        // create
        let dto = CreateDtoDto {
            name: "test".to_string(),
            ..Default::default()
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &dto);
        assert!(result.is_ok());

        // get with valid id
        let id = 1;
        let result = get(&db_context, &id);
        assert!(result.is_ok());
        let dto = result.unwrap();
        assert!(dto.is_some());
        assert_eq!(dto.unwrap().name, "test");
    }

    #[test]
    fn test_update_dto() {
        // update with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let dto = DtoDto {
            id: 115,
            ..Default::default()
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = update(&db_context, &event_hub, &mut undo_redo_manager, &dto);
        assert!(result.is_err());

        // create
        let dto = CreateDtoDto {
            ..Default::default()
        };
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &dto);
        assert!(result.is_ok());

        // update with valid id
        let dto = DtoDto {
            id: 1,
            name: "test".to_string(),
            ..Default::default()
        };
        let result = update(&db_context, &event_hub, &mut undo_redo_manager, &dto);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "test");
    }

    #[test]
    fn test_remove_dto() {
        // remove with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let id = 115;
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = remove(&db_context, &event_hub, &mut undo_redo_manager, &id);
        assert!(result.is_err());

        // create
        let dto = CreateDtoDto {
            ..Default::default()
        };
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &dto);
        assert!(result.is_ok());

        // remove with valid id
        let id = result.unwrap().id;
        let result = remove(&db_context, &event_hub, &mut undo_redo_manager, &id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_dto_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let dtos = vec![
            CreateDtoDto {
                ..Default::default()
            },
            CreateDtoDto {
                ..Default::default()
            },
            CreateDtoDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &dtos);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());

        // set up
        let dtos = vec![
            CreateDtoDto {
                ..Default::default()
            },
            CreateDtoDto {
                ..Default::default()
            },
            CreateDtoDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &dtos);
        assert!(result.is_ok());

        let ids = vec![1, 2, 3];
        let result = get_multi(&db_context, &ids);
        assert!(result.is_ok());
        let dtos = result.unwrap();
        assert_eq!(dtos.len(), 3);
    }

    #[test]
    fn test_update_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        // set up
        let dtos = vec![
            CreateDtoDto {
                ..Default::default()
            },
            CreateDtoDto {
                ..Default::default()
            },
            CreateDtoDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &dtos);
        assert!(result.is_ok());

        // test update_multi
        let dtos = vec![
            DtoDto {
                id: 1,
                ..Default::default()
            },
            DtoDto {
                id: 2,
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = update_multi(&db_context, &event_hub, &mut undo_redo_manager, &dtos);
        assert!(result.is_ok());
    }

    #[test]
    fn test_remove_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());

        // set up
        let dtos = vec![
            CreateDtoDto {
                ..Default::default()
            },
            CreateDtoDto {
                ..Default::default()
            },
            CreateDtoDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &dtos);
        assert!(result.is_ok());

        // test remove_multi
        let ids = vec![1, 2, 3];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = remove_multi(&db_context, &event_hub, &mut undo_redo_manager, &ids);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_relationship() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());

        // set up
        let dtos = vec![CreateDtoDto {
            fields: vec![1],
            ..Default::default()
        }];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &dtos);
        assert!(result.is_ok());

        let id = 1;
        let field = common::direct_access::dto::DtoRelationshipField::Fields;
        let result = get_relationship(&db_context, &id, &field);
        assert!(result.is_ok());
        let relationships = result.unwrap();
        assert_eq!(relationships.len(), 1);
    }

    #[test]
    fn test_set_relationship() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let undo_redo_manager = &mut UndoRedoManager::new();
        let dto = DtoRelationshipDto {
            id: 1,
            field: common::direct_access::dto::DtoRelationshipField::Fields,
            right_ids: vec![1, 2, 3],
        };
        let result = set_relationship(&db_context, &event_hub, undo_redo_manager, &dto);
        assert!(result.is_ok());
    }
}
