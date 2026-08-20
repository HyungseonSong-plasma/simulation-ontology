"""Minimal design-stage semantic helper for ADR-0025 Interface contracts."""

from __future__ import annotations

from collections import defaultdict
from typing import Mapping, Sequence


class InterfaceContractError(ValueError):
    """Deterministic ADR-0025 validation diagnostic."""


def _unique_by_id(items: Sequence[Mapping[str, object]], *, code_missing: str, code_ambiguous: str):
    index: dict[str, Mapping[str, object]] = {}
    for item in items:
        identifier = item.get("id")
        if not isinstance(identifier, str) or not identifier:
            raise InterfaceContractError(code_missing)
        if identifier in index:
            raise InterfaceContractError(code_ambiguous)
        index[identifier] = item
    return index


def _application_key(app: Mapping[str, object]):
    target = app.get("target")
    if not isinstance(target, Mapping):
        raise InterfaceContractError("INTERFACE_CONSTRAINT_TARGET_INVALID")
    kind = target.get("kind")
    if kind == "interface":
        target_key = ("interface", None)
    elif kind in {"property_requirement", "relation_requirement"}:
        requirement = target.get("requirement")
        if not isinstance(requirement, str) or not requirement:
            raise InterfaceContractError("INTERFACE_CONSTRAINT_TARGET_INVALID")
        target_key = (str(kind), requirement)
    else:
        raise InterfaceContractError("INTERFACE_CONSTRAINT_TARGET_INVALID")
    constraint = app.get("constraint")
    if not isinstance(constraint, str) or not constraint:
        raise InterfaceContractError("INTERFACE_CONSTRAINT_DEFINITION_UNRESOLVED")
    return constraint, target_key


def effective_interface_contract(interface_id: str, interfaces: Sequence[Mapping[str, object]]):
    index = _unique_by_id(
        interfaces,
        code_missing="INTERFACE_ID_INVALID",
        code_ambiguous="INTERFACE_ID_AMBIGUOUS",
    )
    if interface_id not in index:
        raise InterfaceContractError("INTERFACE_UNRESOLVED")

    memo: dict[str, dict[str, object]] = {}
    visiting: set[str] = set()

    def visit(current: str):
        if current in memo:
            return memo[current]
        if current in visiting:
            raise InterfaceContractError("INTERFACE_EXTENSION_CYCLE")
        definition = index.get(current)
        if definition is None:
            raise InterfaceContractError("INTERFACE_PARENT_UNRESOLVED")
        visiting.add(current)

        properties: set[str] = set()
        relations: set[str] = set()
        applications: dict[object, Mapping[str, object]] = {}
        interface_ids: set[str] = {current}

        for parent in definition.get("extends", []):
            if not isinstance(parent, str) or not parent:
                raise InterfaceContractError("INTERFACE_PARENT_UNRESOLVED")
            inherited = visit(parent)
            properties.update(inherited["properties"])
            relations.update(inherited["relations"])
            applications.update(inherited["applications"])
            interface_ids.update(inherited["interfaces"])

        for requirement in definition.get("property_requirements", []):
            identifier = requirement.get("property") if isinstance(requirement, Mapping) else None
            if not isinstance(identifier, str) or not identifier:
                raise InterfaceContractError("INTERFACE_PROPERTY_REQUIREMENT_INVALID")
            properties.add(identifier)

        for requirement in definition.get("relation_requirements", []):
            identifier = requirement.get("relation") if isinstance(requirement, Mapping) else None
            if not isinstance(identifier, str) or not identifier:
                raise InterfaceContractError("INTERFACE_RELATION_REQUIREMENT_INVALID")
            relations.add(identifier)

        for app in definition.get("constraint_applications", []):
            if not isinstance(app, Mapping):
                raise InterfaceContractError("INTERFACE_CONSTRAINT_APPLICATION_INVALID")
            applications.setdefault(_application_key(app), app)

        visiting.remove(current)
        result = {
            "properties": frozenset(properties),
            "relations": frozenset(relations),
            "applications": applications,
            "interfaces": frozenset(interface_ids),
        }
        memo[current] = result
        return result

    return visit(interface_id)


def validate_constraint_applications(
    contract: Mapping[str, object],
    constraint_definitions: Sequence[Mapping[str, object]],
):
    definitions = _unique_by_id(
        constraint_definitions,
        code_missing="INTERFACE_CONSTRAINT_DEFINITION_UNRESOLVED",
        code_ambiguous="INTERFACE_CONSTRAINT_DEFINITION_AMBIGUOUS",
    )
    properties = set(contract["properties"])
    relations = set(contract["relations"])

    for app in contract["applications"].values():
        constraint_id, target_key = _application_key(app)
        definition = definitions.get(constraint_id)
        if definition is None:
            raise InterfaceContractError("INTERFACE_CONSTRAINT_DEFINITION_UNRESOLVED")
        allowed = definition.get("interface_application_target_kinds")
        if allowed is None:
            raise InterfaceContractError("INTERFACE_CONSTRAINT_TARGET_CONTRACT_MISSING")
        if (
            not isinstance(allowed, list)
            or not allowed
            or any(kind not in {"interface", "property_requirement", "relation_requirement"} for kind in allowed)
        ):
            raise InterfaceContractError("INTERFACE_CONSTRAINT_TARGET_CONTRACT_INVALID")
        kind, requirement = target_key
        if kind not in set(allowed):
            raise InterfaceContractError("INTERFACE_CONSTRAINT_TARGET_INCOMPATIBLE")
        if kind == "property_requirement":
            if requirement in relations:
                raise InterfaceContractError("INTERFACE_CONSTRAINT_TARGET_KIND_MISMATCH")
            if requirement not in properties:
                raise InterfaceContractError("INTERFACE_CONSTRAINT_TARGET_UNKNOWN")
        elif kind == "relation_requirement":
            if requirement in properties:
                raise InterfaceContractError("INTERFACE_CONSTRAINT_TARGET_KIND_MISMATCH")
            if requirement not in relations:
                raise InterfaceContractError("INTERFACE_CONSTRAINT_TARGET_UNKNOWN")
    return True


