<template>
  <div class="gate-page-shell gate-workspace-page">
    <GateActionButton intent="quiet" :prepend-icon="mdiArrowLeft" @click="router.push({ name: 'bauwerkswahl' })">
      Zur Bauwerksliste
    </GateActionButton>
    <GatePageHeader :title="bauwerk?.bezeichnung ?? 'Bauwerk'" subtitle="Bauwerksübersicht · Offline-Daten">
      <template #actions>
        <GateActionButton intent="primary" :prepend-icon="mdiArrowRight" @click="openWorkspace">
          Arbeitsbereich öffnen
        </GateActionButton>
      </template>
    </GatePageHeader>
    <v-progress-linear v-if="loading" color="primary" indeterminate aria-label="Bauwerk wird geladen" />
    <v-alert v-if="error" type="error" variant="outlined" class="mb-4">
      {{ error }}
      <GateActionButton intent="secondary" class="mt-3" @click="refresh">Erneut versuchen</GateActionButton>
    </v-alert>
    <template v-if="bauwerk && !loading">
      <div class="gate-workspace-status" aria-label="Bauwerkstatus">
        <GateStatusChip v-if="syncStatus" :status="syncStatus" size="large" />
        <GateStatusChip v-if="serverUpdateAvailable && syncStatus !== 'update-available'" status="update-available" size="large" />
        <v-chip v-if="bauwerk.archived" variant="outlined" size="large">Archiviert</v-chip>
        <v-chip variant="tonal" size="large">{{ projekte.length }} Projekte</v-chip>
        <v-chip variant="tonal" size="large">{{ stellenAnzahl }} Untersuchungsstellen</v-chip>
      </div>
      <WorkspaceBauwerkInfo :bauwerk="bauwerk" />
      <section class="gate-workspace-section" aria-labelledby="gate-projects-heading">
        <div class="gate-workspace-section__heading">
          <div>
            <h2 id="gate-projects-heading">Zugeordnete Projekte ({{ projekte.length }})</h2>
            <p>Ein Bauwerk kann mehreren Projekten zugeordnet sein.</p>
          </div>
        </div>
        <v-alert v-if="projekte.length === 0" variant="tonal" type="info">
          Diesem Bauwerk sind noch keine heruntergeladenen Projekte zugeordnet.
        </v-alert>
        <div v-else class="gate-workspace-project-grid">
          <WorkspaceProjectCard v-for="projekt in projekte" :key="projekt.id" :projekt="projekt"
            :stelle-count="stellenFuerProjekt(projekt.id).length" @open="openProject" />
        </div>
      </section>
      <WorkspaceStellenList :bauwerk-id="bauwerkId" :stellen="untersuchungsstellen" :projekte="projekte" />
    </template>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { mdiArrowLeft, mdiArrowRight } from '@mdi/js';
import { useBauwerkWorkspace } from '@/composables/useBauwerkWorkspace';
import { useDashboardStore } from '@/store/DashboardStore';
import { GateActionButton, GatePageHeader, GateStatusChip, type GateStatus } from '@/components/common';
import WorkspaceBauwerkInfo from '@/components/workspace/WorkspaceBauwerkInfo.vue';
import WorkspaceProjectCard from '@/components/workspace/WorkspaceProjectCard.vue';
import WorkspaceStellenList from '@/components/workspace/WorkspaceStellenList.vue';

const route = useRoute();
const router = useRouter();
const dashboard = useDashboardStore();
const bauwerkId = computed(() => Number(route.params.bauwerkId));
const { bauwerk, projekte, untersuchungsstellen, stellenAnzahl, stellenFuerProjekt,
  loading, error, refresh } = useBauwerkWorkspace(bauwerkId);
const dashboardEntry = computed(() => dashboard.bauwerke.find(item => item.id === bauwerkId.value));
const serverUpdateAvailable = computed(() => dashboardEntry.value?.serverUpdateAvailable ?? false);
const syncStatus = computed<GateStatus | null>(() => {
  const item = dashboardEntry.value;
  if (!item) return null;
  if (item.syncStatus !== 'synced') return item.syncStatus;
  return item.serverUpdateAvailable ? 'update-available' : 'synced';
});
onMounted(() => { void dashboard.loadLocal(); });
function openWorkspace() {
  void router.push({ name: 'Untersuchungsstellen', params: { bauwerkId: String(bauwerkId.value) } });
}
function openProject(projectId: number) {
  void router.push({ name: 'BauwerkProjekt', params: { bauwerkId: String(bauwerkId.value), projektId: String(projectId) } });
}
</script>
