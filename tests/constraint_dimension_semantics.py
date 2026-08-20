"""Minimal design-stage semantic helper for ADR-0020 Dimension constraints."""

from __future__ import annotations

from typing import Mapping


DIMENSION_AXES = (
    "time",
    "length",
    "mass",
    "electric_current",
    "thermodynamic_temperature",
    "amount_of_substance",
    "luminous_intensity",
)


class DimensionConstraintError(ValueError):
    """Deterministic ADR-0020 semantic diagnostic."""


def normalize_dimension_vector(vector: Mapping[str, int]) -> dict[str, int]:
    """Expand sparse authoring to the canonical full seven-axis vector."""
    unknown = set(vector) - set(DIMENSION_AXES)
    if unknown:
        raise DimensionConstraintError("DIMENSION_AXIS_UNKNOWN")
    for exponent in vector.values():
        if isinstance(exponent, bool) or not isinstance(exponent, int):
            raise DimensionConstraintError("DIMENSION_EXPONENT_INVALID")
    return {axis: int(vector.get(axis, 0)) for axis in DIMENSION_AXES}


def normalize_dimension_payload(authored: Mapping[str, object]) -> dict[str, object]:
    if authored.get("type") != "dimension":
        raise DimensionConstraintError("DIMENSION_CONSTRAINT_KIND_INVALID")
    if "vector" not in authored:
        raise DimensionConstraintError("DIMENSION_VECTOR_REQUIRED")
    vector = authored["vector"]
    if not isinstance(vector, Mapping):
        raise DimensionConstraintError("DIMENSION_VECTOR_INVALID")
    return {"type": "dimension", "vector": normalize_dimension_vector(vector)}


def intersect_dimension_vectors(
    left: Mapping[str, int],
    right: Mapping[str, int],
) -> dict[str, int]:
    """Dimension constraints intersect iff their normalized vectors are equal."""
    left_normalized = normalize_dimension_vector(left)
    right_normalized = normalize_dimension_vector(right)
    if left_normalized != right_normalized:
        raise DimensionConstraintError("DIMENSION_INTERSECTION_EMPTY")
    return left_normalized


def is_dimension_one(vector: Mapping[str, int]) -> bool:
    return all(exponent == 0 for exponent in normalize_dimension_vector(vector).values())
