<template>
  <v-card class="gate-card gate-workspace-info" variant="flat">
    <v-card-title>Bauwerksdaten</v-card-title>
    <v-card-text>
      <dl class="gate-workspace-facts">
        <div><dt>Bauwerksnummer</dt><dd>{{ bauwerk.bauwerksnummer || 'Nicht angegeben' }}</dd></div>
        <div><dt>Straße</dt><dd>{{ bauwerk.strasse || 'Nicht angegeben' }}</dd></div>
        <div><dt>Bauwerkstyp</dt><dd>{{ bauwerk.bauwerkstyp || 'Nicht angegeben' }}</dd></div>
        <div><dt>Bauweise</dt><dd>{{ bauwerk.bauweise || 'Nicht angegeben' }}</dd></div>
        <div v-if="bauwerk.beschreibung" class="gate-workspace-facts__wide">
          <dt>Beschreibung</dt><dd>{{ bauwerk.beschreibung }}</dd>
        </div>
        <div v-if="bauwerk.bemerkung" class="gate-workspace-facts__wide">
          <dt>Bemerkung</dt><dd>{{ bauwerk.bemerkung }}</dd>
        </div>
      </dl>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import type { IBauwerk } from '@/types/apiInterfaces';
defineProps<{ bauwerk: IBauwerk }>();
</script>
