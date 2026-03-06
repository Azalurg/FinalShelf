use crate::schema::*;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::book::Book;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = series)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Series {
    pub id: i32,
    pub name: String,
    pub author_name: String,
    pub description: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = series)]
pub struct NewSeries {
    pub name: String,
    pub author_name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeriesWithBooks {
    pub series: Series,
    pub books: Vec<Book>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeriesListItem {
    pub id: i32,
    pub name: String,
    pub author_name: String,
    pub description: Option<String>,
    pub books_count: i64,
}
