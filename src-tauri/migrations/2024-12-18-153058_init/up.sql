CREATE TABLE books (
  book_id INT PRIMARY KEY,
  title VARCHAR(255) UNIQUE NOT NULL,
  relative_cover_path VARCHAR(255),
  genre_id INT,
  author_id INT NOT NULL,
  lector_id INT,
  FOREIGN KEY (genre_id) REFERENCES genres (genre_id),
  FOREIGN KEY (author_id) REFERENCES authors (author_id),
  FOREIGN KEY (lector_id) REFERENCES lectors (lector_id)
);

CREATE TABLE authors (
  author_id INT PRIMARY KEY,
  name VARCHAR(255) UNIQUE NOT NULL,
  relative_img_path VARCHAR(255)
);

CREATE TABLE lectors (
  lector_id INT PRIMARY KEY,
  name VARCHAR(255) UNIQUE NOT NULL
);

CREATE TABLE genres (
  genre_id INT PRIMARY KEY,
  name VARCHAR(255) UNIQUE NOT NULL
);

CREATE TABLE tags (
  tag_id INT PRIMARY KEY,
  name VARCHAR(255) UNIQUE NOT NULL
);

CREATE TABLE tags_authors (
  tag_author_id INT PRIMARY KEY,
  tag_id INT NOT NULL,
  author_name VARCHAR(255) NOT NULL,
  FOREIGN KEY (tag_id) REFERENCES tags (tag_id)
  -- NOTE: Removed FOREIGN KEY to authors(name) because SQLite does not support it
);

CREATE TABLE tags_books (
  tag_book_id INT PRIMARY KEY,
  tag_id INT NOT NULL,
  book_title VARCHAR(255) NOT NULL,
  FOREIGN KEY (tag_id) REFERENCES tags (tag_id)
  -- NOTE: Removed FOREIGN KEY to books(title) because SQLite does not support it
);

CREATE TABLE books_read (
  book_read_id INT PRIMARY KEY,
  book_title VARCHAR(255) NOT NULL,
  rate INT,
  tier INT,
  note VARCHAR(511)
  -- NOTE: Removed FOREIGN KEY to books(title) because SQLite does not support it
);

CREATE TABLE absolute_paths (
  absolute_path_id INT PRIMARY KEY,
  name VARCHAR(255) UNIQUE NOT NULL,
  path VARCHAR(255) UNIQUE NOT NULL
);
