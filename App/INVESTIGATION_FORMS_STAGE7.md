# Stage 7 — Untersuchungsverfahren / Kernbohrung forms

Baseline: Stage 6 App (`feature/app-architecture-refactor`). **No Rust, Django, migrations, or existing repository changes.**

## User flow
Dashboard → Bauwerksübersicht → Untersuchungsstelle → **Neue Untersuchung** → choose Art / Verfahren, Datum / optional Zeit / Bemerkung → **Untersuchung anlegen** → detailed procedure form (Kernbohrung).

- The creation dialog opens entirely offline; available method choices come from the shared `UntersuchungsverfahrenRegistry`. On Tauri, `createUntersuchung` invokes `local_create` through `ApiConnector` and the existing SeaORM repository. The selected Bauwerk/Projekt/Stelle IDs are validated before creation.
- The Kernbohrung details page has a local edit draft, Save and Cancel, drilling measurements, layers, Bohrkern and existing image overview. Edits save through `UntersuchungsverfahrenStore.updateUntersuchung` / Tauri `local_update`. The status `Abgeschlossen` describes the investigation; this is separate from Dashboard synchronization status.
- Creating/linking a new Bohrkern is currently an immediate local write, as in the original implementation. Changing an existing Bohrkern and the parent Kernbohrung requires two API calls; these **are not atomic together**. A future backend enhancement should introduce a Tauri service-level transaction for composite saves.
- The Untersuchungsstelle edit dialog retains Name, Bemerkung, Höhe/Lagenbezug and Abstände; it is visually grouped into tablet-friendly sections. Measurement units for Stelle positions remain unspecified until the client approves them.

## Files and responsibilities

| Team | Area |
| --- | --- |
| Frontend | `components/bwd/CreateUntersuchungButton.vue`, `components/investigations/*`, `views/bwd/Untersuchungsverfahren/VorOrtUntersuchung/kernbohrungView.vue` |
| Frontend/design | `components/bwd/CreateUntersuchungButton.styles.scss`, `views/.../kernbohrungView.styles.scss`, `styles/_stelle.scss` and shared tokens |
| Database | No migration; existing Kernbohrung, Bohrkern and Untersuchungsstelle entities |
| Rust/backend | No new command; evaluate transaction support for composite Bohrkern saves in a later task |
| Django backend | Unchanged; local-first operations use Tauri and SeaORM |

## Test checklist (Mac)

1. `npm --workspace App run type-check`, `npm --workspace App run test`, `npm --workspace App run build`, `npm run dev`.
2. Open a downloaded Bauwerk and Stelle; choose **Neue Untersuchung** → Kernbohrung, set Datum and Bemerkung, save. Confirm the new Kernbohrung detail opens automatically.
3. Enter edit mode; save diameter (mm), drilling length (mm), angle, cooling, closure, reinforcement and completeness. Reload and check values persist.
4. Add two layers, record end depths, save, reopen and check sequential depth boundaries and descriptions.
5. Change the draft and choose Cancel. Reopen the form and confirm unchanged stored values.
6. Create a Bohrkern, set its materials / number of samples, save and reopen.
7. Edit a Stelle’s name, notes, optional reference heights/distances; save and reopen. Test small portrait window width.
8. Stop Django and repeat the already-downloaded local create/edit/read flow. No upload/sync expected.

Do not commit until the frontend build and local CRUD checks pass.
