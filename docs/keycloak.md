# Keycloak Setup

Login runs through [Keycloak](https://www.keycloak.org/). Follow these steps once to run it locally.

## 1. Start Keycloak

Requires [Docker](https://www.docker.com/products/docker-desktop/).

```bash
docker run -p 8080:8080 -e KC_BOOTSTRAP_ADMIN_USERNAME=admin -e KC_BOOTSTRAP_ADMIN_PASSWORD=admin quay.io/keycloak/keycloak:26.0 start-dev
```

Open <http://localhost:8080> and log in with `admin` / `admin`.

## 2. Create the realm

Top left dropdown → **Create realm** → name: `MKP` (capital letters).

## 3. Create the client

Make sure **MKP** is selected top left, then **Clients → Create client**:

- Client ID: `mkp-app`
- Client authentication: **Off**
- Direct access grants: **On**
- Web origins: `http://localhost:1420`

## 4. Create a user

**Users → Add user** → enter a username → **Create**.
Then **Credentials → Set password**, with **Temporary: Off**.

## 5. Start the project

```bash
npm run server:dev
npm run dev
```

Log in to the app with the user from step 4.

## Check it works

```bash
curl -X POST http://127.0.0.1:8000/api/token/ -H "Content-Type: application/json" -d '{"username": "YOUR_USER", "password": "YOUR_PASSWORD"}'
```

You should get back `{"access": "...", "refresh": "..."}`.

## Common problems

| Error | Fix |
|---|---|
| `Realm does not exist` | The realm must be called `MKP`, in capitals. |
| `unauthorized_client` | Turn **Direct access grants** on for `mkp-app`. |
| `invalid_client` | Check the client ID is `mkp-app` and Client authentication is off. |
| `Account is not fully set up` | Set the user's password with Temporary off. |
| `Connection refused` | Keycloak isn't running. Start it again (step 1). |

Note: data is lost when the Docker container is removed. Then repeat steps 2–4.
