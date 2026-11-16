import { defineStore } from 'pinia';
import { ref } from 'vue';
import { isTauri } from '@tauri-apps/api/core';
import type { IBauwerk } from '@/types/apiInterfaces';
import {
  getDashboardSnapshot, fetchServerBauwerke, recordServerCatalog, recordBauwerkOpened,
  probeDjangoServer, ServerHttpError,
  type DashboardBauwerk, type AvailableBauwerk,
} from '@/repositories/dashboardRepository';

export type ServerState = 'unknown' | 'checking' | 'available' | 'unavailable' | 'unauthorized' | 'offline';

export const useDashboardStore = defineStore('dashboard', () => {
  const native = isTauri();
  const bauwerke = ref<DashboardBauwerk[]>([]);
  const available = ref<AvailableBauwerk[]>([]);
  const lastServerCheck = ref<string | null>(null);
  const loading = ref(false);
  const checking = ref(false);
  const probing = ref(false);
  const serverState = ref<ServerState>('unknown');
  const online = ref(typeof navigator !== 'undefined' ? navigator.onLine : true);
  const error = ref('');
  const notice = ref('');

  async function loadLocal(): Promise<void> {
    loading.value = true;
    error.value = '';
    try {
      const snapshot = await getDashboardSnapshot();
      bauwerke.value = snapshot.bauwerke;
      available.value = snapshot.available;
      lastServerCheck.value = snapshot.lastServerCheck;
    } catch (cause) {
      error.value = `Dashboard konnte nicht geladen werden: ${String(cause)}`;
    } finally { loading.value = false; }
  }

  function markServerError(cause: unknown): void {
    online.value = navigator.onLine;
    serverState.value = !online.value ? 'offline' :
      cause instanceof ServerHttpError && [401, 403].includes(cause.statusCode)
        ? 'unauthorized' : 'unavailable';
  }

  // Connectivity probing is independent of catalog checks and local SQLite work.
  // In particular, an in-progress "Updates prüfen" must not block this probe.
  async function checkServerAvailability(): Promise<void> {
    online.value = navigator.onLine;
    if (!online.value) { serverState.value = 'offline'; return; }
    if (probing.value) return;
    probing.value = true;
    // Keep the last confirmed status visible during periodic re-checks.
    if (serverState.value === 'unknown') serverState.value = 'checking';
    try {
      await probeDjangoServer();
      online.value = navigator.onLine;
      serverState.value = online.value ? 'available' : 'offline';
      if (notice.value === 'Serverprüfung fehlgeschlagen. Lokale Daten bleiben verfügbar.' ||
          notice.value === 'Server erreichbar, aber Zugriff nicht erlaubt.') {
        notice.value = '';
      }
    } catch (cause) { markServerError(cause); }
    finally { probing.value = false; }
  }

  // Compare server catalog only; never change local Bauwerke or offline edits.
  async function checkForUpdates(): Promise<void> {
    online.value = navigator.onLine;
    if (!online.value) { serverState.value = 'offline'; return; }
    if (checking.value) return;
    checking.value = true;
    notice.value = '';

    let records: IBauwerk[];
    try {
      records = await fetchServerBauwerke();
      online.value = navigator.onLine;
      serverState.value = online.value ? 'available' : 'offline';
    } catch (cause) {
      markServerError(cause);
      const unauthorized = cause instanceof ServerHttpError && [401, 403].includes(cause.statusCode);
      notice.value = unauthorized
        ? 'Server erreichbar, aber Zugriff nicht erlaubt.'
        : 'Serverprüfung fehlgeschlagen. Lokale Daten bleiben verfügbar.';
      checking.value = false;
      return;
    }

    try {
      await recordServerCatalog(records);
      await loadLocal();
    } catch (cause) {
      // Local metadata errors should not misrepresent server connectivity.
      notice.value = `Server erreichbar, lokale Statusdaten konnten nicht aktualisiert werden: ${String(cause)}`;
    } finally { checking.value = false; }
  }

  async function markOpened(id: number): Promise<void> {
    await recordBauwerkOpened(id);
    await loadLocal();
  }

  return { native, bauwerke, available, lastServerCheck, loading, checking, probing,
    serverState, online, error, notice, loadLocal, checkServerAvailability, checkForUpdates, markOpened };
});
