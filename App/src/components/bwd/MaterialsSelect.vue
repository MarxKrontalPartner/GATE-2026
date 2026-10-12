<template>

    <v-autocomplete v-model="model" :items="materials" label="Material" item-title="name" :variant="props.variant"
        item-value="id" chips multiple :disabled="!props.enabled" :density="props.density" :hide-details="props.hideDetails">
        <template v-slot:append-item>
            <div style="text-align: center;">
                <v-btn color="primary" @click="showAddMaterialDialog = true">+</v-btn>
            </div>
        </template>
    </v-autocomplete>

    <v-dialog v-model="showAddMaterialDialog" max-width="500px">
        <v-card>
            <v-card-title>
                <span class="headline">Neues Material Hinzufügen</span>
            </v-card-title>
            <v-card-text>
                <v-text-field v-model="newMaterial.name" label="Name"></v-text-field>
                <v-text-field v-model="newMaterial.beschreibung" label="Beschreibung"></v-text-field>
            </v-card-text>
            <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn color="blue darken-1"  @click="showAddMaterialDialog = false">Abbrechen</v-btn>
                <v-btn color="blue darken-1"  @click="addMaterial">Hinzufügen</v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>

</template>

<script setup lang="ts">
import { apiConnector } from '@/utils/ApiConnector';
import { ref, onMounted, defineModel, defineProps, type PropType } from 'vue';
import type { Material } from '@/types/Material';

const model = defineModel<any>()

/**
 * @type {{ enabled: boolean }}
 */
const props = defineProps(
    {
        enabled: Boolean,
        variant: {
            type: String as () => 'outlined' | 'filled' | 'solo' | 'solo-inverted' | 'solo-filled' | 'plain' | 'underlined',
            default: 'filled'
        },
        density: {
            type: String as () => 'default' | 'comfortable' | 'compact',
            default: 'default'
        },
        hideDetails: {
            type: [Boolean, String] as PropType<boolean | 'auto'>,
            default: false
        }
    }
);


const materials = ref<Material[]>([]);


const showAddMaterialDialog = ref(false);
const newMaterial = ref({
    name: '',
    beschreibung: ''
});

onMounted(async () => {
    await fetchMaterials();
});


async function fetchMaterials() {
    try {
        const response = await apiConnector.getXResourceList<Material>('bwd', 'Material') 
        materials.value = response
    } catch (error) {
        console.error('Error fetching materials:', error);
    }
}

async function addMaterial() {
    try {
        const response = await apiConnector.postxResource('bwd', 'Material', newMaterial.value);

        const addedMaterial = await response;
        materials.value.push(addedMaterial);

        showAddMaterialDialog.value = false;
        newMaterial.value.name = '';
        newMaterial.value.beschreibung = '';
    } catch (error) {
        console.error('Error adding material:', error);
    }
}
</script>