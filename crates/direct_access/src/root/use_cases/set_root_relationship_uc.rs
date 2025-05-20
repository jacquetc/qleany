use super::RootUnitOfWorkFactoryTrait;
use crate::RootRelationshipDto;
use anyhow::Result;
use common::types::Savepoint;
use common::undo_redo::UndoRedoCommand;
use std::collections::VecDeque;

pub struct SetRootRelationshipUseCase {
    uow_factory: Box<dyn RootUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Savepoint>,
    redo_stack: VecDeque<RootRelationshipDto>,
}

impl SetRootRelationshipUseCase {
    pub fn new(uow_factory: Box<dyn RootUnitOfWorkFactoryTrait>) -> Self {
        SetRootRelationshipUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, dto: &RootRelationshipDto) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let savepoint = uow.create_savepoint()?;
        uow.set_root_relationship(&dto.id, &dto.field, dto.right_ids.as_slice())?;
        uow.commit()?;
        // store savepoint in undo stack
        self.undo_stack.push_back(savepoint);
        self.redo_stack.push_back(dto.clone());

        Ok(())
    }
}

impl UndoRedoCommand for SetRootRelationshipUseCase {
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
        if let Some(RootRelationshipDto {
            id,
            field,
            right_ids,
        }) = self.redo_stack.pop_back()
        {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            let savepoint = uow.create_savepoint()?;
            uow.set_root_relationship(&id, &field, &right_ids)?;
            uow.commit()?;
            self.undo_stack.push_back(savepoint);
        }
        anyhow::Ok(())
    }
}
