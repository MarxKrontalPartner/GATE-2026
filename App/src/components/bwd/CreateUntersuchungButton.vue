<template>
  <v-btn
    :prepend-icon="buttonPrependIcon"
    :color="buttonColor"
    :variant="buttonVariant"
    :size="buttonSize"
    :density="buttonDensity"
    :rounded="buttonRounded"
    :class="buttonClass"
    @click="openDialog"
  >
    {{ buttonLabel }}
    <v-tooltip activator="parent" location="top">
      Neue Untersuchung erstellen
    </v-tooltip>
  </v-btn>

  <!-- Create Untersuchung Dialog -->
  <v-dialog 
    v-model="showDialog" 
    persistent 
    max-width="600px"
    class="create-untersuchung-dialog"
  >
    <v-card class="dialog-card">
      <!-- Dialog Header -->
      <div class="dialog-header">
        <div class="dialog-title-content">
          <div class="dialog-icon">
            <v-icon :icon="mdiPlus" size="20" color="white" />
          </div>
          <div>
            <h3 class="dialog-title">Neue Untersuchung erstellen</h3>
            <p class="dialog-subtitle">
              {{ untersuchungsstelle?.name || 'Unbekannt' }}
            </p>
          </div>
        </div>
      </div>
      
      <v-divider class="dialog-divider" />
      
      <!-- Dialog Content -->
      <div class="dialog-content">
        <v-form 
          ref="formRef" 
          v-model="formValid" 
          @submit.prevent="createUntersuchung"
          class="dialog-form"
        >
          <!-- Art Selection -->
          <v-select
            v-model="formData.art"
            :items="artOptions"
            label="Art der Untersuchung"
            variant="outlined"
            density="comfortable"
            :rules="[rules.required]"
            class="form-field"
            @update:model-value="clearUntersuchungSelection"
          ></v-select>
          
          <!-- Untersuchung Type Selection -->
          <v-select
            v-model="formData.untersuchung"
            :items="untersuchungOptions"
            label="Untersuchungsverfahren"
            variant="outlined"
            density="comfortable"
            :rules="[rules.required]"
            :disabled="!formData.art || loading"
            :loading="loading"
            class="form-field"
          ></v-select>
          
          <!-- Bemerkung Field -->
          <v-textarea
            v-model="formData.bemerkung"
            label="Bemerkung (optional)"
            variant="outlined"
            density="comfortable"
            rows="2"
            class="form-field"
          ></v-textarea>
        </v-form>
      </div>
      
      <v-divider class="dialog-divider" />
      
      <!-- Dialog Actions -->
      <v-card-actions class="dialog-actions">
        <v-btn 
          variant="text" 
          @click="closeDialog"
          class="cancel-btn"
          :disabled="creating"
        >
          Abbrechen
        </v-btn>
        <v-spacer />
        <v-btn 
          color="primary" 
          variant="flat"
          @click="createUntersuchung"
          :loading="creating"
          :disabled="!formValid"
          class="save-btn"
        >
          {{ creating ? 'Wird erstellt...' : 'Erstellen' }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { useUntersuchungsverfahren } from '@/composables/useUntersuchungsverfahren';
import { useFeedbackStore } from '@/store/FeedbackStore';
import { apiConnector } from '@/utils/ApiConnector';
import { getErrorMessage } from '@/utils/errors';
import { getMethodDisplayName, normalizeMethodKey } from '@/utils/UntersuchungsverfahrenRegistry';
import { mdiPlus } from '@mdi/js';
import { computed, ref } from 'vue';
import { useRoute } from 'vue-router';

// Props
interface UntersuchungsstelleInfo {
  id: number;
  name: string;
  projekt: number | null;
}

const props = defineProps<{
  untersuchungsstelle: UntersuchungsstelleInfo;
  buttonVariant?: 'flat' | 'text' | 'elevated' | 'tonal' | 'outlined' | 'plain';
  buttonColor?: string;
  buttonSize?: 'x-small' | 'small' | 'default' | 'large' | 'x-large';
  buttonDensity?: 'default' | 'comfortable' | 'compact';
  buttonRounded?: string | number | boolean;
  buttonClass?: string;
  buttonLabel?: string;
  buttonPrependIcon?: string;
}>();

const buttonVariant = computed(() => props.buttonVariant ?? 'tonal');
const buttonColor = computed(() => props.buttonColor ?? 'primary');
const buttonSize = computed(() => props.buttonSize ?? 'small');
const buttonDensity = computed(() => props.buttonDensity ?? 'comfortable');
const buttonRounded = computed(() => props.buttonRounded ?? false);
const buttonClass = computed(() => props.buttonClass ?? '');
const buttonLabel = computed(() => props.buttonLabel ?? 'Neue Untersuchung');
const buttonPrependIcon = computed(() => props.buttonPrependIcon ?? mdiPlus);

// Emits
const emit = defineEmits<{
  created: [untersuchung: any, art: UntersuchungsverfahrenArtSelection, untersuchungsverfahren: string];
}>();

// The bauwerk comes from the route and the projekt from the Untersuchungsstelle.
const route = useRoute();
const feedbackStore = useFeedbackStore();

// Reactive data
const showDialog = ref(false);
const formValid = ref(false);
const creating = ref(false);
const formRef = ref();

// API-based Untersuchungsverfahren data
type ImplementedUntersuchungsverfahrenResponse = Record<'VorOrtUntersuchung' | 'Laboruntersuchung', string[]>
type UntersuchungsverfahrenArtSelection = keyof ImplementedUntersuchungsverfahrenResponse

const implementedVerfahren = ref<ImplementedUntersuchungsverfahrenResponse>({
  VorOrtUntersuchung: [],
  Laboruntersuchung: [],
});
const loading = ref(false);
const implementedVerfahrenLoaded = ref(false);

// Form data
const formData = ref<{
  art: UntersuchungsverfahrenArtSelection | '';
  untersuchung: string;
  bemerkung: string;
}>({
  art: '',
  untersuchung: '',
  bemerkung: ''
});

// Art options
const artOptions = [
  { title: 'Vor-Ort-Untersuchung', value: 'VorOrtUntersuchung' },
  { title: 'Laboruntersuchung', value: 'Laboruntersuchung' }
];

// Fetch implemented Untersuchungsverfahren from API
async function fetchImplementedUntersuchungsverfahren() {
  try {
    loading.value = true;
    const response = await apiConnector.getXResourceList('bwd', 'ImplementedUntersuchungsverfahren') as unknown as ImplementedUntersuchungsverfahrenResponse;
    implementedVerfahren.value = {
      VorOrtUntersuchung: response.VorOrtUntersuchung.map((verfahren) => normalizeMethodKey(verfahren)),
      Laboruntersuchung: response.Laboruntersuchung.map((verfahren) => normalizeMethodKey(verfahren)),
    };
    implementedVerfahrenLoaded.value = true;
  } catch (error) {
    implementedVerfahren.value = {
      VorOrtUntersuchung: [],
      Laboruntersuchung: [],
    };
    implementedVerfahrenLoaded.value = false;
    feedbackStore.addWarning(getErrorMessage(error, 'Untersuchungsverfahren konnten nicht geladen werden.'));
  } finally {
    loading.value = false;
  }
}

// Computed untersuchung options based on selected art and API data
const untersuchungOptions = computed(() => {
  const selectedArt = formData.value.art

  if (!selectedArt || !implementedVerfahren.value[selectedArt]) {
    return [];
  }
  
  const verfahrenList: string[] = implementedVerfahren.value[selectedArt];

  return verfahrenList.map((verfahren: string) => ({
    title: getMethodDisplayName(verfahren),
    value: verfahren
  }));
});

// Form validation rules
const rules = {
  required: (value: string) => !!value || 'Dieses Feld ist erforderlich'
};

// Composable for API calls
const untersuchungsverfahrenComposable = computed(() => {
  if (formData.value.art) {
    return useUntersuchungsverfahren(formData.value.art as 'VorOrtUntersuchung' | 'Laboruntersuchung');
  }
  return null;
});

// Methods
async function openDialog() {
  resetForm();
  showDialog.value = true;

  if (!implementedVerfahrenLoaded.value && !loading.value) {
    await fetchImplementedUntersuchungsverfahren();
  }
}

function closeDialog() {
  showDialog.value = false;
  resetForm();
}

function resetForm() {
  formData.value = {
    art: '',
    untersuchung: '',
    bemerkung: ''
  };
  if (formRef.value) {
    formRef.value.resetValidation();
  }
}

function clearUntersuchungSelection() {
  formData.value.untersuchung = '';
}

async function createUntersuchung() {
  if (!formValid.value || !untersuchungsverfahrenComposable.value || !formData.value.art) {
    feedbackStore.addError('Bitte fuellen Sie alle Pflichtfelder aus.');
    return;
  }

  const bauwerkId = route.params.bauwerkId?.toString();
  if (!props.untersuchungsstelle.projekt || !bauwerkId) {
    feedbackStore.addError('Projekt- oder Bauwerkskontext fehlt. Der Untersuchungsstelle ist kein Projekt zugeordnet.');
    return;
  }
  
  creating.value = true;
  
  try {
    const createdMethodLabel = getMethodDisplayName(formData.value.untersuchung);
    const art = formData.value.art;
    const untersuchungsverfahren = formData.value.untersuchung;

    // Prepare payload
    const payload: Record<string, any> = {
      untersuchungsstelle: props.untersuchungsstelle.id,
      type: formData.value.untersuchung,
      projekt: props.untersuchungsstelle.projekt,
      bauwerk: parseInt(bauwerkId, 10),
      datum: new Date().toISOString().split('T')[0],
      ...(formData.value.bemerkung && { bemerkung: formData.value.bemerkung })
    };

    // Create the untersuchung using the composable
    const createdUntersuchung = await untersuchungsverfahrenComposable.value.create(
      untersuchungsverfahren,
      payload
    );

    // Emit the created event
    emit('created', createdUntersuchung, art, untersuchungsverfahren);

    // Close dialog and show success
    closeDialog();
    feedbackStore.addSuccess(`${createdMethodLabel} wurde erfolgreich erstellt.`);
  } catch (error: any) {
    feedbackStore.addError(getErrorMessage(error, 'Untersuchung konnte nicht erstellt werden.'));
  } finally {
    creating.value = false;
  }
}
</script>

<style scoped>
/* Enhanced Dialog Styling */
.dialog-card {
  border-radius: 16px;
  overflow: hidden;
  box-shadow: 0 24px 38px 3px rgba(0, 0, 0, 0.14), 0 9px 46px 8px rgba(0, 0, 0, 0.12), 0 11px 15px -7px rgba(0, 0, 0, 0.2);
}

.dialog-header {
  background: linear-gradient(135deg, #42c0eb 0%, #12a5e2 100%);
  color: white;
  padding: 24px 32px 20px 32px;
}

.dialog-title-content {
  display: flex;
  align-items: flex-start;
  gap: 16px;
}

.dialog-icon {
  background: rgba(255, 255, 255, 0.15);
  backdrop-filter: blur(10px);
  border-radius: 12px;
  padding: 8px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
  margin-top: 4px;
}

.dialog-title {
  font-size: 1.4rem;
  font-weight: 600;
  margin: 0;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.dialog-subtitle {
  font-size: 0.95rem;
  opacity: 0.9;
  margin: 4px 0 0 0;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
}

.dialog-divider {
  border-color: rgba(var(--v-theme-on-surface), 0.08);
}

.dialog-content {
  padding: 32px;
  background: rgb(var(--v-theme-surface));
}

.dialog-form {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.form-field {
  transition: all 0.2s ease;
}

.form-field:focus-within {
  transform: translateY(-1px);
  box-shadow: 0 4px 8px rgba(66, 192, 235, 0.15);
}

.dialog-actions {
  padding: 24px 32px;
  background: rgb(var(--v-theme-surface));
  gap: 12px;
}

.cancel-btn {
  min-width: 100px;
  transition: all 0.2s ease;
}

.save-btn {
  min-width: 120px;
  box-shadow: 0 4px 8px rgba(66, 192, 235, 0.3);
  transition: all 0.2s ease;
}

.save-btn:hover {
  transform: translateY(-1px);
  box-shadow: 0 6px 12px rgba(66, 192, 235, 0.4);
}

/* Responsive design */
@media (max-width: 599px) {
  .dialog-content {
    padding: 24px;
  }
  
  .dialog-actions {
    padding: 20px 24px;
  }
  
  .dialog-header {
    padding: 20px 24px 16px 24px;
  }
  
  .dialog-title {
    font-size: 1.2rem;
  }
  
  .dialog-subtitle {
    font-size: 0.85rem;
  }
}
</style>
