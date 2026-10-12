"""ViewSets of the Untersuchungsverfahren."""

from __future__ import annotations

from collections.abc import Iterable
from typing import Any

from django.core.exceptions import ValidationError
from django.db import models
from django.http import Http404
from rest_framework import status, viewsets
from rest_framework.exceptions import ValidationError as DRFValidationError
from rest_framework.response import Response

from core.assetverwaltung.models.bauwerksverwaltung_models import Person
from core.assetverwaltung.views.base_viewset import BaseViewSet
from core.bauwerksdiagnostik.serializer.untersuchungSerializer import (
    BearbeiterSerializer,
    UntersuchungsverfahrenSerializer,
)
from core.bauwerksdiagnostik.views.untersuchungsverfahren_categories import (
    LABOR_UNTERSUCHUNG,
    VOR_ORT_UNTERSUCHUNG,
    all_concrete_subclasses,
    get_all_category_models,
    get_category_models,
)


def _lookup_subclass(name: str) -> type[models.Model] | None:
    """
    Case-insensitive lookup of a concrete Untersuchungs-subclass.
    Accepts short names like "Kernbohrung".
    """
    for sub in get_all_category_models():
        if sub.__name__.lower() == name.lower():
            return sub
    return None


def _prepare_field_value(field: models.Field, raw: Any) -> Any:
    """
    Normalises *raw* incoming values (e.g. empty string → ``None`` for numerics).
    """
    if raw == "" and isinstance(
        field, (models.FloatField, models.IntegerField, models.DecimalField)
    ):
        return None if field.null else raw
    return raw


def _copy_simple_fields(instance: models.Model, payload: dict[str, Any]) -> None:
    """
    Copy **normal** & **FK** fields from *payload* to *instance*.
    """
    for field in instance._meta.fields:
        if field.primary_key or field.name not in payload:
            continue

        value = _prepare_field_value(field, payload[field.name])

        # ForeignKey → resolve PK → object (or None)
        if isinstance(field, models.ForeignKey):
            if value is None:
                if not field.null:
                    raise ValidationError({field.name: "may not be null"})
                setattr(instance, field.name, None)
            else:
                fk_model = field.remote_field.model
                try:
                    setattr(instance, field.name, fk_model.objects.get(pk=value))
                except fk_model.DoesNotExist as e:
                    raise ValidationError(
                        {field.name: f"{fk_model.__name__} pk={value} not found"}
                    ) from e
        else:
            setattr(instance, field.name, value)


def _copy_m2m_fields(instance: models.Model, payload: dict[str, Any]) -> None:
    """
    Copy **many-to-many** fields from *payload* to *instance*.
    """
    for field in instance._meta.many_to_many:
        if field.name in payload:
            ids: list[int] = (
                [payload[field.name]]
                if isinstance(payload[field.name], int)
                else payload[field.name] or []
            )
            getattr(instance, field.name).set(ids)


def _optimize_queryset(queryset, model: type[models.Model]):
    """
    Automatically apply select_related() for all ForeignKey fields and
    prefetch_related() for all ManyToMany fields to avoid N+1 queries.
    """
    # Collect all ForeignKey field names for select_related
    fk_fields = [
        field.name
        for field in model._meta.fields
        if isinstance(field, models.ForeignKey)
    ]

    # Collect all ManyToMany field names for prefetch_related
    m2m_fields = [field.name for field in model._meta.many_to_many]

    # Apply optimizations
    if fk_fields:
        queryset = queryset.select_related(*fk_fields)
    if m2m_fields:
        queryset = queryset.prefetch_related(*m2m_fields)

    return queryset


def _model_to_dict(instance: models.Model) -> dict[str, Any]:
    """
    Serialise model instance to plain dict – incl. FK pk + M2M pk list.
    (Avoids pulling in a DRF serializer for dynamic models.)
    """
    data: dict[str, Any] = {}
    for field in instance._meta.fields:
        value = getattr(instance, field.name)
        if isinstance(field, models.ForeignKey):
            data[field.name] = value.pk if value else None
        elif isinstance(field, models.FileField):
            data[field.name] = value.url if value else None
        else:
            data[field.name] = value

    for field in instance._meta.many_to_many:
        data[field.name] = [obj.pk for obj in getattr(instance, field.name).all()]
    return data


