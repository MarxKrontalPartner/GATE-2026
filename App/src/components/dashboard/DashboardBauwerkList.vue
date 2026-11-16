<template>
  <section class="gate-dashboard-bauwerk-list" aria-label="Bauwerkliste">
    <DashboardBauwerkCard v-for="item in bauwerke" :key="`local-${item.id}`"
      :bauwerk="item" :project-names="projectNames(item.projektIds)"
      @open="emit('open', $event)" @action="(action, name) => emit('action', action, name)" />
    <DashboardBauwerkCard v-for="item in available" :key="`server-${item.id}`"
      :bauwerk="item" :project-names="projectNames(item.projektIds)" is-available
      @action="(action, name) => emit('action', action, name)" />
    <v-sheet v-if="!bauwerke.length && !available.length" class="gate-dashboard-empty" rounded="lg" border>
      <p>Keine Bauwerke für die aktuelle Suche oder den ausgewählten Filter.</p>
    </v-sheet>
  </section>
</template>
<script setup lang="ts">
import type { DashboardBauwerk, AvailableBauwerk } from '@/repositories/dashboardRepository';
import DashboardBauwerkCard from './DashboardBauwerkCard.vue';
defineProps<{
  bauwerke: DashboardBauwerk[];
  available: AvailableBauwerk[];
  projectNames: (ids: number[]) => string;
}>();
const emit = defineEmits<{ open: [id: number]; action: [action: string, name: string] }>();
</script>
