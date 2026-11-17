<template>
  <div class="gate-page-shell gate-kernbohrung-page gate-form">
    <GateActionButton intent="quiet" :prepend-icon="mdiArrowLeft" @click="goBack">Zur Untersuchungsstelle</GateActionButton>
    <GatePageHeader title="Kernbohrung" :subtitle="`Vor-Ort-Untersuchung · ID ${currentUntersuchung.id || ''} · lokal gespeichert`">
      <template #actions>
        <GateActionButton v-if="!editing" intent="primary" :prepend-icon="mdiPencil" @click="startEditing">Bearbeiten</GateActionButton>
        <template v-else>
          <GateActionButton intent="quiet" :disabled="saving" @click="cancelEditing">Abbrechen</GateActionButton>
          <GateActionButton intent="primary" :prepend-icon="mdiContentSaveAll" :loading="saving" :disabled="saving" @click="saveChanges">Speichern</GateActionButton>
        </template>
      </template>
    </GatePageHeader>

    <div class="gate-kernbohrung-page__summary" aria-label="Kernbohrung Übersicht">
      <GateStatusChip :status="draft.abgeschlossen ? 'completed' : 'in-progress'" />
      <v-chip variant="outlined" size="large">Untersuchungsstelle {{ currentUntersuchung.untersuchungsstelle || '–' }}</v-chip>
      <v-chip variant="outlined" size="large">{{ draft.schichtaufbau.length }} Schichten</v-chip>
    </div>
    <v-alert v-if="error" type="error" variant="tonal" class="mb-4">{{ error }}</v-alert>
    <v-alert v-if="success" type="success" variant="tonal" class="mb-4">{{ success }}</v-alert>
    <v-alert v-if="editing" type="info" variant="tonal" class="mb-4">
      Bearbeitungsmodus: Messwerte eingeben und anschließend „Speichern“ wählen.
      Mit „Abbrechen“ verwerfen Sie noch nicht gespeicherte Formulareingaben.
    </v-alert>

    <div class="gate-kernbohrung-page__sections">
      <InvestigationBasicsFields :draft="draft" :editing="editing" :people="people" />
      <KernbohrungDrillFields :draft="draft" :editing="editing" />
      <KernbohrungLayers :draft="draft" :editing="editing" />
      <GateFormSection title="Bohrkern" description="Zuordnung, Probenanzahl und Material des gewonnenen Kerns" :icon="mdiClipboardTextOutline">
        <v-progress-linear v-if="loadingCore" color="primary" indeterminate aria-label="Bohrkern wird geladen" />
        <v-alert v-if="coreError" type="error" variant="tonal" class="gate-kernbohrung-page__core-feedback" role="alert">
          {{ coreError }}
        </v-alert>
        <v-alert v-if="coreSuccess" type="success" variant="tonal" class="gate-kernbohrung-page__core-feedback" role="status">
          {{ coreSuccess }}
        </v-alert>
        <template v-if="core">
          <div class="gate-kernbohrung-page__core-id">Bohrkern · ID {{ core.id }}</div>
          <v-row>
            <v-col cols="12" sm="6"><v-text-field v-model="core.name" label="Bezeichnung" variant="outlined" :readonly="!editing" /></v-col>
            <v-col cols="12" sm="6"><NumberField :model-value="core.anzahlProben" @update:model-value="core.anzahlProben = $event ?? null" label="Anzahl Proben" integer min="0" variant="outlined" :readonly="!editing" /></v-col>
            <v-col cols="12"><MaterialsSelect v-model="core.material" :enabled="editing" variant="outlined" /></v-col>
          </v-row>
        </template>
        <template v-else-if="!loadingCore">
          <div class="mb-3">Noch kein Bohrkern zugeordnet.
            <span v-if="!editing">Wählen Sie „Bearbeiten“, um einen Bohrkern anzulegen.</span>
          </div>
          <template v-if="editing">
            <v-row align="center">
              <v-col cols="12" sm="7">
                <NumberField v-if="recoveryMode" v-model="recoveryCoreId" label="Bereits vorhandene Bohrkern-ID" integer min="1"
                  variant="outlined" hide-details="auto" hint="Nur verwenden, wenn bereits ein nicht verknüpfter Bohrkern existiert, z. B. ID 11." persistent-hint />
                <v-text-field v-else v-model="newCoreName" label="Bohrkern-Bezeichnung" variant="outlined" hide-details="auto" />
              </v-col>
              <v-col cols="12" sm="5">
                <GateActionButton intent="secondary" :prepend-icon="mdiPlus" :loading="creatingCore"
                  :disabled="saving || creatingCore || (!pendingCore && !(recoveryMode ? Number.isSafeInteger(Number(recoveryCoreId)) && Number(recoveryCoreId) > 0 : newCoreName.trim()))"
                  @click="createCore">{{ pendingCore || recoveryMode ? 'Bohrkern verknüpfen' : 'Bohrkern anlegen' }}</GateActionButton>
              </v-col>
            </v-row>
            <GateActionButton v-if="localMode()" intent="quiet" class="mt-2" :disabled="creatingCore"
              @click="recoveryMode = !recoveryMode; coreError = ''">
              {{ recoveryMode ? 'Neuen Bohrkern anlegen' : 'Bereits angelegten Bohrkern verknüpfen' }}
            </GateActionButton>
            <p class="text-medium-emphasis mt-1">Der Bohrkern wird lokal gespeichert und in einer Datenbanktransaktion mit der Kernbohrung verknüpft.</p>
          </template>
        </template>
      </GateFormSection>
      <GateFormSection title="Bilder" description="Fotos der Kernbohrung hinzufügen und lokal speichern – auch ohne Serververbindung" :icon="mdiImage">
        <UntersuchungsImageCard :images="Array.isArray(currentUntersuchung.bilder) ? currentUntersuchung.bilder : []"
          :enabled="editing" />
      </GateFormSection>
    </div>

    <div class="gate-kernbohrung-page__footer">
      <v-alert v-if="error" type="error" variant="tonal" class="gate-kernbohrung-page__footer-message" role="alert">{{ error }}</v-alert>
      <GateActionButton v-if="editing" intent="danger" :prepend-icon="mdiDelete" :disabled="saving" @click="deleteDialog = true">Untersuchung löschen</GateActionButton>
      <div class="gate-kernbohrung-page__footer-actions">
        <GateActionButton intent="quiet" :disabled="saving" @click="editing ? cancelEditing() : goBack()">{{ editing ? 'Abbrechen' : 'Zurück' }}</GateActionButton>
        <GateActionButton v-if="editing" intent="primary" :prepend-icon="mdiContentSaveAll" :loading="saving" :disabled="saving" @click="saveChanges">Änderungen speichern</GateActionButton>
      </div>
    </div>
    <v-dialog v-model="deleteDialog" max-width="540">
      <v-card class="gate-investigation-confirm">
        <v-card-title>Kernbohrung löschen?</v-card-title>
        <v-card-text>Diese lokal gespeicherte Untersuchung wird entfernt. Dies kann nicht rückgängig gemacht werden.</v-card-text>
        <v-card-actions>
          <GateActionButton intent="quiet" @click="deleteDialog = false">Abbrechen</GateActionButton>
          <GateActionButton intent="danger" :loading="deleting" :disabled="deleting" @click="deleteInvestigation">Löschen</GateActionButton>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useRoute, useRouter } from 'vue-router';
