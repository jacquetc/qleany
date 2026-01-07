use super::{
    dtos::{CreateEntityDto, EntityDto},
    units_of_work::{EntityUnitOfWorkFactory, EntityUnitOfWorkROFactory},
    use_cases::{
        create_entity_multi_uc::CreateEntityMultiUseCase, create_entity_uc::CreateEntityUseCase,
        get_entity_multi_uc::GetEntityMultiUseCase, get_entity_uc::GetEntityUseCase,
        remove_entity_multi_uc::RemoveEntityMultiUseCase, remove_entity_uc::RemoveEntityUseCase,
        update_entity_multi_uc::UpdateEntityMultiUseCase, update_entity_uc::UpdateEntityUseCase,
    },
};
use crate::EntityRelationshipDto;
use crate::entity::use_cases::get_entity_relationship_uc::GetEntityRelationshipUseCase;
use crate::entity::use_cases::set_entity_relationship_uc::SetEntityRelationshipUseCase;
use anyhow::{Ok, Result};
use common::direct_access::entity::EntityRelationshipField;
use common::undo_redo::UndoRedoManager;
use common::{database::db_context::DbContext, event::EventHub, types::EntityId};
use std::sync::Arc;

pub fn create(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    entity: &CreateEntityDto,
) -> Result<EntityDto> {
    let uow_factory = EntityUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut entity_uc = CreateEntityUseCase::new(Box::new(uow_factory));
    let result = entity_uc.execute(entity.clone())?;
    undo_redo_manager.add_command(Box::new(entity_uc));
    Ok(result)
}

pub fn get(db_context: &DbContext, id: &EntityId) -> Result<Option<EntityDto>> {
    let uow_factory = EntityUnitOfWorkROFactory::new(&db_context);
    let entity_uc = GetEntityUseCase::new(Box::new(uow_factory));
    entity_uc.execute(id)
}

pub fn update(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    entity: &EntityDto,
) -> Result<EntityDto> {
    let uow_factory = EntityUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut entity_uc = UpdateEntityUseCase::new(Box::new(uow_factory));
    let result = entity_uc.execute(entity)?;
    undo_redo_manager.add_command(Box::new(entity_uc));
    Ok(result)
}

pub fn remove(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    id: &EntityId,
) -> Result<()> {
    // delete entity
    let uow_factory = EntityUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut entity_uc = RemoveEntityUseCase::new(Box::new(uow_factory));
    entity_uc.execute(id)?;
    undo_redo_manager.add_command(Box::new(entity_uc));
    Ok(())
}

pub fn create_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    entitys: &[CreateEntityDto],
) -> Result<Vec<EntityDto>> {
    let uow_factory = EntityUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut entity_uc = CreateEntityMultiUseCase::new(Box::new(uow_factory));
    let result = entity_uc.execute(entitys)?;
    undo_redo_manager.add_command(Box::new(entity_uc));
    Ok(result)
}

pub fn get_multi(db_context: &DbContext, ids: &[EntityId]) -> Result<Vec<Option<EntityDto>>> {
    let uow_factory = EntityUnitOfWorkROFactory::new(&db_context);
    let entity_uc = GetEntityMultiUseCase::new(Box::new(uow_factory));
    entity_uc.execute(ids)
}

pub fn update_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    entitys: &[EntityDto],
) -> Result<Vec<EntityDto>> {
    let uow_factory = EntityUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut entity_uc = UpdateEntityMultiUseCase::new(Box::new(uow_factory));
    let result = entity_uc.execute(entitys)?;
    undo_redo_manager.add_command(Box::new(entity_uc));
    Ok(result)
}

pub fn remove_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    ids: &[EntityId],
) -> Result<()> {
    let uow_factory = EntityUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut entity_uc = RemoveEntityMultiUseCase::new(Box::new(uow_factory));
    entity_uc.execute(ids)?;
    undo_redo_manager.add_command(Box::new(entity_uc));
    Ok(())
}

pub fn get_relationship(
    db_context: &DbContext,
    id: &EntityId,
    field: &EntityRelationshipField,
) -> Result<Vec<EntityId>> {
    let uow_factory = EntityUnitOfWorkROFactory::new(&db_context);
    let entity_uc = GetEntityRelationshipUseCase::new(Box::new(uow_factory));
    entity_uc.execute(id, field)
}

