<template>
  <v-chip
    :color="display.color"
    variant="tonal"
    :prepend-icon="display.icon"
    :size="size"
    label
    class="gate-status-chip font-weight-medium"
  >{{ display.label }}</v-chip>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { mdiCheckCircleOutline, mdiProgressClock, mdiAlertCircleOutline, mdiRefresh, mdiDownloadOutline, mdiClockOutline } from '@mdi/js';

import type { GateStatus } from './statusTypes';

const props = withDefaults(defineProps<{ status: GateStatus; size?: 'x-small' | 'small' | 'default' | 'large' }>(), { size: 'default' });
const definitions: Record<GateStatus, { label: string; color: string; icon: string }> = {
  synced: { label: 'Synchronisiert', color: 'success', icon: mdiCheckCircleOutline },
  unsynced: { label: 'Nicht synchronisiert', color: 'warning', icon: mdiClockOutline },
  conflict: { label: 'Konflikt', color: 'error', icon: mdiAlertCircleOutline },
  'update-available': { label: 'Update verfügbar', color: 'info', icon: mdiRefresh },
  'download-available': { label: 'Download verfügbar', color: 'secondary', icon: mdiDownloadOutline },
  completed: { label: 'Erledigt', color: 'success', icon: mdiCheckCircleOutline },
  'in-progress': { label: 'In Arbeit', color: 'warning', icon: mdiProgressClock },
};
const display = computed(() => definitions[props.status]);
</script>
