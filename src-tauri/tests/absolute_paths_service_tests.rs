use chrono::Utc;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use finalshelf::{
    db::{establish_connection, init},
    models::path::{AbsolutePath, NewAbsolutePath},
    schema,
    services::absolute_paths_service::{add_absolute_path, set_current_absolute_path_by_id},
};
use std::{
    env, fs,
    path::PathBuf,
    sync::{LazyLock, Mutex, MutexGuard},
};

static DB_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

fn setup_test_db(name: &str) -> (SqliteConnection, PathBuf, MutexGuard<'static, ()>) {
    let guard = DB_MUTEX.lock().unwrap();
    let mut path = env::temp_dir();
    path.push(format!("finalshelf_test_paths_{}.sqlite", name));
    let _ = fs::remove_file(&path);
    env::set_var("DATABASE_URL", &path);

    init();
    (establish_connection(), path, guard)
}

#[test]
fn updates_last_use_date_for_existing_path() {
    let (mut conn, path, _guard) = setup_test_db("update_last_use");
    let now = Utc::now().naive_utc();

    let new_path = NewAbsolutePath {
        absolute_path: "/tmp/library".to_string(),
        add_date: now,
        last_use_date: Some(now),
    };

    diesel::insert_into(schema::absolute_paths::table)
        .values(&new_path)
        .execute(&mut conn)
        .unwrap();

    let inserted = schema::absolute_paths::dsl::absolute_paths
        .first::<AbsolutePath>(&mut conn)
        .unwrap();

    // Release the writer lock before invoking service code that opens a new connection.
    let inserted_id = inserted.id.unwrap();
    drop(conn);

    let updated = set_current_absolute_path_by_id(inserted_id).expect("should update path timestamp");

    assert!(updated.last_use_date.is_some());

    let _ = fs::remove_file(path);
}

#[test]
fn rejects_duplicate_paths() {
    let (mut conn, path, _guard) = setup_test_db("duplicate_paths");
    let now = Utc::now().naive_utc();

    let new_path = NewAbsolutePath {
        absolute_path: "/tmp/library".to_string(),
        add_date: now,
        last_use_date: Some(now),
    };

    diesel::insert_into(schema::absolute_paths::table)
        .values(&new_path)
        .execute(&mut conn)
        .unwrap();

    // Release the writer lock before invoking service code that opens a new connection.
    drop(conn);

    let result = add_absolute_path("/tmp/library".to_string());
    assert!(result.is_err());

    let _ = fs::remove_file(path);
}
