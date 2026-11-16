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

<style scoped lang="scss" src="./StructureListItem.styles.scss"></style>
