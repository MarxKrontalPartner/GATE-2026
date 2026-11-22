from rest_framework import serializers

from core.assetverwaltung.serializers.base import BaseViewSerializer
from core.bauwerksdiagnostik.models.baustoffeigenschaften import (
    Chloridanalyse,
    Druckfestigkeitspruefung,
    Probe,
)


class ChloridanalyseSerializer(BaseViewSerializer):
    class Meta:
        model = Chloridanalyse
        fields = "__all__"


class DruckfestigkeitspruefungSerializer(BaseViewSerializer):
    class Meta:
        model = Druckfestigkeitspruefung
        fields = "__all__"


class ProbeSerializer(serializers.ModelSerializer):
    class Meta:
        model = Probe
        fields = "__all__"
