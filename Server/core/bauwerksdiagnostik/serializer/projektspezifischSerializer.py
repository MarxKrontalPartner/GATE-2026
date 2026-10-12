"""Serializers of the project-specific models (plain ModelSerializers)."""

from rest_framework import serializers

from core.bauwerksdiagnostik.models.projektspezifisch import Untersuchungsstelle


class UntersuchungsstelleSerializer(serializers.ModelSerializer):
    class Meta:
        model = Untersuchungsstelle
        fields = "__all__"
