// use crate::{
//     db::establish_db_connection,
//     models::session::{NewSession, Session},
//     schema::sessions,
//     schema::sessions::dsl,
// };

use crate::{
    db::establish_connection,
    models::{book::Book, query::QueryParams},
    schema::books,
    schema::books::dsl,
};
use diesel::prelude::*;

pub fn get_book(title: &str) -> Option<Book> {
    let conn = &mut establish_connection();

    let query = dsl::books.filter(dsl::title.eq(title));

    query.first::<Book>(conn).ok()
}

pub fn list_books(query_params: QueryParams) -> Vec<Book> {
    let conn = &mut establish_connection();

    let query = dsl::books.order_by(dsl::author_name.desc());

    query.load::<Book>(conn).expect("Error loading books")
}

pub fn add_book(new_book: &Book) -> Option<Book> {
    let conn = &mut establish_connection();

    diesel::insert_into(books::table)
        .values(new_book)
        .execute(conn)
        .expect("Error saving new book");

    get_book(&new_book.title)
}

pub fn is_book_exists(title: &str) -> bool {
    let conn = &mut establish_connection();

    let query = dsl::books.filter(dsl::title.eq(title));

    query.first::<Book>(conn).is_ok()
}

// pub fn get_session(session_id: &String) -> Option<Session> {
//     let connection = &mut establish_db_connection();

//     dsl::sessions
//         .filter(dsl::id.eq(session_id))
//         .first::<Session>(connection)
//         .ok()
// }

// pub fn store_session(new_session: &NewSession) {
//     let connection = &mut establish_db_connection();

//     diesel::insert_into(sessions::table)
//         .values(new_session)
//         .execute(connection)
//         .expect("Error saving new session");
// }

// pub fn update_session_name(session_id: String, name: String) {
//     let connection = &mut establish_db_connection();

//     diesel::update(dsl::sessions)
//         .filter(dsl::id.eq(session_id.clone()))
//         .set(dsl::name.eq(name.clone()))
//         .execute(connection)
//         .expect("Error updating session name");
// }

// pub fn delete_session(session_id: String) {
//     let connection = &mut establish_db_connection();

//     diesel::delete(dsl::sessions)
//         .filter(crate::schema::sessions::dsl::id.eq(session_id))
//         .execute(connection)
//         .expect("Error deleting session");
// }
