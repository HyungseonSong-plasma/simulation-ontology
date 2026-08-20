"""Focused semantic validation for ADR-0029 resolved model snapshots."""

from __future__ import annotations

from collections import defaultdict
from typing import Mapping, Sequence

from tests.package_semantics import (
    PackageContractError,
    build_resource_index,
    validate_resolved_environment,
)


class ModelSnapshotError(ValueError):
    """Deterministic ADR-0029 reference-model validation diagnostic."""


def _package_key(package: Mapping[str, object]) -> tuple[str, str]:
    identity = package.get("package")
    if not isinstance(identity, Mapping):
        raise ModelSnapshotError("MODEL_ONTOLOGY_ENVIRONMENT_INVALID")
    name = identity.get("name")
    version = identity.get("version")
    if not isinstance(name, str) or not name or not isinstance(version, str) or not version:
        raise ModelSnapshotError("MODEL_ONTOLOGY_ENVIRONMENT_INVALID")
    return name, version


def _snapshot_environment(snapshot: Mapping[str, object]) -> set[tuple[str, str]]:
    environment = snapshot.get("ontology_environment")
    if not isinstance(environment, list) or not environment:
        raise ModelSnapshotError("MODEL_ONTOLOGY_ENVIRONMENT_INVALID")
    result: set[tuple[str, str]] = set()
    for entry in environment:
        if not isinstance(entry, Mapping):
            raise ModelSnapshotError("MODEL_ONTOLOGY_ENVIRONMENT_INVALID")
        name = entry.get("package")
        version = entry.get("version")
        if not isinstance(name, str) or not name or not isinstance(version, str) or not version:
            raise ModelSnapshotError("MODEL_ONTOLOGY_ENVIRONMENT_INVALID")
        key = (name, version)
        if key in result:
            raise ModelSnapshotError("MODEL_ONTOLOGY_ENVIRONMENT_DUPLICATE")
        result.add(key)
    return result


def _require_resource_kind(resource_index, identifier: object, kind: str, unresolved: str, mismatch: str):
    if not isinstance(identifier, str) or not identifier:
        raise ModelSnapshotError(unresolved)
    resolved = resource_index.get(identifier)
    if resolved is None:
        raise ModelSnapshotError(unresolved)
    if resolved[0] != kind:
        raise ModelSnapshotError(mismatch)
    return resolved[1]


def _build_entity_type_parents(resource_index):
    parents: dict[str, str | None] = {}
    for identifier, (kind, resource) in resource_index.items():
        if kind != "entity_type":
            continue
        parent = resource.get("is_a")
        if parent is not None:
            _require_resource_kind(
                resource_index,
                parent,
                "entity_type",
                "MODEL_ENTITY_PARENT_UNRESOLVED",
                "MODEL_ENTITY_PARENT_KIND_MISMATCH",
            )
        parents[identifier] = parent

    for identifier in parents:
        seen: set[str] = set()
        current: str | None = identifier
        while current is not None:
            if current in seen:
                raise ModelSnapshotError("MODEL_ENTITY_TYPE_CYCLE")
            seen.add(current)
            current = parents.get(current)
    return parents


def _matches_type(actual: str, required: str, parents: Mapping[str, str | None]) -> bool:
    current: str | None = actual
    seen: set[str] = set()
    while current is not None:
        if current == required:
            return True
        if current in seen:
            raise ModelSnapshotError("MODEL_ENTITY_TYPE_CYCLE")
        seen.add(current)
        current = parents.get(current)
    return False


def _matches_any(actual: str, required: Sequence[str], parents: Mapping[str, str | None]) -> bool:
    return any(_matches_type(actual, item, parents) for item in required)


