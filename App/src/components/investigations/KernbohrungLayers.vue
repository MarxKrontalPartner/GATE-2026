<template>
  <GateFormSection title="Schichtaufbau" description="Schichten von oben nach unten, Tiefen in Millimetern" :icon="mdiFormatListBulleted">
    <v-alert v-if="draft.schichtaufbau.length === 0" type="info" variant="tonal" class="mb-4">
      Noch keine Schichten erfasst.
    </v-alert>
    <p v-if="editing" class="gate-kernbohrung-layers__hint">
      Bei einer neuen Schicht wird „Tiefe von“ aus der vorherigen Endtiefe vorgeschlagen.
      Beide Tiefen können unabhängig bearbeitet werden; vorhandene Werte bleiben unverändert.
    </p>
    <div class="gate-kernbohrung-layers">
      <v-card v-for="(layer, index) in draft.schichtaufbau" :key="layer.id || index" variant="outlined" class="gate-kernbohrung-layer">
        <v-card-text>
          <div class="gate-kernbohrung-layer__heading">
            <strong>Schicht {{ index + 1 }}</strong>
            <GateActionButton v-if="editing" intent="danger" :prepend-icon="mdiDelete" @click="remove(index)">Entfernen</GateActionButton>
          </div>
          <v-row>
            <v-col cols="12" md="6"><v-text-field v-model="layer.bezeichnung" label="Bezeichnung" variant="outlined" :readonly="!editing" /></v-col>
            <v-col cols="12" md="6"><v-text-field v-model="layer.beschreibung" label="Beschreibung" variant="outlined" :readonly="!editing" /></v-col>
            <v-col cols="12" sm="6"><NumberField v-model="layer.tiefe_von" label="Tiefe von" suffix="mm"
              variant="outlined" min="0" placeholder="Starttiefe" :readonly="!editing" /></v-col>
            <v-col cols="12" sm="6"><NumberField v-model="layer.tiefe_bis" label="Tiefe bis" suffix="mm"
              variant="outlined" min="0" placeholder="Endtiefe eingeben" :readonly="!editing" /></v-col>
          </v-row>
        </v-card-text>
      </v-card>
    </div>
    <GateActionButton v-if="editing" intent="secondary" :prepend-icon="mdiPlus" class="mt-4" @click="add">Schicht hinzufügen</GateActionButton>
  </GateFormSection>
</template>
<script setup lang="ts">
import { mdiDelete, mdiFormatListBulleted, mdiPlus } from '@mdi/js';
import { GateActionButton } from '@/components/common';
import GateFormSection from './GateFormSection.vue';
import NumberField from '@/components/bwd/NumberField.vue';
import { appendKernbohrungLayer, removeKernbohrungLayer } from '@/utils/kernbohrungLayers';
import type { KernbohrungDraft } from '@/types/kernbohrung';
const props = defineProps<{ draft: KernbohrungDraft; editing: boolean }>();
function add() { props.draft.schichtaufbau = appendKernbohrungLayer(props.draft.schichtaufbau); }
function remove(index: number) { props.draft.schichtaufbau = removeKernbohrungLayer(props.draft.schichtaufbau, index); }
</script>
