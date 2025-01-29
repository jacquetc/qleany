use common::database::CommandUnitOfWork;
use common::direct_access::use_case::UseCaseRelationshipField;
use common::entities::{EntityId, UseCase};
use anyhow::Result;

pub trait UseCaseUnitOfWorkTrait : CommandUnitOfWork {
    fn create_use_case(&self, use_case: &UseCase) -> Result<UseCase>;
    fn get_use_case(&self, id: &EntityId) -> Result<Option<UseCase>>;
    fn update_use_case(&self, use_case: &UseCase) -> Result<UseCase>;
    fn delete_use_case(&self, id: &EntityId) -> Result<()>;
    fn get_relationships_of(&self, field: &UseCaseRelationshipField, right_ids: &[EntityId]) -> Result<Vec<(EntityId, Vec<EntityId>)>>;
    fn set_relationships(&mut self, field: &UseCaseRelationshipField, relationships: Vec<(EntityId, Vec<EntityId>)>) -> Result<()>;
}
