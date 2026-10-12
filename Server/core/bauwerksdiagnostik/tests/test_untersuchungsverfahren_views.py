import pytest
from django.core.management import call_command
from rest_framework.test import APIClient

from core.assetverwaltung.models.bauwerksverwaltung_models import (
    Bauwerk,
    Person,
    Projekt,
)
from core.bauwerksdiagnostik.models.bohrung import Kernbohrung
from core.bauwerksdiagnostik.models.projektspezifisch import Untersuchungsstelle

URL = "/api/bwd/Untersuchungsverfahren/"


@pytest.fixture
def client():
    return APIClient()


@pytest.fixture
def structure():
    projekt = Projekt.objects.create(projektnummer="P-1", name="Test")
    bauwerk = Bauwerk.objects.create(bezeichnung="BW-1")
    bauwerk.projekt.add(projekt)
    stelle = Untersuchungsstelle.objects.create(
        bauwerk=bauwerk, projekt=projekt, name="US-1"
    )
    bearbeiter = Person.objects.create(name="Muster", vorname="Max")
    return {
        "projekt": projekt.pk,
        "bauwerk": bauwerk.pk,
        "untersuchungsstelle": stelle.pk,
        "bearbeiter": [bearbeiter.pk],
    }


@pytest.mark.django_db
def test_implemented_untersuchungsverfahren(client):
    response = client.get("/api/bwd/ImplementedUntersuchungsverfahren/")

    assert response.status_code == 200
    assert sorted(response.json()["VorOrtUntersuchung"]) == [
        "Chloridanalyse",
        "Kernbohrung",
        "Rissbreitenmessung",
        "Rueckprallhammer",
    ]
    assert response.json()["Laboruntersuchung"] == ["Druckfestigkeitspruefung"]


@pytest.mark.django_db
def test_untersuchungsverfahren_crud(client, structure):
    created = client.post(
        URL,
        {**structure, "type": "Kernbohrung", "durchmesser": 100},
        format="json",
    )
    assert created.status_code == 201, created.content
    uv_id = created.json()["id"]
    assert created.json()["bearbeiter"] == structure["bearbeiter"]
    assert created.json()["sort_order"] == 1

    detail = client.get(f"{URL}{uv_id}/?Untersuchung=Kernbohrung")
    assert detail.json()["durchmesser"] == 100

    updated = client.patch(
        f"{URL}{uv_id}/?Untersuchung=Kernbohrung",
        {**structure, "bohrlaenge": ""},
        format="json",
    )
    assert updated.status_code == 200
    assert updated.json()["bohrlaenge"] is None

    grouped = client.get(f"{URL}?art=all&bauwerk={structure['bauwerk']}")
    assert [uv["id"] for uv in grouped.json()["VorOrtUntersuchung"]["kernbohrung"]] == [
        uv_id
    ]
    assert grouped.json()["Laboruntersuchung"] == {"druckfestigkeitspruefung": []}

    deleted = client.delete(f"{URL}{uv_id}/?Untersuchung=Kernbohrung")
    assert deleted.status_code == 204
    assert not Kernbohrung.objects.exists()


@pytest.mark.django_db
def test_untersuchungsverfahren_requires_relations(client, structure):
    response = client.post(
        URL, {"type": "Kernbohrung", "projekt": structure["projekt"]}, format="json"
    )

    assert response.status_code == 400
    assert response.json()["fields"] == ["bauwerk", "untersuchungsstelle"]


@pytest.mark.django_db
def test_untersuchungsstelle_filter_by_bauwerk(client, structure):
    Untersuchungsstelle.objects.create(
        bauwerk=Bauwerk.objects.create(bezeichnung="BW-2"), name="Andere"
    )

    response = client.get(
        f"/api/bwd/Untersuchungsstelle/?bauwerk_id={structure['bauwerk']}"
    )

    assert [us["id"] for us in response.json()] == [structure["untersuchungsstelle"]]


@pytest.mark.django_db
def test_seed_bauwerksdiagnostik_is_idempotent():
    call_command("seed_bauwerksdiagnostik")
    call_command("seed_bauwerksdiagnostik")

    assert Bauwerk.objects.count() == 1
    assert Kernbohrung.objects.count() == 1


@pytest.mark.django_db
def test_rissbreitenmessung_stores_risse_and_validates_choices(client, structure):
    risse = [
        {
            "id": "riss-1",
            "bezeichnung": "R1",
            "breite": 0.3,
            "laenge": 200,
            "verlauf": "quer",
            "wasserfuehrend": False,
        }
    ]
    created = client.post(
        URL,
        {
            **structure,
            "type": "Rissbreitenmessung",
            "messmittel": "risslupe",
            "risse": risse,
            "maxRissbreite": 0.3,
        },
        format="json",
    )
    assert created.status_code == 201, created.content
    assert created.json()["risse"] == risse

    invalid = client.patch(
        f"{URL}{created.json()['id']}/?Untersuchung=Rissbreitenmessung",
        {**structure, "messmittel": "zollstock"},
        format="json",
    )
    assert invalid.status_code == 400
    assert "messmittel" in invalid.json()
