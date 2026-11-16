# Stage 5 — Tablet-friendly Untersuchungsstelle detail

**Branch:** `feature/app-architecture-refactor` (after the working Stage 4 commit)

## User journey

1. Dashboard -> downloaded Bauwerk -> Projekt (or Bauwerksübersicht) -> select an Untersuchungsstelle -> **UntersuchungsstelleDetail**.
2. Inspect the Stelle's name, explicit project association, remarks, location references, height, and two distances. The DB does not define display units for the numeric fields; do not assume units.
3. View *only* this Stelle's linked Vor-Ort-/Laboruntersuchungen with counts and completion states. Open a method in the existing investigation detail form.
4. **Bearbeiten** opens a large, tablet-friendly dialog. Edit name, remark, and location reference/measurement fields. Save with the existing `patchxResource('bwd', 'Untersuchungsstelle', ...)` API; in Tauri it calls `local_update` -> `domain_service` -> SeaORM ActiveModel -> SQLite. No new Tauri command, SQL or migrations.
5. **Neue Untersuchung** uses the already-working creation dialog, then refreshes the list. Return to Bauwerks-/Projektübersicht or the full Arbeitsbereich.
6. Offline: after **Vorbereiten**, Tauri reads/edits SQLite without needing Django. Upload and conflict resolution remain separate prototype work.

## Changes, by team

| Team | Owner/contract | Changed in Stage 5 |
|---|---|---|
| Frontend UI | `src/views/bwd/UntersuchungsstelleDetailView.vue`, `src/components/workspace/WorkspaceStelle*.vue` | Detail page, facts, investigation cards, edit dialog |
| Frontend state and API | `src/composables/bwd/useUntersuchungsstelleDetail.ts`, `stelleMethods.ts`, existing `ApiConnector` | Load/save through existing Tauri API; filter by exact `bauwerk` and `untersuchungsstelle` FK |
| Frontend navigation | `src/router/index.ts`, `WorkspaceStellenList.vue`, `StructureListItem.vue`, `UntersuchungsstellenView.vue` | Routes and detail links without removing existing expansion/edit/create paths |
| Design/UX | `src/styles/_stelle.scss`, central `_tokens.scss` | High-contrast Vuetify cards, responsive grid, 58px actions; no component-inline CSS |
| Rust/backend | Existing `local_get`, `local_update`, `local_uv_grouped`; SeaORM repository | **No changes** required for existing fields; review API contract if validation rules evolve |
| Database designer | Existing `bauwerksdiagnostik_untersuchungsstelle`, method foreign keys | **No schema changes**; agree on measurement unit semantics before adding units to UI |
| Django backend | Existing BWD API endpoints | **No changes** for this stage; future auth/sync contracts are separate |
| QA | `tests/unit/stelleMethods.test.ts`, manual Tauri checks | Cross-Stelle isolation, project navigation, offline edits, investigation details |

## Data flow

```text
Projekt / Bauwerk / Arbeitsbereich
  -> router /:bauwerkId/bwd/Untersuchungsstelle/:stelleId
  -> useUntersuchungsstelleDetail
  -> apiConnector.getXResource('bwd','Untersuchungsstelle', id)
       -> Tauri local_get -> SeaORM Entity::find_by_id -> SQLite
  -> apiConnector.BWD.fetchUvData('all', bauwerkId)
       -> Tauri grouped method data from local SeaORM entities
       -> filter by exact untersuchungsstelle ID (never infer by Projekt)
  -> WorkspaceStelleFacts + WorkspaceStelleMethods
  -> edit dialog -> patchxResource -> local_update -> SeaORM ActiveModel -> SQLite
```

## QA checks

- `npm --workspace App run type-check`, `npm --workspace App run test`, `npm --workspace App run build`, `npm run dev`.
- Open a downloaded Bauwerk, then a Projekt and one Untersuchungsstelle. Project back button should return to that project.
- Open the same Stelle from Bauwerksübersicht, and from the Arbeitsbereich's **Details öffnen** action.
- Check the correct name, project, location refs, measurements, remarks and listed Verfahren.
- Two Stellen (including different Projekte) must never mix investigation methods.
- Edit a Stelle (including 0/empty measurements), save, navigate away and back. Values must persist locally.
- Open a method and test its existing detail/edit flow. Create a method and see the count refresh.
- Stop Django; cached details and local editing must still work.
- Check landscape and portrait tablet widths; buttons must be easy to tap.

**Limits:** No project reassignment in this editor; relationship changes need their own workflow. Media thumbnails/transfers, complete syncing, field-level conflict resolution and server-side version history are not implemented here. We do not claim numeric fields have a particular physical unit until agreed with the domain team.

**Packaging:** full `App/` archive, preserving `.env` locally. Do not overwrite the whole `src/` folder from a sparse ZIP. Do not stage root `package-lock.json` or ZIP backups in Git.