import { storeToRefs } from 'pinia';
import { mdiArrowLeft, mdiPencil, mdiContentSaveAll, mdiClipboardTextOutline, mdiPlus, mdiDelete, mdiImage } from '@mdi/js';
import { GateActionButton, GatePageHeader, GateStatusChip } from '@/components/common';
import GateFormSection from '@/components/investigations/GateFormSection.vue';
import InvestigationBasicsFields from '@/components/investigations/InvestigationBasicsFields.vue';
import KernbohrungDrillFields from '@/components/investigations/KernbohrungDrillFields.vue';
import KernbohrungLayers from '@/components/investigations/KernbohrungLayers.vue';
import NumberField from '@/components/bwd/NumberField.vue';
import MaterialsSelect from '@/components/bwd/MaterialsSelect.vue';
import UntersuchungsImageCard from '@/components/bwd/media/UntersuchungsImageCard.vue';
import { useUntersuchungenStore } from '@/store/bwd/UntersuchungsverfahrenStore';
import { apiConnector } from '@/utils/ApiConnector';
import { localMode } from '@/repositories/localDomainRepository';
import { getErrorMessage } from '@/utils/errors';
import { kernbohrungPatch, toKernbohrungDraft, validateKernbohrungDraft, kernbohrungPersistenceDifferences } from '@/types/kernbohrung';
import type { KernbohrungDraft } from '@/types/kernbohrung';
import type { IPerson } from '@/types/apiInterfaces';

