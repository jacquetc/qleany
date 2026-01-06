use super::EntityUnitOfWorkROFactoryTrait;
use anyhow::Result;
use common::types::EntityId;

pub struct GetEntityRelationshipUseCase {
    uow_factory: Box<dyn EntityUnitOfWorkROFactoryTrait>,
}

impl GetEntityRelationshipUseCase {
    pub fn new(uow_factory: Box<dyn EntityUnitOfWorkROFactoryTrait>) -> Self {
        GetEntityRelationshipUseCase { uow_factory }
    }

    pub fn execute(
        &self,
        id: &EntityId,
        field: &common::direct_access::entity::EntityRelationshipField,
    ) -> Result<Vec<EntityId>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let entitys = uow.get_entity_relationship(id, field)?;
        uow.end_transaction()?;
        Ok(entitys)
    }
}
