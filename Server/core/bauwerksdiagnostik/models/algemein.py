"""General models of the Bauwerksdiagnostik (Media types, Bauweise, ...).

Files are stored by Django's default FileSystemStorage below MEDIA_ROOT.
"""

from django.db import models
from PIL import Image
from PIL.ExifTags import TAGS

from core.assetverwaltung.models.assetverwaltung_models import BaseModel


class Material(BaseModel):
    class Meta(BaseModel.Meta):
        ordering = []
        verbose_name = "Material"
        verbose_name_plural = "Materialien"

    name = models.CharField(max_length=100)
    beschreibung = models.TextField(blank=True)

    def __str__(self):
        return self.name


# ==================================================================================================
# MKP choices
# ==================================================================================================
class Kompass(models.TextChoices):
    N = "N", "N"
    NO = "NO", "NO"
    EAST = "O", "O"
    SO = "SO", "SO"
    S = "S", "S"
    SW = "SW", "SW"
    W = "W", "W"
    NW = "NW", "NW"


class Bauweise(models.TextChoices):
    HOLZ = "Holz", "Holz"
    STAHL = "Stahl", "Stahl"
    STAHLBETON = "Stahlbeton", "Stahlbeton"
    STAHLVERBUND = "Stahlverbund", "Stahlverbund"
    NATURSTEIN = "Naturstein", "Naturstein"
    SPANNBETON = "Spannbeton", "Spannbeton"


def upload_to(instance, filename):
    if instance.model_name and instance.model_pk:
        return f"files/{instance.model_name}/{instance.model_pk}/{filename}"
    else:
        raise ValueError(
            "The instance has no model_name or model_pk. Please save the Model instance first before uploading a file"
        )


class Media(BaseModel):
    class Meta(BaseModel.Meta):
        ordering = []
        verbose_name = "Media"
        verbose_name_plural = "Media"

    name = models.CharField(max_length=100)
    beschreibung = models.TextField(blank=True)
    local_id = models.IntegerField(
        null=True, blank=True
    )  # for frontend use like order in gallery
    datei = models.FileField(upload_to=upload_to, blank=True)
    hochgeladen_von = models.CharField(max_length=100, blank=True)
    hochgeladen_am = models.DateTimeField(auto_now_add=True)
    zuletzt_geaendert_am = models.DateTimeField(auto_now=True)

    # the model and pk of the object the media is attached to, used to gennrate the file path
    model_name = models.CharField(max_length=100, blank=False)
    model_pk = models.IntegerField(blank=False, null=False)

    bauwerk = models.ForeignKey(
        "assetverwaltung.Bauwerk",
        on_delete=models.CASCADE,
        blank=True,
        null=True,
        related_name="media",
    )

    # File deletion handled via signals (post_delete) to ensure CASCADE + bulk deletes are covered.


class ImageMedia(Media):
    width = models.IntegerField(null=True, blank=True)
    height = models.IntegerField(null=True, blank=True)
    # image annotation data
    historie = models.JSONField(blank=True, null=True)
    drawing = models.JSONField(blank=True, null=True)
    exif = models.JSONField(blank=True, null=True)

    def save(self, *args, **kwargs):
        is_new = True
        if self.pk:
            try:
                old_instance = ImageMedia.objects.get(pk=self.pk)
                if self.datei == old_instance.datei:
                    is_new = False
            except ImageMedia.DoesNotExist:
                is_new = True

        # Process image data before saving if it's a new file
        if is_new and self.datei:
            try:
                self.datei.seek(0)  # Reset file pointer to start
                with Image.open(self.datei) as img:
                    # Get image dimensions
                    self.width, self.height = img.size
                    # Get EXIF data
                    exif_data = {}
                    raw_exif = img.getexif()
                    for tag_id, value in raw_exif.items():
                        tag = TAGS.get(tag_id, tag_id)
                        exif_data[str(tag)] = str(value)
                    self.exif = exif_data
                self.datei.seek(0)  # Reset file pointer again
            except Exception as e:
                print(f"Error reading image data: {e}")

        super().save(*args, **kwargs)


class DocumentMedia(Media):
    file_format = models.CharField(max_length=10, blank=True)
    preview = models.TextField(blank=True)
    preview_image = models.FileField(upload_to=upload_to, blank=True)

    # No preview is generated; preview and preview_image stay empty.


class MessdateiMedia(Media):
    measurement_data = models.JSONField(blank=True, null=True)
    # messdatei-specific fields
