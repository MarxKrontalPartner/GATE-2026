<template>
  <div class="gate-page-shell gate-workspace-page gate-stelle-page">
    <div class="gate-stelle-breadcrumbs">
      <GateActionButton intent="quiet" :prepend-icon="mdiArrowLeft" @click="goBack">
        {{ backLabel }}
      </GateActionButton>
      <GateActionButton intent="quiet" :prepend-icon="mdiFormatListBulleted" @click="goToList">
        Alle Untersuchungsstellen
      </GateActionButton>
    </div>
    <GatePageHeader :title="stelle?.name || 'Untersuchungsstelle'"
      :subtitle="bauwerk ? `${bauwerk.bezeichnung} · Untersuchungsstelle ${stelleId}` : 'Untersuchungsstelle · Offline-Arbeitsbereich'">
      <template #actions>
        <GateActionButton intent="secondary" :prepend-icon="mdiRefresh" :loading="loading" @click="refresh">
          Aktualisieren
        </GateActionButton>
        <GateActionButton intent="primary" :prepend-icon="mdiPencil" :disabled="!stelle || loading" @click="editing = true">
          Bearbeiten
        </GateActionButton>
      </template>
    </GatePageHeader>
    <v-progress-linear v-if="loading" indeterminate color="primary" aria-label="Untersuchungsstelle wird geladen" class="mb-3" />
    <v-alert v-if="error" type="error" variant="outlined" class="mb-3">
      {{ error }}
      <GateActionButton class="mt-3" intent="secondary" @click="refresh">Erneut versuchen</GateActionButton>
    </v-alert>
    <v-alert v-if="saveError" type="error" variant="tonal" class="mb-3">{{ saveError }}</v-alert>
    <v-alert v-if="successMessage" type="success" variant="tonal" class="mb-3">{{ successMessage }}</v-alert>
    <template v-if="stelle && !loading && !error">
      <div class="gate-stelle-summary" aria-label="Untersuchungsstellenübersicht">
        <v-chip size="large" variant="tonal" color="primary" :prepend-icon="mdiMapMarkerOutline">
          {{ projekt ? `${projekt.projektnummer} · ${projekt.name}` : 'Kein Projekt zugeordnet' }}
        </v-chip>
        <v-chip size="large" variant="outlined">{{ methods.length }} Verfahren</v-chip>
        <v-chip size="large" color="success" variant="tonal">{{ completed }} erledigt</v-chip>
      </div>
      <WorkspaceStelleFacts :stelle="stelle" :projekt="projekt" />
      <WorkspaceStelleMethods :stelle="stelle" :methods="methods" @created="onCreated" @open="openMethod" />
      <v-alert type="info" variant="tonal" class="gate-stelle-note">
        Auf dem Tablet werden Änderungen lokal gespeichert. Der Upload zum Server erfolgt später über die Synchronisierung.
      </v-alert>
    </template>
    <WorkspaceStelleEditDialog v-if="stelle" :open="editing" :stelle="stelle" :saving="saving"
      @close="editing = false" @save="saveEdits" />
  </div>
</template>
<script setup lang="ts">
import { computed, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { mdiArrowLeft, mdiFormatListBulleted, mdiMapMarkerOutline, mdiPencil, mdiRefresh } from '@mdi/js';
import { GateActionButton, GatePageHeader } from '@/components/common';
import { useUntersuchungsstelleDetail, type StelleEdits } from '@/composables/bwd/useUntersuchungsstelleDetail';
import type { StelleMethod } from '@/composables/bwd/stelleMethods';
import { getMethodRouteSlug } from '@/utils/UntersuchungsverfahrenRegistry';
import { getErrorMessage } from '@/utils/errors';
import WorkspaceStelleFacts from '@/components/workspace/WorkspaceStelleFacts.vue';
import WorkspaceStelleMethods from '@/components/workspace/WorkspaceStelleMethods.vue';
import WorkspaceStelleEditDialog from '@/components/workspace/WorkspaceStelleEditDialog.vue';

const route = useRoute();
const router = useRouter();
const bauwerkId = computed(() => Number(route.params.bauwerkId));
const stelleId = computed(() => Number(route.params.stelleId));
const { bauwerk, stelle, projekt, methods, completed, loading, saving, error, refresh, save } =
  useUntersuchungsstelleDetail(bauwerkId, stelleId);
const editing = ref(false);
const saveError = ref('');
const successMessage = ref('');
const fromProjekt = computed(() => Number(route.query.projekt) > 0 &&
  projekt.value?.id === Number(route.query.projekt));
const backLabel = computed(() => fromProjekt.value ? 'Zur Projektübersicht' : 'Zur Bauwerksübersicht');
function goBack() {
  if (fromProjekt.value && projekt.value) {
    void router.push({ name: 'BauwerkProjekt', params: {
      bauwerkId: String(bauwerkId.value), projektId: String(projekt.value.id),
    } });
  } else {
    void router.push({ name: 'BauwerkUebersicht', params: { bauwerkId: String(bauwerkId.value) } });
  }
}
function goToList() {
  void router.push({ name: 'Untersuchungsstellen', params: { bauwerkId: String(bauwerkId.value) },
    query: { stelle: String(stelleId.value), ...(fromProjekt.value && projekt.value ? { projekt: String(projekt.value.id) } : {}) } });
}
function openMethod(method: StelleMethod) {
  void router.push({ name: 'UntersuchungsverfahrenDetail', params: { bauwerkId: String(bauwerkId.value),
    art: method.category, untersuchung: getMethodRouteSlug(method.type), id: String(method.id) } });
}
function onCreated(method: StelleMethod) {
  // Open the new investigation immediately—no additional tap is required.
  openMethod(method);
}
async function saveEdits(changes: StelleEdits) {
  saveError.value = '';
  successMessage.value = '';
  try {
    await save(changes);
    editing.value = false;
    successMessage.value = 'Untersuchungsstelle wurde gespeichert.';
  } catch (cause) {
    saveError.value = getErrorMessage(cause, 'Änderungen konnten nicht gespeichert werden.');
  }
}
</script>
