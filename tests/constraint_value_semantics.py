"""Minimal design-stage semantic helper for ADR-0021 scalar Value constraints.

Numeric authoring normalization consumes lossless numeric lexemes supplied by the
authoring reader. This avoids making Python/JavaScript native floating-point values
semantic authorities.
"""

from __future__ import annotations

from decimal import Decimal
import re
from typing import Mapping, Sequence


_NUMBER_RE = re.compile(r"^(-?)(0|[1-9][0-9]*)(?:\.([0-9]+))?(?:[eE]([+-]?[0-9]+))?$")


class ValueConstraintError(ValueError):
    """Deterministic ADR-0021 semantic/compiler diagnostic."""


def canonicalize_decimal_lexeme(text: str) -> dict[str, object]:
    """Normalize one finite JSON-compatible decimal numeric token exactly."""
    if not isinstance(text, str):
        raise ValueConstraintError("VALUE_NUMBER_LEXEME_REQUIRED")
    match = _NUMBER_RE.fullmatch(text)
    if not match:
        raise ValueConstraintError("VALUE_NUMBER_INVALID")
    sign, integer_part, fraction_part, exponent_part = match.groups()
    fraction_part = fraction_part or ""
    explicit_exp = int(exponent_part or "0")

    digits = (integer_part + fraction_part).lstrip("0")
    if not digits:
        return {"coefficient": "0", "exponent10": 0}

    exponent10 = explicit_exp - len(fraction_part)
    trailing = len(digits) - len(digits.rstrip("0"))
    if trailing:
        digits = digits[:-trailing]
        exponent10 += trailing

    coefficient = ("-" if sign else "") + digits
    return {"coefficient": coefficient, "exponent10": exponent10}


def _normalize_integer_value(value: object) -> int:
    """Match JSON Schema integer-value semantics for normalized exponent10."""
    if isinstance(value, bool):
        raise ValueConstraintError("VALUE_NUMBER_CANONICAL_INVALID")
    if isinstance(value, int):
        return value
    if isinstance(value, float) and value.is_integer():
        return int(value)
    raise ValueConstraintError("VALUE_NUMBER_CANONICAL_INVALID")


def _decimal_value(value: Mapping[str, object]) -> Decimal:
    coefficient = value.get("coefficient")
    if not isinstance(coefficient, str):
        raise ValueConstraintError("VALUE_NUMBER_CANONICAL_INVALID")
    exponent10 = _normalize_integer_value(value.get("exponent10"))
    if coefficient == "0":
        if exponent10 != 0:
            raise ValueConstraintError("VALUE_NUMBER_CANONICAL_INVALID")
        return Decimal(0)
    if not re.fullmatch(r"-?[1-9](?:[0-9]*[1-9])?", coefficient):
        raise ValueConstraintError("VALUE_NUMBER_CANONICAL_INVALID")
    return Decimal(f"{coefficient}e{exponent10}")


def compare_decimal(left: Mapping[str, object], right: Mapping[str, object]) -> int:
    lvalue = _decimal_value(left)
    rvalue = _decimal_value(right)
    return -1 if lvalue < rvalue else (1 if lvalue > rvalue else 0)


def require_numeric_comparison_space(state: str) -> None:
    if state == "unresolved":
        raise ValueConstraintError("VALUE_COMPARISON_SPACE_UNRESOLVED")
    if state not in {"not_required", "resolved"}:
        raise ValueConstraintError("VALUE_COMPARISON_SPACE_STATE_INVALID")


def _normalize_bound(bound: Mapping[str, object] | None) -> dict[str, object] | None:
    if bound is None:
        return None
    lexeme = bound.get("value_lexeme")
    inclusive = bound.get("inclusive")
    if not isinstance(inclusive, bool):
        raise ValueConstraintError("VALUE_BOUND_INCLUSIVE_REQUIRED")
    return {"value": canonicalize_decimal_lexeme(lexeme), "inclusive": inclusive}


def _interval_empty(lower: Mapping[str, object] | None, upper: Mapping[str, object] | None) -> bool:
    if lower is None or upper is None:
        return False
    cmp = compare_decimal(lower["value"], upper["value"])
    if cmp > 0:
        return True
    if cmp == 0 and (not lower["inclusive"] or not upper["inclusive"]):
        return True
    return False


def normalize_numeric_interval(
    *,
    lower: Mapping[str, object] | None = None,
    upper: Mapping[str, object] | None = None,
    comparison_space_state: str,
) -> dict[str, object]:
    require_numeric_comparison_space(comparison_space_state)
    nlower = _normalize_bound(lower)
    nupper = _normalize_bound(upper)
    if _interval_empty(nlower, nupper):
        return {"status": "empty"}
    payload: dict[str, object] = {"type": "value", "form": "numeric_interval"}
    if nlower is not None:
        payload["lower"] = nlower
    if nupper is not None:
        payload["upper"] = nupper
    return {"status": "satisfiable", "payload": payload}


