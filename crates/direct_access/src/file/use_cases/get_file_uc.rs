use super::FileUnitOfWorkROFactoryTrait;
use crate::file::dtos::FileDto;
use anyhow::Result;
use common::types::EntityId;

pub struct GetFileUseCase {
    uow_factory: Box<dyn FileUnitOfWorkROFactoryTrait>,
}

impl GetFileUseCase {
    pub fn new(uow_factory: Box<dyn FileUnitOfWorkROFactoryTrait>) -> Self {
        GetFileUseCase { uow_factory }
    }

    pub fn execute(&self, id: &EntityId) -> Result<Option<FileDto>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let file_option = uow.get_file(&id)?;
        uow.end_transaction()?;

        Ok(file_option.map(|file| file.into()))
    }
}
