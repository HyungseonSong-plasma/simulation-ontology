"""Non-normative SOL v0.1 Phase-1 contract-mechanics reference harness v0.2.

The harness exercises Proposed ADR-0011/0012 determinism. Backend behavior is
represented only by versioned synthetic adapter fixtures and is not backend truth.
"""
from __future__ import annotations

import json
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
    resolved_qualifier: Optional[str] = None,
    interval: Optional[Tuple[int, Optional[int]]] = None,
    count: Optional[int] = None,
    counted: Optional[Sequence[str]] = None,
) -> Dict[str, Any]:
    return {
        "source_identity": constraint.get("source_identity"),
        "relation_identity": constraint.get("relation"),
        "qualifier_type_identity": resolved_qualifier or constraint.get("qualifier", {}).get("target_type"),
        "interval": list(interval) if interval else None,
        "count": count,
        "counted": sorted(counted or []),
        "ontology_package_versions": snapshot.get("ontology_package_versions"),
    }


def evaluate_qrc(constraint: Dict[str, Any], snapshot: Dict[str, Any]) -> Dict[str, Any]:
    if not snapshot.get("complete", False):
        return {"decision": BLOCKED, "code": "QRC_CLOSED_SNAPSHOT_REQUIRED", "diagnostics": _qrc_diagnostics(constraint, snapshot)}

    authored_qualifier = constraint.get("qualifier", {}).get("target_type")
    if not isinstance(authored_qualifier, str) or not authored_qualifier:
        return {"decision": FAIL, "code": "QRC_QUALIFIER_MALFORMED", "diagnostics": _qrc_diagnostics(constraint, snapshot)}

    qualifier = authored_qualifier
    candidates = snapshot.get("qualifier_candidates")
    if candidates is not None:
        matches = list(candidates.get(authored_qualifier, []))
        if len(matches) > 1:
            return {"decision": FAIL, "code": "QRC_QUALIFIER_AMBIGUOUS", "diagnostics": _qrc_diagnostics(constraint, snapshot)}
        if len(matches) == 0:
            if not snapshot.get("type_provider_resolved", True):
                return {"decision": BLOCKED, "code": "QRC_QUALIFIER_EVIDENCE_MISSING", "diagnostics": _qrc_diagnostics(constraint, snapshot)}
            return {"decision": FAIL, "code": "QRC_QUALIFIER_NOT_TYPE", "diagnostics": _qrc_diagnostics(constraint, snapshot)}
        qualifier = matches[0]

    available_types = set(snapshot.get("available_types", []))
    if qualifier not in available_types:
        if snapshot.get("type_provider_resolved", True):
            return {"decision": FAIL, "code": "QRC_QUALIFIER_NOT_TYPE", "diagnostics": _qrc_diagnostics(constraint, snapshot, qualifier)}
        return {"decision": BLOCKED, "code": "QRC_QUALIFIER_EVIDENCE_MISSING", "diagnostics": _qrc_diagnostics(constraint, snapshot, qualifier)}

    if not snapshot.get("subtype_resolver_available", True):
        return {"decision": FAIL, "code": "QRC_SUBTYPE_RESOLVER_UNAVAILABLE", "diagnostics": _qrc_diagnostics(constraint, snapshot, qualifier)}

    if not snapshot.get("identity_resolution_complete", True):
        return {"decision": BLOCKED, "code": "QRC_IDENTITY_EVIDENCE_MISSING", "diagnostics": _qrc_diagnostics(constraint, snapshot, qualifier)}
    if snapshot.get("ambiguous_target_ids"):
        return {"decision": FAIL, "code": "QRC_IDENTITY_AMBIGUOUS", "diagnostics": _qrc_diagnostics(constraint, snapshot, qualifier)}

    status, code, interval = normalize_qrc_bounds(constraint)
    if status != PASS:
        return {"decision": status, "code": code, "diagnostics": _qrc_diagnostics(constraint, snapshot, qualifier)}

    lower, upper = interval  # type: ignore[misc]
    edges = snapshot.get("edges", [])
    canonical_map = snapshot.get("canonical_target_ids", {})
    canonical_edges = [canonical_map.get(serialized_id, serialized_id) for serialized_id in edges]
    target_types = snapshot.get("target_types", {})
    closure = snapshot.get("subtype_closure", {})
    inconsistent = set(snapshot.get("inconsistent_target_ids", []))
    if inconsistent.intersection(canonical_edges):
        return {"decision": FAIL, "code": "QRC_TARGET_TYPE_INCONSISTENT", "diagnostics": _qrc_diagnostics(constraint, snapshot, qualifier, interval)}

    counted = set()
    for target_id in canonical_edges:
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
    result["diagnostics"] = _qrc_diagnostics(constraint, snapshot, qualifier, interval, q, sorted(counted))
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


