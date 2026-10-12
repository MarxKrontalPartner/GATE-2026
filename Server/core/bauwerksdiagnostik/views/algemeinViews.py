"""Views of the general Bauwerksdiagnostik models (Media, ...)."""

import mimetypes

from django.db import IntegrityError, transaction
from django.http import Http404, HttpResponse
from django.shortcuts import get_object_or_404
from rest_framework.decorators import action
from rest_framework.parsers import FormParser, JSONParser, MultiPartParser
from rest_framework.response import Response

from core.assetverwaltung.views.base_viewset import BaseViewSet
from core.bauwerksdiagnostik.models.algemein import Material, Media
from core.bauwerksdiagnostik.serializer.algemeinSerializer import (
    DocumentMediaSerializer,
    ImageMediaSerializer,
    MaterialSerializer,
    MediaSerializer,
    MessdateiMediaSerializer,
)


class MaterialViewSet(BaseViewSet):
    serializer_class = MaterialSerializer
    parser_classes = [MultiPartParser, FormParser, JSONParser]

    class Meta(BaseViewSet.Meta):
        model = Material


def _concrete_media(media: Media):
    """Return the concrete media subtype instance (or the plain Media)."""
    if hasattr(media, "imagemedia"):
        return media.imagemedia
    elif hasattr(media, "documentmedia"):
        return media.documentmedia
    elif hasattr(media, "messdateimedia"):
        return media.messdateimedia
    return media


class MediaViewSet(BaseViewSet):
    serializer_class = MediaSerializer  # Default serializer
    parser_classes = [MultiPartParser, FormParser, JSONParser]

    class Meta(BaseViewSet.Meta):
        model = Media

    def get_serializer_class(self):
        """Return the appropriate serializer class based on media_type or instance type."""
        if self.action in ["create", "update", "partial_update"]:
            # For write operations, use media_type from request data
            media_type = self.request.data.get("media_type", "media")
            if media_type == "image":
                return ImageMediaSerializer
            elif media_type == "document":
                return DocumentMediaSerializer
            elif media_type == "messdatei":
                return MessdateiMediaSerializer
        elif self.action == "retrieve" and hasattr(self, "get_object"):
            # For read operations, determine by instance type
            try:
                media = self.get_object()
                if hasattr(media, "imagemedia"):
                    return ImageMediaSerializer
                elif hasattr(media, "documentmedia"):
                    return DocumentMediaSerializer
                elif hasattr(media, "messdateimedia"):
                    return MessdateiMediaSerializer
            except Exception as err:
                raise Exception(
                    "Media instance not found or invalid type (check get_serializer_class implementation)"
                ) from err

        return MediaSerializer

    def create(self, request, *args, **kwargs):
        serializer = self.get_serializer(data=request.data)
        serializer.is_valid(raise_exception=True)

        try:
            with transaction.atomic():
                serializer.save()
                return Response(serializer.data, status=201)
        except IntegrityError as e:
            if "duplicate key" in str(e).lower():
                return Response(
                    {"error": "Media record already exists. Please try again."},
                    status=409,
                )
            raise

    def partial_update(self, request, pk=None, *args, **kwargs):
        media = get_object_or_404(Media, pk=pk)

        # Determine the correct instance to update based on existing type
        instance = _concrete_media(media)

        serializer = self.get_serializer(instance, data=request.data, partial=True)
        serializer.is_valid(raise_exception=True)
        serializer.save()

        # Return the updated instance with the correct serializer
        response_serializer = self.get_serializer(instance)
        return Response(response_serializer.data, status=200)

    def retrieve(self, request, pk=None, *args, **kwargs):
        media = get_object_or_404(Media, pk=pk)

        # Determine the correct instance to serialize
        instance = _concrete_media(media)

        serializer = self.get_serializer(instance)
        return Response(serializer.data, status=200)

    def destroy(self, request, pk=None, *args, **kwargs):
        media = get_object_or_404(Media, pk=pk)
        media.datei.delete(
            save=False
        )  # Delete file first, don't save since we're deleting the object
        media.delete()  # Delete the database record
        return Response(status=204)

    # serve the media file content
    @action(detail=True, methods=["get"], url_path="content")
    def content(self, request, pk=None):
        if pk == "undefined":
            raise Http404("Media file not found, pk is undefined")
        media = get_object_or_404(Media, pk=pk)
        if not media.datei:
            raise Http404("Media file not found")

        try:
            # Read the file and close it right away. A streamed FileResponse keeps the
            # handle open, which on Windows blocks an immediately following DELETE.
            with media.datei.open("rb") as file:
                content = file.read()
        except OSError as err:
            raise Http404("Media file not found") from err

        content_type = mimetypes.guess_type(media.datei.name)[0]
        response = HttpResponse(
            content, content_type=content_type or "application/octet-stream"
        )
        response["Content-Disposition"] = (
            f'attachment; filename="{media.name}.{media.datei.name.split(".")[-1]}"'
        )
        response["Cache-Control"] = (
            "private, max-age=3600"  # Allow browser to cache for 1 hour
        )
        return response

    @action(detail=False, methods=["post"], url_path="batch")
    def batch(self, request):
        """Batch endpoint to retrieve multiple media items by their IDs."""
        ids = request.data.get("ids", [])

        if not ids:
            return Response({"error": "No IDs provided"}, status=400)

        if not isinstance(ids, list):
            return Response({"error": "IDs must be provided as a list"}, status=400)

        # Fetch media objects by IDs
        media_objects = Media.objects.filter(id__in=ids)

        # Create response data with enhanced information
        results = []
        for media in media_objects:
            # Determine the correct instance and serializer
            if hasattr(media, "imagemedia"):
                instance = media.imagemedia
                serializer = ImageMediaSerializer(instance)
                media_type = "image"
                preview_url = instance.datei.url if instance.datei else None
            elif hasattr(media, "documentmedia"):
                instance = media.documentmedia
                serializer = DocumentMediaSerializer(instance)
                media_type = "document"
                preview_url = (
                    instance.preview_image.url if instance.preview_image else None
                )
            elif hasattr(media, "messdateimedia"):
                instance = media.messdateimedia
                serializer = MessdateiMediaSerializer(instance)
                media_type = "messdatei"
                preview_url = instance.datei.url if instance.datei else None
            else:
                instance = media
                serializer = MediaSerializer(instance)
                media_type = "media"
                preview_url = media.datei.url if media.datei else None

            # Get serialized data
            media_data = serializer.data

            # Add additional fields that the frontend expects
            media_data.update(
                {
                    "media_type": media_type,
                    "preview_url": preview_url,
                    "tags": [],
                    "hochgeladen_am": media.hochgeladen_am,
                    "hochgeladen_von": media.hochgeladen_von,
                }
            )

            results.append(media_data)

        return Response(results)
