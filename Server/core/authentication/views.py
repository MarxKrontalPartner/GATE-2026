from rest_framework.permissions import AllowAny
from rest_framework.response import Response
from rest_framework.views import APIView

from core.authentication.keycloak import keycloak_logout, keycloak_token


def to_app(tokens: dict) -> dict:
    return {"access": tokens["access_token"], "refresh": tokens["refresh_token"]}


class TokenView(APIView):
    authentication_classes = []
    permission_classes = [AllowAny]

    def post(self, request):
        username, password = request.data.get("username"), request.data.get("password")
        if not username or not password:
            return Response({"detail": "Username and password are required."}, status=400)
        tokens = keycloak_token({"grant_type": "password", "username": username,
                                 "password": password, "scope": "openid"})
        if tokens is None:
            return Response({"detail": "Wrong username or password."}, status=401)
        return Response(to_app(tokens))


class TokenRefreshView(APIView):
    authentication_classes = []
    permission_classes = [AllowAny]

    def post(self, request):
        tokens = keycloak_token({"grant_type": "refresh_token",
                                 "refresh_token": request.data.get("refresh", "")})
        if tokens is None:
            return Response({"detail": "Token is invalid or expired."}, status=401)
        return Response(to_app(tokens))


class LogoutView(APIView):
    authentication_classes = []
    permission_classes = [AllowAny]

    def post(self, request):
        refresh = request.data.get("refresh")
        if not refresh:
            return Response({"detail": "Refresh token is required."}, status=400)
        if not keycloak_logout(refresh):
            return Response({"detail": "Token is invalid or expired."}, status=401)
        return Response(status=204)
