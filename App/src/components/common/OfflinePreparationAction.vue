<template>
  <div v-if="canPrepareOffline" class="d-flex align-center flex-wrap ga-3 mb-4">
    <GateActionButton :loading="preparing" @click="prepareLocal">
      Daten für Offline-Arbeit vorbereiten
    </GateActionButton>
    <span role="status" aria-live="polite">{{ prepareMessage }}</span>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { isTauri } from '@tauri-apps/api/core';
import { prepareOfflineData } from '@/services/offlinePreparation';
import GateActionButton from './GateActionButton.vue';

const emit = defineEmits<{ prepared: [] }>();
const canPrepareOffline = isTauri();
const preparing = ref(false);
const prepareMessage = ref('');

async function prepareLocal() {
  preparing.value = true;
  prepareMessage.value = '';
  try {
    const summary = await prepareOfflineData();
    prepareMessage.value = `Lokal gespeichert: ${summary.records} Datensätze, ${summary.files} Dateien.`;
    emit('prepared');
  } catch (error) {
    prepareMessage.value = `Vorbereitung fehlgeschlagen: ${String(error)}`;
  } finally {
    preparing.value = false;
  }
}
</script>
