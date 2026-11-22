// The base URL must not end with a slash, because the BWD helpers resolve
// relative URLs like new URL('api/bwd/...', API_BASE_URL) against it.
export const API_BASE_URL: string =
  import.meta.env.VITE_VARS_API_BASE_URL ?? 'http://127.0.0.1:8000/api';
