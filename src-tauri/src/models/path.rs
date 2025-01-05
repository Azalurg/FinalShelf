use crate::schema::*;

use chrono::NaiveDateTime;
use diesel::Insertable;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "absolute_paths"]
pub struct AbsolutePath {
    pub id: Option<i32>,
    pub absolute_path: String,
    pub add_date: NaiveDateTime,
    pub last_use_date: Option<NaiveDateTime>,
}
