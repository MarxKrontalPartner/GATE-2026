<template>
  <v-dialog v-model="showUploadDialog" max-width="800px" @update:model-value="onDialogUpdate">
    <v-card>
      <v-card-title>
        <span class="headline">Bild Hochladen</span>
      </v-card-title>

      <v-card-text>
        <v-text-field
          v-model="newImage.name"
          label="Name"
          class="text-card"
          outlined
        ></v-text-field>
        <v-file-input
          v-model="newImage.file"
          label="Image"
          accept="image/*"
          counter
          multiple
          show-size
        ></v-file-input>
        <v-alert v-if="errorMessage" type="error" variant="tonal" density="compact">
          {{ errorMessage }}
        </v-alert>
      </v-card-text>

      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="blue darken-1" variant="text" @click="cancel">Cancel</v-btn>
        <v-btn color="blue darken-1" variant="text" :loading="uploading" @click="handleUploadImage">
          Upload
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
// Accepts images only, shows the upload loading/error state and uses the file name when no name is given.
import { ref, watch } from "vue";


interface UploadData {
  name: string;
  media_type: string;
  datei: File;
}

const props = defineProps<{
  modelValue: boolean;
  onUploadImage: (data: UploadData) => Promise<boolean>;
}>();

const emits = defineEmits<{
  (e: "update:modelValue", value: boolean): void;
}>();

// Track the dialog's visibility (sync with modelValue via v-model)
const showUploadDialog = ref(props.modelValue);
watch(
  () => props.modelValue,
  (newVal) => {
    showUploadDialog.value = newVal;
  }
);

const newImage = ref<{ name: string; file: File[] | File | null }>({
  name: "",
  file: null,
});
const uploading = ref(false);
const errorMessage = ref("");

function onDialogUpdate(value: boolean) {
  if (!value) {
    cancel();
  }
}

function cancel() {
  showUploadDialog.value = false;
  errorMessage.value = "";
  emits("update:modelValue", false);
}

async function handleUploadImage() {
  // Vuetify returns File[] for multiple inputs, older versions/single mode a File
  const files = Array.isArray(newImage.value.file)
    ? newImage.value.file
    : newImage.value.file ? [newImage.value.file] : [];
  // Make sure at least one file is selected
  if (files.length === 0) {
    errorMessage.value = "Bitte ein Bild auswählen.";
    return;
  }

  uploading.value = true;
  errorMessage.value = "";
  try {
    // Prepare the data
    const data = {
      name: newImage.value.name || files[0].name,
      media_type: "image",
      datei: files[0], // We'll just use the first file
    };

    // Call the parent's upload handler
    const success = await props.onUploadImage(data);

    if (success) {
      // Reset form and close on success
      newImage.value = { name: "", file: null };
      cancel();
    } else {
      errorMessage.value = "Das Bild konnte nicht hochgeladen werden.";
    }
  } catch (error) {
    console.error("Upload error:", error);
    errorMessage.value = "Das Bild konnte nicht hochgeladen werden.";
  } finally {
    uploading.value = false;
  }
}
</script>
