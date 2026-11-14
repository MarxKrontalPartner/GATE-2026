/**
 * Explicit initial data download, not synchronization.
 * One-way copy from the existing Django API to local SQLite. The download does
 * not upload edits, resolve conflicts, or overwrite existing local records.
 * It requires Django to be reachable, but the resulting data is read offline.
 */
import { API_BASE_URL } from '@/utils/ApiConnector';
import { attachLocalMedia, importLocal, importLocalLinks, type LocalRecord } from '@/repositories/localDomainRepository';

type PlannedImport = { resource: string; records: LocalRecord[] };

async function readJson(path: string): Promise<any> {
  const url = `${API_BASE_URL}/${path}`;
  const response = await fetch(url);
  if (!response.ok) throw new Error(`Initial download: ${path} returned HTTP ${response.status}`);
  return response.json();
}
function asRecords(value: any, name: string): LocalRecord[] {
  if (Array.isArray(value)) return value;
  if (Array.isArray(value?.results)) return value.results;
  throw new Error(`${name}: expected a record list, got a different API response`);
}

export async function prepareOfflineData(): Promise<{ records: number; files: number }> {
  const stages: PlannedImport[] = [];
  // Load all referenced entities before diagnostic rows and join links.
  for (const resource of ['Firma', 'Person', 'Projekt', 'Bauwerk']) {
    stages.push({ resource, records: asRecords(await readJson(`assetverwaltung/${resource.toLowerCase()}/`), resource) });
  }
  for (const resource of ['Material', 'Bohrkern', 'Untersuchungsstelle', 'Probe', 'Media']) {
    stages.push({ resource, records: asRecords(await readJson(`bwd/${resource}/`), resource) });
  }
  // The existing endpoint includes all five supported diagnostic types.
  const grouped = await readJson('bwd/Untersuchungsverfahren/?art=all');
  for (const category of ['VorOrtUntersuchung', 'Laboruntersuchung']) {
    const types = grouped?.[category];
    if (!types || typeof types !== 'object') throw new Error(`Missing ${category} in diagnostic export`);
    for (const [resource, records] of Object.entries(types)) {
      stages.push({ resource, records: asRecords(records, resource) });
    }
  }
  let records = 0;
  // Persist base entities (the Rust importer keeps existing local records).
  for (const stage of stages) {
    records += await importLocal(stage.resource, stage.records);
  }
  // Install M:N relations only after every parent and target table has data.
  for (const stage of stages) {
    await importLocalLinks(stage.resource, stage.records);
  }
  // Download actual file contents. Metadata alone is insufficient for offline media.
  let files = 0;
  const media = stages.find(stage => stage.resource === 'Media')?.records ?? [];
  for (const item of media) {
    if (!item.datei || !item.id) continue;
    const response = await fetch(`${API_BASE_URL}/bwd/Media/${item.id}/content/`);
    if (!response.ok) throw new Error(`Cannot download media ${item.id}: HTTP ${response.status}`);
    await attachLocalMedia(Number(item.id), await response.blob());
    files++;
  }
  return { records, files };
}
