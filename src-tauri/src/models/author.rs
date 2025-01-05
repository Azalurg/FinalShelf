use crate::schema::*;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Insertable;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "authors"]
pub struct Author {
    pub name: String,
    pub relative_img_path: Option<String>,
}
