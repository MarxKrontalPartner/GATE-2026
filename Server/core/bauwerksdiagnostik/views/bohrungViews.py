from core.assetverwaltung.views.base_viewset import BaseViewSet
from core.bauwerksdiagnostik.models.bohrung import (
    Bohrkern,
    Kernbohrung,
)
from core.bauwerksdiagnostik.serializer.bohrungSerializer import (
    BohrkernSerializer,
    KernbohrungSerializer,
)


class KernbohrungViewSet(BaseViewSet):
    serializer_class = KernbohrungSerializer

    class Meta(BaseViewSet.Meta):
        model = Kernbohrung


class BohrkernViewSet(BaseViewSet):
    serializer_class = BohrkernSerializer

    class Meta(BaseViewSet.Meta):
        model = Bohrkern
