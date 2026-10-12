# GATE-Startpunkt

Tauri-App (`App/`) und Django-Backend (`Server/`) für die Bauwerksdiagnostik.

## Voraussetzungen

Node.js 22+, Python 3.12, [uv](https://docs.astral.sh/uv/), Rust mit der [Tauri-Toolchain](https://tauri.app/start/prerequisites/).

## Starten

Backend:

```powershell
npm install
npm run server:install
npm run server:migrate
npm run server:seed      # optional: Demodaten
npm run server:dev       # http://127.0.0.1:8000/api
```

App, in einem zweiten Terminal:

```powershell
npm run dev              # Tauri-App
npm run dev:web          # alternativ im Browser: http://localhost:1420
```

Die API-Adresse der App steht in `VITE_VARS_API_BASE_URL` (Vorlage: `App/.env.example`).

## Prüfen

```powershell
npm run check
```
