# FinalShelf — AI Agent Instructions

> **For:** GitHub Copilot, Claude, and other AI coding assistants  
> **Version:** 0.5.2  
> **Last updated:** 2026-03-09

---

## 1. Project Overview

| Aspect | Details |
|--------|---------|
| **Type** | Desktop audiobook library manager |
| **Stack** | Angular 17 (frontend) · Rust/Tauri 2.0 (backend) · Diesel ORM · SQLite |
| **Key docs** | `docs/DEVELOPMENT PLAN.md` (roadmap), `docs/DOCUMENTATION.md` (architecture), `docs/AGENT-LOG.md` (action history) |

### 1.1 Design Principles

- Prefer **targeted, minimal edits** over large refactors.
- Do **not** redesign UI, routing, or architecture unless explicitly requested.
- Maintain existing naming conventions and folder structure.
- Keep changes **testable** and **reversible**.

---

## 2. Architecture Rules

### 2.1 Frontend (`src/app/`)

| Layer | Location | Responsibility |
|-------|----------|----------------|
| Features | `src/app/features/*` | Page components, feature-specific logic |
| Shared | `src/app/shared/components/*` | Reusable UI components |
| Services | `src/app/shared/services/*` | Cross-cutting concerns (notifications, etc.) |
| Models | `src/app/models/*` | TypeScript interfaces matching backend DTOs |
| Constants | `src/app/shared/constants/*` | App-wide constants (themes, config) |
| Utils | `src/app/shared/utils/*` | Pure helper functions |

### 2.2 Backend (`src-tauri/src/`)

| Layer | Location | Responsibility |
|-------|----------|----------------|
| Commands | `commands/*.rs` | Thin Tauri IPC handlers — delegate to services |
| Services | `services/*.rs` | Business logic, DB operations, orchestration |
| Models | `models/*.rs` | Diesel ORM models + response DTOs |
| Schema | `schema.rs` | Auto-generated Diesel schema (do not edit manually) |
| Scanner | `scanner.rs` | File system walking, metadata extraction |

### 2.3 Separation of Concerns

- **Commands** must stay thin — no business logic, just input parsing → service call → response mapping.
- **Services** own all DB access and domain logic.
- **Models** are data structures only — no methods beyond derive macros.

---

## 3. Coding Standards

### 3.1 TypeScript / Angular

- Use **standalone components** (no NgModules).
- Implement `OnInit` / `OnDestroy` lifecycle interfaces explicitly.
- Use `strictNullChecks`-safe patterns (`?.`, `??`, explicit null handling).
- Prefer `const` over `let`; avoid `any` — use typed interfaces.
- Use `crypto.randomUUID()` for ID generation, not Math.random().
- Add `type="button"` to non-submit buttons; add `aria-label` for accessibility.

### 3.2 Rust

- Return `Result<T, String>` from all service functions — no `unwrap()` or `expect()` in production paths.
- Use `?` operator for error propagation.
- Avoid `panic!()` for control flow.
- Use `LazyLock` for static regex compilation.
- Keep functions explicit with named parameters and clear return types.
- Use `clippy` recommendations.

### 3.3 Naming Conventions

| Type | Convention | Example |
|------|------------|---------|
| Files | kebab-case | `score-input.component.ts` |
| TypeScript types | PascalCase | `BookListResponse` |
| TypeScript variables | camelCase | `bookDetails` |
| Rust structs | PascalCase | `BookWithDetails` |
| Rust functions | snake_case | `get_books_list` |
| CSS classes | kebab-case | `.book-card-title` |
| Database tables | snake_case | `tags_books` |

### 3.4 Code Quality

- No single-letter variable names except loop indices (`i`, `j`).
- Maximum function length: ~50 lines — split if longer.
- Add JSDoc / Rustdoc for public APIs.
- Remove unused imports and dead code.

---

## 4. Database & Migrations

- Database: **SQLite** via Diesel ORM.
- Migrations location: `src-tauri/migrations/`.
- Naming: `{YYYY-MM-DD-HHMMSS}_{description}/`.

### 4.1 Creating Migrations

```bash
cd src-tauri
diesel migration generate <migration_name>
```

### 4.2 Migration Requirements

- Always provide **both** `up.sql` and `down.sql`.
- `down.sql` must be a complete rollback — test with `diesel migration revert`.
- After migration: run `diesel migration run` to regenerate `schema.rs`.
- Update corresponding Rust models in `models/*.rs`.
- Update frontend TypeScript models in `src/app/models/*.ts`.

### 4.3 Migration Safety

- Avoid destructive changes unless explicitly requested.
- For column renames: use copy-migrate-drop pattern.
- Test both `run` and `revert` before commit.

---

## 5. Development Workflow

### 5.1 Common Commands

| Task | Command |
|------|---------|
| Frontend dev server | `npm start` |
| Tauri dev mode | `npm run tauri dev` |
| Frontend lint | `npm run lint` |
| Rust check | `cargo check --manifest-path src-tauri/Cargo.toml` |
| Rust lint | `cargo clippy --manifest-path src-tauri/Cargo.toml` |
| Rust format | `cargo fmt --manifest-path src-tauri/Cargo.toml` |
| Run migrations | `cd src-tauri && diesel migration run` |

### 5.2 Validation Before Handoff

Run checks in order of scope — fail fast:

1. **Frontend edits:** `npm run lint`
2. **Rust edits:** `cargo check --manifest-path src-tauri/Cargo.toml`
3. **Rust logic changes:** `cargo clippy --manifest-path src-tauri/Cargo.toml`
4. **Format check:** `cargo fmt --check --manifest-path src-tauri/Cargo.toml` (optional)

