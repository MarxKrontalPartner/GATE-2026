<template>
  <v-sheet :elevation="5" border rounded>
    <v-toolbar :elevation="1">
      <v-toolbar-title>{{ title }}</v-toolbar-title>
    </v-toolbar>
    <v-container>
      <div v-if="schichten.length === 0" class="text-center pa-4">
        <v-chip color="info" size="large" class="mb-4">Keine Schichten definiert</v-chip>
      </div>

      <transition-group name="schicht" tag="div" class="schicht-list"
        @before-enter="beforeEnter" @enter="enter" @leave="leave">
        <v-row v-for="(schicht, index) in schichten" :key="`schicht-${schicht.id || index}`" 
          :data-index="index" class="mb-1 schicht-item">
          <v-col cols="12">
            <v-card variant="outlined" class="pa-2 schicht-card">
              <!-- Delete button in top-right corner -->
              <v-btn v-if="enabled" :icon="mdiDelete" @click="removeSchicht(index)"
                color="error" variant="text" size="small" class="delete-btn-corner" />

              <v-row align="center" class="py-1">
                <v-col cols="12" md="1" class="d-flex flex-column align-center py-1">
                  <v-btn v-if="enabled && index > 0" :icon="mdiChevronUp"
                    @click="moveSchichtUp(index)" color="primary" variant="text"
                    size="x-small" class="mb-1" />
                  <v-btn v-if="enabled && index < schichten.length - 1"
                    :icon="mdiChevronDown" @click="moveSchichtDown(index)"
                    color="primary" variant="text" size="x-small" />
                </v-col>
                <v-col cols="12" md="1" class="d-flex align-center py-1">
                  <div class="text-subtitle-2 font-weight-medium">
                    {{ index + 1 }}
                  </div>
                </v-col>
                <v-col cols="12" md="2" class="">
                  <v-text-field v-model="schicht.bezeichnung" label="Bezeichnung"
                    :disabled="!enabled" density="compact"
                    placeholder="z.B. Betonschicht" hide-details />
                </v-col>
                <v-col cols="12" md="4" class="py-1">
                  <div class="depth-field-wrapper">
                    <div class="depth-from">
                      <v-text-field v-model.number="schicht.tiefe_von"
                        :disabled="true" label="von" density="compact"
                        placeholder="0" hide-details suffix="mm"
                        type="number" min="0" step="0.01" inputmode="decimal" class="depth-input"
                        variant="outlined" />
                    </div>
                    <div class="depth-separator">
                      <v-icon size="16" color="primary">{{ mdiMinus }}</v-icon>
                    </div>
                    <div class="depth-to">
                      <v-text-field v-model.number="schicht.tiefe_bis"
                        @update:model-value="onDepthBisChange(index, $event)"
                        label="bis" :disabled="!enabled" density="compact"
                        placeholder="0" hide-details suffix="mm"
                        type="number" min="0" step="0.01" inputmode="decimal" class="depth-input"
                        variant="outlined" />
                    </div>
                  </div>
                </v-col>
                <v-col cols="12" md="4" class="py-1">
                  <v-text-field v-model="schicht.beschreibung"
                    label="Beschreibung" :disabled="!enabled" density="compact"
                    placeholder="z.B. Beton, leicht porös" hide-details />
                </v-col>
              </v-row>
            </v-card>
          </v-col>
        </v-row>
      </transition-group>

      <div v-if="enabled" class="schicht-actions">
        <v-btn :prepend-icon="mdiPlus" @click="addSchicht" variant="outlined" size="small">
          Schicht hinzufügen
        </v-btn>
      </div>
    </v-container>
  </v-sheet>
</template>

<script setup lang="ts">
import { computed, nextTick } from 'vue';
import { mdiPlus, mdiDelete, mdiChevronUp, mdiChevronDown, mdiMinus } from '@mdi/js';

interface SchichtItem {
  bezeichnung: string;
  tiefe_von: number;
  tiefe_bis: number;
  beschreibung: string;
  id?: string;
}

interface Props {
  modelValue?: SchichtItem[] | null;
  enabled: boolean;
  title?: string;
}

interface Emits {
  (e: 'update:modelValue', value: SchichtItem[]): void;
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: null,
  title: 'Schichtaufbau'
});

const emit = defineEmits<Emits>();

const schichten = computed<SchichtItem[]>(() => props.modelValue ?? []);

function addSchicht() {
  const currentSchichten = schichten.value;
  
  // Determine starting depth for new layer
  let startDepth = 0;
  if (currentSchichten.length > 0) {
    const lastSchicht = currentSchichten[currentSchichten.length - 1];
    startDepth = lastSchicht.tiefe_bis || 0;
  }

  const newSchicht: SchichtItem = {
    bezeichnung: '',
    tiefe_von: startDepth,
    tiefe_bis: startDepth,
    beschreibung: '',
    id: `schicht-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`
  };
  
  const updatedSchichten = [...currentSchichten, newSchicht];
  emit('update:modelValue', updatedSchichten);
}

function removeSchicht(index: number) {
  const currentSchichten = [...schichten.value];
  currentSchichten.splice(index, 1);
  updateDepthProgression(currentSchichten);
  emit('update:modelValue', currentSchichten);
}

