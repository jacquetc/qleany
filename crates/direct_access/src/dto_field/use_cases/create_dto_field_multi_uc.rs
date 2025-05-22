use super::DtoFieldUnitOfWorkFactoryTrait;
use crate::dto_field::dtos::{CreateDtoFieldDto, DtoFieldDto};
use anyhow::{Ok, Result};
use common::entities::DtoField;
use common::undo_redo::UndoRedoCommand;
use std::collections::VecDeque;

pub struct CreateDtoFieldMultiUseCase {
    uow_factory: Box<dyn DtoFieldUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Vec<DtoField>>,
    redo_stack: VecDeque<Vec<DtoField>>,
}

impl CreateDtoFieldMultiUseCase {
    pub fn new(uow_factory: Box<dyn DtoFieldUnitOfWorkFactoryTrait>) -> Self {
        CreateDtoFieldMultiUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }
    pub fn execute(&mut self, dtos: &[CreateDtoFieldDto]) -> Result<Vec<DtoFieldDto>> {
        // create
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let dto_fields =
            uow.create_dto_field_multi(&dtos.iter().map(|dto| dto.into()).collect::<Vec<_>>())?;
        uow.commit()?;

        //store in undo stack
        self.undo_stack.push_back(dto_fields.clone());
        self.redo_stack.clear();

        Ok(dto_fields
            .into_iter()
            .map(|dto_field| dto_field.into())
            .collect())
    }
}

impl UndoRedoCommand for CreateDtoFieldMultiUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_dto_fields) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.delete_dto_field_multi(
                &last_dto_fields
                    .iter()
                    .map(|dto_field| dto_field.id.clone())
                    .collect::<Vec<_>>(),
            )?;
            uow.commit()?;
            self.redo_stack.push_back(last_dto_fields);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(last_dto_fields) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.create_dto_field_multi(&last_dto_fields)?;
            uow.commit()?;
            self.undo_stack.push_back(last_dto_fields);
        }
        Ok(())
    }
}
