/** Tauri dashboard metadata and independent Django catalog calls. */
import { invoke, isTauri } from '@tauri-apps/api/core';
import { API_BASE_URL, apiConnector } from '@/utils/ApiConnector';
import type { IBauwerk } from '@/types/apiInterfaces';

export interface DashboardBauwerk {
  id: number;
  bezeichnung: string;
  bauwerksnummer: string;
  archived: boolean;
  projektIds: number[];
  syncStatus: 'synced' | 'unsynced' | 'conflict';
  serverUpdateAvailable: boolean;
  hasBaseline: boolean;
  lastOpenedAt: string | null;
}
export interface AvailableBauwerk {
  id: number;
  bezeichnung: string;
  bauwerksnummer: string;
  archived: boolean;
  projektIds: number[];
}
export interface DashboardSnapshot {
  bauwerke: DashboardBauwerk[];
  available: AvailableBauwerk[];
  lastServerCheck: string | null;
}

export class ServerHttpError extends Error {
  constructor(public readonly statusCode: number) {
    super(`Django responded with HTTP ${statusCode}`);
  }
}

export class ServerTimeoutError extends Error {
  constructor(timeoutMs: number) {
    super(`Django did not respond within ${timeoutMs} ms`);
    this.name = 'ServerTimeoutError';
  }
}

export async function getDashboardSnapshot(): Promise<DashboardSnapshot> {
  if (isTauri()) return invoke<DashboardSnapshot>('dashboard_snapshot');
  // Browser preview has no local SQLite. Statuses cannot be verified here.
  const records = await apiConnector.getBauwerke();
  return {
    bauwerke: records.map((record: IBauwerk) => ({
      id: record.id, bezeichnung: record.bezeichnung, bauwerksnummer: record.bauwerksnummer,
      archived: record.archived, projektIds: record.projekt,
      syncStatus: 'unsynced' as const, serverUpdateAvailable: false,
      hasBaseline: false, lastOpenedAt: null,
    })),
    available: [], lastServerCheck: null,
  };
}

export async function recordDownloadedBauwerke(records: IBauwerk[]): Promise<void> {
  if (isTauri() && records.length) await invoke('dashboard_record_baselines', { bauwerke: records });
}

const BAUWERK_URL = `${API_BASE_URL}/assetverwaltung/bauwerk/`;

/**
 * A deadline covers the entire operation, including reading JSON, not only the
 * time until HTTP response headers arrive. Abort alone is not always enough:
 * Promise.race also settles the UI if a network stack ignores cancellation.
 */
async function withServerTimeout<T>(
  timeoutMs: number,
  operation: (signal: AbortSignal) => Promise<T>,
): Promise<T> {
  const controller = new AbortController();
  let timer: ReturnType<typeof setTimeout> | undefined;
  try {
    return await Promise.race([
      operation(controller.signal),
      new Promise<never>((_resolve, reject) => {
        timer = setTimeout(() => {
          controller.abort();
          reject(new ServerTimeoutError(timeoutMs));
        }, timeoutMs);
      }),
    ]);
  } finally {
    if (timer !== undefined) clearTimeout(timer);
  }
}

async function requestServer(method: 'HEAD' | 'GET', signal: AbortSignal): Promise<Response> {
  return fetch(BAUWERK_URL, {
    method,
    headers: await apiConnector.makeHeaders(),
    cache: 'no-store',
    signal,
  });
}

export async function fetchServerBauwerke(): Promise<IBauwerk[]> {
  const payload: unknown = await withServerTimeout(8000, async (signal) => {
    const response = await requestServer('GET', signal);
    if (!response.ok) throw new ServerHttpError(response.status);
    return response.json();
  });
  const records = Array.isArray(payload) ? payload :
    (payload && typeof payload === 'object' && 'results' in payload && Array.isArray(payload.results) ? payload.results : null);
  if (!records || !records.every((r: unknown) => !!r && typeof r === 'object' &&
      typeof (r as IBauwerk).id === 'number' && typeof (r as IBauwerk).bezeichnung === 'string')) {
    throw new Error('Unexpected server Bauwerk catalog response');
  }
  return records as IBauwerk[];
}
export async function recordServerCatalog(records: IBauwerk[]): Promise<void> {
  if (isTauri()) await invoke('dashboard_record_server_check', { bauwerke: records });
}
export async function recordBauwerkOpened(id: number): Promise<void> {
  if (isTauri()) await invoke('dashboard_record_opened', { id });
}

/** Lightweight connectivity probe: never writes to SQLite or imports data. */
export async function probeDjangoServer(): Promise<void> {
  await withServerTimeout(5000, async (signal) => {
    let response = await requestServer('HEAD', signal);
    if (response.status === 405 || response.status === 501) {
      response = await requestServer('GET', signal);
      // A fallback GET is only used for reachability; no catalog parsing here.
      void response.body?.cancel().catch(() => undefined);
    }
    if (!response.ok) throw new ServerHttpError(response.status);
  });
}
