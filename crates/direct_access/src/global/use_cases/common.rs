use anyhow::Result;
use common::database::CommandUnitOfWork;
use common::entities::{EntityId, Global};

pub trait GlobalUnitOfWorkFactoryTrait {
    fn create(&self) -> Box<dyn GlobalUnitOfWorkTrait>;
}

pub trait GlobalUnitOfWorkTrait: CommandUnitOfWork {
    fn create_global(&self, global: &Global) -> Result<Global>;
    fn get_global(&self, id: &EntityId) -> Result<Option<Global>>;
    fn update_global(&self, global: &Global) -> Result<Global>;
    fn delete_global(&self, id: &EntityId) -> Result<()>;
}
