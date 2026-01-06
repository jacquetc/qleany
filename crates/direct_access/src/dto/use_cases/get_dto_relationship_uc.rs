use super::DtoUnitOfWorkROFactoryTrait;
use anyhow::Result;
use common::types::EntityId;

pub struct GetDtoRelationshipUseCase {
    uow_factory: Box<dyn DtoUnitOfWorkROFactoryTrait>,
}

impl GetDtoRelationshipUseCase {
    pub fn new(uow_factory: Box<dyn DtoUnitOfWorkROFactoryTrait>) -> Self {
        GetDtoRelationshipUseCase { uow_factory }
    }

    pub fn execute(
        &self,
        id: &EntityId,
        field: &common::direct_access::dto::DtoRelationshipField,
    ) -> Result<Vec<EntityId>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let dtos = uow.get_dto_relationship(id, field)?;
        uow.end_transaction()?;
        Ok(dtos)
    }
}
