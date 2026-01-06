use super::FeatureUnitOfWorkFactoryTrait;
use crate::FeatureRelationshipDto;
use anyhow::Result;
use common::types::Savepoint;
use common::undo_redo::UndoRedoCommand;
use std::any::Any;
use std::collections::VecDeque;

pub struct SetFeatureRelationshipUseCase {
    uow_factory: Box<dyn FeatureUnitOfWorkFactoryTrait>,
    undo_stack: VecDeque<Savepoint>,
    redo_stack: VecDeque<FeatureRelationshipDto>,
}

impl SetFeatureRelationshipUseCase {
    pub fn new(uow_factory: Box<dyn FeatureUnitOfWorkFactoryTrait>) -> Self {
        SetFeatureRelationshipUseCase {
            uow_factory,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, dto: &FeatureRelationshipDto) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let savepoint = uow.create_savepoint()?;
        uow.set_feature_relationship(&dto.id, &dto.field, dto.right_ids.as_slice())?;
        uow.commit()?;
        // store savepoint in undo stack
        self.undo_stack.push_back(savepoint);
        self.redo_stack.push_back(dto.clone());

        Ok(())
    }
}

impl UndoRedoCommand for SetFeatureRelationshipUseCase {
    fn undo(&mut self) -> Result<()> {
        if let Some(savepoint) = self.undo_stack.pop_back() {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            uow.restore_to_savepoint(savepoint)?;
            uow.commit()?;
        }
        anyhow::Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        if let Some(FeatureRelationshipDto {
            id,
            field,
            right_ids,
        }) = self.redo_stack.pop_back()
        {
            let mut uow = self.uow_factory.create();
            uow.begin_transaction()?;
            let savepoint = uow.create_savepoint()?;
            uow.set_feature_relationship(&id, &field, &right_ids)?;
            uow.commit()?;
            self.undo_stack.push_back(savepoint);
        }
        anyhow::Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
