<template>
  <section class="gate-dashboard-bauwerk-list" aria-label="Bauwerkliste">
    <DashboardBauwerkCard v-for="item in bauwerke" :key="`local-${item.id}`"
      :bauwerk="item" :project-names="projectNames(item.projektIds)"
      :busy="busyId === item.id"
      @open="emit('open', $event)" @action="(action, name, id) => emit('action', action, name, id)" />
    <DashboardBauwerkCard v-for="item in available" :key="`server-${item.id}`"
      :bauwerk="item" :project-names="projectNames(item.projektIds)" is-available
      :busy="busyId === item.id"
      @action="(action, name, id) => emit('action', action, name, id)" />
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
  busyId?: number | null;
}>();
const emit = defineEmits<{ open: [id: number]; action: [action: string, name: string, id: number] }>();
</script>
