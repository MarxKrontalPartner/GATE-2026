<template>
  <VLayoutItem model-value position="bottom" class="text-end" size="60">
    <!-- Positioned container anchored to bottom-right -->
    <div class="edit-actions d-flex align-center">
      <!-- Action buttons shown in edit mode -->
      <v-fab
        v-if="editInfo && undoChanges"
        :icon="mdiUndoVariant" 
        color="yellow" 
        size="small"
        @click="undoChanges"
      ></v-fab>
      
      <v-fab
        v-if="editInfo && del"
        :icon="mdiDelete" 
        color="red" 
        size="small"
        @click="showDeleteDialog = true"
      ></v-fab>

      <!-- Main edit/save button -->
      <v-fab
        size="large"
        :color="editInfo ? 'green' : 'primary'"
        :icon="editInfo ? mdiContentSaveAll : mdiPencil"
        @click="handleFabClick"
      ></v-fab>
    </div>

    <v-dialog v-model="showDeleteDialog" max-width="500">
      <v-card>
        <v-card-title class="headline">Bestätige Löschen</v-card-title>
        <v-card-text>Möchten Sie diese Seite wirklich löschen?</v-card-text>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn color="green darken-1" variant="text" @click="showDeleteDialog = false">Cancel</v-btn>
          <v-btn color="red darken-1" variant="text" @click="confirmDelete">Delete</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </VLayoutItem>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { mdiContentSaveAll, mdiDelete, mdiUndoVariant, mdiPencil } from "@mdi/js";

const props = defineProps({
  saveChanges: {
    type: Function,
    required: true,
  },
  undoChanges: {
    type: Function,
    required: false,
  },
  del: {
    type: Function,
    required: false,
  }
});

const editInfo = defineModel("editInfo", {type: Boolean, required: true});
const showDeleteDialog = ref<boolean>(false);

function handleFabClick(){
  // When in edit mode (editInfo is true), clicking the FAB should save and exit edit mode
  if(editInfo.value){
    props.saveChanges();
    editInfo.value = false; // Exit edit mode after saving
  } else {
    // Enter edit mode
    editInfo.value = true;
  }
}

function confirmDelete() {
  showDeleteDialog.value = false;
  if (props.del) {
    props.del();
  }
}
</script>

<style scoped>
.edit-actions {
  position: fixed; /* Keep buttons visible even when scrolling */
  bottom: 8px; /* Distance from bottom */
  right: 8px; /* Distance from right edge */
  gap: 12px; /* Space between FABs */
  z-index: 1000; /* Above regular content */
}

@media (max-width: 600px) {
  .edit-actions {
    bottom: 16px;
    right: 16px;
    gap: 8px;
  }
}
</style>