<template>
  <div class="hierarchical-structure">
    <FeedbackFeedComponent />

    <!-- Navigation Bar with Header -->
    <NavigationHeader
      :structure-name="bauwerkName"
      :navigation-path="navigationPath"
      :show-progress="showProgress"
      :loading="loading"
      @toggle-progress="showProgress = !showProgress"
      @refresh="loadStructure"
      @navigate-to-level="navigateToLevel"
    />

    <!-- Loading State -->
    <v-card v-if="loading && !loadedOnce" class="state-card loading-card" elevation="2">
      <div class="loading-content">
        <v-progress-circular indeterminate color="primary" size="64" width="4" class="loading-spinner" />
        <div class="loading-text">
          <h2>Struktur wird geladen …</h2>
          <p class="text-caption">Bitte warten Sie einen Moment</p>
        </div>
      </div>
    </v-card>

    <!-- Error State -->
    <v-card v-else-if="error" class="state-card error-card" elevation="2">
      <div class="error-content">
        <v-icon size="48" color="error">{{ mdiAlertCircle }}</v-icon>
        <div>
          <h2 class="error-title">Fehler beim Laden</h2>
          <p class="error-message">{{ error }}</p>
        </div>
        <v-btn color="error" variant="tonal" size="large" class="retry-button" :prepend-icon="mdiRefresh" @click="loadStructure">
          Erneut versuchen
        </v-btn>
      </div>
    </v-card>

    <!-- Structure Content -->
    <div v-else>
      <!-- Current Level Info & Actions -->
      <v-card class="mb-3 level-info-card" elevation="1">
        <v-card-text class="py-4">
          <div class="level-header">
            <div class="level-info">
              <div class="level-name">
                <v-icon :icon="mdiDomain" class="mr-2 level-icon" size="20" />
                <h2 class="level-title">{{ bauwerkName }}</h2>
                <div
                  class="level-progress ml-4 progress-preserve"
                  :class="{ 'is-visible': showProgress && bauwerkProgress.total > 0 }"
                >
                  <v-tooltip v-if="bauwerkProgress.total > 0" location="top">
                    <template #activator="{ props }">
                      <div v-bind="props" class="level-progress-indicator">
                        <v-progress-circular
                          :model-value="showProgress ? Math.round(bauwerkProgress.percent) : 0"
                          :size="36"
                          :width="5"
                          color="primary"
                        >
                          <small v-show="showProgress">{{ Math.round(bauwerkProgress.percent) }}%</small>
                        </v-progress-circular>
                      </div>
                    </template>
                    {{ bauwerkProgress.completed }} / {{ bauwerkProgress.total }} Verfahren abgeschlossen
                  </v-tooltip>
                </div>
              </div>
              <p class="level-description">{{ levelDescription }}</p>
            </div>
            <div class="level-actions">
              <v-text-field
                v-model="searchQuery"
                placeholder="Untersuchungsstellen durchsuchen …"
                aria-label="Untersuchungsstellen durchsuchen"
                variant="outlined"
                density="compact"
                hide-details
                clearable
                class="search-field"
                :prepend-inner-icon="mdiMagnify"
              >
                <template v-slot:append-inner>
                  <v-fade-transition>
                    <v-chip
                      v-if="searchQuery && filteredUntersuchungsstellen.length !== untersuchungsstellen.length"
                      size="x-small"
                      color="primary"
                      variant="flat"
                      class="search-result-count"
                    >
                      {{ filteredUntersuchungsstellen.length }}/{{ untersuchungsstellen.length }}
                    </v-chip>
                  </v-fade-transition>
                </template>
              </v-text-field>
            </div>
          </div>
        </v-card-text>
      </v-card>

      <v-alert v-if="activeProjektId !== null" type="info" variant="tonal" class="mb-3">
        Projektfilter: {{ getProjektName(activeProjektId) || `Projekt #${activeProjektId}` }}
        <v-btn variant="text" color="primary" @click="clearProjectFilter">Alle Projekte anzeigen</v-btn>
      </v-alert>

      <!-- Compact List View -->
      <v-card elevation="2" class="structure-list-card">
        <v-list class="py-0 structure-list">
          <!-- Add new item (always at the top) -->
          <v-list-item class="add-new-item" @click="openCreateDialog">
            <v-list-item-title class="add-item-title">
              <v-icon :icon="mdiPlus" size="18" class="mr-2" />
              Neue Untersuchungsstelle hinzufügen
            </v-list-item-title>
          </v-list-item>

          <v-divider v-if="filteredUntersuchungsstellen.length > 0" />

          <!-- List items -->
          <template v-for="(item, index) in filteredUntersuchungsstellen" :key="item.id">
            <StructureListItem
              :item="item"
              :show-progress="showProgress"
              :progress="getNodeProgress(item.id)"
              :expanded="expandedUS === item.id"
              :compact="mobile"
              :projekt-name="getProjektName(item.projekt)"
              @click="toggleExpansion"
              @edit="openEditDialog"
              @delete="requestDelete"
            />

            <!-- Investigation Methods Expansion for Untersuchungsstelle -->
            <InvestigationMethodsExpansion
              :is-expanded="expandedUS === item.id"
              :methods="methodsByStelle.get(item.id) ?? []"
              :untersuchungsstelle="item"
              @created="onUntersuchungCreated"
              @navigate-to-method="navigateToInvestigationMethod"
            />

            <v-divider v-if="index < filteredUntersuchungsstellen.length - 1" />
          </template>

          <!-- Empty state -->
          <v-list-item v-if="filteredUntersuchungsstellen.length === 0" class="empty-state-item">
            <div class="empty-state">
              <v-icon size="64" :color="searchQuery ? 'orange-lighten-1' : 'grey-lighten-1'">
                {{ searchQuery ? mdiMagnify : mdiPackageVariantClosed }}
              </v-icon>
              <h3 class="mt-4 empty-state-title">
                {{ searchQuery ? 'Keine Suchergebnisse' : 'Keine Einträge' }}
              </h3>
              <p class="text-caption mb-3 empty-state-message">
                {{
                  searchQuery
                    ? `Keine Ergebnisse für „${searchQuery}“ gefunden`
                    : 'Für dieses Bauwerk sind noch keine Untersuchungsstellen angelegt.'
                }}
              </p>
              <v-btn
                v-if="searchQuery"
                color="primary"
                variant="outlined"
                size="small"
                :prepend-icon="mdiClose"
                class="empty-state-action"
                @click="searchQuery = ''"
              >
                Suche zurücksetzen
              </v-btn>
            </div>
          </v-list-item>
        </v-list>
      </v-card>
    </div>

    <!-- Add/Edit Dialog -->
    <DialogComponent max-width="600" :show="showDialog" @update:modelValue="closeDialog()">
      <template #title>
        <span class="font-weight-regular text-secondary">
          {{ editingId ? 'Untersuchungsstelle bearbeiten' : 'Neue Untersuchungsstelle' }}
        </span>
      </template>
      <template #text>
        <v-form ref="formRef" v-model="formValid" @submit.prevent="saveUntersuchungsstelle">
          <v-text-field v-model="form.name" label="Name" :rules="[rules.required]" />
          <v-select
            v-model="form.projekt"
            :items="projektList"
            :item-title="(projekt: IProjekt) => `${projekt.projektnummer} ${projekt.name}`"
            item-value="id"
            label="Projekt"
            :rules="[rules.required]"
            no-data-text="Dem Bauwerk ist kein Projekt zugeordnet"
          />
          <v-textarea v-model="form.bemerkung" label="Bemerkung" rows="2" />
        </v-form>
      </template>
      <template #actions>
        <v-spacer />
        <DismissButton @click="closeDialog()">Abbrechen</DismissButton>
        <NeutralButton :loading="saving" :disabled="!formValid" @click="saveUntersuchungsstelle()">Speichern</NeutralButton>
      </template>
    </DialogComponent>

    <!-- Delete Confirmation Dialog -->
    <DialogComponent max-width="520" :show="!!nodeToDelete" @update:modelValue="nodeToDelete = null">
      <template #title>
        <span class="font-weight-regular">Untersuchungsstelle löschen?</span>
      </template>
      <template #text>
        <p class="delete-message">
          „{{ nodeToDelete?.name }}“ wird gelöscht.
        </p>
        <p v-if="deleteMethodCount > 0" class="delete-warning">
          Dabei werden auch {{ deleteMethodCount }} zugehörige Untersuchungsverfahren gelöscht.
        </p>
      </template>
      <template #actions>
        <v-spacer />
        <DismissButton @click="nodeToDelete = null">Abbrechen</DismissButton>
        <v-btn color="error" variant="flat" :loading="deleting" @click="confirmDelete">Löschen</v-btn>
      </template>
    </DialogComponent>
  </div>
