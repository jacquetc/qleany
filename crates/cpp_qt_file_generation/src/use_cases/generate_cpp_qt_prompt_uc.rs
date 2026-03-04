use crate::GenerateCppQtPromptDto;
use crate::GenerateCppQtPromptReturnDto;
use crate::use_cases::common::cpp_qt_code_generator::{GenerationReadOps, SnapshotBuilder};
use anyhow::Result;
use common::entities::{File, FileStatus};
use tera::{Context, Tera};

const CONTEXT_TEMPLATE: &str = include_str!("generate_cpp_qt_prompt_uc/cpp_qt_context.tera");
const USE_CASE_PROMPT_TEMPLATE: &str =
    include_str!("generate_cpp_qt_prompt_uc/cpp_qt_use_case_prompt.tera");

pub trait GenerateCppQtPromptUnitOfWorkFactoryTrait {
    fn create(&self) -> Box<dyn GenerateCppQtPromptUnitOfWorkTrait>;
}

// Code UoW must provide at least the read-ops required by snapshot builder
pub trait GenerateCppQtPromptUnitOfWorkTrait: GenerationReadOps {}

pub struct GenerateCppQtPromptUseCase {
    uow_factory: Box<dyn GenerateCppQtPromptUnitOfWorkFactoryTrait>,
}

impl GenerateCppQtPromptUseCase {
    pub fn new(uow_factory: Box<dyn GenerateCppQtPromptUnitOfWorkFactoryTrait>) -> Self {
        GenerateCppQtPromptUseCase { uow_factory }
    }

    pub fn execute(
        &mut self,
        dto: &GenerateCppQtPromptDto,
    ) -> Result<GenerateCppQtPromptReturnDto> {
        let (template_name, file) = if dto.context {
            // Context mode: load all features and all entities for a broad overview
            (
                "cpp_qt_context",
                File {
                    id: 0,
                    name: "context".to_string(),
                    template_name: "cpp_qt_context".to_string(),
                    status: FileStatus::Unknown,
                    feature: Some(0),
                    entity: Some(0),
                    ..Default::default()
                },
            )
        } else {
            // Use case prompt mode: load specific feature and use case
            (
                "cpp_qt_use_case_prompt",
                File {
                    id: 0,
                    name: "use_case_prompt".to_string(),
                    template_name: "cpp_qt_use_case_prompt".to_string(),
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
        tera.add_raw_template("cpp_qt_context", CONTEXT_TEMPLATE)?;
        tera.add_raw_template("cpp_qt_use_case_prompt", USE_CASE_PROMPT_TEMPLATE)?;

        let mut context = Context::new();
        context.insert("s", &snapshot);
        let prompt_text = tera.render(template_name, &context)?;

        uow.end_transaction()?;

        Ok(GenerateCppQtPromptReturnDto { prompt_text })
    }
}
