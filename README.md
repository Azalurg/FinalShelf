# FinalShelf

<img src="./img/image1.jpg" alt="FinalShelf UI" align="right" width="300px">

**FinalShelf** is a modern desktop audiobook manager for Linux. It's the next generation of [LibraAlchemy](https://github.com/Azalurg/LibraAlchemy), rebuilt from the ground up with Rust, SQLite, and Angular for a fast, responsive, single-application experience.

**Version:** 0.4.2

<div align="center">

⚠️ **Linux only** (for now) ⚠️

</div>

## Table of Contents

- [Features](#features)
- [Current Status](#current-status)
- [Installation](#installation)
- [Development](#development)
- [Roadmap](#roadmap)
- [Contributing](#contributing)

---

## Features

### ✅ Core Functionality

- **📚 Metadata Scanning** — Automatically extracts metadata (title, author, lector, genre) from audio files (MP3, M4B, M4A, OGG, FLAC, AAC) using embedded ID3 tags
- **🎨 Built-in UI** — Fully integrated desktop application (no browser needed); includes customizable themes
- **🔍 Advanced Search** — Multi-field search across title, author, genre, and lector with field toggles
- **📖 Book Management** — View, filter, sort, and toggle read status for your entire library
- **🏷️ Authors, Genres & Lectors** — Browse and explore your collection by creator/genre/narrator
- **📊 Dashboard** — See library statistics, newest books, and top-rated selections at a glance

### 🎯 Recently Added

- **Pagination on all list pages** — Authors, genres, lectors, search results all support full pagination + sorting
- **Search filtering** — Toggle which fields to search in (title, author, genre, lector)
- **Read Books page** — Dedicated view for books you've marked as read
- **Ranking page** — Browse books sorted by your personal score

---

## Current Status

| Feature          | Status      | Notes                                       |
| ---------------- | ----------- | ------------------------------------------- |
| Book browser     | ✅ Complete | Paginated gallery with sorting & filtering  |
| Authors view     | ✅ Complete | Gallery + pagination + sorting              |
| Genres view      | ✅ Complete | List with pagination + sorting              |
| Lectors view     | ✅ Complete | List with pagination + sorting              |
| Dashboard        | ✅ Complete | Stats, new books, top-rated                 |
| Search           | ✅ Complete | Multi-field with toggles & pagination       |
| Metadata scanner | ✅ Complete | Supports 6 audio formats; quick + full scan |
| Read tracking    | ✅ Complete | Mark/unmark books as read                   |
| Rating system    | ✅ Complete | Score books 0–10                            |
| Themes           | ✅ Complete | 5 themes with persistence                   |
| Book notes       | ⏳ Planned  | Story 3.2 in roadmap                        |
| Series/Cycles    | ⏳ Planned  | Story 4.1-4.3 in roadmap                    |
| Tags             | ⏳ Planned  | Story 5.1-5.2 in roadmap                    |
| Statistics page  | ⏳ Planned  | Story 6.1 in roadmap                        |
| Export/Import    | ⏳ Planned  | Story 6.2 in roadmap                        |

See [docs/plan.md](docs/plan.md) for the complete development roadmap with timeline and task breakdown.

---

## Installation

### Prerequisites

- Linux (Ubuntu/Fedora/Arch tested)
- Node.js 22 LTS or later
- Rust 1.70+
- SQLite3

### Build from Source

```bash
git clone https://github.com/Azalurg/FinalShelf.git
cd FinalShelf

# Install frontend dependencies
npm install

# Build the Tauri app
npm run tauri build
```

The compiled app will be in `src-tauri/target/release/`.

### Run in Development

```bash
# Terminal 1: Start Angular dev server
npm start

# Terminal 2: Start Tauri in dev mode
npm run tauri dev
```

---

## Development

### Project Structure

```
├── src/                    # Angular frontend (TypeScript)
│   ├── app/features/       # Page components (books, authors, etc.)
│   ├── shared/             # Reusable components & utilities
│   └── styles/             # Global SCSS with theme definitions
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── commands/       # Tauri IPC handlers
│   │   ├── services/       # Business logic (CRUD, queries)
│   │   ├── models/         # Diesel ORM models
│   │   ├── scanner.rs      # Metadata extraction
│   │   └── db.rs           # Database setup
│   └── migrations/         # Diesel SQL migrations
├── docs/
│   ├── DOCUMENTATION.md    # Technical deep-dive
│   └── plan.md             # Development roadmap (7 milestones)
```

### Technology Stack

- **Frontend:** Angular 17, TypeScript, SCSS
- **Backend:** Rust, Tauri 2.0, Diesel ORM
- **Database:** SQLite (embedded)
- **Audio Parsing:** lofty (metadata), walkdir (file traversal)

### Key Commands

| Command                     | Purpose                              |
| --------------------------- | ------------------------------------ |
| `npm start`                 | Start Angular dev server             |
| `npm run tauri dev`         | Run app in dev mode with live reload |
| `npm run build`             | Build frontend (production)          |
| `npm run tauri build`       | Build executable                     |
| `npm run lint`              | Run ESLint on frontend               |
| `cargo build -p finalshelf` | Build backend only                   |

### CI & Branch Protection

- Workflow: `CI` runs on pushes and pull requests targeting `main` and `develop`.
- Jobs: frontend lint (`npm run lint`), Rust `cargo check`, `cargo test`, `cargo clippy` with warnings as errors.
- Recommended branch protection: require the `CI / checks (ubuntu-latest)` job to pass before merging.

### Database

The app uses SQLite with Diesel ORM and embedded migrations. The database file is stored at `./finalshelf.sql` (or set via `DATABASE_URL` environment variable).

**Current schema:** 5 tables (books, authors, genres, lectors, tags, absolute_paths) with full relationships.

---

## Roadmap

FinalShelf follows a **7-milestone development plan** targeting version 1.0:

1. **[M1] Stability & Error Handling** (1–2 weeks)

   - Replace `panic!()`/`alert()` with graceful error handling
   - Introduce connection pooling for concurrent DB access

2. **[M2] UI/UX Polish** (2–3 weeks)

   - Scan progress feedback with real-time updates
   - Book score editing via star rating widget
   - Theme persistence across sessions

3. **[M3] Book Management** (1–2 weeks)

   - Delete books with orphaned author cleanup
   - Add personal notes/reviews to books
   - Manual cover image management

4. **[M4] Series & Cycles** (3–4 weeks)

   - Auto-detect series from title patterns & directory structure
   - UI for series browsing and manual assignment
   - Ordered book display within series

5. **[M5] Tags & Custom Labels** (1–2 weeks)

   - Activate existing tag system with CRUD operations
   - Tag assignment UI on book details
   - Filter books by tags

6. **[M6] Advanced Statistics & Export** (2–3 weeks)

   - Statistics dashboard with charts (genre/author distribution, timeline)
   - Database export/import (JSON format)
   - Total listening duration metrics

7. **[M7] Infrastructure & Quality** (2–3 weeks)
   - Fix broken database migrations
   - Replace title-based primary key with composite/surrogate key
   - Responsive window sizing and keyboard shortcuts

**Total estimate:** 12–19 weeks to v1.0

See [docs/plan.md](docs/plan.md) for detailed task breakdowns, dependencies, and acceptance criteria.

---

## Contributing

We welcome contributions! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Commit your changes (`git commit -am 'Add feature'`)
4. Push to the branch (`git push origin feature/your-feature`)
5. Open a Pull Request

### Code Standards

- **Frontend:** TypeScript, ESLint, Prettier-formatted
- **Backend:** Rust, `cargo fmt`, `clippy` linting
- Run `npm run lint` before committing

---

## License

This project is developed as a personal project. See LICENSE file for details.

---

## Links

- **Documentation:** [docs/DOCUMENTATION.md](docs/DOCUMENTATION.md) — Architecture, schema, and technical deep-dive
- **Development Plan:** [docs/plan.md](docs/plan.md) — Detailed roadmap with 68 tasks across 7 milestones
- **Original Project:** [LibraAlchemy](https://github.com/Azalurg/LibraAlchemy)
