// @generated automatically by Diesel CLI.

diesel::table! {
    authors (name) {
        name -> Text,
        relative_img_path -> Nullable<Text>,
    }
}

diesel::table! {
    books (title) {
        title -> Text,
        relative_cover_path -> Nullable<Text>,
        author_name -> Text,
        genre -> Nullable<Text>,
        lector -> Nullable<Text>,
        create_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    books_read (id) {
        id -> Integer,
        book_title -> Text,
        rate -> Nullable<Integer>,
        tier -> Nullable<Integer>,
        note -> Nullable<Text>,
        read_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    tags_books (tag_id, book_title) {
        tag_id -> Integer,
        book_title -> Text,
    }
}

diesel::joinable!(books -> authors (author_name));
diesel::joinable!(books_read -> books (book_title));
diesel::joinable!(tags_books -> books (book_title));
diesel::joinable!(tags_books -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    books,
    books_read,
    tags,
    tags_books,
);
