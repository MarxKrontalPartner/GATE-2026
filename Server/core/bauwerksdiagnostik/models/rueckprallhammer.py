"""Rückprallhammer."""

from django.db import models

from .algemein import Media
from .untersuchung import VorOrtUntersuchung


class Rueckprallhammer(VorOrtUntersuchung):
    class Hammertyp(models.TextChoices):
        N = "N", "Typ N"
        L = "L", "Typ L"
        PENDEL = "pendel", "Pendelhammer"

    class Schlagrichtung(models.TextChoices):
        HORIZONTAL = "horizontal", "Horizontal"
        OBEN = "oben", "Senkrecht nach oben"
        UNTEN = "unten", "Senkrecht nach unten"

    class Meta(VorOrtUntersuchung.Meta):
        verbose_name = "Rückprallhammer"
        verbose_name_plural = "Rückprallhammer"

    hammertyp = models.CharField(max_length=20, blank=True, choices=Hammertyp.choices)
    schlagrichtung = models.CharField(
        max_length=20, blank=True, choices=Schlagrichtung.choices
    )
    oberflaecheGeschliffen = models.BooleanField(blank=True, null=True)
    kalibriertAm = models.DateField(blank=True, null=True)
    # List of rebound values, e.g. [41, 43, 40]
    messreihe = models.JSONField(blank=True, null=True)
    medianRueckprallwert = models.FloatField(blank=True, null=True)
    messprotokolle = models.ManyToManyField(
        Media, blank=True, related_name="rueckprallhammer_messprotokolle"
    )

    def __str__(self):
        return f"Rueckprallhammer {str(self.id)}"
