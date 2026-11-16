# Dashboard connectivity fix — Stage 3

## Reported behavior

Stopping Django left the "Updates prüfen" control spinning and the separate server indicator did not recover after Django restarted.

## Root causes and changes

1. `BauwerkswahlView.vue` registered focus/online/offline listeners and the interval **after** awaiting `checkForUpdates()`. Monitoring now starts immediately and probes every 15 seconds.
2. `DashboardStore.ts` previously prevented a reachability probe whenever a catalog check was in progress. The two operations are now independent.
3. `dashboardRepository.ts` cleared the abort timeout after response **headers**, but before `response.json()`. The 8-second catalog deadline now covers the body as well, with a race to settle even if an underlying fetch hangs. A lightweight HEAD probe has its own 5-second deadline.
4. Errors saving the catalog to SQLite no longer incorrectly mark Django as unavailable; a subdued inline notice reports them.
5. Fixes the earlier TS2367 error by checking the HTTP error directly, rather than relying on narrowing of the `serverState` ref.

## Manual test

1. Keep the tablet app running, with Django online. Expect `Gerät online` and `Server erreichbar`.
2. Stop Django, but leave Wi-Fi on. Within approximately 20 seconds (15-second polling + up to 5-second probe), expect `Gerät online` and `Server nicht erreichbar`.
3. Tap `Updates prüfen` while Django is down. The spinner should stop after at most about 8 seconds; local records should remain visible.
4. Restart Django. Within approximately 20 seconds expect `Server erreichbar`, without restarting Tauri or pressing the button.
5. Switch Wi-Fi off/on. The `Gerät` indicator reflects `navigator.onLine`; Django remains a separately probed status.

## Validate before committing

From the repository root:

```bash
npm --workspace App run type-check
npm --workspace App run test
npm --workspace App run build
npm run dev
```

No changes are made to Rust, migrations, Django or the SQLite database path. This does not implement synchronizing local edits.
