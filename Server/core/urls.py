"""
URL configuration for core project.

Includes the assetverwaltung (Bauwerk, Projekt, Person, Firma) and bwd
(Bauwerksdiagnostik) APIs.
"""

from django.conf import settings
from django.conf.urls.static import static
from django.contrib import admin
from django.urls import include, path
from rest_framework.decorators import api_view
from rest_framework.response import Response


@api_view(["GET"])
def api_root(request, format=None):
    return Response(
        {
            "api/assetverwaltung": request.build_absolute_uri("api/assetverwaltung/"),
            "api/bwd": request.build_absolute_uri("api/bwd/"),
        }
    )


# Wire up our API using automatic URL routing.
# Additionally, we include login URLs for the browsable API.
urlpatterns = [
    path("", api_root, name="api_root"),
    path("api/", include("core.assetverwaltung.urls")),
    path("api/bwd/", include("core.bauwerksdiagnostik.urls")),
    path("api-auth/", include("rest_framework.urls", namespace="rest_framework")),
    path("admin/", admin.site.urls),
]

urlpatterns += static(settings.MEDIA_URL, document_root=settings.MEDIA_ROOT)
