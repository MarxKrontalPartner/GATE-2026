# Login with Keycloak

Login is handled by [Keycloak](https://www.keycloak.org/). Django no longer issues
tokens itself (simplejwt has been replaced). Instead it

1. forwards the username and password to Keycloak and returns the tokens to the app, and
2. checks the Keycloak token in the `Authorization` header on every API request.


The app (`App/src-tauri/src/login.rs`, `App/src/utils/auth.ts`) only talks to Django and
doesn't need to know about Keycloak.

## Prerequisites

- [Docker](https://www.docker.com/products/docker-desktop/) (on macOS: Docker Desktop)
- Backend set up as described in the [main README](../README.md)

## 1. Start Keycloak

```bash
docker run -p 8080:8080 -e KC_BOOTSTRAP_ADMIN_USERNAME=admin -e KC_BOOTSTRAP_ADMIN_PASSWORD=admin quay.io/keycloak/keycloak:26.0 start-dev
```

Admin console: <http://localhost:8080> (login `admin` / `admin`).

`start-dev` is meant for local development only: data is stored inside the container and is
lost when the container is deleted.

## 2. Set up Keycloak

Do all steps in the **`MKP`** realm (selector at the top left, **not** `master`).

### Realm

*Create realm* → name `MKP`. The name is case-sensitive and appears exactly like this in all URLs.

### Client

*Clients → Create client*:

| Setting | Value |
|---|---|
| Client ID | `mkp-app` |
| Client authentication | **Off** (public client: the app can't keep a secret) |
| Direct access grants | **On** (allows login with username and password) |
| Standard flow | can stay on, not used |
| Web origins | `http://localhost:1420` |

### User

*Users → Add user* → choose a username, then on the *Credentials* tab set a password
with **Temporary = Off**. *Required user actions* must be empty.

### Token lifetimes (optional)

| Location | Setting | Default |
|---|---|---|
| *Realm settings → Tokens* | Access Token Lifespan | 5 minutes |
| *Realm settings → Sessions* | SSO Session Idle (≈ lifetime of the refresh token) | 30 minutes |
| *Realm settings → Sessions* | SSO Session Max | 10 hours |

## 3. Django configuration

In `Server/core/settings/extendable/base.py`:

```python
KEYCLOAK_URL = "http://localhost:8080"
KEYCLOAK_REALM = "MKP"
KEYCLOAK_CLIENT_ID = "mkp-app"

REST_FRAMEWORK = {
    ...
    "DEFAULT_AUTHENTICATION_CLASSES": [
        "core.authentication.keycloak.KeycloakAuthentication",
        "rest_framework.authentication.SessionAuthentication",
    ],
}
```

Dependencies (already in `Server/pyproject.toml`): `requests` (calls to Keycloak) and
`pyjwt[crypto]` (checking the RS256 signature).

### Files involved

| File | Purpose |
|---|---|
| `Server/core/authentication/keycloak.py` | `KeycloakAuthentication` (token check) and `keycloak_token()` (calls Keycloak's `/token`) |
| `Server/core/authentication/views.py` | `TokenView` and `TokenRefreshView` |
| `Server/core/urls.py` | routes `/api/token/` and `/api/token/refresh/` |

## 4. Endpoints

### `POST /api/token/`: log in

```json
{ "username": "username", "password": "secret" }
```

| Status | Response |
|---|---|
| 200 | `{ "access": "...", "refresh": "..." }` |
| 400 | username or password missing |
| 401 | wrong username or password |

### `POST /api/token/refresh/`: renew the access token

```json
{ "refresh": "..." }
```

| Status | Response |
|---|---|
| 200 | `{ "access": "...", "refresh": "..." }` (a **new** refresh token; the old one expires sooner) |
| 401 | refresh token invalid or expired → log in again |

### Protected API calls

Header `Authorization: Bearer <access>`. If the token is invalid or expired, Django answers
with **401**, and the app then gets a new token from `/api/token/refresh/`.

## 5. How the token check works

`KeycloakAuthentication` (in `keycloak.py`) checks on every request:

1. **Signature**: using the public key from
   `http://localhost:8080/realms/MKP/protocol/openid-connect/certs`. The keys are loaded
   once and cached, so Django doesn't call Keycloak on every request.
2. **`exp`**: the token hasn't expired.
3. **`iss`**: the token comes from `http://localhost:8080/realms/MKP`.
4. **`azp`**: the token was issued for the client `mkp-app`.

After that, the Django user is looked up by `preferred_username`, or created on the first
call. Passwords are stored only in Keycloak.

## 6. Testing

Is Keycloak reachable?

```bash
curl http://localhost:8080/realms/MKP/.well-known/openid-configuration
```

Get a token directly from Keycloak (body as form data, **not** JSON):

```bash
curl -X POST http://localhost:8080/realms/MKP/protocol/openid-connect/token -d grant_type=password -d client_id=mkp-app -d username=anna -d password=secret -d scope=openid
```

Get a token through Django (JSON):

```bash
curl -X POST http://127.0.0.1:8000/api/token/ -H "Content-Type: application/json" -d '{"username": "anna", "password": "secret"}'
```

Call a protected endpoint with the token:

```bash
curl http://127.0.0.1:8000/api/bauwerk/ -H "Authorization: Bearer <access>"
```

> Note: `Server/core/settings/local.py` removes the `IsAuthenticated` permission locally,
> so most endpoints there also answer **without** a token. To test the login, use an
> endpoint that sets `permission_classes = [IsAuthenticated]` itself.

## 7. Troubleshooting

| Error | Cause / fix |
|---|---|
| `Realm does not exist` | Realm name misspelled. It's `MKP` (capital letters). |
| 404 on `.../protocol/openid-connect` | That's only the start of the URLs. Always add `/token`, `/certs`, `/userinfo` or `/logout`. |
| `invalid_client` | Wrong client ID, or *Client authentication* is on. |
| `unauthorized_client` | *Direct access grants* is off for the client. |
| `invalid_grant`: "Invalid user credentials" | Wrong username or password. |
| `invalid_grant`: "Account is not fully set up" | Temporary password or pending *Required user actions* on the user. |
| "Missing form parameter: grant_type" | The body was sent to Keycloak as JSON instead of form data. |
| Django: `Invalid token: Invalid issuer` | `localhost` and `127.0.0.1` count as different. The token must be fetched through the same address that's in `KEYCLOAK_URL`. |
| Django: `Connection refused` while loading keys | Keycloak isn't running (`docker ps`). |
| The app logs out after about 30 minutes | See "Known limitations". |

## Known limitations

- **The app doesn't save the new refresh token yet.** Keycloak returns a new refresh token
  on every refresh. `login.rs` (`struct Refreshed`) and `auth.ts` (`browserRefresh`)
  currently keep only the access token. As a result, the user is logged out when the first
  refresh token expires (30 minutes by default), even while actively working.
- **No logout in Keycloak.** On logout the app only discards its tokens; the Keycloak
  session stays open until it times out.
- **Password login (Direct Access Grant).** The password passes through the app and Django,
  so two-factor login and single sign-on don't work. Those would need the browser login
  (Authorization Code with PKCE).
- **Configuration is hard-coded.** `KEYCLOAK_URL`, `KEYCLOAK_REALM` and `KEYCLOAK_CLIENT_ID`
  are set in `base.py` and must be changed for other environments.