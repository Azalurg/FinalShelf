use crate::models::book::Book;
use crate::schema::*;
use diesel::sqlite::Sqlite;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use diesel::prelude::*;

#[derive(QueryableByName, Serialize, Deserialize, Debug)]
pub struct Lector {
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub name: String,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub books_amount: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LectorWithBooks {
    pub name: String,
    pub books: Vec<Book>,
    pub books_amount: i64,
}
