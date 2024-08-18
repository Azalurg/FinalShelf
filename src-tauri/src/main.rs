// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::File, io::Read};

use rusqlite::Result;

pub mod db;
pub mod metadata;
pub mod structs;

// -------------------
// General functions
// -------------------

#[tauri::command]
fn tauri_scan(directory: String) -> Result<(), String> {
    match metadata::scan_for_metadata(directory) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn tauri_clear_db() -> Result<(), String> {
    db::clear_db();
    let conn = db::get_db_connection().map_err(|e| e.to_string())?;
    db::init_db(&conn);
    Ok(())
}

#[tauri::command]
fn tauri_kill() -> Result<(), String> {
    println!("App will be closed");
    panic!()
}

// -------------------
// Book functions
// -------------------

#[tauri::command]
fn tauri_get_books() -> Result<Vec<structs::FrontendBook>, String> {
    let conn = db::get_db_connection().map_err(|e| e.to_string())?;
    match db::get_all_books_list_frontend(&conn) {
        Ok(books) => Ok(books),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn tauri_get_book_details(book_id: i64) -> Result<structs::FrontendBookDetails, String> {
    println!("Getting book details for book_id: {}", book_id);
    let conn = db::get_db_connection().map_err(|e| e.to_string())?;
    match db::get_book_by_id(&conn, book_id) {
        Ok(book) => Ok(book),
        Err(e) => Err(e.to_string()),
    }
}

// -------------------
// Author functions
// -------------------


#[tauri::command]
fn tauri_get_authors() -> Result<Vec<structs::Author>, String> {
    let conn = db::get_db_connection().map_err(|e| e.to_string())?;
    match db::get_all_authors(&conn) {
        Ok(authors) => Ok(authors),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn tauri_get_author_details(author_id: i64) -> Result<structs::AuthorDetails, String> {
    let conn = db::get_db_connection().map_err(|e| e.to_string())?;
    match db::get_author_by_id(&conn, author_id) {
        Ok(author) => Ok(author),
        Err(e) => Err(e.to_string()),
    }
}

// -------------------
// Dashboard functions
// -------------------

#[tauri::command]
fn tauri_get_dashboard_data() -> Result<structs::DashboardData, String> {
    let conn = db::get_db_connection().map_err(|e| e.to_string())?;
    match db::get_dashboard_data(&conn) {
        Ok(data) => Ok(data),
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
        .invoke_handler(tauri::generate_handler![
            tauri_scan,
            tauri_clear_db,
            tauri_kill,
            tauri_get_books,
            tauri_get_book_details,
            tauri_get_authors,
            tauri_get_author_details,
            tauri_get_dashboard_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
