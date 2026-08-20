"""Focused metrology validation for ADR-0020/0025/0026 thermal fixtures."""

from __future__ import annotations

from typing import Mapping, Sequence

from tests.interface_semantics import (
    InterfaceContractError,
    effective_entity_conformance,
    effective_interface_contract,
    validate_constraint_applications,
    validate_requirement_definitions,
)
from tests.model_snapshot_semantics import ModelSnapshotError, validate_resolved_model_snapshot
from tests.value_graph_semantics import ValueGraphError, dimensions_equal, unit_reference_key


class ReferenceMetrologyError(ValueError):
    """Definite semantic contradiction in focused metrology evidence."""


def _collect_package_resources(packages: Sequence[Mapping[str, object]]):
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


def _metrology_index(evidence: Mapping[str, object]):
    units = evidence.get("units")
    if not isinstance(units, list):
        raise ReferenceMetrologyError("REFERENCE_METROLOGY_EVIDENCE_INVALID")
    result = {}
    for entry in units:
        if not isinstance(entry, Mapping):
            raise ReferenceMetrologyError("REFERENCE_METROLOGY_EVIDENCE_INVALID")
        unit = entry.get("unit")
        dimension = entry.get("dimension")
        if not isinstance(unit, Mapping) or not isinstance(dimension, Mapping):
            raise ReferenceMetrologyError("REFERENCE_METROLOGY_EVIDENCE_INVALID")
        try:
            key = unit_reference_key(unit)
        except ValueGraphError as exc:
            raise ReferenceMetrologyError("REFERENCE_METROLOGY_EVIDENCE_INVALID") from exc
        if key in result and result[key] != dimension:
            raise ReferenceMetrologyError("REFERENCE_METROLOGY_IDENTITY_CONTENT_CONFLICT")
        result[key] = dimension
    return result


def validate_snapshot_metrology(
    snapshot: Mapping[str, object],
    packages: Sequence[Mapping[str, object]],
    evidence: Mapping[str, object] | None,
):
    base = validate_resolved_model_snapshot(snapshot, packages)
    interfaces, implementations, constraints, entity_parent, definition_kinds = _collect_package_resources(packages)

    try:
        for interface in interfaces:
            contract = effective_interface_contract(interface["id"], interfaces)
            validate_requirement_definitions(contract, definition_kinds)
            validate_constraint_applications(contract, constraints)
    except InterfaceContractError as exc:
        raise ReferenceMetrologyError(f"REFERENCE_INTERFACE_CONTRACT_INVALID:{exc}") from exc

    if evidence is None:
        return {
            "state": "INDETERMINATE",
            "diagnostics": ["REFERENCE_METROLOGY_EVIDENCE_UNRESOLVED"],
            "base": base,
        }

    metrology = _metrology_index(evidence)
    constraint_index = {
        item["id"]: item for item in constraints if isinstance(item, Mapping) and isinstance(item.get("id"), str)
    }
    diagnostics: list[str] = []

    for entity_id, entity in base["instance_index"].items():
        entity_type = base["instance_types"][entity_id]
        try:
            conformance = effective_entity_conformance(
                entity_type,
                entity_parent=entity_parent,
                interfaces=interfaces,
                implementations=implementations,
                definition_kinds=definition_kinds,
            )
        except InterfaceContractError as exc:
            raise ReferenceMetrologyError(f"REFERENCE_INTERFACE_CONFORMANCE_INVALID:{exc}") from exc

        assignments = {
            assignment["property"]: assignment
            for assignment in entity.get("properties", [])
            if isinstance(assignment, Mapping) and isinstance(assignment.get("property"), str)
        }

        expected_by_property: dict[str, list[Mapping[str, object]]] = {}
        for application in conformance["applications"].values():
            target = application.get("target")
            if not isinstance(target, Mapping) or target.get("kind") != "property_requirement":
                continue
            requirement = target.get("requirement")
            if not isinstance(requirement, str):
                continue
            concrete = conformance["mappings"].get(("property", requirement))
            if concrete is None:
                continue
            constraint = constraint_index.get(application.get("constraint"))
            if constraint is None:
                raise ReferenceMetrologyError("REFERENCE_DIMENSION_CONSTRAINT_UNRESOLVED")
            payload = constraint.get("payload")
            if isinstance(payload, Mapping) and payload.get("type") == "dimension":
                vector = payload.get("vector")
                if not isinstance(vector, Mapping):
                    raise ReferenceMetrologyError("REFERENCE_DIMENSION_CONSTRAINT_INVALID")
                expected_by_property.setdefault(concrete, []).append(vector)

        for property_id, vectors in expected_by_property.items():
            if property_id not in assignments:
                # Interface capability defines the semantic property, but this focused
                # validator checks dimensional consistency only when the model instance
                # carries a concrete value assignment.
                continue
            first = vectors[0]
            for vector in vectors[1:]:
                try:
                    equal = dimensions_equal(first, vector)
                except ValueGraphError as exc:
                    raise ReferenceMetrologyError("REFERENCE_DIMENSION_CONSTRAINT_INVALID") from exc
                if not equal:
                    raise ReferenceMetrologyError("REFERENCE_PROPERTY_DIMENSION_CONFLICT")

            value_definition = assignments[property_id].get("value_definition")
            if not isinstance(value_definition, Mapping) or value_definition.get("mechanism") != "literal":
                diagnostics.append("REFERENCE_VALUE_DIMENSION_UNRESOLVED")
                continue
            value = value_definition.get("value")
            if not isinstance(value, Mapping):
                diagnostics.append("REFERENCE_VALUE_DIMENSION_UNRESOLVED")
                continue
            unit = value.get("unit")
            if not isinstance(unit, Mapping):
                diagnostics.append("REFERENCE_VALUE_UNIT_UNRESOLVED")
                continue
            try:
                key = unit_reference_key(unit)
            except ValueGraphError as exc:
                raise ReferenceMetrologyError("REFERENCE_VALUE_UNIT_INVALID") from exc
            resolved_dimension = metrology.get(key)
            if resolved_dimension is None:
                diagnostics.append("REFERENCE_VALUE_UNIT_UNRESOLVED")
                continue
            try:
                equal = dimensions_equal(first, resolved_dimension)
            except ValueGraphError as exc:
                raise ReferenceMetrologyError("REFERENCE_METROLOGY_EVIDENCE_INVALID") from exc
            if not equal:
                raise ReferenceMetrologyError("REFERENCE_VALUE_UNIT_DIMENSION_MISMATCH")

    state = "INDETERMINATE" if diagnostics else "PASS"
    return {"state": state, "diagnostics": diagnostics, "base": base}
