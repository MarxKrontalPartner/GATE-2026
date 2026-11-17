# Stage 7 follow-up — Kernbohrung field, Bohrkern and image fixes

## Schichtaufbau

- `Tiefe von` is now editable in edit mode.
- The next Schicht suggests the previous `Tiefe bis` as its initial `Tiefe von`, but does **not** copy its final depth, name or description.
- A newly added Schicht has an empty `Tiefe bis`; both values must be completed before saving.
- Editing or deleting a Schicht does **not** silently rewrite other Schicht measurements.
- No schema or API change: `schichtaufbau` remains the same JSON field.

## Bohrkern

- Errors from creation/linking display inside the Bohrkern section, beside the button.
- A Bohrkern saved but not yet linked is held in memory for retry, preventing another click from creating a duplicate during that editing session.
- After successful linking, the Bohrkern is shown immediately, and remains visible after switching to view mode.
- The creation/link sequence still comprises two separate existing SeaORM-backed Tauri calls. A future Rust service transaction should make them atomic. Users should report any **inline error text** if it still fails.

## Images and documents

- Existing `UntersuchungsImageCard` now renders even when there are zero images, so the first image can be uploaded.
- In edit mode, image add/edit tools are available, using the existing local media API. In view mode, images can be viewed.
- General-purpose document/PDF attachment management is **not** part of this Kernbohrung form. That should be designed against the client's required attachment relationship, rather than storing documents as `bilder`.
- Image uploads are saved separately from unsaved measurement drafts. The UI should explain this distinction to users before final release.

## macOS test

1. Build: `npm --workspace App run type-check && npm --workspace App run test && npm --workspace App run build`.
2. Open Kernbohrung, choose **Bearbeiten**.
3. Enter Schicht 1 (`Tiefe von: 0`, `Tiefe bis: 12`), add Schicht 2: it must suggest `Tiefe von: 12` and leave `Tiefe bis` empty.
4. Change Schicht 2 `Tiefe von` independently; Schicht 1 must remain unchanged.
5. Save and reopen; verify the exact depths are retained.
6. Add a named Bohrkern. Confirm it appears immediately and after **Änderungen speichern**, reopening the investigation and restarting the app.
7. Add the first image with **HINZUFÜGEN**, while in edit mode; verify it remains after reloading, including offline.
8. If Bohrkern creation fails, report the error **inside the Bohrkern section**. Do not repeatedly click until the error is understood.

No Rust, migration or Django files were modified in this package.
