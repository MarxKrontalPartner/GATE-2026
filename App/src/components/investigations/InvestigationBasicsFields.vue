<template>
  <GateFormSection title="Allgemeine Angaben" description="Aufnahmedatum, Umgebungsbedingungen und Bearbeitungsstand"
    :icon="mdiClipboardTextOutline">
    <v-row>
      <v-col cols="12" sm="6"><v-text-field v-model="draft.datum" type="date" label="Datum *" variant="outlined" :readonly="!editing" /></v-col>
      <v-col cols="12" sm="6"><v-text-field v-model="draft.zeit" type="time" label="Uhrzeit" variant="outlined" :readonly="!editing" /></v-col>
      <v-col cols="12" sm="6"><NumberField :model-value="draft.lufttemperatur" @update:model-value="draft.lufttemperatur = $event ?? null" label="Lufttemperatur" suffix="°C" variant="outlined" :readonly="!editing" /></v-col>
      <v-col cols="12" sm="6"><NumberField :model-value="draft.luftfeuchte" @update:model-value="draft.luftfeuchte = $event ?? null" label="Relative Luftfeuchtigkeit" suffix="%" variant="outlined" :readonly="!editing" /></v-col>
      <v-col cols="12"><v-textarea v-model="draft.bemerkung" label="Bemerkung" variant="outlined" rows="3" auto-grow :readonly="!editing" /></v-col>
      <v-col cols="12">
        <v-autocomplete v-model="draft.bearbeiter" :items="people" :item-title="personName" item-value="id"
          label="Bearbeiter" variant="outlined" chips multiple
          :readonly="!editing || allowPersonnelEdit === false || people.length === 0"
          :loading="peopleLoading" no-data-text="Keine Bearbeiter vorhanden" />
        <v-alert v-if="peopleWarning" type="info" variant="tonal" density="compact" class="mt-2">{{ peopleWarning }}</v-alert>
        <v-btn v-if="showPeopleRefresh" variant="text" color="primary" class="mt-2"
          :loading="peopleLoading" :disabled="peopleLoading" @click="$emit('refreshPeople')">Bearbeiter aktualisieren</v-btn>
      </v-col>
      <v-col cols="12" class="gate-investigation-basics__status">
        <v-checkbox v-model="draft.abgeschlossen" label="Untersuchung abgeschlossen" color="success" hide-details :disabled="!editing" />
        <GateStatusChip :status="draft.abgeschlossen ? 'completed' : 'in-progress'" />
      </v-col>
    </v-row>
  </GateFormSection>
</template>
<script setup lang="ts">
import { mdiClipboardTextOutline } from '@mdi/js';
import GateFormSection from './GateFormSection.vue';
import NumberField from '@/components/bwd/NumberField.vue';
import { GateStatusChip } from '@/components/common';
import type { InvestigationBasicsDraft } from '@/types/investigationBasics';
import type { IPerson } from '@/types/apiInterfaces';
defineProps<{ draft: InvestigationBasicsDraft; editing: boolean; people: IPerson[]; allowPersonnelEdit?: boolean;
  peopleLoading?: boolean; peopleWarning?: string; showPeopleRefresh?: boolean }>();
defineEmits<{ refreshPeople: [] }>();
function personName(person: IPerson): string { return `${person.vorname || ''} ${person.name || ''}`.trim(); }
</script>
