from __future__ import annotations

from django.db import models

from core.bauwerksdiagnostik.models.baustoffeigenschaften import Laboruntersuchung
from core.bauwerksdiagnostik.models.untersuchung import VorOrtUntersuchung

VOR_ORT_UNTERSUCHUNG = "VorOrtUntersuchung"
LABOR_UNTERSUCHUNG = "Laboruntersuchung"


def all_concrete_subclasses(cls: type[models.Model]) -> list[type[models.Model]]:
    """Recursively return every non-abstract subclass of ``cls``."""
    subclasses: list[type[models.Model]] = []
    for sub in cls.__subclasses__():
        subclasses.extend(all_concrete_subclasses(sub))
        if not sub._meta.abstract:
            subclasses.append(sub)
    return subclasses


def get_category_models() -> dict[str, list[type[models.Model]]]:
    """Return concrete investigation models grouped by their public category."""
    return {
        VOR_ORT_UNTERSUCHUNG: all_concrete_subclasses(VorOrtUntersuchung),
        LABOR_UNTERSUCHUNG: all_concrete_subclasses(Laboruntersuchung),
    }


def get_all_category_models() -> list[type[models.Model]]:
    """Return all concrete investigation models across all categories."""
    return [
        model
        for models_by_category in get_category_models().values()
        for model in models_by_category
    ]


def get_category_for_model(model: type[models.Model]) -> str:
    """Return the public category name for a concrete investigation model."""
    if issubclass(model, Laboruntersuchung):
        return LABOR_UNTERSUCHUNG
    return VOR_ORT_UNTERSUCHUNG


def get_category_by_model_lower() -> dict[str, str]:
    """Return a lowercased model-name to public category mapping."""
    return {
        model.__name__.lower(): category
        for category, models_by_category in get_category_models().items()
        for model in models_by_category
    }
