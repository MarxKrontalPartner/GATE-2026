<template>
  <v-card class="gate-card gate-workspace-stelle" variant="flat">
    <v-card-text>
      <div class="gate-workspace-stelle__header">
        <div class="gate-workspace-stelle__symbol" aria-hidden="true">
          <v-icon :icon="mdiMapMarkerOutline" size="30" color="primary" />
        </div>
        <div class="gate-workspace-stelle__title">
          <div class="gate-workspace-stelle__eyebrow">Untersuchungsstelle {{ stelle.id }}</div>
          <h3>{{ stelle.name }}</h3>
          <p v-if="projectName" class="gate-workspace-muted">{{ projectName }}</p>
        </div>
      </div>
      <div class="gate-workspace-stelle__facts">
        <div class="gate-workspace-stelle__fact">
          <span>Bemerkung</span>
          <strong>{{ stelle.bemerkung || 'Keine Bemerkung' }}</strong>
        </div>
        <div v-if="hasPosition" class="gate-workspace-stelle__position">
          <span class="gate-workspace-stelle__section-label">Positionsangaben</span>
          <dl>
            <div v-if="stelle.lagenbezugHoehe || stelle.hoehe !== null && stelle.hoehe !== undefined">
              <dt>Höhe</dt>
              <dd>{{ positionValue(stelle.lagenbezugHoehe, stelle.hoehe) }}</dd>
            </div>
            <div v-if="stelle.lagenbezugAbstand1 || stelle.abstand1 !== null && stelle.abstand1 !== undefined">
              <dt>Abstand 1</dt>
              <dd>{{ positionValue(stelle.lagenbezugAbstand1, stelle.abstand1) }}</dd>
            </div>
            <div v-if="stelle.lagenbezugAbstand2 || stelle.abstand2 !== null && stelle.abstand2 !== undefined">
              <dt>Abstand 2</dt>
              <dd>{{ positionValue(stelle.lagenbezugAbstand2, stelle.abstand2) }}</dd>
            </div>
          </dl>
        </div>
      </div>
      <div class="gate-workspace-stelle__methods" aria-label="Untersuchungsverfahren dieser Stelle">
        <div class="gate-workspace-stelle__methods-heading">
          <h4>Untersuchungen</h4>
          <span v-if="!methodsLoading && !methodsError" class="gate-workspace-stelle__count">
            {{ methods.length }} gesamt · {{ completed }} erledigt
          </span>
        </div>
        <v-progress-linear v-if="methodsLoading" indeterminate color="primary" aria-label="Untersuchungen werden geladen" />
        <p v-else-if="methodsError" class="gate-workspace-muted">Untersuchungen momentan nicht verfügbar.</p>
        <p v-else-if="methods.length === 0" class="gate-workspace-muted">Noch keine Untersuchungen angelegt</p>
        <ul v-else class="gate-workspace-stelle__method-list">
          <li v-for="method in previewMethods" :key="`${method.category}-${method.type}-${method.id}`">
            <div class="gate-workspace-stelle__method-name">
              <strong>{{ getMethodDisplayName(method.type) }}</strong>
              <span>{{ method.category === 'Laboruntersuchung' ? 'Labor' : 'Vor Ort' }} · #{{ method.id }}</span>
            </div>
            <GateStatusChip :status="method.abgeschlossen ? 'completed' : 'in-progress'" size="small" />
          </li>
        </ul>
        <p v-if="!methodsLoading && !methodsError && moreMethods > 0" class="gate-workspace-stelle__more">
          + {{ moreMethods }} weitere Untersuchungen
        </p>
      </div>
      <GateActionButton intent="secondary" block :append-icon="mdiArrowRight" @click="$emit('open', stelle.id)">
        Untersuchungsstelle öffnen
      </GateActionButton>
    </v-card-text>
  </v-card>
</template>
<script setup lang="ts">
import { computed } from 'vue';
import { mdiArrowRight, mdiMapMarkerOutline } from '@mdi/js';
import type { IUntersuchungsstelle } from '@/types/apiInterfaces';
import type { StelleMethod } from '@/composables/bwd/stelleMethods';
import { GateActionButton, GateStatusChip } from '@/components/common';
import { getMethodDisplayName } from '@/utils/UntersuchungsverfahrenRegistry';

const props = defineProps<{
  stelle: IUntersuchungsstelle;
  projectName: string;
  methods: StelleMethod[];
  methodsLoading: boolean;
  methodsError: boolean;
}>();
defineEmits<{ open: [id: number] }>();
const previewMethods = computed(() => props.methods.slice(0, 3));
const moreMethods = computed(() => Math.max(props.methods.length - previewMethods.value.length, 0));
const completed = computed(() => props.methods.filter(method => method.abgeschlossen === true).length);
const hasPosition = computed(() => Boolean(
  props.stelle.lagenbezugHoehe || props.stelle.lagenbezugAbstand1 || props.stelle.lagenbezugAbstand2 ||
  [props.stelle.hoehe, props.stelle.abstand1, props.stelle.abstand2].some(value => value !== null && value !== undefined),
));
function positionValue(reference: string | null, distance: number | null): string {
  const parts = [reference?.trim(), distance === null || distance === undefined
    ? '' : new Intl.NumberFormat('de-DE', { maximumFractionDigits: 3 }).format(distance)];
  // The data contract does not specify units; don't invent 'm' or 'cm'.
  return parts.filter(Boolean).join(' · ');
}
</script>
