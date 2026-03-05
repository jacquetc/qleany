use crate::app_context::AppContext;
use crate::cli::OutputContext;
use anyhow::Result;
use handling_manifest::handling_manifest_controller;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
pub enum TargetLanguage {
    Rust,
    CppQt,
}
pub fn get_target_language(app_context: &Arc<AppContext>) -> Result<TargetLanguage> {
    use direct_access::global_controller;

    let global_dtos = global_controller::get_all(&app_context.db_context)?;
    let global_dto = global_dtos
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No global configuration found"))?;

    Ok(match global_dto.language.as_str() {
        "cpp-qt" => TargetLanguage::CppQt,
        "rust" => TargetLanguage::Rust,
        _ => anyhow::bail!("Unsupported language: {}", global_dto.language),
    })
}

/// Run semantic checks on the loaded manifest. Prints warnings/errors and
/// returns an error if any critical errors are found.
pub fn run_checks(app_context: &Arc<AppContext>, output: &OutputContext) -> Result<()> {
    let check_result =
        handling_manifest_controller::check(&app_context.db_context, &app_context.event_hub)?;

    for warning in &check_result.warnings {
        output.warn(&format!("Warning: {}", warning));
    }
    for error in &check_result.critical_errors {
        eprintln!("✗ Error: {}", error);
    }

    if !check_result.critical_errors.is_empty() {
        anyhow::bail!(
            "Manifest has {} critical error(s)",
            check_result.critical_errors.len()
        );
    }

    Ok(())
}
