# Keycloak für die Anmeldung einrichten

Die App meldet Benutzer über **Keycloak** an. Django leitet Benutzername und
Passwort an Keycloak weiter und prüft bei jeder API-Anfrage das Token.

Diese Anleitung richtet Keycloak lokal in Docker ein – mit genau den Namen,
die der Server erwartet (`Server/core/settings/extendable/base.py`):

| Einstellung          | Wert                    |
|----------------------|-------------------------|
| `KEYCLOAK_URL`       | `http://localhost:8080` |
| `KEYCLOAK_REALM`     | `MKP`                   |
| `KEYCLOAK_CLIENT_ID` | `mkp-app`               |

> Groß-/Kleinschreibung zählt: Der Realm heißt `MKP` (nicht `mkp`), der Client `mkp-app`.

**Überblick**

1. Keycloak in Docker starten
2. In der Admin-Konsole anmelden
3. Realm `MKP` anlegen
4. Client `mkp-app` anlegen
5. Benutzer anlegen (E-Mail, Vorname, Nachname)
6. Passwort setzen (**Temporär: Aus**)
7. Anmeldung testen
8. Fehlerbehebung

---

## Voraussetzungen

- **Docker Desktop** ist installiert und läuft (`docker --version` zeigt eine Version).
- Port **8080** ist frei. Prüfen:

  ```powershell
  docker ps --format "{{.Names}}  {{.Image}}  {{.Ports}}"
  ```

  Läuft dort schon ein Keycloak-Container auf 8080, entweder diesen weiterverwenden
  (weiter mit Schritt 2) oder ihn zuerst stoppen: `docker stop <Name>`.

---

## 1. Keycloak in Docker starten

In PowerShell:

```powershell
docker run -d --name keycloak -p 8080:8080 `
  -e KC_BOOTSTRAP_ADMIN_USERNAME=admin `
  -e KC_BOOTSTRAP_ADMIN_PASSWORD=admin `
  -v keycloak_data:/opt/keycloak/data `
  quay.io/keycloak/keycloak:26.0 start-dev
```

Was die Optionen bedeuten:

| Option | Bedeutung |
|---|---|
| `-d` | läuft im Hintergrund |
| `--name keycloak` | fester Name, damit man den Container später einfach starten/stoppen kann |
| `-p 8080:8080` | Keycloak ist unter `http://localhost:8080` erreichbar |
| `KC_BOOTSTRAP_ADMIN_USERNAME` / `..._PASSWORD` | Administrator für die Admin-Konsole (`admin` / `admin`, nur lokal!) |
| `-v keycloak_data:/opt/keycloak/data` | Realm, Client und Benutzer bleiben erhalten, auch wenn der Container gelöscht wird |
| `start-dev` | Entwicklungsmodus: HTTP statt HTTPS, eingebaute Datenbank |

Der Start dauert etwa 20–40 Sekunden. Fortschritt ansehen:

```powershell
docker logs -f keycloak
```

Fertig ist Keycloak, wenn eine Zeile wie `Keycloak 26.0... started in ...` und
`Listening on: http://0.0.0.0:8080` erscheint. Mit `Strg+C` die Log-Anzeige beenden
(der Container läuft weiter).

**Später** starten und stoppen (die Daten bleiben erhalten):

```powershell
docker stop keycloak
```

```powershell
docker start keycloak
```

---

## 2. In der Admin-Konsole anmelden

1. Im Browser **http://localhost:8080** öffnen.
2. **Administration Console** wählen (falls nicht direkt das Anmeldeformular erscheint).
3. Anmelden mit Benutzername `admin` und Passwort `admin`.

Oben links steht jetzt der Realm **Keycloak / master**. Den `master`-Realm
**nicht** für die App verwenden – er ist nur für die Verwaltung da.

---

## 3. Realm `MKP` anlegen

Ein Realm ist ein eigener Bereich mit eigenen Benutzern und Clients.

1. Oben links auf das Realm-Menü klicken (zeigt `master`).
2. **Create realm** wählen.
3. Ausfüllen:
   - **Realm name:** `MKP`
   - **Enabled:** **On**
4. **Create** klicken.

