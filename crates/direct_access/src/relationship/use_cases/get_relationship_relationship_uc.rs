use super::RelationshipUnitOfWorkROFactoryTrait;
use anyhow::Result;
use common::types::EntityId;

pub struct GetRelationshipRelationshipUseCase {
    uow_factory: Box<dyn RelationshipUnitOfWorkROFactoryTrait>,
}

impl GetRelationshipRelationshipUseCase {
    pub fn new(uow_factory: Box<dyn RelationshipUnitOfWorkROFactoryTrait>) -> Self {
        GetRelationshipRelationshipUseCase { uow_factory }
    }

    pub fn execute(
        &self,
        id: &EntityId,
        field: &common::direct_access::relationship::RelationshipRelationshipField,
    ) -> Result<Vec<EntityId>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let relationships = uow.get_relationship_relationship(id, field)?;
        uow.end_transaction()?;
        Ok(relationships)
    }
}
