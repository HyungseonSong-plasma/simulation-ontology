"""Focused semantic validation for ADR-0028 normalized ontology packages."""

from __future__ import annotations

from collections import defaultdict
from typing import Mapping, Sequence


CANONICAL_PREFIX = "https://simulation-ontology.org/id/"
RESOURCE_COLLECTIONS = {
    "entity_type": "entity_types",
    "property": "properties",
    "relation": "relations",
    "constraint_definition": "constraint_definitions",
    "interface": "interfaces",
}


class PackageContractError(ValueError):
    """Deterministic ADR-0028 validation diagnostic."""


def require_canonical_id(value: object) -> str:
    if not isinstance(value, str) or not value.startswith(CANONICAL_PREFIX) or len(value) <= len(CANONICAL_PREFIX):
        raise PackageContractError("CANONICAL_REFERENCE_INVALID")
    return value


def build_resource_index(packages: Sequence[Mapping[str, object]]):
    index: dict[str, tuple[str, Mapping[str, object]]] = {}
    for package in packages:
        for kind, collection_name in RESOURCE_COLLECTIONS.items():
            collection = package.get(collection_name, [])
            if not isinstance(collection, list):
                raise PackageContractError("PACKAGE_RESOURCE_COLLECTION_INVALID")
            for resource in collection:
                if not isinstance(resource, Mapping):
                    raise PackageContractError("PACKAGE_RESOURCE_INVALID")
                identifier = require_canonical_id(resource.get("id"))
                existing = index.get(identifier)
                if existing is not None:
                    existing_kind, existing_resource = existing
                    if existing_kind != kind or existing_resource != resource:
                        raise PackageContractError("CANONICAL_RESOURCE_IDENTITY_CONTENT_CONFLICT")
                    raise PackageContractError("CANONICAL_RESOURCE_DUPLICATE_ACTIVE")
                index[identifier] = (kind, resource)
    return index


def validate_namespace_providers(packages: Sequence[Mapping[str, object]]):
    providers: dict[str, list[tuple[str, str]]] = defaultdict(list)
    for package in packages:
        package_identity = package.get("package")
        if not isinstance(package_identity, Mapping):
            raise PackageContractError("PACKAGE_IDENTITY_INVALID")
        package_name = package_identity.get("name")
        package_version = package_identity.get("version")
        if not isinstance(package_name, str) or not package_name or not isinstance(package_version, str) or not package_version:
            raise PackageContractError("PACKAGE_IDENTITY_INVALID")
        for namespace in package.get("namespaces", []):
            if not isinstance(namespace, Mapping):
                raise PackageContractError("NAMESPACE_PROVIDER_INVALID")
            name = namespace.get("name")
            if not isinstance(name, str) or not name:
                raise PackageContractError("NAMESPACE_PROVIDER_INVALID")
            providers[name].append((package_name, package_version))

    for namespace, active in providers.items():
        if len(active) > 1:
            raise PackageContractError("NAMESPACE_PROVIDER_AMBIGUOUS")
    return {namespace: active[0] for namespace, active in providers.items()}


def validate_namespace_exports(package: Mapping[str, object], resource_index):
    for namespace in package.get("namespaces", []):
        name = namespace.get("name")
        seen: dict[str, tuple[str, str]] = {}
        for export in namespace.get("exports", []):
            export_name = export.get("name")
            kind = export.get("kind")
            identifier = require_canonical_id(export.get("id"))
            if not isinstance(export_name, str) or not export_name or kind not in RESOURCE_COLLECTIONS:
                raise PackageContractError("NAMESPACE_EXPORT_INVALID")
            previous = seen.get(export_name)
            current = (kind, identifier)
            if previous is not None and previous != current:
                raise PackageContractError("NAMESPACE_EXPORT_AMBIGUOUS")
            seen[export_name] = current
            resolved = resource_index.get(identifier)
            if resolved is None:
                raise PackageContractError("NAMESPACE_EXPORT_TARGET_UNRESOLVED")
            if resolved[0] != kind:
                raise PackageContractError("NAMESPACE_EXPORT_KIND_MISMATCH")
        if not isinstance(name, str) or not name:
            raise PackageContractError("NAMESPACE_PROVIDER_INVALID")
    return True


def _require_kind(resource_index, identifier: object, expected_kind: str, *, unresolved_code: str, mismatch_code: str):
    canonical = require_canonical_id(identifier)
    resolved = resource_index.get(canonical)
    if resolved is None:
        raise PackageContractError(unresolved_code)
    if resolved[0] != expected_kind:
        raise PackageContractError(mismatch_code)
    return resolved[1]


def validate_entity_types(package: Mapping[str, object], resource_index):
    for entity in package.get("entity_types", []):
        if "is_a" in entity:
            _require_kind(
                resource_index,
                entity["is_a"],
                "entity_type",
                unresolved_code="ENTITY_PARENT_UNRESOLVED",
                mismatch_code="ENTITY_PARENT_KIND_MISMATCH",
            )
    return True


def _projection_interval(payload: Mapping[str, object]):
    return payload.get("min"), payload.get("max")


