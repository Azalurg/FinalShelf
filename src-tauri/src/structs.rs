use serde::Serialize;

#[derive(Serialize)]
pub struct Author {
    id: i64,
    name: String,
    picture_path: String,
}

#[derive(Serialize)]
pub struct Lector {
    id: i64,
    name: String,
}

#[derive(Serialize)]
pub struct Genre{
    id: i64,
    name: String,
}

#[derive(Serialize)]
pub struct DBBook {
    pub id: i64,
    pub title: String,
    pub duration: u64,
    pub year: i32,
    pub cover_path: String,
    pub genre_id: i64,
    pub author_id: i64,
    pub lector_id: i64,
}

#[derive(Serialize)]
pub struct FrontendBook {
    pub id: i64,
    pub title: String,
    pub cover_path: String,
    pub author_id: i64,
    pub author_name: String,
}