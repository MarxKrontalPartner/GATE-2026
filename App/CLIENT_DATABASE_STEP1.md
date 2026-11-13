# GATE-MKP — Step 1: Client SQLite + SeaORM

**This package is based on the original uploaded `GATE-2026-main(4).zip`.**
Only `App/src-tauri/` is changed, plus this README. **No files inside `Server/` or Vue `App/src/` were changed.**

## What has been added

- `src-tauri/migrations/0001_server_domain_schema.sql`: 35 server-domain tables mirrored as empty local SQLite tables, including their existing foreign keys, indexes, and constraints.
- `src-tauri/migrations/0002_sync_and_conflicts.sql`: 9 *local metadata tables only* for future synchronization, conflicts, media transfers, and offline context. No sync logic is included.
- `src-tauri/src/entities/`: 44 SeaORM 2 compact-format model modules, one for each SQLite table, with foreign-key relation declarations.
- `src-tauri/src/db.rs`: SQLite creation on startup, SQLx migrations, SeaORM pool, and a small ORM count query.
- `src-tauri/src/lib.rs`: registers the shared ORM connection in Tauri managed state.
- `src-tauri/Cargo.toml`: SeaORM 2.0.4, SQLx 0.9.0, serde dependencies.

## Run on Windows

1. Extract the ZIP to a clean directory. Keep a backup of your old project.
2. Open PowerShell in `GATE-2026-main/App`.
3. Install JavaScript dependencies if needed: `npm install`.
4. Start the **Tauri desktop app**: `npx tauri dev` (not `npm run dev`, which runs Vite only in this uploaded package).
5. Rust will download and compile the new crates on first run, which uses additional disk space. The checked-in `Cargo.lock` is from the original project and may be updated by Cargo on first build.
6. Startup should log `[GATE] Client SQLite: ...` and `[GATE] SeaORM ready. Local projects: 0`.
7. SQLite lives in your Tauri application-data directory, not in source control. On Windows, it is typically `%APPDATA%\com.mkp.gatestartpunkt\gate-offline.sqlite3`. Tauri determines the exact directory from the application identifier.
8. Optional schema check without installing Rust: from `App/`, run `python src-tauri/scripts/verify_migrations.py`.

## Important limits (intentional for Step 1)

- This creates the schema **without copying any server data**. It does not download or upload anything.
- The server's `auth`, `django_*` administrative/session/migration tables are excluded for security; all business-domain tables are included.
- Neither `App/src/` Vue views/stores nor `Server/` were modified. Vue is not yet connected to SeaORM; that is a later, separately requested step.
- A future sync API needs **backend support** for reliable revision checks. Creating metadata tables alone is not conflict resolution.
- The files under `src/entities/` are schema-derived compact SeaORM entities, but **the Rust crate could not be compiled in the packaging environment**, because Cargo/Rust is unavailable there. Run `cargo check` on your Windows laptop and share any error output if necessary.
- `Cargo.lock` is retained from the original ZIP and Cargo will refresh it to resolve new dependencies. Commit the refreshed lockfile after successful build.
- SQLite migrations are immutable once deployed; add `0003_...sql` for later changes rather than editing previous migrations.

## Client database architecture

`Vue` (unchanged) -> future Tauri command -> **Rust / SeaORM** -> **local SQLite**.

The existing online flow `Vue -> Django REST -> server database` is unchanged in this delivery.
