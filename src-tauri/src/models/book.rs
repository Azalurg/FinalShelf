use crate::schema::*;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Debug, AsChangeset)]
#[diesel(table_name = books)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Book {
    pub title: String,
    pub relative_cover_path: Option<String>,
    pub author_name: String,
    pub genre: Option<String>,
    pub lector: Option<String>,
    pub create_date: Option<NaiveDateTime>,
    pub read: Option<bool>,
    pub score: Option<i32>,
    pub relative_file_path: String,
    pub duration_seconds: Option<i32>,
    pub duration_is_estimated: Option<bool>,
    pub file_count: Option<i32>,
    pub orphaned: Option<bool>,
}

#[derive(Serialize)]
pub struct BookListResponse {
    pub books: Vec<Book>,
    pub total_count: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}
