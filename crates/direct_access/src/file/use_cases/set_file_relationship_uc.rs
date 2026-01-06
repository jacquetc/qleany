use super::FileUnitOfWorkFactoryTrait;
use crate::FileRelationshipDto;
use anyhow::Result;
use common::types::{EntityId, Savepoint};
use common::undo_redo::UndoRedoCommand;
use std::any::Any;
use std::collections::VecDeque;

pub struct SetFileRelationshipUseCase {
    uow_factory: Box<dyn FileUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<FileRelationshipDto>,
    redo_stack: VecDeque<FileRelationshipDto>,
}

impl SetFileRelationshipUseCase {
    pub fn new(uow_factory: Box<dyn FileUnitOfWorkFactoryTrait>) -> Self {
        SetFileRelationshipUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, dto: &FileRelationshipDto) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        // savepoint for undo
        let saved_relationship_ids = uow.get_file_relationship(&dto.id, &dto.field)?;
        let undo_relationship = FileRelationshipDto {
            id: dto.id.clone(),
            field: dto.field.clone(),
            right_ids: saved_relationship_ids,
        };
        //
        uow.set_file_relationship(&dto.id, &dto.field, dto.right_ids.as_slice())?;
        uow.commit()?;
        // store savepoint in undo stack
        self.undo_stack.push_back(undo_relationship);
        self.redo_stack.push_back(dto.clone());

        Ok(())
    }
}

impl UndoRedoCommand for SetFileRelationshipUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(undo_relationship) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.set_file_relationship(
                &undo_relationship.id,
                &undo_relationship.field,
                &undo_relationship.right_ids,
            )?;
            uow.commit()?;
        }
        anyhow::Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(dto) = self.redo_stack.pop_back() {
            self.execute(&dto)?;
        }
        anyhow::Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
