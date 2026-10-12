from core.settings.extendable.base import *

DEBUG = True
# SECURITY WARNING: keep the secret key used in production secret!
SECRET_KEY = "django-insecure-local-startpunkt-only"

CORS_ALLOW_ALL_ORIGINS = True

ALLOWED_HOSTS = [
    "127.0.0.1",
    "localhost",
]
INSTALLED_APPS += [
    "django.contrib.staticfiles",
]
STATIC_URL = "static/"
STATIC_ROOT = BASE_DIR / "staticfiles"


DISABLE_DEFAULT_PERMISSION_CLASSES = ["rest_framework.permissions.IsAuthenticated"]
if default_perm_classes := REST_FRAMEWORK.get("DEFAULT_PERMISSION_CLASSES"):
    for permission_class in DISABLE_DEFAULT_PERMISSION_CLASSES:
        if permission_class in default_perm_classes:
            default_perm_classes.remove(permission_class)