def registry_entry_key(entry: Dict[str, Any]) -> Tuple[Any, ...]:
    directionality = entry.get("directionality", "symmetric")
    symmetric = directionality == "symmetric"
    purpose = entry["comparison_purpose"]
    lk, rk = entry["left_kind"], entry["right_kind"]
    if entry["scope"] == "local":
        binding = entry["component_binding"]
        kinds = [lk, rk]
        if symmetric:
            kinds.sort()
        return ("local", _binding_tuple(binding), purpose, kinds[0], kinds[1])
    left_binding = entry["left_component_binding"]
    right_binding = entry["right_component_binding"]
    left = (left_binding["component_id"], _binding_tuple(left_binding), lk)
    right = (right_binding["component_id"], _binding_tuple(right_binding), rk)
    if symmetric and right < left:
        left, right = right, left
    orch = _binding_tuple(entry["orchestration_component_binding"])
    return ("cross_component", left, right, orch, purpose)


def resolve_comparator(registry: Sequence[Dict[str, Any]], key: Tuple[Any, ...]) -> Dict[str, Any]:
    try:
        matches = [r for r in registry if registry_entry_key(r) == key]
    except (KeyError, TypeError):
        return {"decision": FAIL, "code": "COMPARATOR_REGISTRY_ENTRY_MALFORMED"}
    if len(matches) == 0:
        return {"decision": FAIL, "code": "COMPARATOR_MISSING"}
    if len(matches) > 1:
        return {"decision": FAIL, "code": "COMPARATOR_AMBIGUOUS"}
    return {"decision": PASS, "code": "OK", "comparator": matches[0]["id"], "version": matches[0]["version"]}


def effect_pairs(effects: Sequence[Dict[str, Any]]) -> List[Tuple[str, str]]:
    ids = [e["id"] for e in effects]
    if len(ids) != len(set(ids)):
        raise ValueError("EFFECT_ID_AMBIGUOUS")
    return list(combinations(sorted(ids), 2))


def retained_pairs(effects: Sequence[Dict[str, Any]], disjointness_evidence: Sequence[Dict[str, Any]]) -> Dict[str, Any]:
    universe = effect_pairs(effects)
    evidence_by_pair: Dict[Tuple[str, str], Dict[str, Any]] = {}
    evidence_decisions: List[str] = []
    for evidence in disjointness_evidence:
        if not isinstance(evidence, dict):
            return {"decision": FAIL, "code": "DISJOINTNESS_EVIDENCE_MALFORMED", "pairs": universe}
        required = {"effect_pair", "decision", "resource_relation", "scope_overlap", "comparator_id", "comparator_version", "target_snapshot_fingerprint", "input_fingerprints"}
        if not required.issubset(evidence):
            return {"decision": FAIL, "code": "DISJOINTNESS_EVIDENCE_MALFORMED", "pairs": universe}
        pair = tuple(sorted(evidence["effect_pair"]))
        if pair not in universe:
            return {"decision": FAIL, "code": "DISJOINTNESS_EVIDENCE_UNKNOWN_PAIR", "pairs": universe}
        if pair in evidence_by_pair:
            return {"decision": FAIL, "code": "DISJOINTNESS_EVIDENCE_AMBIGUOUS", "pairs": universe}
        evidence_by_pair[pair] = evidence
        evidence_decisions.append(evidence["decision"])
    retained: List[Tuple[str, str]] = []
    for pair in universe:
        evidence = evidence_by_pair.get(pair)
        if evidence and evidence["decision"] == PASS and evidence["resource_relation"] == "separate" and evidence["scope_overlap"] is False:
            continue
        retained.append(pair)
    return {"decision": aggregate_decisions(evidence_decisions), "code": "OK", "pairs": retained}


