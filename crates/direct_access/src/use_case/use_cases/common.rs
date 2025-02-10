use anyhow::Result;
use common::database::{CommandUnitOfWork, QueryUnitOfWork};
use common::direct_access::use_case::UseCaseRelationshipField;
use common::entities::{EntityId, UseCase};

pub trait UseCaseUnitOfWorkFactoryTrait : Send + Sync {
    fn create(&self) -> Box<dyn UseCaseUnitOfWorkTrait>;
}

pub trait UseCaseUnitOfWorkTrait: CommandUnitOfWork {
    fn create_use_case(&self, use_case: &UseCase) -> Result<UseCase>;
    fn create_use_case_multi(&self, use_cases: &[UseCase]) -> Result<Vec<UseCase>>;
    fn get_use_case(&self, id: &EntityId) -> Result<Option<UseCase>>;
    fn get_use_case_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<UseCase>>>;
    fn update_use_case(&self, use_case: &UseCase) -> Result<UseCase>;
    fn update_use_case_multi(&self, use_cases: &[UseCase]) -> Result<Vec<UseCase>>;
    fn delete_use_case(&self, id: &EntityId) -> Result<()>;
    fn delete_use_case_multi(&self, ids: &[EntityId]) -> Result<()>;
    fn get_relationships_of(
        &self,
        field: &UseCaseRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>>;
    fn set_relationships(
        &self,
        field: &UseCaseRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<()>;
}

pub trait UseCaseUnitOfWorkROFactoryTrait {
    fn create(&self) -> Box<dyn UseCaseUnitOfWorkROTrait>;
}

pub trait UseCaseUnitOfWorkROTrait: QueryUnitOfWork {
    fn get_use_case(&self, id: &EntityId) -> Result<Option<UseCase>>;
    fn get_use_case_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<UseCase>>>;
}
