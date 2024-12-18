use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::books)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Book {
    pub id: i64,
    pub title: String,
    pub author_id: i64,
    pub genre_id: Option<i64>,
    pub lector_id: Option<i64>,
    pub relative_cover_path: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::authors)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Author {
    pub id: i64,
    pub name: String,
    pub relative_img_path: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::lectors)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Lector {
    pub id: i64,
    pub name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::genres)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Genre {
    pub id: i64,
    pub name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tags_authors)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TagAuthor {
    pub id: i64,
    pub tag_id: i64,
    pub author_name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tags_books)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TagBook {
    pub id: i64,
    pub tag_id: i64,
    pub book_title: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::books_finished)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BookFinished {
    pub id: i64,
    pub book_title: String,
    pub rate: Option<i64>,
    pub tier: Option<i64>,
    pub note: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::absolute_paths)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AbsolutePath {
    pub id: i64,
    pub name: String,
    pub path: String
}

// use serde::Serialize;

// #[derive(Serialize)]
// pub struct Author {
//     pub id: i64,
//     pub name: String,
//     pub picture_path: String,
// }

// #[derive(Serialize)]
// pub struct AuthorDetails {
//     pub id: i64,
//     pub name: String,
//     pub picture_path: String,
//     pub books: Vec<FrontendBook>,
// }

// #[derive(Serialize)]
// pub struct Lector {
//     pub id: i64,
//     pub name: String,
// }

// #[derive(Serialize)]
// pub struct LectorList {
//     pub id: i64,
//     pub name: String,
//     pub books: i64,
// }

// #[derive(Serialize)]
// pub struct LectorDetails {
//     pub id: i64,
//     pub name: String,
//     pub books: Vec<FrontendBook>,
// }

// #[derive(Serialize)]
// pub struct Genre {
//     pub id: i64,
//     pub name: String,
// }

// #[derive(Serialize)]
// pub struct GenreList {
//     pub id: i64,
//     pub name: String,
//     pub books: i64,
// }

// #[derive(Serialize)]
// pub struct GenreDetails {
//     pub id: i64,
//     pub name: String,
//     pub books: Vec<FrontendBook>,
// }

// #[derive(Serialize)]
// pub struct DBBook {
//     pub id: i64,
//     pub title: String,
//     pub duration: u64,
//     pub year: i32,
//     pub cover_path: String,
//     pub genre_id: i64,
//     pub author_id: i64,
//     pub lector_id: i64,
// }

// #[derive(Serialize)]
// pub struct FrontendBook {
//     pub id: i64,
//     pub title: String,
//     pub cover_path: String,
//     pub author_id: i64,
//     pub author_name: String,
// }

// #[derive(Serialize, Debug)]
// pub struct FrontendBookDetails {
//     pub id: i64,
//     pub title: String,
//     pub cover_path: String,
//     pub duration: u64,
//     pub year: i32,
//     pub genre_id: i64,
//     pub genre_name: String,
//     pub author_id: i64,
//     pub author_name: String,
//     pub author_picture_path: String,
//     pub lector_id: i64,
//     pub lector_name: String,
// }

// #[derive(Serialize)]
// pub struct DashboardData {
//     pub authors_amount: i64,
//     pub genres_amount: i64,
//     pub lectors_amount: i64,
//     pub books_amount: i64,
// }
