use crate::use_cases::common::rust_code_generator::GenerationReadOps;
use common::entities::Workspace;
use common::types::EntityId;

pub fn get_workspace_id(uow: &dyn GenerationReadOps) -> anyhow::Result<EntityId> {
    use anyhow::anyhow;
    let roots = uow.get_root_multi(&[])?;
    let root = roots
        .into_iter()
        .flatten()
        .next()
        .ok_or_else(|| anyhow!("Root entity not found"))?;

    let all_workspace_ids = uow.get_root_relationship(
        &root.id,
        &common::direct_access::root::RootRelationshipField::Workspace,
    )?;

    let workspace_id = all_workspace_ids
        .first()
        .cloned()
        .ok_or(anyhow!("No workspace found"))?;
    Ok(workspace_id)
}

pub fn get_workspace(uow: &dyn GenerationReadOps) -> anyhow::Result<Workspace> {
    use anyhow::anyhow;
    let roots = uow.get_root_multi(&[])?;
    let root = roots
        .into_iter()
        .flatten()
        .next()
        .ok_or_else(|| anyhow!("Root entity not found"))?;

    let all_workspace_ids = uow.get_root_relationship(
        &root.id,
        &common::direct_access::root::RootRelationshipField::Workspace,
    )?;

    let workspace_id = all_workspace_ids
        .first()
        .cloned()
        .ok_or(anyhow!("No workspace found"))?;

    let workspace = uow
        .get_workspace(&workspace_id)?
        .ok_or_else(|| anyhow!("Workspace entity not found"))?;
    Ok(workspace)
}

pub fn strip_leading_and_trailing_slashes(path: &str) -> String {
    let trimmed = path.trim_matches(|c: char| c == '/' || c == '\\' || c.is_whitespace());
    trimmed.to_string()
}
/// Transforms an English word to its plural form following English language rules.
pub fn to_plural(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }

    // Special cases and irregular plurals could be added here

    // Words ending in 'y' preceded by a consonant: change 'y' to 'ies'
    if word.ends_with('y') && word.len() > 1 {
        let second_last = word.chars().nth(word.len() - 2).unwrap();
        if !"aeiou".contains(second_last) {
            return format!("{}ies", &word[..word.len() - 1]);
        }
    }

    // Words ending in 's', 'x', 'z', 'ch', 'sh': add 'es'
    if word.ends_with('s')
        || word.ends_with('x')
        || word.ends_with('z')
        || word.ends_with("ch")
        || word.ends_with("sh")
    {
        return format!("{}es", word);
    }

    // Default case: add 's'
    format!("{}s", word)
}
