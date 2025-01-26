// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod models;
mod scanner;
mod schema;
mod services;

use commands::books::*;
use commands::settings::*;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|_app| {
            db::init();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ping_command,
            quick_scan_command,
            get_books,
            kill_command,
            add_absolute_path_command,
            get_all_absolute_path_command,
            set_current_absolute_path_by_id_command,
            get_current_absolute_path_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
