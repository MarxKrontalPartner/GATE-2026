<template>
  <div class="gate-page-shell gate-chloridanalyse-page gate-form">
    <GateActionButton intent="quiet" :prepend-icon="mdiArrowLeft" @click="goBack">
      Zur Untersuchungsstelle
    </GateActionButton>
    <GatePageHeader title="Chloridanalyse"
      :subtitle="`Vor-Ort-Untersuchung · ID ${currentUntersuchung.id || '–'} · ${localMode() ? 'lokale SQLite-Datenbank' : 'Django-Server'}`">
      <template #actions>
        <GateActionButton v-if="!editing" intent="primary" :prepend-icon="mdiPencil" @click="startEditing">Bearbeiten</GateActionButton>
        <template v-else>
          <GateActionButton intent="quiet" :disabled="saving" @click="cancelEditing">Abbrechen</GateActionButton>
          <GateActionButton intent="primary" :prepend-icon="mdiContentSaveAll"
            :loading="saving" :disabled="saving" @click="saveChanges">Speichern</GateActionButton>
        </template>
      </template>
    </GatePageHeader>

    <div class="gate-chloridanalyse-page__summary" aria-label="Chloridanalyse Übersicht">
      <GateStatusChip :status="draft.abgeschlossen ? 'completed' : 'in-progress'" />
      <v-chip size="large" variant="outlined">Untersuchungsstelle {{ currentUntersuchung.untersuchungsstelle || '–' }}</v-chip>
      <v-chip size="large" variant="outlined">{{ selectedProbe ? selectedProbe.probenbezeichnung : 'Keine Probe' }}</v-chip>
    </div>
    <v-alert v-if="error" type="error" variant="tonal" role="alert" class="mb-4">{{ error }}</v-alert>
    <v-alert v-if="success" type="success" variant="tonal" role="status" class="mb-4">{{ success }}</v-alert>
    <v-alert v-if="editing" type="info" variant="tonal" class="mb-4">
      Messwerte werden mit „Speichern“ übernommen und sind auch offline verfügbar.
      Änderungen an Bearbeiter-Zuordnungen, neue Proben sowie neue Dateien benötigen eine Verbindung zum Server.
    </v-alert>
    <v-alert v-if="editing && !canEditServerRelations" type="warning" variant="tonal" class="mb-4">
      Offline-Modus: Chloridwerte und die Zuordnung einer bereits gespeicherten Probe können lokal bearbeitet werden.
      Neue Proben, Bearbeiter-Zuordnungen, Bilder und Prüfberichte sind erst bei Serververbindung möglich.
    </v-alert>
    <v-alert v-if="editing && unsyncedLocalRecord" type="info" variant="tonal" class="mb-4">
      Diese Untersuchung wurde offline neu angelegt. Dateien können erst nach erfolgreicher Synchronisierung hinzugefügt werden.
    </v-alert>

    <div class="gate-chloridanalyse-page__sections">
      <InvestigationBasicsFields :draft="draft" :editing="editing" :people="people"
        :allow-personnel-edit="canEditServerRelations && peopleWarning === '' && people.length > 0"
        :people-loading="loadingPeople" :people-warning="peopleWarning"
        show-people-refresh @refresh-people="refreshPeople" />

      <GateFormSection title="Probe zuordnen" description="Probe und zugehörigen Bohrkern auswählen" :icon="mdiTestTube">
        <v-progress-linear v-if="loadingProbes" indeterminate color="primary" class="mb-3" />
        <v-alert v-if="probeError" type="error" variant="tonal" class="mb-3">{{ probeError }}</v-alert>
        <v-row align="center">
          <v-col cols="12" sm="8">
            <v-autocomplete v-model="draft.probe" :items="probes" item-title="probenbezeichnung"
              item-value="id" label="Probe" variant="outlined" clearable :readonly="!editing"
              :loading="loadingProbes" no-data-text="Noch keine Proben vorhanden"
              hint="Es werden Proben aus dem aktuellen Projekt und Bauwerk angeboten." persistent-hint />
          </v-col>
          <v-col cols="12" sm="4">
            <GateActionButton v-if="editing" block intent="secondary" :prepend-icon="mdiPlus" :disabled="!canEditServerRelations"
              @click="probeDialog = true">Neue Probe</GateActionButton>
          </v-col>
        </v-row>
        <v-card v-if="selectedProbe" variant="outlined" class="gate-chloridanalyse-page__probe-card">
          <v-card-text>
            <strong>{{ selectedProbe.probenbezeichnung }}</strong>
            <div v-if="selectedProbe.entnahmetiefeVon != null || selectedProbe.entnahmetiefeBis != null">
              Entnahmetiefe: {{ selectedProbe.entnahmetiefeVon ?? '–' }} – {{ selectedProbe.entnahmetiefeBis ?? '–' }} mm
            </div>
            <div v-if="selectedProbe.bemerkung">{{ selectedProbe.bemerkung }}</div>
            <div>Bohrkern: {{ selectedCore?.name || (selectedProbe.bohrkern ? `ID ${selectedProbe.bohrkern}` : 'Nicht zugeordnet') }}</div>
          </v-card-text>
        </v-card>
        <v-alert v-else type="info" variant="tonal" class="mt-3">Noch keine Probe zugeordnet.</v-alert>
      </GateFormSection>

      <GateFormSection title="Analyse" description="Laborstelle, Verfahren und ermittelte Chloridwerte" :icon="mdiFlaskOutline">
        <v-row>
          <v-col cols="12" sm="6">
            <v-text-field v-model="draft.pruefstelle" label="Prüfstelle" variant="outlined"
              :readonly="!editing" counter="100" maxlength="100" />
          </v-col>
          <v-col cols="12" sm="6">
            <ChoiceSelect v-model="draft.analyseverfahren" :items="analyseverfahrenOptions"
              label="Analyseverfahren" variant="outlined" :readonly="!editing" />
          </v-col>
          <v-col cols="12" sm="6">
            <NumberField v-model="draft.chloridgehaltZement" label="Chloridgehalt (bezogen auf Zement)"
              suffix="M.-%" hint="Massenprozent bezogen auf den Zementgehalt" persistent-hint
              min="0" variant="outlined" :readonly="!editing" />
          </v-col>
          <v-col cols="12" sm="6">
            <NumberField v-model="draft.zementgehaltAnnahme" label="Zementgehalt (Annahme)"
              suffix="kg/m³" min="0" variant="outlined" :readonly="!editing" />
          </v-col>
          <v-col cols="12" sm="6">
            <v-select v-model="draft.grenzwertUeberschritten" :items="grenzwertOptions"
              label="Grenzwert überschritten" variant="outlined" :readonly="!editing"
              hint="Unbekannt wählen, solange keine Bewertung vorliegt." persistent-hint />
          </v-col>
        </v-row>
      </GateFormSection>

      <GateFormSection title="Prüfberichte" description="Dateien werden lokal gespeichert und mit dieser Chloridanalyse verknüpft" :icon="mdiFileDocumentOutline">
        <MultiFileField v-model="reports" label="Prüfberichte" :enabled="editing && canUploadFiles && !uploadingReport"
          :on-upload-file="uploadReport" :on-delete-file="detachReport" />
        <v-alert v-if="reportError" type="error" variant="tonal" class="mt-3">{{ reportError }}</v-alert>
        <v-alert type="info" variant="tonal" class="mt-3">
          Prüfberichte benötigen eine Serververbindung und eine synchronisierte Untersuchungs-ID.
          Beim Hinzufügen oder Entfernen werden Verknüpfungen sofort gespeichert; „Abbrechen“ verwirft nur ungespeicherte Messwerte.
        </v-alert>
      </GateFormSection>

      <GateFormSection title="Bilder" description="Fotos bei bestehender Serververbindung hinzufügen oder als Titelbild markieren" :icon="mdiImage">
        <UntersuchungsImageCard :images="Array.isArray(currentUntersuchung.bilder) ? currentUntersuchung.bilder : []"
          :enabled="editing && canUploadFiles" />
      </GateFormSection>
      <SyncConflictNotice table="bauwerksdiagnostik_chloridanalyse"
        :record-id="currentUntersuchung.id" :version="currentUntersuchung" />
    </div>

    <div class="gate-chloridanalyse-page__footer">
      <GateActionButton v-if="editing" intent="danger" :prepend-icon="mdiDelete" :disabled="saving || !canDeleteInvestigation"
        @click="deleteDialog = true">Untersuchung löschen</GateActionButton>
      <div class="gate-chloridanalyse-page__footer-actions">
        <GateActionButton intent="quiet" :disabled="saving" @click="editing ? cancelEditing() : goBack()">
          {{ editing ? 'Abbrechen' : 'Zurück' }}
        </GateActionButton>
        <GateActionButton v-if="editing" intent="primary" :prepend-icon="mdiContentSaveAll"
          :loading="saving" :disabled="saving" @click="saveChanges">Änderungen speichern</GateActionButton>
      </div>
    </div>

    <v-dialog v-model="probeDialog" max-width="620" persistent>
      <v-card class="gate-form">
        <v-card-title>Neue Probe anlegen</v-card-title>
        <v-card-text>
          <v-text-field v-model="newProbe.probenbezeichnung" label="Probenbezeichnung *" variant="outlined" maxlength="100" />
          <v-row>
            <v-col cols="12" sm="6"><NumberField v-model="newProbe.entnahmetiefeVon" label="Entnahmetiefe von" suffix="mm" min="0" variant="outlined" /></v-col>
            <v-col cols="12" sm="6"><NumberField v-model="newProbe.entnahmetiefeBis" label="Entnahmetiefe bis" suffix="mm" min="0" variant="outlined" /></v-col>
          </v-row>
          <v-textarea v-model="newProbe.bemerkung" label="Bemerkung" variant="outlined" rows="2" auto-grow />
          <v-alert v-if="newProbeError" type="error" variant="tonal">{{ newProbeError }}</v-alert>
        </v-card-text>
        <v-card-actions class="gate-chloridanalyse-page__dialog-actions">
          <GateActionButton intent="quiet" :disabled="creatingProbe" @click="probeDialog = false">Abbrechen</GateActionButton>
          <GateActionButton intent="primary" :prepend-icon="mdiPlus" :loading="creatingProbe"
            :disabled="creatingProbe || !newProbe.probenbezeichnung.trim()" @click="createProbe">Probe anlegen</GateActionButton>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-dialog v-model="deleteDialog" max-width="540">
      <v-card class="gate-investigation-confirm">
        <v-card-title>Chloridanalyse löschen?</v-card-title>
        <v-card-text>Die Untersuchung wird gelöscht. Die Probe bleibt erhalten; dies kann nicht rückgängig gemacht werden.</v-card-text>
        <v-card-actions>
          <GateActionButton intent="quiet" @click="deleteDialog = false">Abbrechen</GateActionButton>
          <GateActionButton intent="danger" :loading="deleting" :disabled="deleting" @click="deleteInvestigation">Löschen</GateActionButton>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { storeToRefs } from 'pinia';
