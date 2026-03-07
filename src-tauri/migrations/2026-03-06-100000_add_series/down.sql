-- Remove series columns from books
ALTER TABLE books DROP COLUMN series_id;
ALTER TABLE books DROP COLUMN series_order;

-- Drop series table
DROP TABLE IF EXISTS series;
