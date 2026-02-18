<!--
SYNC IMPACT REPORT
==================
Version change:    (none / template placeholders) → 1.0.0
Bump rationale:    Initial fill — all placeholders replaced with project-specific
                   content; first ratified version of the document.

Modified principles:
  - [PRINCIPLE_1_NAME] → I. Local-First & Self-Contained
  - [PRINCIPLE_2_NAME] → II. Layered Architecture (Commands → Services → Models)
  - [PRINCIPLE_3_NAME] → III. Performance by Default
  - [PRINCIPLE_4_NAME] → IV. Type Safety & Shared Contracts
  - [PRINCIPLE_5_NAME] → V. Incremental, Testable Delivery
  - (template had 5; two additional principles added for this project)
  - NEW → VI. Portability via Relative Paths
  - NEW → VII. Simplicity & Sustainable Maintainability

Added sections:
  - Core Principles (I–VII)
  - Architecture & Technology Stack
  - Development Workflow & Standards
  - Data Models & Tauri Command API
  - Roadmap & Milestones
  - Risks & Constraints
  - Governance

Removed sections: N/A — initial constitution from template.

Templates requiring updates:
  ✅ .specify/templates/plan-template.md   — Constitution Check gates reference
                                             Principles I–VII by name.
  ✅ .specify/templates/spec-template.md   — No structural changes required;
                                             existing shape is compatible.
  ✅ .specify/templates/tasks-template.md  — Phase categories reflect
                                             principle-driven task types
                                             (performance, portability, type
                                             safety).
  ✅ .specify/templates/agent-file-template.md — No agent-specific (CLAUDE-only)
                                                  references found; no change
                                                  required.

Deferred TODOs: None — all fields resolved for initial version.
-->

# FinalShelf Constitution

## Introduction

