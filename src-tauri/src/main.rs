// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;
use id3::{Tag, TagLike};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_authors(root_dir: String) -> Vec<String> {
    let mut authors = HashSet::new();
    
    for entry in WalkDir::new(root_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() && entry.path().extension().and_then(|s| s.to_str()) == Some("mp3") {
            if let Ok(tag) = Tag::read_from_path(entry.path()) {
                if let Some(artist) = tag.artist() {
                    authors.insert(artist.to_string());
                }
            }
        }
    }
    authors.into_iter().collect()
}

fn main() {  
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_authors])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
