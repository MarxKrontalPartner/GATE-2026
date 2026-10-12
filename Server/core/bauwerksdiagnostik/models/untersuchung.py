"""Abstract Untersuchungsverfahren and its base classes."""

from django.db import models
from django.utils import timezone

from core.assetverwaltung.models.assetverwaltung_models import BaseModel
from core.assetverwaltung.models.bauwerksverwaltung_models import (
    Bauwerk,
    Person,
    Projekt,
)

from .algemein import Media


def _all_concrete_subclasses(cls):
    subclasses = []
    for sub in cls.__subclasses__():
        subclasses.extend(_all_concrete_subclasses(sub))
        if not sub._meta.abstract:
            subclasses.append(sub)
    return subclasses


# Base model for Untersuchungsverfahren like Rueckprallhammer, Ultraschall etc.
class Untersuchungsverfahren(BaseModel):
    class Meta(BaseModel.Meta):
        ordering = []
        abstract = True
        verbose_name = "Untersuchungsverfahren"
        verbose_name_plural = "Untersuchungsverfahren"

    datum = models.DateField(default=timezone.now)
    zeit = models.TimeField(blank=True, null=True)
    titelBild = models.ForeignKey(
        Media,
        blank=True,
        null=True,
        on_delete=models.SET_NULL,
        related_name="%(class)s_titelbild",
    )
    bilder = models.ManyToManyField(Media, blank=True, related_name="%(class)s_Bilder")
    bemerkung = models.TextField(blank=True)
    lufttemperatur = models.FloatField(blank=True, null=True)
    luftfeuchte = models.FloatField(blank=True, null=True)
    bearbeiter = models.ManyToManyField(Person, blank=True, related_name="%(class)s")
    abgeschlossen = models.BooleanField(default=False, blank=True, null=True)
    untersuchungsstelle = models.ForeignKey(
        "Untersuchungsstelle",
        on_delete=models.CASCADE,
        blank=True,
        null=True,
        related_name="%(class)s",
    )
    projekt = models.ForeignKey(
        Projekt, on_delete=models.SET_NULL, related_name="%(class)s", null=True
    )
    bauwerk = models.ForeignKey(
        Bauwerk, on_delete=models.CASCADE, related_name="%(class)s", null=True
    )

    sort_order = models.PositiveIntegerField(default=0, db_index=True)

    def _get_next_sort_order(self):
        max_sort_order = 0
        for model in _all_concrete_subclasses(Untersuchungsverfahren):
            queryset = model.objects.filter(
                untersuchungsstelle=self.untersuchungsstelle,
                projekt=self.projekt,
            )
            if self.pk and model is self.__class__:
                queryset = queryset.exclude(pk=self.pk)

            model_max = (
                queryset.aggregate(max_sort_order=models.Max("sort_order"))[
                    "max_sort_order"
                ]
                or 0
            )
            max_sort_order = max(max_sort_order, model_max)

        return max_sort_order + 1

    def save(self, *args, **kwargs):
        # Track old Untersuchungsstelle for updates
        old_untersuchungsstelle = None
        old_projekt = None
        old_sort_order = None
        if self.pk:  # This is an update, not a new instance
            try:
                old_instance = self.__class__.objects.get(pk=self.pk)
                old_untersuchungsstelle = old_instance.untersuchungsstelle
                old_projekt = old_instance.projekt
                old_sort_order = old_instance.sort_order
            except self.__class__.DoesNotExist:
                pass

        target_changed = (
            not self.pk
            or old_untersuchungsstelle != self.untersuchungsstelle
            or old_projekt != self.projekt
        )
        if (
            self.untersuchungsstelle
            and target_changed
            and (self.sort_order <= 0 or self.sort_order == old_sort_order)
        ):
            self.sort_order = self._get_next_sort_order()

        super().save(*args, **kwargs)


class VorOrtUntersuchung(Untersuchungsverfahren):
    class Meta(Untersuchungsverfahren.Meta):
        abstract = True
        verbose_name = "Vor-Ort-Untersuchung"
        verbose_name_plural = "Vor-Ort-Untersuchungen"


class Bohrung(VorOrtUntersuchung):
    """Abstract base for drilling methods."""

    class Kuehlung(models.TextChoices):
        NASS = "nass", "Nassbohrung"
        TROCKEN = "trocken", "Trockenbohrung"

    class Meta(VorOrtUntersuchung.Meta):
        abstract = True
        verbose_name = "Bohrung"
        verbose_name_plural = "Bohrungen"

    durchmesser = models.FloatField(blank=True, null=True)  # mm
    bohrlaenge = models.FloatField(blank=True, null=True)  # mm
    neigungswinkel = models.FloatField(blank=True, null=True)  # ° zur Horizontalen
    kuehlung = models.CharField(max_length=20, blank=True, choices=Kuehlung.choices)
    verschlussmaterial = models.CharField(max_length=100, blank=True)
    projekt = models.ForeignKey(
        Projekt,
        on_delete=models.SET_NULL,
        related_name="%(class)s_bohrungen",
        null=True,
    )
