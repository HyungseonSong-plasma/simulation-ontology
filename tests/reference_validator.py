"""Non-normative SOL v0.1 Phase-1 contract-mechanics reference harness.

This module is intentionally backend-neutral. It exists to exercise the deterministic
contracts in Proposed ADR-0011 and ADR-0012; it is not the SOL implementation.
"""
from __future__ import annotations

from dataclasses import dataclass
from itertools import combinations
from typing import Any, Callable, Dict, Iterable, List, Optional, Sequence, Tuple

PASS = "PASS"
FAIL = "FAIL"
BLOCKED = "BLOCKED"
INDETERMINATE = "INDETERMINATE"

_PRECEDENCE = {PASS: 0, INDETERMINATE: 1, BLOCKED: 2, FAIL: 3}
_REPRESENTABILITY_RANK = {"exact": 0, "transformed": 1, "lossy": 2, "unsupported": 3}


def aggregate_decisions(decisions: Iterable[str]) -> str:
    values = list(decisions)
    if not values:
        return PASS
    unknown = [d for d in values if d not in _PRECEDENCE]
    if unknown:
        raise ValueError(f"UNKNOWN_DECISION:{unknown[0]}")
    return max(values, key=lambda d: _PRECEDENCE[d])


def validate_closed_envelope(obj: Dict[str, Any], required: Sequence[str]) -> Tuple[str, str]:
    keys = set(obj)
    req = set(required)
    if not req.issubset(keys):
        return FAIL, "REQUIRED_FIELD_MISSING"
    if keys != req:
        return FAIL, "UNEXPECTED_TOP_LEVEL_FIELD"
    return PASS, "OK"


def normalize_qrc_bounds(constraint: Dict[str, Any]) -> Tuple[str, str, Optional[Tuple[int, Optional[int]]]]:
    vals = {k: constraint.get(k) for k in ("min", "max", "exact") if k in constraint}
    for value in vals.values():
        if isinstance(value, bool) or not isinstance(value, int) or value < 0:
            return FAIL, "QRC_BOUND_INVALID", None
    lower_candidates = [0]
    upper_candidates: List[int] = []
    if "min" in vals:
        lower_candidates.append(vals["min"])
    if "max" in vals:
        upper_candidates.append(vals["max"])
    if "exact" in vals:
        lower_candidates.append(vals["exact"])
        upper_candidates.append(vals["exact"])
    lower = max(lower_candidates)
    upper = min(upper_candidates) if upper_candidates else None
    if upper is not None and lower > upper:
        return FAIL, "QRC_EMPTY_INTERVAL", None
    return PASS, "OK", (lower, upper)


def _is_subtype(actual: str, target: str, closure: Dict[str, Sequence[str]]) -> bool:
    return actual == target or target in set(closure.get(actual, ()))


def _qrc_diagnostics(
    constraint: Dict[str, Any],
    snapshot: Dict[str, Any],
    interval: Optional[Tuple[int, Optional[int]]] = None,
    count: Optional[int] = None,
    counted: Optional[Sequence[str]] = None,
) -> Dict[str, Any]:
    qualifier = constraint.get("qualifier", {}).get("target_type")
    return {
        "source_identity": constraint.get("source_identity"),
        "relation_identity": constraint.get("relation"),
        "qualifier_type_identity": qualifier,
        "interval": list(interval) if interval else None,
        "count": count,
        "counted": sorted(counted or []),
        "ontology_package_versions": snapshot.get("ontology_package_versions"),
    }