</template>

<script setup lang="ts">
/*
  Arbeitsbereich of a Bauwerk. An Untersuchungsstelle references the Bauwerk directly, so only
  one level is shown.
*/
import {
  mdiAlertCircle,
  mdiClose,
  mdiDomain,
  mdiMagnify,
  mdiPackageVariantClosed,
  mdiPlus,
  mdiRefresh,
} from '@mdi/js';
import { computed, onMounted, ref, watch } from 'vue';
import { useRoute } from 'vue-router';
import { useDisplay } from 'vuetify';

import InvestigationMethodsExpansion, {
  type UntersuchungsstelleMethodItem,
  type UntersuchungsverfahrenArt,
} from '@/components/bwd/Arbeitsbereich/InvestigationMethodsExpansion.vue';
import NavigationHeader from '@/components/bwd/Arbeitsbereich/NavigationHeader.vue';
import StructureListItem from '@/components/bwd/Arbeitsbereich/StructureListItem.vue';
import { DismissButton, NeutralButton } from '@/components/buttons';
import { DialogComponent } from '@/components/dialogs';
import FeedbackFeedComponent from '@/components/FeedbackFeedComponent.vue';
import { useBauwerkIdRouteParam } from '@/composables/bauwerkIdRouteParam';
import { useProjekte } from '@/composables/projekte';
import router from '@/router';
import { useBauwerkStore } from '@/store/BauwerkStore';
import { useFeedbackStore } from '@/store/FeedbackStore';
import type { IProjekt, IUntersuchungsstelle } from '@/types/apiInterfaces';
import { apiConnector } from '@/utils/ApiConnector';
import { getErrorMessage } from '@/utils/errors';
import { getMethodRouteSlug } from '@/utils/UntersuchungsverfahrenRegistry';

