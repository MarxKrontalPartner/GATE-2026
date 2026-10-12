import io

import pytest
from django.core.files.uploadedfile import SimpleUploadedFile
from PIL import Image
from rest_framework.test import APIClient

from core.assetverwaltung.models.bauwerksverwaltung_models import Bauwerk, Projekt
from core.bauwerksdiagnostik.models.algemein import ImageMedia, Media
from core.bauwerksdiagnostik.models.projektspezifisch import Untersuchungsstelle


@pytest.fixture(autouse=True)
def media_root(settings, tmp_path):
    settings.MEDIA_ROOT = tmp_path
    return tmp_path


@pytest.fixture
def client():
    return APIClient()


@pytest.fixture
def bauwerk():
    projekt = Projekt.objects.create(projektnummer="P-1", name="Test")
    bauwerk = Bauwerk.objects.create(bezeichnung="BW-1")
    bauwerk.projekt.add(projekt)
    return bauwerk


def png_file(name="foto.png"):
    buffer = io.BytesIO()
    Image.new("RGB", (4, 3), "red").save(buffer, format="PNG")
    return SimpleUploadedFile(name, buffer.getvalue(), content_type="image/png")


@pytest.mark.django_db
def test_upload_image_media_stores_file_on_disk(client, bauwerk, media_root):
    response = client.post(
        "/api/bwd/Media/",
        {
            "name": "Foto",
            "datei": png_file(),
            "model_name": "kernbohrung",
            "model_pk": 7,
            "media_type": "image",
            "bauwerk": bauwerk.pk,
        },
        format="multipart",
    )

    assert response.status_code == 201, response.content
    data = response.json()
    assert data["media_type"] == "image"
    assert (data["width"], data["height"]) == (4, 3)

    media = ImageMedia.objects.get(pk=data["id"])
    path = media_root / media.datei.name
    assert media.datei.name.startswith("files/kernbohrung/7/")
    assert path.exists()

    content = client.get(f"/api/bwd/Media/{media.pk}/content/")
    assert content.status_code == 200
    assert content.content.startswith(b"\x89PNG")
    assert content["Content-Type"] == "image/png"

    detail = client.get(f"/api/bwd/Media/{media.pk}/")
    assert detail.json()["media_type"] == "image"

    batch = client.post("/api/bwd/Media/batch/", {"ids": [media.pk]}, format="json")
    assert batch.json()[0]["media_type"] == "image"

    deleted = client.delete(f"/api/bwd/Media/{media.pk}/")
    assert deleted.status_code == 204
    assert not path.exists()
    assert not Media.objects.exists()


@pytest.mark.django_db
def test_upload_messdatei(client, bauwerk):
    response = client.post(
        "/api/bwd/Media/",
        {
            "name": "Messung",
            "datei": SimpleUploadedFile("messung.csv", b"r;1\n"),
            "model_name": "rueckprallhammer",
            "model_pk": 1,
            "media_type": "messdatei",
            "bauwerk": bauwerk.pk,
        },
        format="multipart",
    )

    assert response.status_code == 201, response.content
    assert response.json()["media_type"] == "messdatei"


@pytest.mark.django_db
def test_upload_media_to_untersuchungsstelle(client, bauwerk):
    stelle = Untersuchungsstelle.objects.create(bauwerk=bauwerk, name="US-1")

    response = client.post(
        "/api/bwd/upload-media/",
        {
            "structure_type": "untersuchungsstelle",
            "structure_id": stelle.pk,
            "media_type": "image",
            "file": png_file(),
            "bauwerk": bauwerk.pk,
        },
        format="multipart",
    )

    assert response.status_code == 200, response.content
    media_id = response.json()["media_item"]["id"]
    assert list(stelle.media.values_list("id", flat=True)) == [media_id]
