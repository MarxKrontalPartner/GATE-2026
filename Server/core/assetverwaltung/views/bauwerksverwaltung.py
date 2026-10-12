"""ViewSets of the Bauwerksverwaltung."""

from rest_framework import viewsets

from core.assetverwaltung.models.bauwerksverwaltung_models import Bauwerk, Firma, Person
from core.assetverwaltung.serializers.bauwerksverwaltung import (
    BauwerkSerializer,
    BulkUpdateBauwerkSerializer,
    BulkUpdateFirmaSerializer,
    BulkUpdatePersonSerializer,
    BulkUpdateProjektSerializer,
    FirmaSerializer,
    PersonSerializer,
    ProjektSerializer,
)
from core.assetverwaltung.views.mixins import (
    BauwerkQuerySetMixin,
    BulkDeleteViewSetMixin,
    BulkPostViewSetMixin,
    BulkUpdateViewSetMixin,
)


class BauwerksverwaltungViewSet(
    BulkPostViewSetMixin,
    BulkUpdateViewSetMixin,
    BulkDeleteViewSetMixin,
    viewsets.ModelViewSet,
):
    """
    Base ViewSet for bauwerksverwaltung models.
    """


class BauwerkViewSet(BauwerksverwaltungViewSet):
    queryset = Bauwerk.objects.all()
    serializer_class = BauwerkSerializer
    bulk_update_serializer_class = BulkUpdateBauwerkSerializer


class FirmaViewSet(BauwerksverwaltungViewSet):
    queryset = Firma.objects.all()
    serializer_class = FirmaSerializer
    bulk_update_serializer_class = BulkUpdateFirmaSerializer


class PersonViewSet(BauwerksverwaltungViewSet):
    queryset = Person.objects.all()
    serializer_class = PersonSerializer
    bulk_update_serializer_class = BulkUpdatePersonSerializer


class ProjektViewSet(BauwerkQuerySetMixin, BauwerksverwaltungViewSet):
    queryset_bauwerk_path = "bauwerk__id"
    serializer_class = ProjektSerializer
    bulk_update_serializer_class = BulkUpdateProjektSerializer
