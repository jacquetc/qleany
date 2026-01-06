use super::FeatureUnitOfWorkROFactoryTrait;
use crate::feature::dtos::FeatureDto;
use anyhow::Result;
use common::types::EntityId;

pub struct GetFeatureUseCase {
    uow_factory: Box<dyn FeatureUnitOfWorkROFactoryTrait>,
}

impl GetFeatureUseCase {
    pub fn new(uow_factory: Box<dyn FeatureUnitOfWorkROFactoryTrait>) -> Self {
        GetFeatureUseCase { uow_factory }
    }

    pub fn execute(&self, id: &EntityId) -> Result<Option<FeatureDto>> {
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let feature_option = uow.get_feature(&id)?;
        uow.end_transaction()?;

        Ok(feature_option.map(|feature| feature.into()))
    }
}
