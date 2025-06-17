use serde::{Deserialize, Serialize};

use super::author::Author;
use super::book::Book;

#[derive(Serialize, Deserialize, Debug)]
pub struct Dashboard {
    pub books_count: i64,
    pub read_books_count: i64,
    pub authors_count: i64,
    pub genres_count: i64,
    pub lectors_count: i64,
    pub new_books: Vec<Book>,
    pub top_authors: Vec<(Author, i64)>,
    pub top_books: Vec<Book>,
}
