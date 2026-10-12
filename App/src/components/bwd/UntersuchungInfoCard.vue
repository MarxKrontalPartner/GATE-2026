<template>

    <v-row> 
        <v-col v-if="store.currentUntersuchung?.bilder" cols="12" md="6">
            <v-sheet class="fill-height" :elevation="3" border rounded>
                <UntersuchungsImageCard class="fill-height" :images="store.currentUntersuchung?.bilder" :enabled="props.enabled"
                    @uploadImage="uploadImage" @removeImage="removeImage" @saveChanges="saveImageChanges" />
            </v-sheet>
        </v-col>
        <v-col cols="12" :md="store.currentUntersuchung?.bilder ? 6 : 12">
            <v-sheet class="d-flex align-center justify-center flex-wrap text-center mx-auto px-4 fill-height" :elevation="3" border
                rounded width="100%">
                <v-col v-if="currentUntersuchung.name !== undefined && currentUntersuchung.name !== null" cols="12" md="12" >
                    <v-text-field v-model="currentUntersuchung.name" label="Name" :disabled="!props.enabled" />
                </v-col>

                <v-col cols="12" md="12">
                    <v-text-field 
                        :model-value="currentUntersuchung.id" 
                        label="Untersuchungs-ID" 
                        :disabled="!enabled"
                        readonly />
                </v-col>
                <v-col cols="12">
                    <v-row>
                        <v-col cols="12" md="5">
                            <v-text-field 
                                v-model="currentUntersuchung.datum" 
                                label="Datum" 
                                :disabled="!enabled"
                                type="date" />
                        </v-col>
                        <v-col cols="12" md="5">
                            <v-text-field 
                                v-model="currentUntersuchung.zeit" 
                                label="Zeit" 
                                :disabled="!enabled"
                                type="time" />
                        </v-col>
                        <v-col cols="12" md="2" >
                            <v-btn
                                variant="tonal"
                                color="primary"
                                :disabled="!enabled"
                                @click="setCurrentDateTime"
                                title="Jetzt setzen"
                                class="datetime-now-btn"
                                height="56"
                                width="56">
                                <v-icon :icon="mdiClockEditOutline" size="28"></v-icon>
                            </v-btn>
                        </v-col>
                    </v-row>
                </v-col>
                <v-col cols="12" md="12">
                    <v-text-field 
                        v-model="currentUntersuchung.lufttemperatur" 
                        label="Lufttemperatur" 
                        :disabled="!enabled"
                        type="number"
                        step="0.1"
                        inputmode="decimal"
                        suffix="°C"
                        :prepend-inner-icon="mdiThermometer"
                        hint="Temperatur in Celsius" />
                </v-col>
                <v-col cols="12" md="12">
                    <v-text-field 
                        v-model="currentUntersuchung.luftfeuchte" 
                        label="Luftfeuchtigkeit" 
                        :disabled="!enabled"
                        type="number"
                        step="0.1"
                        inputmode="decimal"
                        suffix="%"
                        :prepend-inner-icon="mdiWaterPercent"
                        hint="Relative Luftfeuchtigkeit in Prozent" />
                </v-col>

                
                <v-col cols="12">
                    <v-autocomplete 
                        v-model="store.currentUntersuchung.bearbeiter" 
                        :items="bearbeiter"
                        label="Bearbeiter" 
                        :disabled="!enabled" 
                        :item-title="(item: IPerson) => `${item.vorname} ${item.name}`" 
                        item-value="id" 
                        chips 
                        multiple 
                        :prepend-inner-icon="mdiAccountMultiple"
                        hint="Verantwortliche Bearbeiter für diese Untersuchung" />
                </v-col>

                
                <v-col cols="12" md="12">
                    <v-textarea 
                        v-model="store.currentUntersuchung.bemerkung" 
                        label="Bemerkung" 
                        :disabled="!enabled" 
                        :prepend-inner-icon="mdiNoteText"
                        hint="Zusätzliche Anmerkungen zur Untersuchung" />
                </v-col>

                <v-col cols="12">
                    <div :class="['abgeschlossen-wrapper', { completed: currentUntersuchung.abgeschlossen }]">
                        <v-checkbox
                            v-model="currentUntersuchung.abgeschlossen"
                            label="Untersuchung abgeschlossen"
                            :color="abgeschlossenColor"
                            :disabled="!props.enabled"
                            hide-details
                            density="comfortable"
                            class="ma-0 pa-0"
                        />
                        <v-chip
                            :color="abgeschlossenColor"
                            variant="tonal"
                            size="small"
                            class="ml-2"
                            label
                        >
                            <v-icon
                              start
                              :icon="currentUntersuchung.abgeschlossen ? mdiCheckCircleOutline : mdiProgressClock"
                              size="18"
                              class="mr-1"
                            />
                            {{ currentUntersuchung.abgeschlossen ? 'Erledigt' : 'In Arbeit' }}
                        </v-chip>
                    </div>
                </v-col>

            </v-sheet>
        </v-col>
    </v-row>



