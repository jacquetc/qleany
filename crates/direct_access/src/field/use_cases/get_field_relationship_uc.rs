use super::FieldUnitOfWorkROFactoryTrait;
use anyhow::Result;
use common::types::EntityId;

pub struct GetFieldRelationshipUseCase {
    uow_factory: Box<dyn FieldUnitOfWorkROFactoryTrait>,
}

impl GetFieldRelationshipUseCase {
    pub fn new(uow_factory: Box<dyn FieldUnitOfWorkROFactoryTrait>) -> Self {
        GetFieldRelationshipUseCase { uow_factory }
    }

    pub fn execute(
        &self,
        id: &EntityId,
        field: &common::direct_access::field::FieldRelationshipField,
    ) -> Result<Vec<EntityId>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let fields = uow.get_field_relationship(id, field)?;
        uow.end_transaction()?;
        Ok(fields)
    }
}
