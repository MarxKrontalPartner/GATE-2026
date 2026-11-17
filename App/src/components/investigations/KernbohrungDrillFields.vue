<template>
  <GateFormSection title="Bohrung" description="Messwerte der Bohrung – Zahlen manuell eingeben" :icon="mdiClipboardTextOutline">
    <v-row>
      <v-col cols="12" sm="6" lg="4"><NumberField :model-value="draft.durchmesser" @update:model-value="draft.durchmesser = $event ?? null" label="Durchmesser" suffix="mm"
        min="0" variant="outlined" :readonly="!editing" /></v-col>
      <v-col cols="12" sm="6" lg="4"><NumberField :model-value="draft.bohrlaenge" @update:model-value="draft.bohrlaenge = $event ?? null" label="Bohrlänge" suffix="mm"
        min="0" variant="outlined" :readonly="!editing" /></v-col>
      <v-col cols="12" sm="6" lg="4"><NumberField :model-value="draft.neigungswinkel" @update:model-value="draft.neigungswinkel = $event ?? null" label="Neigungswinkel" suffix="°"
        variant="outlined" hint="zur Horizontalen" persistent-hint :readonly="!editing" /></v-col>
      <v-col cols="12" sm="6"><ChoiceSelect :model-value="draft.kuehlung" @update:model-value="draft.kuehlung = $event ?? ''" label="Bohrverfahren / Kühlung"
        :items="kuehlungOptions" variant="outlined" :disabled="!editing" /></v-col>
      <v-col cols="12" sm="6"><v-text-field v-model="draft.verschlussmaterial" label="Verschlussmaterial"
        variant="outlined" :readonly="!editing" /></v-col>
      <v-col cols="12" sm="6"><v-checkbox v-model="draft.bewehrungAngeschnitten" label="Bewehrung angeschnitten"
        :disabled="!editing" color="primary" hide-details /></v-col>
      <v-col cols="12" sm="6"><v-checkbox v-model="draft.kernVollstaendig" label="Bohrkern vollständig"
        :disabled="!editing" color="primary" hide-details /></v-col>
    </v-row>
  </GateFormSection>
</template>
<script setup lang="ts">
import { mdiClipboardTextOutline } from '@mdi/js';
import GateFormSection from './GateFormSection.vue';
import NumberField from '@/components/bwd/NumberField.vue';
import ChoiceSelect, { type ChoiceOption } from '@/components/bwd/ChoiceSelect.vue';
import type { KernbohrungDraft } from '@/types/kernbohrung';
defineProps<{ draft: KernbohrungDraft; editing: boolean }>();
const kuehlungOptions: ChoiceOption[] = [
  { value: 'nass', title: 'Nassbohrung' },
  { value: 'trocken', title: 'Trockenbohrung' },
];
</script>
