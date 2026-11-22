<template>
  <div class="gate-page-shell gate-dashboard-page">
    <GatePageHeader title="Bauwerke" subtitle="MKP · Übersicht und Offline-Arbeit">
      <template #actions>
        <DashboardConnectionStatus :online="dashboard.online" :server-state="dashboard.serverState" />
      </template>
    </GatePageHeader>

    <v-alert v-if="dashboard.error" type="error" variant="outlined" class="mb-4">
      {{ dashboard.error }}
    </v-alert>
    <v-alert v-if="!dashboard.native" type="warning" variant="tonal" class="mb-4">
      Browser-Vorschau: Der lokale SQLite-Status ist nur in der Tauri-App verfügbar.
    </v-alert>
    <FeedbackFeedComponent />

    <section class="gate-dashboard-section" aria-labelledby="gate-status-heading">
      <div class="gate-dashboard-section__heading">
        <div>
          <h2 id="gate-status-heading">Arbeitsübersicht</h2>
          <p>Tippen Sie auf einen Status, um die Liste zu filtern.</p>
        </div>
        <GateActionButton intent="secondary" :loading="dashboard.checking"
          @click="dashboard.checkForUpdates()">Updates prüfen</GateActionButton>
      </div>
      <DashboardStatusOverview :counts="counts" :selected="statusFilter" @select="statusFilter = $event" />
      <p class="gate-dashboard-last-check" role="status">
        Letzte Serverprüfung: {{ lastCheckedLabel }}
      </p>
      <p v-if="dashboard.notice" class="gate-dashboard-last-check" role="alert">
        {{ dashboard.notice }}
      </p>
    </section>

    <section v-if="lastOpened && archiveFilter === 'active'" class="gate-dashboard-section"
      aria-label="Zuletzt geöffnetes Bauwerk">
      <h2 class="gate-dashboard-section__title">Zuletzt geöffnet</h2>
      <DashboardBauwerkCard :bauwerk="lastOpened" :project-names="projectNames(lastOpened.projektIds)"
        recent :busy="downloadingId === lastOpened.id" @open="openBauwerk" @action="onCardAction" />
    </section>

    <section class="gate-dashboard-section" aria-labelledby="gate-bauwerke-heading">
      <div class="gate-dashboard-section__heading">
        <div>
          <h2 id="gate-bauwerke-heading">Bauwerksliste</h2>
          <p>Aktive Bauwerke und archivierte Bauwerke getrennt anzeigen.</p>
        </div>
        <BauwerkCreateCard compact />
      </div>
      <div class="gate-dashboard-filters">
        <div class="gate-dashboard-filters__tabs" role="group" aria-label="Archivfilter">
          <v-btn :aria-pressed="archiveFilter === 'active'" :variant="archiveFilter === 'active' ? 'flat' : 'outlined'"
            color="primary" @click="archiveFilter = 'active'">
            Aktiv ({{ archiveCounts.active }})
          </v-btn>
          <v-btn :aria-pressed="archiveFilter === 'archived'" :variant="archiveFilter === 'archived' ? 'flat' : 'outlined'"
            color="primary" @click="archiveFilter = 'archived'">
            Archiviert ({{ archiveCounts.archived }})
          </v-btn>
        </div>
        <v-text-field v-model="search" label="Bauwerke suchen" aria-label="Bauwerke suchen"
          :prepend-inner-icon="mdiMagnify" variant="outlined" hide-details clearable
          class="gate-dashboard-filters__search" />
        <GateActionButton v-if="search || statusFilter" intent="quiet" @click="clearFilters">
          Filter zurücksetzen
        </GateActionButton>
      </div>
      <DashboardBauwerkList :bauwerke="filteredLocal" :available="filteredServer"
        :project-names="projectNames" :busy-id="downloadingId" @open="openBauwerk" @action="onCardAction" />
      <div class="gate-dashboard-import">
        <OfflinePreparationAction @prepared="onPrepared" />
        <p>Vorbereiten lädt die Serverdaten einmalig herunter; es ist keine Synchronisierung.</p>
      </div>
    </section>
    <v-snackbar v-model="snackbarVisible" :timeout="6000" location="bottom">
      {{ snackbarText }}
      <template #actions>
        <v-btn variant="text" @click="snackbarVisible = false">Schließen</v-btn>
      </template>
    </v-snackbar>
  </div>