Oben links steht jetzt **MKP**. Alle folgenden Schritte finden in diesem Realm statt –
vor jedem Schritt kurz prüfen, dass oben links `MKP` ausgewählt ist.

---

## 4. Client `mkp-app` anlegen

Der Client steht für die App. Django holt die Tokens mit Benutzername und Passwort
(„Direct access grants“) und sendet dabei **kein Client-Secret** – der Client muss
deshalb *öffentlich* sein.

1. Links **Clients** → **Create client**.
2. **Seite 1 – General settings**
   - **Client type:** `OpenID Connect`
   - **Client ID:** `mkp-app`
   - **Name:** z. B. `MKP App` (optional)
   - **Next**
3. **Seite 2 – Capability config**
   - **Client authentication:** **Off** ← wichtig (öffentlicher Client, kein Secret)
   - **Authorization:** **Off**
   - **Authentication flow:**
     - ☑ **Standard flow**
     - ☑ **Direct access grants** ← wichtig (Anmeldung mit Benutzername + Passwort)
     - alle anderen ohne Haken
   - **Next**
4. **Seite 3 – Login settings**
   - Alle Felder leer lassen (werden für die Anmeldung über Django nicht gebraucht).
   - **Save**

Kontrolle: Unter **Clients** → `mkp-app` → Reiter **Settings** → Abschnitt
**Capability config** muss *Client authentication* auf **Off** und
*Direct access grants* angehakt sein.

---

## 5. Benutzer anlegen

1. Links **Users** → **Create new user** (bzw. **Add user**).
2. Ausfüllen:

   | Feld | Wert | Hinweis |
   |---|---|---|
   | **Required user actions** | *leer lassen* | sonst verlangt Keycloak bei der Anmeldung eine Aktion, und die App-Anmeldung schlägt fehl |
   | **Email verified** | **On** | sonst kann Keycloak eine Bestätigung verlangen |
   | **Username** | z. B. `max.mustermann` | damit meldet man sich in der App an (Kleinbuchstaben) |
   | **Email** | z. B. `max.mustermann@beispiel.de` | **Pflicht** – Anmeldung ist auch mit der E-Mail möglich |
   | **First name** | z. B. `Max` | **Pflicht** |
   | **Last name** | z. B. `Mustermann` | **Pflicht** |

3. **Create** klicken.

> **Warum E-Mail, Vorname und Nachname Pflicht sind:** Keycloak 26 prüft bei der Anmeldung
> das Benutzerprofil. Fehlt eines dieser Felder, antwortet Keycloak mit
> `Account is not fully set up` – in der App erscheint dann „Benutzername oder Passwort ist falsch“.

---

## 6. Passwort setzen – „Temporary“ ausschalten

Direkt nach dem Anlegen ist der neue Benutzer geöffnet (sonst: **Users** → Benutzer anklicken).

1. Reiter **Credentials** öffnen.
2. **Set password** klicken.
3. Ausfüllen:
   - **Password:** das gewünschte Passwort
   - **Password confirmation:** dasselbe Passwort noch einmal
   - **Temporary:** **Off** ← unbedingt ausschalten
4. **Save** klicken und die Rückfrage mit **Save password** bestätigen.

> **Warum Temporary aus sein muss:** Ein temporäres Passwort muss bei der ersten Anmeldung
> auf der Keycloak-Seite geändert werden. Die App meldet sich aber direkt über Django an
> und kann diese Seite nicht anzeigen – die Anmeldung schlägt mit
> `Account is not fully set up` fehl.

Dasselbe gilt beim späteren **Zurücksetzen** eines Passworts
(**Credentials** → **Reset password**): auch dort **Temporary: Off** wählen.

Kontrolle: Im Reiter **Details** des Benutzers darf unter **Required user actions**
nichts eingetragen sein (insbesondere nicht *Update Password*).

---

## 7. Anmeldung testen

### a) Direkt bei Keycloak

Prüft Realm, Client und Benutzer ohne Django. In PowerShell (`curl.exe`, nicht der
PowerShell-Alias `curl`; Benutzername und Passwort ersetzen):

