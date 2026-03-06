use anyhow::Result;
use common::database::CommandUnitOfWork;
use common::direct_access::system::SystemRelationshipField;
use common::entities::Workspace;
use common::types::EntityId;

pub trait CloseUnitOfWorkFactoryTrait {
    fn create(&self) -> Box<dyn CloseUnitOfWorkTrait>;
}

#[macros::uow_action(entity = "System", action = "GetRelationship")]
#[macros::uow_action(entity = "File", action = "RemoveMulti")]
#[macros::uow_action(entity = "Workspace", action = "GetAll")]
#[macros::uow_action(entity = "Workspace", action = "RemoveMulti")]
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
        let workspaces = uow.get_all_workspace()?;
        if workspaces.is_empty() {
            return Err(anyhow::anyhow!("No root found"));
        }

        // Remove the workspace
        uow.get_all_workspace()?;
        let workspace_ids: Vec<EntityId> =
            workspaces.iter().map(|workspace| workspace.id).collect();
        uow.remove_workspace_multi(&workspace_ids)?;

        // Get all files
        let file_ids = uow.get_system_relationship(&1, &SystemRelationshipField::Files)?;
        uow.remove_file_multi(&file_ids)?;

        uow.commit()?;

        Ok(())
    }
}
