<template>
  <v-card class="gate-investigation-section" variant="outlined">
    <v-card-title class="gate-investigation-section__title">
      <v-icon v-if="icon" :icon="icon" color="primary" size="28" />
      <span>{{ title }}</span>
    </v-card-title>
    <v-card-subtitle v-if="description" class="gate-investigation-section__description">{{ description }}</v-card-subtitle>
    <v-card-text class="gate-investigation-section__body"><slot /></v-card-text>
  </v-card>
</template>
<script setup lang="ts">
defineProps<{ title: string; description?: string; icon?: string }>();
</script>
