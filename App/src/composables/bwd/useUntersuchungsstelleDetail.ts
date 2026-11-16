/** Local-first detail screen: the Tauri adapter reads/writes SeaORM entities.
 * Do not alter a Stelle's project or Bauwerk from this detail form.
 */
import { computed, ref, watch, type Ref } from 'vue';
import type { IBauwerk, IProjekt, IUntersuchungsstelle } from '@/types/apiInterfaces';
import { apiConnector } from '@/utils/ApiConnector';
import { getErrorMessage } from '@/utils/errors';
import { flattenStelleMethods, methodsForStelle, type GroupedStelleMethods } from './stelleMethods';

export type StelleEdits = Pick<IUntersuchungsstelle, 'name' | 'bemerkung' |
  'lagenbezugHoehe' | 'hoehe' | 'lagenbezugAbstand1' | 'abstand1' |
  'lagenbezugAbstand2' | 'abstand2'>;

export function useUntersuchungsstelleDetail(bauwerkId: Ref<number>, stelleId: Ref<number>) {
  const bauwerk = ref<IBauwerk | null>(null);
  const stelle = ref<IUntersuchungsstelle | null>(null);
  const projekt = ref<IProjekt | null>(null);
  const methods = ref<ReturnType<typeof methodsForStelle>>([]);
  const loading = ref(false);
  const saving = ref(false);
  const error = ref('');
  let generation = 0;

  async function refresh(): Promise<void> {
    const token = ++generation;
    const bId = bauwerkId.value;
    const sId = stelleId.value;
    stelle.value = null;
    bauwerk.value = null;
    projekt.value = null;
    methods.value = [];
    error.value = '';
    if (![bId, sId].every(id => Number.isSafeInteger(id) && id > 0)) {
      error.value = 'Ungültige Bauwerk- oder Untersuchungsstellen-ID.';
      return;
    }
    loading.value = true;
    try {
      const [current, structure, projects, grouped] = await Promise.all([
        apiConnector.getXResource<IUntersuchungsstelle>('bwd', 'Untersuchungsstelle', sId),
        apiConnector.getResource('Bauwerk', bId) as Promise<IBauwerk>,
        apiConnector.getProjekte(true),
        apiConnector.BWD.fetchUvData('all', String(bId)) as Promise<GroupedStelleMethods>,
      ]);
      if (generation !== token) return;
      if (!current || Number(current.bauwerk) !== bId) {
        error.value = 'Diese Untersuchungsstelle gehört nicht zum gewählten Bauwerk.';
        return;
      }
      bauwerk.value = structure;
      stelle.value = current;
      projekt.value = projects.find(p => p.id === Number(current.projekt)) ?? null;
      methods.value = methodsForStelle(flattenStelleMethods(grouped), sId);
    } catch (cause) {
      if (generation === token) error.value = getErrorMessage(cause, 'Untersuchungsstelle konnte nicht geladen werden.');
    } finally {
      if (generation === token) loading.value = false;
    }
  }

  async function save(changes: StelleEdits): Promise<void> {
    if (!stelle.value || saving.value) return;
    saving.value = true;
    const currentStelleId = stelle.value.id;
    try {
      const result = await apiConnector.patchxResource('bwd', 'Untersuchungsstelle', changes, currentStelleId);
      // Keep the dialog stable during saving. A full reload would briefly unmount it.
      // Both Django PATCH and the local SeaORM command return the saved record.
      if (stelle.value?.id === currentStelleId) {
        stelle.value = { ...stelle.value, ...changes, ...(result as Partial<IUntersuchungsstelle>) };
      }
    } finally { saving.value = false; }
  }

  const completed = computed(() => methods.value.filter(m => m.abgeschlossen === true).length);
  watch([bauwerkId, stelleId], () => { void refresh(); }, { immediate: true });
  return { bauwerk, stelle, projekt, methods, completed, loading, saving, error, refresh, save };
}
