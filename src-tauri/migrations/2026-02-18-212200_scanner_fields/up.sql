-- Scanner Optimization & Enhancement: Add 4 new nullable columns
-- Feature: 001-scanner-optimization
-- Date: 2026-02-18
--
-- All four columns default to NULL to preserve existing rows without backfill.
-- The scanner will populate these fields on first scan after this migration.

ALTER TABLE books ADD COLUMN duration_seconds INTEGER DEFAULT NULL;
ALTER TABLE books ADD COLUMN duration_is_estimated BOOLEAN DEFAULT NULL;
ALTER TABLE books ADD COLUMN file_count INTEGER DEFAULT NULL;
ALTER TABLE books ADD COLUMN orphaned BOOLEAN DEFAULT NULL;
