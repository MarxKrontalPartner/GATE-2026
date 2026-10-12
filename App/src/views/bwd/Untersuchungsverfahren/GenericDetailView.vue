<template>
  
  <v-container class="max-container-width">
    <v-row>
      <v-col cols="12">
        <UntersuchungHeader />
      </v-col>
    </v-row>
    <FeedbackFeedComponent />

    <!-- Buttons (Edit, Save, Delete, Undo) -->
    <EditAndSaveButton v-model:editInfo="editInfo" :saveChanges="saveChanges" :del="del" :undoChanges="resetCurrent"
      @update:editInfo="onEditModeChange" />
    <slot name="untersuchung" :enabled="editInfo">
      <UntersuchungInfoCard :enabled="editInfo"></UntersuchungInfoCard>
    </slot>
    <div v-if="$slots.content">
    <slot name="content" :enabled="editInfo">

      <v-card class="card-border text-card" outlined>
        <v-card-title>Dieses Untersuchungsverfahren befindet sich noch in der Entwicklung</v-card-title>
        <v-card-text>
          {{ currentUntersuchung }}
        </v-card-text>
      </v-card>
    </slot>
  </div>

  </v-container>
</template>

<script setup lang="ts">
// Imports
import { ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { storeToRefs } from 'pinia'

import { useUntersuchungenStore } from '@/store/bwd/UntersuchungsverfahrenStore'
import { useFeedbackStore, FeedbackType } from '@/store/FeedbackStore'
import UntersuchungHeader from '@/components/bwd/UntersuchungHeader.vue'


// Deine Standard-Komponenten (werden im Default-Fall verwendet)
import FeedbackFeedComponent from '@/components/FeedbackFeedComponent.vue'

import EditAndSaveButton from '@/components/buttons/EditAndSaveButton.vue'
import UntersuchungInfoCard from '@/components/bwd/UntersuchungInfoCard.vue'


// Pinia-Stores
const untersuchungenStore = useUntersuchungenStore()
const feedbackStore = useFeedbackStore()

// Refs
const editInfo = ref(false)
const router = useRouter()
const route = useRoute()

// Hole Untersuchung aus dem Store
const { currentUntersuchung } = storeToRefs(untersuchungenStore)

// Routen-Parameter
const artParam = route.params.art as string
const untersuchungParam = route.params.untersuchung as string
const idParam = parseInt(route.params.id as string)

function buildDeleteRedirectTarget() {
  const bauwerkId = route.params.bauwerkId?.toString()

  if (bauwerkId) {
    return {
      name: artParam,
      params: { bauwerkId },
    }
  }

  return { name: 'bauwerkswahl' }
}



// Beispiel: Änderungen speichern
async function saveChanges() {
  if (!currentUntersuchung.value) return
  try {
    await untersuchungenStore.updateUntersuchung(currentUntersuchung.value.id, currentUntersuchung.value.type, currentUntersuchung.value)
    feedbackStore.addFeedback({
      type: FeedbackType.SUCCESS,
      message: 'Änderungen erfolgreich gespeichert',
      action: ''
    })
  } catch (error: any) {
    feedbackStore.addFeedback({
      type: FeedbackType.ERROR,
      message: error?.message ?? 'Fehler beim Speichern',
      action: ''
    })
  }
}

// Beispiel: Löschen
async function del() {
  if (!currentUntersuchung.value) return
  editInfo.value = false
  try {
    await untersuchungenStore.deleteUntersuchung(idParam, artParam as 'VorOrtUntersuchung' | 'Laboruntersuchung', untersuchungParam)
    const redirectTarget = buildDeleteRedirectTarget()
    untersuchungenStore.clearCurrentUntersuchung()
    feedbackStore.addFeedback({
      type: FeedbackType.SUCCESS,
      message: 'Untersuchung erfolgreich gelöscht',
      action: ''
    })
    router.push(redirectTarget)
  } catch (error: any) {
    feedbackStore.addFeedback({
      type: FeedbackType.ERROR,
      message: error?.message ?? 'Fehler beim Löschen',
      action: ''
    })
  }
}

// Beispiel: Reset / Undo
function resetCurrent() {
  try {
    untersuchungenStore.restoreCurrentUntersuchungFromBackup()
    feedbackStore.addFeedback({
      type: FeedbackType.SUCCESS,
      message: 'Änderungen rückgängig gemacht',
      action: ''
    })
  } catch (error: any) {
    feedbackStore.addFeedback({
      type: FeedbackType.WARNING,
      message: error?.message ?? 'Keine Sicherungskopie verfügbar',
      action: ''
    })
  }
}

// Handle edit mode changes to create backup
function onEditModeChange(isEditMode: boolean) {
  if (isEditMode) {
    // Create backup when entering edit mode
    untersuchungenStore.createCurrentUntersuchungBackup()
  }
}

// Expose editInfo so parent components can access it
defineExpose({
  editInfo
})
</script>

<style scoped>
.max-container-width {
  max-width: 95%;
}

.card-border {
  border-radius: 4px;
  border: 1px solid rgb(var(--v-theme-border));
}

.text-card {
  color: rgb(var(--v-theme-text)) !important;
  font-weight: 400 !important;
}


</style>
