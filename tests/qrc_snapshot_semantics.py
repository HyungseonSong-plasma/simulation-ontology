"""Focused ADR-0012 QRC evaluation over ADR-0029 closed snapshots."""

from __future__ import annotations

from typing import Mapping, Sequence

from tests.interface_semantics import (
    InterfaceContractError,
    effective_entity_conformance,
    effective_interface_contract,
    validate_constraint_applications,
    validate_requirement_definitions,
)
from tests.model_snapshot_semantics import (
    ModelSnapshotError,
    _matches_type,
    validate_resolved_model_snapshot,
)


class SnapshotQRCError(ValueError):
    """Deterministic QRC/reference-snapshot validation diagnostic."""


def _collect_resources(packages: Sequence[Mapping[str, object]]):
    interfaces = []
    implementations = []
    constraints = []
    entity_parent: dict[str, str | None] = {}
    definition_kinds: dict[str, str] = {}

    for package in packages:
        for entity in package.get("entity_types", []):
            identifier = entity.get("id")
            if isinstance(identifier, str):
                entity_parent[identifier] = entity.get("is_a")
                definition_kinds[identifier] = "entity_type"
        for prop in package.get("properties", []):
            identifier = prop.get("id")
            if isinstance(identifier, str):
                definition_kinds[identifier] = "property"
        for relation in package.get("relations", []):
            identifier = relation.get("id")
            if isinstance(identifier, str):
                definition_kinds[identifier] = "relation"
        for constraint in package.get("constraint_definitions", []):
            identifier = constraint.get("id")
            if isinstance(identifier, str):
                definition_kinds[identifier] = "constraint_definition"
            constraints.append(constraint)
        for interface in package.get("interfaces", []):
            identifier = interface.get("id")
            if isinstance(identifier, str):
                definition_kinds[identifier] = "interface"
            interfaces.append(interface)
        implementations.extend(package.get("interface_implementations", []))

    return interfaces, implementations, constraints, entity_parent, definition_kinds


def _contains(count: int, minimum: object, maximum: object) -> bool:
    if isinstance(minimum, bool) or not isinstance(minimum, int) or minimum < 0:
        raise SnapshotQRCError("QRC_BOUND_INVALID")
    if maximum != "unbounded":
        if isinstance(maximum, bool) or not isinstance(maximum, int) or maximum < 0:
            raise SnapshotQRCError("QRC_BOUND_INVALID")
        if minimum > maximum:
            raise SnapshotQRCError("QRC_EMPTY_INTERVAL")
    if count < minimum:
        return False
    if maximum == "unbounded":
        return True
    return count <= maximum


def validate_snapshot_qrc(
    snapshot: Mapping[str, object],
    packages: Sequence[Mapping[str, object]],
):
    try:
        base = validate_resolved_model_snapshot(snapshot, packages)
    except ModelSnapshotError as exc:
        raise SnapshotQRCError(f"QRC_MODEL_PRECONDITION_FAIL:{exc}") from exc

    interfaces, implementations, constraints, entity_parent, definition_kinds = _collect_resources(packages)
    constraint_index = {
        item["id"]: item
        for item in constraints
        if isinstance(item, Mapping) and isinstance(item.get("id"), str)
    }

    try:
        for interface in interfaces:
            contract = effective_interface_contract(interface["id"], interfaces)
            validate_requirement_definitions(contract, definition_kinds)
            validate_constraint_applications(contract, constraints)
    except InterfaceContractError as exc:
        raise SnapshotQRCError(f"QRC_INTERFACE_CONTRACT_FAIL:{exc}") from exc

    outgoing: dict[tuple[str, str], set[str]] = {}
    for relation_id, source_id, target_id in base["relation_triples"]:
        outgoing.setdefault((relation_id, source_id), set()).add(target_id)

    evaluated = []
    for source_id, source_type in base["instance_types"].items():
        try:
            conformance = effective_entity_conformance(
                source_type,
                entity_parent=entity_parent,
                interfaces=interfaces,
                implementations=implementations,
                definition_kinds=definition_kinds,
            )
        except InterfaceContractError as exc:
            raise SnapshotQRCError(f"QRC_INTERFACE_CONFORMANCE_FAIL:{exc}") from exc

        for application in conformance["applications"].values():
            target = application.get("target")
            if not isinstance(target, Mapping) or target.get("kind") != "relation_requirement":
                continue
            requirement = target.get("requirement")
            if not isinstance(requirement, str):
                raise SnapshotQRCError("QRC_RELATION_REQUIREMENT_INVALID")
            concrete_relation = conformance["mappings"].get(("relation", requirement))
            if concrete_relation is None:
                raise SnapshotQRCError("QRC_RELATION_MAPPING_UNRESOLVED")

            constraint_id = application.get("constraint")
            constraint = constraint_index.get(constraint_id)
            if constraint is None:
                raise SnapshotQRCError("QRC_CONSTRAINT_DEFINITION_UNRESOLVED")
            payload = constraint.get("payload")
            if not isinstance(payload, Mapping) or payload.get("type") != "cardinality":
                continue
            qualifier = payload.get("qualifier")
            if not isinstance(qualifier, Mapping):
                # Ordinary cardinality is handled by the base snapshot validator when
                # represented as relation authority/projection. This helper is QRC-only.
                continue
            if payload.get("direction") != "source":
                raise SnapshotQRCError("QRC_DIRECTION_UNSUPPORTED")
            if payload.get("relation") != concrete_relation:
                raise SnapshotQRCError("QRC_RELATION_AUTHORITY_MISMATCH")

            target_type = qualifier.get("target_type")
            resolved = base["resource_index"].get(target_type)
            if resolved is None:
                raise SnapshotQRCError("QRC_QUALIFIER_TARGET_TYPE_UNRESOLVED")
            if resolved[0] != "entity_type":
                raise SnapshotQRCError("QRC_QUALIFIER_TARGET_TYPE_KIND_MISMATCH")

            matching_targets: set[str] = set()
            for target_id in outgoing.get((concrete_relation, source_id), set()):
                actual_type = base["instance_types"].get(target_id)
                if actual_type is None:
                    raise SnapshotQRCError("QRC_TARGET_IDENTITY_UNRESOLVED")
                if _matches_type(actual_type, target_type, entity_parent):
                    matching_targets.add(target_id)

            count = len(matching_targets)
            if not _contains(count, payload.get("min"), payload.get("max")):
                raise SnapshotQRCError(f"QRC_CARDINALITY_VIOLATION:{constraint_id}")
            evaluated.append(
                {
                    "source": source_id,
                    "constraint": constraint_id,
                    "relation": concrete_relation,
                    "target_type": target_type,
                    "count": count,
                    "targets": frozenset(matching_targets),
                }
            )

    return {"state": "PASS", "evaluated": evaluated, "base": base}
