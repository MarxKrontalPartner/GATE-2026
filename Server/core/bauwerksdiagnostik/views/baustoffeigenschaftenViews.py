from core.assetverwaltung.views.base_viewset import BaseViewSet
from core.bauwerksdiagnostik.models.baustoffeigenschaften import (
    Chloridanalyse,
    Druckfestigkeitspruefung,
    Probe,
)
from core.bauwerksdiagnostik.serializer.baustoffeigenschaftenSerializer import (
    ChloridanalyseSerializer,
    DruckfestigkeitspruefungSerializer,
    ProbeSerializer,
)


class ChloridanalyseViewSet(BaseViewSet):
    serializer_class = ChloridanalyseSerializer

    class Meta(BaseViewSet.Meta):
        model = Chloridanalyse

    def get_queryset(self):
        queryset = Chloridanalyse.objects.all()
        project_id = self.request.query_params.get("projekt_id")
        if project_id:
            queryset = queryset.filter(projekt=project_id)
        return queryset


class DruckfestigkeitspruefungViewSet(BaseViewSet):
    serializer_class = DruckfestigkeitspruefungSerializer

    class Meta(BaseViewSet.Meta):
        model = Druckfestigkeitspruefung

    def get_queryset(self):
        queryset = Druckfestigkeitspruefung.objects.all()
        project_id = self.request.query_params.get("projekt_id")
        if project_id:
            queryset = queryset.filter(projekt=project_id)
        return queryset


class ProbeViewSet(BaseViewSet):
    serializer_class = ProbeSerializer

    def get_queryset(self):
        queryset = Probe.objects.all()
        projekt_id = self.request.query_params.get("projekt_id")
        bauwerk_id = self.request.query_params.get("bauwerk_id")
        bohrkern_id = self.request.query_params.get(
            "bohrkern"
        ) or self.request.query_params.get("bohrkern_id")

        if projekt_id:
            queryset = queryset.filter(projekt_id=projekt_id)

        if bauwerk_id:
            queryset = queryset.filter(bauwerk_id=bauwerk_id)

        if bohrkern_id:
            queryset = queryset.filter(bohrkern_id=bohrkern_id)

        return queryset

    class Meta(BaseViewSet.Meta):
        model = Probe
