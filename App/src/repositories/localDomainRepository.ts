/** Tauri-only, strongly bounded IPC API for the offline SQLite domain repository. */
import { invoke, isTauri } from '@tauri-apps/api/core';

export type LocalRecord = Record<string, any>;

export function localMode(): boolean {
  // On web the existing Django-backed workflows remain available.
  return isTauri();
}

function ensureNative() {
  if (!localMode()) throw new Error('Local SQLite requires the Tauri application');
}
export async function listLocal<T = LocalRecord>(resource: string, filters?: Record<string, string | number>): Promise<T[]> {
  ensureNative();
  return invoke<T[]>('local_list', { resource, filters: filters ?? null });
}
export async function getLocal<T = LocalRecord>(resource: string, id: number | string): Promise<T> {
  ensureNative();
  return invoke<T>('local_get', { resource, id: Number(id) });
}
export async function createLocal<T = LocalRecord>(resource: string, data: object): Promise<T> {
  ensureNative();
  return invoke<T>('local_create', { resource, data });
}
export async function updateLocal<T = LocalRecord>(resource: string, id: number, data: object): Promise<T> {
  ensureNative();
  return invoke<T>('local_update', { resource, id, data });
}
export async function deleteLocal(resource: string, id: number): Promise<void> {
  ensureNative();
  await invoke('local_delete', { resource, id });
}
export async function importLocal(resource: string, records: LocalRecord[]): Promise<number> {
  ensureNative();
  return invoke<number>('local_import', { resource, records });
}
export async function importLocalLinks(resource: string, records: LocalRecord[]): Promise<void> {
  ensureNative();
  await invoke('local_import_links', { resource, records });
}
async function fileToBase64(file: Blob): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => {
      const result = reader.result;
      if (typeof result !== 'string') return reject(new Error('Could not read media'));
      resolve(result.substring(result.indexOf(',') + 1));
    };
    reader.onerror = () => reject(reader.error);
    reader.readAsDataURL(file);
  });
}
export async function saveLocalMedia(metadata: LocalRecord, file: Blob): Promise<LocalRecord> {
  ensureNative();
  return invoke<LocalRecord>('local_save_media', {
    metadata,
    contentBase64: await fileToBase64(file),
    mime: file.type || 'application/octet-stream',
  });
}
export async function attachLocalMedia(id: number, file: Blob): Promise<void> {
  ensureNative();
  await invoke('local_attach_media', {
    id,
    contentBase64: await fileToBase64(file),
    mime: file.type || 'application/octet-stream',
  });
}
export async function localMediaBlobUrl(id: number): Promise<string> {
  ensureNative();
  const content = await invoke<string>('local_read_media', { id });
  const binary = atob(content);
  const bytes = Uint8Array.from(binary, character => character.charCodeAt(0));
  const meta = await getLocal('Media', id);
  const ext = String(meta.datei ?? '').split('.').pop()?.toLowerCase() ?? '';
  const mime = ({jpg: 'image/jpeg', jpeg: 'image/jpeg', png: 'image/png', webp: 'image/webp', pdf: 'application/pdf', mp4: 'video/mp4'} as Record<string, string>)[ext] ?? 'application/octet-stream';
  return URL.createObjectURL(new Blob([bytes], { type: mime }));
}