import { mdiArrowLeft, mdiPencil, mdiContentSaveAll, mdiTestTube, mdiFlaskOutline,
  mdiFileDocumentOutline, mdiImage, mdiPlus, mdiDelete } from '@mdi/js';
import { GateActionButton, GatePageHeader, GateStatusChip } from '@/components/common';
import GateFormSection from '@/components/investigations/GateFormSection.vue';
import InvestigationBasicsFields from '@/components/investigations/InvestigationBasicsFields.vue';
import NumberField from '@/components/bwd/NumberField.vue';
import ChoiceSelect, { type ChoiceOption } from '@/components/bwd/ChoiceSelect.vue';
import MultiFileField from '@/components/bwd/MultiFileField.vue';
import UntersuchungsImageCard from '@/components/bwd/media/UntersuchungsImageCard.vue';
import SyncConflictNotice from '@/components/bwd/SyncConflictNotice.vue';
import { useUntersuchungenStore } from '@/store/bwd/UntersuchungsverfahrenStore';
import { apiConnector } from '@/utils/ApiConnector';
import { localMode } from '@/repositories/localDomainRepository';
import { isServerOnline } from '@/services/connectivity';
import { getErrorMessage } from '@/utils/errors';
import { chloridanalysePatch, chloridanalysePersistenceDifferences, toChloridanalyseDraft,
  validateChloridanalyseDraft } from '@/types/chloridanalyse';
