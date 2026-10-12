from rest_framework import serializers

from core.assetverwaltung.models.bauwerksverwaltung_models import Person

# Untersuchungsverfahren subclasses are abstract category bases.


class BearbeiterSerializer(serializers.ModelSerializer):
    class Meta:
        model = Person
        fields = "__all__"


class UntersuchungsverfahrenSerializer(serializers.Serializer):
    class Meta:
        model = None
        fields = "__all__"
