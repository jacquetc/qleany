use super::FeatureUnitOfWorkFactoryTrait;
use crate::feature::dtos::{CreateFeatureDto, FeatureDto};
use anyhow::{Ok, Result};
use common::entities::Feature;
use common::undo_redo::UndoRedoCommand;
use std::collections::VecDeque;

pub struct CreateFeatureUseCase {
    uow_factory: Box<dyn FeatureUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Feature>,
    redo_stack: VecDeque<Feature>,
}

impl CreateFeatureUseCase {
    pub fn new(uow_factory: Box<dyn FeatureUnitOfWorkFactoryTrait>) -> Self {
        CreateFeatureUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, dto: CreateFeatureDto) -> Result<FeatureDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let feature = uow.create_feature(&dto.into())?;
        uow.commit()?;

        // store in undo stack
        self.undo_stack.push_back(feature.clone());
        self.redo_stack.clear();

        Ok(feature.into())
    }
}

impl UndoRedoCommand for CreateFeatureUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(last_feature) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.delete_feature(&last_feature.id)?;
            uow.commit()?;
            self.redo_stack.push_back(last_feature);
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(last_feature) = self.redo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.create_feature(&last_feature)?;
            uow.commit()?;
            self.undo_stack.push_back(last_feature);
        }
        Ok(())
    }
}
