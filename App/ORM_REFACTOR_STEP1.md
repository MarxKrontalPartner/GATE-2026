# GATE-MKP | Architecture Refactor: Stage 1 — Entity-first SeaORM

Branch: `feature/app-architecture-refactor` (based on committed Step 2 `db5ef17`).

## Why

The local domain repository previously assembled SQL `SELECT`, `INSERT`, `UPDATE` and `DELETE` clauses using database column names. Adding a normal field meant maintaining SQL projections and mutation strings. This stage replaces those operations with SeaORM `Entity::find()`, `ActiveModel`, and entity-based relationship queries. It does **not** change the Vue UI or Django backend.

## Files changed

- `src-tauri/src/repositories/domain_repository.rs`: Entity finder and ActiveModel CRUD for 14 whitelisted domain resources; junction entities for all 18 many-to-many links; media subclass CRUD. Retains only `PRAGMA table_info` introspection and narrowly scoped SQL for polymorphic media reference cleanup. Filters are checked against actual entity columns.
- `src-tauri/src/services/domain_service.rs`: ORM finders for cross-method diagnostic sort ordering, with no hand-assembled SQL.
- `src-tauri/src/db.rs`: Debug builds use an isolated DB filename by default to avoid conflicts with earlier Dashboard migration 0003–0005. Release builds retain `gate-offline.sqlite3`. Override by environment variable `GATE_CLIENT_DB_FILENAME` (simple `.sqlite3` filename, no path).

## Database migration policy

1. Add a *new* incremental migration to `src-tauri/migrations` for every schema change. Never edit an already applied migration.
2. Update/regenerate the matching file in `src-tauri/src/entities` (including special `#[sea_orm(column_name = "...")]` mappings).
3. Standard CRUD and load-all queries use the entity field list without editing SQL. UI forms, Tauri payload contracts, synchronization rules, and special server mappings must still be changed when the new field participates in their behavior.
4. Do not bring migration 0003–0005 from the unfinished Dashboard into this branch. Resolve migration sequencing deliberately when integrating Dashboard later.

## Test on your Mac (before commit)

```bash
# Confirm branch
 git branch --show-current
# Native backend
 cd App/src-tauri
 cargo check
 cargo test
 # Optional formatting step (may reformat other Rust modules): cargo fmt
 cd ../..
# Frontend (existing Step 2; should stay unchanged)
 npm --workspace App run build
# Native app with a fresh isolated debug DB
 npm run dev
```

Verify create/read/update/delete with Bauwerk, Untersuchungsstelle and each diagnostic method; verify M:N associations and media annotations; test `local_import` twice to ensure existing local rows are not overwritten. Test with no network connection.

Check the console for `[GATE] Client SQLite:` to confirm the isolated development DB path. The previous Dashboard DB remains untouched.

## Git staging advice

Do not stage unrelated root `package-lock.json` or ZIP backups. Stage explicitly:

```bash
 git add App/src-tauri/src/repositories/domain_repository.rs \
   App/src-tauri/src/services/domain_service.rs \
   App/src-tauri/src/db.rs App/ORM_REFACTOR_STEP1.md
```

**Status:** Source-level structural checks performed for this candidate; native Rust compilation and runtime tests must be performed on the Mac. Stage 2 (Vue/Vuetify/SCSS design system) is deliberately not mixed into this code change.
