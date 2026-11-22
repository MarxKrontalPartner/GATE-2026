from core.assetverwaltung.views.base_viewset import BaseViewSet
from core.bauwerksdiagnostik.models.rissbreitenmessung import Rissbreitenmessung
from core.bauwerksdiagnostik.serializer.rissbreitenmessungSerializer import (
    RissbreitenmessungSerializer,
)


class RissbreitenmessungViewSet(BaseViewSet):
    serializer_class = RissbreitenmessungSerializer

    class Meta(BaseViewSet.Meta):
        model = Rissbreitenmessung

    def get_queryset(self):
        queryset = Rissbreitenmessung.objects.all()
        project_id = self.request.query_params.get("projekt_id")

        if project_id:
            queryset = queryset.filter(projekt=project_id)

        return queryset
