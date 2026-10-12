from django.apps import AppConfig


class BauwerksdiagnostikConfig(AppConfig):
    default_auto_field = "django.db.models.BigAutoField"
    name = "core.bauwerksdiagnostik"

    def ready(self):  # pragma: no cover - import side effects only
        # Import signal handlers to ensure they are registered
        from . import signals
