use crate::schema::*;

use chrono::NaiveDateTime;
use diesel::prelude::AsChangeset;
use diesel::Insertable;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, AsChangeset)]
#[table_name = "books"]
pub struct Book {
    pub title: String,
    pub relative_cover_path: Option<String>,
    pub author_name: String,
    pub genre: Option<String>,
    pub lector: Option<String>,
    pub create_date: Option<NaiveDateTime>,
    pub read: Option<bool>,
    pub score: Option<i32>,
}

#[derive(Serialize)]
pub struct BookListResponse {
    pub books: Vec<Book>,
    pub total_count: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}
