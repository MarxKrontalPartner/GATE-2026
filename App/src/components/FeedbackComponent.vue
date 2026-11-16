<template>
    <div :class="`gate-feedback--${type}`" class="rounded border-sm border-opacity-100">
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