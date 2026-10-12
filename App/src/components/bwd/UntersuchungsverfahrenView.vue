<template>
  <v-container class="untersuchungen-wrapper">
    <v-row>
    <v-col :cols="6" :xs="12" :sm="6" :md="5" :lg="6">
      <h1 class="text-h6 font-weight-regular text-secondary">{{ pageTitle }}</h1>
    </v-col>
    <v-spacer></v-spacer>
    <v-col :md="3">
      <v-combobox 
        multiple 
        clearable 
        label="Verfahren"
        :items="untersuchungsverfahrenOtions" 
        v-model="selectedUntersuchungsverfahrenOptions"
        item-title="title"
        item-value="value"
        return-object
        variant="underlined">
      </v-combobox>
    </v-col>
    <v-col :md="3">
      <v-select 
        clearable :items="untersuchungsstellenList" label="U-stelle" v-model="selectedUntersuchungsstelle"
        variant="underlined" :item-title="structureItemTitle" item-value="id"></v-select>

    </v-col>
    </v-row>


    <!-- Untersuchungen-Liste -->
    <v-row>
      <v-col v-if="isLoading" 
        :cols="12"
        :sm="12"
        :md="6">
        <v-skeleton-loader type="card" class="mb-4" />
        <v-skeleton-loader type="card" class="mb-4" />
      </v-col>
      <v-col v-if="isLoading" 
        :cols="12"
        :sm="12"
        :md="6">
        <v-skeleton-loader type="card" class="mb-4" />
        <v-skeleton-loader type="card" class="mb-4" />
      </v-col>
      
      <v-col v-else-if="error" cols="12">
        <v-alert type="error" closable>
          Fehler beim Laden der Untersuchungen: {{ error.message }}
        </v-alert>
      </v-col>

      <v-col v-else-if="filteredUntersuchungen.length === 0" cols="12">
        <v-alert type="info" variant="tonal">
          Keine Untersuchungen vorhanden. Neue Untersuchungen über den Plus-Button anlegen.
        </v-alert>
      </v-col>

      <v-col
        v-for="(untersuchung) in filteredUntersuchungen"
        :key="`${untersuchung.type}-${untersuchung.id}`"
        :cols="12"
        :sm="12"
        :md="6"
        :lg="6"
        :xl="6"
        @click="handleSelectUntersuchung(untersuchung.id, props.art , untersuchung.type)"
      >
        <v-card class="border-1 card-border" max-width="700" hover elevation-1>

            <v-img :src="getImageUrl(untersuchung.titelBild)" height="200px" cover>
            <template v-slot:placeholder>
              <v-skeleton-loader  type="image" />
            </template>
            <div class="corner-badge">
              <span class="regular-id">
                ID: {{ untersuchung.id }}
              </span>
            </div>
            <div class="duplicate-button">
              <v-btn
                icon
                size="small"
                color="primary"
                @click.stop="duplicateUntersuchung(untersuchung)"
                class="duplicate-btn"
              >
                <v-icon size="16">{{ mdiContentDuplicate }}</v-icon>
              </v-btn>
            </div>
            </v-img>
          <v-card-title class="card-text" >
            {{ untersuchung.name || getMethodDisplayName(untersuchung.type) }}
            
          </v-card-title>
          <v-card-subtitle class="mb-3 card-text">
            <v-row>
              <v-col class="text-left">
              <v-chip
                color="secondary"
              >
                {{ getUntersuchungsstelleName(untersuchung.untersuchungsstelle) }}
              </v-chip>
              </v-col>
              <v-col class="text-right">
                <v-chip color="secondary">
                  {{ getProjektName(untersuchung.projekt) }}
                </v-chip>
              </v-col>
            </v-row>
          </v-card-subtitle>
        </v-card>
      </v-col>
    </v-row>



    <FeedbackFeedComponent />
    <VLayoutItem 
      model-value 
      position="bottom" 
      class="text-end" 
      size="88">
      <div class="ma-4">
        <VBtn 
          :icon="mdiPlus" 
          size="large" 
          color="primary" 
          elevation="8" 
          @click="openCreateDialog"/>
      </div>
    </VLayoutItem>
    <!-- Dialog -->
    <v-dialog v-model="dialog" persistent max-width="600px">
      <v-card>
        <v-card-title>
          Neue Untersuchung Anlegen
        </v-card-title>
        <v-card-text>
          
          <v-select 
            v-model="newUntersuchung.type"
            :items="untersuchungsverfahrenOtions" 
            label="Type"
            item-title="title"
            item-value="value"
            auto-select-first>
          </v-select>
          <v-select 
            clearable 
            :items="untersuchungsstellenList" 
            label="untersuchungsstelle" 
            v-model="newUntersuchung.untersuchungsstelle"
            :item-title="structureItemTitle"
            item-value="id"
            @update:model-value="onNewUntersuchungsstelleChange">
          </v-select>
          <v-select
            :items="projektList"
            label="Projekt"
            v-model="newUntersuchung.projekt"
            :item-title="(projekt: IProjekt) => `${projekt.projektnummer} ${projekt.name}`"
            item-value="id">
          </v-select>

        </v-card-text>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn color="blue darken-1" variant="text" @click="dialog = false">Schließen</v-btn>
          <v-btn color="blue darken-1" variant="text" @click="createNewUntersuchung">Speichern</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </v-container>