interface ProcedureProgress {
  completed: number;
  total: number;
  percent: number;
}

type GroupedUvResponse = Partial<Record<UntersuchungsverfahrenArt, Record<string, Array<Record<string, any>>>>>;

const route = useRoute();
const bauwerkStore = useBauwerkStore();
const feedbackStore = useFeedbackStore();
const { bauwerkId } = useBauwerkIdRouteParam();
// only the projects of this bauwerk (bauwerk_id is added from the route)
const { projektList, refreshProjektList } = useProjekte();
// "mobile" follows the Vuetify mobile breakpoint (< 960px)
const { mobile } = useDisplay();

const untersuchungsstellen = ref<IUntersuchungsstelle[]>([]);
const methods = ref<UntersuchungsstelleMethodItem[]>([]);
const loading = ref(false);
const loadedOnce = ref(false);
const error = ref<string | null>(null);
const showProgress = ref(false);
const searchQuery = ref<string | null>('');
const expandedUS = ref<number | null>(null);
const activeProjektId = computed<number | null>(() => {
  const value = Number(route.query.projekt);
  return route.query.projekt && Number.isSafeInteger(value) && value > 0 ? value : null;
});
const requestedStelleId = computed<number | null>(() => {
  const value = Number(route.query.stelle);
  return route.query.stelle && Number.isSafeInteger(value) && value > 0 ? value : null;
});

const bauwerkName = computed(() => bauwerkStore.selectedBauwerk?.bezeichnung ?? 'Bauwerk');

const navigationPath = computed(() => [
  { id: 0, name: 'Bauwerke', type: 'bauwerkswahl' },
  { id: Number(bauwerkId.value), name: bauwerkName.value, type: 'bauwerk' },
]);

const filteredUntersuchungsstellen = computed(() => {
  const query = (searchQuery.value ?? '').trim().toLowerCase();
  const sorted = [...untersuchungsstellen.value].sort((a, b) =>
    (a.name || '').localeCompare(b.name || '', 'de', { numeric: true }),
  );
  const byProject = activeProjektId.value === null ? sorted : sorted.filter(item => item.projekt === activeProjektId.value);
  if (!query) return byProject;
  return byProject.filter((item) =>
    [item.name, item.bemerkung, getProjektName(item.projekt)]
      .filter(Boolean)
      .some((value) => String(value).toLowerCase().includes(query)),
  );
});

