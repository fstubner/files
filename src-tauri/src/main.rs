#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use std::path::PathBuf;
use tauri::Manager;
use tracing_subscriber;

mod types;
mod services;
mod commands;

use services::{FileSystemService, SearchService};

#[derive(Clone)]
pub struct AppState {
    pub file_service: Arc<FileSystemService>,
    pub search_service: Arc<SearchService>,
}

fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            tauri::async_runtime::spawn(async move {
                // Initialize services
                let file_service = Arc::new(FileSystemService::new());
                
                // Create search index in app data directory
                let app_data_dir = handle.path_resolver()
                    .app_data_dir()
                    .unwrap_or_else(|| PathBuf::from("./data"));
                
                std::fs::create_dir_all(&app_data_dir).unwrap();
                let index_path = app_data_dir.join("search_index");
                
                let search_service = Arc::new(
                    SearchService::new(&index_path, file_service.clone())
                        .await.expect("Failed to initialize search service")
                );

                let app_state = AppState {
                    file_service,
                    search_service,
                };

                handle.manage(app_state);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // File operations
            commands::list_directory,
            commands::list_directory_recursive,
            commands::get_file_info,
            commands::copy_files,
            commands::move_files,
            commands::delete_files,
            commands::create_directory,
            commands::rename_file,
            commands::get_disk_usage,
            commands::get_operation_status,
            commands::open_file,
            commands::show_in_explorer,
            
            // Search operations
            commands::search_files,
            commands::index_directory,
            commands::reindex_file,
            commands::remove_from_index,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