def _relation_source_applicable(relation: Mapping[str, object], source_type: str, parents) -> bool:
    endpoint = relation.get("endpoint_contract")
    if not isinstance(endpoint, Mapping):
        raise ModelSnapshotError("MODEL_RELATION_ENDPOINT_CONTRACT_INVALID")
    kind = endpoint.get("kind")
    if kind == "typed":
        domain = endpoint.get("domain")
        if domain is None:
            return True
        if not isinstance(domain, list):
            raise ModelSnapshotError("MODEL_RELATION_ENDPOINT_CONTRACT_INVALID")
        return _matches_any(source_type, domain, parents)
    if kind == "allowed_pairs":
        pairs = endpoint.get("pairs")
        if not isinstance(pairs, list):
            raise ModelSnapshotError("MODEL_RELATION_ENDPOINT_CONTRACT_INVALID")
        return any(
            isinstance(pair, Mapping)
            and isinstance(pair.get("source"), str)
            and _matches_type(source_type, pair["source"], parents)
            for pair in pairs
        )
    raise ModelSnapshotError("MODEL_RELATION_ENDPOINT_CONTRACT_INVALID")


def _validate_relation_endpoint(relation: Mapping[str, object], source_type: str, target_type: str, parents) -> None:
    endpoint = relation.get("endpoint_contract")
    if not isinstance(endpoint, Mapping):
        raise ModelSnapshotError("MODEL_RELATION_ENDPOINT_CONTRACT_INVALID")
    kind = endpoint.get("kind")

    if kind == "typed":
        domain = endpoint.get("domain")
        target_range = endpoint.get("range")
        if domain is not None and (not isinstance(domain, list) or not _matches_any(source_type, domain, parents)):
            raise ModelSnapshotError("MODEL_RELATION_SOURCE_TYPE_MISMATCH")
        if target_range is not None and (
            not isinstance(target_range, list) or not _matches_any(target_type, target_range, parents)
        ):
            raise ModelSnapshotError("MODEL_RELATION_TARGET_TYPE_MISMATCH")
        return

    if kind == "allowed_pairs":
        pairs = endpoint.get("pairs")
        if not isinstance(pairs, list):
            raise ModelSnapshotError("MODEL_RELATION_ENDPOINT_CONTRACT_INVALID")
        for pair in pairs:
            if not isinstance(pair, Mapping):
                continue
            pair_source = pair.get("source")
            targets = pair.get("targets")
            if not isinstance(pair_source, str) or not isinstance(targets, list):
                continue
            if _matches_type(source_type, pair_source, parents) and _matches_any(target_type, targets, parents):
                return
        raise ModelSnapshotError("MODEL_RELATION_ALLOWED_PAIR_MISMATCH")

    raise ModelSnapshotError("MODEL_RELATION_ENDPOINT_CONTRACT_INVALID")


def _bounded_interval_contains(count: int, minimum: object, maximum: object) -> bool:
    if isinstance(minimum, bool) or not isinstance(minimum, int) or minimum < 0:
        raise ModelSnapshotError("MODEL_CARDINALITY_AUTHORITY_INVALID")
    if count < minimum:
        return False
    if maximum == "unbounded":
        return True
    if isinstance(maximum, bool) or not isinstance(maximum, int) or maximum < 0:
        raise ModelSnapshotError("MODEL_CARDINALITY_AUTHORITY_INVALID")
    return count <= maximum


