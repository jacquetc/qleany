use common::database::CommandUnitOfWork;
use common::entities::{EntityId, Global};
use anyhow::Result;

pub trait GlobalUnitOfWorkTrait : CommandUnitOfWork {
    fn create_global(&self, global: &Global) -> Result<Global>;
    fn get_global(&self, id: &EntityId) -> Result<Option<Global>>;
    fn update_global(&self, global: &Global) -> Result<Global>;
    fn delete_global(&self, id: &EntityId) -> Result<()>;
}
