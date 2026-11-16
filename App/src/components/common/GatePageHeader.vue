<template>
  <header class="gate-page-header">
    <div>
      <h1 class="gate-page-title">{{ title }}</h1>
      <p v-if="subtitle" class="gate-page-subtitle">{{ subtitle }}</p>
    </div>
    <div v-if="$slots.actions" class="gate-page-header__actions">
      <slot name="actions" />
    </div>
  </header>
</template>

<script setup lang="ts">
defineProps<{ title: string; subtitle?: string }>();
</script>
