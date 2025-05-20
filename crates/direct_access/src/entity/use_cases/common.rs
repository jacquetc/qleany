use anyhow::Result;
use common::database::CommandUnitOfWork;
use common::direct_access::entity::EntityRelationshipField;
use common::entities::Entity;
use common::types::EntityId;

pub trait EntityUnitOfWorkFactoryTrait {
    fn create(&self) -> Box<dyn EntityUnitOfWorkTrait>;
}

pub trait EntityUnitOfWorkTrait: CommandUnitOfWork {
    fn create_entity(&self, entity: &Entity) -> Result<Entity>;
    fn get_entity(&self, id: &EntityId) -> Result<Option<Entity>>;
    fn update_entity(&self, entity: &Entity) -> Result<Entity>;
    fn delete_entity(&self, id: &EntityId) -> Result<()>;
    fn get_relationships_from_right_ids(
        &self,
        field: &EntityRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>>;
    fn set_relationship_multi(
        &self,
        field: &EntityRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<()>;
}