// Update depth progression for all layers
function updateDepthProgression(schichten: SchichtItem[]) {
  for (let i = 0; i < schichten.length; i++) {
    if (i === 0) {
      // First layer always starts at 0
      schichten[i].tiefe_von = 0;
    } else {
      // Subsequent layers start where the previous layer ended
      const prevTiefeBis = schichten[i - 1].tiefe_bis;
      schichten[i].tiefe_von = prevTiefeBis || 0;
    }
  }
}

// Handle end depth change
function onDepthBisChange(index: number, newBisValue: number | string) {
  const currentSchichten = [...schichten.value];
  const numValue = typeof newBisValue === 'string' ? parseFloat(newBisValue) || 0 : newBisValue;
  currentSchichten[index].tiefe_bis = numValue;

  // Update all subsequent layers immediately
  updateDepthProgression(currentSchichten);
  emit('update:modelValue', currentSchichten);
}

async function moveSchichtUp(index: number) {
  if (index <= 0) return;

  const currentSchichten = [...schichten.value];

  // Add a small delay to make the animation more noticeable
  const targetCard = document.querySelector(`[data-index="${index}"] .schicht-card`);
  if (targetCard) {
    targetCard.classList.add('moving-up');
    setTimeout(() => targetCard.classList.remove('moving-up'), 300);
  }

  [currentSchichten[index - 1], currentSchichten[index]] = [currentSchichten[index], currentSchichten[index - 1]];

  await nextTick();

  // Update depth progression after reordering
  updateDepthProgression(currentSchichten);
  emit('update:modelValue', currentSchichten);
}

async function moveSchichtDown(index: number) {
  if (index >= schichten.value.length - 1) return;

  const currentSchichten = [...schichten.value];

  // Add a small delay to make the animation more noticeable
  const targetCard = document.querySelector(`[data-index="${index}"] .schicht-card`);
  if (targetCard) {
    targetCard.classList.add('moving-down');
    setTimeout(() => targetCard.classList.remove('moving-down'), 300);
  }

  [currentSchichten[index], currentSchichten[index + 1]] = [currentSchichten[index + 1], currentSchichten[index]];

  await nextTick();

  // Update depth progression after reordering
  updateDepthProgression(currentSchichten);
  emit('update:modelValue', currentSchichten);
}

// Animation lifecycle hooks
function beforeEnter(el: Element) {
  const htmlEl = el as HTMLElement;
  htmlEl.style.opacity = '0';
  htmlEl.style.transform = 'translateY(-30px)';
}

function enter(el: Element, done: () => void) {
  const htmlEl = el as HTMLElement;
  const delay = Number(htmlEl.dataset.index) * 50;

  setTimeout(() => {
    htmlEl.style.transition = 'all 0.3s ease-out';
    htmlEl.style.opacity = '1';
    htmlEl.style.transform = 'translateY(0)';

    setTimeout(done, 300);
  }, delay);
}

function leave(el: Element, done: () => void) {
  const htmlEl = el as HTMLElement;
  htmlEl.style.transition = 'all 0.3s ease-in';
  htmlEl.style.opacity = '0';
  htmlEl.style.transform = 'translateX(-100%)';

  setTimeout(done, 300);
}
</script>

<style scoped>
.schicht-actions {
  display: flex;
  justify-content: flex-end;
  margin-top: 16px;
}

.schicht-list {
  position: relative;
}

.schicht-item {
  transition: all 0.3s cubic-bezier(0.25, 0.8, 0.25, 1);
}

.schicht-card {
  transition: all 0.3s cubic-bezier(0.25, 0.8, 0.25, 1);
  transform-origin: center;
}

.schicht-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.12), 0 2px 4px rgba(0, 0, 0, 0.08);
}

.schicht-card.moving-up {
  transform: translateY(-8px) scale(1.02);
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.15);
  z-index: 10;
}

.schicht-card.moving-down {
  transform: translateY(8px) scale(1.02);
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.15);
  z-index: 10;
}

/* Transition group animations */
.schicht-enter-active,
.schicht-leave-active {
  transition: all 0.3s cubic-bezier(0.25, 0.8, 0.25, 1);
}

.schicht-enter-from {
  opacity: 0;
  transform: translateY(-20px) scale(0.95);
}

.schicht-leave-to {
  opacity: 0;
  transform: translateX(-100%) scale(0.95);
}

.schicht-move {
  transition: transform 0.3s cubic-bezier(0.25, 0.8, 0.25, 1);
}

/* Delete button positioning */
.delete-btn-corner {
  position: absolute !important;
  top: 8px;
  right: 8px;
  z-index: 10;
}

/* Button hover animations */
.v-btn {
  transition: all 0.2s ease;
}

.v-btn:hover {
  transform: scale(1.05);
}

.v-btn:active {
  transform: scale(0.95);
}

/* Depth display styling */
.depth-container {
  width: 100%;
}

.depth-field-wrapper {
  display: flex;
  align-items: center;
  gap: 8px;
  border: 1px solid rgba(var(--v-theme-outline), 0.2);
  border-radius: 8px;
  padding: 8px 12px;
  transition: all 0.2s ease;
}
</style>
