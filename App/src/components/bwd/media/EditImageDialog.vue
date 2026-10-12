<template>
  <v-dialog v-model="showEditDialog" persistent max-width="1000px" max-height="90vh">
    <v-card class="d-flex flex-column" style="height: 80vh;">
      <v-card-title class="d-flex align-center pa-4 flex-shrink-0">
        <span class="text-h5">Bilder bearbeiten</span>
      </v-card-title>
      <v-divider></v-divider>
      <v-card-text class="overflow-y-auto flex-grow-1" :class="mobile ? 'pa-2' : 'pa-4'">
        <v-list class="image-list">
          <v-list-item
            v-for="(image, index) in editedImages" 
            :key="image.id"
            :draggable="!isLockedImage(image.id)"
            @dragstart="handleDragStart(index, $event)"
            @dragover.prevent="handleDragOver(index, $event)"
            @drop="handleDrop(index, $event)"
            @dragend="handleDragEnd"
            :class="{ 'dragging': draggedIndex === index, 'drag-over': dragOverIndex === index, 'locked': isLockedImage(image.id) }"
            class="image-list-item mb-2"
          >
            <template v-slot:prepend>
              <v-icon 
                v-if="!isLockedImage(image.id)"
                class="drag-handle mr-3" 
                size="large"
                :icon="mdiDragVertical"
              >
                
              </v-icon>
              <v-icon 
                v-else
                class="lock-icon mr-3" 
                size="large"
                color="primary"
                :icon="mdiStar"
              >
              </v-icon>
              <div v-if="!mobile" class="image-number" :class="{ 'locked-number': isLockedImage(image.id) }">{{ index + 1 }}</div>
            </template>
            
            <v-list-item-title class="d-flex align-center py-2">
              <v-avatar :size="mobile ? 48 : 80" rounded class="flex-shrink-0" :class="mobile ? 'mr-2' : 'mr-4'">
                <v-img :src="image.datei" cover></v-img>
              </v-avatar>
              
              <div class="flex-grow-1" style="min-width: 0;">
                <v-text-field 
                  v-model="image.name" 
                  label="Name"
                  variant="outlined"
                  density="compact"
                  hide-details
                  class="mb-2"
                ></v-text-field>
                <v-text-field
                  v-model="image.beschreibung"
                  label="Beschreibung"
                  variant="outlined"
                  density="compact"
                  hide-details
                ></v-text-field>
              </div>
            </v-list-item-title>

            <template v-slot:append>
              <v-btn 
                icon
                color="error" 
                variant="text"
                class="flex-shrink-0"
                title="Bild löschen"
                @click="handleRemoveImage(image.id)"
              >
                <v-icon :icon="mdiDelete" />
              </v-btn>
            </template>
          </v-list-item>
        </v-list>
      </v-card-text>
      <v-divider></v-divider>
      <v-card-actions class="pa-4 flex-shrink-0">
        <v-spacer></v-spacer>
        <v-btn variant="text" @click="close">
          Abbrechen
        </v-btn>
        <v-btn color="primary" variant="elevated" @click="handleSaveChanges">
          Speichern
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
/*
  Compact layout on small screens (smaller avatar, no position number).
  The images are passed with blob URLs in `datei` (see UntersuchungsImageCard).
*/
import { ref, watch } from "vue";
import { useDisplay } from "vuetify";
import { mdiDragVertical, mdiStar, mdiDelete } from "@mdi/js";

interface Image {
  id: number;
  name: string;
  datei: string;
  beschreibung?: string;
  local_id?: number | null;
}

const props = defineProps<{
  images: Image[];
  modelValue: boolean;
  titelBildId?: number | string | null;
  onRemoveImage: (id: number) => void;
  onSaveChanges: (images: Image[]) => Promise<boolean>;
}>();

const emits = defineEmits<{
  (e: "update:modelValue", value: boolean): void;
}>();

const { mobile } = useDisplay();

const showEditDialog = ref(props.modelValue);
const editedImages = ref<Image[]>([]);
const draggedIndex = ref<number | null>(null);
const dragOverIndex = ref<number | null>(null);

