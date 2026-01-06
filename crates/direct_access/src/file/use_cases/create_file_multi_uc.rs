use super::FileUnitOfWorkFactoryTrait;
use crate::file::dtos::{CreateFileDto, FileDto};
use anyhow::{Ok, Result};
use common::entities::File;
use common::undo_redo::UndoRedoCommand;
use std::any::Any;
use std::collections::VecDeque;

pub struct CreateFileMultiUseCase {
    uow_factory: Box<dyn FileUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Vec<File>>,
    redo_stack: VecDeque<Vec<File>>,
}

impl CreateFileMultiUseCase {
    pub fn new(uow_factory: Box<dyn FileUnitOfWorkFactoryTrait>) -> Self {
        CreateFileMultiUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }
    pub fn execute(&mut self, dtos: &[CreateFileDto]) -> Result<Vec<FileDto>> {
        // create
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let files =
            uow.create_file_multi(&dtos.iter().map(|dto| dto.into()).collect::<Vec<_>>())?;
        uow.commit()?;

        //store in undo stack
        self.undo_stack.push_back(files.clone());
        self.redo_stack.clear();

        Ok(files.into_iter().map(|file| file.into()).collect())
    }
}

impl UndoRedoCommand for CreateFileMultiUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_files) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.delete_file_multi(
                &last_files
                    .iter()
                    .map(|file| file.id.clone())
                    .collect::<Vec<_>>(),
            )?;
            uow.commit()?;
            self.redo_stack.push_back(last_files);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(last_files) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.create_file_multi(&last_files)?;
            uow.commit()?;
            self.undo_stack.push_back(last_files);
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
