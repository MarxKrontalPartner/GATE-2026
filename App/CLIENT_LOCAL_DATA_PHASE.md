# Client local data layer — production replacement for Step 2 demo

## Scope

Only `App/` changes. This is a local-first client persistence baseline for existing Bauwerksdiagnostik workflows; it does **not** upload changes to Django, synchronize revisions, or resolve conflicts. The nine local sync-related schema tables inherited from Step 1 are unused in this phase.

The Step 2 demo screen, route, store, repository and `LOCAL-DEMO` Tauri commands were removed. Existing Pinia stores/views are kept; their `ApiConnector` delegates to local Tauri calls in native builds and uses existing Django HTTP APIs in the web browser.

## Architecture

Existing Vue screens and Pinia stores → `src/utils/ApiConnector.ts` → `src/repositories/localDomainRepository.ts` → Tauri `src-tauri/src/commands/local_data.rs` → `src-tauri/src/services/{domain_service,media_service}.rs` → `src-tauri/src/repositories/domain_repository.rs` → SeaORM `DatabaseConnection` + SQL statements → local SQLite.

The generated SeaORM entity files remain in `src-tauri/src/entities/`, and the schema comes from Step 1. The broad shared repository uses bound statements through SeaORM's connection and transactions, rather than writing one SeaORM entity repository per Django model. The SQL API is scoped by a Rust whitelist of domain resources and validated table columns. Core local business rules for diagnostic creation and ordering reside in Rust services.

## Why `assetverwaltung` is included

`Bauwerk`, `Projekt`, `Person`, and `Firma` are supporting references. Diagnostic methods and investigation points have foreign keys to them. The phase does not add new asset-management screens or expand the server-side asset module.

## How to run

Keep using the existing root scripts documented by your project (`npm run server:dev` for the backend when initial data import is needed, `npm run dev` for the Tauri app). Existing local migrations are automatic at native app startup.

## Prepare existing data once while online

1. Run Django and the Tauri desktop app.
2. In **Bauwerkswahl**, click **Daten für Offline-Arbeit vorbereiten**.
3. The app downloads reference records, investigation points, five method types, material/core/sample records, media metadata and media file contents via the **existing** Django read endpoints.
4. It imports parents first, then many-to-many links. Existing client rows are not overwritten. It reports errors rather than silently treating a partial import as complete.
5. Stop Django and reopen the app. Navigate through buildings, investigation points and diagnostic forms, then save and reopen a measurement.

Initial preparation is **not** synchronization. It never uploads local records. Re-running does not merge server edits, and users should prepare before making local edits. Current server API endpoints must expose all requested records; private server-only rows cannot be automatically transferred without new API support.

## Supported local data

- Projects, buildings, people and companies: offline reference records and existing UI operations.
- Investigation points, five diagnostic methods, core drilling samples, materials: SQLite create/read/update/delete, foreign keys, relevant M:N links and JSON fields.
- Media: metadata in SQLite and file bytes under Tauri app-data `local-media/`; media inheritance records and image annotations are handled for known media types.
- Web (`dev:web`) continues to use the server API, because browser mode has no Tauri Rust/SQLite connection.

## Important limitations / validation gates

- This is a **source implementation, not a verified release**. Rust/Cargo and the complete Vue app could not be compiled in this delivery environment; build and test it in your macOS development setup. The SQL migrations and sample relational data were tested separately.
- Imported server IDs and locally generated numeric IDs are not yet mapped for multi-device synchronization. Do not treat this phase as safe for deploying to multiple tablets or for uploading offline edits. Copy and back up the local database before testing migrations.
- Date defaults, server-only rules, image metadata details, cascading deletes and very large media files require end-to-end acceptance tests against representative diagnostic records. Media IPC is base64-based and has a 96 MB per-file limit for this phase.
- The download is an explicit snapshot (not a background cache or revision-based update). No queueing, conflict resolution, authentication or authorization additions were made.
- Verify complex form data and linked files before relying on offline entry for production fieldwork.

## Tests to run on your machine

1. `npm run dev` from the project root; confirm Tauri compiles.
2. `cd App && npm run type-check` (or `npm run build`).
3. Download data from a populated Django server; check row counts and linked project/building IDs.
4. With Django stopped, open each of the five diagnostic types; create, edit, delete and restart the app.
5. Test JSON fields (`risse`, `messreihe`, `schichtaufbau`), M:N (`bilder`, `bearbeiter`, `material`, `foto`), and file uploads/reopen.
6. Verify errors for missing FK records, and ensure no native CRUD call hits Django.

Changes to `Server/` are neither required nor included.