import type { ChloridanalyseDraft } from '@/types/chloridanalyse';
import type { IPerson } from '@/types/apiInterfaces';
import { loadPersonReferences } from '@/services/personReferenceData';
import type { Probe, Bohrkern } from '@/composables/bwd/useProbe';

const store = useUntersuchungenStore();
const { currentUntersuchung } = storeToRefs(store);
const route = useRoute();
const router = useRouter();
const draft = ref<ChloridanalyseDraft>(toChloridanalyseDraft({}));
const people = ref<IPerson[]>([]);
const loadingPeople = ref(false);
const peopleWarning = ref('');
async function refreshPeople() {
  loadingPeople.value = true;
  try {
    const result = await loadPersonReferences();
    people.value = result.people;
    peopleWarning.value = result.warning;
  } catch (cause) {
    peopleWarning.value = getErrorMessage(cause, 'Bearbeiter konnten nicht geladen werden.');
  } finally { loadingPeople.value = false; }
}

const probes = ref<Probe[]>([]);
const selectedCore = ref<Bohrkern | null>(null);
const loadingProbes = ref(false);
const probeError = ref('');
const probeDialog = ref(false);
const creatingProbe = ref(false);
const newProbeError = ref('');
const newProbe = ref({ probenbezeichnung: '', bemerkung: '', entnahmetiefeVon: null as number | null, entnahmetiefeBis: null as number | null });
const editing = ref(false);
const saving = ref(false);
const deleting = ref(false);
const deleteDialog = ref(false);
const uploadingReport = ref(false);
const reports = ref<number[]>([]);
const reportError = ref('');
const error = ref('');
const success = ref('');
// In the authentication-enabled App, server writes mirror to SQLite and offline writes use
// the local SeaORM repository with a sync outbox. Probe/Media have no offline sync contract.
const serverAvailable = ref(!localMode());
const canEditServerRelations = computed(() => !localMode() || serverAvailable.value);
const unsyncedLocalRecord = computed(() => Number(currentUntersuchung.value?.id) < 0);
const canUploadFiles = computed(() => canEditServerRelations.value && !unsyncedLocalRecord.value);
const canDeleteInvestigation = computed(() => canEditServerRelations.value && !unsyncedLocalRecord.value);
let statusPoll: ReturnType<typeof setInterval> | undefined;
async function refreshConnection(): Promise<void> {
  serverAvailable.value = !localMode() || await isServerOnline();
}
onMounted(() => {
  void refreshConnection();
  statusPoll = setInterval(() => void refreshConnection(), 15000);
});
onUnmounted(() => { if (statusPoll) clearInterval(statusPoll); });
const analyseverfahrenOptions: ChoiceOption[] = [
  { value: 'titration', title: 'Potentiometrische Titration' },
  { value: 'photometrie', title: 'Photometrie' },
  { value: 'schnelltest', title: 'Schnelltest vor Ort' },
];
const grenzwertOptions = [
  { title: 'Noch nicht bewertet', value: null },
  { title: 'Ja', value: true },
  { title: 'Nein', value: false },
];
const selectedProbe = computed(() => probes.value.find(p => p.id === draft.value.probe) ?? null);

