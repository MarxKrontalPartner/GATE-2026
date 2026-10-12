<template>



    <v-sheet>
        <v-row>
            <v-col cols="12" md="12">
                <v-dialog v-model="addDialog" transition="dialog-bottom-transition" max-width="800px">
                    <template v-slot:activator="{ props: activatorProps }">
                        <div class="d-flex justify-center">
                            <v-btn v-bind="activatorProps" text="+" color="primary" :disabled="!props.enabled" title="Datei hinzufügen"></v-btn>
                        </div>
                    </template>

                    <template v-slot:default="{ isActive }">
                        <v-card>
                            <v-card-title>
                                <span class="headline">Datei hinzufügen</span>
                            </v-card-title>

                            <v-card-text>
                                <v-tabs v-model="uploadMode" align-tabs="center">
                                    <v-tab value="file">Datei hochladen</v-tab>
                                    <v-tab value="link">Von Link</v-tab>
                                </v-tabs>
                                
                                <v-window v-model="uploadMode" class="mt-4">
                                    <v-window-item value="file">
                                        <v-text-field v-model="newFile.name" label="Name" class="text-card"
                                            outlined></v-text-field>
                                        <v-text-field v-model="newFile.beschreibung" label="Beschreibung" class="text-card"
                                            outlined></v-text-field>
                                        <v-file-input v-model="newFile.file" label="Datei" counter multiple
                                            show-size></v-file-input>
                                    </v-window-item>
                                    
                                    <v-window-item value="link">
                                        <v-text-field v-model="newFile.name" label="Name" class="text-card"
                                            outlined></v-text-field>
                                        <v-text-field v-model="newFile.beschreibung" label="Beschreibung" class="text-card"
                                            outlined></v-text-field>
                                        <v-text-field v-model="newFile.link" label="Datei URL/Link" class="text-card"
                                            outlined placeholder="https://beispiel.com/datei.pdf"></v-text-field>
                                    </v-window-item>
                                </v-window>
                                <v-alert v-if="errorMessage" type="error" variant="tonal" density="compact">
                                    {{ errorMessage }}
                                </v-alert>
                            </v-card-text>

                            <v-card-actions>
                                <v-spacer></v-spacer>
                                <v-btn color="blue darken-1" variant="text"
                                    @click="isActive.value = false">Abbrechen</v-btn>
                                <v-btn color="blue darken-1" variant="text" :loading="uploading" :disabled="!props.enabled"
                                    @click="handleAddFile(isActive)">
                                    Hinzufügen
                                </v-btn>
                            </v-card-actions>
                        </v-card>
                    </template>
                </v-dialog>
            </v-col>

            <v-col v-for="file in files" :key="file.id" cols="12" md="4">
                <v-card :title="file.name" elevation="2" outlined
                    :subtitle="formatDate(file.hochgeladen_am)">
                    <template v-slot:prepend>
                        <v-avatar v-if="isImageFile(file.datei) && previewSrc(file)" :image="previewSrc(file)"></v-avatar>
                        <v-icon v-else :icon="getFileIcon(file.datei)" size="32" color="primary"></v-icon>
                    </template>
                    
                    <!-- Image Preview -->
                    <div v-if="isImageFile(file.datei)" class="pa-2">
                        <v-img 
                            :src="previewSrc(file)" 
                            :alt="file.name"
                            height="200"
                            cover
                            class="rounded"
                        ></v-img>
                    </div>
                    
                    <v-card-actions>
                        <v-dialog transition="dialog-bottom-transition" :fullscreen="mobile" max-width="1200px"
                            @update:model-value="(open: boolean) => open && loadContent(file)">
                            <template v-slot:activator="{ props: activatorProps }">
                                <v-btn v-bind="activatorProps" text="Inhalt Anzeigen" color="primary"></v-btn>
                            </template>

                            <template v-slot:default="{ isActive }">
                                <v-card>
                                    <v-toolbar :title="file.name"></v-toolbar>

                                    <v-card-text>
                                        <div v-if="!contentUrls[file.id]" class="d-flex justify-center pa-8">
                                            <v-progress-circular indeterminate color="primary"></v-progress-circular>
                                        </div>
                                        <div v-else-if="getFileExtension(file.datei) === 'pdf'">
                                            <iframe :src="contentUrls[file.id]" width="100%" class="file-frame"></iframe>
                                        </div>
                                        <div v-else-if="isImageFile(file.datei)">
                                            <img :src="contentUrls[file.id]" style="max-width: 100%; height: auto;" :alt="file.name" />
                                        </div>
                                        <div v-else>
                                            <div class="text-center pa-4">
                                                <v-icon :icon="getFileIcon(file.datei)" size="64" class="mb-4"></v-icon>
                                                <div class="text-subtitle-1 mb-2">{{ file.name }}</div>
                                            </div>
                                        </div>
                                        <div class="text-center pa-2">
                                            <v-btn 
                                                color="primary" 
                                                variant="outlined"
                                                :prepend-icon="mdiDownload"
                                                @click="downloadFile(file)"
                                            >
                                                Datei herunterladen
                                            </v-btn>
                                        </div>
                                    </v-card-text>

                                    <v-card-actions class="justify-end">
                                        <v-btn text="Löschen" @click="handleDelete(file, () => { isActive.value = false })" color="red" :disabled="!props.enabled"></v-btn>
                                        <v-spacer></v-spacer>
                                        <v-btn text="Schließen" @click="isActive.value = false" color="primary"></v-btn>
                                    </v-card-actions>
                                </v-card>
                            </template>
                        </v-dialog>

                        <v-spacer></v-spacer>

                        <v-btn :icon="show[file.id] ? mdiChevronUp : mdiChevronDown"
                            @click="show[file.id] = !show[file.id]"></v-btn>
                    </v-card-actions>

                    <v-expand-transition>
                        <div v-show="show[file.id]">
                            <v-divider></v-divider>

                            <v-card-text>
                                {{ file.beschreibung }}
                                
                            </v-card-text>
                        </div>
                    </v-expand-transition>

                </v-card>
            </v-col>
        </v-row>
    </v-sheet>



