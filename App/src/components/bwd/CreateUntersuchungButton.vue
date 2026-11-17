<template>
  <GateActionButton
    :intent="buttonVariant === 'text' ? 'quiet' : 'primary'"
    :prepend-icon="buttonPrependIcon ?? mdiPlus"
    :class="buttonClass"
    :disabled="!untersuchungsstelle.projekt"
    @click="openDialog"
  >{{ buttonLabel }}</GateActionButton>

  <v-dialog v-model="showDialog" persistent scrollable max-width="720" class="gate-investigation-create-dialog">
    <v-card class="gate-investigation-create gate-form">
      <v-card-title class="gate-investigation-create__title">
        <v-icon :icon="mdiClipboardTextOutline" color="primary" size="32" />
        <span>Neue Untersuchung</span>
      </v-card-title>
      <v-card-subtitle>Für Untersuchungsstelle „{{ untersuchungsstelle.name }}“ · lokal speichern</v-card-subtitle>
      <v-card-text>
        <v-form ref="formRef" @submit.prevent="createUntersuchung">
          <div class="gate-investigation-create__context">
            <v-icon :icon="mdiMapMarkerOutline" color="primary" size="24" />
            <div>
              <strong>{{ untersuchungsstelle.name }}</strong>
              <span>Projekt {{ untersuchungsstelle.projekt }} · Bauwerk {{ bauwerkId || '–' }}</span>
            </div>
          </div>
          <h3 class="gate-investigation-create__section">1. Untersuchungsverfahren</h3>
          <v-row>
            <v-col cols="12" sm="6">
              <v-select v-model="art" :items="artOptions" label="Art *" variant="outlined"
                :rules="[required]" :disabled="creating" @update:model-value="method = ''" />
            </v-col>
            <v-col cols="12" sm="6">
              <v-select v-model="method" :items="methodOptions" label="Verfahren *" variant="outlined"
                :rules="[required]" :disabled="!art || creating" />
            </v-col>
          </v-row>
          <h3 class="gate-investigation-create__section">2. Erste Angaben</h3>
          <v-row>
            <v-col cols="12" sm="6">
              <v-text-field v-model="datum" type="date" label="Untersuchungsdatum *" variant="outlined"
                :rules="[required]" :disabled="creating" />
            </v-col>
            <v-col cols="12" sm="6">
              <v-text-field v-model="zeit" type="time" label="Uhrzeit (optional)" variant="outlined" :disabled="creating" />
            </v-col>
            <v-col cols="12">
              <v-textarea v-model="bemerkung" label="Bemerkung (optional)" variant="outlined"
                rows="3" auto-grow :disabled="creating" />
            </v-col>
          </v-row>
          <v-alert type="info" variant="tonal" class="mb-4">
            Nach dem Anlegen können Sie die Messwerte, Schichten und Bohrkern-Angaben im Verfahren ergänzen.
            Die Daten werden auf diesem Gerät gespeichert.
          </v-alert>
          <v-alert v-if="error" type="error" variant="tonal" class="mb-3">{{ error }}</v-alert>
        </v-form>
      </v-card-text>
      <v-card-actions class="gate-investigation-create__actions">
        <GateActionButton intent="quiet" :disabled="creating" @click="closeDialog">Abbrechen</GateActionButton>
        <GateActionButton intent="primary" :loading="creating" :disabled="creating || !art || !method"
          :prepend-icon="mdiPlus" @click="createUntersuchung">Untersuchung anlegen</GateActionButton>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { useRoute } from 'vue-router';
import { mdiPlus, mdiClipboardTextOutline, mdiMapMarkerOutline } from '@mdi/js';
import { GateActionButton } from '@/components/common';
import { useUntersuchungenStore } from '@/store/bwd/UntersuchungsverfahrenStore';
import { getAllKnownMethods, getMethodDisplayName, type UntersuchungsverfahrenArt } from '@/utils/UntersuchungsverfahrenRegistry';
import { getErrorMessage } from '@/utils/errors';
import { buildInvestigationCreatePayload } from '@/utils/investigationCreatePayload';

