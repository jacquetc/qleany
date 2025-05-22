use super::{
    dtos::{CreateFeatureDto, FeatureDto},
    units_of_work::{FeatureUnitOfWorkFactory, FeatureUnitOfWorkROFactory},
    use_cases::{
        create_feature_multi_uc::CreateFeatureMultiUseCase,
        create_feature_uc::CreateFeatureUseCase, get_feature_multi_uc::GetFeatureMultiUseCase,
        get_feature_uc::GetFeatureUseCase, remove_feature_multi_uc::RemoveFeatureMultiUseCase,
        remove_feature_uc::RemoveFeatureUseCase,
        update_feature_multi_uc::UpdateFeatureMultiUseCase,
        update_feature_uc::UpdateFeatureUseCase,
    },
};
use crate::feature::use_cases::get_feature_relationship_uc::GetFeatureRelationshipUseCase;
use crate::feature::use_cases::set_feature_relationship_uc::SetFeatureRelationshipUseCase;
use crate::FeatureRelationshipDto;
use anyhow::{Ok, Result};
use common::direct_access::feature::FeatureRelationshipField;
use common::undo_redo::UndoRedoManager;
use common::{database::db_context::DbContext, event::EventHub, types::EntityId};
use std::sync::Arc;

pub fn create(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    feature: &CreateFeatureDto,
) -> Result<FeatureDto> {
    let uow_factory = FeatureUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut feature_uc = CreateFeatureUseCase::new(Box::new(uow_factory));
    let result = feature_uc.execute(feature.clone())?;
    undo_redo_manager.add_command(Box::new(feature_uc));
    Ok(result)
}

pub fn get(db_context: &DbContext, id: &EntityId) -> Result<Option<FeatureDto>> {
    let uow_factory = FeatureUnitOfWorkROFactory::new(&db_context);
    let feature_uc = GetFeatureUseCase::new(Box::new(uow_factory));
    feature_uc.execute(id)
}

pub fn update(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    feature: &FeatureDto,
) -> Result<FeatureDto> {
    let uow_factory = FeatureUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut feature_uc = UpdateFeatureUseCase::new(Box::new(uow_factory));
    let result = feature_uc.execute(feature)?;
    undo_redo_manager.add_command(Box::new(feature_uc));
    Ok(result)
}

pub fn remove(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    id: &EntityId,
) -> Result<()> {
    // delete feature
    let uow_factory = FeatureUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut feature_uc = RemoveFeatureUseCase::new(Box::new(uow_factory));
    feature_uc.execute(id)?;
    undo_redo_manager.add_command(Box::new(feature_uc));
    Ok(())
}

pub fn create_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    features: &[CreateFeatureDto],
) -> Result<Vec<FeatureDto>> {
    let uow_factory = FeatureUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut feature_uc = CreateFeatureMultiUseCase::new(Box::new(uow_factory));
    let result = feature_uc.execute(features)?;
    undo_redo_manager.add_command(Box::new(feature_uc));
    Ok(result)
}

pub fn get_multi(db_context: &DbContext, ids: &[EntityId]) -> Result<Vec<Option<FeatureDto>>> {
    let uow_factory = FeatureUnitOfWorkROFactory::new(&db_context);
    let feature_uc = GetFeatureMultiUseCase::new(Box::new(uow_factory));
    feature_uc.execute(ids)
}

pub fn update_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    features: &[FeatureDto],
) -> Result<Vec<FeatureDto>> {
    let uow_factory = FeatureUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut feature_uc = UpdateFeatureMultiUseCase::new(Box::new(uow_factory));
    let result = feature_uc.execute(features)?;
    undo_redo_manager.add_command(Box::new(feature_uc));
    Ok(result)
}

pub fn remove_multi(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    ids: &[EntityId],
) -> Result<()> {
    let uow_factory = FeatureUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut feature_uc = RemoveFeatureMultiUseCase::new(Box::new(uow_factory));
    feature_uc.execute(ids)?;
    undo_redo_manager.add_command(Box::new(feature_uc));
    Ok(())
}

pub fn get_relationship(
    db_context: &DbContext,
    id: &EntityId,
    field: &FeatureRelationshipField,
) -> Result<Vec<EntityId>> {
    let uow_factory = FeatureUnitOfWorkROFactory::new(&db_context);
    let feature_uc = GetFeatureRelationshipUseCase::new(Box::new(uow_factory));
    feature_uc.execute(id, field)
}