def synthetic_describe_effects(descriptor: Dict[str, Any], adapter_contract: Dict[str, Any]) -> Dict[str, Any]:
    descriptor_id = descriptor.get("descriptor_id")
    effects_by_descriptor = adapter_contract.get("descriptor_effects", {})
    if descriptor_id not in effects_by_descriptor:
        return {"decision": FAIL, "code": "DESCRIBE_EFFECTS_BINDING_MISSING"}
    return {"decision": PASS, "code": "OK", "effects": effects_by_descriptor[descriptor_id]}


def validate_executable_effect_coverage(
    declared: Sequence[Dict[str, Any]],
    descriptor: Dict[str, Any],
    adapter_contract: Dict[str, Any],
    equivalence: Optional[Callable[[Dict[str, Any], Dict[str, Any]], bool]] = None,
) -> Dict[str, Any]:
    described = synthetic_describe_effects(descriptor, adapter_contract)
    if described["decision"] != PASS:
        return described
    comparator = adapter_contract.get("effect_equivalence_comparator")
    if not isinstance(comparator, dict) or not comparator.get("id") or not comparator.get("version"):
        return {"decision": FAIL, "code": "EFFECT_EQUIVALENCE_COMPARATOR_MISSING"}
    bookkeeping_kinds = set(adapter_contract.get("bookkeeping_kinds", []))
    equivalent = equivalence or (lambda a, b: a.get("fingerprint") == b.get("fingerprint"))
    unmatched_declared = list(declared)
    for effect in described["effects"]:
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
    return {
        "decision": PASS,
        "code": "OK",
        "comparator_id": comparator["id"],
        "comparator_version": comparator["version"],
        "adapter_contract_id": adapter_contract.get("id"),
        "adapter_contract_version": adapter_contract.get("version"),
    }


def validate_idempotency(
    contract: Dict[str, Any],
    adapter_contract: Dict[str, Any],
    pre_state: Optional[Dict[str, Any]] = None,
) -> Dict[str, Any]:
    mode = contract.get("mode")
    if mode == "state_independent":
        descriptor_kind = contract.get("descriptor_kind")
        guarantees = set(adapter_contract.get("state_independent_idempotency_guarantees", []))
        if descriptor_kind not in guarantees:
            return {"decision": FAIL, "code": "IDEMPOTENCY_GUARANTEE_MISSING"}
        return {"decision": PASS, "code": "OK", "adapter_contract_id": adapter_contract.get("id"), "adapter_contract_version": adapter_contract.get("version")}
    if mode == "state_dependent":
        pid, pver = contract.get("procedure_id"), contract.get("procedure_version")
        matches = [p for p in adapter_contract.get("idempotency_procedures", []) if p.get("id") == pid and p.get("version") == pver]
        if len(matches) == 0:
            return {"decision": FAIL, "code": "IDEMPOTENCY_PROCEDURE_MISSING"}
        if len(matches) > 1:
            return {"decision": FAIL, "code": "IDEMPOTENCY_PROCEDURE_AMBIGUOUS"}
        if not pre_state or not pre_state.get("available", False):
            return {"decision": BLOCKED, "code": "IDEMPOTENCY_PRESTATE_MISSING"}
        result = pre_state.get("procedure_results", {}).get(f"{pid}@{pver}")
        if result == "idempotent":
            return {"decision": PASS, "code": "OK"}
        if result == "indeterminate":
            return {"decision": INDETERMINATE, "code": "IDEMPOTENCY_INDETERMINATE"}
        if result == "not-idempotent":
            return {"decision": FAIL, "code": "NON_IDEMPOTENT"}
        return {"decision": FAIL, "code": "IDEMPOTENCY_RESULT_INVALID"}
    return {"decision": FAIL, "code": "IDEMPOTENCY_CONTRACT_INVALID"}


