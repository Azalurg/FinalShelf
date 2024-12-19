use crate::schema::*;

use diesel::prelude::*;
use diesel::Queryable;
use diesel::Insertable;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "authors"]
pub struct Author {
    pub name: String,
    pub relative_img_path: Option<String>,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "books"]
pub struct Book {
    pub title: String,
    pub relative_cover_path: Option<String>,
    pub author_name: String,
    pub genre: Option<String>,
    pub lector: Option<String>,
    pub create_date: Option<NaiveDateTime>
}

// BooksRead table model
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

// Tags table model
#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "tags"]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

// TagsBooks table model
#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "tags_books"]
pub struct TagBook {
    pub tag_id: i32,
    pub book_title: String,
}