watch(() => currentUntersuchung.value, record => {
  if (!record?.id || editing.value) return;
  draft.value = toChloridanalyseDraft(record);
  reports.value = [...draft.value.pruefberichte];
}, { immediate: true });

// Reference data uses ApiConnector, which reads local SQLite when running in Tauri.
void refreshPeople();
let probeLoad = 0;
watch(() => [currentUntersuchung.value?.projekt, currentUntersuchung.value?.bauwerk], async ([projekt, bauwerk]) => {
  const token = ++probeLoad;
  const projectId = Number(projekt);
  const buildingId = Number(bauwerk);
  if (![projectId, buildingId].every(id => Number.isSafeInteger(id) && id > 0)) return;
  loadingProbes.value = true;
  probeError.value = '';
  try {
    const items = await apiConnector.getXResourceList<Probe>('bwd', 'Probe', String(projectId), String(buildingId));
    if (token === probeLoad) probes.value = items;
  } catch (cause) {
    if (token === probeLoad) probeError.value = getErrorMessage(cause, 'Proben konnten nicht geladen werden.');
  } finally { if (token === probeLoad) loadingProbes.value = false; }
}, { immediate: true });
let coreLoad = 0;
watch(() => selectedProbe.value?.bohrkern, async value => {
  const token = ++coreLoad;
  selectedCore.value = null;
  if (!value) return;
  try {
    const core = await apiConnector.getXResource<Bohrkern>('bwd', 'Bohrkern', Number(value));
    if (token === coreLoad) selectedCore.value = core;
  } catch { /* A missing core must not block the chloride form. */ }
}, { immediate: true });

