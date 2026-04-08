use std::path::Path;
use std::sync::Arc;
use anyhow::{Result, Context};
use tokio::sync::RwLock;

use crate::types::search::{SearchQuery, SearchResult};
use crate::types::file_system::FileItem;
use crate::services::FileSystemService;

pub struct SearchService {
    fs_service: Arc<FileSystemService>,
}

impl SearchService {
    pub async fn new(_index_path: &Path, fs_service: Arc<FileSystemService>) -> Result<Self> {
        Ok(SearchService {
            fs_service,
        })
    }

    pub async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        Ok(vec![])
    }
}