const methodsByStelle = computed(() => {
  const map = new Map<number, UntersuchungsstelleMethodItem[]>();
  for (const method of methods.value) {
    const stelleId = (method as UntersuchungsstelleMethodItem & { untersuchungsstelle: number | null }).untersuchungsstelle;
    if (stelleId === null) continue;
    const list = map.get(stelleId) ?? [];
    list.push(method);
    map.set(stelleId, list);
  }
  return map;
});

const visibleMethodList = computed(() => {
  const ids = new Set(filteredUntersuchungsstellen.value.map(stelle => stelle.id));
  return methods.value.filter(method => {
    const stelleId = (method as UntersuchungsstelleMethodItem & { untersuchungsstelle?: number | null }).untersuchungsstelle;
    return ids.has(stelleId ?? -1);
  });
});
const bauwerkProgress = computed(() => toProgress(visibleMethodList.value));

const levelDescription = computed(() => {
  const stellen = filteredUntersuchungsstellen.value.length;
  const verfahren = visibleMethodList.value.length;
  const stellenText = stellen === 1 ? '1 Untersuchungsstelle' : `${stellen} Untersuchungsstellen`;
  const verfahrenText = verfahren === 1 ? '1 Untersuchungsverfahren' : `${verfahren} Untersuchungsverfahren`;
  return `${stellenText} · ${verfahrenText}`;
});

function toProgress(list: UntersuchungsstelleMethodItem[]): ProcedureProgress {
  const total = list.length;
  const completed = list.filter((method) => method.abgeschlossen === true).length;
  return { completed, total, percent: total > 0 ? (completed / total) * 100 : 0 };
}

function getNodeProgress(untersuchungsstelleId: number): ProcedureProgress {
  return toProgress(methodsByStelle.value.get(untersuchungsstelleId) ?? []);
}

function getProjektName(projektId: number | null): string {
  if (!projektId) return '';
  const projekt = projektList.value.find((p) => p.id === projektId);
  return projekt ? `${projekt.projektnummer} ${projekt.name}` : '';
}

function flattenUvResponse(response: GroupedUvResponse): UntersuchungsstelleMethodItem[] {
  const items: UntersuchungsstelleMethodItem[] = [];
  for (const category of ['VorOrtUntersuchung', 'Laboruntersuchung'] as const) {
    for (const [type, list] of Object.entries(response[category] ?? {})) {
      for (const uv of list) {
        items.push({
          id: uv.id,
          type,
          category,
          datum: uv.datum ?? null,
          abgeschlossen: uv.abgeschlossen ?? null,
          sort_order: uv.sort_order ?? 0,
          untersuchungsstelle: uv.untersuchungsstelle ?? null,
        } as UntersuchungsstelleMethodItem);
      }
    }
  }
  return items;
}

async function loadStructure() {
  if (!bauwerkId.value || bauwerkId.value === '0') return;
  loading.value = true;
  error.value = null;
  try {
    const [stellen, uvResponse] = await Promise.all([
      apiConnector.getXResourceList<IUntersuchungsstelle>('bwd', 'Untersuchungsstelle', undefined, String(bauwerkId.value)),
      apiConnector.BWD.fetchUvData('all', String(bauwerkId.value)),
      refreshProjektList({ errorFeedback: true }),
    ]);
    untersuchungsstellen.value = stellen.filter(stelle => stelle.bauwerk === Number(bauwerkId.value));
    expandedUS.value = requestedStelleId.value && untersuchungsstellen.value.some(s => s.id === requestedStelleId.value)
      ? requestedStelleId.value : null;
    methods.value = flattenUvResponse(uvResponse);
    loadedOnce.value = true;
  } catch (e) {
    error.value = getErrorMessage(e, 'Untersuchungsstellen konnten nicht geladen werden');
  } finally {
    loading.value = false;
  }
}

onMounted(loadStructure);
watch(requestedStelleId, (value) => {
  expandedUS.value = value && untersuchungsstellen.value.some(s => s.id === value) ? value : null;
});
watch(bauwerkId, () => {
  expandedUS.value = null;
  loadStructure();
});

