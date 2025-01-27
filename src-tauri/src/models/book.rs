use crate::schema::*;

use chrono::NaiveDateTime;
use diesel::Insertable;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "books"]
pub struct Book {
    pub title: String,
    pub relative_cover_path: Option<String>,
    pub author_name: String,
    pub genre: Option<String>,
    pub lector: Option<String>,
    pub create_date: Option<NaiveDateTime>,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "books_read"]
pub struct BookRead {
    pub id: i32,
    pub book_title: String,
    pub rate: Option<i32>,
    pub tier: Option<i32>,
    pub note: Option<String>,
    pub read_date: Option<NaiveDateTime>,
}
