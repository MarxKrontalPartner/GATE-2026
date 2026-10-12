from django.apps import apps
from django.contrib import admin
from django.contrib.admin.sites import AlreadyRegistered

from core.bauwerksdiagnostik.models.baustoffeigenschaften import (
    Chloridanalyse,
    Probe,
)
from core.bauwerksdiagnostik.models.bohrung import Kernbohrung


# Function to register all models
def register_all_models(app_name):
    app_config = apps.get_app_config(app_name)
    for model in app_config.get_models():
        try:
            admin.site.register(model)
        except AlreadyRegistered:
            pass  # Ignore already registered models


admin.site.register(Kernbohrung)
admin.site.register(Chloridanalyse)
admin.site.register(Probe)


# Call the function to register all models
register_all_models("bauwerksdiagnostik")
