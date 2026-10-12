<template>
  <v-expand-transition>
    <div v-if="isExpanded">
      <v-card flat class="methods-expansion-card">
        <v-card-title class="methods-title">
          <div class="methods-heading">
            <v-icon :icon="mdiFileDocument" size="18" class="mr-2 methods-icon" />
            <span>Untersuchungsverfahren</span>
            <v-chip size="small" variant="tonal" color="primary" class="methods-count-chip">
              {{ methods.length }}
            </v-chip>
          </div>
          <CreateUntersuchungButton
            :untersuchungsstelle="untersuchungsstelle"
            button-variant="tonal"
            button-color="primary"
            button-size="small"
            button-density="comfortable"
            button-rounded="xl"
            button-class="text-none"
            @created="handleCreated"
          />
        </v-card-title>
        <v-card-text class="methods-content">
          <div v-if="methods.length === 0" class="empty-methods-state">
            Noch keine Untersuchungsverfahren vorhanden.
          </div>
          <div v-else class="methods-groups">
            <section
              v-for="group in methodGroups"
              :key="group.key"
              class="methods-group"
              :class="{ 'methods-group--labor': group.key === 'Laboruntersuchung' }"
            >
              <div class="methods-group-header">
                <h3 class="methods-group-title">{{ group.label }}</h3>
                <v-chip size="x-small" variant="outlined" color="primary">
                  {{ group.methods.length }}
                </v-chip>
              </div>

              <div
                v-for="methodItem in group.methods"
                :key="`${methodItem.type}:${methodItem.id}`"
                class="method-item"
              >
                <v-list-item
                  class="method-list-item"
                  :class="{ 'method-list-item--labor': group.key === 'Laboruntersuchung' }"
                  density="comfortable"
                  @click="$emit('navigate-to-method', methodItem)"
                >
                  <template #prepend>
                    <v-avatar size="24" color="primary" variant="tonal" class="mr-3">
                      <v-icon :icon="mdiFileDocument" size="12" />
                    </v-avatar>
                  </template>
                  <v-list-item-title class="method-title">
                    {{ getMethodDisplayName(methodItem.type) }}
                  </v-list-item-title>
                  <v-list-item-subtitle class="method-subtitle">
                    {{ formatDate(methodItem.datum) }} · ID: {{ methodItem.id }}
                  </v-list-item-subtitle>
                  <template #append>
                    <div class="method-status-wrapper">
                      <v-tooltip location="top">
                        <template #activator="{ props }">
                          <v-avatar
                            v-bind="props"
                            :color="methodItem.abgeschlossen ? 'success' : 'warning'"
                            size="24"
                            class="mr-1 status-avatar"
                            variant="tonal"
                            role="img"
                            :aria-label="methodItem.abgeschlossen ? 'Abgeschlossen' : 'In Bearbeitung'"
                          >
                            <v-icon size="16">{{ methodItem.abgeschlossen ? mdiCheck : mdiProgressClock }}</v-icon>
                          </v-avatar>
                        </template>
                        {{ methodItem.abgeschlossen ? 'Abgeschlossen' : 'In Bearbeitung' }}
                      </v-tooltip>
                      <v-icon :icon="mdiChevronRight" size="18" color="primary" />
                    </div>
                  </template>
                </v-list-item>
              </div>
            </section>
          </div>
        </v-card-text>
      </v-card>
    </div>
  </v-expand-transition>
</template>

<script setup lang="ts">
// The methods are passed in by the parent view.
import { computed } from 'vue';
import { mdiFileDocument, mdiCheck, mdiProgressClock, mdiChevronRight } from '@mdi/js';

import CreateUntersuchungButton from '@/components/bwd/CreateUntersuchungButton.vue';
import type { IUntersuchungsstelle } from '@/types/apiInterfaces';
import { getMethodDisplayName } from '@/utils/UntersuchungsverfahrenRegistry';

export type UntersuchungsverfahrenArt = 'VorOrtUntersuchung' | 'Laboruntersuchung';

export interface UntersuchungsstelleMethodItem {
  id: number;
  type: string;
  category: UntersuchungsverfahrenArt;
  datum: string | null;
  abgeschlossen: boolean | null;
  sort_order: number;
}

interface Props {
  isExpanded: boolean;
  methods: UntersuchungsstelleMethodItem[];
  untersuchungsstelle: IUntersuchungsstelle;
}

interface MethodGroup {
  key: UntersuchungsverfahrenArt;
  label: string;
  methods: UntersuchungsstelleMethodItem[];
}

const props = defineProps<Props>();

