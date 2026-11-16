<template>
  <div class="gate-dashboard-status-grid" aria-label="Nach Synchronisationsstatus filtern">
    <v-btn v-for="item in statusItems" :key="item.key" class="gate-dashboard-status-tile"
      :class="{ 'gate-dashboard-status-tile--selected': selected === item.key }"
      :aria-pressed="selected === item.key" variant="outlined"
      @click="emit('select', selected === item.key ? null : item.key)">
      <span class="gate-dashboard-status-tile__count">{{ counts[item.key] }}</span>
      <span class="gate-dashboard-status-tile__label">{{ item.label }}</span>
    </v-btn>
  </div>
</template>
<script setup lang="ts">
import type { StatusFilter } from './statusTypes';
defineProps<{ counts: Record<StatusFilter, number>; selected: StatusFilter | null }>();
const emit = defineEmits<{ select: [next: StatusFilter | null] }>();
const statusItems: { key: StatusFilter; label: string }[] = [
  { key: 'synced', label: 'Synchronisiert' },
  { key: 'unsynced', label: 'Nicht synchronisiert' },
  { key: 'conflict', label: 'Konflikte' },
  { key: 'updates', label: 'Update verfügbar' },
  { key: 'available', label: 'Download verfügbar' },
];
</script>
