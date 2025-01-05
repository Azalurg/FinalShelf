use std::path::PathBuf;
use crate::services::absolute_paths_service::get_current_absolute_path;

pub fn get_absolute_path(relative_path: &str) -> PathBuf {
    let current_absolute_path = get_current_absolute_path().unwrap();
    let mut absolute_path = PathBuf::from(current_absolute_path.absolute_path);
    absolute_path.push(relative_path);
    absolute_path
}