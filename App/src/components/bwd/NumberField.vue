<template>
  <v-text-field
    v-bind="$attrs"
    :model-value="model ?? ''"
    type="number"
    :step="integer ? 1 : 'any'"
    :inputmode="integer ? 'numeric' : 'decimal'"
    @update:model-value="onUpdate"
  >
    <template v-for="(_, name) in $slots" #[name]="slotProps">
      <slot :name="name" v-bind="slotProps ?? {}" />
    </template>
  </v-text-field>
</template>

<script setup lang="ts">
// v-text-field for numeric backend fields: emits a number or null (never "").
import { toNumberOrNull } from '@/utils/messwerte';

defineOptions({ inheritAttrs: false });

const model = defineModel<number | null | undefined>();
const props = defineProps<{ integer?: boolean }>();

function onUpdate(value: unknown) {
  const parsed = toNumberOrNull(value);
  model.value = parsed !== null && props.integer ? Math.trunc(parsed) : parsed;
}
</script>
