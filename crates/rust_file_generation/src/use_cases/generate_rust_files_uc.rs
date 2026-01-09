use crate::use_cases::common::rust_code_generator::{
    GenerationReadOps, GenerationSnapshot, SnapshotBuilder, generate_code_with_snapshot,
};
use crate::use_cases::common::rust_formatter::rustfmt_files_batch;
use crate::{GenerateRustFilesDto, GenerateRustFilesReturnDto};
use anyhow::{Result, anyhow};
use common::entities::{File, Global, Root};
use common::long_operation::LongOperation;
use common::types::EntityId;
use std::path::PathBuf;
use std::sync::Arc;

pub trait GenerateRustFilesUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn GenerateRustFilesUnitOfWorkTrait>;
}

#[macros::uow_action(entity = "Root", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Root", action = "GetMultiRO")]
#[macros::uow_action(entity = "Global", action = "GetMultiRO")]
pub trait GenerateRustFilesUnitOfWorkTrait: GenerationReadOps + Send + Sync {}

pub struct GenerateRustFilesUseCase {
    uow_factory: Box<dyn GenerateRustFilesUnitOfWorkFactoryTrait>,
    dto: GenerateRustFilesDto,
}

impl GenerateRustFilesUseCase {
    pub fn new(
        uow_factory: Box<dyn GenerateRustFilesUnitOfWorkFactoryTrait>,
        dto: &GenerateRustFilesDto,
    ) -> Self {
        GenerateRustFilesUseCase {
            uow_factory,
            dto: dto.clone(),
        }
    }
}
impl LongOperation for GenerateRustFilesUseCase {
    type Output = GenerateRustFilesReturnDto;

    fn execute(
        &self,
        progress_callback: Box<dyn Fn(common::long_operation::OperationProgress) + Send>,
        cancel_flag: Arc<std::sync::atomic::AtomicBool>,
    ) -> Result<Self::Output> {
        use std::fs;
        use std::sync::atomic::Ordering;

        let start_time = std::time::Instant::now();
        let timestamp = chrono::Utc::now();
        let total = self.dto.file_ids.len().max(1); // avoid div by zero
        let prefix_path = if self.dto.prefix.is_empty() {
            PathBuf::new()
        } else {
            PathBuf::from(&self.dto.prefix)
        };

        progress_callback(common::long_operation::OperationProgress::new(
            0.0,
            Some("Starting Rust file generation...".to_string()),
        ));

        // Create UoW and open a read transaction for snapshot building
        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let uow_read: &dyn GenerationReadOps = &*uow;

        let mut written_files: Vec<String> = Vec::new();
        let mut rust_files_to_format: Vec<PathBuf> = Vec::new();

        let root_path: PathBuf = if self.dto.root_path.is_empty() || self.dto.root_path == "." {
            let manifest_absolute_path = uow_read
                .get_root_multi(&vec![])?
                .into_iter()
                .filter_map(|r| r)
                .next()
                .ok_or_else(|| anyhow!("Root entity not found"))?
                .manifest_absolute_path;
            PathBuf::from(manifest_absolute_path)
        } else {
            PathBuf::from(&self.dto.root_path)
        };

        println!(
            "Generating Rust files to root path: {}, with prefix: {}",
            root_path.display(),
            prefix_path.display()
        );

        // create a cache for GenerationSnapshot if needed in the future
        let mut generation_snapshot_cache: Vec<GenerationSnapshot> = Vec::new();
        generation_snapshot_cache.reserve(self.dto.file_ids.len());

        for (idx, file_id) in self.dto.file_ids.iter().enumerate() {
            if cancel_flag.load(Ordering::Relaxed) {
                uow.end_transaction()?;
                return Err(anyhow!("Operation was cancelled"));
            }

            // Load file metadata (name, relative path)
            let file_meta: File = uow_read
                .get_file(file_id)?
                .ok_or_else(|| anyhow!("File not found"))?;
            println!("Processing file ID {}: {}", file_id, file_meta.name);

            // Build snapshot and generate code for the file
            let (snapshot, from_cache) =
                SnapshotBuilder::for_file(uow_read, *file_id, &generation_snapshot_cache)?;
            let mut code = generate_code_with_snapshot(&snapshot)?;
            if !from_cache {
                generation_snapshot_cache.push(snapshot);
            }

            let file_name = &file_meta.name;
            println!("Generated code for file {}:\n{}", file_name, "code omitted");

            // Compute destination path: root_path/prefix/relative_path/name
            let mut out_dir = root_path.clone();
            if !prefix_path.as_os_str().is_empty() {
                out_dir = out_dir.join(&prefix_path);
            }
            if !file_meta.relative_path.is_empty() {
                out_dir = out_dir.join(&file_meta.relative_path);
            }

            fs::create_dir_all(&out_dir)?;
            let out_path = out_dir.join(file_name);

            // Collect Rust source files for batch formatting later
            if file_name.ends_with(".rs") {
                rust_files_to_format.push(out_path.clone());
            }

            // Write file content
            fs::write(&out_path, code.as_bytes())?;
            // ensure that the file was written
            if !out_path.exists() {
                return Err(anyhow!("Failed to write file: {}", out_path.display()));
            }

            // Record written file path as string
            if let Some(s) = out_path.to_str() {
                written_files.push(s.to_string());
            } else {
                written_files.push(out_path.display().to_string());
            }

            // Progress update
            let percentage = ((idx + 1) as f32 / total as f32) * 100.0;
            let rel_display = format!(
                "{}{}{}",
                self.dto.prefix,
                if self.dto.prefix.is_empty() || file_meta.relative_path.is_empty() {
                    ""
                } else {
                    "/"
                },
                format!(
                    "{}{}{}",
                    file_meta.relative_path,
                    if file_meta.relative_path.is_empty() {
                        ""
                    } else {
                        "/"
                    },
                    file_name
                )
            );
            progress_callback(common::long_operation::OperationProgress::new(
                percentage,
                Some(format!("Generated {}/{}: {}", idx + 1, total, rel_display)),
            ));
        }

        uow.end_transaction()?;

        // Batch format all Rust files at once (much faster than per-file formatting)
        if !rust_files_to_format.is_empty() {
            progress_callback(common::long_operation::OperationProgress::new(
                99.0,
                Some(format!(
                    "Formatting {} Rust files...",
                    rust_files_to_format.len()
                )),
            ));
            rustfmt_files_batch(&rust_files_to_format);
        }

        // Final progress
        progress_callback(common::long_operation::OperationProgress::new(
            100.0,
            Some("Rust file generation completed".to_string()),
        ));

        let duration = start_time.elapsed();
        println!(
            "Rust file generation completed in {:?}, total files written: {}",
            duration,
            written_files.len()
        );
        Ok(GenerateRustFilesReturnDto {
            files: written_files,
            timestamp: timestamp.to_string(),
            duration: format!("{:?}", duration),
        })
    }
}
