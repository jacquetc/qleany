use crate::entities::{File, FileNature, FileStatus};

pub struct FileListBuilder {
    files: Vec<File>,
}

impl FileListBuilder {
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    /// Add a file with the 5 essential fields. Returns `&mut File`
    /// so the caller can set optional fields directly.
    pub fn add(
        &mut self,
        name: impl Into<String>,
        relative_path: impl Into<String>,
        group: impl Into<String>,
        template_name: impl Into<String>,
        nature: FileNature,
    ) -> &mut File {
        let now = chrono::Utc::now();
        self.files.push(File {
            id: 0,
            created_at: now,
            updated_at: now,
            name: name.into(),
            relative_path: relative_path.into(),
            group: group.into(),
            template_name: template_name.into(),
            generated_code: None,
            status: FileStatus::Unknown,
            nature,
            feature: None,
            all_features: false,
            entity: None,
            all_entities: false,
            use_case: None,
            all_use_cases: false,
            field: None,
        });
        self.files.last_mut().unwrap()
    }

    /// Add multiple files that share the same path, group, and nature.
    /// Each tuple is (name, template_name).
    pub fn add_batch(
        &mut self,
        relative_path: &str,
        group: &str,
        nature: FileNature,
        entries: &[(&str, &str)],
    ) {
        for &(name, template_name) in entries {
            self.add(name, relative_path, group, template_name, nature.clone());
        }
    }

    /// Consume the builder and return the file list.
    pub fn build(self) -> Vec<File> {
        self.files
    }

    /// Consume the builder, applying a filter, and return filtered files.
    pub fn build_filtered(self, predicate: impl Fn(&File) -> bool) -> Vec<File> {
        self.files.into_iter().filter(predicate).collect()
    }
}

impl Default for FileListBuilder {
    fn default() -> Self {
        Self::new()
    }
}
