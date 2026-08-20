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


def _normalize_integer_exponent(value: object) -> int:
    """Match JSON Schema integer-value semantics for ordinary JSON decoders.

    JSON Schema treats numbers such as 1 and 1.0 as integer instances when their
    mathematical value is integral. Booleans are not numbers for this purpose.
    """
    if isinstance(value, bool):
        raise DimensionConstraintError("DIMENSION_EXPONENT_INVALID")
    if isinstance(value, int):
        return value
    if isinstance(value, float) and value.is_integer():
        return int(value)
    raise DimensionConstraintError("DIMENSION_EXPONENT_INVALID")


def normalize_dimension_vector(vector: Mapping[str, object]) -> dict[str, int]:
    """Expand sparse authoring to the canonical full seven-axis vector."""
    unknown = set(vector) - set(DIMENSION_AXES)
    if unknown:
        raise DimensionConstraintError("DIMENSION_AXIS_UNKNOWN")
    normalized: dict[str, int] = {}
    for axis in DIMENSION_AXES:
        if axis in vector:
            normalized[axis] = _normalize_integer_exponent(vector[axis])
        else:
            normalized[axis] = 0
    return normalized


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
    left: Mapping[str, object],
    right: Mapping[str, object],
) -> dict[str, int]:
    """Dimension constraints intersect iff their normalized vectors are equal."""
    left_normalized = normalize_dimension_vector(left)
    right_normalized = normalize_dimension_vector(right)
    if left_normalized != right_normalized:
        raise DimensionConstraintError("DIMENSION_INTERSECTION_EMPTY")
    return left_normalized


def is_dimension_one(vector: Mapping[str, object]) -> bool:
    return all(exponent == 0 for exponent in normalize_dimension_vector(vector).values())
