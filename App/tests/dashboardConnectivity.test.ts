import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { createPinia, setActivePinia } from 'pinia';
import { useDashboardStore } from '@/store/DashboardStore';

vi.mock('@tauri-apps/api/core', () => ({
  isTauri: () => false,
  invoke: vi.fn(),
}));

vi.mock('@/utils/ApiConnector', () => ({
  API_BASE_URL: 'http://127.0.0.1:8000/api',
  apiConnector: {
    makeHeaders: vi.fn().mockResolvedValue({}),
    getBauwerke: vi.fn().mockResolvedValue([]),
  },
}));

describe('Dashboard Django connectivity', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.stubGlobal('navigator', { onLine: true });
  });

  afterEach(() => {
    vi.useRealTimers();
    vi.unstubAllGlobals();
  });

  it('changes from available to unavailable and back when Django stops and restarts', async () => {
    const fetchMock = vi.fn()
      .mockResolvedValueOnce(new Response(null, { status: 200 }))
      .mockRejectedValueOnce(new TypeError('Failed to fetch'))
      .mockResolvedValueOnce(new Response(null, { status: 200 }));
    vi.stubGlobal('fetch', fetchMock);
    const dashboard = useDashboardStore();

    await dashboard.checkServerAvailability();
    expect(dashboard.serverState).toBe('available');

    await dashboard.checkServerAvailability();
    expect(dashboard.serverState).toBe('unavailable');
    expect(dashboard.online).toBe(true); // Wi-Fi is independent from Django.

    await dashboard.checkServerAvailability();
    expect(dashboard.serverState).toBe('available');
    expect(dashboard.probing).toBe(false);
  });

  it('does not keep Updates prüfen spinning when response JSON stalls', async () => {
    vi.useFakeTimers();
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: () => new Promise<never>(() => {}),
    }));
    const dashboard = useDashboardStore();
    const check = dashboard.checkForUpdates();
    expect(dashboard.checking).toBe(true);

    await vi.advanceTimersByTimeAsync(8100);
    await check;
    expect(dashboard.checking).toBe(false);
    expect(dashboard.serverState).toBe('unavailable');
    expect(dashboard.notice).toContain('Serverprüfung fehlgeschlagen');
  });

  it('can probe the server while Updates prüfen is still running', async () => {
    vi.useFakeTimers();
    vi.stubGlobal('fetch', vi.fn().mockImplementation((_url, init: RequestInit) =>
      init.method === 'HEAD'
        ? Promise.resolve(new Response(null, { status: 200 }))
        : new Promise<Response>(() => {}),
    ));
    const dashboard = useDashboardStore();
    const updateCheck = dashboard.checkForUpdates();
    await dashboard.checkServerAvailability();
    expect(dashboard.serverState).toBe('available');
    expect(dashboard.checking).toBe(true);

    await vi.advanceTimersByTimeAsync(8100);
    await updateCheck;
    expect(dashboard.checking).toBe(false);
  });
});
