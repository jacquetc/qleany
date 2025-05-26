use super::FileUnitOfWorkFactoryTrait;
use crate::file::dtos::{CreateFileDto, FileDto};
use anyhow::{Ok, Result};
use common::entities::File;
use common::undo_redo::UndoRedoCommand;
use std::any::Any;
use std::collections::VecDeque;

pub struct CreateFileUseCase {
    uow_factory: Box<dyn FileUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<File>,
    redo_stack: VecDeque<File>,
}

impl CreateFileUseCase {
    pub fn new(uow_factory: Box<dyn FileUnitOfWorkFactoryTrait>) -> Self {
        CreateFileUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, dto: CreateFileDto) -> Result<FileDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let file = uow.create_file(&dto.into())?;
        uow.commit()?;

        // store in undo stack
        self.undo_stack.push_back(file.clone());
        self.redo_stack.clear();

        Ok(file.into())
    }
}

impl UndoRedoCommand for CreateFileUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_file) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.delete_file(&last_file.id)?;
            uow.commit()?;
            self.redo_stack.push_back(last_file);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(last_file) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.create_file(&last_file)?;
            uow.commit()?;
            self.undo_stack.push_back(last_file);
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