```powershell
curl.exe -s -X POST "http://localhost:8080/realms/MKP/protocol/openid-connect/token" -d "client_id=mkp-app" -d "grant_type=password" -d "scope=openid" -d "username=max.mustermann" -d "password=DEIN_PASSWORT"
```

Erfolg: eine JSON-Antwort mit `"access_token"` und `"refresh_token"`.
Bei einem Fehler steht in `"error"` / `"error_description"` die Ursache (siehe Abschnitt 8).

### b) Über den Django-Server

Den Server starten (im Ordner `Server`):

```powershell
uv run python manage.py runserver
```

In einem zweiten Terminal:

```powershell
curl.exe -s -X POST "http://127.0.0.1:8000/api/auth/token/" -H "Content-Type: application/json" -d '{\"username\": \"max.mustermann\", \"password\": \"DEIN_PASSWORT\"}'
```

Erfolg: `{"access": "...", "refresh": "..."}`.
Falsche Angaben: `{"detail": "Wrong username or password."}` (HTTP 401).

Die Endpunkte des Servers:

| Endpunkt | Zweck | Body |
|---|---|---|
| `POST /api/auth/token/` | Anmelden | `{"username", "password"}` |
| `POST /api/auth/token/refresh/` | neues Access-Token | `{"refresh"}` |
| `POST /api/auth/logout/` | Abmelden (beendet die Keycloak-Sitzung) | `{"refresh"}` |

### c) In der App

1. App starten (im Ordner `App`): `npm run tauri dev`
2. Auf der Anmeldeseite **Benutzername oder E-Mail-Adresse** und **Passwort** eingeben → **Anmelden**.
3. Abmelden: oben rechts das Account-Symbol → **Abmelden**.

---

## 8. Fehlerbehebung

| Meldung | Ursache | Lösung |
|---|---|---|
| `Account is not fully set up` | Passwort ist temporär, eine *Required user action* ist gesetzt, oder E-Mail/Vorname/Nachname fehlen | Schritt 5 und 6 prüfen: Temporary **Off**, keine Required user actions, alle Profilfelder ausgefüllt |
| `Invalid user credentials` | Benutzername oder Passwort falsch, oder Benutzer im falschen Realm angelegt | Benutzer im Realm **MKP** prüfen, Passwort neu setzen (Temporary Off) |
| `invalid_client` / `Invalid client credentials` | *Client authentication* ist **On** | Client `mkp-app` → Settings → *Client authentication* **Off** → Save |
| `unauthorized_client` / `Client not allowed for direct access grants` | *Direct access grants* nicht angehakt | Client `mkp-app` → Settings → *Direct access grants* anhaken → Save |
| `Realm does not exist` | Realm-Name falsch geschrieben | Der Realm muss exakt `MKP` heißen |
| `Token was issued for another client.` | Token stammt von einem anderen Client | In der App mit dem Client `mkp-app` anmelden; `KEYCLOAK_CLIENT_ID` prüfen |
| App: „Der Server ist nicht erreichbar.“ | Django läuft nicht | `uv run python manage.py runserver` im Ordner `Server` |
| Django-Fehler 500 beim Anmelden, `ConnectionError` im Log | Keycloak läuft nicht | `docker start keycloak`, dann `docker logs -f keycloak` |
| Port 8080 belegt | anderer Dienst/Container auf 8080 | `docker ps` prüfen und den anderen Container stoppen |
| Realm/Benutzer nach Neustart weg | Container ohne Volume gestartet und gelöscht | Container mit `-v keycloak_data:/opt/keycloak/data` starten (Schritt 1) und Schritte 3–6 wiederholen |

---

## Hinweise

- `admin` / `admin` und `start-dev` sind **nur für die lokale Entwicklung**. Für einen echten
  Server braucht Keycloak HTTPS, eine eigene Datenbank und ein sicheres Admin-Passwort.
- Lokal nimmt der Server API-Anfragen auch **ohne** Token an, weil
  `Server/core/settings/local.py` die Berechtigung `IsAuthenticated` abschaltet.
  Die Anmeldepflicht gilt dort nur in der App.
- Keycloak komplett zurücksetzen (löscht Realm, Client und alle Benutzer):

  ```powershell
  docker rm -f keycloak
  ```

  ```powershell
  docker volume rm keycloak_data
  ```
