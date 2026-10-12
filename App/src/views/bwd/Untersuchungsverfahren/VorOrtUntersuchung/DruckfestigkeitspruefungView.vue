<template>
  <GenericDetailView>
    <template v-slot:untersuchung="{ enabled }">
      <v-container>
        <!-- Probe Selection -->
        <v-row>
          <v-col cols="12" md="12">
            <v-sheet :elevation="5" border rounded>
              <v-toolbar :elevation="1">
                <v-toolbar-title>Zuordnung</v-toolbar-title>
                <v-spacer></v-spacer>
                <v-btn v-if="enabled" :prepend-icon="mdiFileEdit" @click="probeDialog = true">
                  Probe auswählen
                </v-btn>
              </v-toolbar>
              <v-container>
                <div v-if="store.currentUntersuchung.probe">
                  <v-row>
                    <v-col cols="12" md="6">
                      <v-card variant="outlined">
                        <v-card-title class="text-subtitle-1">Probe</v-card-title>
                        <v-card-text>
                          {{ probe?.probenbezeichnung || `Probe ID: ${store.currentUntersuchung.probe}` }}
                        </v-card-text>
                      </v-card>
                    </v-col>
                    <v-col cols="12" md="6">
                      <v-card variant="outlined">
                        <v-card-title class="text-subtitle-1">Bohrkern</v-card-title>
                        <v-card-text>
                          {{ relatedBohrkern?.name || 'Nicht zugeordnet' }}
                        </v-card-text>
                      </v-card>
                    </v-col>
                  </v-row>
                </div>
                <v-row v-else justify="center" align="center" class="pa-4">
                  <v-col cols="auto">
                    <v-chip color="warning" size="large">Keine Probe ausgewählt</v-chip>
                  </v-col>
                </v-row>
              </v-container>
            </v-sheet>
          </v-col>
        </v-row>

        <!-- Probe Selection Dialog -->
        <v-dialog v-model="probeDialog" max-width="600px">
          <v-card>
            <v-toolbar color="primary">
              <v-toolbar-title>Probe auswählen</v-toolbar-title>
              <v-spacer></v-spacer>
              <v-btn icon @click="probeDialog = false">
                <v-icon>{{ mdiClose }}</v-icon>
              </v-btn>
            </v-toolbar>
            <v-card-text class="pa-4">
              <v-alert v-if="proben.length === 0" type="info" variant="tonal">
                Für dieses Bauwerk sind keine Proben vorhanden.
              </v-alert>
              <v-row>
                <v-col v-for="availableProbe in proben" :key="availableProbe.id" cols="12" md="6">
                  <v-card
                    variant="outlined"
                    :class="{'bg-primary': availableProbe.id === store.currentUntersuchung.probe}"
                    @click="selectProbe(availableProbe)"
                    style="cursor: pointer"
                  >
                    <v-card-title>{{ availableProbe.probenbezeichnung }}</v-card-title>
                    <v-card-text>
                      <div v-if="availableProbe.bemerkung">Bemerkung: {{ availableProbe.bemerkung }}</div>
                      <div v-if="availableProbe.bohrkern">Bohrkern: {{ availableProbe.bohrkern }}</div>
                    </v-card-text>
                  </v-card>
                </v-col>
              </v-row>
            </v-card-text>
          </v-card>
        </v-dialog>

        <UntersuchungInfoCard :enabled="enabled" />

        <v-row>
          <v-col cols="12" md="12">
            <v-sheet :elevation="5" border rounded>
              <v-toolbar :elevation="1">
                <v-toolbar-title>Prüfung</v-toolbar-title>
              </v-toolbar>
              <v-container>
                <v-row>
                  <v-col cols="12" md="6">
                    <NumberField v-model="currentUntersuchung.pruefalter" integer label="Prüfalter" suffix="Tage"
                      min="0" :disabled="!enabled" />
                  </v-col>
                  <v-col cols="12" md="6">
                    <ChoiceSelect v-model="currentUntersuchung.lagerung" :items="lagerungOptions" label="Lagerung"
                      :disabled="!enabled" />
                  </v-col>
                </v-row>
                <v-row>
                  <v-col cols="12" md="6">
                    <NumberField v-model="currentUntersuchung.bruchlast" label="Bruchlast" suffix="kN" min="0"
                      :disabled="!enabled" />
                  </v-col>
                  <v-col cols="12" md="6">
                    <NumberField v-model="currentUntersuchung.druckfestigkeit" label="Druckfestigkeit"
                      suffix="N/mm²" min="0" :disabled="!enabled" />
                  </v-col>
                </v-row>
                <v-row>
                  <v-col cols="12" md="6">
                    <ChoiceSelect v-model="currentUntersuchung.bruchbild" :items="bruchbildOptions" label="Bruchbild"
                      :disabled="!enabled" />
                  </v-col>
                </v-row>
              </v-container>
            </v-sheet>
          </v-col>
        </v-row>

        <!-- Reports -->
        <v-row>
          <v-col cols="12" md="12">
            <v-sheet :elevation="5" border rounded>
              <v-toolbar :elevation="1">
                <v-toolbar-title>Prüfberichte</v-toolbar-title>
              </v-toolbar>
              <v-container>
                <SingleFileField v-model="currentUntersuchung.pruefbericht" label="Prüfbericht"
                  :enabled="enabled" :onUploadFile="uploadFile" :onDeleteFile="deleteFile" />
              </v-container>
            </v-sheet>
          </v-col>
        </v-row>

      </v-container>
    </template>
  </GenericDetailView>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { storeToRefs } from 'pinia';
