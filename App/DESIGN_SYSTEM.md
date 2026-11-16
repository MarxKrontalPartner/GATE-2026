# GATE-MKP Design System — Stage 2

**Branch:** `feature/app-architecture-refactor` (continue from completed SeaORM Stage 1)

## Source of truth

- `src/plugins/vuetify.ts`: Vuetify semantic theme colors (light and dark).
- `src/styles/_tokens.scss`: tablet design tokens; CSS custom properties provide runtime responsiveness.
- `src/styles/_typography.scss`, `_layout.scss`, `_buttons.scss`, `_cards.scss`, `_forms.scss`, `_responsive.scss`, `_feedback.scss`: reusable styles.
- `src/styles/_index.scss`: SCSS entry for all shared styles; imported by `src/assets/main.scss`.
- View/component-local style files: `MyComponent.styles.scss`, referenced with `<style scoped lang="scss" src="./MyComponent.styles.scss"></style>`.
- `src/components/common/`: reusable Vuetify components. Do not wrap every native Vuetify component needlessly.

## Components

- `GateActionButton`: shared, glove-sized buttons with `intent` (`primary`, `secondary`, `danger`, `quiet`).
- `GatePageHeader`: responsive page headings with an `actions` slot.
- `GateStatusChip`: semantic status colors and labels (5 synchronization states plus completed/in-progress).
- `OfflinePreparationAction`: reusable local download action; emits `prepared` when done.
- Existing `DialogComponent`: controlled dialog that now forwards `update:modelValue` correctly.
- `BauwerkSearch`, `BauwerkList`: display-only Bauwerkswahl elements.
- `useBauwerkCatalog`: data filtering and existing refresh logic, separate from template.

## Rules for new UI

1. Prefer standard Vuetify (`v-btn`, `v-card`, `v-text-field`, `v-row`, `v-col`, `v-dialog`).
2. Reuse shared wrappers only for common GATE-specific semantics/behavior.
3. Store colors in Vuetify themes; do not hardcode theme colors in each Vue file.
4. Define sizes, responsive spacing, and touch dimensions in SCSS tokens. Target at least 58px for primary touch actions.
5. Views compose smaller components and composables. Keep data logic out of the template.
6. Put component-only CSS in matching `.styles.scss` and keep it scoped. Put shared layout in `src/styles`.
7. Preserve behavior and avoid changing stores, Rust Tauri commands, or server contracts during UI-only refactors.
8. Use Vuetify responsive grid and `useDisplay()` for breakpoint-dependent behavior.

## What is in scope

Stage 2 refactors the existing **Step 2** Bauwerkswahl, common buttons/dialogs, status display, and stylesheet organization across the current Vue application. It does **not** implement the new Dashboard; that's Stage 3. It does not modify `Server/` or the Tauri database layer.

## Test plan before committing Stage 2

From project root:

```sh
npm --workspace App run type-check
npm --workspace App run build
npm --workspace App run test
npm run dev
```

Test Bauwerkswahl search/open/create; offline preparation from Django; Untersuchungsstellen navigation; editing an investigation and its completion status; light/dark theme; landscape and portrait window sizes; dialogs and feedback messages. Do not commit Stage 2 if these checks fail.
