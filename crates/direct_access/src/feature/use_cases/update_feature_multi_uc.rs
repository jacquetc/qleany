use super::FeatureUnitOfWorkFactoryTrait;
use crate::feature::dtos::FeatureDto;
use anyhow::{Ok, Result};
use common::{entities::Feature, undo_redo::UndoRedoCommand};
use std::any::Any;
use std::collections::VecDeque;

pub struct UpdateFeatureMultiUseCase {
    uow_factory: Box<dyn FeatureUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Vec<Feature>>,
    redo_stack: VecDeque<Vec<Feature>>,
}

impl UpdateFeatureMultiUseCase {
    pub fn new(uow_factory: Box<dyn FeatureUnitOfWorkFactoryTrait>) -> Self {
        UpdateFeatureMultiUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, dtos: &[FeatureDto]) -> Result<Vec<FeatureDto>> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // check if id exists
        let mut exists = true;
        for FeatureDto { id, .. } in dtos {
            if uow.get_feature(id)?.is_none() {
                exists = false;
                break;
            }
        }
        if !exists {
            return Err(anyhow::anyhow!("One or more ids do not exist"));
        }

        let features =
            uow.update_feature_multi(&dtos.iter().map(|dto| dto.into()).collect::<Vec<_>>())?;
        uow.commit()?;

        // store in undo stack
        self.undo_stack.push_back(features.clone());
        self.redo_stack.clear();

        Ok(features.into_iter().map(|feature| feature.into()).collect())
    }
}

impl UndoRedoCommand for UpdateFeatureMultiUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_features) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_feature_multi(&last_features)?;
            uow.commit()?;
            self.redo_stack.push_back(last_features);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(features) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.update_feature_multi(&features)?;
            uow.commit()?;
            self.undo_stack.push_back(features);
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