function navigateToLevel(levelIndex: number) {
  if (navigationPath.value[levelIndex]?.type === 'bauwerkswahl') {
    router.push({ name: 'bauwerkswahl' });
    return;
  }
  void router.push({ name: 'BauwerkUebersicht', params: { bauwerkId: String(bauwerkId.value) } });
}

function clearProjectFilter() {
  void router.replace({ name: 'Untersuchungsstellen', params: { bauwerkId: String(bauwerkId.value) } });
}

function toggleExpansion(item: IUntersuchungsstelle) {
  expandedUS.value = expandedUS.value === item.id ? null : item.id;
}

function navigateToInvestigationMethod(methodItem: UntersuchungsstelleMethodItem) {
  router.push({
    name: 'UntersuchungsverfahrenDetail',
    params: {
      bauwerkId: String(bauwerkId.value),
      art: methodItem.category,
      untersuchung: getMethodRouteSlug(methodItem.type),
      id: String(methodItem.id),
    },
  });
}

async function onUntersuchungCreated() {
  // Stay in the Arbeitsbereich and refresh the structure.
  await loadStructure();
}

// ---------------------------------------------------------------------------
// Add / Edit
// ---------------------------------------------------------------------------
const showDialog = ref(false);
const formRef = ref();
const formValid = ref(false);
const saving = ref(false);
const editingId = ref<number | null>(null);
const form = ref<{ name: string; projekt: number | null; bemerkung: string }>({
  name: '',
  projekt: null,
  bemerkung: '',
});

const rules = {
  required: (value: unknown) => !!value || 'Dieses Feld ist erforderlich',
};

function openCreateDialog() {
  editingId.value = null;
  form.value = {
    name: '',
    projekt: projektList.value.length === 1 ? projektList.value[0].id : null,
    bemerkung: '',
  };
  showDialog.value = true;
}

function openEditDialog(item: IUntersuchungsstelle) {
  editingId.value = item.id;
  form.value = {
    name: item.name,
    projekt: item.projekt,
    bemerkung: item.bemerkung,
  };
  showDialog.value = true;
}

function closeDialog() {
  showDialog.value = false;
  formRef.value?.resetValidation();
}

async function saveUntersuchungsstelle() {
  if (!formValid.value) return;
  saving.value = true;
  const label = form.value.name;
  try {
    if (editingId.value) {
      await apiConnector.patchxResource('bwd', 'Untersuchungsstelle', form.value, editingId.value);
      feedbackStore.addSuccess(`Untersuchungsstelle „${label}“ wurde gespeichert.`, 'Speichern');
    } else {
      await apiConnector.postxResource('bwd', 'Untersuchungsstelle', {
        ...form.value,
        bauwerk: Number(bauwerkId.value),
      });
      feedbackStore.addSuccess(`Untersuchungsstelle „${label}“ wurde angelegt.`, 'Speichern');
    }
    closeDialog();
    await loadStructure();
  } catch (e) {
    feedbackStore.addError(getErrorMessage(e, 'Untersuchungsstelle konnte nicht gespeichert werden'), 'Speichern');
  } finally {
    saving.value = false;
  }
}

// ---------------------------------------------------------------------------
// Delete
// ---------------------------------------------------------------------------
const nodeToDelete = ref<IUntersuchungsstelle | null>(null);
const deleting = ref(false);
const deleteMethodCount = computed(() =>
  nodeToDelete.value ? (methodsByStelle.value.get(nodeToDelete.value.id)?.length ?? 0) : 0,
);

function requestDelete(item: IUntersuchungsstelle) {
  nodeToDelete.value = item;
}

async function confirmDelete() {
  if (!nodeToDelete.value) return;
  deleting.value = true;
  const item = nodeToDelete.value;
  try {
    await apiConnector.deletexResource('bwd', 'Untersuchungsstelle', item.id);
    feedbackStore.addSuccess(`Untersuchungsstelle „${item.name}“ wurde gelöscht.`, 'Löschen');
    if (expandedUS.value === item.id) expandedUS.value = null;
    nodeToDelete.value = null;
    await loadStructure();
  } catch (e) {
    feedbackStore.addError(getErrorMessage(e, 'Untersuchungsstelle konnte nicht gelöscht werden'), 'Löschen');
  } finally {
    deleting.value = false;
  }
}
</script>

<style scoped lang="scss" src="./UntersuchungsstellenView.styles.scss"></style>
