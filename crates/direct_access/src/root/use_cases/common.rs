use common::database::CommandUnitOfWork;
use common::direct_access::root::RootRelationshipField;
use common::entities::{EntityId, Root};
use anyhow::Result;

pub trait RootUnitOfWorkTrait : CommandUnitOfWork {
    fn create_root(&self, root: &Root) -> Result<Root>;
    fn get_root(&self, id: &EntityId) -> Result<Option<Root>>;
    fn update_root(&self, root: &Root) -> Result<Root>;
    fn delete_root(&self, id: &EntityId) -> Result<()>;
    fn get_relationships_of(&self, field: &RootRelationshipField, right_ids: &[EntityId]) -> Result<Vec<(EntityId, Vec<EntityId>)>>;
    fn set_relationships(&self, field: &RootRelationshipField, relationships: Vec<(EntityId, Vec<EntityId>)>) -> Result<()>;
}
