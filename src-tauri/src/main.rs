// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod models;
mod scanner;
mod schema;
mod services;

use commands::{authors_commands::*, books_commands::*, settings_commands::*};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|_app| {
            db::init();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // --- settings ---
            ping_command,
            quick_scan_command,
            kill_command,
            add_absolute_path_command,
            get_all_absolute_path_command,
            set_current_absolute_path_by_id_command,
            get_current_absolute_path_command,
            // --- books ---
            get_books_list_command,
            get_book_command,
            // --- authors ---
            get_authors_list_command,
            get_author_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
