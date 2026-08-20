"""Minimal design-stage semantic helper for ADR-0023/0024 Predicate and Conditional validation."""

from __future__ import annotations

from typing import Mapping, Sequence

from tests.constraint_value_semantics import compare_decimal


class PredicateEvaluationError(ValueError):
    """Deterministic Predicate normalization/reference error."""


def truth(value: str, diagnostics=()):
    if value not in {"TRUE", "FALSE", "INDETERMINATE"}:
        raise PredicateEvaluationError("PREDICATE_TRUTH_INVALID")
    return {"kind": "truth", "truth": value, "diagnostics": frozenset(diagnostics)}


def failure(*codes: str):
    return {"kind": "failure", "codes": frozenset(codes)}


def resolve_binding(key: str, bindings: Sequence[Mapping[str, object]]) -> Mapping[str, object]:
    matches = [binding for binding in bindings if binding.get("key") == key]
    if not matches:
        raise PredicateEvaluationError("PREDICATE_REFERENCE_UNRESOLVED")
    if len(matches) != 1:
        raise PredicateEvaluationError("PREDICATE_REFERENCE_AMBIGUOUS")
    return matches[0]


def lookup_reference(
    ref: Mapping[str, object],
    bindings: Sequence[Mapping[str, object]],
    snapshot: Mapping[str, Mapping[str, object]],
) -> Mapping[str, object]:
    key = ref.get("key")
    if not isinstance(key, str) or not key:
        raise PredicateEvaluationError("PREDICATE_REFERENCE_KEY_INVALID")
    binding = resolve_binding(key, bindings)
    slot = binding.get("slot")
    if not isinstance(slot, str) or not slot:
        raise PredicateEvaluationError("PREDICATE_BINDING_SLOT_INVALID")
    result = snapshot.get(slot)
    if result is None:
        return {"state": "ABSENT"}
    state = result.get("state")
    if state not in {"PRESENT", "ABSENT", "UNRESOLVED"}:
        raise PredicateEvaluationError("PREDICATE_LOOKUP_STATE_INVALID")
    return result


def _scalar_kind(value: object) -> str | None:
    if isinstance(value, bool):
        return "boolean"
    if isinstance(value, str):
        return "string"
    if isinstance(value, Mapping) and "coefficient" in value and "exponent10" in value:
        return "number"
    return None


def _same_scalar(left: object, right: object, kind: str) -> bool:
    if kind == "number":
        return compare_decimal(left, right) == 0
    return left == right


def normalize_membership_values(values: Sequence[object], scalar_kind: str) -> list[object]:
    """Collapse duplicate normalized Membership values under ADR-0023 scalar equality.

    Input values are already in normalized scalar representation. Authoring numeric-lexeme
    capture remains the compiler/reader responsibility, as in ADR-0021.
    """
    if scalar_kind not in {"number", "string", "boolean"}:
        raise PredicateEvaluationError("PREDICATE_SCALAR_KIND_INVALID")
    normalized: list[object] = []
    for value in values:
        if _scalar_kind(value) != scalar_kind:
            raise PredicateEvaluationError("PREDICATE_SCALAR_KIND_MISMATCH")
        if not any(_same_scalar(value, existing, scalar_kind) for existing in normalized):
            normalized.append(value)
    return normalized


def evaluate_compare(predicate: Mapping[str, object], lookup: Mapping[str, object]):
    state = lookup.get("state")
    if state == "ABSENT":
        return truth("INDETERMINATE", {"PREDICATE_VALUE_ABSENT"})
    if state == "UNRESOLVED":
        return truth("INDETERMINATE", {"PREDICATE_VALUE_UNRESOLVED"})
    if state != "PRESENT":
        raise PredicateEvaluationError("PREDICATE_LOOKUP_STATE_INVALID")

    declared = predicate.get("scalar_kind")
    actual = lookup.get("scalar_kind") or _scalar_kind(lookup.get("value"))
    if declared != actual:
        return failure("PREDICATE_SCALAR_KIND_MISMATCH")

    op = predicate.get("op")
    expected = predicate.get("value")
    actual_value = lookup.get("value")
    if declared == "number":
        cmp = compare_decimal(actual_value, expected)
        result = {
            "eq": cmp == 0,
            "ne": cmp != 0,
            "lt": cmp < 0,
            "le": cmp <= 0,
            "gt": cmp > 0,
            "ge": cmp >= 0,
        }.get(op)
    elif declared in {"string", "boolean"}:
        if op not in {"eq", "ne"}:
            return failure("PREDICATE_OPERATOR_KIND_MISMATCH")
        equal = actual_value == expected
        result = equal if op == "eq" else not equal
    else:
        return failure("PREDICATE_SCALAR_KIND_INVALID")
    if result is None:
        return failure("PREDICATE_OPERATOR_INVALID")
    return truth("TRUE" if result else "FALSE")


def evaluate_membership(predicate: Mapping[str, object], lookup: Mapping[str, object]):
    state = lookup.get("state")
    if state == "ABSENT":
        return truth("INDETERMINATE", {"PREDICATE_VALUE_ABSENT"})
    if state == "UNRESOLVED":
        return truth("INDETERMINATE", {"PREDICATE_VALUE_UNRESOLVED"})
    if state != "PRESENT":
        raise PredicateEvaluationError("PREDICATE_LOOKUP_STATE_INVALID")

    declared = predicate.get("scalar_kind")
    actual = lookup.get("scalar_kind") or _scalar_kind(lookup.get("value"))
    if declared != actual:
        return failure("PREDICATE_SCALAR_KIND_MISMATCH")

    actual_value = lookup.get("value")
    values = predicate.get("values")
    if not isinstance(values, list):
        raise PredicateEvaluationError("PREDICATE_MEMBERSHIP_VALUES_INVALID")
    present = any(_same_scalar(actual_value, candidate, declared) for candidate in values)
    return truth("TRUE" if present else "FALSE")


