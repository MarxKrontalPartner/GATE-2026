# Stage 7 follow-up: Kernbohrung persistence and Bohrkern linking

## Fixes

- The Kernbohrung screen now responds to a **fresh record object**, not only a changed investigation ID. It avoids overwriting a user’s draft while editing.
- Saving verifies a **fresh database read** for Lufttemperatur, Luftfeuchte, Verschlussmaterial and the other primary drilling fields before reporting success. If a value is missing, the editable draft remains and an explicit error is shown.
- Bohrkern creation/linking is one **SeaORM transaction** in `repositories/kernbohrung_repository.rs`, exposed through `local_create_or_link_bohrkern` and used only for native Tauri. Neither a new database column nor a Django change is needed.
- The optional **Bereits angelegten Bohrkern verknüpfen** path allows linking an existing orphan core after an earlier partial failure. The provided ID must belong to an existing, currently unlinked Bohrkern; it is not a generated name or measurement. Confirm the core ID before using this recovery option.
- Browser-only mode still uses the original Django operations until the server offers an atomic endpoint; the native guarantee applies only to SQLite/Tauri.

## How to test

1. Back up the existing `gate-offline-architecture-dev.sqlite3` (including its `-wal` and `-shm` files if present) **after stopping the app**. Do not reset the database.
2. Run `npm --workspace App run type-check`, `npm --workspace App run test`, `npm --workspace App run build`.
3. Run `cd App/src-tauri && cargo check && cargo test`, then `npm run dev` from the project root.
4. In an existing Kernbohrung, enter temperature (e.g. 18.5), humidity (e.g. 53) and Verschlussmaterial (e.g. Mörtel), save, navigate away, then reopen; both view and edit modes should show the values.
5. Create a Bohrkern, save, leave the form, then reopen. The association and name must remain.
6. If you previously created an unlinked Bohrkern, choose **Bereits angelegten Bohrkern verknüpfen**, enter the **actual numeric ID** from the earlier error message, and verify it appears after reopening. Do not create another duplicate to recover it.
7. Try a second create on an already linked Kernbohrung: the command should reject it and avoid creating another core.

### Troubleshooting

If the read-back check reports missing fields, inspect the **existing database row** in a safe read-only tool. Do not replace the local SQLite database or remove migrations. The exact runtime error (if any) and `cargo check` output are needed before changing the underlying generic ORM repository.

## Scope

The patch changes only the Kernbohrung form, a pure draft helper, the new local SeaORM transaction repository/command registration and unit tests. It does not touch `Server/`, modify the migration history, or change the other investigation forms.