</template>

<script setup lang="ts">
import { ref, toRef, computed, onMounted, onActivated } from 'vue'
import { useRoute } from 'vue-router'
import { useUvList } from '@/composables/bwd/useUvList'
import { useProjekte } from '@/composables/projekte'
import { useBauwerkIdRouteParam } from '@/composables/bauwerkIdRouteParam'
import { useUntersuchungenStore, type UntersuchungDetail, type UntersuchungListItem } from '@/store/bwd/UntersuchungsverfahrenStore'
import FeedbackFeedComponent from '@/components/FeedbackFeedComponent.vue'
import { mdiPlus, mdiContentDuplicate } from '@mdi/js'
import { apiConnector } from '@/utils/ApiConnector'
import { getErrorMessage } from '@/utils/errors'
import { getMethodDisplayName, normalizeMethodKey } from '@/utils/UntersuchungsverfahrenRegistry'
import type { IProjekt, IUntersuchungsstelle } from '@/types/apiInterfaces'
import router from '@/router'
import fallbackSVG from '@/assets/test.svg'
import { useFeedbackStore } from '@/store/FeedbackStore'
import { useUvImages } from '@/composables/useUvImages'

const props = defineProps<{ art: 'VorOrtUntersuchung' | 'Laboruntersuchung' }>()

const pageTitle = computed(() => props.art === 'VorOrtUntersuchung' ? 'Vor-Ort-Untersuchungen' : 'Laboruntersuchungen')

// The bauwerk comes from the route; all projects of the bauwerk are shown.
const route = useRoute()
const { bauwerkId } = useBauwerkIdRouteParam()
const bauwerkIdParam = computed(() => bauwerkId.value?.toString() ?? null)
const { projektList, refreshProjektList } = useProjekte()

// fetch list via Query (no server-side filtering)
const { data: uvList, error, isLoading, refetch } = useUvList({
  art: toRef(props, 'art'),
  bauwerkId: bauwerkIdParam,
  projectId: ref(null)
})

// Template now renders uvList.value (may be undefined while loading)
// We still need some additional UI state and helper functions
const newUntersuchung = ref<UntersuchungDetail>({} as UntersuchungDetail)
const untersuchungsverfahrenOtions = ref<Array<{ title: string; value: string }>>([])
const untersuchungsstellenList = ref<IUntersuchungsstelle[]>([])
const dialog = ref(false)