###############################################################################
# Lightweight ViewSets                                                         #
###############################################################################


class BearbeiterViewSet(BaseViewSet):
    serializer_class = BearbeiterSerializer

    class Meta(BaseViewSet.Meta):
        model = Person


class ImplementedUntersuchungsverfahrenListView(viewsets.ReadOnlyModelViewSet):
    """
    GET-only endpoint returning the list of available concrete Untersuchungsmodelle.
    """

    serializer_class = UntersuchungsverfahrenSerializer  # never used but required

    def list(self, request, *args, **kwargs):
        def _names(cls: type[models.Model]) -> list[str]:
            return [c.__name__ for c in all_concrete_subclasses(cls)]

        return Response(
            {
                VOR_ORT_UNTERSUCHUNG: _names(
                    __import__(
                        "core.bauwerksdiagnostik.models.untersuchung",
                        fromlist=["VorOrtUntersuchung"],
                    ).VorOrtUntersuchung
                ),
                LABOR_UNTERSUCHUNG: _names(
                    __import__(
                        "core.bauwerksdiagnostik.models.baustoffeigenschaften",
                        fromlist=["Laboruntersuchung"],
                    ).Laboruntersuchung
                ),
            }
        )


###############################################################################
# Main ViewSet                                                                 #
###############################################################################