function startEditing() {
  void refreshPeople();
  draft.value = toChloridanalyseDraft(currentUntersuchung.value);
  reports.value = [...draft.value.pruefberichte];
  error.value = ''; success.value = ''; reportError.value = '';
  editing.value = true;
}
function cancelEditing() {
  editing.value = false;
  draft.value = toChloridanalyseDraft(currentUntersuchung.value);
  reports.value = [...draft.value.pruefberichte];
  error.value = ''; reportError.value = '';
}
function goBack() {
  const bauwerkId = Number(route.params.bauwerkId);
  const stelleId = Number(currentUntersuchung.value.untersuchungsstelle);
  if (Number.isSafeInteger(stelleId) && stelleId > 0 && Number.isSafeInteger(bauwerkId) && bauwerkId > 0) {
    void router.push({ name: 'UntersuchungsstelleDetail', params: { bauwerkId: String(bauwerkId), stelleId: String(stelleId) } });
  } else {
    void router.push({ name: 'BauwerkUebersicht', params: { bauwerkId: String(bauwerkId) } });
  }
}
async function freshRecord(id: number): Promise<Record<string, unknown>> {
  return apiConnector.BWD.fetchUntersuchungDetailData(id, 'chloridanalyse', 'VorOrtUntersuchung') as Promise<Record<string, unknown>>;
}
async function saveChanges() {
  if (!editing.value || saving.value) return;
  error.value = ''; success.value = '';
  const message = validateChloridanalyseDraft(draft.value);
  if (message) { error.value = message; return; }
  if (draft.value.probe !== null && !probes.value.some(p => p.id === draft.value.probe)) {
    error.value = 'Die ausgewählte Probe gehört nicht zum aktuellen Bauwerk und Projekt.';
    return;
  }
  const id = Number(currentUntersuchung.value.id);
  if (!Number.isSafeInteger(id) || id === 0) return;
  // Bearbeiter M:N links are not covered by offline field-level synchronization.
  const oldBearbeiter = toChloridanalyseDraft(currentUntersuchung.value).bearbeiter;
  const changedBearbeiter = JSON.stringify([...draft.value.bearbeiter].sort()) !==
    JSON.stringify([...oldBearbeiter].sort());
  if (changedBearbeiter && localMode() && (id < 0 || !(await isServerOnline(true)))) {
    error.value = 'Bearbeiter-Zuordnungen benötigen eine synchronisierte Chloridanalyse und eine Serververbindung.';
    return;
  }
  saving.value = true;
  try {
    // Vue refs contain reactive Proxies, which structuredClone() cannot serialize.
    // Rebuild a plain, typed draft from the current form values instead.
    const requested = toChloridanalyseDraft({ ...draft.value });
    requested.pruefstelle = requested.pruefstelle.trim();
    requested.pruefberichte = [...reports.value];
    await store.updateUntersuchung(id, 'chloridanalyse', chloridanalysePatch(requested));
    const persisted = await freshRecord(id);
    const differences = chloridanalysePersistenceDifferences(requested, persisted);
    if (differences.length) {
      throw new Error(`Nicht korrekt aus der Datenbank zurückgelesen: ${differences.join(', ')}. Die Eingaben bleiben erhalten.`);
    }
    editing.value = false;
    store.currentUntersuchung = { ...persisted, type: 'chloridanalyse' } as typeof store.currentUntersuchung;
    draft.value = toChloridanalyseDraft(persisted);
    reports.value = [...draft.value.pruefberichte];
    success.value = localMode() ? 'Chloridanalyse in SQLite gespeichert und überprüft.' : 'Chloridanalyse gespeichert und überprüft.';
  } catch (cause) {
    success.value = '';
    error.value = getErrorMessage(cause, 'Die Änderungen konnten nicht gespeichert werden.');
  } finally { saving.value = false; }
}

