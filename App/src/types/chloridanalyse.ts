import type { InvestigationBasicsDraft } from './investigationBasics';

export type ChloridAnalyseverfahren = '' | 'titration' | 'photometrie' | 'schnelltest';

/** Editing state only; the existing Tauri resource + SeaORM entity persist these fields. */
export interface ChloridanalyseDraft extends InvestigationBasicsDraft {
  probe: number | null;
  pruefstelle: string;
  analyseverfahren: ChloridAnalyseverfahren;
  chloridgehaltZement: number | null;
  zementgehaltAnnahme: number | null;
  grenzwertUeberschritten: boolean | null;
  pruefberichte: number[];
}

const ALLOWED_METHODS = ['titration', 'photometrie', 'schnelltest'];
function numericOrNull(value: unknown): number | null {
  if (value === null || value === undefined || value === '') return null;
  const converted = Number(value);
  return Number.isFinite(converted) ? converted : null;
}
function positiveIdOrNull(value: unknown): number | null {
  const number = numericOrNull(value);
  return number !== null && Number.isSafeInteger(number) && number > 0 ? number : null;
}
function idList(value: unknown): number[] {
  if (!Array.isArray(value)) return [];
  return [...new Set(value.map(positiveIdOrNull).filter((id): id is number => id !== null))];
}
function nullableBool(value: unknown): boolean | null {
  return value === true || value === 1 ? true : value === false || value === 0 ? false : null;
}

export function toChloridanalyseDraft(record: Record<string, unknown>): ChloridanalyseDraft {
  const method = String(record.analyseverfahren ?? '');
  return {
    datum: String(record.datum ?? ''),
    zeit: String(record.zeit ?? '').slice(0, 5),
    bemerkung: String(record.bemerkung ?? ''),
    lufttemperatur: numericOrNull(record.lufttemperatur),
    luftfeuchte: numericOrNull(record.luftfeuchte),
    abgeschlossen: record.abgeschlossen === true,
    bearbeiter: idList(record.bearbeiter),
    probe: positiveIdOrNull(record.probe),
    pruefstelle: String(record.pruefstelle ?? ''),
    analyseverfahren: (ALLOWED_METHODS.includes(method) ? method : '') as ChloridAnalyseverfahren,
    chloridgehaltZement: numericOrNull(record.chloridgehaltZement),
    zementgehaltAnnahme: numericOrNull(record.zementgehaltAnnahme),
    grenzwertUeberschritten: nullableBool(record.grenzwertUeberschritten),
    pruefberichte: idList(record.pruefberichte),
  };
}

export function validateChloridanalyseDraft(draft: ChloridanalyseDraft): string | null {
  if (!/^\d{4}-\d{2}-\d{2}$/.test(draft.datum) || Number.isNaN(Date.parse(`${draft.datum}T12:00:00`))) {
    return 'Bitte ein gültiges Untersuchungsdatum eingeben.';
  }
  if (draft.zeit && !/^([01]\d|2[0-3]):[0-5]\d$/.test(draft.zeit)) {
    return 'Bitte eine gültige Uhrzeit eingeben.';
  }
  if (draft.pruefstelle.trim().length > 100) return 'Die Prüfstelle darf höchstens 100 Zeichen enthalten.';
  if (draft.analyseverfahren && !ALLOWED_METHODS.includes(draft.analyseverfahren)) {
    return 'Bitte ein gültiges Analyseverfahren wählen.';
  }
  for (const value of [draft.chloridgehaltZement, draft.zementgehaltAnnahme]) {
    if (value !== null && (!Number.isFinite(value) || value < 0)) {
      return 'Chlorid- und Zementgehalt müssen nicht negative Zahlen sein.';
    }
  }
  if (draft.luftfeuchte !== null && (!Number.isFinite(draft.luftfeuchte) || draft.luftfeuchte < 0 || draft.luftfeuchte > 100)) {
    return 'Die Luftfeuchtigkeit muss zwischen 0 und 100 % liegen.';
  }
  if (draft.lufttemperatur !== null && !Number.isFinite(draft.lufttemperatur)) {
    return 'Bitte eine gültige Lufttemperatur eingeben.';
  }
  if (draft.probe !== null && (!Number.isSafeInteger(draft.probe) || draft.probe < 1)) {
    return 'Bitte eine gültige Probe wählen.';
  }
  return null;
}

/** PATCH uses the original Django field names; Tauri maps them through SeaORM. */
export function chloridanalysePatch(draft: ChloridanalyseDraft): Record<string, unknown> {
  return {
    ...draft,
    pruefstelle: draft.pruefstelle.trim(),
    zeit: draft.zeit ? `${draft.zeit}:00` : null,
  };
}

/** Compare with a NEW database read, not the potentially stale Pinia state. */
export function chloridanalysePersistenceDifferences(
  expected: ChloridanalyseDraft, persisted: Record<string, unknown>,
): string[] {
  const saved = toChloridanalyseDraft(persisted);
  const differences: string[] = [];
  const fields: Array<[keyof ChloridanalyseDraft, string]> = [
    ['datum', 'Datum'], ['zeit', 'Uhrzeit'], ['bemerkung', 'Bemerkung'],
    ['lufttemperatur', 'Lufttemperatur'], ['luftfeuchte', 'Luftfeuchtigkeit'],
    ['abgeschlossen', 'Abgeschlossen'], ['probe', 'Probe'],
    ['pruefstelle', 'Prüfstelle'], ['analyseverfahren', 'Analyseverfahren'],
    ['chloridgehaltZement', 'Chloridgehalt'], ['zementgehaltAnnahme', 'Zementgehalt'],
    ['grenzwertUeberschritten', 'Grenzwert'],
  ];
  for (const [key, label] of fields) {
    if (expected[key] !== saved[key]) differences.push(label);
  }
  for (const [key, label] of [ ['bearbeiter', 'Bearbeiter'], ['pruefberichte', 'Prüfberichte'] ] as const) {
    if (JSON.stringify([...expected[key]].sort((a, b) => a - b)) !==
        JSON.stringify([...saved[key]].sort((a, b) => a - b))) differences.push(label);
  }
  return differences;
}
