<template>
  <div class="image-card-root">
    <v-card class="image-card">
      <div class="image-card-media">
      <v-carousel
        v-if="imageData.length > 0"
        v-model:model-value="count"
        :show-arrows="imageData.length > 1"
        height="100%"
      >
        <v-carousel-item v-for="(image, i) in imageData" :key="image.id" :src="imageSrc(image)" cover reverse-transition="fade-transition"
          transition="fade-transition" position-absolute>
          <template #placeholder>
            <div class="d-flex fill-height align-center justify-center">
              <v-progress-circular indeterminate color="primary"></v-progress-circular>
            </div>
          </template>
          <div class="image-title-container">
            <InfoLabel class="text-h6">
              <span>{{ image.name }}</span>
            </InfoLabel>
            <v-chip 
              v-if="currentUntersuchung?.titelBild === image.id" 
              color="primary" 
              size="small" 
              class="ml-2"
            >
              <v-icon start size="small">{{ mdiStar }}</v-icon>
              Titelbild
            </v-chip>
          </div>
          <v-row class="ma-0 pa-0" style="position: absolute; top: -8px; right: 0; width: auto;">
            <v-col cols="auto" class="pa-0 ma-0">
              <v-row justify="end" class="ma-0 pa-0 mt-3 mr-2">
                <RoundIconButton 
                  :append-icon="mdiStar" 
                  :title="currentUntersuchung?.titelBild === image.id ? 'Dies ist das Titelbild' : 'Dieses Bild als Titelbild setzen'" 
                  :color="currentUntersuchung?.titelBild === image.id ? 'primary' : undefined"
                  :disabled="!props.enabled"
                  @click="setTitelBild(image.id)">
                </RoundIconButton>
              </v-row>
            </v-col>
            <v-col cols="auto" class="pa-0 ma-0">
              <v-row justify="end" class="ma-0 pa-0 mt-3 mr-2">
                <RoundIconButton
                  :append-icon="mdiDownload"
                  title="Original herunterladen"
                  :loading="isDownloading && downloadingId === image.id"
                  @click="downloadOriginal(image, i)">
                </RoundIconButton>
              </v-row>
            </v-col>
          </v-row>
        </v-carousel-item>
      </v-carousel>
      <div v-else class="image-card-placeholder d-flex flex-column align-center justify-center pa-8 text-medium-emphasis">
        <v-icon :icon="mdiImageOff" size="48" class="mb-2"></v-icon>
        <span>Keine Bilder vorhanden</span>
      </div>
      </div>
      <v-card-actions class="image-card-actions">
        <v-btn color="secondary" variant="tonal" :append-icon="mdiImage" :disabled="!props.enabled || imageData.length === 0"
          @click="showEditDialog = true">
          BEARBEITEN
        </v-btn>
        <EditImageDialog 
          v-model:modelValue="showEditDialog" 
          :images="displayImages" 
          :titelBildId="currentUntersuchung?.titelBild"
          @removeImage="removeImage"
          @saveChanges="saveChanges" 
        />
        <v-spacer></v-spacer>
        <v-btn color="primary" variant="tonal" :append-icon="mdiImage" :disabled="!props.enabled" @click="showUploadDialog = true">
          HINZUFÜGEN
        </v-btn>
        <AddImageDialog v-model="showUploadDialog" @uploadImage="uploadImage" />
      </v-card-actions>
    </v-card>
  </div>
</template>

<script setup lang="ts">
/*
  Images are shown via blob URLs from the content endpoint (useUvImages), downloads use an
  <a download> (no new windows in the Tauri app), the buttons follow the edit mode (`enabled`)
  and an empty state is shown when there are no images.
*/
import { ref, watch, onMounted, computed } from "vue";
import { mdiDownload, mdiImage, mdiImageOff, mdiStar } from "@mdi/js";
import { RoundIconButton } from "@/components/buttons";
import InfoLabel from "@/components/InfoLabel.vue";
import EditImageDialog from "@/components/bwd/media/EditImageDialog.vue";
import AddImageDialog from "@/components/bwd/media/AddImageDialog.vue";
import { apiConnector } from "@/utils/ApiConnector";
import { downloadMediaOriginal } from "@/components/bwd/media/annotatedImageExport";
import { useUntersuchungenStore } from "@/store/bwd/UntersuchungsverfahrenStore";
import { useFeedbackStore } from "@/store/FeedbackStore";
import { useUvImages } from "@/composables/useUvImages";
import { getErrorMessage } from "@/utils/errors";
import { storeToRefs } from "pinia";

// Define emits properly
const emit = defineEmits(['save', 'uploadImage', 'removeImage', 'saveChanges']);

// Include enabled in props
const props = defineProps<{
  images: number[];
  enabled?: boolean;
}>();

const count = ref<number>(0);
const showEditDialog = ref<boolean>(false);
const showUploadDialog = ref<boolean>(false);

const untersuchungenStore = useUntersuchungenStore();
const feedbackStore = useFeedbackStore();
const { currentUntersuchung  } = storeToRefs(untersuchungenStore);

interface Image {
  id: number;
  name: string;
  datei: string;
  beschreibung?: string;
  local_id?: number | null;
}

const imageData = ref<Image[]>([]);

// Blob URLs of the image contents (cached in memory by vue-query)
const imageIds = computed(() => imageData.value.map((image) => image.id));
const { getImage } = useUvImages(imageIds);

function imageSrc(image: Image): string | undefined {
  return getImage(image.id)?.data;
}

// The EditImageDialog shows `datei` as thumbnail, so it gets the blob URLs
const displayImages = computed(() =>
  imageData.value.map((image) => ({ ...image, datei: imageSrc(image) ?? '' }))
);