def validate_atomic_action(action: Dict[str, Any], adapter_contract: Dict[str, Any]) -> Dict[str, Any]:
    descriptor = action.get("descriptor", {})
    if not descriptor.get("atomic", False):
        return {"decision": PASS, "code": "NOT_ATOMIC"}
    kind = descriptor.get("descriptor_kind")
    guarantee = adapter_contract.get("atomicity_guarantees", {}).get(kind)
    if guarantee != "semantic_all_or_none":
        return {"decision": FAIL, "code": "ATOMICITY_CONTRACT_INSUFFICIENT"}
    return {"decision": PASS, "code": "OK"}


def validate_action_graph(
    actions: Sequence[Dict[str, Any]],
    external_bindings: Optional[Dict[str, Dict[str, Any]]] = None,
    producer_equivalence: Sequence[Dict[str, Any]] = (),
    ordering_evidence: Sequence[Dict[str, Any]] = (),
) -> Dict[str, Any]:
    external_bindings = external_bindings or {}
    findings: List[Dict[str, Any]] = []
    producers: Dict[str, List[str]] = {}
    action_ids = [a["id"] for a in actions]
    if len(action_ids) != len(set(action_ids)):
        return {"decision": FAIL, "code": "ACTION_ID_AMBIGUOUS", "findings": []}
    for action in actions:
        if action.get("must_precede"):
            findings.append({"decision": FAIL, "code": "UNPROVEN_ORDERING", "action_id": action["id"]})
        for handle in action.get("produces", []):
            producers.setdefault(handle, []).append(action["id"])

    canonical_producer: Dict[str, str] = {}
    for handle, ids in sorted(producers.items()):
        if len(ids) == 1:
            canonical_producer[handle] = ids[0]
            continue
        matches = [e for e in producer_equivalence if e.get("handle") == handle and sorted(e.get("producer_ids", [])) == sorted(ids)]
        if len(matches) != 1:
            findings.append({"decision": FAIL, "code": "DUPLICATE_PRODUCER_UNRESOLVED", "handle": handle})
            continue
        decision = matches[0].get("decision")
        if decision not in _PRECEDENCE:
            findings.append({"decision": FAIL, "code": "PRODUCER_EQUIVALENCE_EVIDENCE_MALFORMED", "handle": handle})
            continue
        findings.append({"decision": decision, "code": f"PRODUCER_EQUIVALENCE_{decision}", "handle": handle})
        if decision == PASS:
            canonical_producer[handle] = sorted(ids)[0]

    edges: Dict[str, set[str]] = {a["id"]: set() for a in actions}
    for action in actions:
        for req in action.get("requires", []):
            if req in external_bindings:
                ext = external_bindings[req]
                if ext.get("resolved", False):
                    continue
                findings.append({"decision": BLOCKED, "code": "EXTERNAL_BINDING_UNRESOLVED", "handle": req})
                continue
            producer = canonical_producer.get(req)
            if producer is None:
                if req not in producers:
                    findings.append({"decision": FAIL, "code": "UNRESOLVED_PREREQUISITE", "handle": req})
                continue
            edges[producer].add(action["id"])

    seen_order_pairs: set[Tuple[str, str]] = set()
    for evidence in ordering_evidence:
        before, after = evidence.get("before"), evidence.get("after")
        required = {"before", "after", "evidence_id", "adapter_contract_id", "adapter_contract_version"}
        if not required.issubset(evidence) or before not in edges or after not in edges:
            findings.append({"decision": FAIL, "code": "ORDER_EVIDENCE_MALFORMED"})
            continue
        pair = (before, after)
        if pair in seen_order_pairs:
            findings.append({"decision": FAIL, "code": "ORDER_EVIDENCE_AMBIGUOUS", "pair": pair})
            continue
        seen_order_pairs.add(pair)
        edges[before].add(after)

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
        findings.append({"decision": FAIL, "code": "ACTION_DEPENDENCY_CYCLE"})
    decision = aggregate_decisions(f["decision"] for f in findings)
    return {"decision": decision, "code": "OK" if decision == PASS else "ACTION_GRAPH_FINDINGS", "findings": sorted(findings, key=lambda f: json.dumps(f, sort_keys=True)), "edges": sorted((a, b) for a, bs in edges.items() for b in bs)}


