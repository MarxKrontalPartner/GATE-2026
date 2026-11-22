# urls.py in your new app
"""URL routes of the Bauwerksdiagnostik (bwd) API."""

from django.urls import include, path
from rest_framework.routers import DefaultRouter
from core.bauwerksdiagnostik.views.algemeinViews import (
    MaterialViewSet,
    MediaViewSet,
)
from core.bauwerksdiagnostik.views.bohrungViews import (
    BohrkernViewSet,
    KernbohrungViewSet,
)
from core.bauwerksdiagnostik.views.filterViews import (
    BearbeiterByIDsView,
    ProjektByBauwerkView,
    UntersuchungsstelleByBauwerkView,
)
from core.bauwerksdiagnostik.views.projektspezifisch import (
    UntersuchungsstelleViewSet,
)
from core.bauwerksdiagnostik.views.rueckprallhammerViews import (
    RueckprallhammerViewSet,
)
from core.bauwerksdiagnostik.views.untersuchungViews import (
    BearbeiterViewSet,
    ImplementedUntersuchungsverfahrenListView,
    UntersuchungsverfahrenViewSet,
)
from core.bauwerksdiagnostik.views.baustoffeigenschaftenViews import (
    ChloridanalyseViewSet,
    DruckfestigkeitspruefungViewSet,
    ProbeViewSet,
)
from core.bauwerksdiagnostik.views.rissbreitenmessungViews import RissbreitenmessungViewSet
from .views.structure_views import MediaUploadViewSet

router_bwd = DefaultRouter()

# Allgemein
router_bwd.register(r"Material", MaterialViewSet, basename="MaterialViewSet")

# Rueckprallhammer
router_bwd.register(
    r"RueckprallHammer", RueckprallhammerViewSet, basename="rueckprallHammerViewSet"
)

# Untersuchung
router_bwd.register(r"Bearbeiter", BearbeiterViewSet, basename="BearbeiterViewSet")
router_bwd.register(
    r"Untersuchungsverfahren",
    UntersuchungsverfahrenViewSet,
    basename="UntersuchungsverfahrenViewSet",
)
router_bwd.register(
    r"ImplementedUntersuchungsverfahren",
    ImplementedUntersuchungsverfahrenListView,
    basename="ImplementedUntersuchungsverfahrenListView",
)

# Projektspezifisch
router_bwd.register(
    r"Untersuchungsstelle",
    UntersuchungsstelleViewSet,
    basename="UntersuchungsstelleViewSet",
)

router_bwd.register(r"Media", MediaViewSet, basename="MediaViewSet")
router_bwd.register(r"upload-media", MediaUploadViewSet, basename="media-upload")

# Bohrung
router_bwd.register(r"Kernbohrung", KernbohrungViewSet, basename="KernbohrungViewSet")
router_bwd.register(r"Bohrkern", BohrkernViewSet, basename="BohrkernViewSet")

router_bwd.register(r"Probe", ProbeViewSet, basename="ProbeViewSet")

# Own endpoints for the remaining forms (list, bulk create, sync)
router_bwd.register(
    r"Rissbreitenmessung", RissbreitenmessungViewSet, basename="RissbreitenmessungViewSet"
)
router_bwd.register(
    r"Chloridanalyse", ChloridanalyseViewSet, basename="ChloridanalyseViewSet"
)
router_bwd.register(
    r"Druckfestigkeitspruefung",
    DruckfestigkeitspruefungViewSet,
    basename="DruckfestigkeitspruefungViewSet",
)

bwd_patterns = [
    path(
        r"UntersuchungsstelleByBauwerk/",
        UntersuchungsstelleByBauwerkView.as_view(),
        name="UntersuchungsstelleByBauwerk",
    ),
    path(
        r"BearbeiterByIDs/",
        BearbeiterByIDsView.as_view(),
        name="BearbeiterByIDs",
    ),
    path(
        r"ProjektByBauwerk/<int:bauwerk_id>/",
        ProjektByBauwerkView.as_view(),
        name="ProjektByBauwerk",
    ),
    *router_bwd.urls,
]

urlpatterns = [
    path("", include(bwd_patterns)),
]
