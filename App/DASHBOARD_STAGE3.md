# GATE-MKP Dashboard — Stage 3 (architecture branch)

## Scope

- Five clickable status filters: Synchronisiert, Nicht synchronisiert, Konflikte, Update verfügbar, Download verfügbar.
- Separate device and server connectivity, lightweight 30-second server probe.
- Last opened Bauwerk persisted in SQLite; active/archived views; search and responsive card lists.
- Reuses GateActionButton, GateStatusChip, GatePageHeader, OfflinePreparationAction, BauwerkCreateCard, and SCSS design tokens.
- Uses SeaORM entity finders and ActiveModels for all new Dashboard persistence. The domain Bauwerk delete operation cleans up dashboard FK metadata before deletion. Django Server unchanged.
- Full existing offline preparation is functional; per-Bauwerk download, upload, update and conflict resolution are **explicit placeholders**, not implemented yet.

## SQLite migrations

The included 0003, 0004 and 0005 migrations match the previous Dashboard prototype. The migration history is append-only. The debug app uses `gate-offline-architecture-dev.sqlite3` unless `GATE_CLIENT_DB_FILENAME` overrides it. Do not delete an existing offline database. Existing data imported at Step 2 should be prepared again so the Dashboard can establish verified server baselines; this never overwrites local records.

## Status rules and limitations

- Synced means a locally downloaded Bauwerk with a verified baseline, unchanged Bauwerk-level fields/project associations, and no pending **Bauwerk** record state.
- Unsynced means missing/unverified baseline, changed local Bauwerk-level attributes, or an existing non-synced Bauwerk state.
- Conflict requires an actual unresolved `client_sync_conflicts` record. Server updates alone never cause a conflict.
- Update verfügbar compares server Bauwerk attributes/project links to the downloaded baseline; it can coexist with local Unsynced state.
- Download verfügbar means the server catalog contains an ID not in local SQLite.
- **Not yet covered:** nested investigation revisions, child records or file changes. Correct end-to-end sync state will require server revision metadata and proper outbox tracking for nested edits. No false claim of completed synchronization is made by the UI actions.

## Test

1. `npm --workspace App run type-check` and `npm --workspace App run build` from repo root.
2. `cd App/src-tauri && cargo check && cargo test` on Mac.
3. Run Django and `npm run dev`; open Dashboard and press Vorbereiten.
4. Check all five counters; select/clear filters; search; active/archive tabs; open a Bauwerk and verify recent tile.
5. Stop Django while remaining online; verify independent device/server indicators and that local Bauwerke remain readable.
6. Modify a Bauwerk field locally; it should show Unsynced after reload (comparisons are Bauwerk-level only).
7. Server catalog changes should show Update verfügbar or Download verfügbar after Updates prüfen without altering local data.
8. Do not commit before the TypeScript build, Rust compile and UI smoke tests pass.

## Git staging

`git add App/src App/src-tauri/src App/src-tauri/migrations App/DASHBOARD_STAGE3.md` — avoid root `package-lock.json`, user ZIP backups and Django Server.
