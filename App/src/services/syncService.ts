/**
 * Sends Rückprallhammer changes made offline to the server.
 *
 * 1. `sync_take_pending` (Rust) builds `[{id, patch}]` from the client_sync_changes table.
 * 2. PATCH it to the Django sync endpoint (200, 409 and 404 are expected answers).
 * 3. `sync_apply` (Rust) stores the answer: saved rows, conflicts, failures.
 *    Changes wait in the client_sync_changes table until they are saved on the server.
 */
import { invoke, isTauri } from '@tauri-apps/api/core';
import { API_BASE_URL } from '@/config/apiBase';
import { markServerUnreachable } from '@/services/connectivity';

export const SYNCED_RESOURCES = [
  'Rueckprallhammer', 'Kernbohrung', 'Rissbreitenmessung', 'Chloridanalyse', 'Druckfestigkeitspruefung',
];

export interface SyncResult {
  resource: string;
  /** Records created offline that the server created (status "new" → "synced"). */
  created: number;
  sent: number;
  saved: number;
  conflicts: number;
  failed: boolean;
}

/**
 * Sends the records created offline as one bulk list (POST, without local ids).
 * The server answers with the created records in the same order; their ids
 * replace the temporary local ids. Returns the number created, or -1 on failure.
 */
async function syncCreates(resource: string, headers: Record<string, string>): Promise<number> {
  const { endpoint, localIds, records } = await invoke<{ endpoint: string; localIds: number[]; records: unknown[] }>(
    'sync_take_creates', { resource });
  if (!records.length) return 0;
  let response: Response;
  try {
    response = await window.fetch(`${API_BASE_URL}/${endpoint}`, {
      method: 'POST',
      headers: { ...headers, 'Content-Type': 'application/json' },
      body: JSON.stringify(records),
    });
  } catch {
    markServerUnreachable();
    await invoke('sync_apply_creates', { resource, localIds, status: 'FAILED', created: [], error: 'Keine Verbindung zum Server' });
    return -1;
  }
  if (response.status !== 201) {
    const error = `HTTP ${response.status}: ${(await response.text()).slice(0, 500)}`;
    await invoke('sync_apply_creates', { resource, localIds, status: 'FAILED', created: [], error });
    return -1;
  }
  const created = await response.json();
  await invoke('sync_apply_creates', { resource, localIds, status: 'OK', created, error: null });
  return localIds.length;
}

export async function syncResource(resource: string, headers: Record<string, string> = {}): Promise<SyncResult> {
  const result: SyncResult = { resource, created: 0, sent: 0, saved: 0, conflicts: 0, failed: false };
  if (!isTauri()) return result;
  // 1. Records created offline, so later changes refer to their server ids.
  const created = await syncCreates(resource, headers);
  if (created < 0) result.failed = true; else result.created = created;
  // 2. Changes of existing records.
  const { endpoint, body } = await invoke<{ endpoint: string; body: unknown[] }>('sync_take_pending', { resource });
  result.sent = body.length;
  if (!body.length) return result;

  let response: Response;
  try {
    response = await window.fetch(`${API_BASE_URL}/${endpoint}`, {
      method: 'PATCH',
      headers: { ...headers, 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
  } catch {
    markServerUnreachable();
    await invoke('sync_apply', { resource, status: 'FAILED', body: {}, sent: body });
    result.failed = true;
    return result;
  }
  if (![200, 404, 409].includes(response.status)) {
    await invoke('sync_apply', { resource, status: 'FAILED', body: {}, sent: body });
    result.failed = true;
    return result;
  }
  const answer = await response.json();
  await invoke('sync_apply', { resource, status: answer.status, body: answer, sent: body });
  result.saved = Array.isArray(answer.saved) ? answer.saved.length : 0;
  result.conflicts = Array.isArray(answer.conflicts) ? answer.conflicts.length : 0;
  result.failed = result.failed || answer.status === 'NOT_FOUND';
  return result;
}

let running: Promise<SyncResult[]> | null = null;

/** Sends all pending changes. Concurrent calls share one run. */
export function syncAll(headers: Record<string, string> = {}): Promise<SyncResult[]> {
  running ??= (async () => {
    try {
      const results: SyncResult[] = [];
      for (const resource of SYNCED_RESOURCES) results.push(await syncResource(resource, headers));
      return results;
    } finally {
      running = null;
    }
  })();
  return running;
}

/** Stored conflicts (read only; nothing changes them yet). */
export const listConflicts = () => invoke<unknown[]>('sync_list_conflicts');
