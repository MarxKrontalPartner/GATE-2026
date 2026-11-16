<template>
  <v-btn
    v-bind="$attrs"
    class="gate-action-button"
    :color="color"
    :variant="variant"
    :loading="loading"
    :disabled="disabled"
  >
    <slot />
  </v-btn>
</template>

<script setup lang="ts">
import { computed } from 'vue';

defineOptions({ inheritAttrs: false });

const props = withDefaults(defineProps<{
  intent?: 'primary' | 'secondary' | 'danger' | 'quiet';
  loading?: boolean;
  disabled?: boolean;
}>(), { intent: 'primary', loading: false, disabled: false });

const color = computed(() => ({
  primary: 'primary', secondary: 'secondary', danger: 'error', quiet: 'secondary',
}[props.intent]));
const variant = computed<'flat' | 'tonal' | 'outlined' | 'text'>(() => ({
  primary: 'flat', secondary: 'tonal', danger: 'tonal', quiet: 'text',
} as const)[props.intent]);
</script>
