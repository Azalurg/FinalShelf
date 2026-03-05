# FinalShelf Copilot Instructions

## Project scope

- Desktop app built with Angular (frontend) + Rust/Tauri (backend) + Diesel/SQLite.
- Prefer targeted, minimal edits in existing modules.
- Do not redesign UI or routing unless explicitly requested.

## Architecture rules

- Frontend changes belong in `src/app/features/*` and shared UI in `src/app/shared/*`.
- Tauri command handlers stay thin in `src-tauri/src/commands/*`.
- Business logic and DB operations belong in `src-tauri/src/services/*`.
- Diesel models and DTOs belong in `src-tauri/src/models/*`.
- Keep command/service/model separation intact.

## Coding standards

- TypeScript: follow existing Angular standalone component style.
- Rust: keep functions explicit and strongly typed, avoid panics in normal paths.
- Do not introduce one-letter variable names.
- Keep changes consistent with existing naming and folder conventions.

## Database and migrations

- Database is SQLite with Diesel migrations in `src-tauri/migrations`.
- For schema changes: add forward + rollback SQL and update Diesel schema/model usage.
- Avoid destructive migration changes unless explicitly requested.

## Validation before handoff

Run the smallest relevant checks for touched areas first, then broader checks:

1. `npm run lint` for frontend edits.
2. `cargo check --manifest-path src-tauri/Cargo.toml` for Rust edits.
3. `cargo clippy --manifest-path src-tauri/Cargo.toml` when Rust logic changes.

If a command fails due to pre-existing issues, report clearly what is unrelated.

## Common development commands

- Frontend dev server: `npm start`
- Tauri dev mode: `npm run tauri dev`
- Frontend lint: `npm run lint`
- Rust check: `cargo check --manifest-path src-tauri/Cargo.toml`

## Agent Action Logging

**IMPORTANT:** After completing any development task (fix, feature, refactor), log your actions in `.agent-log.md`:

1. Add a new entry with format: `### [YYYY-MM-DD HH:MM] | [TYPE] | [SUMMARY]`
2. List affected files and describe changes (1-3 lines).
3. Record validation commands and their outcomes (✅ or ❌).
4. Commit the log update with your changes.

See `.agent-log.md` for examples and full logging format. Keep logs concise and searchable for future reference.
