use super::EntityUnitOfWorkFactoryTrait;
use crate::EntityRelationshipDto;
use anyhow::Result;
use common::types::Savepoint;
use common::undo_redo::UndoRedoCommand;
use std::collections::VecDeque;

pub struct SetEntityRelationshipUseCase {
    uow_factory: Box<dyn EntityUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Savepoint>,
    redo_stack: VecDeque<EntityRelationshipDto>,
}

impl SetEntityRelationshipUseCase {
    pub fn new(uow_factory: Box<dyn EntityUnitOfWorkFactoryTrait>) -> Self {
        SetEntityRelationshipUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, dto: &EntityRelationshipDto) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let savepoint = uow.create_savepoint()?;
        uow.set_entity_relationship(&dto.id, &dto.field, dto.right_ids.as_slice())?;
        uow.commit()?;
        // store savepoint in undo stack
        self.undo_stack.push_back(savepoint);
        self.redo_stack.push_back(dto.clone());

        Ok(())
    }
}

impl UndoRedoCommand for SetEntityRelationshipUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(savepoint) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.restore_to_savepoint(savepoint)?;
            uow.commit()?;
        }
        anyhow::Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(EntityRelationshipDto {
            id,
            field,
            right_ids,
        }) = self.redo_stack.pop_back()
        {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            let savepoint = uow.create_savepoint()?;
            uow.set_entity_relationship(&id, &field, &right_ids)?;
            uow.commit()?;
            self.undo_stack.push_back(savepoint);
        }
        anyhow::Ok(())
    }
}
