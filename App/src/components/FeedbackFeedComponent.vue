<template>
  <ul 
    v-if="feedbackStore.feedbackFeed.length > 0" 
    id="feedbackFeed" 
    class="position-fixed bottom-0 right-0 mx-1"
  >
    <li 
      v-for="feedback in feedbackStore.feedbackFeed" :key="feedback.id" 
      class="d-flex justify-between align-center my-1"
    >
      <FeedbackComponent 
        class="flex-grow-1 py-2 pl-4 pr-10" 
        :action="feedback.action" 
        :message="feedback.message" 
        :type="feedback.type" 
        :id="feedback.id" 
      />
      <v-btn 
        variant="plain"
        :ripple="false"
        @click="feedbackStore.removeFeedback(feedback.id)" 
        :append-icon="mdiClose"
        class="position-absolute right-0"
      />
    </li>
  </ul>
</template>

<script lang="ts" setup>
import { mdiClose } from '@mdi/js';

import FeedbackComponent from './FeedbackComponent.vue';
import { useFeedbackStore } from '@/store/FeedbackStore';
/*
  Displays a feed of feedback messages 
  and basically just acts as a container for FeedbackComponents.
  Also gives the user the ability to dismiss messages.
*/
const feedbackStore = useFeedbackStore();
</script>
<style scoped lang="scss">
#feedbackFeed {
    z-index: 2401;  // some grid stuff uses 2. so display on top -> 3 (old) || now 2401 because vuetify card uses 2400
    width: 45ch;
}
</style>