"""Kernbohrung and Bohrkern."""

from django.db import models

from core.assetverwaltung.models.assetverwaltung_models import BaseModel
from core.bauwerksdiagnostik.models.algemein import Material, Media

from .untersuchung import Bohrung


class Kernbohrung(Bohrung):
    class Meta(Bohrung.Meta):
        verbose_name = "Kernbohrung"
        verbose_name_plural = "Kernbohrungen"

    bohrkern = models.OneToOneField(
        "Bohrkern", on_delete=models.SET_NULL, blank=True, null=True
    )
    # Items: {id, bezeichnung, tiefe_von, tiefe_bis, beschreibung}
    schichtaufbau = models.JSONField(blank=True, null=True)
    bewehrungAngeschnitten = models.BooleanField(blank=True, null=True)
    kernVollstaendig = models.BooleanField(blank=True, null=True)


class Bohrkern(BaseModel):
    class Meta(BaseModel.Meta):
        ordering = []
        verbose_name = "Bohrkern"
        verbose_name_plural = "Bohrkerne"

    name = models.CharField(max_length=100, blank=True)
    material = models.ManyToManyField(Material, blank=True, related_name="bohrkerne")
    anzahlProben = models.IntegerField(blank=True, null=True)
    fotos = models.ManyToManyField(Media, blank=True, related_name="%(class)s_Bilder")

    def __str__(self):
        return self.name or f"Bohrkern {self.pk}"
