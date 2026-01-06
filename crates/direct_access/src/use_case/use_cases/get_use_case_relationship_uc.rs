use super::UseCaseUnitOfWorkROFactoryTrait;
use anyhow::Result;
use common::direct_access::use_case::UseCaseRelationshipField;
use common::types::EntityId;

pub struct GetUseCaseRelationshipUseCase {
    uow_factory: Box<dyn UseCaseUnitOfWorkROFactoryTrait>,
}

impl GetUseCaseRelationshipUseCase {
    pub fn new(uow_factory: Box<dyn UseCaseUnitOfWorkROFactoryTrait>) -> Self {
        GetUseCaseRelationshipUseCase { uow_factory }
    }

    pub fn execute(
        &self,
        id: &EntityId,
        field: &UseCaseRelationshipField,
    ) -> Result<Vec<EntityId>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let use_cases = uow.get_use_case_relationship(id, field)?;
        uow.end_transaction()?;
        Ok(use_cases)
    }
}
