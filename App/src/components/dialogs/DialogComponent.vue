<template>
  <v-dialog v-model="showModel" max-width="600" v-bind="$attrs">
    <v-card class="gate-card gate-form">
      <v-card-title><slot name="title" /></v-card-title>
      <v-card-text><slot name="text" /></v-card-text>
      <v-card-actions class="flex-wrap ga-2"><v-spacer /><slot name="actions" /></v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { computed } from 'vue';

defineOptions({ inheritAttrs: false });
const props = defineProps<{ show: boolean }>();
const emit = defineEmits<{ 'update:modelValue': [value: boolean] }>();
const showModel = computed({
  get: () => props.show,
  set: (value: boolean) => emit('update:modelValue', value),
});
</script>
