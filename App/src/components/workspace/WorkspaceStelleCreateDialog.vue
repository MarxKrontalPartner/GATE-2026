<template>
  <v-dialog :model-value="open" persistent scrollable max-width="620"
    aria-label="Untersuchungsstelle hinzufügen" @update:model-value="onDialogChange">
    <v-card class="gate-workspace-create-dialog">
      <v-card-title>Neue Untersuchungsstelle</v-card-title>
      <v-card-subtitle>Für dieses Bauwerk anlegen · lokal speicherbar</v-card-subtitle>
      <v-card-text>
        <v-form ref="formRef" @submit.prevent="submit">
          <v-text-field v-model="name" label="Name *" variant="outlined"
            autofocus :disabled="saving" :rules="[required]" maxlength="200" counter
            placeholder="z. B. Pfeiler 2 – Nordseite" />
          <v-select v-if="projekte.length > 1" v-model="selectedProjectId"
            :items="projectOptions" item-title="title" item-value="value"
            label="Projekt *" variant="outlined" :disabled="saving"
            :rules="[projectRequired]" />
          <div v-else-if="projekte.length === 1" class="gate-workspace-create-dialog__project">
            <span>Projekt</span>
            <strong>{{ projectOptions[0]?.title }}</strong>
          </div>
          <v-alert v-else type="warning" variant="tonal" class="mb-3">
            Diesem Bauwerk ist kein Projekt zugeordnet. Eine neue Untersuchungsstelle benötigt ein Projekt.
          </v-alert>
          <v-textarea v-model="bemerkung" label="Bemerkung (optional)"
            variant="outlined" rows="3" auto-grow :disabled="saving" />
          <v-alert v-if="error" type="error" variant="tonal" class="mb-3">{{ error }}</v-alert>
        </v-form>
      </v-card-text>
      <v-card-actions class="gate-workspace-create-dialog__actions">
        <GateActionButton intent="quiet" :disabled="saving" @click="$emit('close')">Abbrechen</GateActionButton>
        <GateActionButton intent="primary" :loading="saving" :disabled="saving || !projekte.length"
          :prepend-icon="mdiPlus" @click="submit">Hinzufügen</GateActionButton>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>
<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { mdiPlus } from '@mdi/js';
import type { IProjekt } from '@/types/apiInterfaces';
import { GateActionButton } from '@/components/common';
import { getErrorMessage } from '@/utils/errors';
import { useWorkspaceStelleCreate } from '@/composables/bwd/useWorkspaceStelleCreate';

const props = withDefaults(defineProps<{
  open: boolean;
  bauwerkId: number;
  projekte: IProjekt[];
  projektId?: number | null;
}>(), { projektId: null });
const emit = defineEmits<{ close: []; created: [] }>();
const { saving, create } = useWorkspaceStelleCreate();
const formRef = ref<{ validate: () => Promise<{ valid: boolean }> } | null>(null);
const name = ref('');
const bemerkung = ref('');
const selectedProjectId = ref<number | null>(null);
const error = ref('');
const projectOptions = computed(() => props.projekte.map(project => ({
  value: project.id,
  title: `${project.projektnummer} · ${project.name}`,
})));
const required = (value: string) => Boolean(value?.trim()) || 'Name ist erforderlich.';
const projectRequired = (value: number | null) => value !== null || 'Projekt ist erforderlich.';

watch(() => props.open, open => {
  if (!open) return;
  name.value = '';
  bemerkung.value = '';
  selectedProjectId.value = props.projektId !== null && props.projekte.some(p => p.id === props.projektId)
    ? props.projektId : props.projekte.length === 1 ? props.projekte[0]!.id : null;
  error.value = '';
});
function onDialogChange(open: boolean) { if (!open && !saving.value) emit('close'); }
async function submit() {
  if (saving.value) return;
  error.value = '';
  const validation = await formRef.value?.validate();
  if (!validation?.valid || selectedProjectId.value === null) {
    error.value = 'Bitte Name und Projekt angeben.';
    return;
  }
  try {
    await create({ name: name.value, bemerkung: bemerkung.value, projekt: selectedProjectId.value },
      props.bauwerkId, props.projekte.map(p => p.id));
    emit('close');
    emit('created');
  } catch (cause) {
    error.value = getErrorMessage(cause, 'Untersuchungsstelle konnte nicht gespeichert werden.');
  }
}
</script>
