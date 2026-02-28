// @generated automatically by Diesel CLI.

diesel::table! {
    absolute_paths (id) {
        id -> Nullable<Integer>,
        absolute_path -> Text,
        add_date -> Timestamp,
        last_use_date -> Nullable<Timestamp>,
    }
}

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
        read -> Nullable<Bool>,
        score -> Nullable<Integer>,
        relative_file_path -> Text,
        duration_seconds -> Nullable<Integer>,
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
diesel::joinable!(tags_books -> books (book_title));
diesel::joinable!(tags_books -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(absolute_paths, authors, books, tags, tags_books,);
