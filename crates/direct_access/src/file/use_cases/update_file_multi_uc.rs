use super::FileUnitOfWorkFactoryTrait;
use crate::file::dtos::FileDto;
use anyhow::{Ok, Result};
use common::{entities::File, undo_redo::UndoRedoCommand};
use std::any::Any;
use std::collections::VecDeque;

pub struct UpdateFileMultiUseCase {
    uow_factory: Box<dyn FileUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Vec<File>>,
    redo_stack: VecDeque<Vec<File>>,
}

impl UpdateFileMultiUseCase {
    pub fn new(uow_factory: Box<dyn FileUnitOfWorkFactoryTrait>) -> Self {
        UpdateFileMultiUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, dtos: &[FileDto]) -> Result<Vec<FileDto>> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // check if id exists
        let mut exists = true;
        for FileDto { id, .. } in dtos {
            if uow.get_file(id)?.is_none() {
                exists = false;
                break;
            }
        }
        if !exists {
            return Err(anyhow::anyhow!("One or more ids do not exist"));
        }

        let files =
            uow.update_file_multi(&dtos.iter().map(|dto| dto.into()).collect::<Vec<_>>())?;
        uow.commit()?;

        // store in undo stack
        self.undo_stack.push_back(files.clone());
        self.redo_stack.clear();

        Ok(files.into_iter().map(|file| file.into()).collect())
    }
}

impl UndoRedoCommand for UpdateFileMultiUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_files) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_file_multi(&last_files)?;
            uow.commit()?;
            self.redo_stack.push_back(last_files);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(files) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_file_multi(&files)?;
            uow.commit()?;
            self.undo_stack.push_back(files);
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
