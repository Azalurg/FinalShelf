-- Revert scanner_fields migration
-- Note: SQLite >= 3.35 required for DROP COLUMN support.
-- This is a dev/test convenience only — production never needs to revert.

ALTER TABLE books DROP COLUMN orphaned;
ALTER TABLE books DROP COLUMN file_count;
ALTER TABLE books DROP COLUMN duration_is_estimated;
ALTER TABLE books DROP COLUMN duration_seconds;
