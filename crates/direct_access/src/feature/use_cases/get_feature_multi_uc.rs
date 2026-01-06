use super::FeatureUnitOfWorkROFactoryTrait;
use crate::feature::dtos::FeatureDto;
use anyhow::Result;
use common::types::EntityId;

pub struct GetFeatureMultiUseCase {
    uow_factory: Box<dyn FeatureUnitOfWorkROFactoryTrait>,
}

impl GetFeatureMultiUseCase {
    pub fn new(uow_factory: Box<dyn FeatureUnitOfWorkROFactoryTrait>) -> Self {
        GetFeatureMultiUseCase { uow_factory }
    }

    pub fn execute(&self, ids: &[EntityId]) -> Result<Vec<Option<FeatureDto>>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let features = uow.get_feature_multi(ids)?;
        uow.end_transaction()?;
        Ok(features
            .into_iter()
            .map(|feature| feature.map(|r| r.into()))
            .collect())
    }
}