</template>

<script setup lang="ts">
/*
  Contents are shown via blob URLs from the content endpoint and downloaded via <a download>
  (no new windows in the Tauri app); file types are shown with mdi icons.
  The bauwerk comes from the route; "+"/"Löschen" follow `enabled`; the list reloads when the
  model changes.
*/
import { ref, computed, onMounted, watch } from 'vue';
import { useRoute } from 'vue-router';
import { useDisplay } from 'vuetify';
import { mdiChevronDown, mdiChevronUp, mdiDownload } from '@mdi/js';
import { apiConnector } from '@/utils/ApiConnector';
import { useUvImages } from '@/composables/useUvImages';
import { downloadMediaOriginal } from '@/components/bwd/media/annotatedImageExport';
import { formatDate, getFileExtension, getFileIcon, isImageFile } from '@/components/bwd/fileFieldHelpers';



const model = defineModel<number[]>()
const show = ref<{ [key: number]: boolean }>({});
const uploadMode = ref('file'); // 'file' or 'link'
const addDialog = ref(false);
const uploading = ref(false);
const errorMessage = ref('');
const route = useRoute();
const { mobile } = useDisplay();

const newFile = ref<any>({
    name: '',
    file: null as File[] | File | null,
    beschreibung: '',
    link: ''
});


//model contains a list of pk's of the selected media

const props = defineProps(
    {
        enabled: Boolean,
        label: String,
        onUploadFile: Function,
        onDeleteFile: Function
    }
);


export interface IFiles {
    id: number;
    name: string;
    beschreibung: string;
    datei: string;
    hochgeladen_am: string;
    type: string;

}


const files = ref<IFiles[]>([]);
const contentUrls = ref<Record<number, string>>({});

// Blob URLs for the image previews (in-memory vue-query cache)
const previewIds = computed(() => files.value.filter((file) => isImageFile(file.datei)).map((file) => file.id));
const { getImage } = useUvImages(previewIds);

function previewSrc(file: IFiles): string | undefined {
    return getImage(file.id)?.data;
}


onMounted(async () => {
    await fetchFiles();
});

watch(
    () => (model.value ?? []).join(','),
    () => {
        fetchFiles();
    }
);

let fetchRequestId = 0;

// for each file (pk) in model, fetch the file from the backend
async function fetchFiles() {
    if (!model.value || !Array.isArray(model.value)) {
        files.value = [];
        return;
    }
    const requestId = ++fetchRequestId;
    const fetched: IFiles[] = [];
    try {
        for (let i = 0; i < model.value.length; i++) {
            const response = await apiConnector.getXResource('bwd', 'Media', model.value[i]) 
            fetched.push(response as unknown as IFiles);
        }
    } catch (error) {
        console.error('Error fetching files:', error);
    }
    if (requestId === fetchRequestId) {
        files.value = fetched;
    }
}

async function loadContent(file: IFiles) {
    if (contentUrls.value[file.id]) {
        return;
    }
    try {
        contentUrls.value[file.id] = await apiConnector.getMediaContentAsBlobURL(file.id);
    } catch (error) {
        console.error('Error loading file content:', error);
    }
}