def evaluate_exists(lookup: Mapping[str, object]):
    state = lookup.get("state")
    if state == "PRESENT":
        return truth("TRUE")
    if state == "ABSENT":
        return truth("FALSE")
    if state == "UNRESOLVED":
        return truth("INDETERMINATE", {"PREDICATE_REFERENCE_VALUE_UNRESOLVED"})
    raise PredicateEvaluationError("PREDICATE_LOOKUP_STATE_INVALID")


def _boolean_failure(children):
    codes = set()
    for child in children:
        if child.get("kind") == "failure":
            codes.update(child.get("codes", ()))
    return failure(*codes) if codes else None


def evaluate_boolean(kind: str, children: Sequence[Mapping[str, object]]):
    if kind == "not":
        if len(children) != 1:
            raise PredicateEvaluationError("PREDICATE_NOT_ARITY_INVALID")
        child = children[0]
        if child.get("kind") == "failure":
            return failure(*child.get("codes", ()))
        value = child.get("truth")
        return truth(
            {"TRUE": "FALSE", "FALSE": "TRUE", "INDETERMINATE": "INDETERMINATE"}[value],
            child.get("diagnostics", ()),
        )

    if kind not in {"and", "or"} or not children:
        raise PredicateEvaluationError("PREDICATE_BOOLEAN_OPERANDS_INVALID")

    failed = _boolean_failure(children)
    if failed is not None:
        return failed

    values = [child.get("truth") for child in children]
    if kind == "and":
        if "FALSE" in values:
            return truth("FALSE")
        if "INDETERMINATE" in values:
            diagnostics = set().union(*(child.get("diagnostics", ()) for child in children if child.get("truth") == "INDETERMINATE"))
            return truth("INDETERMINATE", diagnostics)
        return truth("TRUE")

    if "TRUE" in values:
        return truth("TRUE")
    if "INDETERMINATE" in values:
        diagnostics = set().union(*(child.get("diagnostics", ()) for child in children if child.get("truth") == "INDETERMINATE"))
        return truth("INDETERMINATE", diagnostics)
    return truth("FALSE")


def evaluate_predicate(
    predicate: Mapping[str, object],
    bindings: Sequence[Mapping[str, object]],
    snapshot: Mapping[str, Mapping[str, object]],
):
    kind = predicate.get("predicate")
    if kind in {"compare", "membership", "exists"}:
        lookup = lookup_reference(predicate.get("ref", {}), bindings, snapshot)
        if kind == "compare":
            return evaluate_compare(predicate, lookup)
        if kind == "membership":
            return evaluate_membership(predicate, lookup)
        return evaluate_exists(lookup)
    if kind in {"and", "or"}:
        children = [evaluate_predicate(child, bindings, snapshot) for child in predicate.get("operands", [])]
        return evaluate_boolean(kind, children)
    if kind == "not":
        child = evaluate_predicate(predicate.get("operand", {}), bindings, snapshot)
        return evaluate_boolean("not", [child])
    raise PredicateEvaluationError("PREDICATE_KIND_INVALID")


def activate_conditional(predicate_result: Mapping[str, object], consequents: Sequence[Mapping[str, object]]):
    if not consequents:
        raise PredicateEvaluationError("CONDITIONAL_CONSEQUENT_REQUIRED")
    if predicate_result.get("kind") == "failure":
        return {"state": "FAIL", "codes": predicate_result.get("codes", frozenset()), "constraints": []}
    value = predicate_result.get("truth")
    if value == "TRUE":
        return {"state": "ACTIVE", "constraints": list(consequents)}
    if value == "FALSE":
        return {"state": "INACTIVE", "constraints": []}
    if value == "INDETERMINATE":
        return {
            "state": "INDETERMINATE",
            "diagnostics": predicate_result.get("diagnostics", frozenset()),
            "constraints": [],
        }
    raise PredicateEvaluationError("CONDITIONAL_PREDICATE_RESULT_INVALID")


def aggregate_validation_states(
    ordinary_states: Sequence[str],
    activation_states: Sequence[str],
    *,
    evaluation_failure_codes=(),
) -> str:
    allowed_ordinary = {"PASS", "FAIL", "INDETERMINATE"}
    if any(state not in allowed_ordinary for state in ordinary_states):
        raise PredicateEvaluationError("VALIDATION_ORDINARY_STATE_UNSUPPORTED")
    allowed_activation = {"ACTIVE", "INACTIVE", "INDETERMINATE", "FAIL"}
    if any(state not in allowed_activation for state in activation_states):
        raise PredicateEvaluationError("VALIDATION_ACTIVATION_STATE_INVALID")
    if evaluation_failure_codes or "FAIL" in ordinary_states or "FAIL" in activation_states:
        return "FAIL"
    if "INDETERMINATE" in ordinary_states or "INDETERMINATE" in activation_states:
        return "INDETERMINATE"
    return "PASS"
