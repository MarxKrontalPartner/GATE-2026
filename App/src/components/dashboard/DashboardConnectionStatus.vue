<template>
  <div class="gate-dashboard-connection" aria-label="Verbindungsstatus">
    <v-chip :color="online ? 'success' : 'warning'" variant="tonal" size="large"
      :prepend-icon="online ? mdiWifi : mdiWifiOff" role="status">
      {{ online ? 'Gerät online' : 'Gerät offline' }}
    </v-chip>
    <v-chip :color="serverColor" variant="tonal" size="large"
      :prepend-icon="serverState === 'available' ? mdiServer : mdiServerOff" role="status">
      {{ serverLabel }}
    </v-chip>
  </div>
</template>
<script setup lang="ts">
import { computed } from 'vue';
import { mdiWifi, mdiWifiOff, mdiServer, mdiServerOff } from '@mdi/js';
import type { ServerState } from '@/store/DashboardStore';
const props = defineProps<{ online: boolean; serverState: ServerState }>();
const serverColor = computed(() => props.serverState === 'available' ? 'success' :
  props.serverState === 'checking' || props.serverState === 'unknown' ? 'secondary' : 'warning');
const serverLabel = computed(() => ({
  unknown: 'Serverstatus unbekannt', checking: 'Server wird geprüft',
  available: 'Server erreichbar', unavailable: 'Server nicht erreichbar',
  unauthorized: 'Server: kein Zugriff', offline: 'Server offline',
})[props.serverState]);
</script>
