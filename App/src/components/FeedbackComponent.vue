<template>
    <div :class="type" class="rounded border-sm border-opacity-100">
        <p class="ma-1">
            <template v-if="action"><span class="font-weight-medium">{{ action }}</span>&nbsp;</template>{{ message }}
        </p>
    </div>
</template>

<script setup lang="ts">
import { type IFeedback } from '@/store/FeedbackStore';


/*
  Displays feedback messages of different types.
  Used by FeedbackFeedComponent.
*/

defineProps<IFeedback>();
</script>
<style lang="scss" scoped>
.success {
  background-color: #8ee2be;  // not corp
  color: #007542;
  border-color: #007542 !important;  // not corp
}
.error {
  background-color: #f6b9c4;
  color: #750016;
  border-color: #750016 !important;
}
.warning {
  background-color: #f4ef84;
  color: #756F00;  // not corp
  border-color: #756F00 !important;
}
.info {
  background-color: #d7effc;
  color: #004C75;
  border-color: #095371 !important;
}
</style>