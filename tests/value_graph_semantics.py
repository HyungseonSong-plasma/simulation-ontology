"""Focused design-stage semantic helpers for ADR-0026/0027."""

from __future__ import annotations

from math import prod
from typing import Mapping, Sequence


DIMENSION_AXES = (
    "time",
    "length",
    "mass",
    "electric_current",
    "thermodynamic_temperature",
    "amount_of_substance",
    "luminous_intensity",
)


class ValueGraphError(ValueError):
    """Deterministic ADR-0026/0027 validation diagnostic."""


def _semantic_integer(value: object) -> int:
    if isinstance(value, bool):
        raise ValueGraphError("VALUE_INTEGER_INVALID")
    if isinstance(value, int):
        return value
    if isinstance(value, float) and value.is_integer():
        return int(value)
    raise ValueGraphError("VALUE_INTEGER_INVALID")


def dimension_tuple(vector: Mapping[str, object]) -> tuple[int, ...]:
    if set(vector) != set(DIMENSION_AXES):
        raise ValueGraphError("DIMENSION_VECTOR_INVALID")
    return tuple(_semantic_integer(vector[axis]) for axis in DIMENSION_AXES)


def is_dimension_one(vector: Mapping[str, object]) -> bool:
    return all(exponent == 0 for exponent in dimension_tuple(vector))


def dimensions_equal(left: Mapping[str, object], right: Mapping[str, object]) -> bool:
    return dimension_tuple(left) == dimension_tuple(right)


def unit_reference_key(unit: Mapping[str, object]) -> tuple[str, str]:
    if set(unit) != {"namespace", "id"}:
        raise ValueGraphError("UNIT_REFERENCE_INVALID")
    namespace = unit.get("namespace")
    identifier = unit.get("id")
    if not isinstance(namespace, str) or not namespace or not isinstance(identifier, str) or not identifier:
        raise ValueGraphError("UNIT_REFERENCE_INVALID")
    return namespace, identifier


def validate_value_semantics(value: Mapping[str, object]) -> bool:
    shape = value.get("shape")
    scalar_kind = value.get("scalar_kind")
    if shape not in {"scalar", "vector", "tensor"}:
        raise ValueGraphError("VALUE_SHAPE_INVALID")
    if scalar_kind not in {"number", "string", "boolean"}:
        raise ValueGraphError("VALUE_SCALAR_KIND_INVALID")

    if "unit" in value:
        if scalar_kind != "number":
            raise ValueGraphError("VALUE_UNIT_NON_NUMERIC")
        unit = value["unit"]
        if not isinstance(unit, Mapping):
            raise ValueGraphError("UNIT_REFERENCE_INVALID")
        unit_reference_key(unit)

    data = value.get("data")
    if shape == "scalar":
        if "tensor_shape" in value:
            raise ValueGraphError("VALUE_TENSOR_SHAPE_UNEXPECTED")
        return True

    if not isinstance(data, list) or not data:
        raise ValueGraphError("VALUE_ARRAY_EMPTY")

    if shape == "vector":
        if "tensor_shape" in value:
            raise ValueGraphError("VALUE_TENSOR_SHAPE_UNEXPECTED")
        return True

    tensor_shape = value.get("tensor_shape")
    if not isinstance(tensor_shape, list) or len(tensor_shape) < 2:
        raise ValueGraphError("VALUE_TENSOR_SHAPE_INVALID")
    dimensions = [_semantic_integer(item) for item in tensor_shape]
    if any(item <= 0 for item in dimensions):
        raise ValueGraphError("VALUE_TENSOR_SHAPE_INVALID")
    if len(data) != prod(dimensions):
        raise ValueGraphError("VALUE_TENSOR_COMPONENT_COUNT_MISMATCH")
    return True


