<template>
  <v-sheet :elevation="5" border rounded>
    <v-toolbar :elevation="1">
      <v-toolbar-title>{{ title }}</v-toolbar-title>
    </v-toolbar>
    <v-container>
      <div v-if="risse.length === 0" class="text-center pa-4">
        <v-chip color="info" size="large">Keine Risse erfasst</v-chip>
      </div>

      <v-card v-for="(riss, index) in risse" :key="riss.id || `riss-${index}`" variant="outlined" class="pa-2 mb-3">
        <div class="d-flex align-center flex-wrap ga-1 mb-2">
          <div class="text-subtitle-2 font-weight-medium">Riss {{ index + 1 }}</div>
          <v-spacer />
          <template v-if="enabled">
            <v-btn :icon="mdiChevronUp" :disabled="index === 0" @click="moveRiss(index, -1)"
              color="primary" variant="text" size="small" title="Nach oben" />
            <v-btn :icon="mdiChevronDown" :disabled="index === risse.length - 1" @click="moveRiss(index, 1)"
              color="primary" variant="text" size="small" title="Nach unten" />
            <v-btn :icon="mdiDelete" @click="removeRiss(index)"
              color="error" variant="text" size="small" title="Riss entfernen" />
          </template>
        </div>

        <v-row dense>
          <v-col cols="12" sm="6" md="3">
            <v-text-field :model-value="riss.bezeichnung" @update:model-value="updateRiss(index, 'bezeichnung', $event)"
              label="Bezeichnung" placeholder="z.B. R1" :disabled="!enabled" density="compact" hide-details />
          </v-col>
          <v-col cols="6" sm="3" md="2">
            <NumberField :model-value="riss.breite" @update:model-value="updateRiss(index, 'breite', $event ?? null)"
              label="Breite" suffix="mm" min="0" :disabled="!enabled" density="compact" hide-details />
          </v-col>
          <v-col cols="6" sm="3" md="2">
            <NumberField :model-value="riss.laenge" @update:model-value="updateRiss(index, 'laenge', $event ?? null)"
              label="Länge" suffix="mm" min="0" :disabled="!enabled" density="compact" hide-details />
          </v-col>
          <v-col cols="12" sm="6" md="3">
            <ChoiceSelect :model-value="riss.verlauf" @update:model-value="updateRiss(index, 'verlauf', ($event ?? '') as RissVerlauf)"
              :items="verlaufOptions" label="Verlauf" :disabled="!enabled" density="compact" hide-details />
          </v-col>
          <v-col cols="12" sm="6" md="2" class="d-flex align-center">
            <v-checkbox :model-value="riss.wasserfuehrend" @update:model-value="updateRiss(index, 'wasserfuehrend', Boolean($event))"
              label="Wasserführend" :disabled="!enabled" density="compact" hide-details />
          </v-col>
        </v-row>
      </v-card>

      <div v-if="enabled" class="d-flex justify-end mt-2">
        <v-btn :prepend-icon="mdiPlus" @click="addRiss" variant="outlined" size="small">
          Riss hinzufügen
        </v-btn>
      </div>
    </v-container>
  </v-sheet>
</template>

<script setup lang="ts">
// Editable list of cracks (Rissbreitenmessung.risse), built like the SchichtenCard.
import { computed } from 'vue';
import { mdiPlus, mdiDelete, mdiChevronUp, mdiChevronDown } from '@mdi/js';

import NumberField from '@/components/bwd/NumberField.vue';
import ChoiceSelect, { type ChoiceOption } from '@/components/bwd/ChoiceSelect.vue';

export type RissVerlauf = '' | 'laengs' | 'quer' | 'schraeg' | 'netz';

export interface RissItem {
  id: string;
  bezeichnung: string;
  breite: number | null;
  laenge: number | null;
  verlauf: RissVerlauf;
  wasserfuehrend: boolean;
}

const props = withDefaults(defineProps<{
  modelValue?: RissItem[] | null;
  enabled: boolean;
  title?: string;
}>(), {
  modelValue: null,
  title: 'Risse',
});

const emit = defineEmits<{
  (e: 'update:modelValue', value: RissItem[]): void;
}>();

const verlaufOptions: ChoiceOption[] = [
  { value: 'laengs', title: 'Längsriss' },
  { value: 'quer', title: 'Querriss' },
  { value: 'schraeg', title: 'Schrägriss' },
  { value: 'netz', title: 'Netzriss' },
];

const risse = computed<RissItem[]>(() => props.modelValue ?? []);

function addRiss() {
  const newRiss: RissItem = {
    id: `riss-${Date.now()}-${Math.random().toString(36).slice(2, 9)}`,
    bezeichnung: `R${risse.value.length + 1}`,
    breite: null,
    laenge: null,
    verlauf: '',
    wasserfuehrend: false,
  };
  emit('update:modelValue', [...risse.value, newRiss]);
}

function removeRiss(index: number) {
  emit('update:modelValue', risse.value.filter((_, i) => i !== index));
}

function moveRiss(index: number, direction: -1 | 1) {
  const target = index + direction;
  if (target < 0 || target >= risse.value.length) return;
  const updated = [...risse.value];
  [updated[index], updated[target]] = [updated[target], updated[index]];
  emit('update:modelValue', updated);
}

function updateRiss<K extends keyof RissItem>(index: number, key: K, value: RissItem[K]) {
  emit('update:modelValue', risse.value.map((riss, i) => (i === index ? { ...riss, [key]: value } : riss)));
}
</script>
