"""Rissbreitenmessung."""

from django.db import models

from .untersuchung import UNTERSUCHUNG_EDITABLE, VorOrtUntersuchung


class Rissbreitenmessung(VorOrtUntersuchung):
    class Messmittel(models.TextChoices):
        RISSLUPE = "risslupe", "Risslupe"
        RISSKARTE = "risskarte", "Rissbreitenkarte"
        MIKROSKOP = "mikroskop", "Digitales Mikroskop"

    class Meta(VorOrtUntersuchung.Meta):
        verbose_name = "Rissbreitenmessung"
        verbose_name_plural = "Rissbreitenmessungen"

    messmittel = models.CharField(max_length=20, blank=True, choices=Messmittel.choices)
    # Items: {id, bezeichnung, breite (mm), laenge (mm),
    #         verlauf ("laengs" | "quer" | "schraeg" | "netz"), wasserfuehrend}
    risse = models.JSONField(blank=True, null=True)
    maxRissbreite = models.FloatField(blank=True, null=True)  # mm
    monitoringEmpfohlen = models.BooleanField(blank=True, null=True)
    # Incremented on every successful submit of the sync endpoint
    version = models.PositiveIntegerField(default=1)

    # Fields that may be changed via the sync endpoint (no M2M fields)
    EDITABLE = UNTERSUCHUNG_EDITABLE + [
        "messmittel",
        "risse",
        "maxRissbreite",
        "monitoringEmpfohlen",
    ]

    def __str__(self):
        return f"Rissbreitenmessung {str(self.id)}"