def _entity_ancestors(entity_type: str, entity_parent: Mapping[str, str | None]):
    result: list[str] = []
    seen: set[str] = set()
    current: str | None = entity_type
    while current is not None:
        if current in seen:
            raise InterfaceContractError("ENTITY_TAXONOMY_CYCLE")
        seen.add(current)
        result.append(current)
        current = entity_parent.get(current)
    return tuple(result)


def validate_direct_declaration_uniqueness(implementations: Sequence[Mapping[str, object]]):
    seen: set[tuple[str, str]] = set()
    for implementation in implementations:
        entity = implementation.get("entity_type")
        interface = implementation.get("interface")
        if not isinstance(entity, str) or not isinstance(interface, str):
            raise InterfaceContractError("INTERFACE_IMPLEMENTATION_REFERENCE_INVALID")
        key = (entity, interface)
        if key in seen:
            raise InterfaceContractError("INTERFACE_IMPLEMENTATION_DUPLICATE_DIRECT")
        seen.add(key)
    return True


def _mapping_dict(
    mappings: Sequence[Mapping[str, object]],
    required: set[str],
    *,
    expected_kind: str,
    definition_kinds: Mapping[str, str],
):
    grouped: dict[str, list[str]] = defaultdict(list)
    for mapping in mappings:
        requirement = mapping.get("requirement")
        concrete = mapping.get("concrete")
        if not isinstance(requirement, str) or not isinstance(concrete, str):
            raise InterfaceContractError("INTERFACE_REQUIREMENT_MAPPING_INVALID")
        if requirement not in required:
            raise InterfaceContractError("INTERFACE_REQUIREMENT_MAPPING_UNKNOWN")
        if definition_kinds.get(concrete) != expected_kind:
            raise InterfaceContractError("INTERFACE_REQUIREMENT_KIND_MISMATCH")
        grouped[requirement].append(concrete)
    for requirement in required:
        values = grouped.get(requirement, [])
        if not values:
            raise InterfaceContractError("INTERFACE_REQUIREMENT_MAPPING_MISSING")
        if len(values) != 1:
            raise InterfaceContractError("INTERFACE_REQUIREMENT_MAPPING_AMBIGUOUS")
    return {requirement: values[0] for requirement, values in grouped.items()}


def validate_direct_implementation(
    implementation: Mapping[str, object],
    interfaces: Sequence[Mapping[str, object]],
    definition_kinds: Mapping[str, str],
):
    interface_id = implementation.get("interface")
    if not isinstance(interface_id, str):
        raise InterfaceContractError("INTERFACE_IMPLEMENTATION_REFERENCE_INVALID")
    contract = effective_interface_contract(interface_id, interfaces)
    properties = _mapping_dict(
        implementation.get("property_mappings", []),
        set(contract["properties"]),
        expected_kind="property",
        definition_kinds=definition_kinds,
    )
    relations = _mapping_dict(
        implementation.get("relation_mappings", []),
        set(contract["relations"]),
        expected_kind="relation",
        definition_kinds=definition_kinds,
    )
    return {"property": properties, "relation": relations, "contract": contract}


def effective_entity_conformance(
    entity_type: str,
    *,
    entity_parent: Mapping[str, str | None],
    interfaces: Sequence[Mapping[str, object]],
    implementations: Sequence[Mapping[str, object]],
    definition_kinds: Mapping[str, str],
):
    validate_direct_declaration_uniqueness(implementations)
    ancestors = set(_entity_ancestors(entity_type, entity_parent))
    relevant = [implementation for implementation in implementations if implementation.get("entity_type") in ancestors]

    concrete_by_requirement: dict[tuple[str, str], set[str]] = defaultdict(set)
    effective_interfaces: set[str] = set()
    effective_applications: dict[object, Mapping[str, object]] = {}

    for implementation in relevant:
        validated = validate_direct_implementation(implementation, interfaces, definition_kinds)
        contract = validated["contract"]
        effective_interfaces.update(contract["interfaces"])
        effective_applications.update(contract["applications"])
        for kind in ("property", "relation"):
            for requirement, concrete in validated[kind].items():
                concrete_by_requirement[(kind, requirement)].add(concrete)

    effective_mappings: dict[tuple[str, str], str] = {}
    for key, concrete_ids in concrete_by_requirement.items():
        if len(concrete_ids) != 1:
            raise InterfaceContractError("INTERFACE_EFFECTIVE_MAPPING_CONFLICT")
        effective_mappings[key] = next(iter(concrete_ids))

    return {
        "interfaces": frozenset(effective_interfaces),
        "mappings": effective_mappings,
        "applications": effective_applications,
    }