def evaluate_qrc(constraint: Dict[str, Any], snapshot: Dict[str, Any]) -> Dict[str, Any]:
    if not snapshot.get("complete", False):
        return {
            "decision": BLOCKED,
            "code": "QRC_CLOSED_SNAPSHOT_REQUIRED",
            "diagnostics": _qrc_diagnostics(constraint, snapshot),
        }

    qualifier = constraint.get("qualifier", {}).get("target_type")
    if not isinstance(qualifier, str) or not qualifier:
        return {
            "decision": FAIL,
            "code": "QRC_QUALIFIER_MALFORMED",
            "diagnostics": _qrc_diagnostics(constraint, snapshot),
        }

    candidates = snapshot.get("qualifier_candidates")
    if candidates is not None:
        matches = list(candidates.get(qualifier, []))
        if len(matches) > 1:
            return {
                "decision": FAIL,
                "code": "QRC_QUALIFIER_AMBIGUOUS",
                "diagnostics": _qrc_diagnostics(constraint, snapshot),
            }
        if len(matches) == 0:
            if not snapshot.get("type_provider_resolved", True):
                return {
                    "decision": BLOCKED,
                    "code": "QRC_QUALIFIER_EVIDENCE_MISSING",
                    "diagnostics": _qrc_diagnostics(constraint, snapshot),
                }
            return {
                "decision": FAIL,
                "code": "QRC_QUALIFIER_NOT_TYPE",
                "diagnostics": _qrc_diagnostics(constraint, snapshot),
            }
        qualifier = matches[0]

    available_types = set(snapshot.get("available_types", []))
    if qualifier not in available_types:
        if snapshot.get("type_provider_resolved", True):
            return {
                "decision": FAIL,
                "code": "QRC_QUALIFIER_NOT_TYPE",
                "diagnostics": _qrc_diagnostics(constraint, snapshot),
            }
        return {
            "decision": BLOCKED,
            "code": "QRC_QUALIFIER_EVIDENCE_MISSING",
            "diagnostics": _qrc_diagnostics(constraint, snapshot),
        }

    if not snapshot.get("subtype_resolver_available", True):
        return {
            "decision": FAIL,
            "code": "QRC_SUBTYPE_RESOLVER_UNAVAILABLE",
            "diagnostics": _qrc_diagnostics(constraint, snapshot),
        }

    status, code, interval = normalize_qrc_bounds(constraint)
    if status != PASS:
        return {
            "decision": status,
            "code": code,
            "diagnostics": _qrc_diagnostics(constraint, snapshot),
        }

    lower, upper = interval  # type: ignore[misc]
    edges = snapshot.get("edges", [])
    target_types = snapshot.get("target_types", {})
    closure = snapshot.get("subtype_closure", {})
    inconsistent = set(snapshot.get("inconsistent_target_ids", []))
    if inconsistent.intersection(edges):
        return {
            "decision": FAIL,
            "code": "QRC_TARGET_TYPE_INCONSISTENT",
            "diagnostics": _qrc_diagnostics(constraint, snapshot, interval),
        }

    counted = set()
    for target_id in edges:
        types = target_types.get(target_id, [])
        if any(_is_subtype(t, qualifier, closure) for t in types):
            counted.add(target_id)
    q = len(counted)
    ok = q >= lower and (upper is None or q <= upper)
    result = {
        "decision": PASS if ok else FAIL,
        "code": "OK" if ok else "QRC_COUNT_OUT_OF_RANGE",
        "interval": [lower, upper],
        "count": q,
        "counted": sorted(counted),
    }
    result["diagnostics"] = _qrc_diagnostics(constraint, snapshot, interval, q, sorted(counted))
    return result


def _binding_tuple(binding: Dict[str, str]) -> Tuple[Tuple[str, str], ...]:
    return tuple(sorted(binding.items()))


def comparator_key(
    effect_a: Dict[str, Any],
    effect_b: Dict[str, Any],
    purpose: str,
    component_bindings: Dict[str, Dict[str, str]],
    orchestration_component_id: Optional[str],
    symmetric: bool = True,
) -> Tuple[Any, ...]:
    ca = effect_a["resource_component_id"]
    cb = effect_b["resource_component_id"]
    ka = effect_a["kind"]
    kb = effect_b["kind"]
    if ca not in component_bindings or cb not in component_bindings:
        raise ValueError("EFFECT_COMPONENT_SCOPE_UNRESOLVED")
    if ca == cb:
        binding = _binding_tuple(component_bindings[ca])
        kinds = [ka, kb]
        if symmetric:
            kinds.sort()
        return ("local", binding, purpose, kinds[0], kinds[1])
    if orchestration_component_id is None:
        raise ValueError("TARGET_ORCHESTRATION_MISSING")
    if orchestration_component_id not in component_bindings:
        raise ValueError("TARGET_ORCHESTRATION_UNRESOLVED")
    left = (ca, _binding_tuple(component_bindings[ca]), ka)
    right = (cb, _binding_tuple(component_bindings[cb]), kb)
    if symmetric and right < left:
        left, right = right, left
    orch = _binding_tuple(component_bindings[orchestration_component_id])
    return ("cross_component", left, right, orch, purpose)