interface Bohrkern { id: number; name: string; anzahlProben: number | null; material: number[] }
const store = useUntersuchungenStore();
const { currentUntersuchung } = storeToRefs(store);
const route = useRoute();
const router = useRouter();
const draft = ref<KernbohrungDraft>(toKernbohrungDraft({}));
const editing = ref(false);
const saving = ref(false);
const deleting = ref(false);
const deleteDialog = ref(false);
const loadingCore = ref(false);
const creatingCore = ref(false);
const core = ref<Bohrkern | null>(null);
const coreBackup = ref<Bohrkern | null>(null);
// If the Bohrkern was created but linking failed, retry linking the same ID.
const pendingCore = ref<Bohrkern | null>(null);
const coreError = ref('');
const coreSuccess = ref('');
const newCoreName = ref('');
const recoveryMode = ref(false);
const recoveryCoreId = ref<number | null>(null);
const people = ref<IPerson[]>([]);
const error = ref('');
const success = ref('');
let coreRequest = 0;

watch(() => currentUntersuchung.value, async record => {
  if (!record?.id || editing.value) return;
  editing.value = false;
  draft.value = toKernbohrungDraft(record);
  core.value = null;
  pendingCore.value = null;
  coreError.value = '';
  coreSuccess.value = '';
  const request = ++coreRequest;
  const coreId = Number(record.bohrkern);
  if (!coreId) return;
  loadingCore.value = true;
  try {
    const result = await store.fetchBohrkern(coreId) as Bohrkern;
    if (request === coreRequest) {
      core.value = { id: result.id, name: result.name ?? '',
        anzahlProben: result.anzahlProben ?? null, material: Array.isArray(result.material) ? result.material : [] };
    }
  } catch (cause) {
    if (request === coreRequest) coreError.value = getErrorMessage(cause, 'Bohrkern konnte nicht geladen werden.');
  } finally { if (request === coreRequest) loadingCore.value = false; }
}, { immediate: true });

