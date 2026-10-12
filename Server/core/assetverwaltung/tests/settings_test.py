from core.settings.extendable.base import *

DEBUG = True
SECRET_KEY = "django-insecure-tests-only"

DATABASES = {
    "default": {
        "ENGINE": "django.db.backends.sqlite3",
        "NAME": ":memory:",
    }
}

DISABLE_DEFAULT_PERMISSION_CLASSES = ["rest_framework.permissions.IsAuthenticated"]
if default_perm_classes := REST_FRAMEWORK.get("DEFAULT_PERMISSION_CLASSES"):
    for permission_class in DISABLE_DEFAULT_PERMISSION_CLASSES:
        if permission_class in default_perm_classes:
            default_perm_classes.remove(permission_class)
