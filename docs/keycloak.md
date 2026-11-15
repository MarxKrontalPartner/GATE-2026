# Login mit Keycloak

Die Anmeldung läuft über [Keycloak](https://www.keycloak.org/). Django stellt selbst keine
Tokens mehr aus (simplejwt wurde ersetzt), sondern

1. leitet Benutzername und Passwort an Keycloak weiter und gibt die Tokens an die App zurück, und
2. prüft bei jeder API-Anfrage das Keycloak-Token im `Authorization`-Header.

```
App ── {username, password} ──► Django /api/token/ ── grant_type=password ──► Keycloak
App ◄────── {access, refresh} ── Django ◄────── access_token, refresh_token ── Keycloak

App ── Authorization: Bearer <access> ──► Django API
                                           └─ prüft Signatur mit Keycloaks öffentlichem
                                              Schlüssel (/certs, wird zwischengespeichert)
```

Die App (`App/src-tauri/src/login.rs`, `App/src/utils/auth.ts`) spricht nur mit Django und
muss Keycloak nicht kennen.

## Voraussetzungen

- [Docker](https://www.docker.com/products/docker-desktop/) (unter macOS: Docker Desktop)
- Backend wie im [Haupt-README](../README.md) eingerichtet

## 1. Keycloak starten

```bash
docker run -p 8080:8080 -e KC_BOOTSTRAP_ADMIN_USERNAME=admin -e KC_BOOTSTRAP_ADMIN_PASSWORD=admin quay.io/keycloak/keycloak:26.0 start-dev
```

Admin-Konsole: <http://localhost:8080> (Login `admin` / `admin`).

`start-dev` ist nur für die lokale Entwicklung gedacht: Daten liegen im Container und sind
nach dem Löschen des Containers weg.

## 2. Keycloak einrichten

Alle Schritte im Realm **`MKP`** ausführen (Auswahl oben links, **nicht** `master`).

### Realm

*Create realm* → Name `MKP`. Der Name ist case-sensitiv und steht so in allen URLs.

### Client

*Clients → Create client*:

| Einstellung | Wert |
|---|---|
| Client ID | `mkp-app` |
| Client authentication | **Aus** (öffentlicher Client, die App kann kein Secret geheim halten) |
| Direct access grants | **An** (erlaubt die Anmeldung mit Benutzername und Passwort) |
| Standard flow | darf an bleiben, wird nicht genutzt |
| Web origins | `http://localhost:1420` |

### Benutzer

*Users → Add user* → Benutzername vergeben, dann im Tab *Credentials* ein Passwort setzen
mit **Temporary = Aus**. Unter *Required user actions* darf nichts eingetragen sein.

### Token-Laufzeiten (optional)

| Ort | Einstellung | Standard |
|---|---|---|
| *Realm settings → Tokens* | Access Token Lifespan | 5 Minuten |
| *Realm settings → Sessions* | SSO Session Idle (≈ Laufzeit des Refresh-Tokens) | 30 Minuten |
| *Realm settings → Sessions* | SSO Session Max | 10 Stunden |

## 3. Django-Konfiguration

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

Abhängigkeiten (bereits in `Server/pyproject.toml`): `requests` (Aufrufe an Keycloak) und
`pyjwt[crypto]` (Prüfung der RS256-Signatur).

### Beteiligte Dateien

| Datei | Aufgabe |
|---|---|
| `Server/core/authentication/keycloak.py` | `KeycloakAuthentication` (Token-Prüfung) und `keycloak_token()` (Aufruf von Keycloaks `/token`) |
| `Server/core/authentication/views.py` | `TokenView` und `TokenRefreshView` |
| `Server/core/urls.py` | Routen `/api/token/` und `/api/token/refresh/` |

## 4. Endpunkte

### `POST /api/token/` – Anmelden

```json
{ "username": "anna", "password": "geheim" }
```

| Status | Antwort |
|---|---|
| 200 | `{ "access": "...", "refresh": "..." }` |
| 400 | Benutzername oder Passwort fehlt |
| 401 | Benutzername oder Passwort falsch |

### `POST /api/token/refresh/` – Access-Token erneuern

```json
{ "refresh": "..." }
```

| Status | Antwort |
|---|---|
| 200 | `{ "access": "...", "refresh": "..." }` – **neues** Refresh-Token, das alte verfällt früher |
| 401 | Refresh-Token ungültig oder abgelaufen → neu anmelden |

### Geschützte API-Aufrufe

Header `Authorization: Bearer <access>`. Bei ungültigem oder abgelaufenem Token antwortet
Django mit **401**; die App holt sich dann über `/api/token/refresh/` ein neues Token.

## 5. Wie die Token-Prüfung funktioniert

`KeycloakAuthentication` (in `keycloak.py`) prüft bei jeder Anfrage:

1. **Signatur** – mit dem öffentlichen Schlüssel aus
   `http://localhost:8080/realms/MKP/protocol/openid-connect/certs`. Die Schlüssel werden
   einmal geladen und zwischengespeichert; Django ruft Keycloak dafür nicht bei jeder Anfrage auf.
2. **`exp`** – das Token ist nicht abgelaufen.
3. **`iss`** – das Token stammt aus `http://localhost:8080/realms/MKP`.
4. **`azp`** – das Token wurde für den Client `mkp-app` ausgestellt.

Danach wird der Django-Benutzer anhand von `preferred_username` gesucht bzw. beim ersten
Aufruf angelegt. Passwörter liegen nur in Keycloak.

## 6. Testen

Keycloak erreichbar?

```bash
curl http://localhost:8080/realms/MKP/.well-known/openid-configuration
```

Token direkt bei Keycloak holen (Body als Formular, **nicht** JSON):

```bash
curl -X POST http://localhost:8080/realms/MKP/protocol/openid-connect/token -d grant_type=password -d client_id=mkp-app -d username=anna -d password=geheim -d scope=openid
```

Token über Django holen (JSON):

```bash
curl -X POST http://127.0.0.1:8000/api/token/ -H "Content-Type: application/json" -d '{"username": "anna", "password": "geheim"}'
```

Geschützten Endpunkt mit dem Token aufrufen:

```bash
curl http://127.0.0.1:8000/api/bauwerk/ -H "Authorization: Bearer <access>"
```

> Hinweis: `Server/core/settings/local.py` entfernt lokal die Berechtigung `IsAuthenticated`.
> Die meisten Endpunkte antworten dort auch **ohne** Token. Zum Testen der Anmeldung einen
> Endpunkt verwenden, der `permission_classes = [IsAuthenticated]` selbst setzt.

## 7. Fehlerbehebung

| Fehler | Ursache / Lösung |
|---|---|
| `Realm does not exist` | Realm-Name falsch geschrieben – es heißt `MKP` (Großbuchstaben). |
| 404 auf `.../protocol/openid-connect` | Das ist nur der Anfang der URLs. Immer `/token`, `/certs`, `/userinfo` oder `/logout` anhängen. |
| `invalid_client` | Client ID falsch oder *Client authentication* ist an. |
| `unauthorized_client` | *Direct access grants* ist beim Client aus. |
| `invalid_grant` – „Invalid user credentials" | Benutzername oder Passwort falsch. |
| `invalid_grant` – „Account is not fully set up" | Temporäres Passwort oder offene *Required user actions* beim Benutzer. |
| „Missing form parameter: grant_type" | Body wurde als JSON statt als Formular an Keycloak geschickt. |
| Django: `Invalid token: Invalid issuer` | `localhost` und `127.0.0.1` gelten als verschieden. Das Token muss über dieselbe Adresse geholt werden, die in `KEYCLOAK_URL` steht. |
| Django: `Connection refused` beim Laden der Schlüssel | Keycloak läuft nicht (`docker ps`). |
| App wird nach ca. 30 Minuten abgemeldet | Siehe „Bekannte Einschränkungen". |

## Bekannte Einschränkungen

- **Neues Refresh-Token wird in der App noch nicht gespeichert.** Keycloak gibt bei jedem
  Refresh ein neues Refresh-Token zurück. `login.rs` (`struct Refreshed`) und
  `auth.ts` (`browserRefresh`) übernehmen bisher nur das Access-Token. Folge: Nach Ablauf
  des ersten Refresh-Tokens (Standard 30 Minuten) wird der Benutzer abgemeldet, auch wenn
  er aktiv arbeitet.
- **Kein Logout bei Keycloak.** Die App verwirft beim Abmelden nur ihre Tokens; die Sitzung
  in Keycloak läuft bis zum Timeout weiter.
- **Passwort-Login (Direct Access Grant).** Das Passwort läuft durch App und Django;
  Zwei-Faktor-Anmeldung und Single Sign-On funktionieren damit nicht. Für diese Funktionen
  wäre der Browser-Login (Authorization Code mit PKCE) nötig.
- **Konfiguration fest im Code.** `KEYCLOAK_URL`, `KEYCLOAK_REALM` und `KEYCLOAK_CLIENT_ID`
  stehen in `base.py` und müssen für andere Umgebungen angepasst werden.