@dataclass(frozen=True)
class EvaluationRevision:
    revision_id: str
    state: str
    prior_revision_id: Optional[str] = None
    target_snapshot_fingerprint: Optional[str] = None


def finish_revision(revision_id: str, terminal_state: str, target_snapshot_fingerprint: Optional[str] = None) -> EvaluationRevision:
    if terminal_state not in {"blocked", "indeterminate", "complete"}:
        raise ValueError("INVALID_TERMINAL_STATE")
    return EvaluationRevision(revision_id, terminal_state, None, target_snapshot_fingerprint)


def reevaluate(previous: EvaluationRevision, new_revision_id: str, terminal_state: str, target_snapshot_fingerprint: Optional[str] = None) -> EvaluationRevision:
    if terminal_state not in {"blocked", "indeterminate", "complete"}:
        raise ValueError("INVALID_TERMINAL_STATE")
    return EvaluationRevision(new_revision_id, terminal_state, previous.revision_id, target_snapshot_fingerprint)


def aggregate_representability(
    validation_decision: str,
    evaluation_state: str,
    required_outcomes: Sequence[str],
    optional_nonsemantic_outcomes: Sequence[str] = (),
) -> Optional[str]:
    del optional_nonsemantic_outcomes
    if validation_decision != PASS or evaluation_state != "complete":
        return None
    unknown = [o for o in required_outcomes if o not in _REPRESENTABILITY_RANK]
    if unknown:
        raise ValueError(f"UNKNOWN_REPRESENTABILITY:{unknown[0]}")
    return max(required_outcomes or ["exact"], key=lambda x: _REPRESENTABILITY_RANK[x])


def canonical_loss_key(source_identity: str, obligation: str, aspect_identity: str) -> str:
    return json.dumps([source_identity, obligation, aspect_identity], ensure_ascii=False, separators=(",", ":"))


def execution_permission(outcome: Optional[str], losses: Sequence[str], policies: Sequence[Dict[str, str]], independent_blocker: bool = False) -> str:
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


def _component_map(items: Sequence[Dict[str, Any]]) -> Tuple[Optional[Dict[str, Dict[str, Any]]], List[Dict[str, Any]]]:
    mapping: Dict[str, Dict[str, Any]] = {}
    findings: List[Dict[str, Any]] = []
    for item in items:
        cid = item.get("component_id")
        if not isinstance(cid, str) or not cid:
            findings.append({"decision": FAIL, "code": "TARGET_COMPONENT_SCOPE_MISSING"})
            continue
        if cid in mapping:
            findings.append({"decision": FAIL, "code": "TARGET_COMPONENT_SCOPE_AMBIGUOUS", "component_id": cid})
            continue
        mapping[cid] = item
    return mapping, findings