import { mdiFileEdit, mdiClose } from '@mdi/js';
import { useUntersuchungenStore } from '@/store/bwd/UntersuchungsverfahrenStore';
import { useProbe, type Probe } from '@/composables/bwd/useProbe';

import GenericDetailView from '@/views/bwd/Untersuchungsverfahren/GenericDetailView.vue';
import UntersuchungInfoCard from '@/components/bwd/UntersuchungInfoCard.vue';
import NumberField from '@/components/bwd/NumberField.vue';
import ChoiceSelect, { type ChoiceOption } from '@/components/bwd/ChoiceSelect.vue';
import SingleFileField from '@/components/bwd/SingleFileField.vue';
import { apiConnector } from '@/utils/ApiConnector';

// The Prüfbericht upload sends the bauwerk of the Untersuchung.
// The Bemerkung is edited in the UntersuchungInfoCard only.

const store = useUntersuchungenStore();
const { currentUntersuchung } = storeToRefs(store);

const lagerungOptions: ChoiceOption[] = [
  { value: 'wasser', title: 'Wasserlagerung' },
  { value: 'luft', title: 'Luftlagerung' },
  { value: 'trocken', title: 'Trockenlagerung' },
];

const bruchbildOptions: ChoiceOption[] = [
  { value: 'regelmaessig', title: 'Regelmäßig' },
  { value: 'unregelmaessig', title: 'Unregelmäßig' },
];

const probeDialog = ref(false);
const proben = ref<Probe[]>([]);

const { probe, relatedBohrkern, loadProbesOptions, loadProbe } = useProbe(currentUntersuchung.value.probe);

async function loadProben() {
  try {
    // bauwerk_id is added from the route by the ApiConnector
    proben.value = await loadProbesOptions(currentUntersuchung.value.projekt ? String(currentUntersuchung.value.projekt) : undefined);
  } catch (error) {
    console.error('Error loading proben:', error);
  }
}

async function selectProbe(selectedProbe: Probe) {
  try {
    store.currentUntersuchung.probe = selectedProbe.id;
    if (selectedProbe.id !== undefined) {
      await loadProbe(selectedProbe.id);
    }
    probeDialog.value = false;
  } catch (error) {
    console.error('Error selecting probe:', error);
  }
}

onMounted(async () => {
  await loadProben();
});

// pruefbericht is a single media FK
async function uploadFile(img: any) {
  const data = {
    "beschreibung": img.beschreibung,
    "datei": img.datei,
    "model_name": currentUntersuchung.value.type,
    "model_pk": currentUntersuchung.value.id,
    "name": img.name,
    "bauwerk": currentUntersuchung.value.bauwerk,
    "type": img.type,
  }
  const resp = await apiConnector.uploadMedia("bwd", data);
  if (resp) {
    currentUntersuchung.value.pruefbericht = resp.id;
    await store.saveCurrentUntersuchung();
    return true;
  }
  return false;
}

async function deleteFile(file: { id: number, [key: string]: any }) {
  const resp = await apiConnector.deleteMedia(file.id);
  if (resp) {
    currentUntersuchung.value.pruefbericht = null;
    await store.saveCurrentUntersuchung();
    return true;
  }
  return false;
}
</script>
