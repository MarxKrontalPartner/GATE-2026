from functools import lru_cache

import jwt
import requests
from django.conf import settings
from django.contrib.auth import get_user_model
from rest_framework import authentication, exceptions


def issuer() -> str:
    return f"{settings.KEYCLOAK_URL}/realms/{settings.KEYCLOAK_REALM}"


def endpoint(name: str) -> str:
    return f"{issuer()}/protocol/openid-connect/{name}"


@lru_cache
def jwks_client() -> jwt.PyJWKClient:
    # downloads Keycloak's public keys once and caches them
    return jwt.PyJWKClient(endpoint("certs"))


# ---------- calls to Keycloak (used by the views) ----------

def keycloak_token(data: dict) -> dict | None:
    res = requests.post(
        endpoint("token"),
        data={"client_id": settings.KEYCLOAK_CLIENT_ID, **data},
        timeout=10,
    )
    return res.json() if res.status_code == 200 else None


def keycloak_logout(refresh_token: str) -> bool:
    # ends the Keycloak session, so the refresh token can't be used again
    res = requests.post(
        endpoint("logout"),
        data={"client_id": settings.KEYCLOAK_CLIENT_ID, "refresh_token": refresh_token},
        timeout=10,
    )
    return res.status_code == 204


# ---------- DRF authentication ----------

class KeycloakAuthentication(authentication.BaseAuthentication):
    def authenticate(self, request):
        parts = authentication.get_authorization_header(request).split()
        if not parts or parts[0].lower() != b"bearer":
            return None                     # no token: let SessionAuthentication try
        if len(parts) != 2:
            raise exceptions.AuthenticationFailed("Invalid Authorization header.")

        token = parts[1].decode()
        try:
            key = jwks_client().get_signing_key_from_jwt(token).key
            claims = jwt.decode(
                token, key,
                algorithms=["RS256"],
                issuer=issuer(),
                options={"verify_aud": False},
            )
        except jwt.PyJWTError as e:
            raise exceptions.AuthenticationFailed(f"Invalid token: {e}")

        if claims.get("azp") != settings.KEYCLOAK_CLIENT_ID:
            raise exceptions.AuthenticationFailed("Token was issued for another client.")

        user, _ = get_user_model().objects.get_or_create(
            username=claims["preferred_username"],
            defaults={"email": claims.get("email", "")},
        )
        return (user, claims)               # request.user, request.auth

    def authenticate_header(self, request):
        return 'Bearer realm="api"'         # DRF answers 401 (not 403), so the app refreshes