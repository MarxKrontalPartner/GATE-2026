import { describe, expect, it } from 'vitest';
import { reactive } from 'vue';
import { buildInvestigationCreatePayload } from '@/utils/investigationCreatePayload';
import {
  chloridanalysePatch, chloridanalysePersistenceDifferences, toChloridanalyseDraft,
  validateChloridanalyseDraft,
} from '@/types/chloridanalyse';

describe('Chloridanalyse form (SeaORM / offline-first)', () => {
  it('is selectable as a Vor-Ort-Untersuchung using the existing creation flow', () => {
    expect(buildInvestigationCreatePayload({
      bauwerkId: 4, projektId: 6, stelleId: 9, art: 'VorOrtUntersuchung',
      method: 'chloridanalyse', datum: '2026-11-24', zeit: '09:10',
    })).toMatchObject({
      bauwerk: 4, projekt: 6, untersuchungsstelle: 9,
      datum: '2026-11-24', zeit: '09:10:00', abgeschlossen: false,
    });
  });
  it('preserves source field names, decimals and optional/null booleans', () => {
    const draft = toChloridanalyseDraft({
      datum: '2026-11-24', zeit: '13:30:00', probe: 18,
      pruefstelle: 'Labor', analyseverfahren: 'titration',
      chloridgehaltZement: 0.35, zementgehaltAnnahme: 320,
      grenzwertUeberschritten: null, bearbeiter: [5, 2], pruefberichte: [31],
    });
    expect(draft.grenzwertUeberschritten).toBeNull();
    expect(draft.chloridgehaltZement).toBe(0.35);
    expect(validateChloridanalyseDraft(draft)).toBeNull();
    expect(chloridanalysePatch(draft)).toMatchObject({
      zeit: '13:30:00', probe: 18, analyseverfahren: 'titration',
      chloridgehaltZement: 0.35, grenzwertUeberschritten: null,
      pruefberichte: [31],
    });
  });
  it('makes a cloneable save draft from Vue reactive state', () => {
    const state = reactive(toChloridanalyseDraft({
      datum: '2026-11-25', zeit: '14:30:00', pruefstelle: 'Labor Ost',
      chloridgehaltZement: 0.35, probe: 18, bearbeiter: [2, 5],
    }));
    expect(() => structuredClone(state)).toThrow();
    // The form must always normalize proxies before saving through Tauri or HTTP.
    const plain = toChloridanalyseDraft({ ...state });
    expect(() => structuredClone(plain)).not.toThrow();
    expect(chloridanalysePatch(plain)).toMatchObject({
      pruefstelle: 'Labor Ost', chloridgehaltZement: 0.35,
      probe: 18, zeit: '14:30:00', bearbeiter: [2, 5],
    });
    expect(plain.bearbeiter).not.toBe(state.bearbeiter);
  });
  it('reports missing persisted values instead of showing false success', () => {
    const expected = toChloridanalyseDraft({
      datum: '2026-11-24', probe: 18, chloridgehaltZement: 0.42,
      zementgehaltAnnahme: 330, grenzwertUeberschritten: false,
      pruefberichte: [31, 32],
    });
    const lost = { ...chloridanalysePatch(expected), probe: null, chloridgehaltZement: null,
      grenzwertUeberschritten: null, pruefberichte: [31] };
    expect(chloridanalysePersistenceDifferences(expected, lost)).toEqual([
      'Probe', 'Chloridgehalt', 'Grenzwert', 'Prüfberichte',
    ]);
  });
  it('rejects negative measurement values and implausible humidity', () => {
    const draft = toChloridanalyseDraft({ datum: '2026-11-24', chloridgehaltZement: -0.1 });
    expect(validateChloridanalyseDraft(draft)).toMatch(/nicht negative/i);
    draft.chloridgehaltZement = 0;
    draft.luftfeuchte = 101;
    expect(validateChloridanalyseDraft(draft)).toMatch(/100/);
  });
  it('does not automatically infer exceeding a threshold', () => {
    const draft = toChloridanalyseDraft({ datum: '2026-11-24', chloridgehaltZement: 2.1 });
    expect(draft.grenzwertUeberschritten).toBeNull();
  });
});