</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { storeToRefs } from 'pinia';
import type { IPerson } from '@/types/apiInterfaces';
import { mdiThermometer, mdiWaterPercent, mdiAccountMultiple, mdiNoteText, mdiCheckCircleOutline, mdiProgressClock, mdiClockEditOutline } from '@mdi/js';

import UntersuchungsImageCard from '@/components/bwd/media/UntersuchungsImageCard.vue';
import { useUntersuchungenStore } from '@/store/bwd/UntersuchungsverfahrenStore';

// The "abgeschlossen" box is shown in the form column.


// Props
const props = defineProps<{
    enabled: boolean;
}>();

const store = useUntersuchungenStore();
const { currentUntersuchung } = storeToRefs(store);

const bearbeiter = ref<IPerson[]>([]);

// Computed helpers for styled "Abgeschlossen" area
const abgeschlossenColor = computed(() => currentUntersuchung.value?.abgeschlossen ? 'success' : 'warning');



store.fetchBearbeiter().then(response => {
    bearbeiter.value = response;
});


function uploadImage(image: any) {
    // Logic to upload an image
    console.log('uploadImage', image);

}

function removeImage(image: number) {
    // Logic to remove an image
    const index = (store.currentUntersuchung.bilder as number[]).indexOf(image);
    if (index > -1) {
        (store.currentUntersuchung.bilder as number[]).splice(index, 1);
    }
}

function saveImageChanges() {
    // Logic to save image changes
    // This could involve making an API call to save the changes
}


function setCurrentDateTime() {
    const now = new Date();
    
    // Set date in YYYY-MM-DD format
    const year = now.getFullYear();
    const month = String(now.getMonth() + 1).padStart(2, '0');
    const day = String(now.getDate()).padStart(2, '0');
    currentUntersuchung.value.datum = `${year}-${month}-${day}`;
    
    // Set time in HH:MM format
    const hours = String(now.getHours()).padStart(2, '0');
    const minutes = String(now.getMinutes()).padStart(2, '0');
    currentUntersuchung.value.zeit = `${hours}:${minutes}`;
}

</script>

<style scoped>
.abgeschlossen-wrapper {
    display: flex;
    align-items: center;
    padding: 4px 12px 4px 4px;
    border: 1px solid var(--v-theme-outline-variant, #cccccc);
    border-radius: 8px;
    background: var(--v-theme-surface-variant, #f5f5f5);
    transition: background-color .25s ease, border-color .25s ease;
}

.abgeschlossen-wrapper.completed {
    background: rgba(var(--v-theme-success-rgb, 56,142,60), 0.10);
    border-color: rgba(var(--v-theme-success-rgb, 56,142,60), 0.6);
}

.abgeschlossen-wrapper :deep(.v-selection-control) {
    /* Reduce left padding of the checkbox to align nicely inside wrapper */
    margin-inline-start: 4px;
}

</style>
