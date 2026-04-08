use std::path::PathBuf;
use tauri::State;
use anyhow::Result;

use crate::types::file_system::FileItem;
use crate::services::file_system::DiskUsage;
use crate::AppState;

#[tauri::command]
pub async fn list_directory(
    path: String,
    state: State<'_, AppState>,
) -> Result<Vec<FileItem>, String> {
    let path = PathBuf::from(path);
    
    state.file_service
        .list_directory(&path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_directory_recursive(
    path: String,
    max_depth: Option<usize>,
    state: State<'_, AppState>,
) -> Result<Vec<FileItem>, String> {
    let path = PathBuf::from(path);
    
    state.file_service
        .list_directory_recursive(&path, max_depth)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_file_info(
    path: String,
    state: State<'_, AppState>,
) -> Result<FileItem, String> {
    let path = PathBuf::from(path);
    
    state.file_service
        .get_file_info(&path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn copy_files(
    sources: Vec<String>,
    destination: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let sources: Vec<PathBuf> = sources.into_iter().map(PathBuf::from).collect();
    let destination = PathBuf::from(destination);
    
    state.file_service
        .copy_files(sources, destination)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn move_files(
    sources: Vec<String>,
    destination: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let sources: Vec<PathBuf> = sources.into_iter().map(PathBuf::from).collect();
    let destination = PathBuf::from(destination);
    
    state.file_service
        .move_files(sources, destination)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_files(
    paths: Vec<String>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let paths: Vec<PathBuf> = paths.into_iter().map(PathBuf::from).collect();
    
    state.file_service
        .delete_files(paths)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_directory(
    path: String,
    state: State<'_, AppState>,
) -> Result<FileItem, String> {
    let path = PathBuf::from(path);
    
    state.file_service
        .create_directory(&path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn rename_file(
    old_path: String,
    new_path: String,
    state: State<'_, AppState>,
) -> Result<FileItem, String> {
    let old_path = PathBuf::from(old_path);
    let new_path = PathBuf::from(new_path);
    
    state.file_service
        .rename_file(&old_path, &new_path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_disk_usage(
    path: String,
    state: State<'_, AppState>,
) -> Result<DiskUsage, String> {
    let path = PathBuf::from(path);
    
    state.file_service
        .get_disk_usage(&path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_operation_status(
    operation_id: String,
    state: State<'_, AppState>,
) -> Result<Option<crate::types::file_system::FileOperation>, String> {
    Ok(state.file_service
        .get_operation_status(&operation_id))
}

#[tauri::command]
pub async fn open_file(
    path: String,
) -> Result<(), String> {
    let path = PathBuf::from(path);
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

#[tauri::command]
pub async fn show_in_explorer(
    path: String,
) -> Result<(), String> {
    let path = PathBuf::from(path);
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg("/select,")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg("-R")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "linux")]
    {
        // Try to open the parent directory
        if let Some(parent) = path.parent() {
            std::process::Command::new("xdg-open")
                .arg(parent)
                .spawn()
                .map_err(|e| e.to_string())?;
        }
    }
    
    Ok(())
}
