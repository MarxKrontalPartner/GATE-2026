from rest_framework import serializers

from core.bauwerksdiagnostik.models.baustoffeigenschaften import (
    Probe,
)


class ProbeSerializer(serializers.ModelSerializer):
    class Meta:
        model = Probe
        fields = "__all__"
