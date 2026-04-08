use std::path::Path;
use std::sync::Arc;
use anyhow::Result;

use crate::services::FileSystemService;

pub struct SearchService {
    file_service: Arc<FileSystemService>,
}

impl SearchService {
    pub fn new(_index_path: &Path, file_service: Arc<FileSystemService>) -> Result<Self> {
        Ok(Self {
            file_service,
        })
    }
}
