use super::DtoUnitOfWorkFactoryTrait;
use crate::dto::dtos::{CreateDtoDto, DtoDto};
use anyhow::{Ok, Result};
use common::entities::Dto;
use common::undo_redo::UndoRedoCommand;
use std::collections::VecDeque;

pub struct CreateDtoMultiUseCase {
    uow_factory: Box<dyn DtoUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Vec<Dto>>,
    redo_stack: VecDeque<Vec<Dto>>,
}

impl CreateDtoMultiUseCase {
    pub fn new(uow_factory: Box<dyn DtoUnitOfWorkFactoryTrait>) -> Self {
        CreateDtoMultiUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }
    pub fn execute(&mut self, dtos: &[CreateDtoDto]) -> Result<Vec<DtoDto>> {
        // create
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let dtos = uow.create_dto_multi(&dtos.iter().map(|dto| dto.into()).collect::<Vec<_>>())?;
        uow.commit()?;

        //store in undo stack
        self.undo_stack.push_back(dtos.clone());
        self.redo_stack.clear();

        Ok(dtos.into_iter().map(|dto| dto.into()).collect())
    }
}

impl UndoRedoCommand for CreateDtoMultiUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_dtos) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.delete_dto_multi(
                &last_dtos
                    .iter()
                    .map(|dto| dto.id.clone())
                    .collect::<Vec<_>>(),
            )?;
            uow.commit()?;
            self.redo_stack.push_back(last_dtos);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(last_dtos) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.create_dto_multi(&last_dtos)?;
            uow.commit()?;
            self.undo_stack.push_back(last_dtos);
        }
        Ok(())
    }
}
