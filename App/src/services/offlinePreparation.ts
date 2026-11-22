/**
 * Explicit initial data download, not synchronization.
 * One-way copy from the existing Django API to local SQLite. The download does
 * not upload edits, resolve conflicts, or overwrite existing local records.
 * It requires Django to be reachable, but the resulting data is read offline.
 */
import { API_BASE_URL } from '@/utils/ApiConnector';
import { recordDownloadedBauwerke } from '@/repositories/dashboardRepository';
import type { IBauwerk } from '@/types/apiInterfaces';
import { attachLocalMedia, importLocal, importLocalLinks, mirrorLocal, type KeptRecord, type LocalRecord } from '@/repositories/localDomainRepository';

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
  // Only record server baselines after the full import succeeds. The Rust
  // repository checks the local copy matches before declaring it downloaded.
  const downloaded = stages.find(stage => stage.resource === 'Bauwerk')?.records ?? [];
  await recordDownloadedBauwerke(downloaded as unknown as IBauwerk[]);
  return { records, files };
}

/** Record fields that point at media rows. */
const MEDIA_FIELDS = ['bilder', 'messprotokolle', 'pruefberichte', 'fotos', 'foto', 'media', 'titelBild'];

function ids(values: unknown): number[] {
  const list = Array.isArray(values) ? values : [values];
  return list.map(Number).filter(id => Number.isSafeInteger(id) && id > 0);
}

/**
 * Downloads one Bauwerk and everything recorded for it on the server into the
 * app database: its Projekte, Untersuchungsstellen, all diagnostic records,
 * Bohrkerne, Proben and the media files they use. Reference data (Firmen,
 * Personen, Material) is downloaded completely, because records link to it.
 *
 * Existing local records are updated with the server state, except records with
 * unsent offline changes or an open conflict: those stay as they are and are
 * returned in `kept`. Requires the server.
 */
export async function downloadBauwerk(bauwerkId: number):
  Promise<{ records: number; files: number; kept: Array<KeptRecord & { resource: string }> }> {
  // 1. Read everything from the server first, so a failure leaves the app database untouched.
  const bauwerk = await readJson(`assetverwaltung/bauwerk/${bauwerkId}/`) as LocalRecord;
  const projektIds = ids(bauwerk.projekt);
  const projekte = asRecords(await readJson('assetverwaltung/projekt/'), 'Projekt')
    .filter(p => projektIds.includes(Number(p.id)));
  const firmen = asRecords(await readJson('assetverwaltung/firma/'), 'Firma');
  const personen = asRecords(await readJson('assetverwaltung/person/'), 'Person');
  const material = asRecords(await readJson('bwd/Material/'), 'Material');
  const stellen = asRecords(await readJson(`bwd/Untersuchungsstelle/?bauwerk_id=${bauwerkId}`), 'Untersuchungsstelle');

  const diagnostics: PlannedImport[] = [];
  const grouped = await readJson(`bwd/Untersuchungsverfahren/?art=all&bauwerk=${bauwerkId}`);
  for (const category of ['VorOrtUntersuchung', 'Laboruntersuchung']) {
    const types = grouped?.[category];
    if (!types || typeof types !== 'object') throw new Error(`Missing ${category} in diagnostic export`);
    for (const [resource, records] of Object.entries(types)) {
      diagnostics.push({ resource, records: asRecords(records, resource) });
    }
  }
  const diagnosticRows = diagnostics.flatMap(stage => stage.records);

  // Proben of this Bauwerk, plus Proben a diagnostic record refers to.
  const referencedProben = new Set(diagnosticRows.flatMap(row => ids(row.probe)));
  const proben = asRecords(await readJson('bwd/Probe/'), 'Probe')
    .filter(p => Number(p.bauwerk_id) === bauwerkId || referencedProben.has(Number(p.id)));
  // Bohrkerne have no Bauwerk field; they are found through Kernbohrung and Probe.
  const bohrkernIds = new Set([...diagnosticRows, ...proben].flatMap(row => ids(row.bohrkern)));
  const bohrkerne = bohrkernIds.size
    ? asRecords(await readJson('bwd/Bohrkern/'), 'Bohrkern').filter(b => bohrkernIds.has(Number(b.id)))
    : [];

  const mediaIds = [...new Set([...stellen, ...bohrkerne, ...proben, ...diagnosticRows]
    .flatMap(row => MEDIA_FIELDS.flatMap(field => ids(row[field]))))];
  let media: LocalRecord[] = [];
  if (mediaIds.length) {
    const response = await fetch(`${API_BASE_URL}/bwd/Media/batch/`, {
      method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ ids: mediaIds }),
    });
    if (!response.ok) throw new Error(`Download: media list returned HTTP ${response.status}`);
    media = asRecords(await response.json(), 'Media');
  }
  const files: Array<{ id: number; blob: Blob }> = [];
  for (const item of media) {
    if (!item.datei || !item.id) continue;
    const response = await fetch(`${API_BASE_URL}/bwd/Media/${item.id}/content/`);
    if (!response.ok) throw new Error(`Cannot download media ${item.id}: HTTP ${response.status}`);
    files.push({ id: Number(item.id), blob: await response.blob() });
  }

  // 2. Write into the app database, referenced records before the records that point at them.
  let records = 0;
  const kept: Array<KeptRecord & { resource: string }> = [];
  const save = async (resource: string, rows: LocalRecord[]) => {
    if (!rows.length) return;
    const result = await mirrorLocal(resource, rows);
    records += result.written;
    kept.push(...result.skipped.map((row) => ({ ...row, resource })));
  };
  await save('Firma', firmen);
  await save('Person', personen);
  await save('Projekt', projekte);
  await save('Bauwerk', [bauwerk]);
  await save('Material', material);
  if (media.length) {
    records += await importLocal('Media', media);
    for (const file of files) await attachLocalMedia(file.id, file.blob);
  }
  await save('Untersuchungsstelle', stellen);
  await save('Bohrkern', bohrkerne);
  await save('Probe', proben);
  for (const stage of diagnostics) await save(stage.resource, stage.records);

  // 3. Mark the Bauwerk as downloaded for the dashboard (status "Synchronisiert").
  await recordDownloadedBauwerke([bauwerk as unknown as IBauwerk]);
  return { records, files: files.length, kept };
}
