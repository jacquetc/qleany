use super::RelationshipUnitOfWorkROFactoryTrait;
use crate::relationship::dtos::RelationshipDto;
use anyhow::Result;
use common::types::EntityId;

pub struct GetRelationshipUseCase {
    uow_factory: Box<dyn RelationshipUnitOfWorkROFactoryTrait>,
}

impl GetRelationshipUseCase {
    pub fn new(uow_factory: Box<dyn RelationshipUnitOfWorkROFactoryTrait>) -> Self {
        GetRelationshipUseCase { uow_factory }
    }

    pub fn execute(&self, id: &EntityId) -> Result<Option<RelationshipDto>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let relationship_option = uow.get_relationship(&id)?;
        uow.end_transaction()?;

        Ok(relationship_option.map(|relationship| relationship.into()))
    }
}
