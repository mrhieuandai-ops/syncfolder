//! Main entry point for SyncFolder Tauri application
//! 
//! Desktop file synchronization app for Windows 11

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tauri::Manager;

mod commands;
mod models;
mod errors;
mod repositories;
mod services;
pub mod state;
pub mod events;

use services::persistence::Database;
use repositories::{ProfilesRepository, JobsRepository};
use commands::profiles::{create_profile, get_profile, list_profiles, update_profile, delete_profile};
use services::scheduler::job_scheduler::{JobScheduler, start_scheduler_task};
use state::AppState;

fn main() {
    // Initialize logging FIRST
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();
    
    log::info!("=== SyncFolder starting ===");
    log::info!("Build info: debug={}", cfg!(debug_assertions));

    // Set panic hook to log panics
    std::panic::set_hook(Box::new(|panic_info| {
        log::error!("PANIC: {}", panic_info);
        eprintln!("PANIC: {}", panic_info);
    }));

    log::info!("Main function starting...");

    // Build and run Tauri application
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            log::info!("Setup phase starting...");
            
            // Get app data directory for SQLite database
            let app_data_dir = app.path().app_data_dir()
                .expect("Failed to get app data directory");
            
            log::info!("App data dir: {:?}", app_data_dir);
            
            // Create directory if it doesn't exist
            std::fs::create_dir_all(&app_data_dir)
                .expect("Failed to create app data directory");
            
            let db_path = app_data_dir.join("syncfolder.db");
            log::info!("Database path: {:?}", db_path);
            
            // Initialize database
            log::info!("Initializing database...");
            let db = Database::new(&db_path)
                .expect("Failed to initialize database");
            let db = Arc::new(db);
            log::info!("Database initialized successfully");

            // Store state
            log::info!("Managing app state...");
            app.manage(AppState { 
                db: db.clone(),
            });
            
            // Initialize repositories and scheduler
            log::info!("Initializing repositories...");
            let profiles_repo = Arc::new(ProfilesRepository::new(db.clone()));
            let jobs_repo = Arc::new(JobsRepository::new(db.clone()));
            
            log::info!("Initializing scheduler...");
            let scheduler = Arc::new(JobScheduler::new(profiles_repo, jobs_repo));
            start_scheduler_task(scheduler, tokio::sync::mpsc::channel(100).1);
            
            log::info!("Application setup complete");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_profile,
            get_profile,
            list_profiles,
            update_profile,
            delete_profile,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running SyncFolder");
}
