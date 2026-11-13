import { invoke, isTauri } from '@tauri-apps/api/core';

// Login against Django simplejwt, working in two places:
//  - Tauri window: calls the Rust commands in src-tauri/src/login.rs (tokens stay in Rust).
//  - Plain browser (npm run dev:web): calls Django directly with fetch.
//    Tokens are kept in sessionStorage, so a page reload keeps you logged in
//    and closing the tab logs you out.

const API_BASE_URL: string =
  import.meta.env.VITE_VARS_API_BASE_URL ?? 'http://127.0.0.1:8000/api';

const ACCESS_KEY = 'auth.access';
const REFRESH_KEY = 'auth.refresh';

// ---------- browser helpers ----------

function readToken(key: string): string | null {
  try {
    return sessionStorage.getItem(key);
  } catch {
    return null;
  }
}

function writeToken(key: string, value: string | null) {
  try {
    if (value) sessionStorage.setItem(key, value);
    else sessionStorage.removeItem(key);
  } catch {
    // storage blocked: the login then only lasts until the next reload
  }
}

async function postJson(path: string, body: unknown): Promise<Response> {
  try {
    return await fetch(`${API_BASE_URL}${path}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
  } catch {
    throw new Error('Server not reachable');
  }
}

async function browserLogin(username: string, password: string) {
  const res = await postJson('/token/', { username, password });
  if (res.status === 401) throw new Error('Wrong username or password');
  if (res.status === 400) throw new Error('Username and password are required');
  if (!res.ok) throw new Error(`Login failed: ${res.status}`);
  const data: { access: string; refresh: string } = await res.json();
  writeToken(ACCESS_KEY, data.access);
  writeToken(REFRESH_KEY, data.refresh);
}

function browserLogout() {
  writeToken(ACCESS_KEY, null);
  writeToken(REFRESH_KEY, null);
}

async function browserRefresh(): Promise<string> {
  const refresh = readToken(REFRESH_KEY);
  if (!refresh) throw new Error('NOT_LOGGED_IN');
  const res = await postJson('/token/refresh/', { refresh });
  if (!res.ok) {
    browserLogout();
    throw new Error('NOT_LOGGED_IN');
  }
  const data: { access: string } = await res.json();
  writeToken(ACCESS_KEY, data.access);
  return data.access;
}

async function browserApi<T>(method: string, path: string, body?: unknown): Promise<T> {
  let token = readToken(ACCESS_KEY) ?? (await browserRefresh());

  for (let attempt = 0; attempt < 2; attempt++) {
    let res: Response;
    try {
      res = await fetch(`${API_BASE_URL}${path}`, {
        method: method.toUpperCase(),
        headers: {
          Authorization: `Bearer ${token}`,
          ...(body !== undefined ? { 'Content-Type': 'application/json' } : {}),
        },
        body: body !== undefined ? JSON.stringify(body) : undefined,
      });
    } catch {
      throw new Error('Server not reachable');
    }

    if (res.status === 401) {
      if (attempt === 0) {
        token = await browserRefresh();
        continue;
      }
      browserLogout();
      throw new Error('NOT_LOGGED_IN');
    }
    if (!res.ok) throw new Error(`${res.status}: ${await res.text()}`);
    if (res.status === 204) return null as T;
    return (await res.json()) as T;
  }
  throw new Error('NOT_LOGGED_IN');
}

// ---------- public API (same in Tauri and browser) ----------

export async function login(username: string, password: string): Promise<void> {
  if (isTauri()) {
    await invoke('login', { username, password });
  } else {
    await browserLogin(username, password);
  }
}

export async function logout(): Promise<void> {
  if (isTauri()) {
    await invoke('logout');
  } else {
    browserLogout();
  }
}

export async function isLoggedIn(): Promise<boolean> {
  if (isTauri()) return invoke<boolean>('is_logged_in');
  return readToken(REFRESH_KEY) !== null;
}

/** Authenticated request to Django, e.g. api('GET', '/me/'). */
export async function api<T = unknown>(method: string, path: string, body?: unknown): Promise<T> {
  if (isTauri()) return invoke<T>('api', { method, path, body: body ?? null });
  return browserApi<T>(method, path, body);
}
