from core.assetverwaltung.views.base_viewset import BaseViewSet
from core.bauwerksdiagnostik.models.rueckprallhammer import Rueckprallhammer
from core.bauwerksdiagnostik.serializer.rueckprallHammerSerializer import (
    RueckprallhammerSerializer,
)


class RueckprallhammerViewSet(BaseViewSet):
    serializer_class = RueckprallhammerSerializer

    class Meta(BaseViewSet.Meta):
        model = Rueckprallhammer

    def get_queryset(self):
        queryset = Rueckprallhammer.objects.all()
        project_id = self.request.query_params.get("projekt_id")

        if project_id:
            queryset = queryset.filter(projekt=project_id)

        return queryset
