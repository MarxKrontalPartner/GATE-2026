"""Sync endpoint and bulk create for all diagnostic forms."""

import pytest
from rest_framework.test import APIClient

from core.assetverwaltung.models.bauwerksverwaltung_models import Bauwerk, Projekt
from core.bauwerksdiagnostik.models.projektspezifisch import Untersuchungsstelle

# (route, a field of the form with its start value and a new value)
FORMS = [
    ("Kernbohrung", "verschlussmaterial", "PCC", "Epoxid"),
    ("Rissbreitenmessung", "maxRissbreite", 0.2, 0.3),
    ("Chloridanalyse", "pruefstelle", "Labor A", "Labor B"),
    ("Druckfestigkeitspruefung", "bruchlast", 100.0, 120.0),
]


@pytest.fixture
def client():
    return APIClient()


@pytest.fixture
def relations():
    projekt = Projekt.objects.create(projektnummer="P-1", name="Test")
    bauwerk = Bauwerk.objects.create(bezeichnung="BW-1")
    stelle = Untersuchungsstelle.objects.create(bauwerk=bauwerk, projekt=projekt, name="US-1")
    # datum is always sent by the app (the model default is a datetime, which
    # the serializer refuses to output as a date).
    return {"projekt": projekt.pk, "bauwerk": bauwerk.pk, "untersuchungsstelle": stelle.pk,
            "datum": "2026-11-22"}


def create(client, route, relations, field, value):
    res = client.post(f"/api/bwd/{route}/", {**relations, "bemerkung": "alt", field: value}, format="json")
    assert res.status_code == 201, res.content
    return res.json()


@pytest.mark.django_db
@pytest.mark.parametrize("route,field,start,new", FORMS)
def test_sync_saves_and_bumps_version(client, relations, route, field, start, new):
    record = create(client, route, relations, field, start)
    res = client.patch(f"/api/bwd/{route}/sync/", [
        {"id": record["id"], "patch": [{"field": field, "base": start, "new": new}]}
    ], format="json")
    assert res.status_code == 200, res.content
    saved = res.json()["saved"][0]
    assert saved[field] == new
    assert saved["version"] == 2


@pytest.mark.django_db
@pytest.mark.parametrize("route,field,start,new", FORMS)
def test_sync_reports_conflict(client, relations, route, field, start, new):
    record = create(client, route, relations, "bemerkung", "Server")
    res = client.patch(f"/api/bwd/{route}/sync/", [
        {"id": record["id"], "patch": [{"field": "bemerkung", "base": "alt", "new": "App"}]}
    ], format="json")
    assert res.status_code == 409
    assert res.json()["conflicts"][0]["conflicts"] == [
        {"field": "bemerkung", "server_value": "Server"}]


@pytest.mark.django_db
@pytest.mark.parametrize("route,field,start,new", FORMS)
def test_bulk_create_returns_records_in_order(client, relations, route, field, start, new):
    res = client.post(f"/api/bwd/{route}/", [
        {**relations, field: start, "bemerkung": "erste"},
        {**relations, field: new, "bemerkung": "zweite"},
    ], format="json")
    assert res.status_code == 201, res.content
    assert [r["bemerkung"] for r in res.json()] == ["erste", "zweite"]
    assert all(r["version"] == 1 for r in res.json())


@pytest.mark.django_db
@pytest.mark.parametrize("route,field,start,new", FORMS)
def test_sync_rejects_media_and_m2m_fields(client, relations, route, field, start, new):
    record = create(client, route, relations, field, start)
    res = client.patch(f"/api/bwd/{route}/sync/", [
        {"id": record["id"], "patch": [{"field": "bilder", "base": [], "new": [1]}]}
    ], format="json")
    assert res.status_code == 400
