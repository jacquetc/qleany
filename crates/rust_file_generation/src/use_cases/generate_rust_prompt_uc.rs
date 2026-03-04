use crate::GenerateRustPromptDto;
use crate::GenerateRustPromptReturnDto;
use anyhow::Result;
use common::entities::{File, FileStatus};
use crate::use_cases::common::rust_code_generator::{GenerationReadOps, SnapshotBuilder};
use tera::{Context, Tera};

const CONTEXT_TEMPLATE: &str = include_str!("generate_rust_prompt_uc/rust_context.tera");
const USE_CASE_PROMPT_TEMPLATE: &str =
    include_str!("generate_rust_prompt_uc/rust_use_case_prompt.tera");

pub trait GenerateRustPromptUnitOfWorkFactoryTrait {
    fn create(&self) -> Box<dyn GenerateRustPromptUnitOfWorkTrait>;
}

// Prompt UoW must provide at least the read-ops required by snapshot builder
pub trait GenerateRustPromptUnitOfWorkTrait: GenerationReadOps {}

pub struct GenerateRustPromptUseCase {
    uow_factory: Box<dyn GenerateRustPromptUnitOfWorkFactoryTrait>,
}

impl GenerateRustPromptUseCase {
    pub fn new(uow_factory: Box<dyn GenerateRustPromptUnitOfWorkFactoryTrait>) -> Self {
        GenerateRustPromptUseCase { uow_factory }
    }

    pub fn execute(
        &mut self,
        dto: &GenerateRustPromptDto,
    ) -> Result<GenerateRustPromptReturnDto> {
        let (template_name, file) = if dto.context {
            // Context mode: load all features and all entities for a broad overview
            (
                "rust_context",
                File {
                    id: 0,
                    name: "context".to_string(),
                    template_name: "rust_context".to_string(),
                    status: FileStatus::Unknown,
                    feature: Some(0),
                    entity: Some(0),
                    ..Default::default()
                },
            )
        } else {
            // Use case prompt mode: load specific feature and use case
            (
                "rust_use_case_prompt",
                File {
                    id: 0,
                    name: "use_case_prompt".to_string(),
                    template_name: "rust_use_case_prompt".to_string(),
                    status: FileStatus::Unknown,
                    feature: dto.feature_id,
                    use_case: dto.use_case_id,
                    ..Default::default()
                },
            )
        };

        let uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // Build a snapshot for the synthetic file
        let uow_ref: &dyn GenerationReadOps = &*uow;
        let (snapshot, _) = SnapshotBuilder::for_file(uow_ref, &file, &Vec::new())?;

        // Render using embedded templates (not in common/templates/)
        let mut tera = Tera::default();
        tera.add_raw_template("rust_context", CONTEXT_TEMPLATE)?;
        tera.add_raw_template("rust_use_case_prompt", USE_CASE_PROMPT_TEMPLATE)?;

        let mut context = Context::new();
        context.insert("s", &snapshot);
        let prompt_text = tera.render(template_name, &context)?;

        uow.end_transaction()?;

        Ok(GenerateRustPromptReturnDto { prompt_text })
    }
}