def resolve_comparator(registry: Sequence[Dict[str, Any]], key: Tuple[Any, ...]) -> Dict[str, Any]:
    matches = [r for r in registry if r["key"] == key]
    if len(matches) == 0:
        return {"decision": FAIL, "code": "COMPARATOR_MISSING"}
    if len(matches) > 1:
        return {"decision": FAIL, "code": "COMPARATOR_AMBIGUOUS"}
    return {
        "decision": PASS,
        "code": "OK",
        "comparator": matches[0]["id"],
        "version": matches[0].get("version"),
    }


def effect_pairs(effects: Sequence[Dict[str, Any]]) -> List[Tuple[str, str]]:
    ids = sorted(e["id"] for e in effects)
    return list(combinations(ids, 2))


def retained_pairs(effects: Sequence[Dict[str, Any]], proven_disjoint: Iterable[Tuple[str, str]]) -> List[Tuple[str, str]]:
    disjoint = {tuple(sorted(p)) for p in proven_disjoint}
    return [p for p in effect_pairs(effects) if tuple(sorted(p)) not in disjoint]


def validate_effect_coverage(
    declared: Sequence[Dict[str, Any]],
    derived: Sequence[Dict[str, Any]],
    bookkeeping_kinds: Sequence[str],
    equivalence: Optional[Callable[[Dict[str, Any], Dict[str, Any]], bool]] = None,
) -> Dict[str, Any]:
    """Validate one deterministic synthetic effect-equivalence procedure.

    The default comparator uses canonical fingerprints. Backend truth is intentionally
    deferred to actual adapter execution fixtures.
    """
    equivalent = equivalence or (lambda a, b: a.get("fingerprint") == b.get("fingerprint"))
    unmatched_declared = list(declared)
    for effect in derived:
        matches = [d for d in unmatched_declared if equivalent(d, effect)]
        if len(matches) == 1:
            unmatched_declared.remove(matches[0])
            continue
        if len(matches) > 1:
            return {"decision": FAIL, "code": "EFFECT_EQUIVALENCE_AMBIGUOUS"}
        if effect.get("kind") not in bookkeeping_kinds:
            return {"decision": FAIL, "code": "EXECUTABLE_EFFECT_MISMATCH"}
    if unmatched_declared:
        return {"decision": FAIL, "code": "EXECUTABLE_EFFECT_MISMATCH"}
    return {"decision": PASS, "code": "OK"}


def validate_idempotency(contract: Dict[str, Any], pre_state_available: bool = True) -> Dict[str, Any]:
    mode = contract.get("mode")
    if mode == "state_independent":
        if not contract.get("adapter_guarantee", False):
            return {"decision": FAIL, "code": "IDEMPOTENCY_GUARANTEE_MISSING"}
        return {"decision": PASS, "code": "OK"}
    if mode == "state_dependent":
        match_count = int(contract.get("procedure_match_count", 1 if contract.get("procedure_bound", False) else 0))
        if match_count == 0:
            return {"decision": FAIL, "code": "IDEMPOTENCY_PROCEDURE_MISSING"}
        if match_count > 1:
            return {"decision": FAIL, "code": "IDEMPOTENCY_PROCEDURE_AMBIGUOUS"}
        if not pre_state_available:
            return {"decision": BLOCKED, "code": "IDEMPOTENCY_PRESTATE_MISSING"}
        result = contract.get("procedure_result")
        if result == "idempotent":
            return {"decision": PASS, "code": "OK"}
        if result == "indeterminate":
            return {"decision": INDETERMINATE, "code": "IDEMPOTENCY_INDETERMINATE"}
        if result == "not-idempotent":
            return {"decision": FAIL, "code": "NON_IDEMPOTENT"}
        return {"decision": FAIL, "code": "IDEMPOTENCY_RESULT_INVALID"}
    return {"decision": FAIL, "code": "IDEMPOTENCY_CONTRACT_INVALID"}


