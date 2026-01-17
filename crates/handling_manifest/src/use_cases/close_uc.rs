use anyhow::Result;
use common::database::CommandUnitOfWork;
use common::direct_access::system::SystemRelationshipField;
use common::entities::Workspace;
use common::types::EntityId;

pub trait CloseUnitOfWorkFactoryTrait {
    fn create(&self) -> Box<dyn CloseUnitOfWorkTrait>;
}

#[macros::uow_action(entity = "System", action = "GetRelationship")]
#[macros::uow_action(entity = "File", action = "DeleteMulti")]
#[macros::uow_action(entity = "Workspace", action = "GetMulti")]
#[macros::uow_action(entity = "Workspace", action = "DeleteMulti")]
pub trait CloseUnitOfWorkTrait: CommandUnitOfWork {}

pub struct CloseUseCase {
    uow_factory: Box<dyn CloseUnitOfWorkFactoryTrait>,
}

impl CloseUseCase {
    pub fn new(uow_factory: Box<dyn CloseUnitOfWorkFactoryTrait>) -> Self {
        CloseUseCase { uow_factory }
    }

    pub fn execute(&mut self) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // Get all workspaces
        let workspaces = uow.get_workspace_multi(&[])?;
        if workspaces.is_empty() {
            return Err(anyhow::anyhow!("No root found"));
        }

        // Remove the workspace
        uow.get_workspace_multi(&[])?;
        let workspace_ids: Vec<EntityId> = workspaces
            .iter()
            .filter_map(|w| w.as_ref().map(|ws| ws.id))
            .collect();
        uow.delete_workspace_multi(&workspace_ids)?;

        // Get all files
        let file_ids = uow.get_system_relationship(&1, &SystemRelationshipField::Files)?;
        uow.delete_file_multi(&file_ids)?;

        uow.commit()?;

        Ok(())
    }
}
