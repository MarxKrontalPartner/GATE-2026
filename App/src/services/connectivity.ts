/**
 * Is the Django server reachable? Decides whether the desktop app works
 * online (server + local copy) or offline (local database only).
 *
 * The result is cached for a short time so not every API call probes.
 * A failed request marks the server as unreachable immediately.
 */
import { API_BASE_URL } from '@/config/apiBase';

const CACHE_MS = 15_000;
const PROBE_TIMEOUT_MS = 4_000;

let serverOnline: boolean | null = null; // null = not checked yet
let checkedAt = 0;
let probing: Promise<boolean> | null = null;
const reachableListeners: Array<() => void> = [];

function setOnline(value: boolean): boolean {
  const cameBack = value && serverOnline !== true;
  serverOnline = value;
  checkedAt = Date.now();
  if (cameBack) reachableListeners.forEach((listener) => listener());
  return value;
}

async function probe(): Promise<boolean> {
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), PROBE_TIMEOUT_MS);
  try {
    // Any HTTP answer means the server is reachable; only network errors do not.
    await window.fetch(`${API_BASE_URL}/`, { method: 'GET', signal: controller.signal });
    return true;
  } catch {
    return false;
  } finally {
    clearTimeout(timer);
  }
}

export async function isServerOnline(force = false): Promise<boolean> {
  if (typeof navigator !== 'undefined' && !navigator.onLine) return setOnline(false);
  if (!force && serverOnline !== null && Date.now() - checkedAt < CACHE_MS) return serverOnline;
  probing ??= probe().then((reachable) => {
    probing = null;
    return setOnline(reachable);
  });
  return probing;
}

/** Called when a request to the server failed with a network error. */
export function markServerUnreachable(): void {
  setOnline(false);
}

/** Runs `listener` whenever the server becomes reachable (also the first time). */
export function onServerReachable(listener: () => void): void {
  reachableListeners.push(listener);
}

/** Re-checks the server regularly, so a reconnect is noticed without user action. */
export function watchServer(intervalMs = 30_000): void {
  void isServerOnline(true);
  setInterval(() => void isServerOnline(true), intervalMs);
  window.addEventListener('online', () => void isServerOnline(true));
  window.addEventListener('offline', () => markServerUnreachable());
}

/** Resources that can be changed without a server connection. */
const OFFLINE_EDITABLE = ['rueckprallhammer', 'kernbohrung', 'rissbreitenmessung', 'chloridanalyse', 'druckfestigkeitspruefung'];

export function isOfflineEditable(resource: string): boolean {
  return OFFLINE_EDITABLE.includes(resource.trim().toLowerCase());
}

export class OfflineWriteError extends Error {
  constructor(resource: string, operation: string) {
    super(
      `Keine Verbindung zum Server: ${resource} kann offline nicht ` +
      `${operation === 'delete' ? 'gelöscht' : operation === 'create' ? 'angelegt' : 'geändert'} werden. ` +
      'Offline können nur Untersuchungen (Rückprallhammer, Kernbohrung, Rissbreitenmessung, ' +
      'Chloridanalyse, Druckfestigkeitsprüfung) angelegt und bearbeitet werden.',
    );
    this.name = 'OfflineWriteError';
  }
}
