use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};
use crate::schema::*;
use crate::models::book::Book;
use diesel::{Queryable};

use diesel::prelude::*;

#[derive(QueryableByName, Serialize, Deserialize, Debug)]
pub struct Lector{
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub name: String,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub books_amount: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LectorWithBooks{
    name: String,
    books: Vec<Book>,
}