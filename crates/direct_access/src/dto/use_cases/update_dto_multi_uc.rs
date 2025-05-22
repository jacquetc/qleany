use super::DtoUnitOfWorkFactoryTrait;
use crate::dto::dtos::DtoDto;
use anyhow::{Ok, Result};
use common::{entities::Dto, undo_redo::UndoRedoCommand};
use std::collections::VecDeque;

pub struct UpdateDtoMultiUseCase {
    uow_factory: Box<dyn DtoUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Vec<Dto>>,
    redo_stack: VecDeque<Vec<Dto>>,
}

impl UpdateDtoMultiUseCase {
    pub fn new(uow_factory: Box<dyn DtoUnitOfWorkFactoryTrait>) -> Self {
        UpdateDtoMultiUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, dtos: &[DtoDto]) -> Result<Vec<DtoDto>> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // check if id exists
        let mut exists = true;
        for DtoDto { id, .. } in dtos {
            if uow.get_dto(id)?.is_none() {
                exists = false;
                break;
            }
        }
        if !exists {
            return Err(anyhow::anyhow!("One or more ids do not exist"));
        }

        let dtos = uow.update_dto_multi(&dtos.iter().map(|dto| dto.into()).collect::<Vec<_>>())?;
        uow.commit()?;

        // store in undo stack
        self.undo_stack.push_back(dtos.clone());
        self.redo_stack.clear();

        Ok(dtos.into_iter().map(|dto| dto.into()).collect())
    }
}

impl UndoRedoCommand for UpdateDtoMultiUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_dtos) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_dto_multi(&last_dtos)?;
            uow.commit()?;
            self.redo_stack.push_back(last_dtos);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(dtos) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_dto_multi(&dtos)?;
            uow.commit()?;
            self.undo_stack.push_back(dtos);
        }
        Ok(())
    }
}
