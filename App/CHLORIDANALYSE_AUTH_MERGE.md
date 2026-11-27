# Stage 8 Chloridanalyse integrated into GATE-auth-offline (2026-11-25)

**Base:** the uploaded `GATE-auth-offline(1).zip` full repository, NOT Stage 8.
The existing Django Server, Keycloak authentication, SQLite migrations `0001–0009`,
Rust services, repositories, entities and sync engine were all preserved.

## What was added

- Tablet-friendly Chloridanalyse form and SCSS, typed draft and validation tests.
- Shared basic investigation fields between Kernbohrung and Chloridanalyse.
- Full create/read/update route through the existing ApiConnector and Tauri/SeaORM.
- Support for offline-created negative investigation IDs in the create dialog,
  investigation list and route, and local edits before the first synchronization.
- Report, photo, Bearbeiter and Probe-creation controls honor the authentication-
  enabled app's server-first and sync rules; unsupported offline media/Probe writes
  are not advertised as working.

## Data flow

- When online, Chloridanalyse create/update uses the Django API then mirrors the
  result into local SQLite, using the Keycloak authorization fetch wrapper.
- When offline, Chloridanalyse create/update uses local SQLite and its existing
  diagnostic sync/outbox machinery. Offline-created records have IDs `-1, -2, ...`.
- The local data read-back checks measurement values and the selected Probe.
- Existing Proben downloaded from Django may be assigned while offline; new
  Proben require the server. Photo/PDF uploads and Bearbeiter links also require
  server access; media changes for a pending negative ID must wait until sync.
- Existing M:N associations (Bearbeiter, media) are NOT part of the field-level
  offline sync specification, so their editing is gated to server-online mode.
- Deleting investigations while offline is not supported by this version.

## Test on macOS

1. Make a backup of your project and **never delete your existing client SQLite DB**.
2. From the repository root: `npm install` if dependencies are missing; `npm run dev`
   launches native Tauri, `npm run dev:web` launches browser-only Django mode.
3. Django + Keycloak setup is the same as in the input repository; do not replace
   its Server with Stage 8. Sign in while online, then prepare/download Bauwerk data.
4. Open Untersuchungsstelle > Neue Untersuchung > Chloridanalyse. Save measurements.
5. Disconnect Django, edit and save Chloridanalyse, reopen and restart Tauri;
   verify the saved measurement values. Create a *new* Chloridanalyse while offline;
   verify the temporary negative ID opens and survives restart.
6. Reconnect, use the existing Synchronisieren action; verify server-side IDs and
   values. Test Probe creation and PDF/photo uploads only when online and synced.
7. Existing DB migration files `0006–0009` must remain; do not switch to the
   older Stage 8 source or delete the database.

## Validation status

Source-level merge was reviewed against the uploaded project. Use
`npm --workspace App run type-check`, `npm --workspace App run test` and
`npm run dev` to verify in your local Tauri environment.