// Map the filter refs to the UI components
const initialUntersuchungsstelle = Number(route.query.untersuchungsstelle)
const selectedUntersuchungsstelle = ref<number | null>(Number.isFinite(initialUntersuchungsstelle) && initialUntersuchungsstelle > 0 ? initialUntersuchungsstelle : null)
const selectedUntersuchungsverfahrenOptions = ref<Array<{ title: string; value: string }>>([])

// Local filtering for all filter options
const filteredUntersuchungen = computed(() => {
  const list = uvList.value ?? []
  return list.filter((untersuchung: any) => {
    // Filter by selected Verfahren
    const matchesVerfahren = selectedUntersuchungsverfahrenOptions.value.length > 0 ? 
      selectedUntersuchungsverfahrenOptions.value.some(option => 
        normalizeMethodKey(untersuchung.type) === option.value
      ) : true;
    
    // Filter by selected Untersuchungsstelle
    const matchesUntersuchungsstelle = selectedUntersuchungsstelle.value ? 
      untersuchung.untersuchungsstelle === selectedUntersuchungsstelle.value : true;
    
    return matchesVerfahren && matchesUntersuchungsstelle;
  });
})

// Image management with the new composable
const imageIds = computed(() => {
  const list = uvList.value ?? []
  return list
    .map(item => item.titelBild)
    .filter((id): id is number => typeof id === 'number')
})

const { imagesMap } = useUvImages(imageIds)

// Helper function to get image URL with fallback
const getImageUrl = (imageId: string | number | null) => {
  if (typeof imageId === 'number') {
    const imageData = imagesMap.value.get(imageId)
    return imageData?.data || fallbackSVG
  }
  return fallbackSVG
}

// Store access for creation operations
const untersuchungenStore = useUntersuchungenStore()
const feedbackStore = useFeedbackStore()

async function fetchImplementedUntersuchungsverfahren() {
  const response = await apiConnector.getXResourceList('bwd', 'ImplementedUntersuchungsverfahren') as unknown as Record<'VorOrtUntersuchung' | 'Laboruntersuchung', string[]>;
  
  if (props.art === 'VorOrtUntersuchung') {
    untersuchungsverfahrenOtions.value = response.VorOrtUntersuchung.map((verfahren: string) => ({
      title: getMethodDisplayName(verfahren),
      value: normalizeMethodKey(verfahren)
    }))
  } else if (props.art === 'Laboruntersuchung') {
    untersuchungsverfahrenOtions.value = response.Laboruntersuchung.map((verfahren: string) => ({
      title: getMethodDisplayName(verfahren),
      value: normalizeMethodKey(verfahren)
    }))
  }
}

async function fetchUntersuchungsstellen() {
  // bauwerk_id is added from the route by the ApiConnector
  const response = await apiConnector.getXResourceList<IUntersuchungsstelle>('bwd', 'Untersuchungsstelle');
  untersuchungsstellenList.value = response;
}

onMounted(() => {
  fetchImplementedUntersuchungsverfahren().catch((e) => feedbackStore.addError(getErrorMessage(e, 'Untersuchungsverfahren konnten nicht geladen werden')));
  fetchUntersuchungsstellen().catch((e) => feedbackStore.addError(getErrorMessage(e, 'Untersuchungsstellen konnten nicht geladen werden')));
  refreshProjektList({ errorFeedback: true });
});

// Refetch data when component becomes active (e.g., navigating back)
onActivated(() => {
  refetch();
});

async function handleSelectUntersuchung(id: number, art: string, untersuchung: string) {
  // use vue router to got to UntersuchungsverfahrenDetailroute
  await untersuchungenStore.fetchUntersuchungDetail(id, art as any, untersuchung)
  router.push({
    name: 'UntersuchungsverfahrenDetail',
    params: { id: id.toString(), art: art, untersuchung: normalizeMethodKey(untersuchung), bauwerkId: bauwerkId.value }
  })
}

function openCreateDialog() {
  newUntersuchung.value = {} as UntersuchungDetail
  if (selectedUntersuchungsstelle.value) {
    newUntersuchung.value.untersuchungsstelle = selectedUntersuchungsstelle.value
    onNewUntersuchungsstelleChange(selectedUntersuchungsstelle.value)
  } else if (projektList.value.length === 1) {
    newUntersuchung.value.projekt = projektList.value[0].id
  }
  dialog.value = true
}