def validate_resolved_model_snapshot(
    snapshot: Mapping[str, object],
    packages: Sequence[Mapping[str, object]],
):
    if snapshot.get("snapshot_state") != "closed":
        raise ModelSnapshotError("MODEL_SNAPSHOT_NOT_CLOSED")

    try:
        validate_resolved_environment(packages)
    except PackageContractError as exc:
        raise ModelSnapshotError(f"MODEL_ONTOLOGY_PACKAGE_INVALID:{exc}") from exc

    declared = _snapshot_environment(snapshot)
    supplied = {_package_key(package) for package in packages}
    if declared != supplied:
        raise ModelSnapshotError("MODEL_ONTOLOGY_ENVIRONMENT_MISMATCH")

    resource_index = build_resource_index(packages)
    parents = _build_entity_type_parents(resource_index)

    entities = snapshot.get("entities")
    if not isinstance(entities, list) or not entities:
        raise ModelSnapshotError("MODEL_ENTITIES_INVALID")

    instance_index: dict[str, Mapping[str, object]] = {}
    instance_types: dict[str, str] = {}
    for entity in entities:
        if not isinstance(entity, Mapping):
            raise ModelSnapshotError("MODEL_ENTITY_INVALID")
        identifier = entity.get("id")
        if not isinstance(identifier, str) or not identifier:
            raise ModelSnapshotError("MODEL_ENTITY_ID_INVALID")
        if identifier in instance_index:
            raise ModelSnapshotError("MODEL_ENTITY_ID_DUPLICATE")
        entity_type = entity.get("type")
        _require_resource_kind(
            resource_index,
            entity_type,
            "entity_type",
            "MODEL_ENTITY_TYPE_UNRESOLVED",
            "MODEL_ENTITY_TYPE_KIND_MISMATCH",
        )
        instance_index[identifier] = entity
        instance_types[identifier] = entity_type

        properties = entity.get("properties")
        if not isinstance(properties, list):
            raise ModelSnapshotError("MODEL_PROPERTIES_INVALID")
        seen_properties: set[str] = set()
        for assignment in properties:
            if not isinstance(assignment, Mapping):
                raise ModelSnapshotError("MODEL_PROPERTY_ASSIGNMENT_INVALID")
            property_id = assignment.get("property")
            _require_resource_kind(
                resource_index,
                property_id,
                "property",
                "MODEL_PROPERTY_DEFINITION_UNRESOLVED",
                "MODEL_PROPERTY_DEFINITION_KIND_MISMATCH",
            )
            if property_id in seen_properties:
                raise ModelSnapshotError("MODEL_PROPERTY_ASSIGNMENT_DUPLICATE")
            seen_properties.add(property_id)
            if "value_definition" not in assignment:
                raise ModelSnapshotError("MODEL_VALUE_DEFINITION_MISSING")

    relations = snapshot.get("relations")
    if not isinstance(relations, list):
        raise ModelSnapshotError("MODEL_RELATIONS_INVALID")

    relation_counts: dict[tuple[str, str], set[str]] = defaultdict(set)
    seen_triples: set[tuple[str, str, str]] = set()
    relation_resources: dict[str, Mapping[str, object]] = {}

    for edge in relations:
        if not isinstance(edge, Mapping):
            raise ModelSnapshotError("MODEL_RELATION_EDGE_INVALID")
        relation_id = edge.get("relation")
        relation = _require_resource_kind(
            resource_index,
            relation_id,
            "relation",
            "MODEL_RELATION_DEFINITION_UNRESOLVED",
            "MODEL_RELATION_DEFINITION_KIND_MISMATCH",
        )
        relation_resources[relation_id] = relation
        source = edge.get("source")
        target = edge.get("target")
        if source not in instance_index:
            raise ModelSnapshotError("MODEL_RELATION_SOURCE_UNRESOLVED")
        if target not in instance_index:
            raise ModelSnapshotError("MODEL_RELATION_TARGET_UNRESOLVED")
        triple = (relation_id, source, target)
        if triple in seen_triples:
            raise ModelSnapshotError("MODEL_RELATION_TRIPLE_DUPLICATE")
        seen_triples.add(triple)

        _validate_relation_endpoint(relation, instance_types[source], instance_types[target], parents)
        relation_counts[(relation_id, source)].add(target)

    # Cardinality is evaluated for every model instance to which the included
    # RelationDefinition's source contract applies, even when that instance has
    # no corresponding edge in the closed snapshot.
    for relation_id, (kind, relation) in resource_index.items():
        if kind != "relation":
            continue
        projection = relation.get("source_cardinality_projection")
        if projection is None:
            continue
        if not isinstance(projection, Mapping):
            raise ModelSnapshotError("MODEL_CARDINALITY_AUTHORITY_INVALID")
        for source_id, source_type in instance_types.items():
            if not _relation_source_applicable(relation, source_type, parents):
                continue
            count = len(relation_counts.get((relation_id, source_id), set()))
            if not _bounded_interval_contains(count, projection.get("min"), projection.get("max")):
                raise ModelSnapshotError("MODEL_CARDINALITY_VIOLATION")

    return {
        "resource_index": resource_index,
        "instance_index": instance_index,
        "instance_types": instance_types,
        "relation_triples": seen_triples,
    }
