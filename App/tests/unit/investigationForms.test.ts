import { describe, expect, it } from 'vitest';
import { buildInvestigationCreatePayload } from '@/utils/investigationCreatePayload';
import { kernbohrungPatch, toKernbohrungDraft, validateKernbohrungDraft } from '@/types/kernbohrung';

describe('Investigation creation (offline-safe)', () => {
  it('keeps the selected Bauwerk, Projekt and Stelle on a Kernbohrung', () => {
    expect(buildInvestigationCreatePayload({
      bauwerkId: 12, projektId: 25, stelleId: 39, art: 'VorOrtUntersuchung', method: 'kernbohrung',
      datum: '2026-11-16', zeit: '15:30', bemerkung: 'Neuer Bohrpunkt',
    })).toMatchObject({
      bauwerk: 12, projekt: 25, untersuchungsstelle: 39,
      datum: '2026-11-16', zeit: '15:30:00', abgeschlossen: false,
      kuehlung: '', verschlussmaterial: '',
    });
  });
  it('rejects a procedure assigned to the wrong category', () => {
    expect(() => buildInvestigationCreatePayload({
      bauwerkId: 1, projektId: 2, stelleId: 3,
      art: 'Laboruntersuchung', method: 'kernbohrung', datum: '2026-11-16',
    })).toThrow(/kombination/i);
  });
  it('rejects missing relationship ids', () => {
    expect(() => buildInvestigationCreatePayload({
      bauwerkId: 1, projektId: 0, stelleId: 3,
      art: 'VorOrtUntersuchung', method: 'kernbohrung', datum: '2026-11-16',
    })).toThrow();
  });
});

describe('Kernbohrung draft', () => {
  it('keeps existing layers and measured values, including zero', () => {
    const draft = toKernbohrungDraft({
      datum: '2026-11-16', durchmesser: 0, kernVollstaendig: true,
      schichtaufbau: [{ bezeichnung: 'Beton', tiefe_von: 0, tiefe_bis: 120, beschreibung: 'intakt' }],
    });
    expect(draft.durchmesser).toBe(0);
    expect(draft.kernVollstaendig).toBe(true);
    expect(draft.schichtaufbau[0]?.tiefe_bis).toBe(120);
    expect(validateKernbohrungDraft(draft)).toBeNull();
    expect(kernbohrungPatch(draft)).toMatchObject({ durchmesser: 0, kernVollstaendig: true });
  });
  it('checks humidity and impossible layer depths before saving', () => {
    const draft = toKernbohrungDraft({ datum: '2026-11-16', luftfeuchte: 108 });
    expect(validateKernbohrungDraft(draft)).toMatch(/100/);
    draft.luftfeuchte = 70;
    draft.schichtaufbau.push({ bezeichnung: 'Mörtel', beschreibung: '', tiefe_von: 20, tiefe_bis: 10 });
    expect(validateKernbohrungDraft(draft)).toMatch(/Schicht/);
  });
});

describe('Kernbohrung persistence read-back', () => {
  it('keeps temperature, relative humidity and closure material after a round trip', async () => {
    const { kernbohrungPersistenceDifferences } = await import('@/types/kernbohrung');
    const original = toKernbohrungDraft({
      datum: '2026-11-16', lufttemperatur: 22.5, luftfeuchte: 67,
      verschlussmaterial: 'Schnellzement', durchmesser: 0,
    });
    const payload = kernbohrungPatch(original);
    const reloaded = { ...payload, id: 8, bohrkern: null };
    expect(kernbohrungPersistenceDifferences(original, reloaded)).toEqual([]);
    expect(toKernbohrungDraft(reloaded).verschlussmaterial).toBe('Schnellzement');
  });
  it('rejects a falsely successful save that dropped measurement fields', async () => {
    const { kernbohrungPersistenceDifferences } = await import('@/types/kernbohrung');
    const original = toKernbohrungDraft({
      datum: '2026-11-16', lufttemperatur: 18.6, luftfeuchte: 42,
      verschlussmaterial: 'Epoxidharz',
    });
    const forgotten = { ...kernbohrungPatch(original), lufttemperatur: null,
      luftfeuchte: null, verschlussmaterial: '' };
    expect(kernbohrungPersistenceDifferences(original, forgotten)).toEqual([
      'Lufttemperatur', 'Relative Luftfeuchtigkeit', 'Verschlussmaterial',
    ]);
  });
});
