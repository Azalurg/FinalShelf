// Kill, Change background, Add path, change path, remove path

use crate::{
    models::path::AbsolutePath,
    scanner::{quick_scan, ScanReport},
    services::absolute_paths_service::{
        add_absolute_path, get_all_absolute_path, get_current_absolute_path, set_current_absolute_path_by_id,
    },
};

#[tauri::command]
pub fn ping_command() -> String {
    "ping".to_string()
}

#[tauri::command]
pub async fn quick_scan_command() -> Result<ScanReport, String> {
    quick_scan()
}

#[tauri::command]
pub fn kill_command() -> Result<(), String> {
    panic!("\n--- Killed by user ---\n");
}

#[tauri::command]
pub fn add_absolute_path_command(absolute_path: String) -> Result<(), String> {
    add_absolute_path(absolute_path)
}

#[tauri::command]
pub fn get_all_absolute_path_command() -> Vec<AbsolutePath> {
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
