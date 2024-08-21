// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv::dotenv;
use rusqlite::Result;

pub mod db;
pub mod scanner;
pub mod structs;

// -------------------
// General functions
// -------------------

#[tauri::command]
fn tauri_full_scan(directory: String) -> Result<(), String> {
    match scanner::full_scan(&directory) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn tauri_quick_scan(directory: String) -> Result<(), String> {
    match scanner::quick_scan(&directory) {
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
fn tauri_get_books(
    author_id: Option<i64>,
    genre_id: Option<i64>,
    lectror_id: Option<i64>,
    sort_params: Option<&str>,
    sort_order: Option<&str>,
    page: u64,
    page_size: u64,
) -> Result<Vec<structs::FrontendBook>, String> {
    let conn = db::get_db_connection().map_err(|e| e.to_string())?;
    match db::get_filtered_and_paginated_books(
        &conn,
        author_id,
        genre_id,
        lectror_id,
        sort_params,
        sort_order,
        page,
        page_size,
    ) {
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
// Lector functions
// -------------------

#[tauri::command]
fn tauri_get_lectors() -> Result<Vec<structs::Lector>, String> {
    let conn = db::get_db_connection().map_err(|e| e.to_string())?;
    match db::get_all_lectors(&conn) {
        Ok(lectors) => Ok(lectors),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn tauri_get_lector_details(lector_id: i64) -> Result<structs::LectorDetails, String> {
    let conn = db::get_db_connection().map_err(|e| e.to_string())?;
    match db::get_lector_by_id(&conn, lector_id) {
        Ok(lector) => Ok(lector),
        Err(e) => Err(e.to_string()),
    }
}

// -------------------
// Genre functions
// -------------------

#[tauri::command]
fn tauri_get_genres() -> Result<Vec<structs::Genre>, String> {
    let conn = db::get_db_connection().map_err(|e| e.to_string())?;
    match db::get_all_genres(&conn) {
        Ok(genres) => Ok(genres),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn tauri_get_genre_details(genre_id: i64) -> Result<structs::GenreDetails, String> {
    let conn = db::get_db_connection().map_err(|e| e.to_string())?;
    match db::get_genre_by_id(&conn, genre_id) {
        Ok(genre) => Ok(genre),
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

// // -------------------
// // Search functions
// // -------------------

#[tauri::command]
fn tauri_search_books(search_query: &str) -> Result<Vec<structs::FrontendBook>, String> {
    let conn = db::get_db_connection().map_err(|e| e.to_string())?;
    match db::search_books(&conn, search_query) {
        Ok(books) => Ok(books),
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    dotenv().ok();

    tauri::Builder::default()
        .setup(|_app| {
            let conn = db::get_db_connection().expect("error while getting db connection");
            db::init_db(&conn).expect("error while initializing db");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            tauri_full_scan,
            tauri_quick_scan,
            tauri_clear_db,
            tauri_kill,
            tauri_get_books,
            tauri_get_book_details,
            tauri_get_authors,
            tauri_get_author_details,
            tauri_get_dashboard_data,
            tauri_get_lectors,
            tauri_get_lector_details,
            tauri_get_genres,
            tauri_get_genre_details,
            tauri_search_books
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
