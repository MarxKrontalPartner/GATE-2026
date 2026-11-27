import { describe, expect, it } from 'vitest';
import { flattenStelleMethods, methodsForStelle } from '../../src/composables/bwd/stelleMethods';

describe('Untersuchungsstelle detail data isolation', () => {
  const response = {
    VorOrtUntersuchung: {
      Rissbreitenmessung: [
        { id: 18, untersuchungsstelle: 3, sort_order: 5, datum: '2026-11-10', abgeschlossen: false },
        { id: 21, untersuchungsstelle: 4, sort_order: 1, datum: '2026-11-11', abgeschlossen: true },
        { id: 17, untersuchungsstelle: 3, sort_order: 2, abgeschlossen: true },
      ],
    },
    Laboruntersuchung: {
      Druckfestigkeitspruefung: [
        { id: 99, untersuchungsstelle: 3, sort_order: 3, abgeschlossen: null },
        { id: 0, untersuchungsstelle: 3 }, // invalid ID ignored
      ],
    },
  };

  it('shows only Verfahren belonging to this exact Stelle, sorted by order', () => {
    const all = flattenStelleMethods(response);
    expect(methodsForStelle(all, 3).map(m => m.id)).toEqual([17, 99, 18]);
    expect(methodsForStelle(all, 4).map(m => m.id)).toEqual([21]);
    expect(methodsForStelle(all, 5)).toEqual([]);
    expect(all.length).toBe(4);
  });

  it('keeps temporary negative IDs for investigations created while offline', () => {
    const methods = flattenStelleMethods({ VorOrtUntersuchung: {
      Chloridanalyse: [
        { id: -1, untersuchungsstelle: 3, datum: '2026-11-25' },
        { id: 0, untersuchungsstelle: 3 },
      ],
    } });
    expect(methodsForStelle(methods, 3).map(method => method.id)).toEqual([-1]);
  });

  it('preserves method categories and completion states without inventing statuses', () => {
    const items = methodsForStelle(flattenStelleMethods(response), 3);
    expect(items.map(m => m.category)).toEqual([
      'VorOrtUntersuchung', 'Laboruntersuchung', 'VorOrtUntersuchung',
    ]);
    expect(items.map(m => m.abgeschlossen)).toEqual([true, null, false]);
  });

  it('never attributes a method with no Stelle foreign key to the selected Stelle', () => {
    const result = flattenStelleMethods({ VorOrtUntersuchung: {
      Kernbohrung: [{ id: 30, untersuchungsstelle: null }],
    } });
    expect(methodsForStelle(result, 3)).toEqual([]);
  });
});
