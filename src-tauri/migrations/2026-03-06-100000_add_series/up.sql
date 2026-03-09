-- Create series table
CREATE TABLE series (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    author_name VARCHAR(255) NOT NULL REFERENCES authors(name) ON DELETE CASCADE,
    description TEXT,
    UNIQUE(name, author_name)
);

-- Add series_id and series_order columns to books
ALTER TABLE books ADD COLUMN series_id INTEGER REFERENCES series(id) ON DELETE SET NULL;
ALTER TABLE books ADD COLUMN series_order INTEGER;
