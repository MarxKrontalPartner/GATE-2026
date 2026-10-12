<template>
  <GenericDetailView>
    <template v-slot:untersuchung="{ enabled }">
      <v-container>

        <UntersuchungInfoCard :enabled="enabled" />

        <v-row>
          <v-col cols="12" md="12">
            <v-sheet :elevation="5" border rounded>
              <v-toolbar :elevation="1">
                <v-toolbar-title>Messung</v-toolbar-title>
              </v-toolbar>
              <v-container>
                <v-row>
                  <v-col cols="12" md="6">
                    <ChoiceSelect v-model="currentUntersuchung.messmittel" :items="messmittelOptions"
                      label="Messmittel" :disabled="!enabled" />
                  </v-col>
                  <v-col cols="12" md="6">
                    <NumberField :model-value="currentUntersuchung.maxRissbreite" label="Maximale Rissbreite"
                      suffix="mm" readonly :disabled="!enabled" hint="Wird aus den erfassten Rissen berechnet"
                      persistent-hint />
                  </v-col>
                </v-row>
                <v-row>
                  <v-col cols="12" md="6">
                    <v-checkbox v-model="currentUntersuchung.monitoringEmpfohlen" label="Monitoring empfohlen"
                      :disabled="!enabled" hide-details />
                  </v-col>
                </v-row>
              </v-container>
            </v-sheet>
          </v-col>
        </v-row>

        <v-row>
          <v-col cols="12">
            <RisseCard :model-value="currentUntersuchung.risse" @update:model-value="onRisseChange"
              :enabled="enabled" />
          </v-col>
        </v-row>

      </v-container>
    </template>
  </GenericDetailView>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia';

import GenericDetailView from '@/views/bwd/Untersuchungsverfahren/GenericDetailView.vue';
import UntersuchungInfoCard from '@/components/bwd/UntersuchungInfoCard.vue';
import ChoiceSelect, { type ChoiceOption } from '@/components/bwd/ChoiceSelect.vue';
import NumberField from '@/components/bwd/NumberField.vue';
import RisseCard, { type RissItem } from '@/components/bwd/RisseCard.vue';

import { useUntersuchungenStore } from '@/store/bwd/UntersuchungsverfahrenStore';
import { maximum } from '@/utils/messwerte';

const store = useUntersuchungenStore();
const { currentUntersuchung } = storeToRefs(store);

const messmittelOptions: ChoiceOption[] = [
  { value: 'risslupe', title: 'Risslupe' },
  { value: 'risskarte', title: 'Rissbreitenkarte' },
  { value: 'mikroskop', title: 'Digitales Mikroskop' },
];

// maxRissbreite is derived client-side and stored with the Untersuchung
function onRisseChange(risse: RissItem[]) {
  currentUntersuchung.value.risse = risse;
  currentUntersuchung.value.maxRissbreite = maximum(risse.map((riss) => riss.breite));
}
</script>
