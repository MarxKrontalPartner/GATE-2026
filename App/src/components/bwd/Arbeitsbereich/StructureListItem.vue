<template>
  <v-list-item
    class="structure-list-item"
    :class="{ 'is-expanded': expanded }"
    :prepend-icon="mdiMapMarker"
    :aria-expanded="expanded"
    @click="$emit('click', item)"
  >
    <v-list-item-title class="structure-item-title">
      <div class="item-title-row">
        <v-tooltip v-if="item.bemerkung" :text="item.bemerkung" max-width="360">
          <template v-slot:activator="{ props }">
            <span v-bind="props" class="item-name-text">{{ displayName }}</span>
          </template>
        </v-tooltip>
        <span v-else class="item-name-text">{{ displayName }}</span>

      </div>
    </v-list-item-title>
    <v-list-item-subtitle class="structure-item-subtitle">
      <span>ID: {{ item.id }}</span>
      <span class="subtitle-part">{{ methodSummary }}</span>
      <span v-if="projektName" class="subtitle-part">{{ projektName }}</span>
    </v-list-item-subtitle>

    <!-- Action Menu -->
    <template v-slot:append>
      <div
        class="node-progress-wrapper progress-preserve"
        :class="{ 'is-visible': showProgress && progress.total > 0, 'is-collapsed': !showProgress }"
      >
        <v-tooltip v-if="progress.total > 0" location="top">
          <template #activator="{ props }">
            <div v-bind="props" class="progress-indicator-wrapper">
              <v-progress-circular
                :model-value="showProgress ? Math.round(progress.percent) : 0"
                :size="38"
                :width="4"
                color="primary"
              >
                <small v-show="showProgress">{{ Math.round(progress.percent) }}%</small>
              </v-progress-circular>
            </div>
          </template>
          {{ progress.completed }} / {{ progress.total }} Verfahren abgeschlossen
        </v-tooltip>
      </div>

      <div v-if="!compact" class="action-buttons">
        <v-btn
          :icon="mdiPencil"
          size="small"
          variant="tonal"
          color="info"
          class="edit-btn"
          :aria-label="`${displayName} bearbeiten`"
          @click.stop="$emit('edit', item)"
        >
          <v-icon size="16">{{ mdiPencil }}</v-icon>
          <v-tooltip activator="parent" location="top">Bearbeiten</v-tooltip>
        </v-btn>
        <v-btn
          :icon="mdiDelete"
          size="small"
          variant="tonal"
          color="error"
          class="delete-btn"
          :aria-label="`${displayName} löschen`"
          @click.stop="$emit('delete', item)"
        >
          <v-icon size="16">{{ mdiDelete }}</v-icon>
          <v-tooltip activator="parent" location="top">Löschen</v-tooltip>
        </v-btn>
      </div>

      <!-- Compact screens: actions in a menu so the name keeps its width -->
      <v-menu v-else location="bottom end">
        <template #activator="{ props }">
          <v-btn
            v-bind="props"
            :icon="mdiDotsVertical"
            size="small"
            variant="text"
            :aria-label="`Aktionen für ${displayName}`"
            @click.stop
          />
        </template>
        <v-list density="compact">
          <v-list-item :prepend-icon="mdiPencil" title="Bearbeiten" @click="$emit('edit', item)" />
          <v-list-item
            :prepend-icon="mdiDelete"
            title="Löschen"
            base-color="error"
            @click="$emit('delete', item)"
          />
        </v-list>
      </v-menu>

      <v-icon :icon="mdiChevronDown" size="20" class="expand-indicator" />
    </template>
  </v-list-item>
</template>

<script setup lang="ts">
// List entry of an Untersuchungsstelle.
import { computed } from 'vue';
import { mdiPencil, mdiDelete, mdiMapMarker, mdiDotsVertical, mdiChevronDown } from '@mdi/js';

import type { IUntersuchungsstelle } from '@/types/apiInterfaces';

interface ProcedureProgress {
  completed: number;
  total: number;
  percent: number;
}

interface Props {
  item: IUntersuchungsstelle;
  showProgress: boolean;
  progress: ProcedureProgress;
  expanded: boolean;
  compact: boolean;
  projektName?: string;
}

const props = defineProps<Props>();

defineEmits<{
  'click': [item: IUntersuchungsstelle];
  'edit': [item: IUntersuchungsstelle];
  'delete': [item: IUntersuchungsstelle];
}>();

