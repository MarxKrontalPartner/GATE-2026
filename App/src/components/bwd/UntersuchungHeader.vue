<template>

      <!-- Hierarchy Navigation (Scrollable) -->
    <div class="hierarchy-container rounded-lg px-3 py-2 border-thin">
        <div class="hierarchy-scroll-area d-flex align-center">
          <!-- Path Items -->
          <template v-for="(node, index) in hierarchyPath" :key="`${node.type}-${node.id}`">
            <div class="d-flex align-center flex-shrink-0">
              <v-btn
                  variant="text"
                  density="comfortable"
                  class="hierarchy-btn px-2 text-none"
                  :color="index === hierarchyPath.length - 1 ? 'primary' : 'medium-emphasis'"
                  @click="navigateToLevel(index)"
                  rounded
              >
                  <span v-if="shouldShowTypeLabel(node.type)" class="text-caption mr-1 opacity-70 d-none d-sm-inline-block">{{ getTypeLabel(node.type) }}</span>
                  <span class="font-weight-bold text-body-2">{{ node.name }}</span>
              </v-btn>
              
              <!-- Separator -->
              <v-icon 
                :icon="mdiChevronRight" 
                size="small" 
                color="grey-lighten-1"
                class="mx-1"
              />
            </div>
          </template>
          
          <!-- Current Item (Leaf) -->
          <div class="d-flex align-center flex-shrink-0">
             <span class="text-body-2 font-weight-bold text-high-emphasis ml-1">
               {{ title }}
             </span>
          </div>
        </div>
      </div>

</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useUntersuchungenStore } from '@/store/bwd/UntersuchungsverfahrenStore';
import { useBauwerkStore } from '@/store/BauwerkStore';
import { apiConnector } from '@/utils/ApiConnector';
import { useRouter, useRoute } from 'vue-router';
import { storeToRefs } from 'pinia';
import { useQuery } from '@tanstack/vue-query';
import { getMethodDisplayName } from '@/utils/UntersuchungsverfahrenRegistry';
import type { IUntersuchungsstelle } from '@/types/apiInterfaces';

import { mdiChevronRight } from '@mdi/js';

interface HierarchyNode {
  id: number;
  name: string;
  type: 'bauwerk' | 'untersuchungsstelle';
}

const untersuchungenStore = useUntersuchungenStore();
const { currentUntersuchung } = storeToRefs(untersuchungenStore);
const bauwerkStore = useBauwerkStore();
const router = useRouter();
const route = useRoute();

const title = computed(() => {
    // Try to get a nice name, fallback to type
    if (currentUntersuchung.value?.name) return currentUntersuchung.value.name;
    const type = currentUntersuchung.value?.type || '';
    return `${getMethodDisplayName(type)} ${currentUntersuchung.value?.id ?? ''}`.trim();
});

const bauwerkId = computed(() => {
  return bauwerkStore.selectedBauwerk?.id || parseInt(route.params.bauwerkId as string);
});

const untersuchungsstelleId = computed(() => currentUntersuchung.value?.untersuchungsstelle ?? null);

// Hierarchy: Bauwerk → Untersuchungsstelle.
const { data: untersuchungsstelle } = useQuery({
    queryKey: ['untersuchungsstelle', untersuchungsstelleId],
    queryFn: () => apiConnector.getXResource<IUntersuchungsstelle>('bwd', 'Untersuchungsstelle', untersuchungsstelleId.value as number),
    enabled: computed(() => !!untersuchungsstelleId.value),
    staleTime: 1000 * 60 * 10, // 10 minutes cache
});

const hierarchyPath = computed<HierarchyNode[]>(() => {
    const path: HierarchyNode[] = [];
    if (bauwerkStore.selectedBauwerk) {
        path.push({ id: bauwerkStore.selectedBauwerk.id, name: bauwerkStore.selectedBauwerk.bezeichnung, type: 'bauwerk' });
    }
    if (untersuchungsstelle.value) {
        path.push({ id: untersuchungsstelle.value.id, name: getNodeDisplayName(untersuchungsstelle.value), type: 'untersuchungsstelle' });
    }
    return path;
});

function getNodeDisplayName(node: { name?: string } | null | undefined): string {
    if (!node) return '';
    return node.name || '';
}

function getTypeLabel(type: string): string {
    const labels: Record<string, string> = {
        'bauwerk': 'Bauwerk',
        'untersuchungsstelle': 'US'
    };
    return labels[type] || type;
}

function shouldShowTypeLabel(type: string): boolean {
    return type !== 'untersuchungsstelle';
}

function navigateToLevel(index: number) {
    const node = hierarchyPath.value[index];
    if (!node) return;

    if (node.type === 'bauwerk') {
        router.push({ name: 'Untersuchungsstellen', params: { bauwerkId: bauwerkId.value } });
        return;
    }

    // show all Untersuchungsverfahren of this Untersuchungsstelle in the overview
    router.push({
        name: (route.params.art as string) || 'VorOrtUntersuchung',
        params: { bauwerkId: bauwerkId.value },
        query: { untersuchungsstelle: String(node.id) },
    });
}
</script>

<style scoped>
.hierarchy-container {
    overflow-x: auto;
    -webkit-overflow-scrolling: touch;
    scrollbar-width: none; /* Firefox */
    background: rgb(var(--v-theme-surface-darken-1));
    color: rgb(var(--v-theme-on-surface));
    border-color: rgba(var(--v-theme-border), 0.28);
    box-shadow: 0 8px 18px rgba(0, 0, 0, 0.12);
}

.hierarchy-container::-webkit-scrollbar {
    display: none; /* Chrome/Safari */
}

.hierarchy-scroll-area {
    min-width: min-content;
}

.hierarchy-btn {
    transition: all 0.2s ease;
}

.hierarchy-btn:hover {
    background-color: rgba(var(--v-theme-primary), 0.05);
}

.border-thin {
    border: 1px solid rgba(var(--v-theme-border), 0.28);
}

.opacity-70 {
    opacity: 0.7;
}

:deep(.hierarchy-btn .v-btn__content) {
    color: inherit;
}
</style>
