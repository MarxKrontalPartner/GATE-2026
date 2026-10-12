from django.contrib import admin

from core.assetverwaltung.models.bauwerksverwaltung_models import (
    Bauwerk,
    Firma,
    Person,
    Projekt,
)

# Register your models here.
admin.site.register(Bauwerk)
admin.site.register(Projekt)
admin.site.register(Firma)
admin.site.register(Person)
