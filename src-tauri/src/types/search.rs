use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::types::file_system::FileItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub text: String,
    pub mode: SearchMode,
    pub filters: SearchFilters,
    pub sort_by: SortOption,
    pub limit: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchMode {
    Simple,
    Regex,
    Semantic,
    Advanced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilters {
    pub file_types: Option<Vec<String>>,
    pub size_range: Option<SizeRange>,
    pub date_range: Option<DateRange>,
    pub tags: Option<TagFilter>,
    pub path: Option<String>,
    pub content: Option<bool>,
    pub extensions: Option<Vec<String>>,
    pub mime_types: Option<Vec<String>>,
    pub hidden: Option<bool>,
    pub readonly: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeRange {
    pub min: Option<u64>,
    pub max: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagFilter {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
    pub logic: TagLogic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TagLogic {
    And,
    Or,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortOption {
    pub field: SortField,
    pub order: SortOrder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortField {
    Name,
    Size,
    Modified,
    Created,
    Type,
    Relevance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub file: FileItem,
    pub score: f32,
    pub highlights: Vec<Highlight>,
    pub matched_fields: Vec<String>,
    pub snippet: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Highlight {
    pub field: String,
    pub text: String,
    pub start: usize,
    pub end: usize,
}
