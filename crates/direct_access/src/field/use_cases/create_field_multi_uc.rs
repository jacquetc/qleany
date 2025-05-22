use super::FieldUnitOfWorkFactoryTrait;
use crate::field::dtos::{CreateFieldDto, FieldDto};
use anyhow::{Ok, Result};
use common::entities::Field;
use common::undo_redo::UndoRedoCommand;
use std::collections::VecDeque;

pub struct CreateFieldMultiUseCase {
    uow_factory: Box<dyn FieldUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Vec<Field>>,
    redo_stack: VecDeque<Vec<Field>>,
}

impl CreateFieldMultiUseCase {
    pub fn new(uow_factory: Box<dyn FieldUnitOfWorkFactoryTrait>) -> Self {
        CreateFieldMultiUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }
    pub fn execute(&mut self, dtos: &[CreateFieldDto]) -> Result<Vec<FieldDto>> {
        // create
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let fields =
            uow.create_field_multi(&dtos.iter().map(|dto| dto.into()).collect::<Vec<_>>())?;
        uow.commit()?;

        //store in undo stack
        self.undo_stack.push_back(fields.clone());
        self.redo_stack.clear();

        Ok(fields.into_iter().map(|field| field.into()).collect())
    }
}

impl UndoRedoCommand for CreateFieldMultiUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_fields) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.delete_field_multi(
                &last_fields
                    .iter()
                    .map(|field| field.id.clone())
                    .collect::<Vec<_>>(),
            )?;
            uow.commit()?;
            self.redo_stack.push_back(last_fields);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(last_fields) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.create_field_multi(&last_fields)?;
            uow.commit()?;
            self.undo_stack.push_back(last_fields);
        }
        Ok(())
    }
}
