<template>
    <GenericDetailView>
  
      <template #content>
        <h1>Detailansicht für {{ untersuchung }} - ID: {{ id }} Art: {{ art }}</h1>
        <h2> Untersuchung noch nicht implementiert</h2>
        {{ store.currentUntersuchung }}
      </template>
  
    </GenericDetailView>
  </template>
  
  <script setup lang="ts">
  import GenericDetailView from '@/views/bwd/Untersuchungsverfahren/GenericDetailView.vue'
  import { useUntersuchungenStore } from '@/store/bwd/UntersuchungsverfahrenStore';
  import { computed } from 'vue';
  import { useRoute } from 'vue-router';
  
  const store = useUntersuchungenStore();
  const route = useRoute();
  const id = computed(() => route.params.id as string);
  const untersuchung = computed(() => store.currentUntersuchung?.name || route.params.untersuchung || 'Unbekannt');
  const art = computed(() => route.params.art || 'Unbekannt');
  </script>
