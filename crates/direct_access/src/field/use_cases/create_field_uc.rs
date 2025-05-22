use super::FieldUnitOfWorkFactoryTrait;
use crate::field::dtos::{CreateFieldDto, FieldDto};
use anyhow::{Ok, Result};
use common::entities::Field;
use common::undo_redo::UndoRedoCommand;
use std::collections::VecDeque;

pub struct CreateFieldUseCase {
    uow_factory: Box<dyn FieldUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Field>,
    redo_stack: VecDeque<Field>,
}

impl CreateFieldUseCase {
    pub fn new(uow_factory: Box<dyn FieldUnitOfWorkFactoryTrait>) -> Self {
        CreateFieldUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, dto: CreateFieldDto) -> Result<FieldDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let field = uow.create_field(&dto.into())?;
        uow.commit()?;

        // store in undo stack
        self.undo_stack.push_back(field.clone());
        self.redo_stack.clear();

        Ok(field.into())
    }
}

impl UndoRedoCommand for CreateFieldUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_field) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.delete_field(&last_field.id)?;
            uow.commit()?;
            self.redo_stack.push_back(last_field);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(last_field) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.create_field(&last_field)?;
            uow.commit()?;
            self.undo_stack.push_back(last_field);
        }
        Ok(())
    }
}
