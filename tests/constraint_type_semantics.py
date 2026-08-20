"""Minimal semantic helper for ADR-0019 relation-target Type constraints.

This is a design-stage reference validator slice, not a normative SOL runtime.
JSON Schema validates payload structure; this module exercises the semantic
boundaries that ADR-0019 deliberately leaves outside structural schemas.
"""

from __future__ import annotations

from collections import defaultdict
from typing import Iterable, Mapping, Sequence


SEMANTIC_AXIS = "semantic"
REPRESENTABILITY_AXIS = "representability"


class TypeConstraintError(ValueError):
    """Deterministic validation error carrying an ADR-0019 diagnostic code."""


def is_same_or_subtype(child: str, parent: str, direct_parent: Mapping[str, str]) -> bool:
    """Return whether child == parent or child is in parent's canonical is_a closure."""
    if child == parent:
        return True
    seen: set[str] = set()
    current = child
    while current in direct_parent:
        if current in seen:
            raise TypeConstraintError("TYPE_TAXONOMY_CYCLE")
        seen.add(current)
        current = direct_parent[current]
        if current == parent:
            return True
    return False


def require_entity_type_target(
    target_type: str,
    *,
    entity_type_ids: set[str],
    interface_ids: set[str],
) -> None:
    """Enforce ADR-0019's taxonomic Entity Type axis, excluding Interfaces."""
    if target_type in interface_ids:
        raise TypeConstraintError("TYPE_TARGET_INTERFACE_FORBIDDEN")
    if target_type not in entity_type_ids:
        raise TypeConstraintError("TYPE_TARGET_ENTITY_TYPE_REQUIRED")


def normalize_type_payload(
    authored: Mapping[str, str],
    *,
    canonical_relation: str | None,
    canonical_target_type: str | None,
    entity_type_ids: set[str],
    interface_ids: set[str],
) -> dict[str, str]:
    """Construct the canonical three-field ADR-0019 semantic payload."""
    if authored.get("type") != "type":
        raise TypeConstraintError("TYPE_CONSTRAINT_KIND_INVALID")
    if not canonical_relation:
        raise TypeConstraintError("TYPE_RELATION_CANONICAL_ID_REQUIRED")
    if not canonical_target_type:
        raise TypeConstraintError("TYPE_TARGET_CANONICAL_ID_REQUIRED")
    require_entity_type_target(
        canonical_target_type,
        entity_type_ids=entity_type_ids,
        interface_ids=interface_ids,
    )
    return {
        "type": "type",
        "relation": canonical_relation,
        "target_type": canonical_target_type,
    }


def target_family_allows(
    target_type: str,
    allowed_targets: Iterable[str],
    direct_parent: Mapping[str, str],
) -> bool:
    """Finite-family narrowing: T must equal/subtype at least one allowed member."""
    return any(is_same_or_subtype(target_type, base, direct_parent) for base in allowed_targets)


def validate_target_family(
    target_type: str,
    allowed_targets: Iterable[str],
    direct_parent: Mapping[str, str],
) -> None:
    if not target_family_allows(target_type, allowed_targets, direct_parent):
        raise TypeConstraintError("TYPE_TARGET_NOT_ALLOWED")


def allowed_targets_for_source(
    source_type: str,
    allowed_pairs: Sequence[tuple[str, Sequence[str]]],
    direct_parent: Mapping[str, str],
) -> set[str]:
    """Resolve ADR-0016 allowed target family for an equal/subtype source context."""
    targets: set[str] = set()
    for allowed_source, allowed_targets in allowed_pairs:
        if is_same_or_subtype(source_type, allowed_source, direct_parent):
            targets.update(allowed_targets)
    if not targets:
        raise TypeConstraintError("TYPE_SOURCE_CONTEXT_NOT_ALLOWED")
    return targets


def validate_allowed_pair_narrowing(
    source_type: str,
    target_type: str,
    allowed_pairs: Sequence[tuple[str, Sequence[str]]],
    direct_parent: Mapping[str, str],
) -> None:
    allowed_targets = allowed_targets_for_source(source_type, allowed_pairs, direct_parent)
    validate_target_family(target_type, allowed_targets, direct_parent)


def intersect_type_targets(
    left: str,
    right: str,
    direct_parent: Mapping[str, str],
) -> str:
    """ADR-0019 deterministic single-taxonomy Type-axis intersection."""
    if is_same_or_subtype(left, right, direct_parent):
        return left
    if is_same_or_subtype(right, left, direct_parent):
        return right
    raise TypeConstraintError("TYPE_INTERSECTION_EMPTY")


def group_evidence_by_axis(records: Iterable[Mapping[str, object]]) -> dict[str, list[Mapping[str, object]]]:
    """Keep upstream semantic validity and backend/Profile representability separate."""
    grouped: dict[str, list[Mapping[str, object]]] = defaultdict(list)
    for record in records:
        axis = record.get("axis")
        if axis not in {SEMANTIC_AXIS, REPRESENTABILITY_AXIS}:
            raise TypeConstraintError("TYPE_EVALUATION_AXIS_INVALID")
        grouped[str(axis)].append(record)
    return dict(grouped)
