<!-- InvestigationDetailView.vue -->
<template>
    <div>
        <v-progress-circular v-if="isLoading" indeterminate color="primary"></v-progress-circular>
        <div v-else-if="isRouteInvalid" class="error-container">
            <v-card elevation="2" class="pa-4 mx-auto my-4" max-width="600">
            <v-card-title class="text-center text-h5">
                <v-icon color="warning" size="large" class="me-2" :icon="mdiAlertCircle"></v-icon>
                Ungueltige Route
            </v-card-title>
            <v-card-text class="text-center py-4">{{ invalidRouteMessage }}</v-card-text>
            <v-card-actions class="justify-center">
                <v-btn @click="goBack" color="primary" variant="outlined" class="me-2">
                <v-icon :icon="mdiArrowLeft"></v-icon>
                Zurück
                </v-btn>
                <v-btn @click="goHome" color="primary">
                <v-icon :icon="mdiHome"></v-icon>
                Startseite
                </v-btn>
            </v-card-actions>
            </v-card>
        </div>
        <div v-else-if="isError" class="error-container">
            <v-card elevation="2" class="pa-4 mx-auto my-4" max-width="600">
            <v-card-title class="text-center text-h5">
                <v-icon color="error" size="large" class="me-2" :icon="mdiAlertCircle"></v-icon>
                Fehler
            </v-card-title>
            <v-card-text class="text-center py-4">
                {{ error?.message || `${methodLabel} konnte nicht geladen werden.` }}
            </v-card-text>
            <v-card-actions class="justify-center">
                <v-btn @click="loadDetail" color="primary" variant="flat" class="me-2">
                Erneut versuchen
                </v-btn>
                <v-btn @click="goBack" color="primary" variant="outlined" class="me-2">
                <v-icon :icon="mdiArrowLeft"></v-icon>
                Zurück
                </v-btn>
                <v-btn @click="goHome" color="primary">
                <v-icon  :icon="mdiHome"></v-icon>
                Startseite
                </v-btn>
            </v-card-actions>
            </v-card>
        </div>
        <div v-else-if="currentInvestigationComponent && isDetailReady">
            <component :is="currentInvestigationComponent" :key="`${methodKey}-${id}`" :detail="detail" class="units-persistent"/>
        </div>
        <div v-else class="error-container">
            <v-card elevation="2" class="pa-4 mx-auto my-4" max-width="600">
            <v-card-title class="text-center text-h5">{{ methodLabel }} wird vorbereitet</v-card-title>
            <v-card-text class="text-center py-4">
                Die Detailansicht konnte noch nicht initialisiert werden. Bitte versuchen Sie es erneut.
            </v-card-text>
            <v-card-actions class="justify-center">
                <v-btn @click="loadDetail" color="primary" variant="flat">Erneut laden</v-btn>
            </v-card-actions>
            </v-card>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useUntersuchungenStore } from '@/store/bwd/UntersuchungsverfahrenStore'
import { getMethodDisplayName, normalizeMethodKey } from '@/utils/UntersuchungsverfahrenRegistry'
import { getUntersuchungsverfahrenComponent } from '@/views/bwd/Untersuchungsverfahren/detailViewRegistry'
import { useFeedbackStore } from '@/store/FeedbackStore'

import {mdiAlertCircle, mdiHome, mdiArrowLeft} from '@mdi/js'

// Route and router
const route = useRoute()
const router = useRouter()

// Extract route params reactively
const id = computed(() => Number(route.params.id))
const art = computed(() => route.params.art as string)
const rawUntersuchung = computed(() => (route.params.untersuchung as string)?.toString() ?? '')
const methodKey = computed(() => normalizeMethodKey(rawUntersuchung.value))
const methodLabel = computed(() => getMethodDisplayName(rawUntersuchung.value || 'Untersuchung'))
const invalidRouteMessage = computed(() => {
    if (!route.params.id) {
        return 'Die Untersuchungs-ID fehlt in der URL.'
    }

    if (!art.value) {
        return 'Die Art der Untersuchung fehlt in der URL.'
    }

    if (!rawUntersuchung.value) {
        return 'Das Untersuchungsverfahren fehlt in der URL.'
    }

    return 'Die angeforderte Untersuchung konnte nicht eindeutig bestimmt werden.'
})
const isRouteInvalid = computed(() => !Number.isFinite(id.value) || id.value <= 0 || !art.value || !rawUntersuchung.value)

const store = useUntersuchungenStore()
const feedbackStore = useFeedbackStore()
const detail = computed(() => store.currentUntersuchung)
const isLoading = ref(false)
const isError = ref(false)
const error = ref<Error | null>(null)
let latestLoadRequest = 0

const currentInvestigationComponent = computed(() => getUntersuchungsverfahrenComponent(rawUntersuchung.value))
const isDetailReady = computed(() => {
    if (!detail.value?.id || detail.value.id !== id.value) {
        return false
    }

    return normalizeMethodKey(detail.value.type ?? rawUntersuchung.value) === methodKey.value
})

function goBack() {
    router.go(-1)
}

function goHome() {
    router.push('/')
}

async function loadDetail() {
    if (isRouteInvalid.value) {
        isError.value = false
        error.value = null
        isLoading.value = false
        return
    }

    const requestId = ++latestLoadRequest
    isLoading.value = true
    isError.value = false
    error.value = null

    try {
        await store.fetchUntersuchungDetail(id.value, art.value, methodKey.value)

        if (requestId !== latestLoadRequest) {
            return
        }
    } catch (e: any) {
        if (requestId !== latestLoadRequest) {
            return
        }

        isError.value = true
        error.value = e instanceof Error ? e : new Error('Unbekannter Fehler')
        feedbackStore.addError(error.value.message)
    } finally {
        if (requestId === latestLoadRequest) {
            isLoading.value = false
        }
    }
}

watch([id, methodKey, art], loadDetail, { immediate: true })
</script>

<style scoped>
/* ALWAYS show prefix/suffix in all child components of this view */
.units-persistent :deep(.v-text-field__suffix),
.units-persistent :deep(.v-text-field__prefix) {
  opacity: 1 !important;             /* Vuetify sets these to 0 when empty */
  pointer-events: none;
  white-space: nowrap;
  transition: none;
}
</style>
