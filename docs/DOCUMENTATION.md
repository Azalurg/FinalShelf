# FinalShelf — Technical Documentation & Development Roadmap

> **Version:** 0.5.2  
> **Last updated:** 2026-03-09  
> **Stack:** Tauri 2.0 · Angular 17 · Diesel (SQLite) · Rust

---

## Table of Contents

1. [Implementation Status Analysis](#1-implementation-status-analysis)
2. [Technical Documentation](#2-technical-documentation)
   - [Architecture Overview](#21-architecture-overview)
   - [File & Module Structure](#22-file--module-structure)
   - [Database Schema](#23-database-schema)
   - [Backend — Key Modules](#24-backend--key-modules)
   - [Frontend — Key Modules](#25-frontend--key-modules)
   - [Libraries & Technologies](#26-libraries--technologies)
3. [Development Roadmap](#3-development-roadmap)
   - [Phase 1 — MVP](#phase-1--mvp)
   - [Phase 2 — Core Features](#phase-2--core-features)
   - [Phase 3 — Polish](#phase-3--polish)
4. [Risks and Recommendations](#4-risks-and-recommendations)

---

## 1. Implementation Status Analysis

| # | Feature | Status | Notes / Gaps |
|---|---------|--------|--------------|
| 1 | **Metadata Scanner** | ✅ Implemented | Quick scan walks directory tree, reads ID3 tags from `.mp3` files, extracts title, author, lector, genre, cover art, duration, and creation date. **Progress feedback** with real-time UI updates. Series auto-detection from title patterns and folder structure. **Gaps:** Only `.mp3` files supported — no `.m4b`, `.ogg`, `.flac`. |
| 2 | **Book Browser** | ✅ Implemented | Paginated list with configurable page size. Sorting by title, author, creation date, score. **Filtering by author, genre, lector, and read status fully wired** to frontend with toggle panel. |
| 3 | **Authors View** | ✅ Implemented | Authors list displayed as a gallery grid with images. Author detail page shows author photo + all associated books as a gallery. |
| 4 | **Narrators (Lectors) View** | ✅ Implemented | Lectors list page showing name and book count. Lector detail page lists associated books via `GenericListComponent`. |
| 5 | **Genres View** | ✅ Implemented | Genres list page showing name and book count. Genre detail page lists associated books via `GenericListComponent`. |
| 6 | **Statistics / Dashboard** | 🔧 Partially | Dashboard displays: total book count, read book count, author count, genre count, lector count, newest books (8), top authors by book count (5), top-rated books (5). **Gaps:** No total duration statistic displayed. No read-over-time chart or trends. No export capability. |
| 7 | **Book Details** | ✅ Implemented | Shows title, cover image, author (link), lector (link), genre (link), duration. Toggle read status. **Score editing** via reusable star rating component (0–10). Series assignment with order. **Gaps:** No description/notes field. |
| 8 | **Series / Cycles** | ✅ Implemented | Full data model with `series` table and book relationships. **Auto-detection** from title patterns (`Series - 01 - Title`) and directory structure (`Author/Series/Book`). Series list page with pagination. Series detail page with ordered book list. Manual series assignment UI on book details. |
| 9 | **UI/UX — Themes & Sizing** | ✅ Implemented | 5 color themes via CSS custom properties with runtime switching. **Theme selection persisted** to localStorage. Toast notification system replacing all `alert()` calls. **Gaps:** Window size fixed at 1360×850 — no responsive/dynamic sizing. |
| 10 | **Tags / Custom Labels** | 🔧 Schema Only | `tags` / `tags_books` tables exist but no service, commands, or UI. Planned for M5. |

---

## 2. Technical Documentation

### 2.1 Architecture Overview

FinalShelf follows a classic **Tauri desktop application** pattern:

```
┌─────────────────────────────────────────────────┐
│                 Angular 17 (SPA)                │
│   Components ← invoke() → Tauri IPC Bridge     │
├─────────────────────────────────────────────────┤
│              Tauri 2.0 Runtime (Rust)           │
│  Commands → Services → Diesel ORM → SQLite DB  │
├─────────────────────────────────────────────────┤
│            File System (audiobook files)        │
│         External drive / portable path          │
└─────────────────────────────────────────────────┘
```

- **Frontend** — Angular 17 standalone-component-based SPA. No Angular services or `HttpClient`; all data access is via Tauri's `invoke()` IPC.
- **Backend** — Rust binary with a 3-layer architecture: `commands` (IPC handlers) → `services` (business logic) → `models` + Diesel ORM.
- **Database** — SQLite via Diesel ORM with embedded migrations. Default path: `./finalshelf.sql` (overridable via `DATABASE_URL` env var).
- **File access** — Audiobook directories are referenced via `absolute_paths` table. Tauri's `asset://` protocol serves cover images using `convertFileSrc()`.

### 2.2 File & Module Structure

#### Backend (`src-tauri/src/`)

```
src-tauri/src/
├── main.rs              # Tauri app builder, plugin registration, command handler registration
├── db.rs                # SQLite connection factory (Diesel), migration runner
├── scanner.rs           # File-system walker, ID3 tag parser, book/author extraction
├── schema.rs            # Auto-generated Diesel schema (5 tables)
├── commands/            # Tauri IPC command handlers (thin layer)
│   ├── mod.rs
│   ├── authors_commands.rs
│   ├── books_commands.rs
│   ├── dashboard_commands.rs
│   ├── genres_commands.rs
│   ├── lectors_commands.rs
│   ├── search_commands.rs
│   └── settings_commands.rs
├── models/              # Diesel ORM models + response DTOs
│   ├── mod.rs
│   ├── author.rs        # Author, AuthorWithBooks
│   ├── book.rs          # Book, BookListResponse
│   ├── dashboard.rs     # Dashboard (aggregate DTO)
│   ├── genre.rs         # Genre, GenreWithBooks
│   ├── lector.rs        # Lector, LectorWithBooks
│   ├── path.rs          # AbsolutePath, NewAbsolutePath
│   ├── query.rs         # QueryParams (pagination, filtering, sorting)
│   └── tag.rs           # Tag, TagBook (unused)
└── services/            # Business logic layer
    ├── mod.rs
    ├── absolute_paths_service.rs
    ├── authors_service.rs
    ├── books_service.rs
    ├── genres_service.rs
    ├── lectors_service.rs
    └── search_service.rs
```

#### Frontend (`src/app/`)

```
src/app/
├── app.component.ts/html/scss   # Root shell: sidebar + topbar + router-outlet
├── app.config.ts                # Angular providers (router only)
├── app.routes.ts                # Route definitions (12 routes)
├── features/
│   ├── authors/
│   │   ├── list/                # AuthorsListPageComponent — gallery grid
│   │   └── details/             # AuthorDetailsPageComponent — photo + book gallery
│   ├── books/
│   │   ├── list/                # BooksListPageComponent — paginated, sorted gallery
│   │   ├── details/             # BookDetailsPageComponent — cover + metadata + read toggle
│   │   └── read-books-list/     # (Commented out) ReadBooksListPageComponent
│   ├── dashboard/               # DashboardPageComponent — stats grid, new books, top lists
│   ├── genres/
│   │   ├── list/                # GenresListPageComponent — name + count grid
│   │   └── details/             # GenresDetailsPageComponent — book gallery
│   ├── lectors/
│   │   ├── list/                # LectorsListPageComponent — name + count grid
│   │   └── details/             # LectorsDetailsPageComponent — book gallery
│   ├── search/                  # SearchPageComponent — query-param-driven book search
│   └── settings/                # SettingsPageComponent — path management, scan, themes
├── models/
│   ├── absolute-paths.ts        # AbsolutePath interface
│   ├── authors.ts               # Author, AuthorDetails interfaces
│   ├── books.ts                 # Book, BookListResponse interfaces
│   ├── genres.ts                # Genre, GenreDetails interfaces
│   └── lectors.ts               # Lector, LectorDetails interfaces
└── shared/
    ├── components/
    │   ├── generic-list/        # GenericListComponent — reusable gallery with pagination/sort
    │   ├── sidebar/             # SidebarComponent — icon navigation, exit button
    │   └── topbar/              # TopbarComponent — breadcrumbs, search bar, clock
    └── utils/
        ├── convertImgPath.ts    # Resolves relative paths → Tauri asset:// URLs
        └── getCurrentAbsolutePath.ts  # Fetches active library root from backend
```

#### Styling (`src/styles/`)

```
src/styles/
├── main.scss          # Entry point — imports all partials
├── _themes.scss       # 5 themes via CSS custom properties (:root, .dark, .light, .lsd, .night-city)
├── _reset.scss        # CSS reset
├── _typography.scss   # Font definitions
├── _gallery.scss      # Gallery grid layout for book/author cards
├── _details.scss      # Detail page layout (cover + metadata table)
└── _mixins.scss       # SCSS mixins
```

### 2.3 Database Schema

After applying all 4 migrations, the database contains **5 active tables** (plus Diesel's internal `__diesel_schema_migrations`).

#### Table: `books`

| Column | Type | Constraints | Added in |
|---|---|---|---|
| `title` | VARCHAR(255) | PRIMARY KEY, UNIQUE, NOT NULL | init |
| `relative_cover_path` | VARCHAR(255) | nullable | init |
| `author_name` | VARCHAR(36) | NOT NULL, FK → `authors.name` ON DELETE CASCADE | init |
| `genre` | VARCHAR(255) | nullable | init |
| `lector` | VARCHAR(255) | nullable | init |
| `create_date` | DATETIME | nullable | init |
| `read` | BOOLEAN | default `FALSE` | migration 3 |
| `score` | INT | default `0` | migration 3 |
| `relative_file_path` | VARCHAR(255) | NOT NULL, default `""` | migration 4 |

#### Table: `authors`

| Column | Type | Constraints | Added in |
|---|---|---|---|
| `name` | VARCHAR(255) | PRIMARY KEY, UNIQUE, NOT NULL | init |
| `relative_img_path` | VARCHAR(255) | nullable | init |

#### Table: `tags` (unused — no service reads or writes this table)

| Column | Type | Constraints | Added in |
|---|---|---|---|
| `id` | INTEGER | PRIMARY KEY, NOT NULL | init |
| `name` | VARCHAR(255) | UNIQUE, NOT NULL | init |

#### Table: `tags_books` (unused — no service reads or writes this table)

| Column | Type | Constraints | Added in |
|---|---|---|---|
| `tag_id` | INTEGER | PK, NOT NULL, FK → `tags.id` ON DELETE CASCADE | init |
| `book_title` | VARCHAR(255) | PK, NOT NULL, FK → `books.title` ON DELETE CASCADE | init |

#### Table: `absolute_paths`

| Column | Type | Constraints | Added in |
|---|---|---|---|
| `id` | INTEGER | PRIMARY KEY AUTOINCREMENT | migration 2 |
| `absolute_path` | TEXT | NOT NULL, UNIQUE | migration 2 |
| `add_date` | DATETIME | NOT NULL, default `datetime('now')` | migration 2 |
| `last_use_date` | DATETIME | nullable | migration 2 |

#### Dropped table: `books_read`

Created in the init migration with columns `id` (PK), `book_title` (FK → books), `rate`, `tier`, `note`, `read_date`. **Dropped in migration 3** when `read` and `score` were added directly to the `books` table.

#### Relationships diagram

```
 authors                books                    tags
┌───────────────┐      ┌────────────────────┐   ┌──────────┐
│ name (PK)     │◄──FK─│ author_name        │   │ id (PK)  │
│ relative_img_ │      │ title (PK)         │   │ name     │
│   path        │      │ relative_cover_path│   └────┬─────┘
└───────────────┘      │ genre              │        │
                       │ lector             │   tags_books
                       │ create_date        │   ┌────┴──────────┐
                       │ read               │   │ tag_id (FK,PK)│
                       │ score              │◄──│ book_title    │
                       │ relative_file_path │   │   (FK, PK)    │
                       └────────────────────┘   └───────────────┘
```

**Key relationships:**

| Relationship | Type | Details |
|---|---|---|
| `books.author_name` → `authors.name` | Many-to-One | FK with `ON DELETE CASCADE` |
| `tags_books.tag_id` → `tags.id` | Many-to-Many (junction) | FK with `ON DELETE CASCADE` |
| `tags_books.book_title` → `books.title` | Many-to-Many (junction) | FK with `ON DELETE CASCADE` |

**Migration history:**

| # | Date | Migration | `up.sql` | `down.sql` status |
|---|------|-----------|----------|--------------------|
| 1 | 2024-12-18 | `init` | Creates 5 tables: `books`, `authors`, `tags`, `tags_books`, `books_read` | ⚠️ **Broken** — references non-existent tables (`genres`, `lectors`, `tags_authors`, `absolute_paths`), uses MySQL syntax (`DROP FOREIGN KEY`) incompatible with SQLite, and names FK constraints that were never defined. Cannot be rolled back. |
| 2 | 2025-01-05 | `absolute_path` | Creates `absolute_paths` table | ✅ `DROP TABLE IF EXISTS absolute_paths` |
| 3 | 2025-04-26 | `save_read_books` | Adds `read` (BOOLEAN) and `score` (INT) columns to `books`; drops `books_read` table | ✅ Drops added columns, re-creates `books_read` |
| 4 | 2025-06-14 | `update_book_model` | Adds `relative_file_path` (VARCHAR(255), NOT NULL, default `""`) to `books` | ✅ `ALTER TABLE books DROP COLUMN relative_file_path` |

**Design notes:**
- **Title as PK** — Book `title` is used as the primary key. Two books with the same title from different authors will collide; the second will be silently skipped by the scanner (`is_book_exists` checks title only).
- **`author_name` column is VARCHAR(36)** — This was likely sized for UUIDs in an earlier design. Author names longer than 36 characters will be truncated at the database level, though SQLite does not enforce VARCHAR length limits by default.
- **Denormalized genre/lector** — Genre and lector are stored as free-text columns in `books` rather than as normalized entities with their own tables. They are treated as "virtual" entities derived via `GROUP BY` queries in the genre and lector services.
- **Unused tables** — The `tags` / `tags_books` tables exist in the schema but are never populated or queried by any service or command.
- **Broken init rollback** — The `down.sql` for the init migration was written for a different schema design (one with separate `genres`, `lectors`, and `tags_authors` tables). It cannot be executed successfully against the actual database.

### 2.4 Backend — Key Modules

#### `scanner.rs` — Metadata Scanner

| Function | Responsibility |
|---|---|
| `quick_scan()` | Main entry point. Resolves the active `absolute_path`, pre-populates already-scanned directories from the DB, walks the directory tree using `WalkDir`, and processes new `.mp3` files. |
| `get_mp3_path()` | Filters `DirEntry` items to only `.mp3` files. |
| `process_metadata()` | Extracts ID3 tags: `album` → title, `album_artist` → author, `artist` → lector, `genre` → genre. Creates author and book records. |
| `look_for_cover()` | Searches a directory for common cover-art filenames (`cover.jpg`, `folder.png`, etc.). |
| `look_for_author_photo()` | Navigates up to the author directory and searches for a cover image. |

#### `services/books_service.rs` — Book CRUD + Queries

| Function | Responsibility |
|---|---|
| `list_books(QueryParams)` | Paginated, filtered, sorted book listing. Supports filtering by author, genre, title, lector, read status. Supports sorting by title, author, create_date, score. |
| `get_book(title)` | Single book lookup by primary key. |
| `add_book(book)` | Inserts a new book record. |
| `update_book(book)` | Updates an existing book (used for marking read status). |
| `get_books_by_author(name)` | All books for a given author. |
| `get_books_by_genre(genre)` | All books for a given genre. |
| `get_books_by_lector(lector)` | All books for a given lector. |
| `get_read_books()` | All books with `read = true`. |
| `get_books_count()` / `get_read_books_count()` | Aggregate counts. |
| `get_books_by_date(limit)` | Newest books ordered by `create_date`. |
| `get_books_by_score(limit)` | Top-scored books. |
| `get_all_book_paths()` | Returns all `relative_file_path` values (used by scanner to skip known dirs). |

#### `services/search_service.rs` — Multi-field Search

Searches across multiple fields (`title`, `author_name`, `genre_name`, `lector_name`) using SQL `LIKE %target%` and deduplicates results by title.

#### `services/absolute_paths_service.rs` — Library Path Management

Manages multiple library root paths. The "current" path is the one with the most recent `last_use_date`. Supports adding, listing, and switching active paths.

#### `commands/` — Tauri IPC Layer

Thin adapter layer that maps Tauri `invoke()` calls to service functions. All 16 registered commands:

| Command | Delegates To |
|---|---|
| `ping_command` | Returns `"ping"` (health check) |
| `quick_scan_command` | `scanner::quick_scan()` |
| `kill_command` | `panic!()` — forcefully terminates the app |
| `add_absolute_path_command` | `absolute_paths_service::add_absolute_path()` |
| `get_all_absolute_path_command` | `absolute_paths_service::get_all_absolute_path()` |
| `set_current_absolute_path_by_id_command` | `absolute_paths_service::set_current_absolute_path_by_id()` |
| `get_current_absolute_path_command` | `absolute_paths_service::get_current_absolute_path()` |
| `search_command` | `search_service::search()` |
| `get_books_list_command` | `books_service::list_books()` |
| `get_book_command` | `books_service::get_book()` |
| `get_all_read_books_command` | `books_service::get_read_books()` |
| `update_book_command` | `books_service::update_book()` |
| `get_authors_list_command` | `authors_service::list_authors()` |
| `get_author_command` | `authors_service::get_author()` |
| `get_lectors_list_command` | `lectors_service::get_lectors_list()` |
| `get_lector_command` | `lectors_service::get_lector()` |
| `get_genres_list_command` | `genres_service::get_genres_list()` |
| `get_genre_command` | `genres_service::get_genre()` |
| `get_dashboard_data_command` | Aggregates data from multiple services |

### 2.5 Frontend — Key Modules

#### `GenericListComponent` (Shared)

Reusable gallery-grid component with optional pagination and sorting. Used by Books list, Dashboard, Search, Lector details, and Genre details. Accepts `items`, `listType` (`"books"` | `"authors"`), `config` (page/size/sort), and emits events for page/sort changes.

#### `SidebarComponent` (Shared)

Fixed vertical icon navigation bar. Defines three menu groups:
- **Main:** Dashboard, Books, Authors, Lectors, Genres
- **User:** Read (→ `/read`, broken), Ranking (→ `/ranking`, broken)
- **Settings:** GitHub link (external), Settings, Exit button

#### `TopbarComponent` (Shared)

Horizontal bar with breadcrumb navigation (auto-generated from current route), search input (navigates to `/search?query=...`), and a live clock.

#### Utility Functions

- `convertImgPathBook(path, absolute_path)` / `convertImgPathAuthor(path, absolute_path)` — Resolves relative cover/photo paths into Tauri `asset://` URLs. Falls back to placeholder images.
- `getCurrentAbsolutePath()` — Invokes `get_current_absolute_path_command` and returns the active library root directory.

### 2.6 Libraries & Technologies

#### Backend (Rust)

| Crate | Version | Purpose |
|---|---|---|
| `tauri` | 2.0 | Desktop app framework, IPC, asset protocol |
| `tauri-plugin-shell` | 2 | Shell command execution |
| `tauri-plugin-dialog` | 2 | Native dialogs |
| `diesel` | 2.2.10 | ORM for SQLite (with `chrono` feature) |
| `diesel_migrations` | 2.0.0 | Embedded database migrations |
| `rusqlite` | 0.32.0 | SQLite driver (bundled) |
| `walkdir` | 2.5.0 | Recursive directory traversal |
| `id3` | 1.16.0 | MP3 ID3 tag parsing |
| `serde` / `serde_json` | 1.0 | Serialization for IPC |
| `chrono` | 0.4.39 | Date/time handling |
| `dotenv` | 0.15.0 | `.env` file support |
| `uuid` | 1 (v4) | UUID generation (imported but unused) |

#### Frontend (TypeScript / Angular)

| Package | Version | Purpose |
|---|---|---|
| `@angular/core` + ecosystem | ^17.0.0 | UI framework (standalone components) |
| `@angular/router` | ^17.0.0 | Client-side routing |
| `@tauri-apps/api` | ^2.1.1 | Tauri IPC bridge (`invoke`, `convertFileSrc`) |
| `@tauri-apps/plugin-dialog` | ^2.2.0 | Dialog plugin API |
| `@tauri-apps/plugin-shell` | ^2.2.0 | Shell plugin API |
| `rxjs` | ~7.8.0 | Reactive extensions (router events) |
| `typescript` | ~5.2.2 | Language |

#### Build & Dev Tools

| Tool | Purpose |
|---|---|
| `@tauri-apps/cli` | Tauri build toolchain |
| `@angular/cli` | Angular CLI for development server and builds |
| `prettier` | Code formatting |
| `eslint` + `angular-eslint` | Linting |
| `karma` + `jasmine` | Unit testing framework (minimal test coverage) |

---

## 3. Development Roadmap

### Phase 1 — MVP

Critical fixes and gaps that affect basic usability.

| # | Task | Description | Priority | Complexity |
|---|------|-------------|----------|------------|
| 1.1 | **Wire frontend filters to book list** | Connect author, genre, lector, title, and read-status filter controls to `get_books_list_command` params (currently hard-coded to `null`). | High | S |
| 1.2 | **Fix broken sidebar routes** | The sidebar has links to `/read` and `/ranking` that lead to 404. Either implement these routes or remove the sidebar entries. | High | S |
| 1.3 | **Persist theme selection** | Save the selected theme to `localStorage` or the SQLite database and restore it on app startup. Currently resets on reload. | High | S |
| 1.4 | **Add scan progress feedback** | The scanner runs synchronously and blocks the UI with no progress indication. Add a progress channel (Tauri events) or at minimum a loading spinner. | High | M |
| 1.5 | **Display and edit book score** | The `score` field exists in the data model but is not shown or editable in the book details UI. Add a rating widget. | Medium | S |
| 1.6 | **Error handling improvements** | Replace `panic!()` in `kill_command`, `.expect()` calls in services, and `alert()` in frontend with proper error handling and user-friendly messages. | High | M |

### Phase 2 — Core Features

Main product features from the requirements list.

| # | Task | Description | Priority | Complexity |
|---|------|-------------|----------|------------|
| 2.1 | **Series / Cycles — data model** | Create a `series` table (`id`, `name`, `author_name`, `description`) and add `series_id` + `series_order` columns to `books`. Write Diesel migration. | High | M |
| 2.2 | **Series / Cycles — auto-detection** | Implement title-pattern detection (`<cycle> - <part> - <title>`) and directory-structure analysis (`author → cycle → books`) in the scanner. | High | L |
| 2.3 | **Series / Cycles — UI** | Create series list page, series detail page (ordered book list), and UI for manual series assignment/editing on book details. | High | L |
| 2.4 | **Book duration extraction** | Extract total duration from audio file metadata (ID3 `TLEN` tag or calculate from file bitrate/size). Add `duration_seconds` column to `books`. | Medium | M |
| 2.5 | **Duration display** | Display formatted duration on book details, book cards, and aggregate totals in dashboard/statistics. | Medium | S |
| 2.6 | **Support additional audio formats** | Extend the scanner to handle `.m4b`, `.m4a`, `.ogg`, `.flac`, `.aac` files using appropriate tag-reading crates (e.g., `lofty` or `symphonia`). | Medium | M |
| 2.7 | **Read Books list page** | Uncomment and fix the `ReadBooksListPageComponent`. Wire it to the existing `get_all_read_books_command` backend. Register the `/read` route. | Medium | S |
| 2.8 | **Ranking page** | Create a `/ranking` page that displays books sorted by score. Wire to existing `get_books_by_score()` or `list_books` with `sort_by=score`. | Medium | S |
| 2.9 | **Full rescan capability** | Add a "Full Rescan" option that clears and re-imports all metadata, complementing the existing "Quick Scan" (incremental). | Medium | M |
| 2.10 | **Author pagination** | `list_authors()` ignores the `QueryParams` pagination/sorting arguments. Implement proper paginated, sorted author listing. | Medium | S |
| 2.11 | **Search — multi-field toggle** | The search currently only queries by `"title"`. Expose checkboxes or toggles in the Search UI so users can search by author, genre, lector simultaneously. | Medium | S |

### Phase 3 — Polish

UX improvements, statistics, and advanced features.

| # | Task | Description | Priority | Complexity |
|---|------|-------------|----------|------------|
| 3.1 | **Advanced statistics page** | Dedicated statistics page with: books read over time, total listening duration, average score, genre distribution chart, most prolific author. | Low | L |
| 3.2 | **Tags / custom labels** | Activate the existing `tags` / `tags_books` schema. Build UI for creating tags and assigning them to books. Enable filtering by tags. | Low | M |
| 3.3 | **Responsive window sizing** | Make the layout responsive: collapsible sidebar on narrow windows, fluid grid breakpoints, respect system DPI. | Low | M |
| 3.4 | **Keyboard navigation** | Add keyboard shortcuts: `Ctrl+F` for search, arrow keys for gallery navigation, `Escape` to go back. | Low | S |
| 3.5 | **Book notes / reviews** | Add a text `notes` field to the book model. Display a notes editor on the book details page. | Low | S |
| 3.6 | **Cover image management** | Allow users to manually set or change cover images. Support drag-and-drop. | Low | M |
| 3.7 | **Database export / import** | Allow exporting the library database to JSON and importing it back (backup/restore). | Low | M |
| 3.8 | **Book title collision prevention** | Replace the `title` primary key with a UUID or composite key (`title` + `author_name`) to prevent silent collisions. Requires migration and FK updates. | Low | L |
| 3.9 | **Lector/Genre normalization** | Move lectors and genres to dedicated tables with proper foreign keys instead of free-text columns, enabling better data integrity and management UIs. | Low | L |
| 3.10 | **Delete / remove books** | Add functionality to remove books (and orphaned authors) from the database. Currently there is no delete capability. | Low | S |
| 3.11 | **Onboarding / first-run wizard** | Guide the user through adding their first library path and running the initial scan on first launch. | Low | M |

---

## 4. Risks and Recommendations

### Risk 1: Book Title as Primary Key

**Problem:** Using `title` (VARCHAR) as the primary key for `books` means two books with the same title from different authors will collide. The second book will fail to insert silently (the scanner checks `is_book_exists` by title only).

**Recommendation:** Introduce a surrogate primary key (auto-incrementing integer or UUID). Create a unique constraint on `(title, author_name)` to preserve logical uniqueness. This requires a significant migration and FK chain update — schedule it early to minimize downstream impact.

### Risk 2: No Connection Pooling

**Problem:** Every service function calls `establish_connection()` to create a fresh `SqliteConnection`. With concurrent Tauri `async` commands, this can lead to:
- "database is locked" errors from simultaneous writers
- Performance overhead from repeatedly opening connections

**Recommendation:** Use a connection pool such as `r2d2` (with `diesel::r2d2::ConnectionManager<SqliteConnection>`) managed as Tauri application state. Pass the pool reference through `State<>` in command handlers.

### Risk 3: Scanner Blocks the Main Thread

**Problem:** `quick_scan_command` is a synchronous `#[tauri::command]` (not `async`). For large libraries (1000+ books), it will block the Tauri command thread and freeze the UI.

**Recommendation:** Make the scanner run in a background `tokio::task::spawn_blocking()` thread. Use Tauri's event system (`app.emit()`) to push progress updates (e.g., percentage, current directory) to the frontend in real-time.

### Risk 4: `panic!()` as Application Exit

**Problem:** `kill_command` uses `panic!()` to terminate the app. This is an unclean shutdown that skips destructors, may corrupt in-flight database writes, and produces alarming error output.

**Recommendation:** Replace with `std::process::exit(0)` or, preferably, Tauri's `app_handle.exit(0)` for a graceful shutdown that runs cleanup hooks.

### Risk 5: No Input Sanitization in Search

**Problem:** `search_service::search()` constructs SQL `LIKE` patterns using `format!("%{}%", target)` without escaping SQL wildcard characters (`%`, `_`). While Diesel parameterizes queries (preventing SQL injection), users can craft searches that return unexpected results.

**Recommendation:** Escape `%` and `_` characters in the search target before constructing the `LIKE` pattern. The commented-out `sanitize()` method in `QueryParams` was a step in this direction — finish and activate it.

### Risk 6: Unvalidated QueryParams

**Problem:** `QueryParams` has commented-out `validate()` and `sanitize()` methods. Invalid values for `sort_by`, `sort_order`, and `page` are silently handled with defaults, but negative page sizes or malicious strings could cause unexpected behavior.

**Recommendation:** Uncomment, complete, and call the validation/sanitization logic before executing queries.

### Risk 7: No Data Deletion Capability

**Problem:** There are no commands or UI to delete books, authors, or tags. If the scanner imports incorrect data (e.g., malformed tags produce garbage entries), the user has no way to clean up without directly editing the SQLite file.

**Recommendation:** Implement delete operations for books (with cascade to `tags_books`) and orphaned authors. Add a confirmation dialog in the UI.

### Risk 8: Hardcoded Frontend Dev URL

**Problem:** `tauri.conf.json` sets `devUrl: "http://localhost:1420"`, but `angular.json` likely defaults to port 4200. Ensure these are aligned or use `ng serve --port 1420`.

**Recommendation:** Verify the Angular dev server port matches the Tauri config, or use a proxy configuration.

### Risk 9: Theme Not Persisted

**Problem:** Theme selection is applied by toggling CSS classes on `<body>` but is never saved. On page reload or app restart, the theme reverts to the default.

**Recommendation:** Persist the selected theme in `localStorage` on the frontend and apply it in `AppComponent.ngOnInit()`. Alternatively, store it in the SQLite database alongside other user preferences.

### Risk 10: Missing `relative_file_path` for Pre-Migration Books

**Problem:** Migration 4 added `relative_file_path` with a default of `""`. Existing books in the database will have an empty file path, which may cause issues if this field is used for playback or validation.

**Recommendation:** Either run a one-time backfill migration that attempts to reconstruct file paths from the scanner, or mark empty paths in the UI with a "rescan needed" indicator.

### Risk 11: Broken Down-Migrations Prevent Rollback

**Problem:** The `down.sql` for the init migration is completely out of sync with the corresponding `up.sql`. It references tables that were never created (`genres`, `lectors`, `tags_authors`), tries to drop `absolute_paths` (which belongs to migration 2), uses MySQL-specific `ALTER TABLE DROP FOREIGN KEY` syntax that SQLite does not support, and names FK constraints (`books_lector_id_fk`, `books_genre_id_fk`, `books_author_id_fk`) that were never defined. Running `diesel migration revert` on the init migration will fail.

**Recommendation:** Rewrite the init `down.sql` to match the actual `up.sql`. The correct rollback should be:
```sql
DROP TABLE IF EXISTS tags_books;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS books;
DROP TABLE IF EXISTS authors;
```
Note: `books_read` should not be in the init `down.sql` since it was dropped by migration 3 before any rollback of init would occur. Consider also adding integration tests that verify all migrations can be applied and reverted cleanly.

### Risk 12: `author_name` Column Sized for UUIDs

**Problem:** The `books.author_name` column is defined as `VARCHAR(36)` — a length typically used for UUIDs. While SQLite does not enforce VARCHAR length limits (it stores any-length text), this sizing suggests the column was originally designed for a UUID foreign key, not for human-readable names. If the application is ever ported to a strict-typing database (PostgreSQL, MySQL with strict mode), author names longer than 36 characters would be silently truncated or cause insertion errors.

**Recommendation:** Update the column definition to `VARCHAR(255)` or `TEXT` to match the `authors.name` column type and avoid future portability issues.

---

*End of document.*
