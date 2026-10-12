"""Seed data for the Bauwerksdiagnostik.

Creates one fictitious Bauwerk with Projekt, Bearbeiter, Untersuchungsstelle and
one Untersuchungsverfahren per implemented method. Idempotent via SEED_MARKER.
"""

from __future__ import annotations

from datetime import date
from typing import Any

from django.core.management.base import BaseCommand
from django.db import transaction

from core.assetverwaltung.models.bauwerksverwaltung_models import (
    Bauwerk,
    Person,
    Projekt,
)
from core.bauwerksdiagnostik.models.algemein import Material
from core.bauwerksdiagnostik.models.baustoffeigenschaften import Probe
from core.bauwerksdiagnostik.models.bohrung import Bohrkern
from core.bauwerksdiagnostik.models.projektspezifisch import Untersuchungsstelle
from core.bauwerksdiagnostik.views.untersuchungsverfahren_categories import (
    get_all_category_models,
)

SEED_MARKER = "mkp-bwd-seed-v1"
PROJECT_NAME = "Offline-Startpunkt Beispielbrücke"
PROJECT_NUMBER = "TEST-BWD-OFFLINE"
STRUCTURE_NAME = "DEMO_Beispielbruecke"
SEED_DATE = date(2026, 10, 8)

PROCEDURE_DEFAULTS: dict[str, dict[str, Any]] = {
    "Kernbohrung": {
        "durchmesser": 100.0,
        "bohrlaenge": 300.0,
        "neigungswinkel": 0.0,
        "kuehlung": "nass",
        "verschlussmaterial": "PCC-Mörtel",
        "bewehrungAngeschnitten": False,
        "kernVollstaendig": True,
        # Same item format as the SchichtenCard of the frontend.
        "schichtaufbau": [
            {
                "id": "schicht-1",
                "bezeichnung": "Mörtel",
                "tiefe_von": 0,
                "tiefe_bis": 20,
                "beschreibung": "",
            },
            {
                "id": "schicht-2",
                "bezeichnung": "Beton",
                "tiefe_von": 20,
                "tiefe_bis": 300,
                "beschreibung": "",
            },
        ],
    },
    "Chloridanalyse": {
        "pruefstelle": "Seed-Labor",
        "analyseverfahren": "titration",
        "chloridgehaltZement": 0.35,
        "zementgehaltAnnahme": 300.0,
        "grenzwertUeberschritten": False,
    },
    "Rissbreitenmessung": {
        "messmittel": "risslupe",
        "risse": [
            {
                "id": "riss-1",
                "bezeichnung": "R1",
                "breite": 0.2,
                "laenge": 450.0,
                "verlauf": "quer",
                "wasserfuehrend": False,
            },
            {
                "id": "riss-2",
                "bezeichnung": "R2",
                "breite": 0.35,
                "laenge": 120.0,
                "verlauf": "schraeg",
                "wasserfuehrend": True,
            },
        ],
        "maxRissbreite": 0.35,
        "monitoringEmpfohlen": True,
    },
    "Rueckprallhammer": {
        "hammertyp": "N",
        "schlagrichtung": "horizontal",
        "oberflaecheGeschliffen": True,
        "messreihe": [40.0, 41.0, 39.0, 42.0, 40.0],
        "medianRueckprallwert": 40.0,
    },
    "Druckfestigkeitspruefung": {
        "pruefalter": 28,
        "lagerung": "wasser",
        "bruchlast": 355.0,
        "druckfestigkeit": 45.2,
        "bruchbild": "regelmaessig",
    },
}


class Command(BaseCommand):
    help = "Legt fiktive Bauwerksdiagnostik-Demodaten an (idempotent)."

    @transaction.atomic
    def handle(self, *args, **options):
        if Bauwerk.objects.filter(bezeichnung=STRUCTURE_NAME).exists():
            self.stdout.write("Demodaten existieren bereits.")
            return

        projekt = Projekt.objects.create(
            projektnummer=PROJECT_NUMBER, name=PROJECT_NAME, start=SEED_DATE
        )
        bauwerk = Bauwerk.objects.create(
            bezeichnung=STRUCTURE_NAME,
            bauwerksnummer="DEMO-01",
            bauweise="Stahlbeton",
            beschreibung=f"Fiktives Bauwerk ({SEED_MARKER})",
        )
        bauwerk.projekt.add(projekt)
        bearbeiter = Person.objects.create(
            name="Muster",
            vorname="Max",
            stellenbezeichnung=Person.Stellenbezeichnung.BEARBEITER,
        )
        beton = Material.objects.create(name="Beton")
        untersuchungsstelle = Untersuchungsstelle.objects.create(
            bauwerk=bauwerk,
            projekt=projekt,
            name="Widerlager Ost",
        )
        bohrkern = Bohrkern.objects.create(name="BK-01", anzahlProben=2)
        bohrkern.material.add(beton)
        probe = Probe.objects.create(
            probenbezeichnung="BK-01 / P-01",
            entnahmetiefeVon=0,
            entnahmetiefeBis=50,
            bohrkern=bohrkern,
            bauwerk_id=bauwerk,
            projekt_id=projekt,
        )

        for model in get_all_category_models():
            values = dict(PROCEDURE_DEFAULTS.get(model.__name__, {}))
            field_names = {field.name for field in model._meta.fields}
            if "bohrkern" in field_names:
                values["bohrkern"] = bohrkern
            if "probe" in field_names:
                values["probe"] = probe
            instance = model.objects.create(
                bauwerk=bauwerk,
                projekt=projekt,
                untersuchungsstelle=untersuchungsstelle,
                datum=SEED_DATE,
                bemerkung=SEED_MARKER,
                **values,
            )
            instance.bearbeiter.add(bearbeiter)

        self.stdout.write(self.style.SUCCESS("Demodaten angelegt."))
