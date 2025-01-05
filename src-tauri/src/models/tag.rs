// Tags table model
use crate::schema::*;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Insertable;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

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