If a command fails due to **pre-existing issues**, clearly report what is unrelated to your changes.

---

## 6. Version Management

### 6.1 Bump Script

Every PR **must** include a version bump unless explicitly exempted.

```bash
./bump.sh <small|mid|big|custom>
```

| Type | When to use |
|------|-------------|
| `small` | Bug fixes, minor improvements, refactors |
| `mid` | New features, significant enhancements |
| `big` | Breaking changes, major releases |
| `custom` | Specify exact version number |

### 6.2 Version Files (must stay synchronized)

- `package.json` → `"version"`
- `src-tauri/Cargo.toml` → `version`
- `src-tauri/tauri.conf.json` → `"version"`

### 6.3 Exemptions

If a PR intentionally skips a bump:
- Document the reason in PR description.
- Common exemptions: docs-only changes, CI/tooling changes, WIP commits.

---

## 7. Pull Request Guidelines

### 7.1 PR Checklist

Before submitting or merging:

- [ ] Code compiles without errors (`cargo check`, `npm run lint`)
- [ ] No new warnings introduced
- [ ] Version bumped appropriately
- [ ] CHANGELOG.md updated (for releases)
- [ ] Agent log updated (see §8)
- [ ] Related documentation updated if behavior changed
- [ ] Commit messages follow conventional format

### 7.2 Commit Message Format

```
<type>(<scope>): <description>

[optional body]
```

**Types:** `feat`, `fix`, `refactor`, `docs`, `test`, `chore`, `perf`, `style`  
**Scope:** module or feature area (e.g., `scanner`, `books`, `M2-2.1`)

**Examples:**
```
feat(series): add auto-detection in scanner
fix(books): guard against null cover paths
docs: update development plan with M2 progress
```

### 7.3 Code Review Focus Areas

When reviewing or being reviewed:

- Error handling completeness
- Type safety (no `any`, proper null handling)
- Accessibility (aria labels, keyboard navigation)
- Performance (unnecessary re-renders, N+1 queries)
- Security (input validation, path traversal)

---

## 8. Agent Action Logging

**Location:** `docs/AGENT-LOG.md`

### 8.1 When to Log

After completing any meaningful development task:
- Bug fixes
- New features
- Refactors
- Documentation updates
- Configuration changes

### 8.2 Log Entry Format

```markdown
### [YYYY-MM-DD HH:MM] | [TYPE] | [SUMMARY]

- **Files affected**: `file1.ts`, `file2.rs`
- **Changes**: Brief description (1-3 lines)
- **Validation**: `npm run lint` ✅, `cargo check` ✅
```

**Types:** `setup`, `fix`, `feature`, `refactor`, `docs`, `perf`, `test`

### 8.3 Best Practices

- Log immediately after completing work.
- Keep descriptions concise but searchable.
- Include validation command outcomes (✅ or ❌).
- Commit the log update with your code changes.

---

## 9. Project Documentation

### 9.1 Documentation Files

| File | Purpose |
|------|---------|
| `README.md` | Project overview, quick start, features |
| `CHANGELOG.md` | Release history in Keep a Changelog format |
| `docs/DOCUMENTATION.md` | Technical architecture, module details |
| `docs/DEVELOPMENT PLAN.md` | Roadmap with milestones, stories, tasks |
| `docs/AGENT-LOG.md` | Chronological AI agent action history |

### 9.2 Keeping Docs Updated

- Update `CHANGELOG.md` when releasing new versions (Added/Changed/Fixed/Removed).
- Update `DEVELOPMENT PLAN.md` progress markers when completing tasks.
- Update `DOCUMENTATION.md` when architecture changes.
- Update `README.md` when user-facing features change.
- Reference documentation in commit messages when relevant.

---

## 10. Environment & Tooling

### 10.1 Required Tools

- Node.js LTS (v22.x recommended)
- Rust (stable channel)
- Diesel CLI: `cargo install diesel_cli --no-default-features --features sqlite`
- VS Code with recommended extensions (see `.vscode/extensions.json`)

### 10.2 VS Code Tasks

Pre-configured tasks in `.vscode/tasks.json`:

- `Dev: Angular` — Start frontend dev server
- `Dev: Tauri` — Start full Tauri dev mode
- `Lint: Frontend` — Run ESLint
- `Check: Rust` — Run cargo check
- `Clippy: Rust` — Run clippy lints
- `Format: Rust` — Run rustfmt

### 10.3 Debugging

- Use `npm run tauri dev` for hot-reload development.
- Tauri DevTools: `Ctrl+Shift+I` in the app window.
- Rust logs: set `RUST_LOG=debug` environment variable.

---

## 11. Quick Reference

### 11.1 File Locations

| What | Where |
|------|-------|
| Add a new page | `src/app/features/<feature>/` |
| Add a shared component | `src/app/shared/components/<name>/` |
| Add a Tauri command | `src-tauri/src/commands/<domain>_commands.rs` |
| Add business logic | `src-tauri/src/services/<domain>_service.rs` |
| Add a data model | `src-tauri/src/models/<entity>.rs` + `src/app/models/<entity>.ts` |
| Add a migration | `src-tauri/migrations/<timestamp>_<name>/` |

### 11.2 Common Patterns

**Calling backend from frontend:**
```typescript
const result = await invoke<BookListResponse>('get_books_list_command', { params });
```

**Returning errors from Rust:**
```rust
pub fn do_something() -> Result<Data, String> {
    some_operation().map_err(|e| e.to_string())?;
    Ok(data)
}
```

**Adding a new route:**
```typescript
// app.routes.ts
{ path: 'new-feature', component: NewFeatureComponent }
```

---

*End of instructions.*