pub fn set_relationship(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    dto: &EntityRelationshipDto,
) -> Result<()> {
    let uow_factory = EntityUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut entity_uc = SetEntityRelationshipUseCase::new(Box::new(uow_factory));
    entity_uc.execute(dto)?;
    undo_redo_manager.add_command(Box::new(entity_uc));
    Ok(())
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use common::database::db_context::DbContext;

    #[test]
    fn test_create_entity() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let entity = CreateEntityDto {
            ..Default::default()
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &entity);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_entity() {
        // get with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let id = 115;
        let result = get(&db_context, &id);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());

        // create
        let entity = CreateEntityDto {
            name: "test".to_string(),
            ..Default::default()
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &entity);
        assert!(result.is_ok());

        // get with valid id
        let id = 1;
        let result = get(&db_context, &id);
        assert!(result.is_ok());
        let entity = result.unwrap();
        assert!(entity.is_some());
        assert_eq!(entity.unwrap().name, "test");
    }

    #[test]
    fn test_update_entity() {
        // update with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let entity = EntityDto {
            id: 115,
            ..Default::default()
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = update(&db_context, &event_hub, &mut undo_redo_manager, &entity);
        assert!(result.is_err());

        // create
        let entity = CreateEntityDto {
            ..Default::default()
        };
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &entity);
        assert!(result.is_ok());

        // update with valid id
        let entity = EntityDto {
            id: 1,
            name: "test".to_string(),
            ..Default::default()
        };
        let result = update(&db_context, &event_hub, &mut undo_redo_manager, &entity);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "test");
    }

    #[test]
    fn test_remove_entity() {
        // remove with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let id = 115;
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = remove(&db_context, &event_hub, &mut undo_redo_manager, &id);
        assert!(result.is_err());

        // create
        let entity = CreateEntityDto {
            ..Default::default()
        };
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &entity);
        assert!(result.is_ok());

        // remove with valid id
        let id = result.unwrap().id;
        let result = remove(&db_context, &event_hub, &mut undo_redo_manager, &id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_entity_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let entitys = vec![
            CreateEntityDto {
                ..Default::default()
            },
            CreateEntityDto {
                ..Default::default()
            },
            CreateEntityDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &entitys);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());

        // set up
        let entitys = vec![
            CreateEntityDto {
                ..Default::default()
            },
            CreateEntityDto {
                ..Default::default()
            },
            CreateEntityDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &entitys);
        assert!(result.is_ok());

        let ids = vec![1, 2, 3];
        let result = get_multi(&db_context, &ids);
        assert!(result.is_ok());
        let entitys = result.unwrap();
        assert_eq!(entitys.len(), 3);
    }

    #[test]
    fn test_update_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        // set up
        let entitys = vec![
            CreateEntityDto {
                ..Default::default()
            },
            CreateEntityDto {
                ..Default::default()
            },
            CreateEntityDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &entitys);
        assert!(result.is_ok());

        // test update_multi
        let entitys = vec![
            EntityDto {
                id: 1,
                ..Default::default()
            },
            EntityDto {
                id: 2,
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = update_multi(&db_context, &event_hub, &mut undo_redo_manager, &entitys);
        assert!(result.is_ok());
    }

    #[test]
    fn test_remove_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());

        // set up
        let entitys = vec![
            CreateEntityDto {
                ..Default::default()
            },
            CreateEntityDto {
                ..Default::default()
            },
            CreateEntityDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &entitys);
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
        let entitys = vec![CreateEntityDto {
            fields: vec![1],
            ..Default::default()
        }];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &entitys);
        assert!(result.is_ok());

        let id = 1;
        let field = common::direct_access::entity::EntityRelationshipField::Fields;
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
        let dto = EntityRelationshipDto {
            id: 1,
            field: common::direct_access::entity::EntityRelationshipField::Fields,
            right_ids: vec![1, 2, 3],
        };
        let result = set_relationship(&db_context, &event_hub, undo_redo_manager, &dto);
        assert!(result.is_ok());
    }

    #[test]
    fn test_remove_entity_uc_with_undo_redo() {
        // Setup: Create a new database context and event hub
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let mut undo_redo_manager = UndoRedoManager::new();

        // Step 1: Create an entity
        let entity = CreateEntityDto {
            name: "test entity".to_string(),
            ..Default::default()
        };
        let create_result = create(&db_context, &event_hub, &mut undo_redo_manager, &entity);
        assert!(create_result.is_ok());
        let created_entity = create_result.unwrap();
        let entity_id = created_entity.id;

        // Verify entity exists
        let get_result = get(&db_context, &entity_id);
        assert!(get_result.is_ok());
        assert!(get_result.unwrap().is_some());

        // Step 2: Remove the entity
        let remove_result = remove(&db_context, &event_hub, &mut undo_redo_manager, &entity_id);
        assert!(remove_result.is_ok());

        // Verify entity is removed
        let get_result = get(&db_context, &entity_id);
        assert!(get_result.is_ok() && get_result.unwrap().is_none());

        // Step 3: Undo the removal
        let undo_result = undo_redo_manager.undo();
        assert!(undo_result.is_ok());

        // Verify entity exists again after undo
        let get_result = get(&db_context, &entity_id);
        assert!(get_result.is_ok());
        let entity_option = get_result.unwrap();
        assert!(entity_option.is_some());
        let entity = entity_option.unwrap();
        assert_eq!(entity.id, entity_id);
        assert_eq!(entity.name, "test entity");

        // Step 4: Redo the removal
        let redo_result = undo_redo_manager.redo();
        assert!(redo_result.is_ok());

        // Verify entity is removed again after redo
        let get_result = get(&db_context, &entity_id);
        assert!(get_result.is_ok() && get_result.unwrap().is_none());
    }
}
