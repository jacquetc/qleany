use super::FileUnitOfWorkROFactoryTrait;
use anyhow::Result;
use common::types::EntityId;

pub struct GetFileRelationshipUseCase {
    uow_factory: Box<dyn FileUnitOfWorkROFactoryTrait>,
}

impl GetFileRelationshipUseCase {
    pub fn new(uow_factory: Box<dyn FileUnitOfWorkROFactoryTrait>) -> Self {
        GetFileRelationshipUseCase { uow_factory }
    }

    pub fn execute(
        &self,
        id: &EntityId,
        field: &common::direct_access::file::FileRelationshipField,
    ) -> Result<Vec<EntityId>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let files = uow.get_file_relationship(id, field)?;
        uow.end_transaction()?;
        Ok(files)
    }
}
