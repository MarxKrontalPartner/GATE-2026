from django.urls import path

from core.authentication.views import LogoutView, TokenRefreshView, TokenView

urlpatterns = [
    path("token/", TokenView.as_view(), name="token"),
    path("token/refresh/", TokenRefreshView.as_view(), name="token_refresh"),
    path("logout/", LogoutView.as_view(), name="logout"),
]
