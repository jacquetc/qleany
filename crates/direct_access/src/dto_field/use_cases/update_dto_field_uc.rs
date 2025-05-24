use super::DtoFieldUnitOfWorkFactoryTrait;
use crate::dto_field::dtos::DtoFieldDto;
use anyhow::{Ok, Result};
use common::{entities::DtoField, undo_redo::UndoRedoCommand};
use std::any::Any;
use std::collections::VecDeque;

pub struct UpdateDtoFieldUseCase {
    uow_factory: Box<dyn DtoFieldUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<DtoField>,
    redo_stack: VecDeque<DtoField>,
}

impl UpdateDtoFieldUseCase {
    pub fn new(uow_factory: Box<dyn DtoFieldUnitOfWorkFactoryTrait>) -> Self {
        UpdateDtoFieldUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn description(&self) -> &str {
        "Update DtoField"
    }

    pub fn execute(&mut self, dto: &DtoFieldDto) -> Result<DtoFieldDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        // check if id exists
        if uow.get_dto_field(&dto.id)?.is_none() {
            return Err(anyhow::anyhow!(
                "DtoField with id {} does not exist",
                dto.id
            ));
        }

        let dto_field = uow.update_dto_field(&dto.into())?;
        uow.commit()?;

        // store in undo stack
        self.undo_stack.push_back(dto_field.clone());
        self.redo_stack.clear();

        Ok(dto_field.into())
    }
}

impl UndoRedoCommand for UpdateDtoFieldUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_dto_field) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_dto_field(&last_dto_field)?;
            uow.commit()?;
            self.redo_stack.push_back(last_dto_field);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(dto_field) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_dto_field(&dto_field)?;
            uow.commit()?;
            self.undo_stack.push_back(dto_field);
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
