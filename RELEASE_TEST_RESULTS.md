# GATE-MKP — focused regression tests (27 November 2026)

Source: `GATE-auth-offline-chloridanalyse-edit-fixed-2026-11-25.zip`.
Authentication/Keycloak and all SQL migrations remain untouched.

## Executed here

| Test category | Result | Command |
| --- | --- | --- |
| `syncService.ts` actual runtime logic, with mocked Tauri and HTTP | **7 passed** | `node --test App/tests/release/syncService.contract.test.cjs` |
| SQLite migrations 0001–0009, material FK integrity, two-database mismatch | **4 passed** | `python3 -m unittest discover -s App/tests/release -p 'test_*.py' -v` |
| TypeScript syntax of changed `syncService.ts` | No errors | TypeScript transpile diagnostics |
| Django Material API test file syntax | No errors | `python3 -m py_compile Server/core/bauwerksdiagnostik/tests/test_material_references.py` |

Before the fix, the test `201 with incorrect created-record count must NOT be marked successful` **failed**. After the fix, **all 11 executable tests passed**.

## Changed production file

`App/src/services/syncService.ts`:
1. A response with HTTP 201 but missing/malformed record IDs or the wrong number of created records is no longer treated as success. The pending outbox is preserved for retry.
2. If a batch create fails, do not immediately send child/field updates that may still contain temporary offline IDs.

## Django API tests added (not run in this environment)

`Server/core/bauwerksdiagnostik/tests/test_material_references.py`:
1. Existing material ID is accepted.
2. Missing material ID returns HTTP 400 without creating the Bohrkern.
3. Stale material ID on edit returns HTTP 400 and preserves the original Material relation.

Run on Mac (with Django venv installed):

```bash
cd Server
uv run pytest -q core/bauwerksdiagnostik/tests/test_material_references.py
```

## Not executed here

- Full Vitest frontend suite: npm dependency downloads unavailable (no network access).
- Full Django pytest suite: required Python 3.12 and Django packages unavailable.
- Rust/SeaORM tests and native Tauri build: Rust toolchain unavailable.
- Real Keycloak login and Mac SQLite ↔ Django integration: requires running services and your local data.

## Remaining release blockers / recommended next tests

- The recurring `material: Invalid pk "2"` indicates a request to Django with a Material ID not present on that server database. The tests demonstrate that **a valid ID in local SQLite is not sufficient**. Compare actual Material rows on the client and Django server; do not assume IDs are interchangeable across independent databases.
- `Material`, `Bohrkern` and `Probe` are not included in the diagnostic `SYNCED_RESOURCES` create-sync loop. The app allows only diagnostic forms to be created offline, but reference integrity must still be checked when online requests use locally selected references. Implement explicit server/local reference mapping or download refresh with validation before final release.
- Bulk-create retries are **not guaranteed idempotent** if Django creates rows but the acknowledgment is lost. A stable client UUID/idempotency mechanism is advisable to avoid duplicate server records on uncertain retries.
- Add native end-to-end tests for login, offline save, restarting, reconnecting, syncing, reconciling Material relationships, and reopening Chloridanalyse/Kernbohrung.

**Do not delete or reset live databases to address these issues.**
