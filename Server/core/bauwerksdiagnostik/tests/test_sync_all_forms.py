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
    ("RueckprallHammer", "hammertyp", "N", "L"),
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


@pytest.mark.django_db
@pytest.mark.parametrize("route,field,start,new", FORMS)
def test_sync_preserves_independent_server_edit(
    client, relations, route, field, start, new
):
    record = create(client, route, relations, field, start)

    server_edit = client.patch(
        f"/api/bwd/{route}/{record['id']}/",
        {"bemerkung": "Changed on server"},
        format="json",
    )
    assert server_edit.status_code == 200, server_edit.content

    response = client.patch(
        f"/api/bwd/{route}/sync/",
        [{
            "id": record["id"],
            "patch": [{"field": field, "base": start, "new": new}],
        }],
        format="json",
    )
    assert response.status_code == 200, response.content

    # Read the persisted record, not just the sync response.
    reread = client.get(f"/api/bwd/{route}/{record['id']}/")
    assert reread.status_code == 200, reread.content
    assert reread.json()[field] == new
    assert reread.json()["bemerkung"] == "Changed on server"


@pytest.mark.django_db
@pytest.mark.parametrize("route,field,start,new", FORMS)
def test_sync_retry_does_not_change_value_or_version(
    client, relations, route, field, start, new
):
    record = create(client, route, relations, field, start)
    url = f"/api/bwd/{route}/sync/"
    payload = [{
        "id": record["id"],
        "patch": [{"field": field, "base": start, "new": new}],
    }]

    first = client.patch(url, payload, format="json")
    assert first.status_code == 200, first.content

    detail_url = f"/api/bwd/{route}/{record['id']}/"
    before_retry = client.get(detail_url)
    assert before_retry.status_code == 200, before_retry.content
    saved = before_retry.json()
    assert saved[field] == new

    # Repeat the exact request, as after a lost response.
    retry = client.patch(url, payload, format="json")
    assert retry.status_code == 200, retry.content
    assert retry.json().get("conflicts") == []

    after_retry = client.get(detail_url)
    assert after_retry.status_code == 200, after_retry.content
    assert after_retry.json()[field] == new
    assert after_retry.json()["version"] == saved["version"]

@pytest.mark.django_db
@pytest.mark.parametrize("route,field,start,new", FORMS)
def test_sync_mixed_batch_saves_clean_record_and_preserves_conflict(
    client, relations, route, field, start, new
):
    conflicting = create(client, route, relations, field, start)
    clean = create(client, route, relations, field, start)

    detail_url = f"/api/bwd/{route}/{conflicting['id']}/"
    server_edit = client.patch(
        detail_url,
        {"bemerkung": "Server note"},
        format="json",
    )
    assert server_edit.status_code == 200, server_edit.content
    conflict_version = server_edit.json()["version"]

    response = client.patch(
        f"/api/bwd/{route}/sync/",
        [
            {
                "id": conflicting["id"],
                "patch": [{
                    "field": "bemerkung",
                    "base": "alt",
                    "new": "Offline note",
                }],
            },
            {
                "id": clean["id"],
                "patch": [{"field": field, "base": start, "new": new}],
            },
        ],
        format="json",
    )
    assert response.status_code == 409, response.content

    result = response.json()
    assert {row["id"] for row in result["saved"]} == {clean["id"]}
    assert {row["id"] for row in result["conflicts"]} == {
        conflicting["id"]
    }

    # Verify both records in the database through fresh GET requests.
    conflicted_get = client.get(detail_url)
    assert conflicted_get.status_code == 200, conflicted_get.content
    assert conflicted_get.json()["bemerkung"] == "Server note"
    assert conflicted_get.json()["version"] == conflict_version

    clean_get = client.get(f"/api/bwd/{route}/{clean['id']}/")
    assert clean_get.status_code == 200, clean_get.content
    assert clean_get.json()[field] == new
    assert clean_get.json()["version"] == clean["version"] + 1

    