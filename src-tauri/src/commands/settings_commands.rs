// Kill, Change background, Add path, change path, remove path

use tauri::{Emitter, Window};

use crate::{
    models::path::AbsolutePath,
    scanner::{scan_with_progress, ScanProgress, ScanResult},
    services::absolute_paths_service::{
        add_absolute_path, get_all_absolute_path, get_current_absolute_path, set_current_absolute_path_by_id,
    },
};

#[tauri::command]
pub fn ping_command() -> String {
    "ping".to_string()
}

#[tauri::command]
pub fn get_version_command() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
pub fn quick_scan_command(window: Window) -> Result<ScanResult, String> {
    scan_with_progress(false, |progress: ScanProgress| {
        let _ = window.emit("scan-progress", progress);
    })
}

#[tauri::command]
pub fn full_scan_command(window: Window) -> Result<ScanResult, String> {
    scan_with_progress(true, |progress: ScanProgress| {
        let _ = window.emit("scan-progress", progress);
    })
}

#[tauri::command]
pub fn kill_command() -> Result<(), String> {
    std::process::exit(0);
}

#[tauri::command]
pub fn add_absolute_path_command(absolute_path: String) -> Result<(), String> {
    add_absolute_path(absolute_path)
}

#[tauri::command]
pub fn get_all_absolute_path_command() -> Result<Vec<AbsolutePath>, String> {
    get_all_absolute_path()
}

#[tauri::command]
pub fn set_current_absolute_path_by_id_command(absolute_path_id: i32) -> Result<(), String> {
    print!("Setting current path by id: {}", absolute_path_id);
    let _ = set_current_absolute_path_by_id(absolute_path_id);
    Ok(())
}

#[tauri::command]
pub fn get_current_absolute_path_command() -> Option<AbsolutePath> {
    get_current_absolute_path()
}
