"""Project-specific models.

The structure is Bauwerk -> Untersuchungsstelle -> Untersuchungsverfahren.
Untersuchungsstelle references the Bauwerk directly (field ``bauwerk``).
Naming uses the plain ``name`` field.
"""

from django.db import models

from core.assetverwaltung.models.assetverwaltung_models import BaseModel
from core.assetverwaltung.models.bauwerksverwaltung_models import Bauwerk, Projekt

from .algemein import Media


# ==================================================================================================
# Untersuchungsstelle
# ==================================================================================================
class Untersuchungsstelle(BaseModel):
    class Meta(BaseModel.Meta):
        ordering = []
        verbose_name = "Untersuchungsstelle"
        verbose_name_plural = "Untersuchungsstellen"

    bauwerk = models.ForeignKey(
        Bauwerk,
        on_delete=models.CASCADE,
        related_name="untersuchungsstellen",
    )
    projekt = models.ForeignKey(
        Projekt,
        on_delete=models.SET_NULL,
        related_name="untersuchungsstellen",
        null=True,
        blank=True,
    )
    name = models.CharField(max_length=100, blank=True)
    bemerkung = models.TextField(blank=True)
    lagenbezugHoehe = models.CharField(max_length=100, blank=True, null=True)
    hoehe = models.FloatField(blank=True, null=True)
    lagenbezugAbstand1 = models.CharField(max_length=100, blank=True, null=True)
    abstand1 = models.FloatField(blank=True, null=True)
    lagenbezugAbstand2 = models.CharField(max_length=100, blank=True, null=True)
    abstand2 = models.FloatField(blank=True, null=True)

    media = models.ManyToManyField(Media, blank=True)

    def __str__(self):
        return self.name or f"Untersuchungsstelle {self.pk}"
