# Kernbohrung / Bearbeiter correction (2026-11-27)

**Base:** `GATE-auth-offline-sync-unit-tested-2026-11-27.zip` (Keycloak + Chloridanalyse + migrations 0001–0009). No Server/Rust/migration changes.

## Why the errors happened

1. A Kernbohrung created offline gets a **temporary negative ID**. The old Bohrkern button rejected every ID <= 0 with the misleading message *"Bitte zuerst die Kernbohrung anlegen und öffnen."* A Bohrkern is not an independently synchronized resource in this project, so blindly inserting a local Bohrkern would leave an unsyncable linked record.
2. The Bearbeiter autocomplete used only local `Person` records; if the offline cache was empty, the user saw no options, and any fetch error was silently discarded.

## What changed

- Kernbohrung now explains temporary IDs and provides an explicit `Kernbohrungen synchronisieren` action (applies to all pending Kernbohrung records, not just the current record). **First save measurements**, then click sync. Once sync succeeds, return to the Untersuchungsstelle and reopen the same investigation with its new server ID before creating a Bohrkern. If sync fails, the offline record is preserved. Bohrkern creation still needs Django online.
- Bearbeiter loads authenticated server `assetverwaltung/person/` while online, also caches `Firma` then `Person` in SQLite. Offline, it displays cached Personen and prevents editing M:N links that cannot be synchronized. If no server Personen exist or a 401/404 occurs, the form explains why and shows `Bearbeiter aktualisieren`.
- Shared Bearbeiter behavior is also used in Chloridanalyse. Changed M:N assignments cannot be silently saved offline.
- New tests cover negative, missing and positive IDs.

## Apply to your current project

If your current source corresponds to the base above, back up/commit changes first and extract the *small patch ZIP* into your repository root, maintaining its `App/` paths. Do not copy the full ZIP over a newer app with other changes.

From the repository root on macOS:

```sh
npm --workspace App run type-check
npm --workspace App run test
npm run dev
```

Optional independent check without npm dependencies (Node 22):

```sh
node --experimental-strip-types --test App/tests/release/kernbohrung-core-state.test.mjs
```

No new migration, seed, or npm package is required. Existing data remains untouched.

## Important limits

The release still needs a true cross-database E2E test (Keycloak / Django / local SQLite) and real reference-ID mapping for Material and other relationships. This patch does not add offline synchronization for Bohrkern or M:N Bearbeiter relationships. It prevents incorrect operations and gives users a working sync-first path.
