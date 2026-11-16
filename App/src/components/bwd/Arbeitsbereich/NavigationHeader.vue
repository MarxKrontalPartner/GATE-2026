<template>
  <v-card
    class="mb-4 navigation-card"
    :class="{ 'navigation-card--dark': isDarkTheme }"
    elevation="2"
  >
    <v-card-text class="navigation-content">
      <!-- Header Section -->
      <div class="integrated-header header-grid">
        <div class="header-left">
          <div class="header-icon-compact subtle-elev">
            <v-icon color="primary" size="24">{{ mdiSitemap }}</v-icon>
          </div>
          <div class="header-info">
            <h1 class="header-title">Arbeitsbereich</h1>
            <p class="header-subtitle" :title="structureName">Untersuchungsstellen des Bauwerks</p>
          </div>
        </div>
        <div class="header-actions-row">
          <div class="header-actions-group">
            <v-btn
              class="progress-toggle-btn"
              variant="tonal"
              size="small"
              :color="showProgress ? 'primary' : 'grey-lighten-1'"
              :aria-pressed="showProgress"
              @click="$emit('toggle-progress')"
            >
              <v-icon size="16" class="mr-1">{{ showProgress ? mdiCheck : mdiProgressClock }}</v-icon>
              Fortschritt
              <v-tooltip activator="parent" location="bottom">
                Fortschritt {{ showProgress ? 'ausblenden' : 'anzeigen' }}
              </v-tooltip>
            </v-btn>
            <v-btn
              color="primary"
              variant="text"
              size="small"
              class="refresh-button-compact icon-btn"
              :loading="loading"
              aria-label="Struktur aktualisieren"
              @click="$emit('refresh')"
            >
              <v-icon size="18">{{ mdiRefresh }}</v-icon>
              <v-tooltip activator="parent" location="bottom">
                Struktur aktualisieren
              </v-tooltip>
            </v-btn>
          </div>
        </div>
      </div>

      <!-- Breadcrumb Navigation -->
      <v-divider class="my-3" />
      <nav class="navigation-breadcrumb" aria-label="Strukturpfad">
        <v-chip
          v-for="(item, index) in navigationPath"
          :key="`${item.type}-${item.id}-${index}`"
          :color="index === navigationPath.length - 1 ? 'primary' : 'default'"
          :variant="index === navigationPath.length - 1 ? 'flat' : 'tonal'"
          size="default"
          class="navigation-chip"
          :prepend-icon="getNodeIcon(item.type)"
          @click="$emit('navigate-to-level', index)"
        >
          <span class="chip-text">{{ item.name }}</span>
          <v-tooltip activator="parent" location="top">
            {{ getTypeDisplayName(item.type) }}: {{ item.name }}
          </v-tooltip>
        </v-chip>
      </nav>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { mdiSitemap, mdiRefresh, mdiCheck, mdiProgressClock, mdiFormatListBulleted, mdiDomain, mdiMapMarker, mdiPackageVariantClosed } from '@mdi/js';
import { useTheme } from 'vuetify';

interface NavigationItem {
  id: number;
  name: string;
  type: string;
}

interface Props {
  structureName?: string;
  navigationPath: NavigationItem[];
  showProgress: boolean;
  loading: boolean;
}

defineProps<Props>();

const theme = useTheme();
const isDarkTheme = computed(() => theme.global.current.value.dark);

defineEmits<{
  'toggle-progress': [];
  'refresh': [];
  'navigate-to-level': [levelIndex: number];
}>();

function getNodeIcon(nodeType: string): string {
  const iconMapping: { [key: string]: string } = {
    'bauwerkswahl': mdiFormatListBulleted,
    'bauwerk': mdiDomain,
    'untersuchungsstelle': mdiMapMarker
  };
  return iconMapping[nodeType] || mdiPackageVariantClosed;
}

function getTypeDisplayName(nodeType: string): string {
  const displayNames: { [key: string]: string } = {
    'bauwerkswahl': 'Übersicht',
    'bauwerk': 'Bauwerk',
    'untersuchungsstelle': 'Untersuchungsstelle'
  };
  return displayNames[nodeType] || nodeType;
}
</script>

<style scoped lang="scss" src="./NavigationHeader.styles.scss"></style>