def validate_backend_target(requirement: Dict[str, Any], resolved: Dict[str, Any]) -> Dict[str, Any]:
    required, findings = _component_map(requirement.get("components", []))
    if not required:
        findings.append({"decision": FAIL, "code": "TARGET_COMPONENT_SCOPE_MISSING"})
    orch = requirement.get("orchestration_component_id")
    if len(required) > 1 and orch not in required:
        findings.append({"decision": FAIL, "code": "TARGET_ORCHESTRATION_MISSING"})

    runtime, runtime_findings = _component_map(resolved.get("components", []))
    findings.extend(runtime_findings)
    unsupported: List[str] = []

    for cid, component in sorted(required.items()):
        observed = runtime.get(cid)
        if observed is None:
            findings.append({"decision": BLOCKED, "code": "TARGET_COMPONENT_UNRESOLVED", "component_id": cid})
            continue
        required_adapter = component.get("adapter_contract", {})
        actual_adapter = observed.get("adapter_contract")
        if actual_adapter is None:
            findings.append({"decision": BLOCKED, "code": "TARGET_ADAPTER_UNRESOLVED", "component_id": cid})
        elif required_adapter and actual_adapter != required_adapter:
            findings.append({"decision": FAIL, "code": "TARGET_ADAPTER_MISMATCH", "component_id": cid})

        actual_release = observed.get("actual_release")
        if not actual_release:
            findings.append({"decision": BLOCKED, "code": "TARGET_RELEASE_UNRESOLVED", "component_id": cid})
        else:
            allowed_releases = component.get("release_compatibility", {}).get("allowed")
            if allowed_releases is not None and actual_release not in set(allowed_releases):
                findings.append({"decision": FAIL, "code": "TARGET_RELEASE_MISMATCH", "component_id": cid})

        required_caps = set(component.get("required_capabilities", []))
        observed_caps = observed.get("capabilities")
        if observed_caps is None:
            findings.append({"decision": BLOCKED, "code": "TARGET_CAPABILITY_EVIDENCE_MISSING", "component_id": cid})
        else:
            unsupported.extend(f"{cid}:capability:{x}" for x in sorted(required_caps - set(observed_caps)))

        formulations = component.get("formulation_bindings", [])
        keys = [f.get("sol_obligation") for f in formulations]
        if None in keys:
            findings.append({"decision": FAIL, "code": "FORMULATION_BINDING_MISSING", "component_id": cid})
        if len(keys) != len(set(keys)):
            findings.append({"decision": FAIL, "code": "FORMULATION_BINDING_AMBIGUOUS", "component_id": cid})
        available_formulations = observed.get("available_formulations")
        if formulations and available_formulations is None:
            findings.append({"decision": BLOCKED, "code": "FORMULATION_EVIDENCE_MISSING", "component_id": cid})
        elif available_formulations is not None:
            for binding in formulations:
                backend_formulation = binding.get("backend_formulation")
                if not backend_formulation:
                    findings.append({"decision": FAIL, "code": "FORMULATION_BINDING_MISSING", "component_id": cid})
                elif backend_formulation not in set(available_formulations):
                    unsupported.append(f"{cid}:formulation:{backend_formulation}")

    if len(required) > 1 and orch in required:
        orchestration_runtime = runtime.get(orch)
        if orchestration_runtime is None:
            findings.append({"decision": BLOCKED, "code": "TARGET_ORCHESTRATION_UNRESOLVED", "component_id": orch})
        elif not orchestration_runtime.get("orchestration_capable", False):
            unsupported.append(f"{orch}:orchestration")

    decision = aggregate_decisions(f["decision"] for f in findings)
    result = {
        "decision": decision,
        "code": "OK" if decision == PASS else "TARGET_VALIDATION_FINDINGS",
        "findings": sorted(findings, key=lambda f: json.dumps(f, sort_keys=True)),
    }
    if decision == PASS and unsupported:
        result.update({"representability": "unsupported", "missing": sorted(set(unsupported)), "code": "TARGET_AUTHORITATIVE_ABSENCE"})
    return result