async function createProbe() {
  if (creatingProbe.value || !editing.value) return;
  newProbeError.value = '';
  if (localMode() && !(await isServerOnline(true))) {
    serverAvailable.value = false;
    newProbeError.value = 'Eine neue Probe kann in dieser Version nur online angelegt werden.';
    return;
  }
  const name = newProbe.value.probenbezeichnung.trim();
  if (!name || name.length > 100) { newProbeError.value = 'Bitte eine gültige Probenbezeichnung eingeben (maximal 100 Zeichen).'; return; }
  const start = newProbe.value.entnahmetiefeVon;
  const end = newProbe.value.entnahmetiefeBis;
  if ([start, end].some(v => v !== null && (!Number.isFinite(v) || v < 0)) ||
      (start !== null && end !== null && end < start)) {
    newProbeError.value = 'Bitte gültige Entnahmetiefen eingeben (Endtiefe ≥ Anfangstiefe).'; return;
  }
  const bauwerk = Number(currentUntersuchung.value.bauwerk);
  const projekt = Number(currentUntersuchung.value.projekt);
  if (![bauwerk, projekt].every(v => Number.isSafeInteger(v) && v > 0)) {
    newProbeError.value = 'Bauwerk und Projekt fehlen.'; return;
  }
  creatingProbe.value = true;
  try {
    // The Probe model uses Django's double-FK names: bauwerk_id and projekt_id.
    // The current Tauri resource adapter maps these to its SeaORM entity columns.
    const created = await apiConnector.postxResource('bwd', 'Probe', {
      probenbezeichnung: name,
      bemerkung: newProbe.value.bemerkung,
      entnahmetiefeVon: start,
      entnahmetiefeBis: end,
      bauwerk_id: bauwerk,
      projekt_id: projekt,
    });
    if (!Number.isSafeInteger(Number(created.id)) || Number(created.id) <= 0) {
      throw new Error('Die Probe wurde ohne gültige ID gespeichert.');
    }
    probes.value = [...probes.value, created];
    draft.value.probe = Number(created.id);
    newProbe.value = { probenbezeichnung: '', bemerkung: '', entnahmetiefeVon: null, entnahmetiefeBis: null };
    probeDialog.value = false;
    success.value = 'Probe angelegt. Die Verknüpfung wird mit „Speichern“ übernommen.';
  } catch (cause) { newProbeError.value = getErrorMessage(cause, 'Probe konnte nicht angelegt werden.'); }
  finally { creatingProbe.value = false; }
}