// preselect the projekt of the chosen Untersuchungsstelle
function onNewUntersuchungsstelleChange(untersuchungsstelleId: number | null) {
  const us = untersuchungsstellenList.value.find((u) => u.id === untersuchungsstelleId)
  if (us?.projekt) {
    newUntersuchung.value.projekt = us.projekt
  }
}

async function createNewUntersuchung() {
  if (!newUntersuchung.value.type || !newUntersuchung.value.untersuchungsstelle || !newUntersuchung.value.projekt) {
    feedbackStore.addError('Typ, Untersuchungsstelle und Projekt sind Pflichtfelder.', 'Speichern');
    return;
  }
  newUntersuchung.value.datum = new Date().toISOString().split('T')[0];
  newUntersuchung.value.bauwerk = Number(bauwerkId.value);
  
  try {
    const response = await apiConnector.BWD.createUntersuchung(newUntersuchung.value.type, newUntersuchung.value);
    const uType = normalizeMethodKey(newUntersuchung.value.type)
    
    // Refetch the list to show the new item
    await refetch()
    dialog.value = false
    feedbackStore.addSuccess(`${getMethodDisplayName(uType)} wurde erstellt.`, 'Speichern')
    await handleSelectUntersuchung(response.id, props.art, uType)
  } catch (error) {
    console.error('Fehler beim Erstellen der Untersuchung:', error);
    feedbackStore.addError(getErrorMessage(error, 'Untersuchung konnte nicht erstellt werden'), 'Speichern');
  }
}

async function duplicateUntersuchung(originalUntersuchung: UntersuchungListItem) {
  const duplicateData: Partial<UntersuchungDetail> = {
    bauwerk: originalUntersuchung.bauwerk ?? Number(bauwerkId.value),
    datum: new Date().toISOString().split('T')[0],
    projekt: originalUntersuchung.projekt,
    type: originalUntersuchung.type,
    untersuchungsstelle: originalUntersuchung.untersuchungsstelle,
  };
  try {
    await apiConnector.BWD.createUntersuchung(duplicateData.type!, duplicateData);
    await refetch()
  } catch (error) {
    console.error('Fehler beim Duplizieren der Untersuchung:', error);
    feedbackStore.addError(getErrorMessage(error, 'Untersuchung konnte nicht dupliziert werden'), 'Duplizieren');
  }
}

function getUntersuchungsstelleName(untersuchungsstelleId: number | null | undefined): string {
  if (!untersuchungsstelleId) return ''
  const us = untersuchungsstellenList.value.find((u) => u.id === untersuchungsstelleId)
  return us?.name || ''
}

function getProjektName(projektId: number | null | undefined): string {
  if (!projektId) return ''
  const projekt = projektList.value.find((p) => p.id === projektId)
  return projekt?.projektnummer ?? ''
}

function structureItemTitle(item: any): string {
  return item?.name || ''
}


</script>

<style scoped>
.corner-badge {
  position: absolute;
  top: 0;
  left: 0;
  background-color: rgba(66, 192, 235, 0.5);
  color: white;
  padding: 0.25rem 0.5rem;
  font-size: 1.1rem;
  border-radius: 0 0 8px 0;
  transition: all 0.3s ease;
}

.regular-id {
  font-weight: 500;
}

.corner-badge:hover {
  background-color: rgba(66, 192, 235, 0.8);
}

.duplicate-button {
  position: absolute;
  top: 8px;
  right: 8px;
  opacity: 0;
  transition: opacity 0.3s ease;
  z-index: 10;
}

.v-card:hover .duplicate-button {
  opacity: 1;
}

/* Show duplicate button always on mobile */
@media (max-width: 768px) {
  .duplicate-button {
    opacity: 1;
  }
}


</style>
