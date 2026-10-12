"""Models of the Bauwerksverwaltung (Bauwerk, Projekt, Person, Firma, ...)."""

from django.db import models

from core.assetverwaltung.models.assetverwaltung_models import BaseModel
from core.bauwerksdiagnostik.models.algemein import Bauweise


class Firma(BaseModel):
    bezeichnung = models.CharField(max_length=100)

    def __str__(self):
        return self.bezeichnung


class Person(BaseModel):
    class Meta(BaseModel.Meta):
        ordering = []
        verbose_name = "Person"
        verbose_name_plural = "Personen"

    class Stellenbezeichnung(models.TextChoices):
        BEARBEITER = "Bearbeiter*in"
        PROJEKTLEITER = "Projektleiter*in"
        TECHNIKER = "Techniker*in"
        INGENIEUR = "Ingenieur*in"
        GESCHAEFTSFUEHRER = "Geschäftsleiter*in"
        PRAKTIKANTAZUBI = "Praktikant*in / Azubi"
        EXTERN = "Extern"
        IT = "IT"

    name = models.CharField(max_length=100)
    vorname = models.CharField(max_length=100)
    stellenbezeichnung = models.CharField(
        max_length=100, choices=Stellenbezeichnung.choices
    )
    firma = models.ForeignKey(Firma, on_delete=models.SET_NULL, blank=True, null=True)
    email = models.EmailField(blank=True)
    telefon = models.CharField(max_length=100, blank=True)

    def __str__(self):
        return f"{self.vorname} {self.name}"


class Projekt(BaseModel):
    class Meta(BaseModel.Meta):
        ordering = []
        verbose_name = "Projekt"
        verbose_name_plural = "Projekte"

    projektnummer = models.CharField(max_length=100, blank=True)
    name = models.CharField(max_length=100, blank=True)
    bezeichnung = models.CharField(max_length=100, blank=True)  # projektnummer + name
    start = models.DateField(blank=True, null=True)
    ende = models.DateField(blank=True, null=True)

    def save(self, *args, **kwargs) -> None:
        self.bezeichnung = f"{self.projektnummer} {self.name}"
        super().save(*args, **kwargs)

    def __str__(self):
        return self.bezeichnung


class Bauwerk(BaseModel):
    class Meta(BaseModel.Meta):
        ordering = []
        verbose_name = "Bauwerk"
        verbose_name_plural = "Bauwerke"

    projekt = models.ManyToManyField(Projekt, blank=True)
    bezeichnung = models.CharField(max_length=100, blank=True, unique=True)
    archived = models.BooleanField(default=False)
    beschreibung = models.TextField(blank=True)
    strasse = models.CharField(max_length=100, blank=True)

    # bwd fields
    bauweise = models.CharField(max_length=200, blank=True, choices=Bauweise.choices)
    bauwerksnummer = models.CharField(max_length=100, blank=True)
    bauwerkstyp = models.CharField(
        max_length=100, blank=True, null=True
    )  # todo: make choices
    bemerkung = models.TextField(blank=True, null=True)

    @staticmethod
    def display_field_name():
        return "bezeichnung"

    def __str__(self):
        return self.bezeichnung
