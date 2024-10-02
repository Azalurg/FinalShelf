use serde::Serialize;

#[derive(Serialize)]
pub struct Author {
    pub id: i64,
    pub name: String,
    pub picture_path: String,
}

#[derive(Serialize)]
pub struct AuthorDetails {
    pub id: i64,
    pub name: String,
    pub picture_path: String,
    pub books: Vec<FrontendBook>,
}

#[derive(Serialize)]
pub struct Lector {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize)]
pub struct LectorList {
    pub id: i64,
    pub name: String,
    pub books: i64,
}

#[derive(Serialize)]
pub struct LectorDetails {
    pub id: i64,
    pub name: String,
    pub books: Vec<FrontendBook>,
}

#[derive(Serialize)]
pub struct Genre {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize)]
pub struct GenreList {
    pub id: i64,
    pub name: String,
    pub books: i64,
}

#[derive(Serialize)]
pub struct GenreDetails {
    pub id: i64,
    pub name: String,
    pub books: Vec<FrontendBook>,
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

#[derive(Serialize, Debug)]
pub struct FrontendBookDetails {
    pub id: i64,
    pub title: String,
    pub cover_path: String,
    pub duration: u64,
    pub year: i32,
    pub genre_id: i64,
    pub genre_name: String,
    pub author_id: i64,
    pub author_name: String,
    pub author_picture_path: String,
    pub lector_id: i64,
    pub lector_name: String,
}

#[derive(Serialize)]
pub struct DashboardData {
    pub authors_amount: i64,
    pub genres_amount: i64,
    pub lectors_amount: i64,
    pub books_amount: i64,
}
