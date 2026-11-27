/** Load Bearbeiter reference data without bypassing the Keycloak fetch wrapper.
 * In Tauri, show cached Personen offline; online, reconcile the display options
 * against the server and cache those records in SQLite for later use.
 */
import type { IPerson } from '@/types/apiInterfaces';
import { API_BASE_URL } from '@/config/apiBase';
import { isServerOnline } from '@/services/connectivity';
import { listLocal, localMode, mirrorLocal } from '@/repositories/localDomainRepository';

export interface PeopleReferenceResult {
  people: IPerson[];
  serverAvailable: boolean;
  /** When set, do not allow editing server-only Bearbeiter associations. */
  warning: string;
}

function asPeople(value: unknown): IPerson[] {
  const records = Array.isArray(value) ? value
    : value && typeof value === 'object' && Array.isArray((value as {results?: unknown}).results)
      ? (value as {results: IPerson[]}).results : null;
  if (!records) throw new Error('Ungültige Personenliste vom Server.');
  return records.filter((row): row is IPerson => Boolean(row &&
    Number.isSafeInteger(Number((row as IPerson).id)) && Number((row as IPerson).id) > 0));
}

export async function loadPersonReferences(): Promise<PeopleReferenceResult> {
  let cached: IPerson[] = [];
  if (localMode()) {
    try { cached = await listLocal<IPerson>('Person'); }
    catch { /* The online fetch below can recover a missing local cache. */ }
  }
  const online = !localMode() || await isServerOnline(true);
  if (!online) {
    return {
      people: cached, serverAvailable: false,
      warning: cached.length
        ? 'Offline: Die gespeicherten Bearbeiter sind sichtbar. Änderungen an Zuordnungen erfordern eine Serververbindung.'
        : 'Keine Bearbeiter offline verfügbar. Bitte bei Serververbindung die Personenliste aktualisieren.',
    };
  }
  try {
    // window.fetch is augmented by installAuthFetch() in the authentication app.
    const response = await window.fetch(`${API_BASE_URL}/assetverwaltung/person/`);
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    const people = asPeople(await response.json());
    let cacheWarning = '';
    if (localMode() && people.length) {
      try {
        // Person may reference Firma, so import firms first to preserve SQLite FKs.
        const firmsResponse = await window.fetch(`${API_BASE_URL}/assetverwaltung/firma/`);
        if (!firmsResponse.ok) throw new Error(`Firma HTTP ${firmsResponse.status}`);
        const data = await firmsResponse.json() as unknown;
        const firms = Array.isArray(data) ? data
          : data && typeof data === 'object' && Array.isArray((data as { results?: unknown }).results)
            ? (data as { results: unknown[] }).results : null;
        if (firms === null) throw new Error('Ungültige Firmenliste');
        if (firms.length) await mirrorLocal('Firma', firms);
        await mirrorLocal('Person', people);
      } catch {
        cacheWarning = 'Bearbeiter sind online verfügbar, konnten jedoch nicht vollständig für Offline-Arbeit gespeichert werden. Bitte Bauwerksdaten erneut herunterladen.';
      }
    }
    return {
      people, serverAvailable: true,
      warning: people.length
        ? cacheWarning
        : 'Auf dem Django-Server sind noch keine Personen angelegt. Bitte zuerst Personen in der Assetverwaltung anlegen.',
    };
  } catch (error) {
    const detail = error instanceof Error ? error.message : String(error);
    return {
      people: cached, serverAvailable: false,
      warning: `Personenliste vom Server konnte nicht geladen werden (${detail}). Bitte Anmeldung und Django-Verbindung prüfen und erneut versuchen.`,
    };
  }
}
