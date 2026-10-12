<template>
  <v-select
    v-bind="$attrs"
    :model-value="model || null"
    :items="items"
    item-title="title"
    item-value="value"
    clearable
    @update:model-value="model = $event ?? ''"
  />
</template>

<script setup lang="ts">
// v-select for backend choice fields: the empty choice is stored as '' (not null).
export interface ChoiceOption {
  value: string;
  title: string;
}

defineOptions({ inheritAttrs: false });

const model = defineModel<string | null | undefined>();
defineProps<{ items: ChoiceOption[] }>();
</script>
