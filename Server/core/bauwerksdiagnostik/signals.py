"""Signals for media file lifecycle management.

Clean up file storage when Media objects are replaced or deleted.
"""

from __future__ import annotations

from django.db.models.signals import post_delete, pre_save
from django.dispatch import receiver

from .models.algemein import Media


@receiver(post_delete, sender=Media)
def delete_media_file(
    sender, instance: Media, **kwargs
):  # pragma: no cover - side-effect
    file_field = instance.datei
    if file_field:
        try:
            file_field.delete(save=False)
        except Exception:
            pass


@receiver(pre_save, sender=Media)
def cleanup_replaced_media_file(sender, instance: Media, **kwargs):  # pragma: no cover
    if not instance.pk:
        return
    try:
        old = Media.objects.only("datei").get(pk=instance.pk)
    except Media.DoesNotExist:
        return
    old_file = old.datei
    if old_file and instance.datei and old_file.name != instance.datei.name:
        try:
            old_file.delete(save=False)
        except Exception:
            pass
