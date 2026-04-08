use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileItem {
    pub id: String,
    pub path: PathBuf,
    pub name: String,
    pub file_type: FileType,
    pub size: u64,
    pub modified: DateTime<Utc>,
    pub created: DateTime<Utc>,
    pub permissions: FilePermissions,
    pub is_hidden: bool,
    pub is_readonly: bool,
    pub mime_type: Option<String>,
    pub extension: Option<String>,
    pub tags: Vec<String>,
    pub metadata: FileMetadata,
    pub cloud_provider: Option<CloudProvider>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FileType {
    File,
    Directory,
    Symlink,
    Cloud,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePermissions {
    pub owner: String,
    pub group: String,
    pub mode: u32,
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration: Option<f64>,
    pub bitrate: Option<u32>,
    pub codec: Option<String>,
    pub pages: Option<u32>,
    pub language: Option<String>,
    pub ai_generated: Option<bool>,
    pub confidence: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudProvider {
    pub name: CloudProviderName,
    pub id: String,
    pub path: String,
    pub sync_status: SyncStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CloudProviderName {
    GoogleDrive,
    Dropbox,
    OneDrive,
    ICloud,
    Box,
    S3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncStatus {
    Synced,
    Syncing,
    Conflict,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperation {
    pub id: String,
    pub operation_type: OperationType,
    pub source: Vec<PathBuf>,
    pub destination: Option<PathBuf>,
    pub status: OperationStatus,
    pub progress: f32,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OperationType {
    Copy,
    Move,
    Delete,
    CreateDirectory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OperationStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}
