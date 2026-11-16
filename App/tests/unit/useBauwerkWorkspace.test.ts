import { describe, expect, it } from 'vitest';
import { workspaceStellenForProjekt } from '../../src/composables/workspaceFilters';
import type { IUntersuchungsstelle } from '../../src/types/apiInterfaces';

function stelle(id: number, bauwerk: number, projekt: number | null, name: string): IUntersuchungsstelle {
  return { id, bauwerk, projekt, name, bemerkung: '', lagenbezugHoehe: '', hoehe: null,
    lagenbezugAbstand1: '', abstand1: null, lagenbezugAbstand2: '', abstand2: null };
}

describe('Bauwerk workspace project filtering', () => {
  const rows = [
    stelle(7, 12, 5, 'Punkt 10'),
    stelle(6, 12, 8, 'Punkt 2'),
    stelle(5, 12, 5, 'Punkt 2'),
    stelle(4, 12, null, 'Nicht zugeordnet'),
  ];

  it('shows only Untersuchungsstellen explicitly linked to the selected project', () => {
    expect(workspaceStellenForProjekt(rows, 5).map(s => s.id)).toEqual([5, 7]);
    expect(workspaceStellenForProjekt(rows, 8).map(s => s.id)).toEqual([6]);
    expect(workspaceStellenForProjekt(rows, 99)).toEqual([]);
  });

  it('shows all Stellen in the Bauwerk overview without inventing project links', () => {
    expect(workspaceStellenForProjekt(rows).map(s => s.id)).toEqual([4, 5, 6, 7]);
    expect(rows.map(s => s.id)).toEqual([7, 6, 5, 4]); // no mutation
  });
});