async function fetchImages() {
  if (!props.images || props.images.length === 0) {
    imageData.value = [];
    return;
  }

  // Use batch endpoint to fetch all images at once
  const images = await apiConnector.getMediaBatch(props.images);
  
  // Sort images by local_id (if available), then by their order in the array
  images.sort((a, b) => {
    // Handle null/undefined local_id values
    const aLocalId = a.local_id ?? Number.MAX_SAFE_INTEGER;
    const bLocalId = b.local_id ?? Number.MAX_SAFE_INTEGER;
    return aLocalId - bLocalId;
  });
  
  // If a titelBild is set, move it to the front
  if (currentUntersuchung.value?.titelBild) {
    const titelBildId = currentUntersuchung.value.titelBild;
    const titelBildIndex = images.findIndex(image => image.id === titelBildId);
    if (titelBildIndex > 0) {
      // Move titelBild to the front
      const [titelBild] = images.splice(titelBildIndex, 1);
      images.unshift(titelBild);
    }
  }
  
  imageData.value = images;
  if (count.value >= images.length) {
    count.value = 0;
  }
}

function loadImages() {
  fetchImages().catch((error) => {
    console.error('Error fetching images:', error);
    feedbackStore.addError(getErrorMessage(error, 'Bilder konnten nicht geladen werden'));
  });
}

onMounted(() => {
  loadImages();
});

// Compare the ids, not the array reference
watch(
  () => (props.images ?? []).join(','),
  () => {
    loadImages();
  }
);

async function uploadImage(img: any) {
  // Calculate the next local_id
  const maxLocalId = imageData.value.reduce((max, image) => {
    const localId = image.local_id ?? -1;
    return Math.max(max, localId);
  }, -1);
  const nextLocalId = maxLocalId + 1;
  
  const data = {
            "beschreibung": "",
            "datei": img.datei,
            "model_name": currentUntersuchung.value.type,
            "model_pk": currentUntersuchung.value.id,
            "name": img.name,
            "media_type": "image",
            "bauwerk": currentUntersuchung.value.bauwerk,
            "local_id": nextLocalId,

        }
  const resp = await apiConnector.uploadMedia("bwd", data);
  if (resp) {
    if (!currentUntersuchung.value.bilder) {
      currentUntersuchung.value.bilder = [];
    }
    // Use type assertion to handle the bilder array
    (currentUntersuchung.value.bilder as number[]).push(resp.id);
    loadImages();
    try {
      await untersuchungenStore.saveCurrentUntersuchung();
    } catch (error) {
      feedbackStore.addError(getErrorMessage(error, 'Bild hochgeladen, aber die Untersuchung konnte nicht gespeichert werden'));
    }
    emit('uploadImage', resp);
    return true;
  }
  return true;
}

async function removeImage(id: number) {
  // Remove the image with the given id from imageData
  currentUntersuchung.value.bilder = currentUntersuchung.value.bilder.filter((img: number) => img !== id);
  // The deleted media can no longer be the Titelbild (otherwise the next save sends a stale id)
  if (currentUntersuchung.value.titelBild === id) {
    currentUntersuchung.value.titelBild = null;
  }
  const deleted = await apiConnector.deleteMedia(id);
  if (!deleted) {
    feedbackStore.addError('Bild konnte nicht gelöscht werden');
  }
  emit('removeImage', id);
}

async function saveChanges(images: Image[]) {
  // Save the changes to the images, including updating local_id for ordering
  try {
    for (const image of images) {
      // Update each image with its new data (name, beschreibung, local_id)
      await apiConnector.patchxResource("bwd", "Media", {
        name: image.name,
        beschreibung: image.beschreibung,
        local_id: image.local_id
      }, image.id);
    }
    
    // Refresh the images to show the updated data
    await fetchImages();
    emit('saveChanges', images);
    return Promise.resolve(true);
  } catch (error) {
    console.error('Error saving image changes:', error);
    feedbackStore.addError(getErrorMessage(error, 'Bilder konnten nicht gespeichert werden'));
    return Promise.resolve(false);
  }
}

const isDownloading = ref(false);
const downloadingId = ref<number | null>(null);

async function downloadOriginal(image: Image, index: number) {
  if (isDownloading.value) return;
  count.value = index;
  isDownloading.value = true;
  downloadingId.value = image.id;
  try {
    await downloadMediaOriginal(image.id, image.name, image.datei);
  } catch (error) {
    feedbackStore.addError(getErrorMessage(error, 'Download fehlgeschlagen'));
  } finally {
    isDownloading.value = false;
    downloadingId.value = null;
  }
}

async function setTitelBild(id: number) {
  currentUntersuchung.value.titelBild = id;
  try {
    await untersuchungenStore.saveCurrentUntersuchung();
  } catch (error) {
    feedbackStore.addError(getErrorMessage(error, 'Titelbild konnte nicht gespeichert werden'));
  }
}

</script>

<style scoped>
/* Fill the height of the surrounding row (the form column next to it sets the height). */
.image-card-root,
.image-card {
  height: 100%;
}

.image-card {
  display: flex;
  flex-direction: column;
}

.image-card-media {
  flex: 1 1 auto;
  display: flex;
  min-height: 280px;
}

.image-card-media > * {
  flex: 1 1 auto;
}

.image-card-placeholder {
  background: rgba(var(--v-theme-on-surface), 0.04);
}

.image-card-actions {
  flex: 0 0 auto;
}

.text-card {
  color: rgb(var(--v-theme-text)) !important;
  font-weight: 400 !important;
}

.image-title-container {
  position: absolute;
  top: 12px;
  left: 12px;
  display: flex;
  align-items: center;
  gap: 8px;
  max-width: calc(100% - 140px);
  flex-wrap: wrap;
}
</style>
