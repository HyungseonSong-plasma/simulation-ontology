"""Minimal design-stage semantic helper for ADR-0022 Compatibility constraints."""

from __future__ import annotations

from collections import Counter
from typing import Iterable, Mapping, Sequence


class CompatibilityConstraintError(ValueError):
    """Deterministic ADR-0022 semantic/compiler diagnostic."""


def _ref_key(ref: Mapping[str, object]) -> tuple[str, str]:
    space = ref.get("identity_space")
    identifier = ref.get("id")
    if space not in {"schema", "model_instance"}:
        raise CompatibilityConstraintError("COMPATIBILITY_IDENTITY_SPACE_INVALID")
    if not isinstance(identifier, str) or not identifier:
        raise CompatibilityConstraintError("COMPATIBILITY_REFERENCE_ID_INVALID")
    return str(space), identifier


def resolve_criterion(
    criterion_id: str,
    definitions: Sequence[Mapping[str, object]],
) -> Mapping[str, object]:
    matches = [d for d in definitions if d.get("id") == criterion_id]
    if not matches:
        raise CompatibilityConstraintError("COMPATIBILITY_CRITERION_UNRESOLVED")
    if len(matches) != 1:
        raise CompatibilityConstraintError("COMPATIBILITY_CRITERION_AMBIGUOUS")
    criterion = matches[0]
    if criterion.get("family") != "compatibility":
        raise CompatibilityConstraintError("COMPATIBILITY_CRITERION_FAMILY_INVALID")
    if criterion.get("arity") != 2:
        raise CompatibilityConstraintError("COMPATIBILITY_CRITERION_ARITY_INVALID")
    if criterion.get("operand_order") not in {"ordered", "symmetric"}:
        raise CompatibilityConstraintError("COMPATIBILITY_OPERAND_ORDER_INVALID")
    evaluator_contract = criterion.get("evaluator_contract")
    if not isinstance(evaluator_contract, str) or not evaluator_contract:
        raise CompatibilityConstraintError("COMPATIBILITY_EVALUATOR_CONTRACT_REQUIRED")
    return criterion


def normalize_payload(
    authored: Mapping[str, object],
    *,
    canonical_criterion: str | None,
    left_ref: Mapping[str, object] | None,
    right_ref: Mapping[str, object] | None,
) -> dict[str, object]:
    if authored.get("type") != "compatibility":
        raise CompatibilityConstraintError("COMPATIBILITY_KIND_INVALID")
    expect = authored.get("expect")
    if expect not in {"compatible", "incompatible"}:
        raise CompatibilityConstraintError("COMPATIBILITY_EXPECT_INVALID")
    if not canonical_criterion:
        raise CompatibilityConstraintError("COMPATIBILITY_CRITERION_UNRESOLVED")
    if left_ref is None or right_ref is None:
        raise CompatibilityConstraintError("COMPATIBILITY_OPERAND_UNRESOLVED")
    _ref_key(left_ref)
    _ref_key(right_ref)
    return {
        "type": "compatibility",
        "criterion": canonical_criterion,
        "left": dict(left_ref),
        "right": dict(right_ref),
        "expect": expect,
    }


def obligation_key(
    payload: Mapping[str, object],
    criterion: Mapping[str, object],
):
    criterion_id = payload.get("criterion")
    if criterion_id != criterion.get("id"):
        raise CompatibilityConstraintError("COMPATIBILITY_CRITERION_BINDING_MISMATCH")
    left = _ref_key(payload["left"])
    right = _ref_key(payload["right"])
    if criterion.get("operand_order") == "ordered":
        return (criterion_id, "ordered", left, right)
    if criterion.get("operand_order") == "symmetric":
        # Semantic equality uses a two-member multiset, not lexical sorting.
        counts = Counter((left, right))
        return (criterion_id, "symmetric", frozenset(counts.items()))
    raise CompatibilityConstraintError("COMPATIBILITY_OPERAND_ORDER_INVALID")


def compose_obligations(
    obligations: Iterable[tuple[Mapping[str, object], Mapping[str, object]]]
) -> list[dict[str, object]]:
    by_key: dict[object, dict[str, object]] = {}
    for payload, criterion in obligations:
        key = obligation_key(payload, criterion)
        existing = by_key.get(key)
        if existing is None:
            by_key[key] = dict(payload)
            continue
        if existing["expect"] != payload["expect"]:
            raise CompatibilityConstraintError("COMPATIBILITY_OBLIGATION_CONFLICT")
    return list(by_key.values())


def evaluate_obligation(
    payload: Mapping[str, object],
    *,
    semantic_context_resolved: bool,
    binary_result: str | None,
) -> str:
    if not semantic_context_resolved:
        if binary_result is not None:
            raise CompatibilityConstraintError("COMPATIBILITY_BINARY_RESULT_WITHOUT_CONTEXT")
        return "INDETERMINATE"
    if binary_result not in {"compatible", "incompatible"}:
        raise CompatibilityConstraintError("COMPATIBILITY_BINARY_RESULT_REQUIRED")
    expect = payload.get("expect")
    if expect not in {"compatible", "incompatible"}:
        raise CompatibilityConstraintError("COMPATIBILITY_EXPECT_INVALID")
    return "PASS" if expect == binary_result else "FAIL"


def aggregate_states(states: Sequence[str]) -> str:
    allowed = {"PASS", "FAIL", "INDETERMINATE"}
    if any(state not in allowed for state in states):
        raise CompatibilityConstraintError("COMPATIBILITY_STATE_INVALID")
    if not states:
        return "PASS"
    if "FAIL" in states:
        return "FAIL"
    if "INDETERMINATE" in states:
        return "INDETERMINATE"
    return "PASS"
