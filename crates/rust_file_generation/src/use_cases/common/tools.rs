use common::entities::Workspace;
use common::types::EntityId;
use crate::use_cases::common::rust_code_generator::GenerationReadOps;

pub fn get_workspace_id(uow: &dyn GenerationReadOps) -> anyhow::Result<EntityId> {
    use anyhow::anyhow;
    let roots = uow.get_root_multi(&vec![])?;
    let root = roots
        .into_iter()
        .filter_map(|r| r)
        .next()
        .ok_or_else(|| anyhow!("Root entity not found"))?;

    let all_workspace_ids = uow.get_root_relationship(
        &root.id,
        &common::direct_access::root::RootRelationshipField::Workspace
    )?;

    let workspace_id = all_workspace_ids.first().cloned().ok_or(anyhow!("No workspace found"))?;
    Ok(workspace_id)
}


pub fn get_workspace(uow: &dyn GenerationReadOps) -> anyhow::Result<Workspace> {
    use anyhow::anyhow;
    let roots = uow.get_root_multi(&vec![])?;
    let root = roots
        .into_iter()
        .filter_map(|r| r)
        .next()
        .ok_or_else(|| anyhow!("Root entity not found"))?;

    let all_workspace_ids = uow.get_root_relationship(
        &root.id,
        &common::direct_access::root::RootRelationshipField::Workspace
    )?;

    let workspace_id = all_workspace_ids.first().cloned().ok_or(anyhow!("No workspace found"))?;
    
    let workspace = uow.get_workspace(&workspace_id)?
        .ok_or_else(|| anyhow!("Workspace entity not found"))?;
    Ok(workspace)
}

