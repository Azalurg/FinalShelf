# FinalShelf — Development Plan

> **Version:** 0.4.2
> **Generated:** 2026-03-05
> **Stack:** Tauri 2.0 · Angular 17 · Diesel (SQLite) · Rust

---

## Overview

| Milestone | Stories | Tasks | Estimated Time |
|---|---|---|---|
| [M1: Stability & Error Handling](#milestone-1-stability--error-handling) | 3 | 10 | 1–2 weeks |
| [M2: UI/UX Polish](#milestone-2-uiux-polish) | 4 | 13 | 2–3 weeks |
| [M3: Book Management](#milestone-3-book-management) | 3 | 9 | 1–2 weeks |
| [M4: Series & Cycles](#milestone-4-series--cycles) | 3 | 11 | 3–4 weeks |
| [M5: Tags & Custom Labels](#milestone-5-tags--custom-labels) | 2 | 7 | 1–2 weeks |
| [M6: Advanced Statistics & Data Export](#milestone-6-advanced-statistics--data-export) | 3 | 9 | 2–3 weeks |
| [M7: Infrastructure & Quality](#milestone-7-infrastructure--quality) | 3 | 9 | 2–3 weeks |
| **Total** | **21** | **68** | **12–19 weeks** |

---

## Milestone 1: Stability & Error Handling

Eliminate panics, unwraps, and `alert()` calls. Establish robust error handling across the full stack so the app never crashes unexpectedly.

> **Estimated time:** 1–2 weeks

---

### Story 1.1: Replace panic-based app exit with graceful shutdown — S {#story-1-1}

**As a** user **I want** the app to shut down cleanly when I press Exit **so that** no data is lost and no error dialogs appear.

**Acceptance criteria:**
- Exit button terminates the app without a panic stack trace
- In-flight DB writes complete before process exit
- No `panic!()` used for intentional control flow anywhere in the codebase

#### Task 1.1.1: Replace `kill_command` panic with graceful exit {#task-1-1-1}

**Type:** Bug

**Description:** `kill_command` in `settings_commands.rs` uses `panic!()` to terminate the app. Replace with `app_handle.exit(0)` via Tauri's managed state or `std::process::exit(0)`.

**Files/components:**
- `src-tauri/src/commands/settings_commands.rs`
- `src-tauri/src/main.rs` (pass `AppHandle` if needed)

**Dependencies:** None

**DoD:** Exit button closes the app cleanly with exit code 0; no panic output in stderr.

#### Task 1.1.2: Replace `expect()`/`unwrap()` in services with `Result` returns {#task-1-1-2}

**Type:** Bug

**Description:** `add_author()` uses `.expect()`, `get_all_absolute_path()` uses `.expect()`, `run_migrations()` uses `.unwrap()`. Replace these with proper `Result<_, String>` returns and propagate errors to the command layer.

**Files/components:**
- `src-tauri/src/services/authors_service.rs` (line 110)
- `src-tauri/src/services/absolute_paths_service.rs` (line 25)
- `src-tauri/src/db.rs` (lines 15, 26)

**Dependencies:** None

**DoD:** No `expect()` or `unwrap()` calls remain in service/db code; all errors are returned as `Result::Err`.

---

### Story 1.2: Replace frontend `alert()` calls with proper UI notifications — S {#story-1-2}

**As a** user **I want** feedback on operations (scan results, errors) displayed as in-app notifications **so that** I get a consistent, non-intrusive UX instead of browser-style alert dialogs.

**Acceptance criteria:**
- No `alert()` or `prompt()` calls remain in the codebase
- Notifications appear as styled toast/banner components
- Error messages are human-readable

#### Task 1.2.1: Create a shared toast notification component {#task-1-2-1}

**Type:** Feature

**Description:** Build a reusable `ToastComponent` that displays success/error/info messages with auto-dismiss (configurable duration) and a close button.

**Files/components:**
- `src/app/shared/components/toast/toast.component.ts` (new)
- `src/app/shared/components/toast/toast.component.html` (new)
- `src/app/shared/components/toast/toast.component.scss` (new)
- `src/app/shared/components/toast/toast.service.ts` (new)

**Dependencies:** None

**DoD:** Toast component renders in the app shell; service exposes `show(message, type, duration)` API; unit test or manual verification.

#### Task 1.2.2: Replace all `alert()` calls in settings with toast notifications {#task-1-2-2}

**Type:** Bug

**Description:** The settings component has 8 `alert()` calls for scan results, path operations, and errors. Replace with `ToastService.show()`.

**Files/components:**
- `src/app/features/settings/settings.component.ts`

**Dependencies:** [Task 1.2.1](#task-1-2-1)

**DoD:** No `alert()` calls remain in `settings.component.ts`; scan results and errors display as toast notifications.

#### Task 1.2.3: Replace `alert()` in sidebar exit handler {#task-1-2-3}

**Type:** Bug

**Description:** The sidebar component has an `alert("Error")` call in the exit handler. Replace with toast.

**Files/components:**
- `src/app/shared/components/sidebar/sidebar.component.ts`

**Dependencies:** [Task 1.2.1](#task-1-2-1)

**DoD:** No `alert()` calls remain in `sidebar.component.ts`.

#### Task 1.2.4: Replace `prompt()` with native dialog for adding paths {#task-1-2-4}

**Type:** Feature

**Description:** `settings.component.ts` uses `prompt()` for entering library paths. Replace with Tauri's native dialog (`@tauri-apps/plugin-dialog`) for directory selection.

**Files/components:**
- `src/app/features/settings/settings.component.ts`

**Dependencies:** [Task 1.2.2](#task-1-2-2)

**DoD:** Adding a path opens a native OS directory picker; selected path is sent to the backend.

---

### Story 1.3: Database connection robustness — S {#story-1-3}

**As a** system **I want** database connections to be managed via a connection pool **so that** concurrent commands don't cause "database is locked" errors.

**Acceptance criteria:**
- A connection pool (e.g., `r2d2`) is initialized at app startup and shared as Tauri state
- All services receive a pooled connection instead of calling `establish_connection()` directly
- No raw `SqliteConnection::establish()` calls remain in service code

#### Task 1.3.1: Introduce `r2d2` connection pool as Tauri managed state {#task-1-3-1}

**Type:** Refactor

**Description:** Add `r2d2` + `diesel::r2d2::ConnectionManager<SqliteConnection>` to `Cargo.toml` and initialize the pool in `main.rs` as Tauri managed state. Update `db.rs` to expose the pool type.

**Files/components:**
- `src-tauri/Cargo.toml`
- `src-tauri/src/db.rs`
- `src-tauri/src/main.rs`

**Dependencies:** None

**DoD:** Pool is created at startup and available as `State<DbPool>` in commands.

#### Task 1.3.2: Update all services and commands to use pooled connections {#task-1-3-2}

**Type:** Refactor

**Description:** Replace all `establish_connection()` calls in services with a `&mut SqliteConnection` parameter. Update commands to extract a connection from the pool and pass it to services.

**Files/components:**
- `src-tauri/src/services/books_service.rs`
- `src-tauri/src/services/authors_service.rs`
- `src-tauri/src/services/genres_service.rs`
- `src-tauri/src/services/lectors_service.rs`
- `src-tauri/src/services/absolute_paths_service.rs`
- `src-tauri/src/commands/*.rs`
- `src-tauri/src/scanner.rs`

**Dependencies:** [Task 1.3.1](#task-1-3-1)

**DoD:** Zero `establish_connection()` calls remain outside `db.rs`; all data access uses pooled connections.

---

## Milestone 2: UI/UX Polish

Persist user preferences, improve the book details page, and enhance the overall user experience.

> **Estimated time:** 2–3 weeks

---

### Story 2.1: Persist theme selection — S {#story-2-1}

**As a** user **I want** my chosen theme to persist across app restarts **so that** I don't have to re-select it every time.

**Acceptance criteria:**
- Selected theme is saved to `localStorage`
- On app startup, theme is restored before first render
- Settings page reflects the current persisted theme

#### Task 2.1.1: Save and restore theme from `localStorage` {#task-2-1-1}

**Type:** Feature

**Description:** In `AppComponent.ngOnInit()`, read the saved theme from `localStorage` and apply the CSS class to `<body>`. In `SettingsPageComponent.updateTheme()`, persist the selection to `localStorage`.

**Files/components:**
- `src/app/app.component.ts`
- `src/app/features/settings/settings.component.ts`

**Dependencies:** None

**DoD:** Theme selection survives app restart; `selectedTheme` property reflects the persisted value on settings page load.

---

### Story 2.2: Book score editing — M {#story-2-2}

**As a** user **I want** to rate books on the details page **so that** I can track my personal scoring and see rankings.

**Acceptance criteria:**
- Book details page shows the current score (0–10)
- User can change the score via a clickable star/number widget
- Updated score is persisted to the database immediately
- Ranking page reflects the updated score

**CONFLICT:** Documentation (§1 row 7) states "Score field exists in the model but is **not displayed or editable** in the UI." However, the current `details.component.html` already **displays** score as `{{ bookDetails.score || '—' }}`. The display is implemented, but editing is not. Resolution: only implement score editing UI.

#### Task 2.2.1: Create a reusable score input component {#task-2-2-1}

**Type:** Feature

**Description:** Build a `ScoreInputComponent` that renders clickable stars or number buttons (0–10). Emits a `scoreChange` event with the new value.

**Files/components:**
- `src/app/shared/components/score-input/score-input.component.ts` (new)
- `src/app/shared/components/score-input/score-input.component.html` (new)
- `src/app/shared/components/score-input/score-input.component.scss` (new)

**Dependencies:** None

**DoD:** Component renders with a numeric score; clicking updates the displayed value and emits the event.

#### Task 2.2.2: Integrate score editing into book details page {#task-2-2-2}

**Type:** Feature

**Description:** Replace the static score display in `details.component.html` with `<app-score-input>`. On score change, call `update_book_command` to persist.

**Files/components:**
- `src/app/features/books/details/details.component.html`
- `src/app/features/books/details/details.component.ts`

**Dependencies:** [Task 2.2.1](#task-2-2-1)

**DoD:** Score is editable on the book details page; changes persist after navigation and app restart.

---

### Story 2.3: Wire frontend filters to book list — M {#story-2-3}

**As a** user **I want** to filter books by author, genre, lector, and read status directly from the books list page **so that** I can narrow down large collections.

**Acceptance criteria:**
- Filter controls (dropdowns, toggles) are visible on the books list page
- Selecting a filter re-fetches the list with the corresponding backend params
- Filters can be combined (e.g., genre + read status)
- Clearing a filter restores the unfiltered view
- Current page resets to 1 when filters change

#### Task 2.3.1: Add filter UI controls to books list page {#task-2-3-1}

**Type:** Feature

**Description:** Add filter dropdowns (author, genre, lector) and a read-status toggle to the books list template. Populate dropdowns from backend list endpoints.

**Files/components:**
- `src/app/features/books/list/list.component.html`
- `src/app/features/books/list/list.component.ts`
- `src/app/features/books/list/list.component.scss`

**Dependencies:** None

**DoD:** Filter controls render with populated options; selecting one triggers `fetchBooks()` with the appropriate filter param.

#### Task 2.3.2: Pass filter params from frontend to `get_books_list_command` {#task-2-3-2}

**Type:** Feature

**Description:** Update `fetchBooks()` to include `author_name`, `genre`, `lector`, and `read_status` params in the `invoke()` call based on the selected filter values.

**Files/components:**
- `src/app/features/books/list/list.component.ts`

**Dependencies:** [Task 2.3.1](#task-2-3-1)

**DoD:** Filtering by any combination of author/genre/lector/read-status returns correctly filtered results.

---

### Story 2.4: Scan progress feedback — M {#story-2-4}

**As a** user **I want** to see progress during library scanning **so that** I know the operation is running and how far along it is.

**Acceptance criteria:**
- During scan, a progress indicator shows on the settings page
- Progress updates include: current phase, items processed, elapsed time
- Scan button is disabled while scanning
- Completion result is shown as a toast notification

#### Task 2.4.1: Add Tauri event emission to scanner for progress updates {#task-2-4-1}

**Type:** Feature

**Description:** Modify `scanner.rs` to accept an `AppHandle` and emit progress events (`scan-progress`) with phase name, current count, and total (when known). Make `quick_scan_command` and `full_scan_command` `async` and pass the app handle.

**Files/components:**
- `src-tauri/src/scanner.rs`
- `src-tauri/src/commands/settings_commands.rs`
- `src-tauri/src/main.rs`

**Dependencies:** None

**DoD:** Tauri events are emitted during scan with phase/count data; can be verified via Tauri devtools.

#### Task 2.4.2: Add progress listener and UI indicator to settings page {#task-2-4-2}

**Type:** Feature

**Description:** Listen for `scan-progress` events in the settings component. Show a progress bar or status text. Disable scan buttons during operation. Show toast on completion.

**Files/components:**
- `src/app/features/settings/settings.component.ts`
- `src/app/features/settings/settings.component.html`
- `src/app/features/settings/settings.component.scss`

**Dependencies:** [Task 2.4.1](#task-2-4-1), [Task 1.2.1](#task-1-2-1)

**DoD:** Progress is displayed in real-time during scan; buttons are disabled; completion message appears as toast.

---

## Milestone 3: Book Management

Improve book-level features: notes, deletion, and image management.

> **Estimated time:** 1–2 weeks

---

### Story 3.1: Delete books and orphaned authors — S {#story-3-1}

**As a** user **I want** to remove incorrectly imported books from the library **so that** my collection only contains relevant entries.

**Acceptance criteria:**
- Book details page has a "Delete" button
- Deleting a book removes it from the database (cascade removes `tags_books` entries)
- If the author has no remaining books, the author record is also removed
- A confirmation dialog appears before deletion

#### Task 3.1.1: Add `delete_book` service and command {#task-3-1-1}

**Type:** Feature

**Description:** Create `books_service::delete_book(title)` that deletes the book record and, if the author has no remaining books, deletes the author. Register `delete_book_command` in Tauri.

**Files/components:**
- `src-tauri/src/services/books_service.rs`
- `src-tauri/src/commands/books_commands.rs`
- `src-tauri/src/main.rs`

**Dependencies:** None

**DoD:** Calling `delete_book_command` with a title removes the book and orphaned author; returns `Ok(())`.

#### Task 3.1.2: Add delete button with confirmation to book details UI {#task-3-1-2}

**Type:** Feature

**Description:** Add a "Delete" button to the book details page. On click, show a Tauri native confirmation dialog. On confirm, call `delete_book_command` and navigate back to the books list.

**Files/components:**
- `src/app/features/books/details/details.component.html`
- `src/app/features/books/details/details.component.ts`

**Dependencies:** [Task 3.1.1](#task-3-1-1)

**DoD:** Deleting a book navigates to `/books`; the book no longer appears in listings.

---

### Story 3.2: Book notes / reviews — S {#story-3-2}

**As a** user **I want** to add personal notes to books **so that** I can record my thoughts and summaries.

**Acceptance criteria:**
- Book details page shows a text area for notes
- Notes are saved to the database on blur or with a save button
- Notes persist across sessions

#### Task 3.2.1: Add `notes` column to books table {#task-3-2-1}

**Type:** Feature

**Description:** Create a Diesel migration adding `notes TEXT DEFAULT NULL` to the `books` table. Update the `Book` model.

**Files/components:**
- `src-tauri/migrations/{timestamp}_add_book_notes/up.sql` (new)
- `src-tauri/migrations/{timestamp}_add_book_notes/down.sql` (new)
- `src-tauri/src/models/book.rs`
- `src-tauri/src/schema.rs` (auto-generated)
- `src/app/models/books.ts`

**Dependencies:** None

**DoD:** Migration applies cleanly; `Book` struct includes `notes: Option<String>`.

#### Task 3.2.2: Add notes editor to book details page {#task-3-2-2}

**Type:** Feature

**Description:** Add a `<textarea>` to the book details template bound to `bookDetails.notes`. On blur, call `update_book_command` to persist.

**Files/components:**
- `src/app/features/books/details/details.component.html`
- `src/app/features/books/details/details.component.ts`

**Dependencies:** [Task 3.2.1](#task-3-2-1)

**DoD:** Notes are editable and persist after page navigation and app restart.

---

### Story 3.3: Manual cover image management — S {#story-3-3}

**As a** user **I want** to set or replace a book's cover image **so that** books with missing or incorrect covers display properly.

**Acceptance criteria:**
- Book details page has a "Change cover" button
- Clicking opens a native file picker (image formats only)
- Selected image is copied to the book's directory and the database path is updated

#### Task 3.3.1: Add `update_cover` backend command {#task-3-3-1}

**Type:** Feature

**Description:** Create a command that accepts a book title and an absolute image path, copies the image to the book's directory, updates `relative_cover_path`, and returns the updated book.

**Files/components:**
- `src-tauri/src/services/books_service.rs`
- `src-tauri/src/commands/books_commands.rs`
- `src-tauri/src/main.rs`

**Dependencies:** None

**DoD:** Command copies image and updates DB; returns updated `Book` with new cover path.

#### Task 3.3.2: Add cover change button to book details UI {#task-3-3-2}

**Type:** Feature

**Description:** Add a "Change cover" button. On click, open Tauri's file dialog (image filter), then call `update_cover_command`. Refresh the displayed image.

**Files/components:**
- `src/app/features/books/details/details.component.html`
- `src/app/features/books/details/details.component.ts`

**Dependencies:** [Task 3.3.1](#task-3-3-1)

**DoD:** Clicking the cover or button lets the user select a new image that immediately renders.

---

## Milestone 4: Series & Cycles

Introduce series/cycles as a core organizational concept, with auto-detection from metadata and a dedicated UI.

> **Estimated time:** 3–4 weeks

---

### Story 4.1: Series data model — M {#story-4-1}

**As a** system **I want** a `series` table and relationships to books **so that** books can be organized into ordered series.

**Acceptance criteria:**
- `series` table exists with `id`, `name`, `author_name` (FK), `description`
- `books` table has `series_id` (FK, nullable) and `series_order` (INT, nullable) columns
- Migration applies and reverts cleanly

#### Task 4.1.1: Create `series` table migration {#task-4-1-1}

**Type:** Feature

**Description:** Write a Diesel migration that creates the `series` table and adds `series_id` + `series_order` columns to `books`. Include `down.sql` that reverses the changes.

**Files/components:**
- `src-tauri/migrations/{timestamp}_add_series/up.sql` (new)
- `src-tauri/migrations/{timestamp}_add_series/down.sql` (new)

**Dependencies:** None

**DoD:** `diesel migration run` and `diesel migration revert` both succeed.

#### Task 4.1.2: Create Series model and update Book model {#task-4-1-2}

**Type:** Feature

**Description:** Add `Series` struct (Queryable/Insertable) to `src-tauri/src/models/series.rs`. Add `series_id` and `series_order` fields to the `Book` model. Update `schema.rs`. Register in `mod.rs`.

**Files/components:**
- `src-tauri/src/models/series.rs` (new)
- `src-tauri/src/models/book.rs`
- `src-tauri/src/models/mod.rs`
- `src-tauri/src/schema.rs` (auto-generated)

**Dependencies:** [Task 4.1.1](#task-4-1-1)

**DoD:** `cargo build` succeeds; new model is available for use in services.

#### Task 4.1.3: Create series service with CRUD operations {#task-4-1-3}

**Type:** Feature

**Description:** Create `series_service.rs` with: `list_series(ListParams)`, `get_series(id)`, `create_series(name, author)`, `assign_book_to_series(title, series_id, order)`. Add corresponding commands and register them.

**Files/components:**
- `src-tauri/src/services/series_service.rs` (new)
- `src-tauri/src/services/mod.rs`
- `src-tauri/src/commands/series_commands.rs` (new)
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/main.rs`

**Dependencies:** [Task 4.1.2](#task-4-1-2)

**DoD:** All CRUD operations work via Tauri `invoke()`; returns proper `ListResponse<Series>` for list.

#### Task 4.1.4: Update frontend models for series {#task-4-1-4}

**Type:** Feature

**Description:** Add `Series` interface to frontend models. Update `Book` interface to include `series_id` and `series_order`. Add route for series list and detail.

**Files/components:**
- `src/app/models/series.ts` (new)
- `src/app/models/books.ts`
- `src/app/app.routes.ts`

**Dependencies:** [Task 4.1.3](#task-4-1-3)

**DoD:** TypeScript models match Rust models; routes are registered.

---

### Story 4.2: Series auto-detection in scanner — L {#story-4-2}

**As a** system **I want** the scanner to automatically detect series from title patterns and directory structure **so that** books are grouped into series without manual effort.

**Acceptance criteria:**
- Title patterns like `<Series> - <Part> - <Title>` are detected and parsed
- Directory structure `author/series/book` is used as a fallback
- Detected series are created automatically with correct ordering
- Existing series are reused (no duplicates)

#### Task 4.2.1: Implement title-pattern series detection {#task-4-2-1}

**Type:** Feature

**Description:** In the scanner's extraction phase, parse the album/title tag for common patterns: `Series - Part N - Title`, `Series #N: Title`, `Series (N) Title`. Extract series name and order number.

**Files/components:**
- `src-tauri/src/scanner.rs`

**Dependencies:** [Task 4.1.3](#task-4-1-3)

**DoD:** Scanner correctly parses at least 3 common title patterns and extracts series name + order.

#### Task 4.2.2: Implement directory-based series inference {#task-4-2-2}

**Type:** Feature

**Description:** When title-pattern detection fails, check if the book's directory is inside a parent directory (between the author dir and the book dir) that represents a series. Use alphabetical or natural sort order.

**Files/components:**
- `src-tauri/src/scanner.rs`

**Dependencies:** [Task 4.2.1](#task-4-2-1)

**DoD:** Books in `author/series_name/book_title/` structure are assigned to the detected series.

#### Task 4.2.3: Persist detected series during scan {#task-4-2-3}

**Type:** Feature

**Description:** In the persist phase, create series records if they don't exist, then assign books to the series with the detected order.

**Files/components:**
- `src-tauri/src/scanner.rs`
- `src-tauri/src/services/series_service.rs`

**Dependencies:** [Task 4.2.1](#task-4-2-1), [Task 4.2.2](#task-4-2-2)

**DoD:** After scanning a library with series-organized books, series records exist in DB with correctly ordered books.

---

### Story 4.3: Series UI — M {#story-4-3}

**As a** user **I want** to browse series, see ordered book lists, and manually assign/reorder books **so that** I can manage my series collections.

**Acceptance criteria:**
- Series list page at `/series` with pagination
- Series detail page showing ordered books
- Book details page shows series name + link
- Manual series assignment UI on book details

#### Task 4.3.1: Create series list page {#task-4-3-1}

**Type:** Feature

**Description:** Create `SeriesListPageComponent` with pagination and sorting (by name, book count). Register at `/series` route. Add sidebar entry.

**Files/components:**
- `src/app/features/series/list/series-list.component.ts` (new)
- `src/app/features/series/list/series-list.component.html` (new)
- `src/app/features/series/list/series-list.component.scss` (new)
- `src/app/app.routes.ts`
- `src/app/shared/components/sidebar/sidebar.component.ts`

**Dependencies:** [Task 4.1.4](#task-4-1-4)

**DoD:** Series list renders with pagination at `/series`; sidebar shows the link.

#### Task 4.3.2: Create series detail page {#task-4-3-2}

**Type:** Feature

**Description:** Create `SeriesDetailPageComponent` that shows series name, author link, and ordered book list (using `GenericListComponent` or numbered list).

**Files/components:**
- `src/app/features/series/details/series-details.component.ts` (new)
- `src/app/features/series/details/series-details.component.html` (new)
- `src/app/features/series/details/series-details.component.scss` (new)
- `src/app/app.routes.ts`

**Dependencies:** [Task 4.3.1](#task-4-3-1)

**DoD:** Series detail page shows ordered books; clicking a book navigates to its details.

#### Task 4.3.3: Add series info and assignment UI to book details {#task-4-3-3}

**Type:** Feature

**Description:** On the book details page, show the series name (linked to series detail) and order number. Add a dropdown to assign or change series and a number input for order.

**Files/components:**
- `src/app/features/books/details/details.component.html`
- `src/app/features/books/details/details.component.ts`

**Dependencies:** [Task 4.3.2](#task-4-3-2)

**DoD:** Book details show series info; user can assign a book to a series with a chosen order.

---

## Milestone 5: Tags & Custom Labels

Activate the existing but unused `tags`/`tags_books` schema and build tagging functionality.

> **Estimated time:** 1–2 weeks

---

### Story 5.1: Tags backend — M {#story-5-1}

**As a** system **I want** CRUD operations for tags and tag-book associations **so that** the tagging feature is fully functional at the API level.

**Acceptance criteria:**
- Tags can be created, listed, and deleted
- Books can have multiple tags assigned and unassigned
- Tags are included in book detail responses
- Filtering books by tag is supported in `list_books`

**CONFLICT:** Documentation (§2.3) says `tags`/`tags_books` tables exist but are "completely unused — no service reads or writes this table." The model files (`tag.rs`) exist with struct definitions but no service, no commands, and no UI. The tables exist in the schema. Resolution: implement the full service/command layer for the existing schema.

#### Task 5.1.1: Create tags service {#task-5-1-1}

**Type:** Feature

**Description:** Create `tags_service.rs` with: `list_tags()`, `create_tag(name)`, `delete_tag(id)`, `assign_tag(tag_id, book_title)`, `unassign_tag(tag_id, book_title)`, `get_tags_for_book(title)`.

**Files/components:**
- `src-tauri/src/services/tags_service.rs` (new)
- `src-tauri/src/services/mod.rs`

**Dependencies:** None

**DoD:** All tag CRUD and assignment operations work correctly.

#### Task 5.1.2: Create tags commands and register them {#task-5-1-2}

**Type:** Feature

**Description:** Create `tags_commands.rs` with Tauri commands wrapping the service. Register in `main.rs`.

**Files/components:**
- `src-tauri/src/commands/tags_commands.rs` (new)
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/main.rs`

**Dependencies:** [Task 5.1.1](#task-5-1-1)

**DoD:** All tag commands are callable via `invoke()`.

#### Task 5.1.3: Add tag filter to `list_books` {#task-5-1-3}

**Type:** Feature

**Description:** Add a `tag` filter to `ListParams` and implement the JOIN-based filtering in `books_service::list_books()`.

**Files/components:**
- `src-tauri/src/models/query.rs`
- `src-tauri/src/services/books_service.rs`

**Dependencies:** [Task 5.1.1](#task-5-1-1)

**DoD:** `list_books` with `tag` filter returns only books having that tag.

---

### Story 5.2: Tags UI — M {#story-5-2}

**As a** user **I want** to create tags and assign them to books **so that** I can organize my library with custom labels.

**Acceptance criteria:**
- Book details page shows assigned tags as chips
- User can add/remove tags from a book
- A tag management section exists in settings (create/delete tags)

#### Task 5.2.1: Add tag management UI to settings {#task-5-2-1}

**Type:** Feature

**Description:** Add a "Tags" section to settings page that lists existing tags and allows creating new ones (text input + button) and deleting.

**Files/components:**
- `src/app/features/settings/settings.component.html`
- `src/app/features/settings/settings.component.ts`
- `src/app/models/tags.ts` (new)

**Dependencies:** [Task 5.1.2](#task-5-1-2)

**DoD:** Tags can be created and deleted from the settings page.

#### Task 5.2.2: Add tag chips to book details page {#task-5-2-2}

**Type:** Feature

**Description:** On book details, display assigned tags as styled chips. Include a "+" button that opens a dropdown of available tags, and an "×" on each chip to remove.

**Files/components:**
- `src/app/features/books/details/details.component.html`
- `src/app/features/books/details/details.component.ts`
- `src/app/features/books/details/details.component.scss`

**Dependencies:** [Task 5.2.1](#task-5-2-1)

**DoD:** Tags are visible, assignable, and removable on the book details page.

#### Task 5.2.3: Add tag filter to books list page {#task-5-2-3}

**Type:** Feature

**Description:** Add a tag filter dropdown to the books list page filter controls, wired to the `tag` filter param.

**Files/components:**
- `src/app/features/books/list/list.component.html`
- `src/app/features/books/list/list.component.ts`

**Dependencies:** [Task 5.1.3](#task-5-1-3), [Task 2.3.1](#task-2-3-1)

**DoD:** Books can be filtered by tag on the list page.

---

## Milestone 6: Advanced Statistics & Data Export

Build a dedicated statistics page and database export/import capabilities.

> **Estimated time:** 2–3 weeks

---

### Story 6.1: Advanced statistics page — L {#story-6-1}

**As a** user **I want** a dedicated statistics page with charts and insights **so that** I can understand my reading/listening patterns.

**Acceptance criteria:**
- Statistics page accessible from the sidebar
- Shows: total books, read books, total duration, avg score, genre distribution, author distribution, books-read-over-time timeline
- At least genre distribution and author top-N use visual charts (bar/pie)

#### Task 6.1.1: Create statistics backend aggregation command {#task-6-1-1}

**Type:** Feature

**Description:** Create `get_statistics_command` that returns aggregated data: genre distribution (name + count), top 10 authors by book count, score distribution (0–10), books by creation month, total duration sum.

**Files/components:**
- `src-tauri/src/services/books_service.rs` (add aggregation functions)
- `src-tauri/src/commands/dashboard_commands.rs` (or new `statistics_commands.rs`)
- `src-tauri/src/main.rs`

**Dependencies:** None

**DoD:** Command returns all aggregation data in a typed struct.

#### Task 6.1.2: Create statistics page component {#task-6-1-2}

**Type:** Feature

**Description:** Build a `StatisticsPageComponent` that displays key metrics as cards and distributions as simple bar charts (pure CSS or lightweight library). Register at `/statistics` route.

**Files/components:**
- `src/app/features/statistics/statistics.component.ts` (new)
- `src/app/features/statistics/statistics.component.html` (new)
- `src/app/features/statistics/statistics.component.scss` (new)
- `src/app/app.routes.ts`
- `src/app/shared/components/sidebar/sidebar.component.ts`

**Dependencies:** [Task 6.1.1](#task-6-1-1)

**DoD:** Statistics page renders with live data; sidebar links to it.

---

### Story 6.2: Database export / import — M {#story-6-2}

**As a** user **I want** to export my library database to JSON and import it back **so that** I can backup and restore my data.

**Acceptance criteria:**
- Export produces a JSON file containing all books, authors, tags, and paths
- Import reads a JSON file and upserts records into the database
- Import does not duplicate existing records
- Native save/open dialogs are used for file selection

#### Task 6.2.1: Create export backend command {#task-6-2-1}

**Type:** Feature

**Description:** Create `export_database_command(path)` that serializes all books, authors, `tags`/`tags_books`, and `absolute_paths` to a JSON file at the given path.

**Files/components:**
- `src-tauri/src/commands/settings_commands.rs`
- `src-tauri/src/main.rs`

**Dependencies:** None

**DoD:** Export produces valid JSON containing all data.

#### Task 6.2.2: Create import backend command {#task-6-2-2}

**Type:** Feature

**Description:** Create `import_database_command(path)` that reads a JSON file and upserts records. Use `INSERT OR REPLACE` semantics.

**Files/components:**
- `src-tauri/src/commands/settings_commands.rs`
- `src-tauri/src/main.rs`

**Dependencies:** [Task 6.2.1](#task-6-2-1)

**DoD:** Importing an exported file restores all data correctly.

#### Task 6.2.3: Add export/import buttons to settings UI {#task-6-2-3}

**Type:** Feature

**Description:** Add "Export Database" and "Import Database" buttons to settings. Use native file picker for save/open. Show progress and result via toast.

**Files/components:**
- `src/app/features/settings/settings.component.html`
- `src/app/features/settings/settings.component.ts`

**Dependencies:** [Task 6.2.1](#task-6-2-1), [Task 6.2.2](#task-6-2-2), [Task 1.2.1](#task-1-2-1)

**DoD:** Export/import round-trip works from the UI.

---

### Story 6.3: Dashboard duration statistic — S {#story-6-3}

**As a** user **I want** the dashboard to show total listening duration **so that** I can see an overview of my library size in time.

**Acceptance criteria:**
- Dashboard displays total duration (formatted as `Xh Ym`) alongside other stats
- Only books with `duration_seconds` are counted

#### Task 6.3.1: Add total duration to dashboard backend {#task-6-3-1}

**Type:** Feature

**Description:** Add `total_duration_seconds: i64` to `Dashboard` struct. Compute via `SELECT SUM(duration_seconds) FROM books`.

**Files/components:**
- `src-tauri/src/models/dashboard.rs`
- `src-tauri/src/commands/dashboard_commands.rs`

**Dependencies:** None

**DoD:** Dashboard API response includes `total_duration_seconds`.

#### Task 6.3.2: Display total duration on dashboard UI {#task-6-3-2}

**Type:** Feature

**Description:** Add a "Total Duration" row to the dashboard stats table. Format using `formatDuration(seconds)`.

**Files/components:**
- `src/app/features/dashboard/dashboard.component.html`
- `src/app/features/dashboard/dashboard.component.ts`

**Dependencies:** [Task 6.3.1](#task-6-3-1)

**DoD:** Dashboard shows total duration in human-readable format.

---

## Milestone 7: Infrastructure & Quality

Fix broken migrations, improve database design, and prepare for multi-platform deployment.

> **Estimated time:** 2–3 weeks

---

### Story 7.1: Fix broken init migration down.sql — S {#story-7-1}

**As a** developer **I want** all migration rollbacks to work correctly **so that** the database can be safely reverted during development.

**Acceptance criteria:**
- `diesel migration revert` succeeds for all migrations
- `down.sql` for init migration uses correct SQLite syntax

#### Task 7.1.1: Rewrite init migration `down.sql` {#task-7-1-1}

**Type:** Bug

**Description:** The current init `down.sql` references non-existent tables (`genres`, `lectors`, `tags_authors`, `absolute_paths`), uses MySQL-incompatible `DROP FOREIGN KEY` syntax, and names constraints that were never defined. Rewrite to correctly drop only the tables created by the init `up.sql`.

**Files/components:**
- `src-tauri/migrations/2024-12-18-153058_init/down.sql`

**Dependencies:** None

**DoD:** `diesel migration revert` on init migration succeeds; all created tables are dropped.

---

### Story 7.2: Replace book title PK with composite key — L {#story-7-2}

**As a** system **I want** books to be uniquely identified by `(title, author_name)` **so that** two books with the same title from different authors don't collide.

**Acceptance criteria:**
- Books PK is a composite of `(title, author_name)` or a surrogate integer PK with a unique constraint on `(title, author_name)`
- Scanner correctly inserts books with duplicate titles from different authors
- All foreign key references are updated
- Existing data is migrated without loss

#### Task 7.2.1: Design and create the PK migration {#task-7-2-1}

**Type:** Refactor

**Description:** Create a migration that: (1) creates a new `books_new` table with integer PK + unique `(title, author_name)`, (2) copies data from `books`, (3) drops old `books` and renames. Update all FK references (`tags_books`). Write reversible `down.sql`.

**Files/components:**
- `src-tauri/migrations/{timestamp}_book_composite_pk/up.sql` (new)
- `src-tauri/migrations/{timestamp}_book_composite_pk/down.sql` (new)

**Dependencies:** [Task 7.1.1](#task-7-1-1)

**DoD:** Migration applies cleanly on existing databases; data is preserved; rollback works.

#### Task 7.2.2: Update all models and services for new PK {#task-7-2-2}

**Type:** Refactor

**Description:** Update `Book` struct to include integer `id` as PK. Update `schema.rs`. Update all services that query by title to use the new PK or the unique constraint. Update `tags_books` FK.

**Files/components:**
- `src-tauri/src/models/book.rs`
- `src-tauri/src/models/tag.rs`
- `src-tauri/src/schema.rs`
- `src-tauri/src/services/books_service.rs`
- `src-tauri/src/commands/books_commands.rs`
- `src-tauri/src/scanner.rs`

**Dependencies:** [Task 7.2.1](#task-7-2-1)

**DoD:** All services compile and work with the new PK; scanner handles title collisions correctly.

#### Task 7.2.3: Update frontend to use new book identifier {#task-7-2-3}

**Type:** Refactor

**Description:** Update frontend `Book` interface, routing (use `id` instead of `title`), and all `invoke()` calls that reference books by title.

**Files/components:**
- `src/app/models/books.ts`
- `src/app/app.routes.ts`
- `src/app/features/books/details/details.component.ts`
- `src/app/shared/components/generic-list/generic-list.component.html`

**Dependencies:** [Task 7.2.2](#task-7-2-2)

**DoD:** All book navigation uses the new identifier; no 404s for books with previously colliding titles.

---

### Story 7.3: Responsive window sizing — M {#story-7-3}

**As a** user **I want** the app to adapt to different window sizes **so that** I can resize the window and still use the app comfortably.

**Acceptance criteria:**
- Sidebar collapses to icon-only on narrow windows (< 900px)
- Gallery grid adjusts number of columns based on available width
- Book details page stacks vertically on narrow windows
- Minimum window size is enforced (e.g., 800×600)

#### Task 7.3.1: Add responsive breakpoints to global styles {#task-7-3-1}

**Type:** Feature

**Description:** Add `@media` queries to `_gallery.scss`, `_details.scss`, and sidebar/topbar component styles for narrow (< 900px) and wide (> 1600px) viewports.

**Files/components:**
- `src/styles/_gallery.scss`
- `src/styles/_details.scss`
- `src/app/shared/components/sidebar/sidebar.component.scss`
- `src/app/shared/components/topbar/topbar.component.scss`

**Dependencies:** None

**DoD:** App is usable at 800×600 and looks good at 1920×1080.

#### Task 7.3.2: Update Tauri window configuration {#task-7-3-2}

**Type:** Feature

**Description:** Update `tauri.conf.json` to allow window resizing, set `minWidth` / `minHeight`, and remove fixed dimensions. Allow fullscreen.

**Files/components:**
- `src-tauri/tauri.conf.json`

**Dependencies:** None

**DoD:** Window is resizable with enforced minimum dimensions.

#### Task 7.3.3: Add keyboard navigation shortcuts {#task-7-3-3}

**Type:** Feature

**Description:** Add global keyboard listener: `Ctrl+F` focuses the search input, `Escape` navigates back, arrow keys for gallery navigation.

**Files/components:**
- `src/app/app.component.ts`
- `src/app/shared/components/topbar/topbar.component.ts`

**Dependencies:** None

**DoD:** `Ctrl+F` focuses search bar; `Escape` navigates back from detail pages.

---

## Risks and External Dependencies

| # | Risk | Severity | Mitigation |
|---|------|----------|------------|
| R1 | **Book title as PK** — Two books with the same title from different authors collide silently. The second is skipped by the scanner. | High | [Story 7.2](#story-7-2) introduces a composite/surrogate PK. Schedule early if the user's library has collisions. |
| R2 | **No connection pooling** — Concurrent Tauri async commands each open a fresh `SqliteConnection`, risking "database is locked" errors and performance overhead. | High | [Story 1.3](#story-1-3) introduces `r2d2` connection pooling. |
| R3 | **Scanner blocks main thread** — `quick_scan_command` and `full_scan_command` are synchronous. Large libraries (1000+ books) will freeze the UI. | High | [Story 2.4](#story-2-4) moves scanning to a background thread with progress events. |
| R4 | **Broken init rollback** — The `down.sql` for the init migration uses MySQL syntax and references non-existent tables. `diesel migration revert` will fail. | Medium | [Task 7.1.1](#task-7-1-1) rewrites the init `down.sql`. |
| R5 | **`panic!()` as exit** — `kill_command` uses `panic!()` which may corrupt in-flight DB writes and produces error output. | Medium | [Task 1.1.1](#task-1-1-1) replaces with graceful exit. |
| R6 | **Theme not persisted** — Theme resets on every app restart, degrading UX. | Low | [Task 2.1.1](#task-2-1-1) saves to `localStorage`. |
| R7 | **`author_name` VARCHAR(36)** — Sized for UUIDs; PostgreSQL/MySQL strict mode would truncate long names. SQLite ignores length limits but this is a portability concern. | Low | Address during [Story 7.2](#story-7-2) PK migration by widening to `TEXT`. |
| R8 | **No test coverage** — Karma/Jasmine are configured but no meaningful tests exist for either frontend or backend. | Medium | Add testing tasks per story as the codebase grows; prioritize service-layer tests. |
| R9 | **Angular 17 + Node.js compatibility** — Build warnings about odd-numbered Node.js versions (v25.x). Production should target an LTS version (v22.x). | Low | Pin Node.js version in `.nvmrc` or `package.json` `engines`. |
| R10 | **Unused `uuid` crate** — `uuid` is declared as a dependency but never used. Minor bloat. | Low | Remove from `Cargo.toml` during any cleanup pass. |

---

*End of plan.*
