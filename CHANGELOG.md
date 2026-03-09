# Changelog

All notable changes to FinalShelf are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.5.2] - 2026-03-09

### Added
- Comprehensive AI agent instructions with 11 sections covering all development phases
- Progress checkboxes (`- [ ]` / `- [x]`) to all stories and tasks in development plan
- `.nvmrc` file pinning Node.js to v22 LTS

### Changed
- Relocated agent log from `.agent-log.md` to `docs/AGENT-LOG.md`
- Updated `DOCUMENTATION.md` implementation status to reflect current state
- Improved milestone status tracking in development plan

### Removed
- Unused `uuid` crate from Cargo.toml dependencies

---

## [0.5.1] - 2026-03-07

### Added
- Reusable `ScoreInputComponent` for book rating (0–10 scale)
- Integrated score editing into book details page

### Fixed
- Image path fallback handling in generic list component

---

## [0.5.0] - 2026-03-06

### Added
- **Series/Cycles data model** (M4-2.1)
  - `series` table with `id`, `name`, `author_name`, `description`
  - `series_id` and `series_order` columns on `books` table
  - Full CRUD service layer and 7 Tauri commands
- **Series auto-detection in scanner** (M4-2.2)
  - Title pattern detection (`Series - 01 - Title`, `Series 01 - Title`)
  - Directory structure detection (`Author/Series/Book`)
  - Automatic series creation and book assignment during import
- **Series UI** (M4-2.3)
  - Series list page at `/series` with pagination and sorting
  - Series details page showing ordered book list
  - Sidebar navigation entry for series
  - Series info and assignment UI on book details page

### Fixed
- PR #12 review comments: series_order clearing, strict equality, redundant calls

---

## [0.4.8] - 2026-03-06

### Changed
- Renamed `Notification` → `AppNotification` to avoid DOM collision
- Use `crypto.randomUUID()` instead of `Math.random()` for ID generation
- Simplified star rating `[src]` binding

### Fixed
- Added `type="button"` to close/star buttons
- Added `aria-label` to star rating for accessibility
- Fixed `fetchFilterOptions` to pass params and read `.items`
- Normalized error objects in notification messages
- Fixed `setupScanProgressListener` void+catch handling
- `add_author` and `get_all_absolute_path` now return `Result`

---

## [0.4.7] - 2026-03-05

### Added
- **Error handling improvements** (M1-1.6)
  - Created `NotificationService` replacing all `alert()` calls
  - Toast notification component with auto-dismiss
  - Improved Rust error handling with `Result` returns

---

## [0.4.6] - 2026-03-05

### Added
- **Book score editing** (M2-2.2)
  - Interactive star rating widget on book details (0–10)
  - Score persisted to database immediately

---

## [0.4.5] - 2026-03-05

### Added
- **Scan progress feedback** (M2-2.4)
  - Real-time progress bar during library scans
  - Tauri event emission with phase/count data
  - Scan buttons disabled during operation

---

## [0.4.4] - 2026-03-05

### Added
- **Theme persistence** (M2-2.1)
  - Selected theme saved to `localStorage`
  - Theme restored on app startup before first render

---

## [0.4.3] - 2026-03-05

### Added
- **Frontend filters wired to book list** (M2-2.3)
  - Filter controls for author, genre, lector, read status
  - Toggle panel UI on books list page
  - Filters passed to backend `get_books_list_command`

---

## [0.4.2] - 2026-03-05

### Added
- Technical documentation and development roadmap
- AI Code Agent action logging system
- Update functionality on frontend

### Fixed
- Frontend lint errors across 5 components
- Rust Clippy warnings in query and paths services

---

## [0.3.3] - 2026-02-28

### Changed
- Unified list endpoints with pagination, sorting, and search
- Replaced `QueryParams` with generic `ListParams` and `ListResponse`

---

## [0.3.0] - 2026-02-15

### Added
- Read Books and Ranking pages
- Proper typing and `OnInit` implementation across components
- Adapted components to unified paginated API

### Fixed
- Frontend type alignment with backend response shapes
- `generic-list` Input-dependent logic moved from constructor to `ngOnInit`

---

## [0.2.0] - 2026-01-15

### Added
- Absolute path management for external drives
- Save read status functionality

---

## [0.1.0] - 2025-12-20

### Added
- Initial release
- Metadata scanner for `.mp3` audiobooks
- Book browser with pagination and sorting
- Authors, Genres, Lectors views
- Dashboard with statistics
- Book details page
- 5 color themes
- SQLite database with Diesel ORM

---

[0.5.2]: https://github.com/azalurg/FinalShelf/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/azalurg/FinalShelf/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/azalurg/FinalShelf/compare/v0.4.8...v0.5.0
[0.4.8]: https://github.com/azalurg/FinalShelf/compare/v0.4.7...v0.4.8
[0.4.7]: https://github.com/azalurg/FinalShelf/compare/v0.4.6...v0.4.7
[0.4.6]: https://github.com/azalurg/FinalShelf/compare/v0.4.5...v0.4.6
[0.4.5]: https://github.com/azalurg/FinalShelf/compare/v0.4.4...v0.4.5
[0.4.4]: https://github.com/azalurg/FinalShelf/compare/v0.4.3...v0.4.4
[0.4.3]: https://github.com/azalurg/FinalShelf/compare/v0.4.2...v0.4.3
[0.4.2]: https://github.com/azalurg/FinalShelf/compare/v0.3.3...v0.4.2
[0.3.3]: https://github.com/azalurg/FinalShelf/compare/v0.3.0...v0.3.3
[0.3.0]: https://github.com/azalurg/FinalShelf/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/azalurg/FinalShelf/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/azalurg/FinalShelf/releases/tag/v0.1.0
