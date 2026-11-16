<template>
  <div class="gate-page-shell gate-workspace-page">
    <GateActionButton intent="quiet" :prepend-icon="mdiArrowLeft" @click="backToBauwerk">
      Zur Bauwerksübersicht
    </GateActionButton>
    <GatePageHeader :title="projekt ? `Projekt ${projekt.projektnummer}` : 'Projekt'"
      :subtitle="bauwerk?.bezeichnung ?? 'Bauwerk'" />
    <v-alert v-if="createdMessage" type="success" variant="tonal" closable class="mb-3"
      @click:close="createdMessage = ''">{{ createdMessage }}</v-alert>
    <v-progress-linear v-if="loading" color="primary" indeterminate aria-label="Projekt wird geladen" />
    <v-alert v-if="error" type="error" variant="outlined" class="mb-4">
      {{ error }}
      <GateActionButton intent="secondary" class="mt-3" @click="refresh">Erneut versuchen</GateActionButton>
    </v-alert>
    <v-alert v-else-if="!loading && !projekt" type="warning" variant="tonal">
      Dieses Projekt ist dem Bauwerk nicht zugeordnet oder nicht lokal verfügbar.
    </v-alert>
    <template v-if="projekt && !loading">
      <v-card class="gate-card gate-workspace-info" variant="flat">
        <v-card-title>{{ projekt.name || projekt.bezeichnung || 'Projektangaben' }}</v-card-title>
        <v-card-text>
          <dl class="gate-workspace-facts">
            <div><dt>Projektnummer</dt><dd>{{ projekt.projektnummer }}</dd></div>
            <div><dt>Projektname</dt><dd>{{ projekt.name || 'Nicht angegeben' }}</dd></div>
            <div><dt>Beginn</dt><dd>{{ formatDate(projekt.start) }}</dd></div>
            <div><dt>Ende</dt><dd>{{ formatDate(projekt.ende) }}</dd></div>
            <div v-if="projekt.bezeichnung" class="gate-workspace-facts__wide">
              <dt>Bezeichnung</dt><dd>{{ projekt.bezeichnung }}</dd>
            </div>
          </dl>
        </v-card-text>
      </v-card>
      <WorkspaceStellenList :bauwerk-id="bauwerkId" :stellen="projectStellen"
        :projekte="projekte" :projekt-id="projektId" @created="onStelleCreated" />
    </template>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { mdiArrowLeft } from '@mdi/js';
import { GateActionButton, GatePageHeader } from '@/components/common';
import { useBauwerkWorkspace, workspaceStellenForProjekt } from '@/composables/useBauwerkWorkspace';
import WorkspaceStellenList from '@/components/workspace/WorkspaceStellenList.vue';

const route = useRoute();
const router = useRouter();
const createdMessage = ref('');
const bauwerkId = computed(() => Number(route.params.bauwerkId));
const projektId = computed(() => Number(route.params.projektId));
const { bauwerk, projekte, untersuchungsstellen, loading, error, refresh } = useBauwerkWorkspace(bauwerkId);
const projekt = computed(() => projekte.value.find(p => p.id === projektId.value) ?? null);
const projectStellen = computed(() => workspaceStellenForProjekt(untersuchungsstellen.value, projektId.value));
function formatDate(value: string | null): string {
  if (!value) return 'Nicht angegeben';
  const date = new Date(`${value.slice(0, 10)}T12:00:00`);
  return Number.isNaN(date.getTime()) ? value : date.toLocaleDateString('de-DE');
}
function backToBauwerk() {
  void router.push({ name: 'BauwerkUebersicht', params: { bauwerkId: String(bauwerkId.value) } });
}
function onStelleCreated() {
  createdMessage.value = 'Untersuchungsstelle wurde lokal angelegt.';
  void refresh();
}
</script>
