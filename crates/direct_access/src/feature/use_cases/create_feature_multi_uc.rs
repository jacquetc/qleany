use super::FeatureUnitOfWorkFactoryTrait;
use crate::feature::dtos::{CreateFeatureDto, FeatureDto};
use anyhow::{Ok, Result};
use common::entities::Feature;
use common::undo_redo::UndoRedoCommand;
use std::any::Any;
use std::collections::VecDeque;

pub struct CreateFeatureMultiUseCase {
    uow_factory: Box<dyn FeatureUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Vec<Feature>>,
    redo_stack: VecDeque<Vec<Feature>>,
}

impl CreateFeatureMultiUseCase {
    pub fn new(uow_factory: Box<dyn FeatureUnitOfWorkFactoryTrait>) -> Self {
        CreateFeatureMultiUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }
    pub fn execute(&mut self, dtos: &[CreateFeatureDto]) -> Result<Vec<FeatureDto>> {
        // create
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let features =
            uow.create_feature_multi(&dtos.iter().map(|dto| dto.into()).collect::<Vec<_>>())?;
        uow.commit()?;

        //store in undo stack
        self.undo_stack.push_back(features.clone());
        self.redo_stack.clear();

        Ok(features.into_iter().map(|feature| feature.into()).collect())
    }
}

impl UndoRedoCommand for CreateFeatureMultiUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_features) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.delete_feature_multi(
                &last_features
                    .iter()
                    .map(|feature| feature.id.clone())
                    .collect::<Vec<_>>(),
            )?;
            uow.commit()?;
            self.redo_stack.push_back(last_features);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(last_features) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.create_feature_multi(&last_features)?;
            uow.commit()?;
            self.undo_stack.push_back(last_features);
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
