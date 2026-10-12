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

<style scoped>
.navigation-card {
  background: rgb(var(--v-theme-surface));
  border: 1px solid rgba(var(--v-theme-primary), 0.14);
  box-shadow: 0 10px 28px rgba(0, 0, 0, 0.1);
  border-radius: 18px;
}

.navigation-card--dark {
  background: rgb(var(--v-theme-surface-darken-1));
}

.navigation-content {
  padding: 20px 24px;
}

.integrated-header.header-grid {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  align-items: center;
  gap: 12px 24px;
  margin-bottom: 4px;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0;
}

.header-icon-compact {
  flex: 0 0 auto;
  width: 40px;
  height: 40px;
  background: rgba(var(--v-theme-primary), 0.1);
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid rgba(var(--v-theme-primary), 0.22);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.12);
}

.header-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.header-title {
  margin: 0;
  font-size: 1.2rem;
  font-weight: 600;
  line-height: 1.3;
  color: rgb(var(--v-theme-on-surface));
  letter-spacing: -0.01em;
}

.header-subtitle {
  margin: 0;
  font-size: 0.875rem;
  color: rgba(var(--v-theme-on-surface), 0.7);
}

.header-actions-row {
  display: flex;
  justify-content: flex-end;
  align-items: center;
}

.header-actions-group {
  display: inline-flex;
  gap: 6px;
  align-items: center;
  background: rgba(var(--v-theme-primary), 0.04);
  padding: 4px 10px 4px 8px;
  border-radius: 14px;
  border: 1px solid rgba(var(--v-theme-primary), 0.16);
}

.refresh-button-compact {
  min-width: 36px;
  border: 1px solid rgba(var(--v-theme-primary), 0.2);
  transition: transform 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.refresh-button-compact.icon-btn:hover {
  transform: rotate(25deg) scale(1.05);
}

.progress-toggle-btn {
  font-size: 0.75rem;
  font-weight: 600;
  letter-spacing: .02em;
}

.navigation-breadcrumb {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
  padding: 4px 0;
}

.navigation-chip {
  max-width: 100%;
  font-weight: 500;
  letter-spacing: 0.02em;
  cursor: pointer;
  box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1);
  transition: transform 0.2s cubic-bezier(0.4, 0, 0.2, 1), box-shadow 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.navigation-chip:hover {
  transform: translateY(-1px);
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.16);
}

.chip-text {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

@media (max-width: 599px) {
  .navigation-content {
    padding: 14px 16px;
  }

  .header-icon-compact {
    width: 36px;
    height: 36px;
  }

  .header-title {
    font-size: 1.1rem;
  }

  .header-subtitle {
    font-size: 0.8rem;
  }

  .progress-toggle-btn {
    min-width: 0;
    padding: 0 8px;
  }
}

@media (max-width: 399px) {
  .integrated-header.header-grid {
    grid-template-columns: minmax(0, 1fr);
  }

  .header-actions-row {
    justify-content: flex-start;
  }
}

@media (prefers-reduced-motion: reduce) {
  .navigation-chip,
  .refresh-button-compact {
    transition: none;
  }

  .navigation-chip:hover,
  .refresh-button-compact.icon-btn:hover {
    transform: none;
  }
}
</style>
