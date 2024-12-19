// @generated automatically by Diesel CLI.

diesel::table! {
    absolute_paths (id) {
        id -> Text,
        name -> Text,
        path -> Text,
    }
}

diesel::table! {
    authors (id) {
        id -> Text,
        name -> Text,
        relative_img_path -> Nullable<Text>,
    }
}

diesel::table! {
    books (id) {
        id -> Text,
        title -> Text,
        relative_cover_path -> Nullable<Text>,
        genre_id -> Nullable<Text>,
        author_id -> Text,
        lector_id -> Nullable<Text>,
    }
}

diesel::table! {
    books_read (id) {
        id -> Text,
        book_id -> Text,
        rate -> Nullable<Integer>,
        tier -> Nullable<Integer>,
        note -> Nullable<Text>,
    }
}

diesel::table! {
    genres (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    lectors (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    tags (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    tags_authors (id) {
        id -> Text,
        tag_id -> Text,
        author_id -> Text,
    }
}

diesel::table! {
    tags_books (id) {
        id -> Text,
        tag_id -> Text,
        book_id -> Text,
    }
}

diesel::joinable!(books -> authors (author_id));
diesel::joinable!(books -> genres (genre_id));
diesel::joinable!(books -> lectors (lector_id));
diesel::joinable!(books_read -> books (book_id));
diesel::joinable!(tags_authors -> authors (author_id));
diesel::joinable!(tags_authors -> tags (tag_id));
diesel::joinable!(tags_books -> books (book_id));
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