def validate_action_graph(actions: Sequence[Dict[str, Any]], external_handles: Sequence[str] = ()) -> Dict[str, Any]:
    producers: Dict[str, List[str]] = {}
    for action in actions:
        for handle in action.get("produces", []):
            producers.setdefault(handle, []).append(action["id"])
    for handle, ids in producers.items():
        if len(ids) > 1:
            return {"decision": FAIL, "code": "DUPLICATE_PRODUCER_UNRESOLVED", "handle": handle}
    external = set(external_handles)
    edges: Dict[str, set[str]] = {a["id"]: set() for a in actions}
    for action in actions:
        for req in action.get("requires", []):
            if req in external:
                continue
            if req not in producers:
                return {"decision": FAIL, "code": "UNRESOLVED_PREREQUISITE", "handle": req}
            edges[producers[req][0]].add(action["id"])
        for pred in action.get("must_precede", []):
            if pred not in edges:
                return {"decision": FAIL, "code": "ORDER_TARGET_UNKNOWN", "action_id": pred}
            edges[action["id"]].add(pred)
    visiting: set[str] = set()
    visited: set[str] = set()

    def dfs(node: str) -> bool:
        if node in visiting:
            return True
        if node in visited:
            return False
        visiting.add(node)
        for nxt in sorted(edges[node]):
            if dfs(nxt):
                return True
        visiting.remove(node)
        visited.add(node)
        return False

    if any(dfs(node) for node in sorted(edges)):
        return {"decision": FAIL, "code": "ACTION_DEPENDENCY_CYCLE"}
    return {"decision": PASS, "code": "OK", "edges": sorted((a, b) for a, bs in edges.items() for b in bs)}


@dataclass(frozen=True)
class EvaluationRevision:
    revision_id: str
    state: str
    prior_revision_id: Optional[str] = None
    target_snapshot_fingerprint: Optional[str] = None


def finish_revision(revision_id: str, terminal_state: str, target_snapshot_fingerprint: Optional[str] = None) -> EvaluationRevision:
    if terminal_state not in {"blocked", "indeterminate", "complete"}:
        raise ValueError("INVALID_TERMINAL_STATE")
    return EvaluationRevision(
        revision_id=revision_id,
        state=terminal_state,
        target_snapshot_fingerprint=target_snapshot_fingerprint,
    )


def reevaluate(
    previous: EvaluationRevision,
    new_revision_id: str,
    terminal_state: str,
    target_snapshot_fingerprint: Optional[str] = None,
) -> EvaluationRevision:
    if terminal_state not in {"blocked", "indeterminate", "complete"}:
        raise ValueError("INVALID_TERMINAL_STATE")
    return EvaluationRevision(
        new_revision_id,
        terminal_state,
        previous.revision_id,
        target_snapshot_fingerprint,
    )


def aggregate_representability(validation_decision: str, evaluation_state: str, outcomes: Sequence[str]) -> Optional[str]:
    if validation_decision != PASS or evaluation_state != "complete":
        return None
    unknown = [o for o in outcomes if o not in _REPRESENTABILITY_RANK]
    if unknown:
        raise ValueError(f"UNKNOWN_REPRESENTABILITY:{unknown[0]}")
    return max(outcomes or ["exact"], key=lambda x: _REPRESENTABILITY_RANK[x])


def canonical_loss_key(source_identity: str, obligation: str, aspect_identity: str) -> str:
    return "|".join((source_identity, obligation, aspect_identity))


def execution_permission(
    outcome: Optional[str],
    losses: Sequence[str],
    policies: Sequence[Dict[str, str]],
    independent_blocker: bool = False,
) -> str:
    if independent_blocker or outcome is None or outcome == "unsupported":
        return "prohibited"
    if outcome in {"exact", "transformed"}:
        return "permitted"
    if outcome == "lossy":
        for loss in sorted(set(losses)):
            applicable = [p["decision"] for p in policies if p["loss_key"] == loss]
            if not applicable or any(d != "allow" for d in applicable):
                return "prohibited"
        return "permitted"
    return "prohibited"