</template>
<script setup lang="ts">
import { computed, onMounted, onBeforeUnmount, ref, watch } from 'vue';
import { useRouter } from 'vue-router';
import { mdiMagnify } from '@mdi/js';
import { useDashboardStore } from '@/store/DashboardStore';
import { useBauwerkStore } from '@/store/BauwerkStore';
import { useProjekte } from '@/composables/projekte';
import { useDashboardFilters } from '@/composables/useDashboardFilters';
import { GateActionButton, GatePageHeader } from '@/components/common';
import OfflinePreparationAction from '@/components/common/OfflinePreparationAction.vue';
import DashboardConnectionStatus from '@/components/dashboard/DashboardConnectionStatus.vue';
import DashboardStatusOverview from '@/components/dashboard/DashboardStatusOverview.vue';
import DashboardBauwerkCard from '@/components/dashboard/DashboardBauwerkCard.vue';
import DashboardBauwerkList from '@/components/dashboard/DashboardBauwerkList.vue';
import BauwerkCreateCard from '@/components/bauwerkCards/BauwerkCreateCard.vue';
import FeedbackFeedComponent from '@/components/FeedbackFeedComponent.vue';
import { downloadBauwerk } from '@/services/offlinePreparation';
import { syncAll } from '@/services/syncService';
import { isServerOnline } from '@/services/connectivity';
import { getErrorMessage } from '@/utils/errors';
import { getMethodDisplayName } from '@/utils/UntersuchungsverfahrenRegistry';
import type { KeptRecord } from '@/repositories/localDomainRepository';

const router = useRouter();
const dashboard = useDashboardStore();
const bauwerkStore = useBauwerkStore();
const { projektList, refreshProjektList } = useProjekte();
const { archiveFilter, statusFilter, search, counts, archiveCounts, filteredLocal,
  filteredServer, lastOpened, clearFilters } = useDashboardFilters();
const snackbarVisible = ref(false);
const snackbarText = ref('');
const lastCheckedLabel = computed(() => dashboard.lastServerCheck
  ? new Date(dashboard.lastServerCheck).toLocaleString('de-DE') : 'Noch nicht geprüft');
const projectNames = (ids: number[]) => {
  const labels = (projektList.value ?? []).filter(p => ids.includes(p.id)).map(p => p.projektnummer);
  return labels.join(', ');
};