async function persistReports(ids: number[]): Promise<void> {
  const id = Number(currentUntersuchung.value.id);
  if (!Number.isSafeInteger(id) || id <= 0) throw new Error('Chloridanalyse muss zuerst angelegt werden.');
  await apiConnector.patchxResource('bwd', 'Chloridanalyse', { pruefberichte: ids }, id);
  const persisted = await freshRecord(id);
  const savedIds = toChloridanalyseDraft(persisted).pruefberichte;
  if (JSON.stringify([...ids].sort((a, b) => a - b)) !== JSON.stringify([...savedIds].sort((a, b) => a - b))) {
    throw new Error('Die Prüfberichte konnten nicht aus der Datenbank bestätigt werden.');
  }
  reports.value = [...savedIds];
  draft.value.pruefberichte = [...savedIds];
  store.currentUntersuchung = { ...persisted, type: 'chloridanalyse' } as typeof store.currentUntersuchung;
}
async function uploadReport(file: { name: string; beschreibung: string; datei: File; type?: string }): Promise<boolean> {
  if (!editing.value || uploadingReport.value || !canUploadFiles.value) return false;
  uploadingReport.value = true; reportError.value = '';
  let uploadedId: number | null = null;
  try {
    const created = await apiConnector.uploadMedia('bwd', {
      name: file.name,
      beschreibung: file.beschreibung,
      datei: file.datei,
      type: file.type,
      media_type: 'document',
      model_name: 'chloridanalyse',
      model_pk: Number(currentUntersuchung.value.id),
      bauwerk: Number(currentUntersuchung.value.bauwerk),
      projekt: Number(currentUntersuchung.value.projekt),
    });
    uploadedId = Number(created.id);
    if (!Number.isSafeInteger(uploadedId) || uploadedId <= 0) throw new Error('Datei wurde ohne gültige ID zurückgegeben.');
    await persistReports([...new Set([...reports.value, uploadedId])]);
    success.value = 'Prüfbericht gespeichert und mit der Chloridanalyse verknüpft.';
    return true;
  } catch (cause) {
    reportError.value = getErrorMessage(cause, 'Prüfbericht konnte nicht gespeichert werden.');
    // Never erase an uploaded file on an uncertain link failure: the user can recover its ID.
    if (uploadedId) reportError.value += ` Die Datei hat die ID ${uploadedId}; bitte nicht erneut hochladen, bevor die Verknüpfung geprüft wurde.`;
    return false;
  } finally { uploadingReport.value = false; }
}
async function detachReport(file: { id: number }): Promise<boolean> {
  if (!editing.value || uploadingReport.value || !canUploadFiles.value) return false;
  reportError.value = '';
  try {
    // Unlink only; deleting the shared Media row could affect other investigations.
    await persistReports(reports.value.filter(id => id !== file.id));
    success.value = 'Prüfbericht von dieser Untersuchung gelöst. Die Mediendatei bleibt erhalten.';
    return true;
  } catch (cause) {
    reportError.value = getErrorMessage(cause, 'Prüfbericht konnte nicht entfernt werden.');
    return false;
  }
}
async function deleteInvestigation() {
  if (deleting.value || !canDeleteInvestigation.value) return;
  deleting.value = true; error.value = '';
  try {
    await store.deleteUntersuchung(Number(currentUntersuchung.value.id), 'VorOrtUntersuchung', 'chloridanalyse');
    deleteDialog.value = false;
    goBack();
  } catch (cause) {
    error.value = getErrorMessage(cause, 'Chloridanalyse konnte nicht gelöscht werden.');
    deleteDialog.value = false;
  } finally { deleting.value = false; }
}
</script>