def evaluate_metrology_state(
    *,
    expected_dimension: Mapping[str, object],
    explicit_unit: Mapping[str, object] | None,
    resolved_unit_dimension: Mapping[str, object] | None = None,
    omission_policy: str = "unresolved",
    compatibility_state: str = "PASS",
) -> tuple[str, str | None]:
    """Return ADR-0026 semantic state and stable diagnostic."""

    dimension_tuple(expected_dimension)

    if explicit_unit is None:
        if omission_policy == "forbidden":
            return "FAIL", "VALUE_UNIT_REQUIRED_MISSING"
        if omission_policy == "unresolved":
            return "INDETERMINATE", "VALUE_CONTEXTUAL_UNIT_POLICY_UNRESOLVED"
        if omission_policy == "permitted":
            return "PASS", None
        raise ValueGraphError("VALUE_UNIT_OMISSION_POLICY_INVALID")

    unit_reference_key(explicit_unit)
    if resolved_unit_dimension is None:
        return "INDETERMINATE", "VALUE_UNIT_REFERENCE_UNRESOLVED"
    if not dimensions_equal(expected_dimension, resolved_unit_dimension):
        return "FAIL", "VALUE_UNIT_DIMENSION_MISMATCH"

    if compatibility_state == "FAIL":
        return "FAIL", "VALUE_UNIT_COMPATIBILITY_FAIL"
    if compatibility_state == "INDETERMINATE":
        return "INDETERMINATE", "VALUE_AFFINE_SEMANTIC_CONTEXT_UNRESOLVED"
    if compatibility_state != "PASS":
        raise ValueGraphError("VALUE_UNIT_COMPATIBILITY_STATE_INVALID")
    return "PASS", None


def choose_value_definition_representation(
    *,
    mechanism: str,
    provider_state: str = "PASS",
    resolved_semantic_dependencies: Sequence[str] = (),
    other_reification_trigger: bool = False,
) -> tuple[str, str | None]:
    """Select inline/reified form after ADR-0027 provider normalization."""

    if mechanism not in {"literal", "expression", "function", "tabular"}:
        raise ValueGraphError("VALUE_DEFINITION_MECHANISM_INVALID")

    if mechanism != "literal":
        if provider_state == "INDETERMINATE":
            return "INDETERMINATE", "VALUE_DEFINITION_FORMAT_PROVIDER_UNRESOLVED"
        if provider_state == "FAIL":
            return "FAIL", "VALUE_DEFINITION_FORMAT_PROVIDER_FAIL"
        if provider_state != "PASS":
            raise ValueGraphError("VALUE_DEFINITION_PROVIDER_STATE_INVALID")

    canonical_dependencies: set[str] = set()
    for dependency in resolved_semantic_dependencies:
        if not isinstance(dependency, str) or not dependency:
            raise ValueGraphError("VALUE_DEFINITION_DEPENDENCY_INVALID")
        canonical_dependencies.add(dependency)

    if canonical_dependencies or other_reification_trigger:
        return "REIFIED", None
    return "INLINE", None


def validate_reified_definition_graph(
    *,
    definition_payloads: Sequence[tuple[str, object]],
    owner_edges: Sequence[tuple[str, str]],
    dependency_edges: Sequence[tuple[str, str]],
) -> bool:
    """Validate one resolved model-instance identity per reified definition."""

    definitions: dict[str, object] = {}
    for identifier, payload in definition_payloads:
        if not isinstance(identifier, str) or not identifier:
            raise ValueGraphError("VALUE_DEFINITION_REFERENCE_UNRESOLVED")
        if identifier in definitions and definitions[identifier] != payload:
            raise ValueGraphError("VALUE_DEFINITION_IDENTITY_CONTENT_CONFLICT")
        definitions[identifier] = payload

    for owner, definition_id in owner_edges:
        if not isinstance(owner, str) or not owner or definition_id not in definitions:
            raise ValueGraphError("VALUE_DEFINITION_REFERENCE_UNRESOLVED")

    for definition_id, dependency in dependency_edges:
        if definition_id not in definitions:
            raise ValueGraphError("VALUE_DEFINITION_RELATION_ENDPOINT_INVALID")
        if not isinstance(dependency, str) or not dependency:
            raise ValueGraphError("VALUE_DEFINITION_RELATION_ENDPOINT_INVALID")

    return True
