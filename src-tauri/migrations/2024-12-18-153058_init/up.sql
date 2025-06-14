CREATE TABLE books (
  title VARCHAR(255) PRIMARY KEY UNIQUE NOT NULL,
  relative_cover_path VARCHAR(255),
  author_name VARCHAR(36) NOT NULL,
  genre VARCHAR(255),
  lector VARCHAR(255),
  create_date DATETIME,
  FOREIGN KEY (author_name) REFERENCES authors (name) ON DELETE CASCADE
);

CREATE TABLE authors (
  name VARCHAR(255) PRIMARY KEY UNIQUE NOT NULL,
  relative_img_path VARCHAR(255)
);

CREATE TABLE tags (
  id INTEGER PRIMARY KEY NOT NULL,
  name VARCHAR(255) UNIQUE NOT NULL
);

-- CREATE TABLE tags_authors (
--   tag_id VARCHAR(36) NOT NULL,
--   author_id VARCHAR(36) NOT NULL,
--   PRIMARY KEY (tag_id, author_id),
--   FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE,
--   FOREIGN KEY (author_id) REFERENCES authors (id) ON DELETE CASCADE
-- );

CREATE TABLE tags_books (
  tag_id INTEGER NOT NULL,
  book_title VARCHAR(255) NOT NULL,
  PRIMARY KEY (tag_id, book_title),
  FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE,
  FOREIGN KEY (book_title) REFERENCES books (title) ON DELETE CASCADE
);

CREATE TABLE books_read (
  id INTEGER PRIMARY KEY NOT NULL,
  book_title VARCHAR(255) NOT NULL,
  rate INTEGER,
  tier INTEGER,
  note VARCHAR(511),
  read_date DATETIME,
  FOREIGN KEY (book_title) REFERENCES books (title) ON DELETE CASCADE
);