async function downloadFile(file: IFiles) {
    try {
        await downloadMediaOriginal(file.id, file.name, file.datei);
    } catch (error) {
        console.error('Download error:', error);
    }
}


async function handleAddFile(dialogRef: any) {
    if (!props.enabled) {
        return;
    }
    errorMessage.value = '';
    try {
        let data: any;

        if (uploadMode.value === 'file') {
            // Handle file upload
            const selected: File[] = Array.isArray(newFile.value.file)
                ? newFile.value.file
                : newFile.value.file ? [newFile.value.file] : [];
            if (selected.length === 0) {
                errorMessage.value = 'Bitte eine Datei auswählen.';
                return;
            }

            const firstFile = selected[0];
            data = {
                name: newFile.value.name || firstFile.name,
                beschreibung: newFile.value.beschreibung,
                datei: firstFile,
                type: firstFile.name.split('.').pop(),
                bauwerk: route.params.bauwerkId,
            };
        } else if (uploadMode.value === 'link') {
            // Handle link addition - download file from URL first
            if (!newFile.value.link || newFile.value.link.trim() === '') {
                errorMessage.value = 'Bitte einen Link angeben.';
                return;
            }

            try {
                // Download the file from the URL
                const response = await fetch(newFile.value.link);
                if (!response.ok) {
                    throw new Error(`HTTP error! status: ${response.status}`);
                }
                
                const blob = await response.blob();
                
                // Extract filename and extension from URL
                let filename = 'downloaded-file';
                let fileType = '';
                
                try {
                    const url = new URL(newFile.value.link);
                    const pathname = url.pathname;
                    const pathParts = pathname.split('/');
                    const lastPart = pathParts[pathParts.length - 1];
                    if (lastPart && lastPart.includes('.')) {
                        filename = lastPart;
                        fileType = lastPart.split('.').pop()?.toLowerCase() || '';
                    }
                } catch {
                    // Fallback filename extraction
                    const parts = newFile.value.link.split('/');
                    const lastPart = parts[parts.length - 1].split(/[#?]/)[0];
                    if (lastPart && lastPart.includes('.')) {
                        filename = lastPart;
                        fileType = lastPart.split('.').pop() || '';
                    }
                }

                // Use provided name if available, otherwise use extracted filename
                const finalFilename = newFile.value.name || filename;
                
                // Create a File object from the blob (keep the extension so the backend stores it)
                const file = new File([blob], filename, { type: blob.type });

                data = {
                    name: finalFilename,
                    beschreibung: newFile.value.beschreibung,
                    datei: file,
                    type: fileType || file.type.split('/')[1] || 'unknown',
                    bauwerk: route.params.bauwerkId,
                };
            } catch (error) {
                console.error("Error downloading file from URL:", error);
                errorMessage.value = 'Fehler beim Herunterladen der Datei von der URL. Bitte überprüfen Sie den Link.';
                return;
            }
        }

        // Call the parent's upload handler
        if (props.onUploadFile) {
            uploading.value = true;
            const success = await props.onUploadFile(data);
            if (success) {
                // Reset form and close on success
                newFile.value = { name: "", file: null, beschreibung: "", link: "" };
                uploadMode.value = 'file';
                dialogRef.value = false;
                fetchFiles();
            } else {
                errorMessage.value = 'Fehler beim Hinzufügen der Datei.';
            }
        } else {
            console.error("Upload handler is not defined.");
            return;
        }
    } catch (error) {
        console.error("Add file error:", error);
        errorMessage.value = 'Fehler beim Hinzufügen der Datei.';
    } finally {
        uploading.value = false;
    }
}

async function handleDelete(targetFile: IFiles, onClose?: () => void) {
    if (!props.enabled) {
        return;
    }
    if (!props.onDeleteFile) {
        console.error("Delete handler is not defined.");
        return;
    }
    try {
        const success = await props.onDeleteFile(targetFile);
        if (success) {
            // Optimistically update UI list
            files.value = files.value.filter(f => f.id !== targetFile.id);
            // Remove id from bound model so future fetches don't reintroduce it
            if (model.value) {
                model.value = model.value.filter(id => id !== targetFile.id);
            }
            // Clean up show state
            delete show.value[targetFile.id];
            if (onClose) onClose();
        } else {
            console.error("Failed to delete file in the parent.");
        }
    } catch (e) {
        console.error("Delete error:", e);
    }
}

</script>

<style scoped>
.file-frame {
    height: 70vh;
    border: 0;
}
</style>
