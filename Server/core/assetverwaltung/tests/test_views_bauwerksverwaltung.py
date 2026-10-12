import pytest
from rest_framework.test import APIClient

from core.assetverwaltung.models.bauwerksverwaltung_models import Bauwerk, Projekt


@pytest.fixture
def client():
    return APIClient()


@pytest.mark.django_db
def test_create_bauwerk_with_projekt(client):
    projekt = Projekt.objects.create(projektnummer="P-1", name="Test")
    response = client.post(
        "/api/assetverwaltung/bauwerk/",
        {"bezeichnung": "BW-1", "projekt": [projekt.pk]},
        format="json",
    )
    assert response.status_code == 201
    assert response.json()["projekt"] == [projekt.pk]


@pytest.mark.django_db
def test_projekt_filter_by_bauwerk(client):
    projekt = Projekt.objects.create(projektnummer="P-1", name="Test")
    Projekt.objects.create(projektnummer="P-2", name="Andere")
    bauwerk = Bauwerk.objects.create(bezeichnung="BW-1")
    bauwerk.projekt.add(projekt)

    response = client.get(f"/api/assetverwaltung/projekt/?bauwerk_id={bauwerk.pk}")

    assert response.status_code == 200
    assert [p["id"] for p in response.json()] == [projekt.pk]
    assert response.json()[0]["bezeichnung"] == "P-1 Test"
