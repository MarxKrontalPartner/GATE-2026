/**
 * Login and logout against Keycloak, through the Django endpoints in
 * Server/core/authentication (api/auth/token/, token/refresh/, logout/).
 *
 * Tokens are kept in localStorage, so the app stays logged in after a restart,
 * also while offline. `installAuthFetch` adds the access token to every request
 * to the API and gets a new one with the refresh token when the server answers 401.
 */
import { API_BASE_URL } from '@/config/apiBase';

const ACCESS_KEY = 'auth.access';
const REFRESH_KEY = 'auth.refresh';
const AUTH_URL = `${API_BASE_URL}/auth/`;

interface TokenPair { access: string; refresh: string }

function read(key: string): string | null {
  try {
    return localStorage.getItem(key);
  } catch {
    return null;
  }
}

function write(key: string, value: string | null) {
  try {
    if (value) localStorage.setItem(key, value);
    else localStorage.removeItem(key);
  } catch {
    // storage blocked: the login then only lasts until the next reload
  }
}

function store(tokens: TokenPair | null) {
  write(ACCESS_KEY, tokens?.access ?? null);
  write(REFRESH_KEY, tokens?.refresh ?? null);
}

/** The unpatched fetch, so auth requests never go through the token handling. */
const plainFetch: typeof window.fetch = window.fetch.bind(window);

async function postAuth(path: string, body: unknown): Promise<Response> {
  try {
    return await plainFetch(`${AUTH_URL}${path}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
  } catch {
    throw new Error('Der Server ist nicht erreichbar.');
  }
}

export function isLoggedIn(): boolean {
  return read(REFRESH_KEY) !== null;
}

export async function login(username: string, password: string): Promise<void> {
  const res = await postAuth('token/', { username, password });
  if (res.status === 401) throw new Error('Benutzername oder Passwort ist falsch.');
  if (res.status === 400) throw new Error('Bitte Benutzername und Passwort eingeben.');
  if (!res.ok) throw new Error(`Anmeldung fehlgeschlagen (HTTP ${res.status}).`);
  store(await res.json());
}

/** Ends the Keycloak session if the server is reachable; logs out locally in any case. */
export async function logout(): Promise<void> {
  const refresh = read(REFRESH_KEY);
  store(null);
  if (refresh) {
    try {
      await postAuth('logout/', { refresh });
    } catch {
      // offline: the local logout is enough, the session expires in Keycloak
    }
  }
}

let refreshing: Promise<string | null> | null = null;

/** New access token from the refresh token; null when the login has expired. */
function refreshAccess(): Promise<string | null> {
  refreshing ??= (async () => {
    try {
      const refresh = read(REFRESH_KEY);
      if (!refresh) return null;
      const res = await postAuth('token/refresh/', { refresh });
      if (res.status === 401) {
        store(null);
        return null;
      }
      if (!res.ok) return null;
      const tokens: TokenPair = await res.json();
      store(tokens);
      return tokens.access;
    } catch {
      return null; // offline: keep the tokens and try again later
    } finally {
      refreshing = null;
    }
  })();
  return refreshing;
}

function requestUrl(input: RequestInfo | URL): string {
  if (typeof input === 'string') return input;
  if (input instanceof URL) return input.href;
  return input.url;
}

function withToken(input: RequestInfo | URL, init: RequestInit | undefined, token: string): RequestInit {
  const headers = new Headers(init?.headers ?? (input instanceof Request ? input.headers : undefined));
  headers.set('Authorization', `Bearer ${token}`);
  return { ...init, headers };
}

/**
 * Adds the access token to all requests to the API (also those made with
 * window.fetch in other modules). On 401 it refreshes the token once and
 * repeats the request; if the login has expired, `onLoggedOut` is called.
 */
export function installAuthFetch(onLoggedOut: () => void) {
  window.fetch = async (input: RequestInfo | URL, init?: RequestInit) => {
    const url = requestUrl(input);
    const token = read(ACCESS_KEY);
    if (!url.startsWith(API_BASE_URL) || url.startsWith(AUTH_URL) || !token) {
      return plainFetch(input, init);
    }

    const response = await plainFetch(input, withToken(input, init, token));
    // A Request object's body can only be read once, so it is not repeated.
    if (response.status !== 401 || input instanceof Request) return response;

    const fresh = await refreshAccess();
    if (fresh) return plainFetch(input, withToken(input, init, fresh));
    if (!isLoggedIn()) onLoggedOut();
    return response;
  };
}
