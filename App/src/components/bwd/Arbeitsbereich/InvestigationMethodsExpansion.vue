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

<style scoped lang="scss" src="./InvestigationMethodsExpansion.styles.scss"></style>
