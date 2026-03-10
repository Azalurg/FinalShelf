use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use finalshelf::{
    db::{establish_connection, init},
    models::{author::Author, book::Book, query::ListParams},
    schema,
    services::books_service::{get_book, list_books, update_book},
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
    path.push(format!("finalshelf_test_books_{}.sqlite", name));
    let _ = fs::remove_file(&path);
    env::set_var("DATABASE_URL", &path);

    init();
    (establish_connection(), path, guard)
}

fn insert_authors(conn: &mut SqliteConnection, names: &[&str]) {
    let authors: Vec<Author> = names
        .iter()
        .map(|name| Author {
            name: (*name).to_string(),
            relative_img_path: None,
        })
        .collect();

    diesel::insert_into(schema::authors::table)
        .values(&authors)
        .execute(conn)
        .unwrap();
}

fn make_book(title: &str, author: &str, read: bool, score: i32) -> Book {
    Book {
        title: title.to_string(),
        relative_cover_path: None,
        author_name: author.to_string(),
        genre: Some("Genre".to_string()),
        lector: Some("Lector".to_string()),
        create_date: Some(NaiveDateTime::from_timestamp_opt(0, 0).unwrap()),
        read: Some(read),
        score: Some(score),
        relative_file_path: format!("{title}.mp3"),
        duration_seconds: Some(100),
        series_id: None,
        series_order: None,
    }
}

#[test]
fn list_books_filters_by_author_and_read_status() {
    let (mut conn, path, _guard) = setup_test_db("filters");

    insert_authors(&mut conn, &["Author A", "Author B"]);

    let books = vec![
        make_book("First", "Author A", true, 7),
        make_book("Second", "Author B", false, 5),
    ];

    diesel::insert_into(schema::books::table)
        .values(&books)
        .execute(&mut conn)
        .unwrap();

    let params = ListParams {
        author_name: Some("Author A".to_string()),
        read_status: Some(true),
        ..Default::default()
    };

    let result = list_books(&params).expect("list_books should succeed");

    assert_eq!(result.items.len(), 1);
    assert_eq!(result.items[0].title, "First");

    drop(conn);
    let _ = fs::remove_file(path);
}

#[test]
fn update_book_persists_score_change() {
    let (mut conn, path, _guard) = setup_test_db("update_score");

    insert_authors(&mut conn, &["Author A"]);

    let book = make_book("Mutable", "Author A", false, 4);
    diesel::insert_into(schema::books::table)
        .values(&book)
        .execute(&mut conn)
        .unwrap();

    let mut stored = get_book("Mutable").expect("book should be present");
    stored.score = Some(9);

    let updated = update_book(&stored).expect("update should succeed");
    let persisted = schema::books::dsl::books
        .filter(schema::books::dsl::title.eq("Mutable"))
        .first::<Book>(&mut conn)
        .unwrap();

    assert_eq!(updated.score, Some(9));
    assert_eq!(persisted.score, Some(9));

    drop(conn);
    let _ = fs::remove_file(path);
}
