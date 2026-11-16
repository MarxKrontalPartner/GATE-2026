<template>
  <v-dialog :model-value="open" persistent max-width="860" scrollable aria-label="Untersuchungsstelle bearbeiten"
    @update:model-value="closeIfAllowed">
    <v-card class="gate-stelle-editor">
      <v-card-title>Untersuchungsstelle bearbeiten</v-card-title>
      <v-card-subtitle>Änderungen werden in der aktuell verwendeten Datenquelle gespeichert.</v-card-subtitle>
      <v-card-text>
        <v-form id="gate-stelle-form" @submit.prevent="submit">
          <v-row>
            <v-col cols="12"><v-text-field v-model="draft.name" label="Name *" variant="outlined" :disabled="saving" /></v-col>
            <v-col cols="12"><v-textarea v-model="draft.bemerkung" label="Bemerkung" variant="outlined" rows="2" :disabled="saving" /></v-col>
            <v-col cols="12" md="6">
              <v-text-field v-model="draft.lagenbezugHoehe" label="Lagenbezug Höhe" variant="outlined" :disabled="saving" />
            </v-col>
            <v-col cols="12" md="6">
              <v-text-field v-model="draft.hoehe" label="Höhe (Zahlenwert)" variant="outlined" inputmode="decimal" :disabled="saving" />
            </v-col>
            <v-col cols="12" md="6">
              <v-text-field v-model="draft.lagenbezugAbstand1" label="Lagenbezug Abstand 1" variant="outlined" :disabled="saving" />
            </v-col>
            <v-col cols="12" md="6">
              <v-text-field v-model="draft.abstand1" label="Abstand 1 (Zahlenwert)" variant="outlined" inputmode="decimal" :disabled="saving" />
            </v-col>
            <v-col cols="12" md="6">
              <v-text-field v-model="draft.lagenbezugAbstand2" label="Lagenbezug Abstand 2" variant="outlined" :disabled="saving" />
            </v-col>
            <v-col cols="12" md="6">
              <v-text-field v-model="draft.abstand2" label="Abstand 2 (Zahlenwert)" variant="outlined" inputmode="decimal" :disabled="saving" />
            </v-col>
          </v-row>
          <v-alert v-if="validationError" type="error" variant="tonal" class="mb-3">{{ validationError }}</v-alert>
        </v-form>
      </v-card-text>
      <v-card-actions class="gate-stelle-editor__actions">
        <GateActionButton intent="quiet" :disabled="saving" @click="$emit('close')">Abbrechen</GateActionButton>
        <GateActionButton intent="primary" :loading="saving" :disabled="saving" @click="submit">Speichern</GateActionButton>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>
<script setup lang="ts">
import { ref, watch } from 'vue';
import { GateActionButton } from '@/components/common';
import type { IUntersuchungsstelle } from '@/types/apiInterfaces';
import type { StelleEdits } from '@/composables/bwd/useUntersuchungsstelleDetail';
const props = defineProps<{ open: boolean; saving: boolean; stelle: IUntersuchungsstelle }>();
const emit = defineEmits<{ close: []; save: [changes: StelleEdits] }>();
type Draft = Record<keyof StelleEdits, string>;
const draft = ref<Draft>(emptyDraft());
const validationError = ref('');
function emptyDraft(): Draft {
  return { name: '', bemerkung: '', lagenbezugHoehe: '', hoehe: '',
    lagenbezugAbstand1: '', abstand1: '', lagenbezugAbstand2: '', abstand2: '' };
}
watch(() => [props.open, props.stelle] as const, () => {
  if (!props.open) return;
  draft.value = { name: props.stelle.name ?? '', bemerkung: props.stelle.bemerkung ?? '',
    lagenbezugHoehe: props.stelle.lagenbezugHoehe ?? '', hoehe: String(props.stelle.hoehe ?? ''),
    lagenbezugAbstand1: props.stelle.lagenbezugAbstand1 ?? '', abstand1: String(props.stelle.abstand1 ?? ''),
    lagenbezugAbstand2: props.stelle.lagenbezugAbstand2 ?? '', abstand2: String(props.stelle.abstand2 ?? '') };
  validationError.value = '';
}, { immediate: true });
function parseNumber(value: string): number | null | undefined {
  const trimmed = value.trim();
  if (!trimmed) return null;
  const numeric = Number(trimmed.replace(',', '.'));
  return Number.isFinite(numeric) ? numeric : undefined;
}
function submit() {
  if (props.saving) return;
  const name = draft.value.name.trim();
  const hoehe = parseNumber(draft.value.hoehe);
  const abstand1 = parseNumber(draft.value.abstand1);
  const abstand2 = parseNumber(draft.value.abstand2);
  if (!name) { validationError.value = 'Bitte einen Namen eingeben.'; return; }
  if ([hoehe, abstand1, abstand2].some(v => v === undefined)) {
    validationError.value = 'Bitte gültige Zahlenwerte für Höhe und Abstände eingeben.';
    return;
  }
  validationError.value = '';
  emit('save', { name, bemerkung: draft.value.bemerkung,
    lagenbezugHoehe: draft.value.lagenbezugHoehe, hoehe: hoehe!,
    lagenbezugAbstand1: draft.value.lagenbezugAbstand1, abstand1: abstand1!,
    lagenbezugAbstand2: draft.value.lagenbezugAbstand2, abstand2: abstand2! });
}
function closeIfAllowed(next: boolean) { if (!next && !props.saving) emit('close'); }
</script>
