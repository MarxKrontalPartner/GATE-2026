# Stage 6 — Untersuchungsstellen directly in Bauwerksübersicht

## Goal
Users can add and review Untersuchungsstellen from the Bauwerk overview without going to the separate Arbeitsbereich first. Project detail reuses the same components.

## User flow
1. Dashboard → Bauwerk → Bauwerksübersicht.
2. Scroll to **Untersuchungsstellen**, see all Stellen assigned to the Bauwerk.
3. Click **Untersuchungsstelle hinzufügen**. The short dialog collects Name, optional Bemerkung, and a linked Projekt. When the Bauwerk has one Projekt it is preselected and displayed read-only; when it has multiple, a Projekt choice is required. Creation is disabled if no project exists.
4. On save, the existing `apiConnector.postxResource('bwd', 'Untersuchungsstelle', ...)` API runs (Tauri local mode → SeaORM CRUD). The list reloads; changes are offline local until future sync.
5. Each card contains name, ID, Projekt, remark, available position references/distances, and **up to three** existing investigation methods with In Arbeit / Erledigt state. A count indicates additional methods. The displayed measurements have **no invented units**.
6. Click **Untersuchungsstelle öffnen** to edit its complete measurements or create/edit investigations on the detail screen.

## Files and ownership
- `src/components/workspace/WorkspaceStelleCreateDialog.vue` — frontend UI; short standard Vuetify form
- `src/components/workspace/WorkspaceStelleCard.vue` — frontend UI; readable reusable Stelle card
- `src/components/workspace/WorkspaceStellenList.vue` — frontend integration; list, filtering/context, navigation and creation
- `src/composables/bwd/useWorkspaceStelleCreate.ts` — frontend data adapter; validation and existing API call
- `src/composables/bwd/useWorkspaceStelleMethods.ts` — frontend data adapter; one grouped request, isolate by Stelle ID
- `src/styles/_workspace-stelle-cards.scss` — design team/frontend; tablet responsive styles
- `src/views/bwd/BauwerkUebersichtView.vue`, `BauwerkProjektView.vue` — screen integration
- `tests/unit/workspaceStellenOverview.test.ts` — tests for Stelle isolation and linked project validation

**No schema or Rust/SeaORM/Django changes** are needed: the existing entities and commands support reading and creating Untersuchungsstellen.

## Verify on Mac
```sh
npm --workspace App run type-check
npm --workspace App run test
npm --workspace App run build
npm run dev
```

Manual checks: create a Stelle for a Bauwerk with one project and several projects; verify it is shown without reloading the app; verify opening it and editing measurements; create a Kernbohrung and return to confirm the card preview; ensure another Stelle's investigations aren't shown; check with Django offline; test portrait and landscape widths. The Bauwerk/Projekt pages no longer require an intermediate Arbeitsbereich click.

## Limitations
- Manual download/synchronization behavior is unchanged. Creating a Stelle stores data locally but does not upload it.
- Position units have not been defined by the client and are intentionally not shown.
- When investigations cannot be loaded, Stelle cards remain usable and show a loading/error message rather than falsely claiming zero methods.
