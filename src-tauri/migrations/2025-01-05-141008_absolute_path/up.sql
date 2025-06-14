CREATE TABLE absolute_paths (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    absolute_path TEXT NOT NULL UNIQUE,
    add_date DATETIME NOT NULL DEFAULT (datetime('now')),
    last_use_date DATETIME
);