use super::{
    dtos::{CreateRelationshipDto, RelationshipDto},
    units_of_work::{RelationshipUnitOfWorkFactory, RelationshipUnitOfWorkROFactory},
    use_cases::{
        create_relationship_multi_uc::CreateRelationshipMultiUseCase,
        create_relationship_uc::CreateRelationshipUseCase,
        get_relationship_multi_uc::GetRelationshipMultiUseCase,
        get_relationship_uc::GetRelationshipUseCase,
        remove_relationship_multi_uc::RemoveRelationshipMultiUseCase,
        remove_relationship_uc::RemoveRelationshipUseCase,
        update_relationship_multi_uc::UpdateRelationshipMultiUseCase,
        update_relationship_uc::UpdateRelationshipUseCase,
    },
};
use crate::RelationshipRelationshipDto;
use crate::relationship::use_cases::get_relationship_relationship_uc::GetRelationshipRelationshipUseCase;
use crate::relationship::use_cases::set_relationship_relationship_uc::SetRelationshipRelationshipUseCase;
use anyhow::{Ok, Result};
use common::direct_access::relationship::RelationshipRelationshipField;
use common::undo_redo::UndoRedoManager;
use common::{database::db_context::DbContext, event::EventHub, types::EntityId};
use std::sync::Arc;

pub fn create(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    relationship: &CreateRelationshipDto,
) -> Result<RelationshipDto> {
    let uow_factory = RelationshipUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut relationship_uc = CreateRelationshipUseCase::new(Box::new(uow_factory));
    let result = relationship_uc.execute(relationship.clone())?;
    undo_redo_manager.add_command(Box::new(relationship_uc));
    Ok(result)
}

pub fn get(db_context: &DbContext, id: &EntityId) -> Result<Option<RelationshipDto>> {
    let uow_factory = RelationshipUnitOfWorkROFactory::new(&db_context);
    let relationship_uc = GetRelationshipUseCase::new(Box::new(uow_factory));
    relationship_uc.execute(id)
}

pub fn update(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    relationship: &RelationshipDto,
) -> Result<RelationshipDto> {
    let uow_factory = RelationshipUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut relationship_uc = UpdateRelationshipUseCase::new(Box::new(uow_factory));
    let result = relationship_uc.execute(relationship)?;
    undo_redo_manager.add_command(Box::new(relationship_uc));
    Ok(result)
}

pub fn remove(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    id: &EntityId,
) -> Result<()> {
    // delete relationship
    let uow_factory = RelationshipUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut relationship_uc = RemoveRelationshipUseCase::new(Box::new(uow_factory));
    relationship_uc.execute(id)?;
    undo_redo_manager.add_command(Box::new(relationship_uc));
    Ok(())
}

pub fn create_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    relationships: &[CreateRelationshipDto],
) -> Result<Vec<RelationshipDto>> {
    let uow_factory = RelationshipUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut relationship_uc = CreateRelationshipMultiUseCase::new(Box::new(uow_factory));
    let result = relationship_uc.execute(relationships)?;
    undo_redo_manager.add_command(Box::new(relationship_uc));
    Ok(result)
}

pub fn get_multi(db_context: &DbContext, ids: &[EntityId]) -> Result<Vec<Option<RelationshipDto>>> {
    let uow_factory = RelationshipUnitOfWorkROFactory::new(&db_context);
    let relationship_uc = GetRelationshipMultiUseCase::new(Box::new(uow_factory));
    relationship_uc.execute(ids)
}

pub fn update_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    relationships: &[RelationshipDto],
) -> Result<Vec<RelationshipDto>> {
    let uow_factory = RelationshipUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut relationship_uc = UpdateRelationshipMultiUseCase::new(Box::new(uow_factory));
    let result = relationship_uc.execute(relationships)?;
    undo_redo_manager.add_command(Box::new(relationship_uc));
    Ok(result)
}

pub fn remove_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    ids: &[EntityId],
) -> Result<()> {
    let uow_factory = RelationshipUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut relationship_uc = RemoveRelationshipMultiUseCase::new(Box::new(uow_factory));
    relationship_uc.execute(ids)?;
    undo_redo_manager.add_command(Box::new(relationship_uc));
    Ok(())
}

pub fn get_relationship(
    db_context: &DbContext,
    id: &EntityId,
    field: &RelationshipRelationshipField,
) -> Result<Vec<EntityId>> {
    let uow_factory = RelationshipUnitOfWorkROFactory::new(&db_context);
    let relationship_uc = GetRelationshipRelationshipUseCase::new(Box::new(uow_factory));
    relationship_uc.execute(id, field)
}

