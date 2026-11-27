import { describe, expect, it, vi } from 'vitest';
// This test checks pure data grouping/validation, not API or authentication.
// Prevent ApiConnector -> router -> Keycloak from initializing in Node.
vi.mock('@/utils/ApiConnector', () => ({
  apiConnector: {
    postxResource: vi.fn(),
    BWD: { fetchUvData: vi.fn() },
  },
}));

import { flattenStelleMethods } from '../../src/composables/bwd/stelleMethods';
import { groupMethodsByStelle } from '../../src/composables/bwd/useWorkspaceStelleMethods';
import { validateWorkspaceStelle } from '../../src/composables/bwd/useWorkspaceStelleCreate';

describe('Bauwerk workspace Stellen overview', () => {
  it('groups only methods with an explicit matching Stelle ID', () => {
    const rows = flattenStelleMethods({
      VorOrtUntersuchung: {
        Kernbohrung: [
          { id: 7, untersuchungsstelle: 12, sort_order: 2, abgeschlossen: false },
          { id: 4, untersuchungsstelle: 11, sort_order: 1, abgeschlossen: true },
          { id: 5, untersuchungsstelle: 12, sort_order: 1, abgeschlossen: true },
          { id: 6, untersuchungsstelle: null, sort_order: 1 },
        ],
      },
    });
    const grouped = groupMethodsByStelle(rows);
    expect(grouped.get(11)?.map(x => x.id)).toEqual([4]);
    expect(grouped.get(12)?.map(x => x.id)).toEqual([5, 7]);
    expect(grouped.has(0)).toBe(false);
    expect(grouped.has(13)).toBe(false);
  });

  it('requires a name and a project actually assigned to the Bauwerk', () => {
    expect(validateWorkspaceStelle({ name: '  ', bemerkung: '', projekt: 8 }, 22, [8])).toMatch(/Namen/);
    expect(validateWorkspaceStelle({ name: 'S1', bemerkung: '', projekt: 4 }, 22, [8])).toMatch(/Projekt/);
    expect(validateWorkspaceStelle({ name: 'S1', bemerkung: '', projekt: 8 }, 0, [8])).toMatch(/Bauwerk/);
    expect(validateWorkspaceStelle({ name: 'S1', bemerkung: '', projekt: 8 }, 22, [8])).toBeNull();
  });
});
