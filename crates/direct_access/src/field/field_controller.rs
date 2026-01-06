use super::{
    dtos::{CreateFieldDto, FieldDto},
    units_of_work::{FieldUnitOfWorkFactory, FieldUnitOfWorkROFactory},
    use_cases::{
        create_field_multi_uc::CreateFieldMultiUseCase, create_field_uc::CreateFieldUseCase,
        get_field_multi_uc::GetFieldMultiUseCase, get_field_uc::GetFieldUseCase,
        remove_field_multi_uc::RemoveFieldMultiUseCase, remove_field_uc::RemoveFieldUseCase,
        update_field_multi_uc::UpdateFieldMultiUseCase, update_field_uc::UpdateFieldUseCase,
    },
};
use crate::FieldRelationshipDto;
use crate::field::use_cases::get_field_relationship_uc::GetFieldRelationshipUseCase;
use crate::field::use_cases::set_field_relationship_uc::SetFieldRelationshipUseCase;
use anyhow::{Ok, Result};
use common::direct_access::field::FieldRelationshipField;
use common::undo_redo::UndoRedoManager;
use common::{database::db_context::DbContext, event::EventHub, types::EntityId};
use std::sync::Arc;

pub fn create(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    field: &CreateFieldDto,
) -> Result<FieldDto> {
    let uow_factory = FieldUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut field_uc = CreateFieldUseCase::new(Box::new(uow_factory));
    let result = field_uc.execute(field.clone())?;
    undo_redo_manager.add_command(Box::new(field_uc));
    Ok(result)
}

pub fn get(db_context: &DbContext, id: &EntityId) -> Result<Option<FieldDto>> {
    let uow_factory = FieldUnitOfWorkROFactory::new(&db_context);
    let field_uc = GetFieldUseCase::new(Box::new(uow_factory));
    field_uc.execute(id)
}

pub fn update(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    field: &FieldDto,
) -> Result<FieldDto> {
    let uow_factory = FieldUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut field_uc = UpdateFieldUseCase::new(Box::new(uow_factory));
    let result = field_uc.execute(field)?;
    undo_redo_manager.add_command(Box::new(field_uc));
    Ok(result)
}

pub fn remove(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    id: &EntityId,
) -> Result<()> {
    // delete field
    let uow_factory = FieldUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut field_uc = RemoveFieldUseCase::new(Box::new(uow_factory));
    field_uc.execute(id)?;
    undo_redo_manager.add_command(Box::new(field_uc));
    Ok(())
}

pub fn create_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    fields: &[CreateFieldDto],
) -> Result<Vec<FieldDto>> {
    let uow_factory = FieldUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut field_uc = CreateFieldMultiUseCase::new(Box::new(uow_factory));
    let result = field_uc.execute(fields)?;
    undo_redo_manager.add_command(Box::new(field_uc));
    Ok(result)
}

pub fn get_multi(db_context: &DbContext, ids: &[EntityId]) -> Result<Vec<Option<FieldDto>>> {
    let uow_factory = FieldUnitOfWorkROFactory::new(&db_context);
    let field_uc = GetFieldMultiUseCase::new(Box::new(uow_factory));
    field_uc.execute(ids)
}

pub fn update_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    fields: &[FieldDto],
) -> Result<Vec<FieldDto>> {
    let uow_factory = FieldUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut field_uc = UpdateFieldMultiUseCase::new(Box::new(uow_factory));
    let result = field_uc.execute(fields)?;
    undo_redo_manager.add_command(Box::new(field_uc));
    Ok(result)
}

pub fn remove_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    ids: &[EntityId],
) -> Result<()> {
    let uow_factory = FieldUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut field_uc = RemoveFieldMultiUseCase::new(Box::new(uow_factory));
    field_uc.execute(ids)?;
    undo_redo_manager.add_command(Box::new(field_uc));
    Ok(())
}

pub fn get_relationship(
    db_context: &DbContext,
    id: &EntityId,
    field: &FieldRelationshipField,
) -> Result<Vec<EntityId>> {
    let uow_factory = FieldUnitOfWorkROFactory::new(&db_context);
    let field_uc = GetFieldRelationshipUseCase::new(Box::new(uow_factory));
    field_uc.execute(id, field)
}

