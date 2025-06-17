use crate::schema::*;

use diesel::Insertable;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use super::book::Book;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = authors)]
pub struct Author {
    pub name: String,
    pub relative_img_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorWithBooks {
    pub author: Author,
    pub books: Vec<Book>,
}
