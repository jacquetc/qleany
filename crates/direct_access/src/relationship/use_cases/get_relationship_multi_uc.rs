use super::RelationshipUnitOfWorkROFactoryTrait;
use crate::relationship::dtos::RelationshipDto;
use anyhow::Result;
use common::types::EntityId;

pub struct GetRelationshipMultiUseCase {
    uow_factory: Box<dyn RelationshipUnitOfWorkROFactoryTrait>,
}

impl GetRelationshipMultiUseCase {
    pub fn new(uow_factory: Box<dyn RelationshipUnitOfWorkROFactoryTrait>) -> Self {
        GetRelationshipMultiUseCase { uow_factory }
    }

    pub fn execute(&self, ids: &[EntityId]) -> Result<Vec<Option<RelationshipDto>>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let relationships = uow.get_relationship_multi(ids)?;
        uow.end_transaction()?;
        Ok(relationships
            .into_iter()
            .map(|relationship| relationship.map(|r| r.into()))
            .collect())
    }
}
