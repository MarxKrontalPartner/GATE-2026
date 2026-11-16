/** Create an Untersuchungsstelle through the established Tauri/SeaORM adapter.
 * The caller must pass a Projekt belonging to the current Bauwerk.
 */
import { ref } from 'vue';
import { apiConnector } from '@/utils/ApiConnector';
import type { IUntersuchungsstelle } from '@/types/apiInterfaces';

export interface NewWorkspaceStelle {
  name: string;
  bemerkung: string;
  projekt: number;
}

export function validateWorkspaceStelle(input: NewWorkspaceStelle, bauwerkId: number, linkedProjectIds: number[]): string | null {
  if (!Number.isSafeInteger(bauwerkId) || bauwerkId <= 0) return 'Ungültiges Bauwerk.';
  if (!input.name.trim()) return 'Bitte einen Namen eingeben.';
  if (!linkedProjectIds.includes(input.projekt)) return 'Bitte ein zugeordnetes Projekt auswählen.';
  return null;
}

export function useWorkspaceStelleCreate() {
  const saving = ref(false);
  async function create(input: NewWorkspaceStelle, bauwerkId: number, linkedProjectIds: number[]): Promise<IUntersuchungsstelle> {
    const validationError = validateWorkspaceStelle(input, bauwerkId, linkedProjectIds);
    if (validationError) throw new Error(validationError);
    if (saving.value) throw new Error('Eine Untersuchungsstelle wird bereits gespeichert.');
    saving.value = true;
    try {
      return await apiConnector.postxResource('bwd', 'Untersuchungsstelle', {
        bauwerk: bauwerkId,
        projekt: input.projekt,
        name: input.name.trim(),
        bemerkung: input.bemerkung,
      }) as IUntersuchungsstelle;
    } finally {
      saving.value = false;
    }
  }
  return { saving, create };
}
