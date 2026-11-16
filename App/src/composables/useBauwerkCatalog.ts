import { computed, ref } from 'vue';
import { storeToRefs } from 'pinia';
import { useBauwerkStore } from '@/store/BauwerkStore';
import { useProjekte } from '@/composables/projekte';
import { getBauwerkeListWithFeedback } from '@/utils/FeedbackHooks';

// UI-independent search/filtering and the existing local-data refresh flow.
export function useBauwerkCatalog() {
  const { projektList, refreshProjektList } = useProjekte();
  const { bauwerke } = storeToRefs(useBauwerkStore());
  const selectedItems = ref<string[]>([]);
  const searchItems = computed(() => bauwerke.value.map(b => b.bezeichnung));
  const filteredItems = computed(() => selectedItems.value.length
    ? bauwerke.value.filter(b => selectedItems.value.includes(b.bezeichnung))
    : bauwerke.value);

  async function refreshData() {
    await refreshProjektList();
    await getBauwerkeListWithFeedback();
  }

  return { projektList, selectedItems, searchItems, filteredItems, refreshData };
}
