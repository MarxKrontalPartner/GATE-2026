<template>
  <GenericDetailView ref="genericDetailView">
    <template v-slot:untersuchung="{ enabled }">
      <v-container>

        <UntersuchungInfoCard :enabled="enabled" />

        <!-- Schichtaufbau -->
        <v-row>
          <v-col cols="12">
            <SchichtenCard 
              v-model="store.currentUntersuchung.schichtaufbau" 
              :enabled="enabled" 
            />
          </v-col>
        </v-row>

        <v-row>
          <v-col cols="12" md="12">
            <v-sheet :elevation="5" border rounded>
              <v-toolbar :elevation="1">
                <v-toolbar-title>Bohrung</v-toolbar-title>
              </v-toolbar>
              <v-container>
                <v-row>
                  <v-col cols="12" md="4">
                    <NumberField v-model="currentUntersuchung.durchmesser" label="Durchmesser" suffix="mm"
                      min="0" :disabled="!enabled" />
                  </v-col>
                  <v-col cols="12" md="4">
                    <NumberField v-model="currentUntersuchung.bohrlaenge" label="Bohrlänge" suffix="mm"
                      min="0" :disabled="!enabled" />
                  </v-col>
                  <v-col cols="12" md="4">
                    <NumberField v-model="currentUntersuchung.neigungswinkel" label="Neigungswinkel" suffix="°"
                      hint="zur Horizontalen" persistent-hint :disabled="!enabled" />
                  </v-col>
                </v-row>
                <v-row>
                  <v-col cols="12" md="6">
                    <ChoiceSelect v-model="currentUntersuchung.kuehlung" :items="kuehlungOptions" label="Kühlung"
                      :disabled="!enabled" />
                  </v-col>
                  <v-col cols="12" md="6">
                    <v-text-field v-model="currentUntersuchung.verschlussmaterial" label="Verschlussmaterial"
                      :disabled="!enabled" />
                  </v-col>
                </v-row>
                <v-row>
                  <v-col cols="12" md="6">
                    <v-checkbox v-model="currentUntersuchung.bewehrungAngeschnitten" label="Bewehrung angeschnitten"
                      :disabled="!enabled" hide-details />
                  </v-col>
                  <v-col cols="12" md="6">
                    <v-checkbox v-model="currentUntersuchung.kernVollstaendig" label="Kern vollständig"
                      :disabled="!enabled" hide-details />
                  </v-col>
                </v-row>
              </v-container>
            </v-sheet>
          </v-col>
        </v-row>
        <!-- Bohrkern -->
        <v-row>
          <v-col>
            <v-sheet :elevation="5" border rounded class="position-relative overflow-hidden">
              <div class="text-subtitle-2 pa-3 pb-0">Bohrkern</div>
              <v-col v-if="bohrkern?.id" cols="12">
              <v-card flat>
                <!-- Corner tag for ID -->
                <div class="position-absolute bg-secondary px-2 py-1 rounded-bl-xl" style="top: 0; right: 0; font-size: 0.8rem;">
                ID: {{ bohrkern.id }}
                </div>
                
                <v-card-title class="text-h6 pb-2">{{ bohrkern.name }}</v-card-title>
                
                <v-card-text>
                <v-row>
                  <v-col cols="12" md="6">
                    <MaterialsSelect
                      v-model="bohrkern.material"
                      :enabled="enabled"
                      variant="outlined"
                    />
                  </v-col>
                  <v-col cols="12" md="6">
                  <v-text-field 
                    v-model="bohrkern.anzahlProben" 
                    label="Anzahl Proben" 
                    :disabled="!enabled"
                    variant="outlined"
                    density="comfortable"
                    type="number"
                  ></v-text-field>
                  </v-col>
                  <v-col cols="12" md="6">
                  <v-text-field 
                    v-model="bohrkern.name" 
                    label="Bezeichnung" 
                    :disabled="!enabled"
                    variant="outlined"
                    density="comfortable"
                  ></v-text-field>
                  </v-col>
                </v-row>
                </v-card-text>
              </v-card>
              </v-col>
              <v-col v-else cols="12" class="pa-4">
                <v-card-title class="text-h6 pb-2">Kein Bohrkern vorhanden</v-card-title>
                <v-card-subtitle class="pb-3">Neuen Bohrkern erstellen</v-card-subtitle>
                <v-form @submit.prevent="createBohrkern">
                  <v-text-field
                    v-model="newBohrkern"
                    label="Bezeichnung"
                    :disabled="!enabled"
                    variant="outlined"
                    density="comfortable"
                    hide-details
                    class="mb-3"
                  >
                    <template #append>
                      <v-btn
                        color="primary"
                        @click="createBohrkern"
                        :disabled="!enabled || !newBohrkern"
                        variant="tonal"
                        :prepend-icon="mdiPlusCircle"
                      >
                        Erstellen
                      </v-btn>
                    </template>
                  </v-text-field>
                </v-form>
              </v-col>
            </v-sheet>

          </v-col>
        </v-row>
      </v-container>


    </template>
  </GenericDetailView>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, watch } from 'vue';
