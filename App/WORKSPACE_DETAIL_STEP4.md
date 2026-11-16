# Stage 4 — Tablet Bauwerk / Projekt Workspace

**Branch:** `feature/app-architecture-refactor` (after commit `cc4827d`)

## New user flow

1. Dashboard / Bauwerksliste → select a **downloaded Bauwerk** → **Bauwerksübersicht**.
2. The overview shows Bauwerk fields, cached synchronization status, linked **Projekte** and Untersuchungsstellen.
3. Select **Projekt ansehen** → Project details and *only that project's* Untersuchungsstellen.
4. Select **Untersuchungen ansehen** → existing Untersuchungsstellen Arbeitsbereich, with the selected Stelle expanded and the Projekt filter applied.
5. Select **Arbeitsbereich öffnen** → all Untersuchungsstellen for the Bauwerk.
6. Use the side drawer or back actions to return.

**Domain rule:** Bauwerk ↔ Projekt is many-to-many; a single Untersuchungsstelle references one Bauwerk and may reference a Projekt. Never filter Bauwerke by an invented single `projekt` property.

## Team responsibilities — concrete file contracts

| Owner | Files / contract | Change in Stage 4 |
|---|---|---|
| Frontend Vue | `src/views/bwd/BauwerkUebersichtView.vue`, `BauwerkProjektView.vue`, `components/workspace/*` | New tablet views and reusable components |
| Frontend state/API | `src/composables/useBauwerkWorkspace.ts`; `src/utils/ApiConnector.ts` existing | Load Bauwerk by ID; linked Projekte; strictly filtered Stellen |
| Frontend navigation | `src/router/index.ts`; `src/components/bwd/SideDrawer.vue`; `src/views/bauwerksverwaltung/BauwerkswahlView.vue` | Explicit routes and links |
| Design/UX | `src/styles/_workspace.scss`, central `src/styles/_tokens.scss` and Vuetify | Shared high-contrast touch targets and responsive breakpoints |
| Rust/Tauri backend | Existing `local_get`, `local_list`, `domain_repository.rs` | **No modification**: already provides `Entity::find()` / filtered finder / relationship links |
| Database designer | Existing `assetverwaltung_bauwerk`, `assetverwaltung_projekt`, `assetverwaltung_bauwerk_projekt`, `bauwerksdiagnostik_untersuchungsstelle` | **No schema migration** required |
| Django backend | Existing Assetverwaltung and BWD endpoints | **No modification** in this step |
| QA | New workspace unit test + Tauri manual tests | Verify FK isolation, linked projects, offline behavior |

## Existing Tauri data flow

```
Dashboard Open
  -> Vue router: /:bauwerkId/bwd/Uebersicht
  -> useBauwerkWorkspace.ts
  -> ApiConnector (Tauri mode)
  -> local_get('Bauwerk', id)                 -> SeaORM find by ID + Bauwerk-Projekt links
  -> local_list('Projekt')                   -> SeaORM Entity::find()
  -> local_list('Untersuchungsstelle',       -> SeaORM Entity::find().filter(BauwerkId)
                { bauwerk: id })
  -> Vue components + SCSS
```

No network required after the data was downloaded using **Vorbereiten**. Browser-preview fallback uses existing Django API.

## Adding a new field later (example: Baustellenadresse)

1. **Database designer + backend:** Agree server/local field, nullability, ownership, default and migration with frontend.
2. **Django backend:** Add/update server model, serializer, endpoint if the field is in server data.
3. **Local DB / Rust developer:** Add SQLite migration and update/regenerate SeaORM entity. The existing `local_get` finder includes the new field without hand-editing SELECT lists.
4. **Frontend developer:** Update `IBauwerk`, the form/display component, and any mapping needed for data exchange. No change to the generic Tauri read command for a simple scalar column.
5. **Sync owner:** Explicitly decide whether the new field participates in server revisions, conflict detection and uploads.
6. **QA:** Test both old migrated SQLite data and fresh installation; test offline and online variants.

## Test checklist

- `npm --workspace App run type-check`
- `npm --workspace App run test`
- `npm --workspace App run build`
- `npm run dev` from project root
- Dashboard: open a downloaded Bauwerk, and verify fields, linked Projekte and available Stellen.
- Click Project A: verify only Stellen explicitly linked to Project A are displayed.
- Click **Untersuchungen ansehen**: old Arbeitsbereich opens with matching Project filter and Stelle expanded.
- Back navigation and side drawer work; existing create/edit/delete investigation operations still work.
- Stop Django: opening cached Bauwerk/Projekt/Stelle still works in Tauri.
- No new migrations, no change to Server; don't include root `package-lock.json` or backup ZIPs in commit.

**Limits:** Server-only Bauwerke still require a download first. This stage does not implement synchronization, media transfer or conflict resolution. For a server Bauwerk not downloaded locally, the Dashboard retains its existing download placeholder.
