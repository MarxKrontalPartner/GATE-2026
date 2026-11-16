/** Read-only Bauwerk workspace data. All Tauri reads pass through existing local SeaORM commands.
 * A Bauwerk can be linked to several Projekte; Untersuchungsstellen belong to one Bauwerk
 * and may additionally be assigned to one Projekt. Never infer a project from the Bauwerk alone.
 */
import { computed, ref, watch, type Ref } from 'vue';
import type { IBauwerk, IProjekt, IUntersuchungsstelle } from '@/types/apiInterfaces';
import { apiConnector } from '@/utils/ApiConnector';
import { getErrorMessage } from '@/utils/errors';

import { workspaceStellenForProjekt } from './workspaceFilters';
export { workspaceStellenForProjekt } from './workspaceFilters';

export function useBauwerkWorkspace(bauwerkId: Ref<number>) {
  const bauwerk = ref<IBauwerk | null>(null);
  const projekte = ref<IProjekt[]>([]);
  const untersuchungsstellen = ref<IUntersuchungsstelle[]>([]);
  const loading = ref(false);
  const error = ref('');
  let requestId = 0;

  async function refresh(): Promise<void> {
    const current = ++requestId;
    const id = bauwerkId.value;
    bauwerk.value = null;
    projekte.value = [];
    untersuchungsstellen.value = [];
    error.value = '';
    if (!Number.isSafeInteger(id) || id <= 0) {
      error.value = 'Ungültige Bauwerk-ID.';
      loading.value = false;
      return;
    }
    loading.value = true;
    try {
      const [record, allProjects, allStellen] = await Promise.all([
        apiConnector.getResource('Bauwerk', id) as Promise<IBauwerk>,
        apiConnector.getProjekte(true),
        apiConnector.getXResourceList<IUntersuchungsstelle>('bwd', 'Untersuchungsstelle', undefined, String(id)),
      ]);
      if (current !== requestId) return;
      const linkedIds = new Set((record.projekt ?? []).map(Number));
      bauwerk.value = record;
      projekte.value = allProjects.filter(p => linkedIds.has(p.id))
        .sort((a, b) => a.projektnummer.localeCompare(b.projektnummer, 'de', { numeric: true }));
      // Restrict by ID as well, even if the remote endpoint ignores the filtering parameter.
      untersuchungsstellen.value = workspaceStellenForProjekt(allStellen.filter(s => s.bauwerk === id));
    } catch (cause) {
      if (current !== requestId) return;
      error.value = getErrorMessage(cause, 'Bauwerksdaten konnten nicht geladen werden.');
    } finally {
      if (current === requestId) loading.value = false;
    }
  }

  const stellenAnzahl = computed(() => untersuchungsstellen.value.length);
  function stellenFuerProjekt(id: number) {
    return workspaceStellenForProjekt(untersuchungsstellen.value, id);
  }

  watch(bauwerkId, () => { void refresh(); }, { immediate: true });
  return { bauwerk, projekte, untersuchungsstellen, loading, error, refresh,
    stellenAnzahl, stellenFuerProjekt };
}
