<template>
  <SideDrawer>
    <!-- Back to Bauwerkswahl -->
    <NavLink :to="{ name: 'bauwerkswahl' }" :prepend-icon="mdiArrowLeft">Bauwerkswahl</NavLink>

    <v-divider class="my-2"></v-divider>

    <v-list-subheader v-if="bauwerkStore.selectedBauwerk">
      {{ bauwerkStore.selectedBauwerk.bezeichnung }}
    </v-list-subheader>

    <!-- The drawer can be rendered before the initial navigation resolved the route params. -->
    <NavCategory v-if="bauwerkId" title="Bauwerksdiagnostik" :prepend-icon="mdiMagnifyScan" value="bwd">
      <NavLink :to="{ name: 'Untersuchungsstellen', params: { bauwerkId } }" :prepend-icon="mdiMapMarker">
        Untersuchungsstellen
      </NavLink>
      <NavLink :to="{ name: 'VorOrtUntersuchung', params: { bauwerkId } }" :prepend-icon="mdiClipboardTextOutline">
        Vor-Ort-Untersuchungen
      </NavLink>
      <NavLink :to="{ name: 'Laboruntersuchung', params: { bauwerkId } }" :prepend-icon="mdiFlask">
        Laboruntersuchungen
      </NavLink>
    </NavCategory>
  </SideDrawer>
</template>

<script setup lang="ts">
import { mdiArrowLeft, mdiClipboardTextOutline, mdiFlask, mdiMagnifyScan, mdiMapMarker } from '@mdi/js';
import { computed } from 'vue';
import { useRoute } from 'vue-router';

import { NavCategory, NavLink, SideDrawer } from '@/components/sidedrawer';
import { useBauwerkStore } from '@/store/BauwerkStore';

const bauwerkStore = useBauwerkStore();
const route = useRoute();
const bauwerkId = computed(() => route.params.bauwerkId?.toString() ?? '');
</script>
