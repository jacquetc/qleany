use crate::app_context::AppContext;
use anyhow::Result;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
pub enum TargetLanguage {
    Rust,
    CppQt,
}
pub fn get_target_language(app_context: &Arc<AppContext>) -> Result<TargetLanguage> {
    use direct_access::global_controller;

    let global_dtos = global_controller::get_multi(&app_context.db_context, &[])?;
    let global_dto = global_dtos
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No global configuration found"))?;

    let global_dto = global_dto.ok_or_else(|| anyhow::anyhow!("Global configuration not found"))?;

    Ok(match global_dto.language.as_str() {
        "cpp-qt" => TargetLanguage::CppQt,
        "rust" => TargetLanguage::Rust,
        _ => anyhow::bail!("Unsupported language: {}", global_dto.language),
    })
}
