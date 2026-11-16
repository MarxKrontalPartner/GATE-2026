<template>
  <section class="gate-workspace-section" aria-labelledby="gate-stelle-methods-heading">
    <div class="gate-workspace-section__heading">
      <div>
        <h2 id="gate-stelle-methods-heading">Untersuchungsverfahren ({{ methods.length }})</h2>
        <p>{{ completed }} abgeschlossen · {{ methods.length - completed }} in Arbeit</p>
      </div>
      <CreateUntersuchungButton v-if="stelle.projekt !== null" :untersuchungsstelle="stelle"
        button-size="large" button-density="default" button-variant="flat"
        button-class="gate-stelle-create-method" @created="$emit('created')" />
    </div>
    <v-alert v-if="stelle.projekt === null" type="warning" variant="tonal">
      Für neue Untersuchungsverfahren muss die Stelle einem Projekt zugeordnet sein.
      Die Projektzuordnung kann im Arbeitsbereich bearbeitet werden.
    </v-alert>
    <v-alert v-if="methods.length === 0" type="info" variant="tonal">
      Noch keine Untersuchungsverfahren für diese Stelle gespeichert. Über „Neue Untersuchung“ können Sie eine anlegen.
    </v-alert>
    <div v-else class="gate-stelle-method-grid">
      <v-card v-for="method in methods" :key="`${method.category}-${method.type}-${method.id}`"
        class="gate-card gate-stelle-method" variant="flat">
        <v-card-text>
          <div class="gate-stelle-method__top">
            <v-icon :icon="mdiClipboardTextOutline" size="30" color="primary" />
            <GateStatusChip :status="method.abgeschlossen ? 'completed' : 'in-progress'" />
          </div>
          <h3>{{ getMethodDisplayName(method.type) }}</h3>
          <p class="gate-workspace-muted">{{ method.category === 'Laboruntersuchung' ? 'Laboruntersuchung' : 'Vor-Ort-Untersuchung' }} · ID {{ method.id }}</p>
          <p class="gate-workspace-muted">Datum: {{ formatDate(method.datum) }}</p>
          <GateActionButton block intent="secondary" :append-icon="mdiArrowRight"
            @click="$emit('open', method)">Verfahren öffnen</GateActionButton>
        </v-card-text>
      </v-card>
    </div>
  </section>
</template>
<script setup lang="ts">
import { mdiArrowRight, mdiClipboardTextOutline } from '@mdi/js';
import { computed } from 'vue';
import CreateUntersuchungButton from '@/components/bwd/CreateUntersuchungButton.vue';
import { GateActionButton, GateStatusChip } from '@/components/common';
import type { IUntersuchungsstelle } from '@/types/apiInterfaces';
import type { StelleMethod } from '@/composables/bwd/stelleMethods';
import { getMethodDisplayName } from '@/utils/UntersuchungsverfahrenRegistry';
const props = defineProps<{ stelle: IUntersuchungsstelle; methods: StelleMethod[] }>();
defineEmits<{ created: []; open: [method: StelleMethod] }>();
const completed = computed(() => props.methods.filter(m => m.abgeschlossen === true).length);
function formatDate(value: string | null): string {
  if (!value) return 'Nicht angegeben';
  const date = new Date(`${value.slice(0, 10)}T12:00:00`);
  return Number.isNaN(date.getTime()) ? value : date.toLocaleDateString('de-DE');
}
</script>