async function openBauwerk(id: number) {
  try { await dashboard.markOpened(id); }
  catch (cause) { console.warn('Last-opened timestamp not saved', cause); }
  await router.push({ name: 'BauwerkUebersicht', params: { bauwerkId: String(id) } });
}
const downloadingId = ref<number | null>(null);
async function onCardAction(action: string, name: string, id: number) {
  if (action === 'Synchronisieren' || action === 'Aktualisieren') return synchronize(name, id);
  if (action !== 'Herunterladen') return showActionNotice(action, name);
  if (downloadingId.value !== null) return;
  downloadingId.value = id;
  try {
    if (!(await isServerOnline(true))) throw new Error('Keine Verbindung zum Server.');
    const { records, files, kept } = await downloadBauwerk(id);
    snackbarText.value = `„${name}“ wurde heruntergeladen (${records} Datensätze, ${files} Dateien) und ist jetzt offline verfügbar.`
      + keptNotice(kept);
    await onPrepared();
  } catch (cause) {
    snackbarText.value = `Herunterladen von „${name}“ fehlgeschlagen: ${getErrorMessage(cause, 'Unbekannter Fehler')}`;
  } finally {
    downloadingId.value = null;
    snackbarVisible.value = true;
  }
}
/** Sends the offline changes (client_sync_changes) to the server. */
async function synchronize(name: string, id: number) {
  if (downloadingId.value !== null) return;
  downloadingId.value = id;
  try {
    if (!(await isServerOnline(true))) throw new Error('Keine Verbindung zum Server.');
    // 1. Send the offline changes.
    const results = await syncAll();
    const sent = results.reduce((n, r) => n + r.sent, 0);
    const created = results.reduce((n, r) => n + r.created, 0);
    const conflicts = results.reduce((n, r) => n + r.conflicts, 0);
    const failed = results.some((r) => r.failed);
    // 2. Download everything else of this Bauwerk from the server. Rows with
    //    unsent changes or a conflict are kept as they are and listed.
    const { records, kept } = await downloadBauwerk(id);
    const parts = [
      ...(created ? [`${created} neue Messung(en) angelegt`] : []),
      !sent ? (created ? 'keine weiteren Änderungen' : 'keine offline Änderungen')
        : failed ? 'einige Änderungen konnten nicht übertragen werden'
        : `${sent - conflicts} Messung(en) übertragen`,
      ...(conflicts ? [`${conflicts} Konflikt(e) – auf dem Server inzwischen geändert`] : []),
      `${records} Datensätze vom Server aktualisiert`,
    ];
    // At least one conflict: the Bauwerk is not synchronized but in conflict.
    snackbarText.value = (conflicts ? `„${name}“: Konflikt – nicht vollständig synchronisiert: `
      : `„${name}“ synchronisiert: `) + `${parts.join(', ')}.` + keptNotice(kept);
    await dashboard.loadLocal();
  } catch (cause) {
    snackbarText.value = `Synchronisieren von „${name}“ fehlgeschlagen: ${getErrorMessage(cause, 'Unbekannter Fehler')}`;
  } finally {
    downloadingId.value = null;
    snackbarVisible.value = true;
  }
}
/** Lists the rows a download did not overwrite, e.g. " Nicht überschrieben: Rückprallhammer #1 (…)." */
function keptNotice(kept: Array<KeptRecord & { resource: string }>): string {
  if (!kept.length) return '';
  const rows = kept.map((row) => `${getMethodDisplayName(row.resource)} #${row.id} (` +
    (row.reason === 'changes' ? 'lokale Änderungen, noch nicht synchronisiert' : 'Konflikt') + ')');
  return ` Nicht überschrieben: ${rows.join(', ')}.`;
}
function showActionNotice(action: string, name: string) {
  snackbarText.value = `${action} für „${name}“ ist noch nicht implementiert. ` +
    'Nutzen Sie bis dahin „Daten für Offline-Arbeit vorbereiten“ für den bestehenden Gesamtdownload.';
  snackbarVisible.value = true;
}
async function onPrepared() {
  await Promise.allSettled([dashboard.loadLocal(), bauwerkStore.setBauwerkList(), refreshProjektList()]);
  await dashboard.checkForUpdates();
}
function onConnectionChange() {
  dashboard.online = navigator.onLine;
  void dashboard.checkServerAvailability();
}
function onFocus() { void dashboard.checkServerAvailability(); }
let timer: ReturnType<typeof setInterval> | undefined;
onMounted(() => {
  // Register monitoring immediately, even when the server catalog request
  // is slow or unavailable. Django reachability and browser Wi-Fi are separate.
  window.addEventListener('online', onConnectionChange);
  window.addEventListener('offline', onConnectionChange);
  window.addEventListener('focus', onFocus);
  timer = setInterval(() => { void dashboard.checkServerAvailability(); }, 15_000);

  void dashboard.checkServerAvailability();
  void Promise.allSettled([
    dashboard.loadLocal(), bauwerkStore.setBauwerkList(), refreshProjektList(),
  ]);
  // Never await the catalog request before starting the connectivity monitor.
  void dashboard.checkForUpdates();
});
onBeforeUnmount(() => {
  window.removeEventListener('online', onConnectionChange);
  window.removeEventListener('offline', onConnectionChange);
  window.removeEventListener('focus', onFocus);
  if (timer) clearInterval(timer);
});
watch(() => bauwerkStore.bauwerke, () => { void dashboard.loadLocal(); });
</script>
