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
                <div v-if="store.currentUntersuchung.probe && probe">
                  <v-row>
                    <v-col cols="12" md="6">
                      <v-card variant="outlined">
                        <v-card-title class="text-subtitle-1">Probe</v-card-title>
                        <v-card-text>
                          <div>{{ probe.probenbezeichnung }}</div>
                          <div v-if="probe.entnahmetiefeVon !== null || probe.entnahmetiefeBis !== null" class="text-caption">
                            Entnahmetiefe: {{ probe.entnahmetiefeVon ?? '?' }} – {{ probe.entnahmetiefeBis ?? '?' }} mm
                          </div>
                          <div v-if="probe.bemerkung" class="text-caption">{{ probe.bemerkung }}</div>
                        </v-card-text>
                      </v-card>
                    </v-col>
                    <v-col cols="12" md="6">
                      <v-card variant="outlined">
                        <v-card-title class="text-subtitle-1">Bohrkern</v-card-title>
                        <v-card-text>
                          {{ relatedBohrkern?.name || getBohrkernLabel() }}
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
              <v-alert v-if="probenList.length === 0" type="info" variant="tonal">
                Für dieses Bauwerk sind keine Proben vorhanden.
              </v-alert>
              <v-row>
                <v-col v-for="availableProbe in probenList" :key="availableProbe.id" cols="12" md="6">
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
                <v-toolbar-title>Analyse</v-toolbar-title>
              </v-toolbar>
              <v-container>
                <v-row>
                  <v-col cols="12" md="6">
                    <v-text-field v-model="currentUntersuchung.pruefstelle" label="Prüfstelle" :disabled="!enabled" />
                  </v-col>
                  <v-col cols="12" md="6">
                    <ChoiceSelect v-model="currentUntersuchung.analyseverfahren" :items="analyseverfahrenOptions"
                      label="Analyseverfahren" :disabled="!enabled" />
                  </v-col>
                </v-row>
                <v-row>
                  <v-col cols="12" md="6">
                    <NumberField v-model="currentUntersuchung.chloridgehaltZement" label="Chloridgehalt"
                      suffix="M.-%" hint="bezogen auf den Zementgehalt" persistent-hint min="0" :disabled="!enabled" />
                  </v-col>
                  <v-col cols="12" md="6">
                    <NumberField v-model="currentUntersuchung.zementgehaltAnnahme" label="Zementgehalt (Annahme)"
                      suffix="kg/m³" min="0" :disabled="!enabled" />
                  </v-col>
                </v-row>
                <v-row>
                  <v-col cols="12" md="6">
                    <v-checkbox v-model="currentUntersuchung.grenzwertUeberschritten" label="Grenzwert überschritten"
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
                <v-toolbar-title>Prüfberichte</v-toolbar-title>
              </v-toolbar>
              <v-container>
                <MultiFileField v-model="currentUntersuchung.pruefberichte" label="Prüfberichte" :enabled="enabled"
                  :onUploadFile="uploadFile" :onDeleteFile="deleteFile" />
              </v-container>
            </v-sheet>
          </v-col>
        </v-row>

      </v-container>



    </template>
  </GenericDetailView>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { storeToRefs } from 'pinia';

import { mdiFileEdit, mdiClose } from '@mdi/js';

import GenericDetailView from '@/views/bwd/Untersuchungsverfahren/GenericDetailView.vue';
import UntersuchungInfoCard from '@/components/bwd/UntersuchungInfoCard.vue';

import { useUntersuchungenStore } from '@/store/bwd/UntersuchungsverfahrenStore';
import { useProbe, type Probe } from '@/composables/bwd/useProbe';
import MultiFileField from '@/components/bwd/MultiFileField.vue';
import NumberField from '@/components/bwd/NumberField.vue';
import ChoiceSelect, { type ChoiceOption } from '@/components/bwd/ChoiceSelect.vue';
import { apiConnector } from '@/utils/ApiConnector';

// The Prüfbericht uploads send the bauwerk of the Untersuchung.

const store = useUntersuchungenStore();
const { currentUntersuchung } = storeToRefs(store);

const probeDialog = ref(false);

const { 
  probe, 
  relatedBohrkern, 
  loadProbe, 
  probenList,
  refreshProbenList 
} = useProbe();

const analyseverfahrenOptions: ChoiceOption[] = [
  { value: 'titration', title: 'Potentiometrische Titration' },
  { value: 'photometrie', title: 'Photometrie' },
  { value: 'schnelltest', title: 'Schnelltest vor Ort' },
];

function getBohrkernLabel() {
  if (!probe.value) {
    return 'Keine Probe ausgewählt';
  }
  if (probe.value.bohrkern) {
    return `Bohrkern ID: ${probe.value.bohrkern} (Lade...)`;
  }
  return 'Nicht zugeordnet';
}

async function selectProbe(selectedProbeItem: Probe) {
  try {
    if (!selectedProbeItem.id) return;
    store.currentUntersuchung.probe = selectedProbeItem.id;
    await loadProbe(selectedProbeItem.id);
    probeDialog.value = false;
  } catch (error) {
    console.error('Error selecting probe:', error);
  }
}


onMounted(async () => {
  // bauwerk_id is added from the route by the ApiConnector
  await refreshProbenList(currentUntersuchung.value.projekt ? String(currentUntersuchung.value.projekt) : undefined);
});

watch(() => store.currentUntersuchung.probe, async (newProbeId) => {
  if (newProbeId) {
    await loadProbe(newProbeId);
  } else {
    probe.value = null;
    relatedBohrkern.value = null;
  }
}, { immediate: true });


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
    if (!currentUntersuchung.value.pruefberichte) {
      currentUntersuchung.value.pruefberichte = [];
    }
    currentUntersuchung.value.pruefberichte.push(resp.id);

    await store.saveCurrentUntersuchung();
    return true;
  }
  return false; // Ensure boolean return
}

async function deleteFile(file: {id: number, [key: string]: any}) {
  const resp = await apiConnector.deleteMedia(file.id);
  if (resp) { // Assuming resp indicates success
    currentUntersuchung.value.pruefberichte = currentUntersuchung.value.pruefberichte.filter((item: number) => item !== file.id);
    await store.saveCurrentUntersuchung();
    return true;
  }
  return false; // Indicate failure
  
}

</script>
