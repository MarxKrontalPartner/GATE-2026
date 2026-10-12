"""Probe and the material-related methods Chloridanalyse and Druckfestigkeitspruefung."""

from django.db import models

from core.assetverwaltung.models.assetverwaltung_models import BaseModel
from core.assetverwaltung.models.bauwerksverwaltung_models import Bauwerk, Projekt
from core.bauwerksdiagnostik.models.algemein import Media
from core.bauwerksdiagnostik.models.untersuchung import (
    Untersuchungsverfahren,
    VorOrtUntersuchung,
)

from .bohrung import Bohrkern


# =============================================================================
# BASE CLASSES
# =============================================================================
class Laboruntersuchung(Untersuchungsverfahren):
    class Meta(Untersuchungsverfahren.Meta):
        verbose_name = "Laboruntersuchung"
        verbose_name_plural = "Laboruntersuchungen"
        abstract = True


class Probe(BaseModel):
    class Meta(BaseModel.Meta):
        ordering = []
        verbose_name = "Probe"
        verbose_name_plural = "Proben"

    probenbezeichnung = models.CharField(max_length=100)
    bemerkung = models.TextField(blank=True, null=True)
    foto = models.ManyToManyField(Media, blank=True)
    material = models.ManyToManyField("Material", blank=True, related_name="proben")
    probendurchmesser = models.FloatField(blank=True, null=True)
    probenlaenge = models.FloatField(blank=True, null=True)
    entnahmetiefeVon = models.FloatField(blank=True, null=True)
    entnahmetiefeBis = models.FloatField(blank=True, null=True)
    bohrkern = models.ForeignKey(
        Bohrkern, on_delete=models.SET_NULL, blank=True, null=True
    )

    bauwerk_id = models.ForeignKey(
        Bauwerk, on_delete=models.SET_NULL, blank=True, null=True
    )
    projekt_id = models.ForeignKey(
        Projekt, on_delete=models.SET_NULL, blank=True, null=True
    )

    def __str__(self):
        return self.probenbezeichnung


class Chloridanalyse(VorOrtUntersuchung):
    class Analyseverfahren(models.TextChoices):
        TITRATION = "titration", "Potentiometrische Titration"
        PHOTOMETRIE = "photometrie", "Photometrie"
        SCHNELLTEST = "schnelltest", "Schnelltest vor Ort"

    class Meta(VorOrtUntersuchung.Meta):
        verbose_name = "Chloridanalyse"
        verbose_name_plural = "Chloridanalysen"

    probe = models.ForeignKey(Probe, on_delete=models.SET_NULL, blank=True, null=True)
    pruefstelle = models.CharField(max_length=100, blank=True)
    analyseverfahren = models.CharField(
        max_length=20, blank=True, choices=Analyseverfahren.choices
    )
    chloridgehaltZement = models.FloatField(blank=True, null=True)  # M.-% bez. Zement
    zementgehaltAnnahme = models.FloatField(blank=True, null=True)  # kg/m³
    grenzwertUeberschritten = models.BooleanField(blank=True, null=True)
    pruefberichte = models.ManyToManyField(
        Media, blank=True, related_name="chloridanalyse_pruefberichte"
    )


class Druckfestigkeitspruefung(Laboruntersuchung):
    class Lagerung(models.TextChoices):
        WASSER = "wasser", "Wasserlagerung"
        LUFT = "luft", "Luftlagerung"
        TROCKEN = "trocken", "Trockenlagerung"

    class Bruchbild(models.TextChoices):
        REGELMAESSIG = "regelmaessig", "Regelmäßig"
        UNREGELMAESSIG = "unregelmaessig", "Unregelmäßig"

    class Meta(Laboruntersuchung.Meta):
        abstract = False
        verbose_name = "Druckfestigkeitsprüfung"
        verbose_name_plural = "Druckfestigkeitsprüfungen"

    probe = models.ForeignKey(Probe, on_delete=models.SET_NULL, blank=True, null=True)
    pruefalter = models.IntegerField(blank=True, null=True)  # Tage
    lagerung = models.CharField(max_length=20, blank=True, choices=Lagerung.choices)
    bruchlast = models.FloatField(blank=True, null=True)  # kN
    druckfestigkeit = models.FloatField(blank=True, null=True)  # N/mm²
    bruchbild = models.CharField(max_length=20, blank=True, choices=Bruchbild.choices)
    pruefbericht = models.ForeignKey(
        Media,
        on_delete=models.SET_NULL,
        blank=True,
        null=True,
        related_name="druckfestigkeitspruefung_pruefbericht",
    )
