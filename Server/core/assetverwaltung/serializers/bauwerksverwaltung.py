from rest_framework import serializers

from core.assetverwaltung.models.bauwerksverwaltung_models import (
    Bauwerk,
    Firma,
    Person,
    Projekt,
)
from core.assetverwaltung.serializers.mixins import BulkUpdateListSerializerMixin


class BauwerkSerializer(serializers.ModelSerializer):
    class Meta:
        model = Bauwerk
        fields = "__all__"


class ProjektSerializer(serializers.ModelSerializer):
    class Meta:
        model = Projekt
        fields = "__all__"


class PersonSerializer(serializers.ModelSerializer):
    class Meta:
        model = Person
        fields = "__all__"


class FirmaSerializer(serializers.ModelSerializer):
    class Meta:
        model = Firma
        fields = "__all__"


class BulkUpdateBauwerkSerializer(BauwerkSerializer):
    id = serializers.IntegerField()

    class Meta(BauwerkSerializer.Meta):
        list_serializer_class = BulkUpdateListSerializerMixin


class BulkUpdateProjektSerializer(ProjektSerializer):
    id = serializers.IntegerField()

    class Meta(ProjektSerializer.Meta):
        list_serializer_class = BulkUpdateListSerializerMixin


class BulkUpdatePersonSerializer(PersonSerializer):
    id = serializers.IntegerField()

    class Meta(PersonSerializer.Meta):
        list_serializer_class = BulkUpdateListSerializerMixin


class BulkUpdateFirmaSerializer(FirmaSerializer):
    id = serializers.IntegerField()

    class Meta(FirmaSerializer.Meta):
        list_serializer_class = BulkUpdateListSerializerMixin