def validate_relations(package: Mapping[str, object], resource_index):
    for relation in package.get("relations", []):
        relation_id = require_canonical_id(relation.get("id"))
        endpoint = relation.get("endpoint_contract")
        if not isinstance(endpoint, Mapping):
            raise PackageContractError("RELATION_ENDPOINT_CONTRACT_INVALID")
        kind = endpoint.get("kind")

        if kind == "typed":
            for field in ("domain", "range"):
                for identifier in endpoint.get(field, []):
                    _require_kind(
                        resource_index,
                        identifier,
                        "entity_type",
                        unresolved_code="RELATION_ENDPOINT_UNRESOLVED",
                        mismatch_code="RELATION_ENDPOINT_KIND_MISMATCH",
                    )
        elif kind == "allowed_pairs":
            for pair in endpoint.get("pairs", []):
                _require_kind(
                    resource_index,
                    pair.get("source"),
                    "entity_type",
                    unresolved_code="RELATION_ENDPOINT_UNRESOLVED",
                    mismatch_code="RELATION_ENDPOINT_KIND_MISMATCH",
                )
                for identifier in pair.get("targets", []):
                    _require_kind(
                        resource_index,
                        identifier,
                        "entity_type",
                        unresolved_code="RELATION_ENDPOINT_UNRESOLVED",
                        mismatch_code="RELATION_ENDPOINT_KIND_MISMATCH",
                    )
        else:
            raise PackageContractError("RELATION_ENDPOINT_CONTRACT_INVALID")

        projection = relation.get("source_cardinality_projection")
        if projection is not None:
            constraint = _require_kind(
                resource_index,
                projection.get("constraint"),
                "constraint_definition",
                unresolved_code="CARDINALITY_PROJECTION_CONSTRAINT_UNRESOLVED",
                mismatch_code="CARDINALITY_PROJECTION_CONSTRAINT_KIND_MISMATCH",
            )
            payload = constraint.get("payload")
            if not isinstance(payload, Mapping) or payload.get("type") != "cardinality":
                raise PackageContractError("CARDINALITY_PROJECTION_AUTHORITY_INVALID")
            if payload.get("relation") != relation_id or payload.get("direction") != "source" or "qualifier" in payload:
                raise PackageContractError("CARDINALITY_PROJECTION_AUTHORITY_INVALID")
            if _projection_interval(payload) != (projection.get("min"), projection.get("max")):
                raise PackageContractError("CARDINALITY_PROJECTION_MISMATCH")

            maximum = projection.get("max")
            minimum = projection.get("min")
            if isinstance(maximum, int) and not isinstance(maximum, bool) and isinstance(minimum, int) and minimum > maximum:
                raise PackageContractError("CARDINALITY_PROJECTION_INTERVAL_INVALID")
    return True


def validate_interfaces(package: Mapping[str, object], resource_index):
    for interface in package.get("interfaces", []):
        for parent in interface.get("extends", []):
            _require_kind(resource_index, parent, "interface", unresolved_code="INTERFACE_PARENT_UNRESOLVED", mismatch_code="INTERFACE_PARENT_KIND_MISMATCH")
        for requirement in interface.get("property_requirements", []):
            _require_kind(resource_index, requirement.get("property"), "property", unresolved_code="INTERFACE_REQUIREMENT_DEFINITION_UNRESOLVED", mismatch_code="INTERFACE_REQUIREMENT_KIND_MISMATCH")
        for requirement in interface.get("relation_requirements", []):
            _require_kind(resource_index, requirement.get("relation"), "relation", unresolved_code="INTERFACE_REQUIREMENT_DEFINITION_UNRESOLVED", mismatch_code="INTERFACE_REQUIREMENT_KIND_MISMATCH")
        for application in interface.get("constraint_applications", []):
            _require_kind(resource_index, application.get("constraint"), "constraint_definition", unresolved_code="INTERFACE_CONSTRAINT_DEFINITION_UNRESOLVED", mismatch_code="INTERFACE_CONSTRAINT_DEFINITION_KIND_MISMATCH")

    for implementation in package.get("interface_implementations", []):
        _require_kind(resource_index, implementation.get("entity_type"), "entity_type", unresolved_code="INTERFACE_IMPLEMENTATION_ENTITY_UNRESOLVED", mismatch_code="INTERFACE_IMPLEMENTATION_ENTITY_KIND_MISMATCH")
        _require_kind(resource_index, implementation.get("interface"), "interface", unresolved_code="INTERFACE_UNRESOLVED", mismatch_code="INTERFACE_IMPLEMENTATION_INTERFACE_KIND_MISMATCH")
        for mapping in implementation.get("property_mappings", []):
            _require_kind(resource_index, mapping.get("requirement"), "property", unresolved_code="INTERFACE_REQUIREMENT_DEFINITION_UNRESOLVED", mismatch_code="INTERFACE_REQUIREMENT_KIND_MISMATCH")
            _require_kind(resource_index, mapping.get("concrete"), "property", unresolved_code="INTERFACE_CONCRETE_DEFINITION_UNRESOLVED", mismatch_code="INTERFACE_REQUIREMENT_KIND_MISMATCH")
        for mapping in implementation.get("relation_mappings", []):
            _require_kind(resource_index, mapping.get("requirement"), "relation", unresolved_code="INTERFACE_REQUIREMENT_DEFINITION_UNRESOLVED", mismatch_code="INTERFACE_REQUIREMENT_KIND_MISMATCH")
            _require_kind(resource_index, mapping.get("concrete"), "relation", unresolved_code="INTERFACE_CONCRETE_DEFINITION_UNRESOLVED", mismatch_code="INTERFACE_REQUIREMENT_KIND_MISMATCH")
    return True


def validate_resolved_environment(packages: Sequence[Mapping[str, object]]):
    validate_namespace_providers(packages)
    resource_index = build_resource_index(packages)
    for package in packages:
        validate_namespace_exports(package, resource_index)
        validate_entity_types(package, resource_index)
        validate_relations(package, resource_index)
        validate_interfaces(package, resource_index)
    return resource_index
