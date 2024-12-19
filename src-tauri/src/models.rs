use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::books)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author_id: String,
    pub genre_id: Option<String>,
    pub lector_id: Option<String>,
    pub relative_cover_path: Option<String>,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::authors)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Author {
    pub id: String,
    pub name: String,
    pub relative_img_path: Option<String>,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::lectors)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Lector {
    pub id: String,
    pub name: String,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::genres)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Genre {
    pub id: String,
    pub name: String,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tag {
    pub id: String,
    pub name: String,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::tags_authors)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TagAuthor {
    pub id: String,
    pub tag_id: String,
    pub author_id: String,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::tags_books)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TagBook {
    pub id: String,
    pub tag_id: String,
    pub book_id: String,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::books_read)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BookRead {
    pub id: String,
    pub book_id: String,
    pub rate: Option<i32>,
    pub tier: Option<i32>,
    pub note: Option<String>,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::absolute_paths)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AbsolutePath {
    pub id: String,
    pub name: String,
    pub path: String,
}
