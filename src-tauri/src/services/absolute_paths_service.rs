//Functions: get_current_absolute_path, get_all_absolute_path, check_if_absolute_path_in_db, check_if_path_exists, add_path, update_last_use_date, delete_path 

use crate::{
    db::establish_connection,
    models::path::AbsolutePath,
};

use diesel::prelude::*;

pub fn get_current_absolute_path() -> Option<AbsolutePath> {
    let conn = &mut establish_connection();

    let query = crate::schema::absolute_paths::dsl::absolute_paths
        .order_by(crate::schema::absolute_paths::dsl::last_use_date.desc());

    query.first::<AbsolutePath>(conn).ok()
}

pub fn get_all_absolute_path() -> Vec<AbsolutePath> {
    let conn = &mut establish_connection();

    let query = crate::schema::absolute_paths::dsl::absolute_paths
        .order_by(crate::schema::absolute_paths::dsl::last_use_date.desc());

    query.load::<AbsolutePath>(conn).expect("Error loading absolute paths")
}

pub fn check_if_absolute_path_in_db(absolute_path: &str) -> bool {
    let conn = &mut establish_connection();

    let query = crate::schema::absolute_paths::dsl::absolute_paths
        .filter(crate::schema::absolute_paths::dsl::absolute_path.eq(absolute_path));

    query.first::<AbsolutePath>(conn).is_ok()
}

pub fn set_current_absolute_path_by_id(absolute_path_id: i32) -> Option<AbsolutePath> {
    use crate::schema::absolute_paths::dsl;

    let conn = &mut establish_connection();

    let now = chrono::Utc::now().naive_utc();

    diesel::update(dsl::absolute_paths.filter(dsl::id.eq(absolute_path_id)))
        .set(dsl::last_use_date.eq(Some(now)))
        .execute(conn)
        .ok()?;

    dsl::absolute_paths
        .filter(dsl::id.eq(absolute_path_id))
        .first::<AbsolutePath>(conn)
        .ok()
}

