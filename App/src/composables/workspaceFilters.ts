import type { IUntersuchungsstelle } from '@/types/apiInterfaces';

export function workspaceStellenForProjekt(
  stellen: IUntersuchungsstelle[], projektId: number | null = null,
): IUntersuchungsstelle[] {
  return stellen
    .filter(item => projektId === null || item.projekt === projektId)
    .sort((a, b) => a.name.localeCompare(b.name, 'de', { numeric: true }) || a.id - b.id);
}

