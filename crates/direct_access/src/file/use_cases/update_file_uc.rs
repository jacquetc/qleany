use super::FileUnitOfWorkFactoryTrait;
use crate::file::dtos::FileDto;
use anyhow::{Ok, Result};
use common::{entities::File, undo_redo::UndoRedoCommand};
use std::any::Any;
use std::collections::VecDeque;

pub struct UpdateFileUseCase {
    uow_factory: Box<dyn FileUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<File>,
    redo_stack: VecDeque<File>,
}

impl UpdateFileUseCase {
    pub fn new(uow_factory: Box<dyn FileUnitOfWorkFactoryTrait>) -> Self {
        UpdateFileUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn description(&self) -> &str {
        "Update File"
    }

    pub fn execute(&mut self, dto: &FileDto) -> Result<FileDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        // check if id exists
        if uow.get_file(&dto.id)?.is_none() {
            return Err(anyhow::anyhow!("File with id {} does not exist", dto.id));
        }

        // store in undo stack
        let file = uow.get_file(&dto.id)?.unwrap();
        self.undo_stack.push_back(file.clone());

        let file = uow.update_file(&dto.into())?;
        uow.commit()?;

        Ok(file.into())
    }
}

impl UndoRedoCommand for UpdateFileUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_file) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_file(&last_file)?;
            uow.commit()?;
            self.redo_stack.push_back(last_file);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(file) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_file(&file)?;
            uow.commit()?;
            self.undo_stack.push_back(file);
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
