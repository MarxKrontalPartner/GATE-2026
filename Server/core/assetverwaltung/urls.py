"""URL routes of the Bauwerksverwaltung."""

from django.urls import include, path

from core.assetverwaltung.routers import CustomRouter
from core.assetverwaltung.views import bauwerksverwaltung

router_assetverwaltung = CustomRouter()

router_assetverwaltung.register(
    "bauwerk", bauwerksverwaltung.BauwerkViewSet, basename="bauwerk"
)
router_assetverwaltung.register(
    "firma", bauwerksverwaltung.FirmaViewSet, basename="firma"
)
router_assetverwaltung.register(
    "person", bauwerksverwaltung.PersonViewSet, basename="person"
)
router_assetverwaltung.register(
    "projekt", bauwerksverwaltung.ProjektViewSet, basename="projekt"
)


assetverwaltung_patterns = [
    *router_assetverwaltung.urls,
]

urlpatterns = [
    path("assetverwaltung/", include(assetverwaltung_patterns)),
]
