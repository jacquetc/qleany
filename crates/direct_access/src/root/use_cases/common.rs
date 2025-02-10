use anyhow::Result;
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::direct_access::root::RootRelationshipField;
use common::entities::{EntityId, Root};

pub trait RootUnitOfWorkFactoryTrait : Send + Sync {
    fn create(&self) -> Box<dyn RootUnitOfWorkTrait>;
}

pub trait RootUnitOfWorkTrait: CommandUnitOfWork {
    fn create_root(&self, root: &Root) -> Result<Root>;
    fn create_root_multi(&self, roots: &[Root]) -> Result<Vec<Root>>;
    fn get_root(&self, id: &EntityId) -> Result<Option<Root>>;
    fn get_root_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Root>>>;
    fn update_root(&self, root: &Root) -> Result<Root>;
    fn update_root_multi(&self, roots: &[Root]) -> Result<Vec<Root>>;
    fn delete_root(&self, id: &EntityId) -> Result<()>;
    fn delete_root_multi(&self, ids: &[EntityId]) -> Result<()>;
    fn get_relationships_of(
        &self,
        field: &RootRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>>;
    fn set_relationships(
        &self,
        field: &RootRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<()>;
}

pub trait RootUnitOfWorkROFactoryTrait {
    fn create(&self) -> Box<dyn RootUnitOfWorkROTrait>;
}

pub trait RootUnitOfWorkROTrait: QueryUnitOfWork {
    fn get_root(&self, id: &EntityId) -> Result<Option<Root>>;
    fn get_root_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Root>>>;
}
