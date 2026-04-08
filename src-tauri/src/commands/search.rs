use std::path::PathBuf;
use tauri::State;

use crate::types::search::{SearchQuery, SearchResult};
use crate::AppState;

#[tauri::command]
pub async fn search_files(
    query: SearchQuery,
    state: State<'_, AppState>,
) -> Result<Vec<SearchResult>, String> {
    state.search_service
        .search(&query)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn index_directory(
    path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let path = PathBuf::from(path);
    
    state.search_service
        .index_directory(&path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn reindex_file(
    path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let path = PathBuf::from(path);
    
    // Get file info and index it
    let file = state.file_service
        .get_file_info(&path)
        .await
        .map_err(|e| e.to_string())?;
    
    state.search_service
        .index_file(&file)
        .await
        .map_err(|e| e.to_string())?;
    
    state.search_service
        .commit()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_from_index(
    path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let path = PathBuf::from(path);
    
    state.search_service
        .remove_file(&path.to_string_lossy())
        .await
        .map_err(|e| e.to_string())?;
    
    state.search_service
        .commit()
        .await
        .map_err(|e| e.to_string())
}
