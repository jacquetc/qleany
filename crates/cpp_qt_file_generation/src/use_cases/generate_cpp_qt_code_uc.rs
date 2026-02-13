use crate::use_cases::common::cpp_qt_code_generator::{
    GenerationReadOps, SnapshotBuilder, generate_code_with_snapshot,
};
use crate::use_cases::common::cpp_qt_formatter::clang_format_string;
use crate::{GenerateCppQtCodeDto, GenerateCppQtCodeReturnDto};
use anyhow::Result;

pub trait GenerateCppQtCodeUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn GenerateCppQtCodeUnitOfWorkTrait>;
}

// Code UoW must provide at least the read-ops required by snapshot builder
pub trait GenerateCppQtCodeUnitOfWorkTrait: GenerationReadOps {}

pub struct GenerateCppQtCodeUseCase {
    uow_factory: Box<dyn GenerateCppQtCodeUnitOfWorkFactoryTrait>,
}

impl GenerateCppQtCodeUseCase {
    pub fn new(uow_factory: Box<dyn GenerateCppQtCodeUnitOfWorkFactoryTrait>) -> Self {
        GenerateCppQtCodeUseCase { uow_factory }
    }
}
impl GenerateCppQtCodeUseCase {
    pub(crate) fn execute(&self, dto: &GenerateCppQtCodeDto) -> Result<GenerateCppQtCodeReturnDto> {
        let timestamp = chrono::Utc::now();

        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        // Build a snapshot for the file
        let uow_ref: &dyn GenerationReadOps = &*uow;
        let (snapshot, _from_cache) = SnapshotBuilder::for_file(uow_ref, dto.file_id, &Vec::new())?;
        uow.end_transaction()?;

        let generated_code = generate_code_with_snapshot(&snapshot)?;

        let formatted_code = clang_format_string(generated_code.as_str(), None);

        Ok(GenerateCppQtCodeReturnDto {
            generated_code: formatted_code.to_string(),
            timestamp: timestamp.to_string(),
        })
    }
}
