use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Dashboard {
    pub books_count: i64,
    pub read_books_count: i64,
    pub authors_count: i64,
    pub genres_count: i64,
    pub lectors_count: i64,
    pub author_with_most_books: Option<String>,
}
