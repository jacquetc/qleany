use super::DtoFieldUnitOfWorkFactoryTrait;
use crate::dto_field::dtos::DtoFieldDto;
use anyhow::{Ok, Result};
use common::{entities::DtoField, undo_redo::UndoRedoCommand};
use std::collections::VecDeque;

pub struct UpdateDtoFieldMultiUseCase {
    uow_factory: Box<dyn DtoFieldUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Vec<DtoField>>,
    redo_stack: VecDeque<Vec<DtoField>>,
}

impl UpdateDtoFieldMultiUseCase {
    pub fn new(uow_factory: Box<dyn DtoFieldUnitOfWorkFactoryTrait>) -> Self {
        UpdateDtoFieldMultiUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, dtos: &[DtoFieldDto]) -> Result<Vec<DtoFieldDto>> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // check if id exists
        let mut exists = true;
        for DtoFieldDto { id, .. } in dtos {
            if uow.get_dto_field(id)?.is_none() {
                exists = false;
                break;
            }
        }
        if !exists {
            return Err(anyhow::anyhow!("One or more ids do not exist"));
        }

        let dto_fields =
            uow.update_dto_field_multi(&dtos.iter().map(|dto| dto.into()).collect::<Vec<_>>())?;
        uow.commit()?;

        // store in undo stack
        self.undo_stack.push_back(dto_fields.clone());
        self.redo_stack.clear();

        Ok(dto_fields
            .into_iter()
            .map(|dto_field| dto_field.into())
            .collect())
    }
}

impl UndoRedoCommand for UpdateDtoFieldMultiUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_dto_fields) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_dto_field_multi(&last_dto_fields)?;
            uow.commit()?;
            self.redo_stack.push_back(last_dto_fields);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(dto_fields) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_dto_field_multi(&dto_fields)?;
            uow.commit()?;
            self.undo_stack.push_back(dto_fields);
        }
        Ok(())
    }
}
