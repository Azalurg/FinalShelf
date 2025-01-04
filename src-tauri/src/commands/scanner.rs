// Functions

// Public: Quick scan, Full scan, Search for missing, Find new (example: form past month)

use crate::scanner;

#[tauri::command]
pub fn quick_scan(directory: String) -> Result<(), String> {
    scanner::quick_scan(&directory)
}