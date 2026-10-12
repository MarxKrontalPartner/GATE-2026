"""MediaUploadViewSet. Supported structure types: Bauwerk and Untersuchungsstelle."""

from django.shortcuts import get_object_or_404
from rest_framework import status
from rest_framework.decorators import action
from rest_framework.parsers import FormParser, JSONParser, MultiPartParser
from rest_framework.response import Response

from core.assetverwaltung.models.bauwerksverwaltung_models import Bauwerk
from core.assetverwaltung.views.base_viewset import BaseViewSet
from core.bauwerksdiagnostik.models.algemein import (
    DocumentMedia,
    ImageMedia,
    Media,
)
from core.bauwerksdiagnostik.models.projektspezifisch import Untersuchungsstelle

# ======================================================================================
# Module-level configuration & helpers
# ======================================================================================

STRUCTURE_MODELS = {
    "bauwerk": Bauwerk,
    "untersuchungsstelle": Untersuchungsstelle,
}


def _resolve_media_class(media_type: str):
    """
    Map a loose media_type string to the concrete Media subclass.
    """
    if not media_type:
        return Media

    media_type = media_type.lower()
    if media_type in {"image", "photo", "bild", "foto"}:
        return ImageMedia
    if media_type in {"document", "dokument", "pdf"}:
        return DocumentMedia
    return Media


# ======================================================================================
# MediaUploadViewSet
# ======================================================================================


class MediaUploadViewSet(BaseViewSet):
    parser_classes = [MultiPartParser, FormParser, JSONParser]

    class Meta(BaseViewSet.Meta):
        model = Media

    def _get_structure_model(self, structure_type):
        model_class = STRUCTURE_MODELS.get(str(structure_type).lower())
        if not model_class:
            raise ValueError(f"Invalid structure type: {structure_type}")
        return model_class

    def _get_bauwerk_for_upload(self, structure_type, structure_obj, bauwerk_id):
        """
        Figure out which Bauwerk a media item belongs to:

        * if we upload to a Bauwerk node, use that instance directly
        * otherwise, resolve bauwerk_id explicitly
        """
        if isinstance(structure_obj, Bauwerk):
            return structure_obj
        return get_object_or_404(Bauwerk, pk=bauwerk_id)

    def _attach(self, obj, media_item):
        if isinstance(obj, Bauwerk):
            # Bauwerk.media is the reverse side of Media.bauwerk (already set).
            return
        obj.media.add(media_item)

    def create(self, request):
        # Validate basic input
        structure_type = request.data.get("structure_type")
        structure_id = request.data.get("structure_id")
        media_type = request.data.get("media_type")
        file = request.FILES.get("file")
        bauwerk_id = request.data.get("bauwerk")

        if not all([structure_type, structure_id, media_type, file, bauwerk_id]):
            return Response(
                {"error": "Missing required fields"},
                status=status.HTTP_400_BAD_REQUEST,
            )

        # Optional name
        name = request.data.get("name") or getattr(file, "name", "")

        try:
            model_class = self._get_structure_model(structure_type)
        except ValueError as exc:
            return Response({"error": str(exc)}, status=status.HTTP_400_BAD_REQUEST)

        try:
            obj = model_class.objects.get(pk=structure_id)
        except model_class.DoesNotExist:
            return Response(
                {"error": f"{structure_type} with id {structure_id} not found"},
                status=status.HTTP_404_NOT_FOUND,
            )

        bauwerk_obj = self._get_bauwerk_for_upload(structure_type, obj, bauwerk_id)
        media_class = _resolve_media_class(media_type)

        media_item = media_class(
            name=name,
            datei=file,
            model_name=str(structure_type).lower(),
            model_pk=int(structure_id),
            hochgeladen_von=(
                request.user.username if request.user.is_authenticated else "Anonymous"
            ),
            bauwerk=bauwerk_obj,
        )
        media_item.save()

        # Associate with related object
        self._attach(obj, media_item)

        return Response(
            {
                "message": "Media uploaded successfully",
                "media_item": {
                    "id": media_item.id,
                    "name": media_item.name,
                    "media_type": media_type,
                    "file_url": media_item.datei.url if media_item.datei else None,
                    "created_at": media_item.hochgeladen_am,
                },
            }
        )

    @action(detail=False, methods=["post"], url_path="batch-upload")
    def batch_upload(self, request):
        # Validate common input
        structure_type = request.data.get("structure_type")
        structure_id = request.data.get("structure_id")
        media_type = request.data.get("media_type")
        bauwerk_id = request.data.get("bauwerk")

        files = request.FILES.getlist("files") or request.FILES.getlist("file")

        if not all([structure_type, structure_id, media_type, bauwerk_id]):
            return Response(
                {
                    "error": "Missing required fields (structure_type, structure_id, media_type, bauwerk)"
                },
                status=status.HTTP_400_BAD_REQUEST,
            )

        if not files:
            return Response(
                {"error": "No files provided"},
                status=status.HTTP_400_BAD_REQUEST,
            )

        try:
            model_class = self._get_structure_model(structure_type)
        except ValueError as exc:
            return Response({"error": str(exc)}, status=status.HTTP_400_BAD_REQUEST)

        try:
            obj = model_class.objects.get(pk=structure_id)
        except model_class.DoesNotExist:
            return Response(
                {"error": f"{structure_type} with id {structure_id} not found"},
                status=status.HTTP_404_NOT_FOUND,
            )

        bauwerk_obj = self._get_bauwerk_for_upload(structure_type, obj, bauwerk_id)
        media_class = _resolve_media_class(media_type)

        created_media = []
        errors = []

        for file in files:
            # Use per-file name or fall back to filename
            name = request.data.get("name") or getattr(file, "name", "")

            media_item = media_class(
                name=name,
                datei=file,
                model_name=str(structure_type).lower(),
                model_pk=int(structure_id),
                hochgeladen_von=(
                    request.user.username
                    if request.user.is_authenticated
                    else "Anonymous"
                ),
                bauwerk=bauwerk_obj,
            )

            try:
                media_item.save()
                self._attach(obj, media_item)

                created_media.append(
                    {
                        "id": media_item.id,
                        "name": media_item.name,
                        "media_type": media_type,
                        "file_url": media_item.datei.url if media_item.datei else None,
                        "created_at": media_item.hochgeladen_am,
                    }
                )
            except Exception as e:
                errors.append({"file": getattr(file, "name", ""), "error": str(e)})

        status_code = (
            status.HTTP_201_CREATED
            if created_media
            else status.HTTP_500_INTERNAL_SERVER_ERROR
            if errors
            else status.HTTP_400_BAD_REQUEST
        )

        return Response(
            {
                "message": (
                    f"Processed {len(files)} files. "
                    f"Success: {len(created_media)}, Failed: {len(errors)}"
                ),
                "media_items": created_media,
                "errors": errors,
            },
            status=status_code,
        )
