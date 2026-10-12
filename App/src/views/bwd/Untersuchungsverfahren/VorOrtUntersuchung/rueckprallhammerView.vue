<template>
  <GenericDetailView>
    <template v-slot:untersuchung="{ enabled }">
      <v-container>

        <UntersuchungInfoCard :enabled="enabled" />

        <v-row>
          <v-col cols="12" md="12">
            <v-sheet :elevation="5" border rounded>
              <v-toolbar :elevation="1">
                <v-toolbar-title>Messgerät</v-toolbar-title>
              </v-toolbar>
              <v-container>
                <v-row>
                  <v-col cols="12" md="6">
                    <ChoiceSelect v-model="currentUntersuchung.hammertyp" :items="hammertypOptions" label="Hammertyp"
                      :disabled="!enabled" />
                  </v-col>
                  <v-col cols="12" md="6">
                    <ChoiceSelect v-model="currentUntersuchung.schlagrichtung" :items="schlagrichtungOptions"
                      label="Schlagrichtung" :disabled="!enabled" />
                  </v-col>
                </v-row>
                <v-row>
                  <v-col cols="12" md="6">
                    <v-text-field :model-value="currentUntersuchung.kalibriertAm ?? ''"
                      @update:model-value="currentUntersuchung.kalibriertAm = $event || null"
                      label="Kalibriert am" type="date" :disabled="!enabled" />
                  </v-col>
                  <v-col cols="12" md="6">
                    <v-checkbox v-model="currentUntersuchung.oberflaecheGeschliffen" label="Oberfläche geschliffen"
                      :disabled="!enabled" hide-details />
                  </v-col>
                </v-row>
              </v-container>
            </v-sheet>
          </v-col>
        </v-row>

        <v-row>
          <v-col cols="12" md="12">
            <v-sheet :elevation="5" border rounded>
              <v-toolbar :elevation="1">
                <v-toolbar-title>Messreihe</v-toolbar-title>
              </v-toolbar>
              <v-container>
                <v-row>
                  <v-col cols="12" sm="6">
                    <NumberField :model-value="currentUntersuchung.medianRueckprallwert" label="Median Rückprallwert"
                      readonly :disabled="!enabled" hint="Wird aus der Messreihe berechnet" persistent-hint />
                  </v-col>
                  <v-col cols="12" sm="6" class="d-flex align-center">
                    <v-chip color="primary" variant="tonal">{{ messreihe.length }} {{ messreihe.length === 1 ? 'Wert' : 'Werte' }}</v-chip>
                  </v-col>
                </v-row>

                <v-row dense class="mt-2">
                  <v-col v-for="(wert, index) in messreihe" :key="index" cols="6" sm="4" md="3" lg="2">
                    <NumberField :model-value="wert" @update:model-value="updateMesswert(index, $event ?? null)"
                      :label="`#${index + 1}`" :disabled="!enabled" density="compact" hide-details>
                      <template v-if="enabled" #append-inner>
                        <v-icon :icon="mdiClose" size="small" title="Wert entfernen" @click="removeMesswert(index)" />
                      </template>
                    </NumberField>
                  </v-col>
                </v-row>
                <div v-if="messreihe.length === 0" class="text-center pa-4">
                  <v-chip color="info" size="large">Keine Messwerte erfasst</v-chip>
                </div>

                <v-row v-if="enabled" class="mt-2" justify="end">
                  <v-col cols="12" sm="6" md="4">
                    <v-form @submit.prevent="addMesswert">
                      <NumberField v-model="neuerMesswert" label="Neuer Messwert" density="compact" hide-details>
                        <template #append>
                          <v-btn :icon="mdiPlus" color="primary" variant="tonal" size="small"
                            :disabled="neuerMesswert === null" title="Messwert hinzufügen" @click="addMesswert" />
                        </template>
                      </NumberField>
                    </v-form>
                  </v-col>
                </v-row>
              </v-container>
            </v-sheet>
          </v-col>
        </v-row>

        <v-row>
          <v-col cols="12" md="12">
            <v-sheet :elevation="5" border rounded>
              <v-toolbar :elevation="1">
                <v-toolbar-title>Messprotokolle</v-toolbar-title>
              </v-toolbar>
              <v-container>
                <MultiFileField v-model="currentUntersuchung.messprotokolle" label="Messprotokolle" :enabled="enabled"
                  :onUploadFile="uploadMessprotokoll" :onDeleteFile="deleteMessprotokoll" />
              </v-container>
            </v-sheet>
          </v-col>
        </v-row>

      </v-container>



    </template>
  </GenericDetailView>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { storeToRefs } from 'pinia';
