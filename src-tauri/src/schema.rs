// @generated automatically by Diesel CLI.

diesel::table! {
    absolute_paths (absolute_path_id) {
        absolute_path_id -> Nullable<Integer>,
        name -> Text,
        path -> Text,
    }
}

diesel::table! {
    authors (author_id) {
        author_id -> Nullable<Integer>,
        name -> Text,
        relative_img_path -> Nullable<Text>,
    }
}

diesel::table! {
    books (book_id) {
        book_id -> Nullable<Integer>,
        title -> Text,
        relative_cover_path -> Nullable<Text>,
        genre_id -> Nullable<Integer>,
        author_id -> Integer,
        lector_id -> Nullable<Integer>,
    }
}

diesel::table! {
    books_read (book_read_id) {
        book_read_id -> Nullable<Integer>,
        book_title -> Text,
        rate -> Nullable<Integer>,
        tier -> Nullable<Integer>,
        note -> Nullable<Text>,
    }
}

diesel::table! {
    genres (genre_id) {
        genre_id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    lectors (lector_id) {
        lector_id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    tags (tag_id) {
        tag_id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    tags_authors (tag_author_id) {
        tag_author_id -> Nullable<Integer>,
        tag_id -> Integer,
        author_name -> Text,
    }
}

diesel::table! {
    tags_books (tag_book_id) {
        tag_book_id -> Nullable<Integer>,
        tag_id -> Integer,
        book_title -> Text,
    }
}

diesel::joinable!(books -> authors (author_id));
diesel::joinable!(books -> genres (genre_id));
diesel::joinable!(books -> lectors (lector_id));
diesel::joinable!(tags_authors -> tags (tag_id));
diesel::joinable!(tags_books -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    absolute_paths,
    authors,
    books,
    books_read,
    genres,
    lectors,
    tags,
    tags_authors,
    tags_books,
);
