use super::FieldUnitOfWorkFactoryTrait;
use crate::field::dtos::FieldDto;
use anyhow::{Ok, Result};
use common::{entities::Field, undo_redo::UndoRedoCommand};
use std::collections::VecDeque;

pub struct UpdateFieldUseCase {
    uow_factory: Box<dyn FieldUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Field>,
    redo_stack: VecDeque<Field>,
}

impl UpdateFieldUseCase {
    pub fn new(uow_factory: Box<dyn FieldUnitOfWorkFactoryTrait>) -> Self {
        UpdateFieldUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn description(&self) -> &str {
        "Update Field"
    }

    pub fn execute(&mut self, dto: &FieldDto) -> Result<FieldDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        // check if id exists
        if uow.get_field(&dto.id)?.is_none() {
            return Err(anyhow::anyhow!("Field with id {} does not exist", dto.id));
        }

        let field = uow.update_field(&dto.into())?;
        uow.commit()?;

        // store in undo stack
        self.undo_stack.push_back(field.clone());
        self.redo_stack.clear();

        Ok(field.into())
    }
}

impl UndoRedoCommand for UpdateFieldUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_field) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_field(&last_field)?;
            uow.commit()?;
            self.redo_stack.push_back(last_field);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(field) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_field(&field)?;
            uow.commit()?;
            self.undo_stack.push_back(field);
        }
        Ok(())
    }
}
