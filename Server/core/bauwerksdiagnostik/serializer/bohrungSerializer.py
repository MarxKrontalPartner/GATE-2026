from rest_framework import serializers

from core.bauwerksdiagnostik.models.baustoffeigenschaften import Probe
from core.bauwerksdiagnostik.models.bohrung import (
    Bohrkern,
    Kernbohrung,
)

# Kernbohrungs Serializer


class BohrkernSerializer(serializers.ModelSerializer):
    class Meta:
        model = Bohrkern
        fields = "__all__"


class NestedProbeSerializer(serializers.ModelSerializer):
    class Meta:
        model = Probe
        fields = "__all__"


class BohrkernDetailSerializer(serializers.ModelSerializer):
    """Bohrkern with nested proben for the BohrkernCard component."""

    proben = serializers.SerializerMethodField()

    class Meta:
        model = Bohrkern
        fields = "__all__"

    def get_proben(self, obj):
        proben = Probe.objects.filter(bohrkern=obj).order_by("entnahmetiefeVon")
        return NestedProbeSerializer(proben, many=True).data


class KernbohrungSerializer(serializers.ModelSerializer):
    class Meta:
        model = Kernbohrung
        fields = "__all__"
