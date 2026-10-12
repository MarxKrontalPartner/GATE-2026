from core.assetverwaltung.views.base_viewset import BaseViewSet
from core.bauwerksdiagnostik.models.projektspezifisch import Untersuchungsstelle
from core.bauwerksdiagnostik.serializer.projektspezifischSerializer import (
    UntersuchungsstelleSerializer,
)


class UntersuchungsstelleViewSet(BaseViewSet):
    serializer_class = UntersuchungsstelleSerializer

    class Meta(BaseViewSet.Meta):
        model = Untersuchungsstelle

    def get_queryset(self):
        queryset = super().get_queryset()
        projekt_id = self.request.query_params.get("projekt_id", None)
        bauwerk_id = self.request.query_params.get("bauwerk_id", None)

        if projekt_id is not None:
            queryset = queryset.filter(projekt=projekt_id)
        if bauwerk_id is not None:
            queryset = queryset.filter(bauwerk=bauwerk_id)

        return queryset
