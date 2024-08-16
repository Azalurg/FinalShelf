// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::Result;

pub mod db;
pub mod metadata;

#[tauri::command]
fn tauri_scan(directory: String) -> Result<(), String> {
    match metadata::scan_for_metadata(directory) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn tauri_get_books() -> Result<Vec<db::Book>, String> {
    let conn = db::get_db_connection().map_err(|e| e.to_string())?;
    match db::get_books(&conn) {
        Ok(books) => Ok(books),
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            let conn = db::get_db_connection().expect("error while getting db connection");
            db::init_db(&conn).expect("error while initializing db");
            Ok(())
            })
        .invoke_handler(tauri::generate_handler![tauri_scan, tauri_get_books])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
