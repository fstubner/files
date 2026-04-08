use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::sync::{Arc, Mutex};
use walkdir::WalkDir;
use mime_guess::from_path;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use anyhow::{Result, Context};

use crate::types::file_system::{
    FileItem, FileType, FilePermissions, FileMetadata,
    FileOperation, OperationType, OperationStatus
};

#[derive(Debug)]
pub struct FileSystemService {
    operation_cache: Arc<Mutex<HashMap<String, FileOperation>>>,
}

impl FileSystemService {
    pub fn new() -> Self {
        Self {
            operation_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn list_directory(&self, path: &Path) -> Result<Vec<FileItem>> {
        let entries = fs::read_dir(path)
            .with_context(|| format!("Failed to read directory: {}", path.display()))?;

        let mut files = Vec::new();
        
        for entry in entries {
            let entry = entry.with_context(|| "Failed to read directory entry")?;
            let file_item = self.create_file_item(entry.path()).await?;
            files.push(file_item);
        }

        files.sort_by(|a, b| a.name.cmp(&b.name));
        
        Ok(files)
    }

    pub async fn list_directory_recursive(&self, path: &Path, max_depth: Option<usize>) -> Result<Vec<FileItem>> {
        let walker = match max_depth {
            Some(depth) => WalkDir::new(path).max_depth(depth),
            None => WalkDir::new(path),
        };

        let mut files = Vec::new();
        for entry in walker {
            let entry = entry.with_context(|| "Failed to walk directory")?;
            let file_item = self.create_file_item(entry.path().to_path_buf()).await?;
            files.push(file_item);
        }

        Ok(files)
    }
}
