use crate::use_cases::common::rust_code_generator::{
    GenerationReadOps, SnapshotBuilder, generate_code_with_snapshot,
};
use crate::use_cases::common::rust_formatter::rustfmt_string;
use crate::{GenerateRustCodeDto, GenerateRustCodeReturnDto};
use anyhow::Result;
use common::database::QueryUnitOfWork;
pub trait GenerateRustCodeUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn GenerateRustCodeUnitOfWorkTrait>;
}

// Code UoW must provide at least the read-ops required by snapshot builder
pub trait GenerateRustCodeUnitOfWorkTrait: GenerationReadOps {}

pub struct GenerateRustCodeUseCase {
    uow_factory: Box<dyn GenerateRustCodeUnitOfWorkFactoryTrait>,
}

impl GenerateRustCodeUseCase {
    pub fn new(uow_factory: Box<dyn GenerateRustCodeUnitOfWorkFactoryTrait>) -> Self {
        GenerateRustCodeUseCase { uow_factory }
    }
}
impl GenerateRustCodeUseCase {
    pub(crate) fn execute(&self, dto: &GenerateRustCodeDto) -> Result<GenerateRustCodeReturnDto> {
        let timestamp = chrono::Utc::now();

        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        // Build a snapshot for the file
        let uow_ref: &dyn GenerationReadOps = &*uow;
        let snapshot = SnapshotBuilder::for_file(uow_ref, dto.file_id)?;
        uow.end_transaction()?;

        let generated_code = generate_code_with_snapshot(&snapshot)?;

        let formatted_code = rustfmt_string(generated_code.as_str(), None);

        Ok(GenerateRustCodeReturnDto {
            generated_code: formatted_code.to_string(),
            timestamp: timestamp.to_string(),
        })
    }
}
