/** Shared, typed fields of the GATE Vor-Ort investigation form. */
export interface InvestigationBasicsDraft {
  datum: string;
  zeit: string;
  bemerkung: string;
  lufttemperatur: number | null;
  luftfeuchte: number | null;
  abgeschlossen: boolean;
  bearbeiter: number[];
}