const displayName = computed(() => {
  return props.item.name || `Untersuchungsstelle ${props.item.id}`;
});

const methodSummary = computed(() => {
  const total = props.progress.total;
  if (total === 0) return 'Keine Verfahren';
  return total === 1 ? '1 Verfahren' : `${total} Verfahren`;
});
</script>

<style scoped>
.structure-list-item {
  cursor: pointer;
  transition: background-color 0.3s cubic-bezier(0.4, 0, 0.2, 1), transform 0.3s cubic-bezier(0.4, 0, 0.2, 1), box-shadow 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  margin: 8px 12px;
  border-radius: 12px;
  border: 1px solid transparent;
  position: relative;
  min-height: 72px;
  padding: 16px 12px;
}

.structure-list-item:hover {
  background-color: rgba(var(--v-theme-primary), 0.06);
  border-color: rgba(var(--v-theme-primary), 0.24);
  transform: translateX(4px);
  box-shadow: 0 10px 24px rgba(0, 0, 0, 0.12);
}

.structure-list-item.is-expanded {
  background-color: rgba(var(--v-theme-primary), 0.06);
  border-color: rgba(var(--v-theme-primary), 0.32);
}

.structure-list-item :deep(.v-list-item__prepend > .v-icon) {
  color: rgb(var(--v-theme-primary));
  opacity: 1;
}

.structure-item-title {
  font-weight: 600;
  font-size: 1rem;
  color: rgb(var(--v-theme-on-surface));
  letter-spacing: -0.01em;
  white-space: normal;
}

.item-title-row {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  max-width: 100%;
}

.item-name-text {
  overflow-wrap: anywhere;
}

.structure-item-subtitle {
  display: flex;
  flex-wrap: wrap;
  column-gap: 8px;
  font-size: 0.875rem;
  color: rgba(var(--v-theme-on-surface), 0.72);
  margin-top: 2px;
  line-height: 1.4;
  white-space: normal;
  -webkit-line-clamp: unset;
}

.subtitle-part::before {
  content: '•';
  margin-right: 8px;
  opacity: 0.6;
}

.action-buttons {
  display: flex;
  gap: 6px;
  opacity: 0.8;
  transition: opacity 0.2s ease;
}

.structure-list-item:hover .action-buttons,
.structure-list-item:focus-within .action-buttons {
  opacity: 1;
}

.action-buttons .v-btn {
  box-shadow: 0 4px 10px rgba(0, 0, 0, 0.12);
  transition: transform 0.2s cubic-bezier(0.4, 0, 0.2, 1), box-shadow 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.action-buttons .v-btn:hover {
  transform: scale(1.05);
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.18);
}

.expand-indicator {
  margin-left: 4px;
  color: rgba(var(--v-theme-on-surface), 0.6);
  transition: transform 0.25s cubic-bezier(0.4, 0, 0.2, 1);
}

.is-expanded .expand-indicator {
  transform: rotate(180deg);
  color: rgb(var(--v-theme-primary));
}

/* Reserve space for progress indicators to avoid layout shift */
.progress-preserve {
  min-width: 54px;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transition: opacity .25s ease;
}

.progress-preserve.is-visible {
  opacity: 1;
}

.progress-indicator-wrapper {
  margin-right: 8px;
}

.progress-indicator-wrapper :deep(small) {
  font-size: 10px !important;
  font-weight: 600;
}

.structure-list-item:focus-visible {
  outline: 2px solid rgb(var(--v-theme-primary));
  outline-offset: 2px;
}

@media (min-width: 960px) {
  .structure-list-item:hover {
    transform: translateX(6px);
  }
}

@media (max-width: 599px) {
  .structure-list-item {
    margin: 6px 4px;
    min-height: 64px;
    padding: 12px 8px;
  }

  .structure-list-item :deep(.v-list-item__prepend) {
    width: 32px;
  }

  .progress-preserve.is-collapsed {
    display: none;
  }

  .progress-preserve {
    min-width: 44px;
  }

  .structure-item-title {
    font-size: 0.95rem;
  }

  .structure-item-subtitle {
    font-size: 0.8rem;
  }
}

@media (hover: none) {
  .structure-list-item:hover {
    transform: none;
    box-shadow: none;
  }
}

@media (prefers-reduced-motion: reduce) {
  .structure-list-item,
  .action-buttons .v-btn,
  .expand-indicator {
    transition: none;
  }

  .structure-list-item:hover,
  .action-buttons .v-btn:hover {
    transform: none;
  }
}
</style>
