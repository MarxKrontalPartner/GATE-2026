/** UI-only draft: persistence remains in the existing Tauri/SeaORM layer. */
export interface KernbohrungSchicht {
  id?: string;
  bezeichnung: string;
  beschreibung: string;
  tiefe_von: number | null;
  tiefe_bis: number | null;
}
export interface KernbohrungDraft {
  datum: string;
  zeit: string;
  bemerkung: string;
  lufttemperatur: number | null;
  luftfeuchte: number | null;
  abgeschlossen: boolean;
  bearbeiter: number[];
  durchmesser: number | null;
  bohrlaenge: number | null;
  neigungswinkel: number | null;
  kuehlung: string;
  verschlussmaterial: string;
  bewehrungAngeschnitten: boolean;
  kernVollstaendig: boolean;
  schichtaufbau: KernbohrungSchicht[];
}

function validNumber(value: unknown): number | null {
  if (value === '' || value === undefined || value === null) return null;
  const result = Number(value);
  return Number.isFinite(result) ? result : null;
}
export function toKernbohrungDraft(record: Record<string, unknown>): KernbohrungDraft {
  let layers: unknown = record.schichtaufbau;
  if (typeof layers === 'string') {
    try { layers = JSON.parse(layers); } catch { layers = []; }
  }
  return {
    datum: String(record.datum ?? ''),
    zeit: String(record.zeit ?? '').slice(0, 5),
    bemerkung: String(record.bemerkung ?? ''),
    lufttemperatur: validNumber(record.lufttemperatur),
    luftfeuchte: validNumber(record.luftfeuchte),
    abgeschlossen: record.abgeschlossen === true,
    bearbeiter: Array.isArray(record.bearbeiter) ? record.bearbeiter.map(Number).filter(Number.isSafeInteger) : [],
    durchmesser: validNumber(record.durchmesser),
    bohrlaenge: validNumber(record.bohrlaenge),
    neigungswinkel: validNumber(record.neigungswinkel),
    kuehlung: String(record.kuehlung ?? ''),
    verschlussmaterial: String(record.verschlussmaterial ?? ''),
    bewehrungAngeschnitten: record.bewehrungAngeschnitten === true,
    kernVollstaendig: record.kernVollstaendig === true,
    schichtaufbau: Array.isArray(layers) ? layers.map((layer, index) => {
      const data = layer && typeof layer === 'object' ? layer as Record<string, unknown> : {};
      return {
        id: String(data.id ?? `schicht-${index}`),
        bezeichnung: String(data.bezeichnung ?? ''),
        beschreibung: String(data.beschreibung ?? ''),
        tiefe_von: validNumber(data.tiefe_von),
        tiefe_bis: validNumber(data.tiefe_bis),
      };
    }) : [],
  };
}

export function validateKernbohrungDraft(draft: KernbohrungDraft): string | null {
  if (!/^\d{4}-\d{2}-\d{2}$/.test(draft.datum) || Number.isNaN(Date.parse(`${draft.datum}T12:00:00`))) {
    return 'Bitte ein gültiges Untersuchungsdatum eingeben.';
  }
  if (draft.zeit && !/^([01]\d|2[0-3]):[0-5]\d(:[0-5]\d)?$/.test(draft.zeit)) {
    return 'Bitte eine gültige Uhrzeit eingeben.';
  }
  if ([draft.durchmesser, draft.bohrlaenge].some(v => v !== null && (!Number.isFinite(v) || v < 0))) {
    return 'Durchmesser und Bohrlänge müssen gültige, nicht negative Werte sein.';
  }
  if (draft.luftfeuchte !== null && (!Number.isFinite(draft.luftfeuchte) || draft.luftfeuchte < 0 || draft.luftfeuchte > 100)) {
    return 'Die relative Luftfeuchtigkeit muss zwischen 0 und 100 % liegen.';
  }
  if ([draft.lufttemperatur, draft.neigungswinkel].some(v => v !== null && !Number.isFinite(v))) {
    return 'Bitte gültige Messwerte eingeben.';
  }
  for (const layer of draft.schichtaufbau) {
    if (layer.tiefe_von === null || layer.tiefe_bis === null ||
        !Number.isFinite(layer.tiefe_von) || !Number.isFinite(layer.tiefe_bis) ||
        layer.tiefe_von < 0 || layer.tiefe_bis < layer.tiefe_von) {
      return 'Bitte für jede Schicht eine gültige Start- und Endtiefe (mm) eingeben. Die Endtiefe muss mindestens so groß wie die Starttiefe sein.';
    }
  }
  return null;
}
export function kernbohrungPatch(draft: KernbohrungDraft): Record<string, unknown> {
  return {
    ...draft,
    zeit: draft.zeit ? `${draft.zeit}:00` : null,
    // Local JSON column. Remove UI-only IDs before persisting if absent in old records?
    schichtaufbau: draft.schichtaufbau.map(layer => ({ ...layer })),
  };
}

/** Check the actual ORM read-back before reporting a successful save. */
export function kernbohrungPersistenceDifferences(
  expected: KernbohrungDraft,
  savedRecord: Record<string, unknown>,
): string[] {
  const actual = toKernbohrungDraft(savedRecord);
  const numeric: Array<[keyof KernbohrungDraft, string]> = [
    ['lufttemperatur', 'Lufttemperatur'],
    ['luftfeuchte', 'Relative Luftfeuchtigkeit'],
    ['durchmesser', 'Durchmesser'],
    ['bohrlaenge', 'Bohrlänge'],
    ['neigungswinkel', 'Neigungswinkel'],
  ];
  const differences: string[] = [];
  for (const [key, label] of numeric) {
    if (expected[key] !== actual[key]) differences.push(label);
  }
  if (expected.verschlussmaterial !== actual.verschlussmaterial) differences.push('Verschlussmaterial');
  if (expected.kuehlung !== actual.kuehlung) differences.push('Kühlung');
  if (expected.bemerkung !== actual.bemerkung) differences.push('Bemerkung');
  if (expected.abgeschlossen !== actual.abgeschlossen) differences.push('Abgeschlossen');
  return differences;
}
