use super::EntityUnitOfWorkFactoryTrait;
use crate::entity::dtos::EntityDto;
use anyhow::{Ok, Result};
use common::{entities::Entity, undo_redo::UndoRedoCommand};
use std::any::Any;
use std::collections::VecDeque;

pub struct UpdateEntityMultiUseCase {
    uow_factory: Box<dyn EntityUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Vec<Entity>>,
    redo_stack: VecDeque<Vec<Entity>>,
}

impl UpdateEntityMultiUseCase {
    pub fn new(uow_factory: Box<dyn EntityUnitOfWorkFactoryTrait>) -> Self {
        UpdateEntityMultiUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, dtos: &[EntityDto]) -> Result<Vec<EntityDto>> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // check if id exists
        let mut exists = true;
        for EntityDto { id, .. } in dtos {
            if uow.get_entity(id)?.is_none() {
                exists = false;
                break;
            }
        }
        if !exists {
            return Err(anyhow::anyhow!("One or more ids do not exist"));
        }

        let entitys =
            uow.update_entity_multi(&dtos.iter().map(|dto| dto.into()).collect::<Vec<_>>())?;
        uow.commit()?;

        // store in undo stack
        self.undo_stack.push_back(entitys.clone());
        self.redo_stack.clear();

        Ok(entitys.into_iter().map(|entity| entity.into()).collect())
    }
}

impl UndoRedoCommand for UpdateEntityMultiUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_entitys) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_entity_multi(&last_entitys)?;
            uow.commit()?;
            self.redo_stack.push_back(last_entitys);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(entitys) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_entity_multi(&entitys)?;
            uow.commit()?;
            self.undo_stack.push_back(entitys);
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
