/** Normalise the existing Django/Tauri grouped investigation response once for all Stelle screens.
 * A method is associated by its explicit untersuchungsstelle ID, never by its project alone.
 */
export type StelleMethodCategory = 'VorOrtUntersuchung' | 'Laboruntersuchung';
export interface StelleMethod {
  id: number;
  type: string;
  category: StelleMethodCategory;
  untersuchungsstelle: number | null;
  datum: string | null;
  abgeschlossen: boolean | null;
  sort_order: number;
}
export type GroupedStelleMethods = Partial<Record<StelleMethodCategory,
  Record<string, Array<Record<string, unknown>>>>>;

export function flattenStelleMethods(grouped: GroupedStelleMethods): StelleMethod[] {
  const result: StelleMethod[] = [];
  for (const category of ['VorOrtUntersuchung', 'Laboruntersuchung'] as const) {
    for (const [type, values] of Object.entries(grouped[category] ?? {})) {
      for (const row of values) {
        const id = Number(row.id);
        const stelleId = row.untersuchungsstelle === null || row.untersuchungsstelle === undefined
          ? null : Number(row.untersuchungsstelle);
        if (!Number.isSafeInteger(id) || id === 0) continue; // Negative IDs identify offline-created investigations.
        result.push({ id, type, category,
          untersuchungsstelle: Number.isSafeInteger(stelleId) && (stelleId as number) > 0 ? stelleId : null,
          datum: typeof row.datum === 'string' ? row.datum : null,
          abgeschlossen: row.abgeschlossen === true ? true : row.abgeschlossen === false ? false : null,
          sort_order: Number(row.sort_order) || 0,
        });
      }
    }
  }
  return result;
}

export function methodsForStelle(methods: StelleMethod[], stelleId: number): StelleMethod[] {
  return methods.filter(method => method.untersuchungsstelle === stelleId)
    .sort((a, b) => a.sort_order - b.sort_order || a.id - b.id);
}