interface UntersuchungsstelleInfo { id: number; name: string; projekt: number | null }
const props = withDefaults(defineProps<{
  untersuchungsstelle: UntersuchungsstelleInfo;
  buttonVariant?: 'flat' | 'text' | 'elevated' | 'tonal' | 'outlined' | 'plain';
  buttonColor?: string;
  buttonSize?: 'x-small' | 'small' | 'default' | 'large' | 'x-large';
  buttonDensity?: 'default' | 'comfortable' | 'compact';
  buttonRounded?: string | number | boolean;
  buttonClass?: string;
  buttonLabel?: string;
  buttonPrependIcon?: string;
}>(), { buttonLabel: 'Neue Untersuchung' });
const emit = defineEmits<{
  created: [untersuchung: { id: number }, art: UntersuchungsverfahrenArt, untersuchungsverfahren: string];
}>();
const route = useRoute();
const store = useUntersuchungenStore();
const showDialog = ref(false);
const creating = ref(false);
const error = ref('');
const art = ref<UntersuchungsverfahrenArt | ''>('');
const method = ref('');
const datum = ref('');
const zeit = ref('');
const bemerkung = ref('');
const formRef = ref<{ validate: () => Promise<{ valid: boolean }> } | null>(null);
const bauwerkId = computed(() => Number(route.params.bauwerkId));
const required = (value: string) => Boolean(value?.trim()) || 'Dieses Feld ist erforderlich.';
const artOptions = [
  { title: 'Vor-Ort-Untersuchung', value: 'VorOrtUntersuchung' },
  { title: 'Laboruntersuchung', value: 'Laboruntersuchung' },
];
// The registry is also available offline; no Django request is required to open the dialog.
const methodOptions = computed(() => getAllKnownMethods()
  .filter(item => item.art === art.value)
  .map(item => ({ title: getMethodDisplayName(item.slug), value: item.slug })));
function todayLocal() {
  const now = new Date();
  const pad = (value: number) => String(value).padStart(2, '0');
  return `${now.getFullYear()}-${pad(now.getMonth() + 1)}-${pad(now.getDate())}`;
}
function openDialog() {
  art.value = 'VorOrtUntersuchung';
  method.value = 'kernbohrung';
  datum.value = todayLocal();
  zeit.value = '';
  bemerkung.value = '';
  error.value = '';
  showDialog.value = true;
}
function closeDialog() { if (!creating.value) showDialog.value = false; }
async function createUntersuchung() {
  if (creating.value) return;
  error.value = '';
  const validation = await formRef.value?.validate();
  if (!validation?.valid || !art.value || !method.value) return;
  if (!props.untersuchungsstelle.projekt || !Number.isSafeInteger(bauwerkId.value) || bauwerkId.value <= 0) {
    error.value = 'Für das Anlegen werden Bauwerk, Projekt und Untersuchungsstelle benötigt.';
    return;
  }
  creating.value = true;
  try {
    const selectedArt = art.value;
    const selectedMethod = method.value;
    const payload = buildInvestigationCreatePayload({
      bauwerkId: bauwerkId.value, projektId: props.untersuchungsstelle.projekt,
      stelleId: props.untersuchungsstelle.id, art: selectedArt, method: selectedMethod,
      datum: datum.value, zeit: zeit.value, bemerkung: bemerkung.value,
    });
    const created = await store.createUntersuchung(selectedArt, selectedMethod, payload);
    if (!Number.isSafeInteger(Number(created?.id)) || Number(created.id) <= 0) {
      throw new Error('Die Untersuchung wurde nicht mit einer gültigen ID zurückgegeben.');
    }
    showDialog.value = false;
    emit('created', { id: Number(created.id) }, selectedArt, selectedMethod);
  } catch (cause) {
    error.value = getErrorMessage(cause, 'Untersuchung konnte nicht angelegt werden.');
  } finally {
    creating.value = false;
  }
}
</script>

