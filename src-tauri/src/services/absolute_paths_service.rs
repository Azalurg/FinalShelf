//Functions: get_current_absolute_path, get_all_absolute_path, check_if_absolute_path_in_db, check_if_path_exists, add_path, update_last_use_date, delete_path

use crate::{
    db::establish_connection,
    models::path::{AbsolutePath, NewAbsolutePath},
};

use diesel::prelude::*;

pub fn get_current_absolute_path() -> Option<AbsolutePath> {
    let conn = &mut establish_connection();

    let query = crate::schema::absolute_paths::dsl::absolute_paths
        .order_by(crate::schema::absolute_paths::dsl::last_use_date.desc());

    query.first::<AbsolutePath>(conn).ok()
}

pub fn get_all_absolute_path() -> Result<Vec<AbsolutePath>, String> {
    let conn = &mut establish_connection();

    let query = crate::schema::absolute_paths::dsl::absolute_paths
        .order_by(crate::schema::absolute_paths::dsl::last_use_date.desc());

    query.load::<AbsolutePath>(conn).map_err(|e| {
        eprintln!("Error loading absolute paths: {}", e);
        format!("Failed to load library paths: {}", e)
    })
}

fn check_if_absolute_path_in_db_by_name(absolute_path: &str) -> bool {
    let conn = &mut establish_connection();

    let query = crate::schema::absolute_paths::dsl::absolute_paths
        .filter(crate::schema::absolute_paths::dsl::absolute_path.eq(absolute_path));

    query.first::<AbsolutePath>(conn).is_ok()
}

fn check_if_absolute_path_in_db_by_id(absolute_path_id: i32) -> bool {
    let conn = &mut establish_connection();

    let query = crate::schema::absolute_paths::dsl::absolute_paths
        .filter(crate::schema::absolute_paths::dsl::id.eq(absolute_path_id));

    query.first::<AbsolutePath>(conn).is_ok()
}

pub fn set_current_absolute_path_by_id(absolute_path_id: i32) -> Result<AbsolutePath, String> {
    if !check_if_absolute_path_in_db_by_id(absolute_path_id) {
        return Err("Path already exists".to_string());
    }

    use crate::schema::absolute_paths::dsl;

    let conn = &mut establish_connection();

    let now = chrono::Utc::now().naive_utc();

    diesel::update(dsl::absolute_paths.filter(dsl::id.eq(absolute_path_id)))
        .set(dsl::last_use_date.eq(Some(now)))
        .execute(conn)
        .ok()
        .ok_or("Error setting current path".to_string())?;

    dsl::absolute_paths
        .filter(dsl::id.eq(absolute_path_id))
        .first::<AbsolutePath>(conn)
        .ok()
        .ok_or("Error setting current path".to_string())
}

pub fn add_absolute_path(absolute_path: String) -> Result<(), String> {
    print!("Adding path: {}", absolute_path);
    let conn = &mut establish_connection();

    if check_if_absolute_path_in_db_by_name(absolute_path.as_str()) {
        return Err("Path already exists".to_string());
    }

    let now = chrono::Utc::now().naive_utc();

    let new_absolute_path = NewAbsolutePath {
        absolute_path,
        add_date: now,
        last_use_date: Some(now),
    };

    println!("Debug: {:?}", new_absolute_path);

    diesel::insert_into(crate::schema::absolute_paths::table)
        .values(&new_absolute_path)
        .execute(conn)
        .map_err(|e| format!("Error inserting absolute path: {}", e))?;

    Ok(())
}

