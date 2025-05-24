use super::DtoUnitOfWorkFactoryTrait;
use crate::dto::dtos::DtoDto;
use anyhow::{Ok, Result};
use common::{entities::Dto, undo_redo::UndoRedoCommand};
use std::any::Any;
use std::collections::VecDeque;

pub struct UpdateDtoUseCase {
    uow_factory: Box<dyn DtoUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Dto>,
    redo_stack: VecDeque<Dto>,
}

impl UpdateDtoUseCase {
    pub fn new(uow_factory: Box<dyn DtoUnitOfWorkFactoryTrait>) -> Self {
        UpdateDtoUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn description(&self) -> &str {
        "Update Dto"
    }

    pub fn execute(&mut self, dto: &DtoDto) -> Result<DtoDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        // check if id exists
        if uow.get_dto(&dto.id)?.is_none() {
            return Err(anyhow::anyhow!("Dto with id {} does not exist", dto.id));
        }

        let dto = uow.update_dto(&dto.into())?;
        uow.commit()?;

        // store in undo stack
        self.undo_stack.push_back(dto.clone());
        self.redo_stack.clear();

        Ok(dto.into())
    }
}

impl UndoRedoCommand for UpdateDtoUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_dto) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_dto(&last_dto)?;
            uow.commit()?;
            self.redo_stack.push_back(last_dto);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(dto) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_dto(&dto)?;
            uow.commit()?;
            self.undo_stack.push_back(dto);
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