class UntersuchungsverfahrenViewSet(BaseViewSet):
    """
    Handles CRUD across all concrete subclasses of VorOrtUntersuchung and Laboruntersuchung.

    Pass query parameter ``?Untersuchung=<ModelName>`` to limit operations to one model.
    """

    serializer_class = UntersuchungsverfahrenSerializer

    # --------------------------------------------------------------------- #
    # Common helpers                                                        #
    # --------------------------------------------------------------------- #

    def _get_instance(self, pk: str, model_name: str | None = None) -> models.Model:
        """
        Fetch object *pk* from the requested model or – if none specified –
        from the first concrete subclass that contains it.
        """
        if model_name:
            model = _lookup_subclass(model_name)
            if not model:
                raise Http404(f"Untersuchung '{model_name}' not found.")
            try:
                # OPTIMIZATION: Use optimized queryset to avoid N+1 queries
                qs = _optimize_queryset(model.objects.all(), model)
                return qs.get(pk=pk)
            except model.DoesNotExist as e:
                raise Http404(f"{model_name} pk={pk} not found.") from e
        # Search across all subclasses
        for model in get_all_category_models():
            try:
                # OPTIMIZATION: Use optimized queryset to avoid N+1 queries
                qs = _optimize_queryset(model.objects.all(), model)
                return qs.get(pk=pk)
            except model.DoesNotExist:
                continue
        raise Http404("Untersuchungsverfahren object not found.")

    def _collect_instances(
        self,
        subclass_iter: Iterable[type[models.Model]],
        projekt_id: str | None,
        bauwerk_id: str | None,
    ) -> dict[str, list[dict[str, Any]]]:
        """
        Build `{ subclass_name: [instance_dict, …] }` for each subclass.
        """
        out: dict[str, list[dict[str, Any]]] = {}
        for subclass in subclass_iter:
            qs = subclass.objects.all()
            if projekt_id:
                qs = qs.filter(projekt=projekt_id)
            if bauwerk_id:
                qs = qs.filter(bauwerk=bauwerk_id)

            # OPTIMIZATION: Apply select_related/prefetch_related to avoid N+1 queries
            qs = _optimize_queryset(qs, subclass)

            out[subclass.__name__.lower()] = [_model_to_dict(obj) for obj in qs]
        return out

    # --------------------------------------------------------------------- #
    # List / Retrieve                                                       #
    # --------------------------------------------------------------------- #

    def list(self, request, *args, **kwargs):
        """
        Query params
        ------------
        Untersuchung=<ModelName> limit to model
        art=(VorOrtUntersuchung|Laboruntersuchung|all) group filter
        projekt=<id> / bauwerk=<id> optional relation filters
        """
        model_name = request.GET.get("Untersuchung")
        art = request.GET.get("art", "all")
        projekt_id = request.GET.get("projekt")
        bauwerk_id = request.GET.get("bauwerk")

        # Single-model fast-path
        if model_name:
            model = _lookup_subclass(model_name)
            if not model:
                raise Http404(f"Untersuchung '{model_name}' not found.")

            qs = model.objects.all()
            if projekt_id:
                qs = qs.filter(projekt=projekt_id)
            if bauwerk_id:
                qs = qs.filter(bauwerk=bauwerk_id)

            # OPTIMIZATION: Apply select_related/prefetch_related to avoid N+1 queries
            qs = _optimize_queryset(qs, model)

            return Response({model_name: [_model_to_dict(obj) for obj in qs]})

        # Otherwise build grouped response
        response: dict[str, Any] = {}
        category_models = get_category_models()
        if art in (VOR_ORT_UNTERSUCHUNG, "all"):
            response[VOR_ORT_UNTERSUCHUNG] = self._collect_instances(
                category_models[VOR_ORT_UNTERSUCHUNG], projekt_id, bauwerk_id
            )
        if art in (LABOR_UNTERSUCHUNG, "all"):
            response[LABOR_UNTERSUCHUNG] = self._collect_instances(
                category_models[LABOR_UNTERSUCHUNG], projekt_id, bauwerk_id
            )
        return Response(response)

    def retrieve(self, request, pk=None, *args, **kwargs):
        model_name = request.GET.get("Untersuchung")
        obj = self._get_instance(pk, model_name)
        return Response(_model_to_dict(obj))

    # --------------------------------------------------------------------- #
    # Mutations                                                             #
    # --------------------------------------------------------------------- #

    def _save_from_payload(
        self,
        instance: models.Model,
        payload: dict[str, Any],
        *,
        full: bool = False,
    ) -> models.Model:
        """
        Copy *payload* into *instance*, validate & save.
        ``full`` indicates PUT vs PATCH (unused for now but handy).
        """
        try:
            _copy_simple_fields(instance, payload)
            instance.full_clean()
            instance.save()  # need PK before m2m set
            _copy_m2m_fields(instance, payload)
            instance.full_clean()
            instance.save()
            return instance
        except ValidationError as exc:
            raise DRFValidationError(exc.message_dict) from exc

    def create(self, request, *args, **kwargs):
        model_name = request.data.get("type")
        if not model_name:
            return Response(
                {"error": "Please provide 'type' of Untersuchung."},
                status=status.HTTP_400_BAD_REQUEST,
            )

        model = _lookup_subclass(model_name)
        if not model:
            return Response(
                {"error": f"Untersuchung '{model_name}' not found."},
                status=status.HTTP_400_BAD_REQUEST,
            )

        # Explicit required FK validation (even if model allows null historically)
        missing = [
            f
            for f in ("projekt", "bauwerk", "untersuchungsstelle")
            if request.data.get(f) in (None, "")
        ]
        if missing:
            return Response(
                {"error": "Missing required relation(s)", "fields": missing},
                status=status.HTTP_400_BAD_REQUEST,
            )

        instance = self._save_from_payload(model(), request.data)
        return Response(_model_to_dict(instance), status=status.HTTP_201_CREATED)

    def update(self, request, pk=None, *args, **kwargs):
        model_name = request.GET.get("Untersuchung")
        if not model_name:
            return Response(
                {"error": "Query parameter 'Untersuchung' required."},
                status=status.HTTP_400_BAD_REQUEST,
            )
        instance = self._get_instance(pk, model_name)
        # For full updates ensure required fields present (PATCH already uses same path)
        missing = [
            f
            for f in ("projekt", "bauwerk", "untersuchungsstelle")
            if request.data.get(f) in (None, "")
        ]
        if missing:
            return Response(
                {"error": "Missing required relation(s)", "fields": missing},
                status=status.HTTP_400_BAD_REQUEST,
            )
        instance = self._save_from_payload(instance, request.data, full=True)
        return Response(_model_to_dict(instance))

    # PATCH
    partial_update = update  # same handling for now

    def destroy(self, request, pk=None, *args, **kwargs):
        model_name = request.GET.get("Untersuchung")
        obj = self._get_instance(pk, model_name)
        obj.delete()
        return Response(status=status.HTTP_204_NO_CONTENT)
