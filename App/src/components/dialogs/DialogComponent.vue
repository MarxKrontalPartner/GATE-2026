<template>
  <v-dialog v-model="showModel">

    <template v-slot:default>
      <v-card>
        <v-card-title>
          <slot name="title"></slot>
        </v-card-title>

        <v-card-text>
          <slot name="text"></slot>
        </v-card-text>

        <v-card-actions>
          <slot name="actions"></slot>
        </v-card-actions>
      </v-card>
    </template>

  </v-dialog>
</template>

<script setup lang="ts">
import { ref, watch, type Ref } from 'vue';


/*
    General use dialog component.
    Should be used to ask the user for confirmation on some action.

    Props:
    - show: Wether the dialog is shown or not. Can be manipulated by the parent.

    Events:
    - update:modelValue: Emitted when the components model changes. For 
    example when the dialog is closed by clicking outside. This should be used 
    to update the parent's show prop. The event is emitted by v-dialog by 
    default which is why there is no corresponding in code here. 

    Usage:
    <DialogComponent :show="showDialog" @update:modelValue="showDialog = $event">
      <template v-slot:title>Dialog title</template>
      <template v-slot:text>Dialog text</template>
      <template v-slot:actions>
        <v-btn @click="doSomething">Do something</v-btn>
      </template>
    </DialogComponent>
*/

const props = defineProps({
  show: Boolean,
});

const showModel: Ref<boolean> = ref(false);

watch(() => props.show, () => {
  if (props.show) {
    showModel.value = true;
  } else {
    showModel.value = false;
  }
})
</script>