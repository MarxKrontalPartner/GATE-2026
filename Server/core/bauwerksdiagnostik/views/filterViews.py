"""Filter views of the Bauwerksdiagnostik.

Untersuchungsstellen are filtered via the direct ``bauwerk`` relation.
"""

from django.shortcuts import get_object_or_404
from rest_framework import serializers, views
from rest_framework.response import Response

from core.assetverwaltung.models.bauwerksverwaltung_models import (
    Bauwerk,
    Person,
    Projekt,
)
from core.assetverwaltung.serializers.bauwerksverwaltung import (
    PersonSerializer,
    ProjektSerializer,
)
from core.bauwerksdiagnostik.models.projektspezifisch import Untersuchungsstelle
from core.bauwerksdiagnostik.serializer.projektspezifischSerializer import (
    UntersuchungsstelleSerializer,
)


class UntersuchungsstelleByBauwerkView(views.APIView):
    def get_queryset(self):
        bauwerk_id = self.request.query_params.get("bauwerk_id")
        bauwerk = get_object_or_404(Bauwerk, id=bauwerk_id)
        return Untersuchungsstelle.objects.filter(bauwerk=bauwerk).distinct()

    def get(self, request, *args, **kwargs):
        queryset = self.get_queryset()
        serializer = UntersuchungsstelleSerializer(queryset, many=True)
        return Response(serializer.data)


class BearbeiterByIDsView(views.APIView):
    def get_queryset(self):
        bearbeiter_ids = self.request.query_params.get("bearbeiter_ids")
        bearbeiter_ids = bearbeiter_ids.split(",")
        return Person.objects.filter(id__in=bearbeiter_ids)

    def get(self, request, *args, **kwargs):
        queryset = self.get_queryset()
        serializer = PersonSerializer(queryset, many=True)
        return Response(serializer.data)


class ProjektByBauwerkView(views.APIView):
    """
    retrieves projects associated with a specific Bauwerk
    """

    def get_queryset(self, bauwerk_id):
        # check if bauwerk_id is provided and a number
        if not bauwerk_id:
            raise serializers.ValidationError("bauwerk_id is required")
        bauwerk = get_object_or_404(Bauwerk, id=bauwerk_id)
        return Projekt.objects.filter(bauwerk=bauwerk)

    def get(self, request, bauwerk_id):
        queryset = self.get_queryset(bauwerk_id)
        serializer = ProjektSerializer(queryset, many=True)
        return Response(serializer.data)