// Dropdown values are loaded from the existing local-first API; no server is needed on tablets.
apiConnector.getResourceList('Person').then(rows => { people.value = rows as IPerson[]; }).catch(() => { people.value = []; });
function startEditing() {
  draft.value = toKernbohrungDraft(currentUntersuchung.value);
  coreBackup.value = core.value ? JSON.parse(JSON.stringify(core.value)) as Bohrkern : null;
  error.value = ''; success.value = '';
  coreError.value = '';
  coreSuccess.value = '';
  editing.value = true;
}
function cancelEditing() {
  draft.value = toKernbohrungDraft(currentUntersuchung.value);
  core.value = coreBackup.value ? JSON.parse(JSON.stringify(coreBackup.value)) as Bohrkern : core.value;
  editing.value = false;
  coreError.value = '';
  coreSuccess.value = '';
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
async function saveChanges() {
  if (!editing.value || saving.value) return;
  error.value = ''; success.value = '';
  const message = validateKernbohrungDraft(draft.value);
  if (message) { error.value = message; return; }
  if (!Number.isSafeInteger(Number(currentUntersuchung.value.id))) return;
  if (core.value && (core.value.anzahlProben !== null && (!Number.isInteger(Number(core.value.anzahlProben)) || Number(core.value.anzahlProben) < 0))) {
    error.value = 'Die Anzahl der Proben muss eine nicht negative ganze Zahl sein.'; return;
  }
  saving.value = true;
  try {
    if (core.value) {
      await apiConnector.patchxResource('bwd', 'Bohrkern', {
        name: core.value.name,
        anzahlProben: core.value.anzahlProben === null ? null : Number(core.value.anzahlProben),
        material: core.value.material,
      }, core.value.id);
    }
    const investigationId = Number(currentUntersuchung.value.id);
    // Snapshot the typed values before the store replaces the current record.
    const requested = JSON.parse(JSON.stringify(draft.value)) as KernbohrungDraft;
    await store.updateUntersuchung(investigationId, 'kernbohrung', kernbohrungPatch(requested));
    // Verify a fresh read from SQLite instead of trusting an old Pinia object.
    const fresh = await apiConnector.BWD.fetchUntersuchungDetailData(
      investigationId, 'kernbohrung', 'VorOrtUntersuchung'
    ) as Record<string, unknown>;
    const differences = kernbohrungPersistenceDifferences(requested, fresh);
    if (differences.length) {
      throw new Error(`Nicht korrekt aus der Datenbank zurückgelesen: ${differences.join(', ')}. Die Eingaben bleiben im Formular erhalten.`);
    }
    store.currentUntersuchung = { ...fresh, type: 'kernbohrung' } as typeof store.currentUntersuchung;
    draft.value = toKernbohrungDraft(fresh);
    editing.value = false;
    coreBackup.value = core.value ? JSON.parse(JSON.stringify(core.value)) as Bohrkern : null;
    success.value = 'Kernbohrung gespeichert und aus SQLite überprüft.';
  } catch (cause) {
    error.value = getErrorMessage(cause, 'Die Änderungen konnten nicht gespeichert werden.');
  } finally { saving.value = false; }
}
async function createCore() {
  if (!editing.value || creatingCore.value || saving.value) return;
  const investigationId = Number(currentUntersuchung.value.id);
  if (!Number.isSafeInteger(investigationId) || investigationId <= 0) {
    coreError.value = 'Bitte zuerst die Kernbohrung anlegen und öffnen.';
    return;
  }
  creatingCore.value = true;
  coreError.value = '';
  coreSuccess.value = '';
  try {
    let created: Bohrkern;
    if (localMode()) {
      // One SeaORM/SQLite transaction: no orphan core on a failed link.
      const existingId = pendingCore.value?.id ?? (recoveryMode.value ? recoveryCoreId.value : null);
      created = await invoke<Bohrkern>('local_create_or_link_bohrkern', {
        kernbohrungId: investigationId,
        name: newCoreName.value.trim(),
        existingBohrkernId: existingId,
      });
    } else {
      // Browser/Django fallback pending a server-side atomic endpoint.
      created = pendingCore.value ?? await store.createBohrkern(newCoreName.value.trim()) as Bohrkern;
      pendingCore.value = created;
      try {
        await store.updateUntersuchung(investigationId, 'kernbohrung', { bohrkern: Number(created.id) });
      } catch (cause) {
        throw new Error(`Bohrkern ID ${created.id} wurde angelegt, aber die Verknüpfung ist fehlgeschlagen: ${getErrorMessage(cause, 'Speicherfehler')}`);
      }
    }
    // Confirm both the row and the foreign key after the transaction.
    const refreshed = await apiConnector.BWD.fetchUntersuchungDetailData(
      investigationId, 'kernbohrung', 'VorOrtUntersuchung'
    ) as Record<string, unknown>;
    if (Number(refreshed.bohrkern) !== Number(created.id)) {
      throw new Error(`Bohrkern ID ${created.id} wurde nicht als Verknüpfung zurückgelesen. Bitte nicht erneut anlegen; Datenbankzustand prüfen.`);
    }
    store.currentUntersuchung = { ...refreshed, type: 'kernbohrung' } as typeof store.currentUntersuchung;
    const confirmed = {
      id: Number(created.id), name: created.name ?? newCoreName.value.trim(),
      anzahlProben: created.anzahlProben ?? null,
      material: Array.isArray(created.material) ? created.material : [],
    };
    core.value = confirmed;
    coreBackup.value = structuredClone(confirmed);
    pendingCore.value = null;
    newCoreName.value = '';
    recoveryMode.value = false;
    recoveryCoreId.value = null;
    coreSuccess.value = 'Bohrkern angelegt/verknüpft und in SQLite überprüft.';
  } catch (cause) {
    coreError.value = getErrorMessage(cause, 'Bohrkern konnte nicht gespeichert werden.');
  } finally {
    creatingCore.value = false;
  }
}
async function deleteInvestigation() {
  if (deleting.value) return;
  deleting.value = true; error.value = '';
  try {
    await store.deleteUntersuchung(Number(currentUntersuchung.value.id), 'VorOrtUntersuchung', 'kernbohrung');
    deleteDialog.value = false;
    goBack();
  } catch (cause) { error.value = getErrorMessage(cause, 'Untersuchung konnte nicht gelöscht werden.'); deleteDialog.value = false; }
  finally { deleting.value = false; }
}
</script>