pub fn set_relationship(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    dto: &FeatureRelationshipDto,
) -> Result<()> {
    let uow_factory = FeatureUnitOfWorkFactory::new(&db_context, &event_hub);
    let mut feature_uc = SetFeatureRelationshipUseCase::new(Box::new(uow_factory));
    feature_uc.execute(dto)?;
    undo_redo_manager.add_command(Box::new(feature_uc));
    Ok(())
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use common::database::db_context::DbContext;

    #[test]
    fn test_create_feature() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let feature = CreateFeatureDto {
            ..Default::default()
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &feature);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_feature() {
        // get with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let id = 115;
        let result = get(&db_context, &id);
        assert!(result.is_err());

        // create
        let feature = CreateFeatureDto {
            name: "test".to_string(),
            ..Default::default()
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &feature);
        assert!(result.is_ok());

        // get with valid id
        let id = 1;
        let result = get(&db_context, &id);
        assert!(result.is_ok());
        let feature = result.unwrap();
        assert!(feature.is_some());
        assert_eq!(feature.unwrap().name, "test");
    }

    #[test]
    fn test_update_feature() {
        // update with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let feature = FeatureDto {
            id: 115,
            ..Default::default()
        };
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = update(&db_context, &event_hub, &mut undo_redo_manager, &feature);
        assert!(result.is_err());

        // create
        let feature = CreateFeatureDto {
            ..Default::default()
        };
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &feature);
        assert!(result.is_ok());

        // update with valid id
        let feature = FeatureDto {
            id: 1,
            name: "test".to_string(),
            ..Default::default()
        };
        let result = update(&db_context, &event_hub, &mut undo_redo_manager, &feature);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "test");
    }

    #[test]
    fn test_remove_feature() {
        // remove with invalid id
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let id = 115;
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = remove(&db_context, &event_hub, &mut undo_redo_manager, &id);
        assert!(result.is_err());

        // create
        let feature = CreateFeatureDto {
            ..Default::default()
        };
        let result = create(&db_context, &event_hub, &mut undo_redo_manager, &feature);
        assert!(result.is_ok());

        // remove with valid id
        let id = result.unwrap().id;
        let result = remove(&db_context, &event_hub, &mut undo_redo_manager, &id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_feature_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        let features = vec![
            CreateFeatureDto {
                ..Default::default()
            },
            CreateFeatureDto {
                ..Default::default()
            },
            CreateFeatureDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &features);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());

        // set up
        let features = vec![
            CreateFeatureDto {
                ..Default::default()
            },
            CreateFeatureDto {
                ..Default::default()
            },
            CreateFeatureDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &features);
        assert!(result.is_ok());

        let ids = vec![1, 2, 3];
        let result = get_multi(&db_context, &ids);
        assert!(result.is_ok());
        let features = result.unwrap();
        assert_eq!(features.len(), 3);
    }

    #[test]
    fn test_update_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());
        // set up
        let features = vec![
            CreateFeatureDto {
                ..Default::default()
            },
            CreateFeatureDto {
                ..Default::default()
            },
            CreateFeatureDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &features);
        assert!(result.is_ok());

        // test update_multi
        let features = vec![
            FeatureDto {
                id: 1,
                ..Default::default()
            },
            FeatureDto {
                id: 2,
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = update_multi(&db_context, &event_hub, &mut undo_redo_manager, &features);
        assert!(result.is_ok());
    }

    #[test]
    fn test_remove_multi() {
        let db_context = DbContext::new().unwrap();
        let event_hub = Arc::new(EventHub::new());

        // set up
        let features = vec![
            CreateFeatureDto {
                ..Default::default()
            },
            CreateFeatureDto {
                ..Default::default()
            },
            CreateFeatureDto {
                ..Default::default()
            },
        ];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &features);
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
        let features = vec![CreateFeatureDto {
            use_cases: vec![1],
            ..Default::default()
        }];
        let mut undo_redo_manager = UndoRedoManager::new();
        let result = create_multi(&db_context, &event_hub, &mut undo_redo_manager, &features);
        assert!(result.is_ok());

        let id = 1;
        let field = common::direct_access::feature::FeatureRelationshipField::UseCases;
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
        let dto = FeatureRelationshipDto {
            id: 1,
            field: common::direct_access::feature::FeatureRelationshipField::UseCases,
            right_ids: vec![1, 2, 3],
        };
        let result = set_relationship(&db_context, &event_hub, undo_redo_manager, &dto);
        assert!(result.is_ok());
    }
}
