use super::DtoUnitOfWorkFactoryTrait;
use crate::dto::dtos::{CreateDtoDto, DtoDto};
use anyhow::{Ok, Result};
use common::entities::Dto;
use common::undo_redo::UndoRedoCommand;
use std::any::Any;
use std::collections::VecDeque;

pub struct CreateDtoUseCase {
    uow_factory: Box<dyn DtoUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Dto>,
    redo_stack: VecDeque<Dto>,
}

impl CreateDtoUseCase {
    pub fn new(uow_factory: Box<dyn DtoUnitOfWorkFactoryTrait>) -> Self {
        CreateDtoUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, dto: CreateDtoDto) -> Result<DtoDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let dto = uow.create_dto(&dto.into())?;
        uow.commit()?;

        // store in undo stack
        self.undo_stack.push_back(dto.clone());
        self.redo_stack.clear();

        Ok(dto.into())
    }
}

impl UndoRedoCommand for CreateDtoUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_dto) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.delete_dto(&last_dto.id)?;
            uow.commit()?;
            self.redo_stack.push_back(last_dto);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(last_dto) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.create_dto(&last_dto)?;
            uow.commit()?;
            self.undo_stack.push_back(last_dto);
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
