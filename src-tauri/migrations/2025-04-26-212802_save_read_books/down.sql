-- This file should undo anything in `up.sql`
ALTER TABLE books DROP COLUMN read;
ALTER TABLE books DROP COLUMN score;

CREATE TABLE books_read (
  id INTEGER PRIMARY KEY NOT NULL,
  book_title VARCHAR(255) NOT NULL,
  rate INTEGER,
  tier INTEGER,
  note VARCHAR(511),
  read_date DATETIME,
  FOREIGN KEY (book_title) REFERENCES books (title) ON DELETE CASCADE
);