const emit = defineEmits<{
  created: [data: any];
  'navigate-to-method': [methodItem: UntersuchungsstelleMethodItem];
}>();

const methodGroups = computed<MethodGroup[]>(() => {
  const groups: MethodGroup[] = [
    { key: 'VorOrtUntersuchung', label: 'Vor Ort Untersuchung', methods: [] },
    { key: 'Laboruntersuchung', label: 'Laboruntersuchung', methods: [] },
  ];
  for (const group of groups) {
    group.methods = props.methods
      .filter((method) => method.category === group.key)
      .sort((a, b) => a.sort_order - b.sort_order || a.id - b.id);
  }
  return groups.filter((group) => group.methods.length > 0);
});

function formatDate(value: string | null): string {
  if (!value) return 'Ohne Datum';
  const [year, month, day] = value.split('-');
  return `${day}.${month}.${year}`;
}

function handleCreated(data: any) {
  emit('created', data);
}
</script>

<style scoped>
.methods-expansion-card {
  margin: 0 16px 16px 48px;
  background: rgb(var(--v-theme-surface)) !important;
  border: 1px solid rgba(var(--v-theme-primary), 0.16);
  border-radius: 12px;
  overflow: hidden;
}

.methods-title {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 8px 12px;
  font-size: 0.95rem;
  font-weight: 600;
  color: rgb(var(--v-theme-on-surface));
  padding: 12px 16px 8px 16px;
  white-space: normal;
}

.methods-heading {
  display: flex;
  align-items: center;
}

.methods-icon {
  color: rgb(var(--v-theme-primary));
}

.methods-count-chip {
  margin-left: 10px;
}

.methods-content {
  padding: 8px 16px 16px 16px;
}

.empty-methods-state {
  padding: 8px 0;
  color: rgba(var(--v-theme-on-surface), 0.72);
  font-size: 0.9rem;
}

.methods-groups {
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.methods-group {
  display: flex;
  flex-direction: column;
}

.methods-group-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
  padding: 0 4px;
}

.methods-group--labor .methods-group-header {
  background: rgba(var(--v-theme-warning), 0.1);
  border: 1px solid rgba(var(--v-theme-warning), 0.24);
  border-radius: 10px;
  padding: 6px 10px;
}

.methods-group-title {
  margin: 0;
  font-size: 0.82rem;
  font-weight: 700;
  letter-spacing: 0.02em;
  text-transform: uppercase;
  color: rgba(var(--v-theme-on-surface), 0.72);
}

.method-item {
  margin-bottom: 6px;
  border-radius: 8px;
  overflow: hidden;
}

.method-list-item {
  border: 1px solid rgba(var(--v-theme-primary), 0.2);
  border-radius: 8px;
  background: rgba(var(--v-theme-background), 0.72);
  cursor: pointer;
  transition: background-color 0.3s cubic-bezier(0.4, 0, 0.2, 1), border-color 0.3s cubic-bezier(0.4, 0, 0.2, 1), transform 0.3s cubic-bezier(0.4, 0, 0.2, 1), box-shadow 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.method-list-item--labor {
  border-color: rgba(var(--v-theme-warning), 0.28);
  background: rgba(var(--v-theme-warning), 0.06);
}

.method-list-item:hover {
  background: rgba(var(--v-theme-primary), 0.1);
  border-color: rgba(var(--v-theme-primary), 0.42);
  transform: translateX(6px) translateY(-1px);
  box-shadow: 0 10px 22px rgba(0, 0, 0, 0.14);
}

.method-list-item--labor:hover {
  background: rgba(var(--v-theme-warning), 0.14);
  border-color: rgba(var(--v-theme-warning), 0.45);
}

.method-status-wrapper {
  display: flex;
  align-items: center;
  gap: 4px;
}

.method-title {
  font-weight: 500;
  font-size: 0.9rem;
  color: rgb(var(--v-theme-on-surface));
  white-space: normal;
}

.method-subtitle {
  font-size: 0.8rem;
  color: rgba(var(--v-theme-on-surface), 0.7);
}

.method-list-item:focus-visible {
  outline: 2px solid rgb(var(--v-theme-primary));
  outline-offset: 2px;
}

@media (max-width: 599px) {
  .methods-expansion-card {
    margin: 0 4px 12px 4px;
  }

  .methods-title,
  .methods-content {
    padding-left: 12px;
    padding-right: 12px;
  }
}

@media (hover: none), (prefers-reduced-motion: reduce) {
  .method-list-item:hover {
    transform: none;
    box-shadow: none;
  }
}
</style>