import { storeToRefs } from 'pinia';

import GenericDetailView from '@/views/bwd/Untersuchungsverfahren/GenericDetailView.vue';
import UntersuchungInfoCard from '@/components/bwd/UntersuchungInfoCard.vue';
import MaterialsSelect from '@/components/bwd/MaterialsSelect.vue';
import SchichtenCard from '@/components/bwd/SchichtenCard.vue';
import NumberField from '@/components/bwd/NumberField.vue';
import ChoiceSelect, { type ChoiceOption } from '@/components/bwd/ChoiceSelect.vue';
import { mdiPlusCircle } from '@mdi/js';

import { useUntersuchungenStore } from '@/store/bwd/UntersuchungsverfahrenStore';
import { useFeedbackStore } from '@/store/FeedbackStore';
import { apiConnector } from '@/utils/ApiConnector';
import { getErrorMessage } from '@/utils/errors';

const kuehlungOptions: ChoiceOption[] = [
  { value: 'nass', title: 'Nassbohrung' },
  { value: 'trocken', title: 'Trockenbohrung' },
];

const genericDetailView = ref<InstanceType<typeof GenericDetailView> | null>(null);
// edit mode of the GenericDetailView
const isEnabled = computed<boolean>(() => Boolean(genericDetailView.value?.editInfo));

const newBohrkern = ref<string>("");
const store = useUntersuchungenStore();
const feedbackStore = useFeedbackStore();
const { currentUntersuchung } = storeToRefs(store);

interface Bohrkern {
  id: number;
  name: string;
  anzahlProben: number | null;
  material: number[];
}


const bohrkern = ref<Bohrkern | null>(null);

function buildBohrkernPatchPayload(currentBohrkern: Bohrkern) {
  return {
    name: currentBohrkern.name,
    anzahlProben: currentBohrkern.anzahlProben === null || String(currentBohrkern.anzahlProben) === ''
      ? null
      : Number(currentBohrkern.anzahlProben),
    material: Array.isArray(currentBohrkern.material) ? currentBohrkern.material : [],
  };
}

async function hydrateLinkedState() {
  bohrkern.value = null;
  if (currentUntersuchung.value.bohrkern) {
    try {
      bohrkern.value = await store.fetchBohrkern(currentUntersuchung.value.bohrkern);
    } catch (e) {
      console.error('Bohrkern laden fehlgeschlagen', e);
      bohrkern.value = null;
    }
  }
}

onMounted(() => {
  // Initialize schichtaufbau if it doesn't exist
  if (!currentUntersuchung.value.schichtaufbau) {
    currentUntersuchung.value.schichtaufbau = [];
  }
  // Ensure existing schichten have required structure
  currentUntersuchung.value.schichtaufbau.forEach((schicht: any, index: number) => {
    if (!Object.prototype.hasOwnProperty.call(schicht, 'bezeichnung')) schicht.bezeichnung = '';
    if (!Object.prototype.hasOwnProperty.call(schicht, 'beschreibung')) schicht.beschreibung = '';
    if (!Object.prototype.hasOwnProperty.call(schicht, 'id')) schicht.id = `schicht-${Date.now()}-${index}`;
    if (!Object.prototype.hasOwnProperty.call(schicht, 'tiefe_von')) schicht.tiefe_von = 0;
    if (!Object.prototype.hasOwnProperty.call(schicht, 'tiefe_bis')) schicht.tiefe_bis = schicht.tiefe_von || 0;
  });

  // Load linked entities
  hydrateLinkedState();
});



function createBohrkern() {
  if (!newBohrkern.value) return;

  store.createBohrkern(newBohrkern.value).then(response => {
    bohrkern.value = response;
    currentUntersuchung.value.bohrkern = bohrkern.value?.id;
    newBohrkern.value = '';
    return store.saveCurrentUntersuchung();
  }).catch(error => {
    feedbackStore.addError(getErrorMessage(error, 'Bohrkern konnte nicht erstellt werden'), 'Bohrkern');
  });

}

// Watch for changes in editInfo (when user clicks save in parent)
watch(() => isEnabled.value , async (newValue, oldValue) => {
  // When switching from edit mode to view mode (save was clicked)
  if (oldValue === true && newValue === false && bohrkern.value) {
    try {
      await apiConnector.patchxResource('bwd', 'Bohrkern', buildBohrkernPatchPayload(bohrkern.value), bohrkern.value.id);
    } catch (error) {
      feedbackStore.addError(getErrorMessage(error, 'Bohrkern konnte nicht gespeichert werden'), 'Bohrkern');
    }
  }
});

watch(() => currentUntersuchung.value.id, async (newId, oldId) => {
  if (!newId || newId === oldId) return;
  await hydrateLinkedState();
});

</script>
