"""Server-side regression coverage for the Bohrkern/Material relationship.

Run from Server with: uv run pytest -q core/bauwerksdiagnostik/tests/test_material_references.py
"""

import pytest
from rest_framework.test import APIClient

from core.bauwerksdiagnostik.models.algemein import Material
from core.bauwerksdiagnostik.models.bohrung import Bohrkern


URL = "/api/bwd/Bohrkern/"


@pytest.mark.django_db
def test_bohrkern_accepts_existing_material_id():
    material = Material.objects.create(name="Beton")
    response = APIClient().post(
        URL,
        {"name": "Pruefkern A", "material": [material.pk]},
        format="json",
    )
    assert response.status_code == 201, response.content
    saved = Bohrkern.objects.get(pk=response.json()["id"])
    assert list(saved.material.values_list("pk", flat=True)) == [material.pk]


@pytest.mark.django_db
def test_bohrkern_rejects_nonexistent_material_without_creating_record():
    # The requested ID must be nonexistent even if test fixtures change.
    absent_id = (Material.objects.order_by("-pk").values_list("pk", flat=True).first() or 0) + 999
    response = APIClient().post(
        URL,
        {"name": "Pruefkern B", "material": [absent_id]},
        format="json",
    )
    assert response.status_code == 400, response.content
    assert "material" in response.json()
    assert not Bohrkern.objects.filter(name="Pruefkern B").exists()


@pytest.mark.django_db
def test_bohrkern_rejects_stale_material_id_on_edit_and_preserves_old_links():
    old = Material.objects.create(name="Beton")
    core = Bohrkern.objects.create(name="Pruefkern C")
    core.material.add(old)
    absent_id = old.pk + 999
    response = APIClient().patch(
        f"{URL}{core.pk}/", {"material": [absent_id]}, format="json"
    )
    assert response.status_code == 400, response.content
    assert "material" in response.json()
    assert list(core.material.values_list("pk", flat=True)) == [old.pk]