// Watch for changes in the v-model (modelValue)
watch(
  () => props.modelValue,
  (newVal) => {
    showEditDialog.value = newVal;
    // Make a deep copy of images so we don't mutate props directly
    editedImages.value = JSON.parse(JSON.stringify(props.images));
  }
);

function close() {
  showEditDialog.value = false;
  emits("update:modelValue", false);
}

function isLockedImage(imageId: number): boolean {
  return props.titelBildId != null && imageId === Number(props.titelBildId);
}

function handleRemoveImage(id: number) {
  // Call the parent's removal function
  props.onRemoveImage(id);
  close();
}

// Drag and drop handlers
function handleDragStart(index: number, event: DragEvent) {
  // Don't allow dragging the locked title image
  if (isLockedImage(editedImages.value[index].id)) {
    event.preventDefault();
    return;
  }
  draggedIndex.value = index;
  if (event.dataTransfer) {
    event.dataTransfer.effectAllowed = 'move';
    event.dataTransfer.setData('text/html', String(index));
  }
}

function handleDragOver(index: number, event: DragEvent) {
  event.preventDefault();
  // Don't allow dropping on position 0 if title image is locked there
  if (index === 0 && editedImages.value[0] && isLockedImage(editedImages.value[0].id)) {
    return;
  }
  dragOverIndex.value = index;
  if (event.dataTransfer) {
    event.dataTransfer.dropEffect = 'move';
  }
}

function handleDrop(index: number, event: DragEvent) {
  event.preventDefault();
  // Don't allow dropping on position 0 if title image is locked there
  if (index === 0 && editedImages.value[0] && isLockedImage(editedImages.value[0].id)) {
    dragOverIndex.value = null;
    return;
  }
  if (draggedIndex.value !== null && draggedIndex.value !== index) {
    const draggedItem = editedImages.value[draggedIndex.value];
    // Remove the dragged item
    editedImages.value.splice(draggedIndex.value, 1);
    // Insert it at the new position
    editedImages.value.splice(index, 0, draggedItem);
  }
  dragOverIndex.value = null;
}

function handleDragEnd() {
  draggedIndex.value = null;
  dragOverIndex.value = null;
}

async function handleSaveChanges() {
  try {
    // Update local_id based on the new order
    editedImages.value.forEach((image, index) => {
      image.local_id = index;
    });
    
    // Call the parent's save function, and wait for the result
    const success = await props.onSaveChanges(editedImages.value);
    if (success) {
      // If the parent indicates success, close the dialog
      close();
    } else {
      // Handle any "failed save" logic here (snackbar, alert, etc.)
      console.error("Failed to save images in the parent.");
    }
  } catch (err) {
    console.error("An error occurred while saving images: ", err);
  }
}
</script>

<style scoped>
.image-list {
  background: transparent;
}

.image-list-item {
  background: white;
  border: 2px solid #e0e0e0;
  border-radius: 8px;
  padding: 12px;
  cursor: move;
  transition: all 0.2s ease;
  user-select: none;
}

.image-list-item.locked {
  cursor: default;
  background-color: #f5f5f5;
  border-color: #1976d2;
}

.image-list-item:not(.locked):hover {
  background-color: #f5f5f5;
  border-color: #1976d2;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.image-list-item.dragging {
  opacity: 0.4;
  transform: scale(0.95);
}

.image-list-item.drag-over:not(.locked) {
  background-color: #e3f2fd;
  border: 2px dashed #1976d2;
  border-color: #1976d2;
  box-shadow: 0 4px 12px rgba(25, 118, 210, 0.2);
}

.drag-handle {
  color: #9e9e9e;
  cursor: grab;
}

.drag-handle:active {
  cursor: grabbing;
}

.lock-icon {
  cursor: default;
}

.image-number {
  min-width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: #e0e0e0;
  border-radius: 50%;
  font-weight: bold;
  color: #616161;
  margin-right: 12px;
}

.image-number.locked-number {
  background-color: #1976d2;
  color: white;
}

.image-list-item.drag-over:not(.locked) .image-number {
  background-color: #1976d2;
  color: white;
}
</style>
