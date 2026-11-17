import { describe, expect, it } from 'vitest';
import { appendKernbohrungLayer, removeKernbohrungLayer } from '@/utils/kernbohrungLayers';
import { toKernbohrungDraft, validateKernbohrungDraft } from '@/types/kernbohrung';

describe('Kernbohrung Schichtaufbau – manual measurements', () => {
  it('suggests the previous end as the next start, but leaves the end blank', () => {
    const original = [{ id: 'one', bezeichnung: 'Beton', beschreibung: 'A', tiefe_von: 0, tiefe_bis: 120 }];
    const next = appendKernbohrungLayer(original);
    expect(next).toHaveLength(2);
    expect(next[0]).toEqual(original[0]);
    expect(next[1]).toMatchObject({ bezeichnung: '', beschreibung: '', tiefe_von: 120, tiefe_bis: null });
    expect(original).toHaveLength(1);
  });

  it('does not automatically rewrite previous or following Schicht measurements', () => {
    const first = appendKernbohrungLayer([]);
    first[0]!.tiefe_von = 7;
    first[0]!.tiefe_bis = 19;
    const second = appendKernbohrungLayer(first);
    second[1]!.tiefe_von = 23;
    second[1]!.tiefe_bis = 44;
    const afterRemove = removeKernbohrungLayer(second, 0);
    expect(afterRemove[0]!.tiefe_von).toBe(23);
    expect(afterRemove[0]!.tiefe_bis).toBe(44);
    expect(second[0]!.tiefe_von).toBe(7);
    expect(second[0]!.tiefe_bis).toBe(19);
  });

  it('requires both depths, accepts zero, and rejects an inverted range', () => {
    const draft = toKernbohrungDraft({ datum: '2026-11-16' });
    draft.schichtaufbau = appendKernbohrungLayer([]);
    expect(validateKernbohrungDraft(draft)).toMatch(/Start- und Endtiefe/);
    draft.schichtaufbau[0]!.tiefe_bis = 12;
    expect(validateKernbohrungDraft(draft)).toBeNull();
    draft.schichtaufbau[0]!.tiefe_von = 15;
    expect(validateKernbohrungDraft(draft)).toMatch(/Endtiefe/);
  });
});
