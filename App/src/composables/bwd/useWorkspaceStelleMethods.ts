/** Fetch all locally available methods once for a Bauwerk, then group strictly by Stelle ID.
 * No per-card requests and no attribution by Bauwerk/Projekt alone.
 */
import { computed, ref, watch, type Ref } from 'vue';
import { apiConnector } from '@/utils/ApiConnector';
import { getErrorMessage } from '@/utils/errors';
import { flattenStelleMethods, type StelleMethod, type GroupedStelleMethods } from './stelleMethods';

export function groupMethodsByStelle(methods: StelleMethod[]): Map<number, StelleMethod[]> {
  const grouped = new Map<number, StelleMethod[]>();
  for (const method of methods) {
    if (method.untersuchungsstelle === null) continue;
    const group = grouped.get(method.untersuchungsstelle) ?? [];
    group.push(method);
    grouped.set(method.untersuchungsstelle, group);
  }
  for (const group of grouped.values()) {
    group.sort((a, b) => a.sort_order - b.sort_order || a.id - b.id);
  }
  return grouped;
}

export function useWorkspaceStelleMethods(bauwerkId: Ref<number>) {
  const methods = ref<StelleMethod[]>([]);
  const loading = ref(false);
  const error = ref('');
  let generation = 0;

  async function refresh(): Promise<void> {
    const token = ++generation;
    const id = bauwerkId.value;
    methods.value = [];
    error.value = '';
    if (!Number.isSafeInteger(id) || id <= 0) return;
    loading.value = true;
    try {
      const grouped = await apiConnector.BWD.fetchUvData('all', String(id)) as GroupedStelleMethods;
      if (token === generation) methods.value = flattenStelleMethods(grouped);
    } catch (cause) {
      if (token === generation) error.value = getErrorMessage(cause, 'Untersuchungen konnten nicht geladen werden.');
    } finally {
      if (token === generation) loading.value = false;
    }
  }

  const methodsByStelle = computed(() => groupMethodsByStelle(methods.value));
  watch(bauwerkId, () => { void refresh(); }, { immediate: true });
  return { methodsByStelle, loading, error, refresh };
}
