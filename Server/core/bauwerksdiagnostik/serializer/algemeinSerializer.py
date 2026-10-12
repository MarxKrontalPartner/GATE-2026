"""Serializers of the general Bauwerksdiagnostik models (Media types, ...)."""

from rest_framework import serializers

from core.bauwerksdiagnostik.models.algemein import (
    DocumentMedia,
    ImageMedia,
    Material,
    Media,
    MessdateiMedia,
)


class MaterialSerializer(serializers.ModelSerializer):
    class Meta:
        model = Material
        fields = "__all__"


class MediaSerializer(serializers.ModelSerializer):
    media_type = serializers.SerializerMethodField()

    class Meta:
        model = Media
        fields = "__all__"

    def get_media_type(self, obj):
        if isinstance(obj, ImageMedia) or hasattr(obj, "imagemedia"):
            return "image"
        elif isinstance(obj, DocumentMedia) or hasattr(obj, "documentmedia"):
            return "document"
        elif isinstance(obj, MessdateiMedia) or hasattr(obj, "messdateimedia"):
            return "messdatei"
        return "media"


class ImageMediaSerializer(MediaSerializer):
    class Meta(MediaSerializer.Meta):
        model = ImageMedia
        fields = "__all__"


class DocumentMediaSerializer(MediaSerializer):
    class Meta(MediaSerializer.Meta):
        model = DocumentMedia
        fields = "__all__"


class MessdateiMediaSerializer(MediaSerializer):
    class Meta(MediaSerializer.Meta):
        model = MessdateiMedia
        fields = "__all__"