pub fn set_relationship(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    dto: &RelationshipRelationshipDto,
) -> Result<()> {
    let uow_factory = RelationshipUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut relationship_uc = SetRelationshipRelationshipUseCase::new(Box::new(uow_factory));
    relationship_uc.execute(dto)?;
    undo_redo_manager.add_command(Box::new(relationship_uc));
    Ok(())
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use common::database::db_context::DbContext;

    #[test]
    fn test_create_relationship() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let relationship = CreateRelationshipDto {
            ..Default::default()
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create(
            &db_context,
            &event_hub,
            &mut undo_redo_manager,
            &relationship,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_relationship() {
        // get with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let id = 115;
        let result = get(&db_context, &id);
        assert!(result.is_err());

        // create
        let relationship = CreateRelationshipDto {
            left_entity: 1,
            ..Default::default()
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create(
            &db_context,
            &event_hub,
            &mut undo_redo_manager,
            &relationship,
        );
        assert!(result.is_ok());

        // get with valid id
        let id = 1;
        let result = get(&db_context, &id);
        assert!(result.is_ok());
        let relationship = result.unwrap();
        assert!(relationship.is_some());
        assert_eq!(relationship.unwrap().left_entity, 1);
    }

    #[test]
    fn test_update_relationship() {
        // update with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let relationship = RelationshipDto {
            id: 115,
            ..Default::default()
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = update(
            &db_context,
            &event_hub,
            &mut undo_redo_manager,
            &relationship,
        );
        assert!(result.is_err());

        // create
        let relationship = CreateRelationshipDto {
            ..Default::default()
        };
        let result = create(
            &db_context,
            &event_hub,
            &mut undo_redo_manager,
            &relationship,
        );
        assert!(result.is_ok());

        // update with valid id
        let relationship = RelationshipDto {
            id: 1,
            field_name: "test".to_string(),
            ..Default::default()
        };
        let result = update(
            &db_context,
            &event_hub,
            &mut undo_redo_manager,
            &relationship,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().field_name, "test");
    }

    #[test]
    fn test_remove_relationship() {
        // remove with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let id = 115;
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = remove(&db_context, &event_hub, &mut undo_redo_manager, &id);
        assert!(result.is_err());

        // create
        let relationship = CreateRelationshipDto {
            ..Default::default()
        };
        let result = create(
            &db_context,
            &event_hub,
            &mut undo_redo_manager,
            &relationship,
        );
        assert!(result.is_ok());

        // remove with valid id
        let id = result.unwrap().id;
        let result = remove(&db_context, &event_hub, &mut undo_redo_manager, &id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_relationship_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let relationships = vec![
            CreateRelationshipDto {
                ..Default::default()
            },
            CreateRelationshipDto {
                ..Default::default()
            },
            CreateRelationshipDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(
            &db_context,
            &event_hub,
            &mut undo_redo_manager,
            &relationships,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());

        // set up
        let relationships = vec![
            CreateRelationshipDto {
                ..Default::default()
            },
            CreateRelationshipDto {
                ..Default::default()
            },
            CreateRelationshipDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(
            &db_context,
            &event_hub,
            &mut undo_redo_manager,
            &relationships,
        );
        assert!(result.is_ok());

        let ids = vec![1, 2, 3];
        let result = get_multi(&db_context, &ids);
        assert!(result.is_ok());
        let relationships = result.unwrap();
        assert_eq!(relationships.len(), 3);
    }

    #[test]
    fn test_update_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        // set up
        let relationships = vec![
            CreateRelationshipDto {
                ..Default::default()
            },
            CreateRelationshipDto {
                ..Default::default()
            },
            CreateRelationshipDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(
            &db_context,
            &event_hub,
            &mut undo_redo_manager,
            &relationships,
        );
        assert!(result.is_ok());

        // test update_multi
        let relationships = vec![
            RelationshipDto {
                id: 1,
                ..Default::default()
            },
            RelationshipDto {
                id: 2,
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = update_multi(
            &db_context,
            &event_hub,
            &mut undo_redo_manager,
            &relationships,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_remove_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());

        // set up
        let relationships = vec![
            CreateRelationshipDto {
                ..Default::default()
            },
            CreateRelationshipDto {
                ..Default::default()
            },
            CreateRelationshipDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(
            &db_context,
            &event_hub,
            &mut undo_redo_manager,
            &relationships,
        );
        assert!(result.is_ok());

        // test remove_multi
        let ids = vec![1, 2, 3];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = remove_multi(&db_context, &event_hub, &mut undo_redo_manager, &ids);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_relationship_relationship() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());

        // set up
        let relationships = vec![CreateRelationshipDto {
            left_entity: 1,
            ..Default::default()
        }];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(
            &db_context,
            &event_hub,
            &mut undo_redo_manager,
            &relationships,
        );
        assert!(result.is_ok());

        let id = 1;
        let field = common::direct_access::relationship::RelationshipRelationshipField::LeftEntity;
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
        let dto = RelationshipRelationshipDto {
            id: 1,
            field: common::direct_access::relationship::RelationshipRelationshipField::LeftEntity,
            right_ids: vec![1, 2, 3],
        };
        let result = set_relationship(&db_context, &event_hub, undo_redo_manager, &dto);
        assert!(result.is_ok());
    }
}