pub fn set_relationship(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    dto: &FieldRelationshipDto,
) -> Result<()> {
    let uow_factory = FieldUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut field_uc = SetFieldRelationshipUseCase::new(Box::new(uow_factory));
    field_uc.execute(dto)?;
    undo_redo_manager.add_command(Box::new(field_uc));
    Ok(())
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use common::database::db_context::DbContext;

    #[test]
    fn test_create_field() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let field = CreateFieldDto {
            ..Default::default()
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &field);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_field() {
        // get with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let id = 115;
        let result = get(&db_context, &id);
        assert!(result.is_err());

        // create
        let field = CreateFieldDto {
            name: "TestField".to_string(),
            field_type: common::entities::FieldType::String,
            ..Default::default()
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &field);
        assert!(result.is_ok());

        // get with valid id
        let id = 1;
        let result = get(&db_context, &id);
        assert!(result.is_ok());
        let field = result.unwrap();
        assert!(field.is_some());
        assert_eq!(field.unwrap().name, "TestField");
    }

    #[test]
    fn test_update_field() {
        // update with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let field = FieldDto {
            id: 115,
            ..Default::default()
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = update(&db_context, &event_hub, &mut undo_redo_manager, &field);
        assert!(result.is_err());

        // create
        let field = CreateFieldDto {
            ..Default::default()
        };
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &field);
        assert!(result.is_ok());

        // update with valid id
        let field = FieldDto {
            id: 1,
            name: "test".to_string(),
            ..Default::default()
        };
        let result = update(&db_context, &event_hub, &mut undo_redo_manager, &field);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "test");
    }

    #[test]
    fn test_remove_field() {
        // remove with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let id = 115;
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = remove(&db_context, &event_hub, &mut undo_redo_manager, &id);
        assert!(result.is_err());

        // create
        let field = CreateFieldDto {
            ..Default::default()
        };
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &field);
        assert!(result.is_ok());

        // remove with valid id
        let id = result.unwrap().id;
        let result = remove(&db_context, &event_hub, &mut undo_redo_manager, &id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_field_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let fields = vec![
            CreateFieldDto {
                ..Default::default()
            },
            CreateFieldDto {
                ..Default::default()
            },
            CreateFieldDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &fields);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());

        // set up
        let fields = vec![
            CreateFieldDto {
                ..Default::default()
            },
            CreateFieldDto {
                ..Default::default()
            },
            CreateFieldDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &fields);
        assert!(result.is_ok());

        let ids = vec![1, 2, 3];
        let result = get_multi(&db_context, &ids);
        assert!(result.is_ok());
        let fields = result.unwrap();
        assert_eq!(fields.len(), 3);
    }

    #[test]
    fn test_update_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        // set up
        let fields = vec![
            CreateFieldDto {
                ..Default::default()
            },
            CreateFieldDto {
                ..Default::default()
            },
            CreateFieldDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &fields);
        assert!(result.is_ok());

        // test update_multi
        let fields = vec![
            FieldDto {
                id: 1,
                ..Default::default()
            },
            FieldDto {
                id: 2,
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = update_multi(&db_context, &event_hub, &mut undo_redo_manager, &fields);
        assert!(result.is_ok());
    }

    #[test]
    fn test_remove_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());

        // set up
        let fields = vec![
            CreateFieldDto {
                ..Default::default()
            },
            CreateFieldDto {
                ..Default::default()
            },
            CreateFieldDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &fields);
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
        let fields = vec![CreateFieldDto {
            entity: Some(1),
            ..Default::default()
        }];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &fields);
        assert!(result.is_ok());

        let id = 1;
        let field = common::direct_access::field::FieldRelationshipField::Entity;
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
        let dto = FieldRelationshipDto {
            id: 1,
            field: common::direct_access::field::FieldRelationshipField::Entity,
            right_ids: vec![1],
        };
        let result = set_relationship(&db_context, &event_hub, undo_redo_manager, &dto);
        assert!(result.is_ok());
    }
}
