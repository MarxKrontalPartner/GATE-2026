/** Temporary negative IDs are valid locally, but are not server IDs.
 * Bohrkern must only be created on the server after its Kernbohrung is synced.
 */
export type CoreCreationState = 'missing-investigation' | 'needs-sync' | 'ready';

export function coreCreationState(id: unknown): CoreCreationState {
  if (id === null || id === undefined || id === '') return 'missing-investigation';
  const value = Number(id);
  if (!Number.isSafeInteger(value) || value === 0) return 'missing-investigation';
  return value < 0 ? 'needs-sync' : 'ready';
}
