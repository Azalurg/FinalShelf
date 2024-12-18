-- Drop foreign key constraints in reverse order of their addition
ALTER TABLE books_read DROP FOREIGN KEY books_read_book_title_fk;
ALTER TABLE tags_books DROP FOREIGN KEY tags_books_book_title_fk;
ALTER TABLE tags_books DROP FOREIGN KEY tags_books_tag_id_fk;
ALTER TABLE tags_authors DROP FOREIGN KEY tags_authors_author_name_fk;
ALTER TABLE tags_authors DROP FOREIGN KEY tags_authors_tag_id_fk;
ALTER TABLE books DROP FOREIGN KEY books_lector_id_fk;
ALTER TABLE books DROP FOREIGN KEY books_author_id_fk;
ALTER TABLE books DROP FOREIGN KEY books_genre_id_fk;

-- Drop tables in reverse order of their creation
DROP TABLE absolute_paths;
DROP TABLE books_read;
DROP TABLE tags_books;
DROP TABLE tags_authors;
DROP TABLE tags;
DROP TABLE genres;
DROP TABLE lectors;
DROP TABLE authors;
DROP TABLE books;