def _component_map(items: Sequence[Dict[str, Any]]) -> Tuple[Optional[Dict[str, Dict[str, Any]]], Optional[Dict[str, Any]]]:
    mapping: Dict[str, Dict[str, Any]] = {}
    for item in items:
        cid = item.get("component_id")
        if not isinstance(cid, str) or not cid:
            return None, {"decision": FAIL, "code": "TARGET_COMPONENT_SCOPE_MISSING"}
        if cid in mapping:
            return None, {"decision": FAIL, "code": "TARGET_COMPONENT_SCOPE_AMBIGUOUS", "component_id": cid}
        mapping[cid] = item
    return mapping, None


def validate_backend_target(requirement: Dict[str, Any], resolved: Dict[str, Any]) -> Dict[str, Any]:
    required, error = _component_map(requirement.get("components", []))
    if error:
        return error
    assert required is not None
    if not required:
        return {"decision": FAIL, "code": "TARGET_COMPONENT_SCOPE_MISSING"}

    orch = requirement.get("orchestration_component_id")
    if len(required) > 1 and orch not in required:
        return {"decision": FAIL, "code": "TARGET_ORCHESTRATION_MISSING"}

    runtime, error = _component_map(resolved.get("components", []))
    if error:
        return error
    assert runtime is not None

    unsupported: List[str] = []
    for cid, component in sorted(required.items()):
        if cid not in runtime:
            return {"decision": BLOCKED, "code": "TARGET_COMPONENT_UNRESOLVED", "component_id": cid}
        observed = runtime[cid]

        required_adapter = component.get("adapter_contract", {})
        actual_adapter = observed.get("adapter_contract")
        if actual_adapter is None:
            return {"decision": BLOCKED, "code": "TARGET_ADAPTER_UNRESOLVED", "component_id": cid}
        if required_adapter and actual_adapter != required_adapter:
            return {"decision": FAIL, "code": "TARGET_ADAPTER_MISMATCH", "component_id": cid}

        actual_release = observed.get("actual_release")
        if not actual_release:
            return {"decision": BLOCKED, "code": "TARGET_RELEASE_UNRESOLVED", "component_id": cid}
        allowed_releases = component.get("release_compatibility", {}).get("allowed")
        if allowed_releases is not None and actual_release not in set(allowed_releases):
            return {"decision": FAIL, "code": "TARGET_RELEASE_MISMATCH", "component_id": cid}

        required_caps = set(component.get("required_capabilities", []))
        observed_caps = observed.get("capabilities")
        if observed_caps is None:
            return {"decision": BLOCKED, "code": "TARGET_CAPABILITY_EVIDENCE_MISSING", "component_id": cid}
        absent_caps = required_caps - set(observed_caps)
        if absent_caps:
            unsupported.extend(f"{cid}:capability:{x}" for x in sorted(absent_caps))

        formulations = component.get("formulation_bindings", [])
        keys = [f.get("sol_obligation") for f in formulations]
        if None in keys:
            return {"decision": FAIL, "code": "FORMULATION_BINDING_MISSING", "component_id": cid}
        if len(keys) != len(set(keys)):
            return {"decision": FAIL, "code": "FORMULATION_BINDING_AMBIGUOUS", "component_id": cid}
        available_formulations = observed.get("available_formulations")
        if formulations and available_formulations is None:
            return {"decision": BLOCKED, "code": "FORMULATION_EVIDENCE_MISSING", "component_id": cid}
        for binding in formulations:
            backend_formulation = binding.get("backend_formulation")
            if not backend_formulation:
                return {"decision": FAIL, "code": "FORMULATION_BINDING_MISSING", "component_id": cid}
            if backend_formulation not in set(available_formulations or []):
                unsupported.append(f"{cid}:formulation:{backend_formulation}")

    if len(required) > 1:
        assert orch is not None
        orchestration_runtime = runtime.get(orch)
        if orchestration_runtime is None:
            return {"decision": BLOCKED, "code": "TARGET_ORCHESTRATION_UNRESOLVED"}
        if not orchestration_runtime.get("orchestration_capable", False):
            unsupported.append(f"{orch}:orchestration")

    if unsupported:
        return {
            "decision": PASS,
            "code": "TARGET_AUTHORITATIVE_ABSENCE",
            "representability": "unsupported",
            "missing": sorted(unsupported),
        }
    return {"decision": PASS, "code": "OK"}
