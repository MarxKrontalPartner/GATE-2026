<template>
  <v-dialog :model-value="open" persistent max-width="820" scrollable
    aria-label="Untersuchungsstelle bearbeiten" @update:model-value="closeIfAllowed">
    <v-card class="gate-stelle-editor gate-form">
      <div class="gate-stelle-editor__header">
        <v-icon :icon="mdiMapMarkerOutline" size="34" color="primary" />
        <div>
          <h2>Untersuchungsstelle bearbeiten</h2>
          <p>Stelle {{ stelle.id }} · Angaben werden lokal gespeichert</p>
        </div>
      </div>
      <v-card-text class="gate-stelle-editor__body">
        <v-form ref="formRef" @submit.prevent="submit">
          <section class="gate-stelle-editor__section" aria-labelledby="stelle-grunddaten-heading">
            <h3 id="stelle-grunddaten-heading">Allgemeine Angaben</h3>
            <p>Kurze, gut erkennbare Bezeichnung für die Stelle und ergänzende Hinweise.</p>
            <v-text-field v-model="draft.name" label="Name *" variant="outlined" maxlength="200"
              :rules="[required]" :disabled="saving" placeholder="z. B. Pfeiler 2 – Nordseite" />
            <v-textarea v-model="draft.bemerkung" label="Bemerkung" variant="outlined"
              rows="3" auto-grow :disabled="saving" />
          </section>
          <section class="gate-stelle-editor__section" aria-labelledby="stelle-position-heading">
            <div class="gate-stelle-editor__section-heading">
              <v-icon :icon="mdiMapMarkerOutline" color="primary" size="26" />
              <h3 id="stelle-position-heading">Positionsangaben</h3>
              <v-chip size="small" variant="tonal">Optional</v-chip>
            </div>
            <p>Referenzpunkte am Bauwerk und die dazu gemessenen Werte. Einheiten nach Projektvorgabe.</p>
            <v-row>
              <v-col cols="12" md="7">
                <v-text-field v-model="draft.lagenbezugHoehe" label="Lagenbezug Höhe" variant="outlined"
                  :disabled="saving" placeholder="z. B. Fahrbahnoberkante" />
              </v-col>
              <v-col cols="12" md="5">
                <v-text-field v-model="draft.hoehe" label="Höhe (Zahlenwert)" variant="outlined"
                  inputmode="decimal" :disabled="saving" />
              </v-col>
              <v-col cols="12" md="7">
                <v-text-field v-model="draft.lagenbezugAbstand1" label="Lagenbezug Abstand 1" variant="outlined"
                  :disabled="saving" />
              </v-col>
              <v-col cols="12" md="5">
                <v-text-field v-model="draft.abstand1" label="Abstand 1 (Zahlenwert)" variant="outlined"
                  inputmode="decimal" :disabled="saving" />
              </v-col>
              <v-col cols="12" md="7">
                <v-text-field v-model="draft.lagenbezugAbstand2" label="Lagenbezug Abstand 2" variant="outlined"
                  :disabled="saving" />
              </v-col>
              <v-col cols="12" md="5">
                <v-text-field v-model="draft.abstand2" label="Abstand 2 (Zahlenwert)" variant="outlined"
                  inputmode="decimal" :disabled="saving" />
              </v-col>
            </v-row>
          </section>
          <v-alert v-if="validationError" type="error" variant="tonal" class="mb-3">{{ validationError }}</v-alert>
        </v-form>
      </v-card-text>
      <v-card-actions class="gate-stelle-editor__actions">
        <GateActionButton intent="quiet" :disabled="saving" @click="$emit('close')">Abbrechen</GateActionButton>
        <GateActionButton intent="primary" :prepend-icon="mdiContentSaveAll" :loading="saving" :disabled="saving" @click="submit">Änderungen speichern</GateActionButton>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>
<script setup lang="ts">
import { ref, watch } from 'vue';
import { mdiMapMarkerOutline, mdiContentSaveAll } from '@mdi/js';
import { GateActionButton } from '@/components/common';
import type { IUntersuchungsstelle } from '@/types/apiInterfaces';
import type { StelleEdits } from '@/composables/bwd/useUntersuchungsstelleDetail';
const props = defineProps<{ open: boolean; saving: boolean; stelle: IUntersuchungsstelle }>();
const emit = defineEmits<{ close: []; save: [changes: StelleEdits] }>();
type Draft = Record<keyof StelleEdits, string>;
const draft = ref<Draft>(emptyDraft());
const validationError = ref('');
const formRef = ref<{ validate: () => Promise<{ valid: boolean }> } | null>(null);
const required = (value: string) => Boolean(value?.trim()) || 'Bitte einen Namen eingeben.';
function emptyDraft(): Draft {
  return { name: '', bemerkung: '', lagenbezugHoehe: '', hoehe: '',
    lagenbezugAbstand1: '', abstand1: '', lagenbezugAbstand2: '', abstand2: '' };
}
watch(() => [props.open, props.stelle] as const, () => {
  if (!props.open) return;
  draft.value = {
    name: props.stelle.name ?? '', bemerkung: props.stelle.bemerkung ?? '',
    lagenbezugHoehe: props.stelle.lagenbezugHoehe ?? '', hoehe: String(props.stelle.hoehe ?? ''),
    lagenbezugAbstand1: props.stelle.lagenbezugAbstand1 ?? '', abstand1: String(props.stelle.abstand1 ?? ''),
    lagenbezugAbstand2: props.stelle.lagenbezugAbstand2 ?? '', abstand2: String(props.stelle.abstand2 ?? ''),
  };
  validationError.value = '';
}, { immediate: true });
function parseNumber(value: string): number | null | undefined {
  const text = value.trim();
  if (!text) return null;
  const result = Number(text.replace(',', '.'));
  return Number.isFinite(result) ? result : undefined;
}
async function submit() {
  if (props.saving) return;
  const validation = await formRef.value?.validate();
  if (!validation?.valid) return;
  const name = draft.value.name.trim();
  const hoehe = parseNumber(draft.value.hoehe);
  const abstand1 = parseNumber(draft.value.abstand1);
  const abstand2 = parseNumber(draft.value.abstand2);
  if (!name) { validationError.value = 'Bitte einen Namen eingeben.'; return; }
  if ([hoehe, abstand1, abstand2].some(value => value === undefined)) {
    validationError.value = 'Bitte gültige Zahlen für Höhe und Abstände eingeben.';
    return;
  }
  validationError.value = '';
  emit('save', {
    name, bemerkung: draft.value.bemerkung,
    lagenbezugHoehe: draft.value.lagenbezugHoehe, hoehe: hoehe!,
    lagenbezugAbstand1: draft.value.lagenbezugAbstand1, abstand1: abstand1!,
    lagenbezugAbstand2: draft.value.lagenbezugAbstand2, abstand2: abstand2!,
  });
}
function closeIfAllowed(next: boolean) { if (!next && !props.saving) emit('close'); }
</script>