def normalize_allowed_set(
    *,
    scalar_kind: str,
    values: Sequence[object],
    comparison_space_state: str | None = None,
) -> dict[str, object]:
    if scalar_kind == "number":
        if comparison_space_state is None:
            raise ValueConstraintError("VALUE_COMPARISON_SPACE_STATE_REQUIRED")
        require_numeric_comparison_space(comparison_space_state)
        normalized: list[dict[str, object]] = []
        seen: set[tuple[str, int]] = set()
        for raw in values:
            if not isinstance(raw, str):
                raise ValueConstraintError("VALUE_NUMBER_LEXEME_REQUIRED")
            item = canonicalize_decimal_lexeme(raw)
            key = (str(item["coefficient"]), int(item["exponent10"]))
            if key not in seen:
                seen.add(key)
                normalized.append(item)
    elif scalar_kind == "string":
        if any(not isinstance(v, str) for v in values):
            raise ValueConstraintError("VALUE_ALLOWED_SET_KIND_MISMATCH")
        normalized = list(dict.fromkeys(values))
    elif scalar_kind == "boolean":
        if any(not isinstance(v, bool) for v in values):
            raise ValueConstraintError("VALUE_ALLOWED_SET_KIND_MISMATCH")
        normalized = list(dict.fromkeys(values))
    else:
        raise ValueConstraintError("VALUE_SCALAR_KIND_INVALID")

    if not normalized:
        return {"status": "empty"}
    return {
        "status": "satisfiable",
        "payload": {
            "type": "value",
            "form": "allowed_set",
            "scalar_kind": scalar_kind,
            "values": normalized,
        },
    }


def _stronger_lower(left: Mapping[str, object] | None, right: Mapping[str, object] | None):
    if left is None:
        return right
    if right is None:
        return left
    cmp = compare_decimal(left["value"], right["value"])
    if cmp > 0:
        return left
    if cmp < 0:
        return right
    return {"value": left["value"], "inclusive": bool(left["inclusive"] and right["inclusive"])}


def _stronger_upper(left: Mapping[str, object] | None, right: Mapping[str, object] | None):
    if left is None:
        return right
    if right is None:
        return left
    cmp = compare_decimal(left["value"], right["value"])
    if cmp < 0:
        return left
    if cmp > 0:
        return right
    return {"value": left["value"], "inclusive": bool(left["inclusive"] and right["inclusive"])}


def intersect_intervals(left: Mapping[str, object], right: Mapping[str, object]) -> dict[str, object]:
    lower = _stronger_lower(left.get("lower"), right.get("lower"))
    upper = _stronger_upper(left.get("upper"), right.get("upper"))
    if _interval_empty(lower, upper):
        return {"status": "empty"}
    payload: dict[str, object] = {"type": "value", "form": "numeric_interval"}
    if lower is not None:
        payload["lower"] = lower
    if upper is not None:
        payload["upper"] = upper
    return {"status": "satisfiable", "payload": payload}


def _number_inside_interval(value: Mapping[str, object], interval: Mapping[str, object]) -> bool:
    lower = interval.get("lower")
    upper = interval.get("upper")
    if lower is not None:
        cmp = compare_decimal(value, lower["value"])
        if cmp < 0 or (cmp == 0 and not lower["inclusive"]):
            return False
    if upper is not None:
        cmp = compare_decimal(value, upper["value"])
        if cmp > 0 or (cmp == 0 and not upper["inclusive"]):
            return False
    return True


def intersect_allowed_sets(left: Mapping[str, object], right: Mapping[str, object]) -> dict[str, object]:
    if left.get("scalar_kind") != right.get("scalar_kind"):
        return {"status": "empty"}
    kind = left["scalar_kind"]
    if kind == "number":
        right_keys = {(v["coefficient"], v["exponent10"]) for v in right["values"]}
        values = [v for v in left["values"] if (v["coefficient"], v["exponent10"]) in right_keys]
    else:
        right_values = set(right["values"])
        values = [v for v in left["values"] if v in right_values]
    if not values:
        return {"status": "empty"}
    return {
        "status": "satisfiable",
        "payload": {"type": "value", "form": "allowed_set", "scalar_kind": kind, "values": values},
    }


def intersect_interval_and_set(interval: Mapping[str, object], allowed_set: Mapping[str, object]) -> dict[str, object]:
    if allowed_set.get("scalar_kind") != "number":
        return {"status": "empty"}
    values = [v for v in allowed_set["values"] if _number_inside_interval(v, interval)]
    if not values:
        return {"status": "empty"}
    return {
        "status": "satisfiable",
        "payload": {"type": "value", "form": "allowed_set", "scalar_kind": "number", "values": values},
    }


def classify_empty_result(result: Mapping[str, object], contributor_context: str) -> str | None:
    if result.get("status") != "empty":
        return None
    if contributor_context == "static":
        return "Schema Conflict"
    if contributor_context == "active_conditional":
        return "Configuration Conflict"
    if contributor_context == "inactive_conditional":
        return None
    raise ValueConstraintError("VALUE_CONTRIBUTOR_CONTEXT_INVALID")
