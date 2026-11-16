<template>
  <v-card class="gate-card gate-dashboard-bauwerk" :class="{ 'gate-dashboard-bauwerk--recent': recent }" variant="flat">
    <v-card-text class="gate-dashboard-bauwerk__body">
      <div class="gate-dashboard-bauwerk__details">
        <h3 class="gate-dashboard-bauwerk__title">{{ bauwerk.bezeichnung }}</h3>
        <p class="gate-dashboard-bauwerk__meta">
          {{ bauwerk.bauwerksnummer || `BW #${bauwerk.id}` }} · {{ projectNames || 'Keine Projekte' }}
        </p>
        <div class="gate-dashboard-bauwerk__badges">
          <GateStatusChip :status="primaryStatus" size="default" />
          <GateStatusChip v-if="!isAvailable && updateAvailable && primaryStatus !== 'update-available'"
            status="update-available" />
          <v-chip v-if="bauwerk.archived" variant="outlined" size="default">Archiviert</v-chip>
        </div>
        <p v-if="!isAvailable && primaryStatus === 'unsynced' && updateAvailable"
          class="gate-dashboard-bauwerk__warning">
          Lokale Änderungen vorhanden — Serverversion nicht überschreiben.
        </p>
        <p v-if="isAvailable" class="gate-dashboard-bauwerk__meta">
          Nur auf dem Server. Für die Offline-Arbeit muss das Bauwerk heruntergeladen werden.
        </p>
      </div>
      <div class="gate-dashboard-bauwerk__actions">
        <GateActionButton v-if="!isAvailable" intent="secondary" @click="emit('open', bauwerk.id)">
          {{ bauwerk.archived ? 'Ansehen' : 'Öffnen' }}
        </GateActionButton>
        <GateActionButton v-if="actionLabel" :intent="primaryStatus === 'conflict' ? 'danger' : 'primary'"
          @click="emit('action', actionLabel, bauwerk.bezeichnung)">
          {{ actionLabel }}
        </GateActionButton>
      </div>
    </v-card-text>
  </v-card>
</template>
<script setup lang="ts">
import { computed } from 'vue';
import { GateActionButton, GateStatusChip, type GateStatus } from '@/components/common';
import type { DashboardBauwerk, AvailableBauwerk } from '@/repositories/dashboardRepository';
const props = withDefaults(defineProps<{
  bauwerk: DashboardBauwerk | AvailableBauwerk;
  projectNames: string;
  recent?: boolean;
  isAvailable?: boolean;
}>(), { recent: false, isAvailable: false });
const emit = defineEmits<{ open: [id: number]; action: [action: string, name: string] }>();
const primaryStatus = computed<GateStatus>(() => {
  if (props.isAvailable) return 'download-available';
  const item = props.bauwerk as DashboardBauwerk;
  if (item.syncStatus === 'conflict') return 'conflict';
  if (item.syncStatus === 'unsynced') return 'unsynced';
  if (item.serverUpdateAvailable) return 'update-available';
  return 'synced';
});
const updateAvailable = computed(() => !props.isAvailable && 'serverUpdateAvailable' in props.bauwerk && props.bauwerk.serverUpdateAvailable);
const actionLabel = computed(() => ({
  'download-available': 'Herunterladen', 'update-available': 'Aktualisieren',
  unsynced: 'Synchronisieren', conflict: 'Konflikt lösen',
  synced: '', completed: '', 'in-progress': '',
})[primaryStatus.value]);
</script>