**FinalShelf** is an open-source, cross-platform desktop application for managing
and navigating personal audiobook collections. It is the successor to
[LibraAlchemy](https://github.com/Azalurg/LibraAlchemy), graduating from a
browser-dependent web viewer to a fully self-contained native desktop experience
powered by Tauri (Rust) and Angular.

**Mission**: Provide audiobook enthusiasts with a fast, beautiful, and reliable
interface to scan, browse, and manage large local audiobook libraries — with zero
server dependencies and zero browser requirement.

**Target audience**: Individual users who maintain local audiobook libraries
(hundreds to tens of thousands of titles) and want metadata-rich, portable
collection management on Linux (and, eventually, other platforms).

**High-level goals**:
- Consolidate metadata scanning, storage, and UI into a single installable
  application.
- Achieve sub-second response times for browsing and searching collections of
  any practical size through server-side pagination and efficient queries.
- Keep all data fully local; no cloud, no telemetry, no external dependencies
  at runtime.
- Deliver a maintainable, well-structured codebase that welcomes open-source
  contributions.

---

## Core Principles

### I. Local-First & Self-Contained

FinalShelf MUST operate entirely without network access to external services.
All metadata, cover art, and user preferences MUST be stored locally (SQLite
database and filesystem). The application MUST bundle its own frontend assets
via Tauri's custom-protocol — no CDN links, no remote font loads, no external
API calls at runtime.

**Rationale**: The core value proposition is a standalone desktop app. Any
dependency on external services would break offline usage and compromise user
privacy.

**Non-negotiable rules**:
- No HTTP calls to external hosts in production builds.
- All Tauri plugin usage MUST be explicitly declared in `tauri.conf.json`
  capabilities; no undeclared permissions.
- CSP relaxation (currently `"csp": null`) is a known technical debt item and
  MUST be addressed before any v1.0 public release by defining an explicit CSP
  that allows only `asset:` and `https:` (localhost).

### II. Layered Architecture (Commands → Services → Models)

The backend MUST maintain a strict three-layer separation:

1. **Commands** (`src-tauri/src/commands/`): Thin Tauri IPC handlers. They
   MUST only deserialize input, delegate to a service, and serialize output.
   No business logic or database queries are permitted here.
2. **Services** (`src-tauri/src/services/`): All business logic and database
   interaction lives here. Services MUST be independently callable (i.e.,
   testable without a running Tauri runtime).
3. **Models** (`src-tauri/src/models/`): Pure Rust structs deriving Diesel
   traits, `Serialize`/`Deserialize`, and `Debug`. No business logic here.

The Angular frontend MUST mirror this discipline:

1. **Feature components** (`src/app/features/`): UI only — no direct Tauri
   `invoke` calls.
2. **Shared services** (future: `src/app/services/`): Wrap all `invoke` calls
   and expose typed Observables or Promises.
3. **Models** (`src/app/models/`): TypeScript interfaces matching the Rust
   structs serialized over IPC.

**Rationale**: Mixing concerns (e.g., SQL in commands, or `invoke` in components)
makes the codebase untestable and brittle. This layer contract is the single most
impactful structural rule.

**Non-negotiable rules**:
- Commands MUST return `Result<T, String>` — never `unwrap()` or `panic!()` in
  command handlers.
- Angular components MUST NOT call `invoke` directly; all Tauri IPC MUST go
  through a dedicated service layer (to be refactored as part of Roadmap
  Phase 1).

### III. Performance by Default

Performance requirements are first-class acceptance criteria, not afterthoughts.

**Mandatory performance gates before shipping any feature that reads data**:
- Pagination MUST be handled server-side (Rust/SQLite) — no full-table loads
  into Angular memory. Default page size: 20; max: 100.
- Cover images MUST be lazy-loaded in all list/gallery views. Viewport-based
  loading is preferred; at minimum, native `loading="lazy"` on `<img>` tags.
- Any Tauri command that queries the `books` table MUST accept `page`, `limit`,
  `sort_by`, and `sort_order` parameters.
- Database queries touching the `books` table MUST use indexed columns for
  filtering. `author_name`, `genre`, and `lector` MUST have database indices
  (add via migration if missing).

**Rationale**: The existing image-loading and full-collection-load patterns cause
visible UI lag on libraries exceeding a few hundred titles. Enforcing this now
prevents compounding technical debt.

### IV. Type Safety & Shared Contracts

TypeScript strict mode MUST be enabled (`"strict": true` in `tsconfig.json`).
`any` types are forbidden except in explicitly annotated escape hatches with a
comment explaining why.

On the Rust side, all public-facing functions MUST use explicit types; inference
is permitted for local variables only. `unsafe` blocks are forbidden unless a
detailed comment justifies the invariant being upheld.

The TypeScript interfaces in `src/app/models/` MUST be kept in sync with their
Rust counterparts in `src-tauri/src/models/`. When a Rust struct field is added,
renamed, or removed, the corresponding TypeScript interface MUST be updated in
the same commit/PR.

**Rationale**: The IPC boundary between Angular and Rust is stringly-typed at
runtime (JSON). Strong compile-time types on both sides are the primary safety
net against deserialization failures at runtime.

**Non-negotiable rules**:
- `tsc --noEmit` and `cargo check` MUST both pass with zero errors before any
  PR is merged.
- ESLint (`ng lint`) MUST produce zero errors; warnings are permitted only if
  accompanied by a tracked issue.

### V. Incremental, Testable Delivery

All new functionality MUST be delivered as independently testable increments.
Features MUST NOT be merged in a partially working state (e.g., a UI route that
points to an unimplemented command).

**Testing expectations**:
- Angular components with business logic MUST have Jasmine/Karma unit tests
  (`.spec.ts` files co-located with the component).
- Rust service functions MUST have `#[cfg(test)]` unit tests in the same file.
  Integration tests that require a real SQLite connection should use an
  in-memory database (`":memory:"`).
- E2E testing is not currently required but is planned for a future phase.

**Feature flags**: Incomplete features MUST be hidden behind a route guard or
a compile-time feature flag — they MUST NOT be reachable from the released UI
(example: the commented-out `read` route in `app.routes.ts` is acceptable
temporarily, but MUST ship uncommented or be deleted within two minor versions).

**Rationale**: Partially-shipped features increase cognitive load, cause
confusion in bug reports, and can expose broken command handlers.

### VI. Portability via Relative Paths

All file-system references persisted to the database MUST be stored as paths
relative to a registered absolute root from the `absolute_paths` table.
Absolute paths MUST NOT appear in `books.relative_file_path`,
`books.relative_cover_path`, or `authors.relative_img_path`.

When resolving a file for display, the application MUST prepend the current
active absolute path (from `absolute_paths_service::get_current_absolute_path`)
to the stored relative path.

**Rationale**: Absolute paths break portability when users move their audiobook
library to another disk or share the database between machines. The relative-path
+ registered-root model (already partially implemented) MUST be enforced
consistently.

**Non-negotiable rules**:
- The scanner (`src-tauri/src/scanner.rs`) MUST strip the absolute root before
  persisting any path.
- The frontend MUST resolve cover and file paths via the Tauri asset protocol
  using the active absolute root — never by constructing raw `file://` URLs.

### VII. Simplicity & Sustainable Maintainability

YAGNI (You Aren't Gonna Need It) applies. New abstractions, patterns, or
dependencies MUST be justified by a concrete, present need — not speculative
future requirements.

**Code hygiene rules**:
- Dead code (commented-out imports, unused components, unreachable routes) MUST
  be removed or tracked as a TODO issue within one release cycle.
- Component files MUST NOT exceed 300 lines; service files MUST NOT exceed 400
  lines. If they do, a refactor task MUST be created before new logic is added.
- Rust modules MUST follow `rustfmt` formatting (enforced via `rustfmt.toml`).
  Angular/TypeScript MUST follow Prettier (`prettier.json`) and ESLint
  (`eslint.config.js`) configurations.
- Dependencies MUST NOT be added without evaluating the maintenance burden.
  Prefer standard-library solutions when the external crate/package provides
  marginal benefit.

**Rationale**: The codebase is maintained by a small team (currently one author).
Complexity must be proportional to delivered value. Technical debt, once
acknowledged, must be scheduled — not indefinitely deferred.

---

## Architecture & Technology Stack

### Frontend (Angular)

| Concern          | Technology                        | Version    |
|------------------|-----------------------------------|------------|
| Framework        | Angular (standalone components)   | ^17.0.0    |
| Language         | TypeScript                        | ~5.2.2     |
| Reactive layer   | RxJS                              | ~7.8.0     |
| Styling          | SCSS (themes, mixins, reset)      | —          |
| IPC              | `@tauri-apps/api` `invoke()`      | ^2.1.1     |
| Native dialogs   | `@tauri-apps/plugin-dialog`       | ^2.2.0     |
| Linting          | ESLint + angular-eslint           | 19.x / 9.x |
| Formatting       | Prettier                          | 3.5.x      |
| Testing          | Jasmine + Karma                   | 5.1 / 6.4  |

**Directory conventions**:

```
src/app/
  features/<domain>/       # One folder per domain (books, authors, genres, …)
    list/                  # List page component
    details/               # Detail page component
  models/                  # TypeScript interfaces (mirror Rust models)
  shared/
    components/            # Reusable UI components (sidebar, topbar, …)
    utils/                 # Pure utility functions
  app.routes.ts            # Centralised route registry
  app.config.ts            # Application-level providers
```

### Backend (Tauri / Rust)

| Concern          | Technology           | Version  |
|------------------|----------------------|----------|
| Desktop runtime  | Tauri                | 2.0      |
| Language         | Rust                 | 2021 ed. |
| ORM              | Diesel               | 2.2.x    |
| Database         | SQLite (bundled)     | via rusqlite 0.32 |
| Metadata parsing | id3 crate            | 1.16.x   |
| File traversal   | walkdir              | 2.5.x    |
| Serialization    | serde + serde_json   | 1.0      |
| Date/time        | chrono               | 0.4.x    |
| Shell plugin     | tauri-plugin-shell   | 2        |
| Dialog plugin    | tauri-plugin-dialog  | 2        |

**Directory conventions**:

```
src-tauri/src/
  commands/      # Thin Tauri IPC handlers (one file per domain)
  services/      # Business logic + database queries
  models/        # Diesel-mapped structs + response types
  schema.rs      # Auto-generated by Diesel CLI — do not edit manually
  scanner.rs     # File-system scanner (metadata extraction)
  db.rs          # Connection pool initialisation & migrations
  main.rs        # Tauri builder + command registration
migrations/      # Diesel migration files (timestamped)
```

### Database (SQLite)

Core tables (see `src-tauri/src/schema.rs` for authoritative definition):

| Table            | Primary Key            | Key Columns                                                              |
|------------------|------------------------|--------------------------------------------------------------------------|
| `books`          | `title`                | `relative_file_path`, `author_name`, `genre`, `lector`, `read`, `score` |
| `authors`        | `name`                 | `relative_img_path`                                                      |
| `absolute_paths` | `id`                   | `absolute_path`, `add_date`, `last_use_date`                             |
| `tags`           | `id`                   | `name`                                                                   |
| `tags_books`     | `(tag_id, book_title)` | join table for many-to-many tags ↔ books                                 |

Schema changes MUST be made exclusively via Diesel migrations
(`diesel migration generate <name>`). Direct schema edits are forbidden.

### Data Flow

```
User Action (Angular component)
        │
        ▼
  Angular Service  ──invoke()──▶  Tauri Command Handler
        │                                │
        │                         delegates to
        │                                ▼
        │                       Rust Service Function
        │                                │
        │                      Diesel ORM query
        │                                │
        │                           SQLite DB
        │                                │
        ◀──── JSON (serde) ─────────────◀
        │
   Component renders
```

### Build & Dev Environment

| Task                    | Command                          |
|-------------------------|----------------------------------|
| Frontend dev server     | `npm run start` (port 1420)      |
| Full Tauri dev mode     | `npm run tauri dev`              |
| Production bundle       | `npm run tauri build`            |
| Frontend lint           | `npm run lint`                   |
| Rust format check       | `cargo fmt --check`              |
| Rust lint               | `cargo clippy -- -D warnings`    |
| Diesel migration run    | `diesel migration run`           |
| Diesel migration revert | `diesel migration revert`        |

---

## Development Workflow & Standards

### Branching & Pull Requests

- `main` is the protected production branch; direct pushes are forbidden.
- Feature branches MUST follow the naming convention `###-short-description`
  (e.g., `024-server-side-pagination`).
- PRs MUST reference a tracked issue and include a summary of changes and
  testing steps.
- At least one self-review pass (for solo projects) or one peer review (when
  contributors join) MUST occur before merge.

### Coding Standards

**TypeScript / Angular**:
- Prefer `readonly` properties and immutable data patterns.
- Observables exposed from services MUST use the `$` suffix convention
  (e.g., `books$`).
- Use `OnPush` change detection strategy for all list components to reduce
  unnecessary render cycles.
- Avoid `ngModel` two-way binding in favour of reactive forms for complex inputs.
- All async operations MUST handle errors explicitly — never swallow errors
  silently.

**Rust**:
- All `unwrap()` calls outside tests MUST be replaced with `?` propagation or
  explicit `match`/`if let` with a meaningful error path.
- Use `thiserror` or custom error types in services for structured error
  reporting (to be introduced as a Phase 1 task).
- Prefer `&str` over `String` in function parameters where ownership is not
  required.
- Clippy MUST pass at `warn` level; new `#[allow(...)]` suppressions require
  a comment justifying the exception.

### Commit Convention

Commits MUST follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <short description>

Types: feat | fix | refactor | perf | test | docs | chore | build
Scope: frontend | backend | db | scanner | config | deps | ci
```

Examples:
- `feat(backend): add server-side pagination to books command`
- `fix(frontend): resolve image path resolution for cover art`
- `perf(db): add indices on books.author_name and books.genre`
- `docs: amend constitution to v1.1.0`

### Dependency Updates

- Dependencies MUST be reviewed before upgrade; breaking changes require a
  dedicated migration task.
- Rust crates are managed via `Cargo.toml`; Node packages via `package.json`.
  Lock files (`Cargo.lock`, `package-lock.json`) MUST be committed.

---

## Data Models & Tauri Command API

### Key Models

**`Book`** (Rust: `src-tauri/src/models/book.rs`,
TypeScript: `src/app/models/books.ts`):

| Field                 | Type                  | Notes                             |
|-----------------------|-----------------------|-----------------------------------|
| `title`               | `String`              | Primary key; unique               |
| `relative_cover_path` | `Option<String>`      | Relative to active absolute root  |
| `author_name`         | `String`              | FK → `authors.name`               |
| `genre`               | `Option<String>`      | Free-text, nullable               |
| `lector`              | `Option<String>`      | Free-text, nullable               |
| `create_date`         | `Option<NaiveDateTime>` | From file metadata              |
| `read`                | `Option<bool>`        | User-set read flag                |
| `score`               | `Option<i32>`         | 1–10 user rating; nullable        |
| `relative_file_path`  | `String`              | Relative to active absolute root  |

**`Author`** (Rust: `src-tauri/src/models/author.rs`):

| Field               | Type             | Notes                             |
|---------------------|------------------|-----------------------------------|
| `name`              | `String`         | Primary key                       |
| `relative_img_path` | `Option<String>` | Relative to active absolute root  |

**`BookListResponse`**: Pagination envelope returned by `get_books_list_command`.
Contains `books: Vec<Book>`, `total_count`, `page`, `limit`, `total_pages`.

**`QueryParams`** (`src-tauri/src/models/query.rs`): Common filter/sort/pagination
parameters accepted by list commands.

### Tauri Commands (Public API)

| Command                                    | Description                             |
|--------------------------------------------|-----------------------------------------|
| `get_books_list_command`                   | Paginated, filtered, sorted book list   |
| `get_book_command`                         | Single book by title                    |
| `get_all_read_books_command`               | All books where `read = true`           |
| `update_book_command`                      | Update a book record (score, read, …)   |
| `search_command`                           | Full-text search across title/author    |
| `get_all_authors_command`                  | Paginated author list                   |
| `get_author_command`                       | Single author by name                   |
| `get_all_genres_command`                   | All genres                              |
| `get_genre_command`                        | Single genre                            |
| `get_all_lectors_command`                  | All lectors                             |
| `get_lector_command`                       | Single lector                           |
| `get_dashboard_data_command`               | Aggregate stats for dashboard           |
| `quick_scan_command`                       | Scan audiobook directory & populate DB  |
| `add_absolute_path_command`                | Register a new library root path        |
| `get_all_absolute_path_command`            | List all registered root paths          |
| `set_current_absolute_path_by_id_command`  | Set the active root path                |
| `get_current_absolute_path_command`        | Get the currently active root path      |
| `ping_command`                             | Health check                            |
| `kill_command`                             | Quit the application                    |

Command contracts MUST NOT change without a corresponding version bump in
`tauri.conf.json` and `package.json`.

---

## Roadmap & Milestones

### Phase 1 — Core Stability (current priority)

Goal: Eliminate known performance issues and structural debt before new features.

| #   | Task                                                           | Principle |
|-----|----------------------------------------------------------------|-----------|
| 1.1 | Implement server-side pagination for all list views            | III       |
| 1.2 | Add lazy loading / viewport-aware image loading                | III       |
| 1.3 | Fix and improve search (accuracy, ranking, new design)         | III       |
| 1.4 | Introduce Angular service layer to wrap all `invoke()` calls   | II        |
| 1.5 | Add Rust unit tests to all service functions                   | V         |
| 1.6 | Enforce TypeScript strict mode; eliminate `any`                | IV        |
| 1.7 | Add DB indices on `author_name`, `genre`, `lector`             | III       |

### Phase 2 — Feature Completeness

Goal: Ship the features outlined in the README TODO list.

| #   | Task                                                                | Principle |
|-----|---------------------------------------------------------------------|-----------|
| 2.1 | Read-books list view (uncomment route, complete component)          | V         |
| 2.2 | Sorting and filtering UI for books/authors lists                    | III       |
| 2.3 | User scoring/ranking UI (1–10 score input on book detail)           | V         |
| 2.4 | Settings persistence (store user preferences in DB or config file)  | I         |
| 2.5 | Tag management UI and tag-based filtering                           | II        |

### Phase 3 — Polish & Cross-Platform

Goal: Broaden platform support and improve UX quality.

| #   | Task                                                           | Principle |
|-----|----------------------------------------------------------------|-----------|
| 3.1 | Windows and macOS build validation and CI pipeline             | V         |
| 3.2 | E2E testing setup (WebDriver / Playwright with Tauri)          | V         |
| 3.3 | Explicit Content Security Policy replacing `"csp": null`       | I         |
| 3.4 | Series field: add to DB schema, scanner extraction, and UI     | II        |
| 3.5 | Accessibility audit (ARIA labels, keyboard navigation)         | VII       |

---

## Risks & Constraints

| Risk / Constraint                         | Impact   | Mitigation                                              |
|-------------------------------------------|----------|---------------------------------------------------------|
| Linux-only builds currently               | Medium   | Phase 3.1 cross-platform CI                             |
| `books.title` as primary key              | High     | Future migration to UUID PK; tracked as long-term debt  |
| CSP disabled (`"csp": null`)              | Medium   | Phase 3.3; documented as known risk in release notes    |
| Broad asset protocol scope (`"/**/*"`)    | Medium   | Restrict to library root(s) in a near-term patch        |
| Large libraries (10k+ titles)             | High     | Addressed in Phase 1 (pagination + indices)             |
| No series field in DB                     | Low      | Tracked as Phase 3.4                                    |
| Single-author project (bus-factor risk)   | Medium   | Constitution + thorough docs as onboarding artefacts    |
| `id3` parses MP3 only; M4B/FLAC excluded  | Medium   | Investigate `lofty` crate; tracked as future task       |

---

## Governance

This constitution is the authoritative source of architectural truth for
FinalShelf. All development decisions — feature additions, refactors, dependency
changes, and breaking API changes — MUST be evaluated against the principles
above.

**Amendment procedure**:
1. Open a GitHub issue labelled `constitution` describing the proposed change
   and which principle(s) are affected.
2. Draft the amendment in a PR that modifies `.specify/memory/constitution.md`
   and updates `CONSTITUTION_VERSION` according to the semantic versioning rules
   below.
3. For solo projects: self-review and a 24-hour reflection period before merge.
   For multi-contributor projects: at least one co-author review is required.
4. Update `LAST_AMENDED_DATE` to the merge date.
5. Propagate changes to dependent templates as described in the Sync Impact
   Report embedded in the constitution header.

**Versioning policy** (semantic versioning applied to governance):
- **MAJOR**: Backward-incompatible change — removal or fundamental redefinition
  of a principle, or removal of a mandatory section.
- **MINOR**: New principle added, new mandatory section added, or material
  expansion of guidance that changes expected developer behaviour.
- **PATCH**: Clarifications, wording improvements, typo fixes, roadmap
  re-prioritisation, or additions to the risk table with no principle changes.

**Compliance review**:
- Constitution Check MUST be performed at the start of every implementation
  plan (see `.specify/templates/plan-template.md`).
- At each minor application release, the Roadmap section MUST be reviewed and
  completed milestones marked as done.
- The `agent-file-template.md` in `.specify/templates/` MUST reference the
  current constitution version in its generated output.

**Version**: 1.0.0 | **Ratified**: 2024-12-18 | **Last Amended**: 2026-02-18
