<template>
  <v-card class="gate-card gate-workspace-info" variant="flat">
    <v-card-title>Angaben zur Untersuchungsstelle</v-card-title>
    <v-card-text>
      <dl class="gate-workspace-facts gate-stelle-facts">
        <div><dt>Stellen-ID</dt><dd>{{ stelle.id }}</dd></div>
        <div><dt>Projekt</dt><dd>{{ projectName }}</dd></div>
        <div><dt>Lagenbezug Höhe</dt><dd>{{ stelle.lagenbezugHoehe || 'Nicht angegeben' }}</dd></div>
        <div><dt>Höhe</dt><dd>{{ distance(stelle.hoehe) }}</dd></div>
        <div><dt>Lagenbezug Abstand 1</dt><dd>{{ stelle.lagenbezugAbstand1 || 'Nicht angegeben' }}</dd></div>
        <div><dt>Abstand 1</dt><dd>{{ distance(stelle.abstand1) }}</dd></div>
        <div><dt>Lagenbezug Abstand 2</dt><dd>{{ stelle.lagenbezugAbstand2 || 'Nicht angegeben' }}</dd></div>
        <div><dt>Abstand 2</dt><dd>{{ distance(stelle.abstand2) }}</dd></div>
        <div class="gate-workspace-facts__wide"><dt>Bemerkung</dt><dd>{{ stelle.bemerkung || 'Keine Bemerkung' }}</dd></div>
      </dl>
    </v-card-text>
  </v-card>
</template>
<script setup lang="ts">
import { computed } from 'vue';
import type { IProjekt, IUntersuchungsstelle } from '@/types/apiInterfaces';
const props = defineProps<{ stelle: IUntersuchungsstelle; projekt: IProjekt | null }>();
const projectName = computed(() => props.projekt ? `${props.projekt.projektnummer} · ${props.projekt.name}` : 'Nicht zugeordnet');
// Numeric units are intentionally not guessed: the existing field model has no unit metadata.
const distance = (value: number | null) => value === null || value === undefined
  ? 'Nicht angegeben' : new Intl.NumberFormat('de-DE', { maximumFractionDigits: 3 }).format(value);
</script>