import { mdiClose, mdiPlus } from '@mdi/js';

import GenericDetailView from '@/views/bwd/Untersuchungsverfahren/GenericDetailView.vue';
import UntersuchungInfoCard from '@/components/bwd/UntersuchungInfoCard.vue';
import MultiFileField from '@/components/bwd/MultiFileField.vue';
import NumberField from '@/components/bwd/NumberField.vue';
import ChoiceSelect, { type ChoiceOption } from '@/components/bwd/ChoiceSelect.vue';
import { useUntersuchungenStore } from '@/store/bwd/UntersuchungsverfahrenStore';
import { apiConnector } from '@/utils/ApiConnector';
import { median } from '@/utils/messwerte';

// The uploads send the bauwerk of the Untersuchung.

const store = useUntersuchungenStore();
const { currentUntersuchung } = storeToRefs(store);

const hammertypOptions: ChoiceOption[] = [
  { value: 'N', title: 'Typ N' },
  { value: 'L', title: 'Typ L' },
  { value: 'pendel', title: 'Pendelhammer' },
];

const schlagrichtungOptions: ChoiceOption[] = [
  { value: 'horizontal', title: 'Horizontal' },
  { value: 'oben', title: 'Senkrecht nach oben' },
  { value: 'unten', title: 'Senkrecht nach unten' },
];

const messreihe = computed<(number | null)[]>(() =>
  Array.isArray(currentUntersuchung.value.messreihe) ? currentUntersuchung.value.messreihe : []);
const neuerMesswert = ref<number | null>(null);

// medianRueckprallwert is derived client-side and stored with the Untersuchung
function setMessreihe(values: (number | null)[]) {
  currentUntersuchung.value.messreihe = values;
  currentUntersuchung.value.medianRueckprallwert = median(values);
}

function addMesswert() {
  if (neuerMesswert.value === null) return;
  setMessreihe([...messreihe.value, neuerMesswert.value]);
  neuerMesswert.value = null;
}

function updateMesswert(index: number, value: number | null) {
  setMessreihe(messreihe.value.map((wert, i) => (i === index ? value : wert)));
}

function removeMesswert(index: number) {
  setMessreihe(messreihe.value.filter((_, i) => i !== index));
}

async function uploadMessprotokoll(file: any) {
  const data = {
    "beschreibung": file.beschreibung,
    "datei": file.datei,
    "model_name": currentUntersuchung.value.type,
    "model_pk": currentUntersuchung.value.id,
    "name": file.name,
    "bauwerk": currentUntersuchung.value.bauwerk,
    "type": file.type,
  }
  const resp = await apiConnector.uploadMedia("bwd", data);
  if (resp) {
    if (!currentUntersuchung.value.messprotokolle) {
      currentUntersuchung.value.messprotokolle = [];
    }
    currentUntersuchung.value.messprotokolle.push(resp.id);

    await store.saveCurrentUntersuchung();
    return true;
  }
  return false;
}

async function deleteMessprotokoll(file: { id: number, [key: string]: any }) {
  const resp = await apiConnector.deleteMedia(file.id);
  if (resp) {
    currentUntersuchung.value.messprotokolle = currentUntersuchung.value.messprotokolle.filter((item: number) => item !== file.id);
    await store.saveCurrentUntersuchung();
    return true;
  }
  return false;
}
</script>
