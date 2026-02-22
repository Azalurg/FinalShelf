// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod models;
mod scanner;
mod schema;
mod services;

use commands::{
    authors_commands::*, books_commands::*, dashboard_commands::*, genres_commands::*, lectors_commands::*,
    search_commands::*, settings_commands::*,
};

fn main() {
    println!("Starting Tauri application...");

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
            // --- search ---
            search_command,
            // --- books ---
            get_books_list_command,
            get_book_command,
            get_all_read_books_command,
            update_book_command,
            // --- authors ---
            get_authors_list_command,
            get_author_command,
            // --- lectors ---
            get_lectors_list_command,
            get_lector_command,
            // --- genres ---
            get_genres_list_command,
            get_genre_command,
            // --- dashboard ---
            get_dashboard_data_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
