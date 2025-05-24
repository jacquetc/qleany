use super::FieldUnitOfWorkFactoryTrait;
use crate::field::dtos::FieldDto;
use anyhow::{Ok, Result};
use common::{entities::Field, undo_redo::UndoRedoCommand};
use std::any::Any;
use std::collections::VecDeque;

pub struct UpdateFieldMultiUseCase {
    uow_factory: Box<dyn FieldUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Vec<Field>>,
    redo_stack: VecDeque<Vec<Field>>,
}

impl UpdateFieldMultiUseCase {
    pub fn new(uow_factory: Box<dyn FieldUnitOfWorkFactoryTrait>) -> Self {
        UpdateFieldMultiUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, dtos: &[FieldDto]) -> Result<Vec<FieldDto>> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // check if id exists
        let mut exists = true;
        for FieldDto { id, .. } in dtos {
            if uow.get_field(id)?.is_none() {
                exists = false;
                break;
            }
        }
        if !exists {
            return Err(anyhow::anyhow!("One or more ids do not exist"));
        }
        // store in undo stack
        let fields = uow.get_field_multi(&dtos.iter().map(|dto| dto.id).collect::<Vec<_>>())?;
        let fields = fields
            .into_iter()
            .filter_map(|field| field)
            .collect::<Vec<_>>();
        self.undo_stack.push_back(fields);

        let fields =
            uow.update_field_multi(&dtos.iter().map(|dto| dto.into()).collect::<Vec<_>>())?;
        uow.commit()?;

        self.redo_stack.clear();

        Ok(fields.into_iter().map(|field| field.into()).collect())
    }
}

impl UndoRedoCommand for UpdateFieldMultiUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_fields) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_field_multi(&last_fields)?;
            uow.commit()?;
            self.redo_stack.push_back(last_fields);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(fields) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_field_multi(&fields)?;
            uow.commit()?;
            self.undo_stack.push_back(fields);
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
