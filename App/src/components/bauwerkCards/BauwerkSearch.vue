<template>
  <v-autocomplete
    v-model="selected"
    :items="items"
    :prepend-inner-icon="mdiMagnify"
    :clear-icon="mdiClose"
    label="Suche"
    aria-label="Bauwerke durchsuchen"
    variant="outlined"
    multiple
    clearable
    hide-details
    class="gate-search-field"
  />
</template>

<script setup lang="ts">
import { mdiMagnify, mdiClose } from '@mdi/js';

defineProps<{ items: string[] }>();
const selected = defineModel<string[]>({ default: () => [] });
</script>
