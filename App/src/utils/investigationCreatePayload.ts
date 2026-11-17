import { getMethodMetadata, type UntersuchungsverfahrenArt } from './UntersuchungsverfahrenRegistry';

export interface NewInvestigationContext {
  bauwerkId: number;
  projektId: number;
  stelleId: number;
  art: UntersuchungsverfahrenArt;
  method: string;
  datum: string;
  zeit?: string;
  bemerkung?: string;
}

/** Shared form → API mapping. Server and local SeaORM commands accept the same IDs. */
export function buildInvestigationCreatePayload(input: NewInvestigationContext): Record<string, unknown> {
  if (![input.bauwerkId, input.projektId, input.stelleId]
    .every(id => Number.isSafeInteger(id) && id > 0)) {
    throw new Error('Bauwerk, Projekt und Untersuchungsstelle müssen ausgewählt sein.');
  }
  const method = getMethodMetadata(input.method);
  if (!method || method.art !== input.art) throw new Error('Ungültige Kombination aus Art und Untersuchungsverfahren.');
  if (!/^\d{4}-\d{2}-\d{2}$/.test(input.datum)) throw new Error('Ungültiges Untersuchungsdatum.');
  return {
    bauwerk: input.bauwerkId,
    projekt: input.projektId,
    untersuchungsstelle: input.stelleId,
    datum: input.datum,
    // SeaORM's chrono::NaiveTime expects HH:MM:SS, while HTML time inputs use HH:MM.
    zeit: input.zeit ? (input.zeit.length === 5 ? `${input.zeit}:00` : input.zeit) : null,
    bemerkung: input.bemerkung ?? '',
    abgeschlossen: false,
    ...(method.slug === 'kernbohrung' ? { kuehlung: '', verschlussmaterial: '' } : {}),
  };
}
