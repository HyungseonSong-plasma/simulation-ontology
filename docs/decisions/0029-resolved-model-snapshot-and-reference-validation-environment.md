# ADR-0029 — Resolved Model Snapshot and Reference Validation Environment

**Status:** Accepted  
**Date:** 2026-08-20  
**Scope:** SOL v0.1 design-stage reference-model validation

## Context

ADR-0028 defines canonical normalized ontology-package resources, but intentionally does not define a general model-instance document. The first minimal Thermal reference-model stress test confirmed that Core thermal semantics are representable across MOOSE, COMSOL, and Ansys, while exposing two design-stage fixture-readiness gaps:

1. no deterministic closed model-instance snapshot input;
2. no committed normalized reference ontology environment with durable canonical IDs for the model to reference.

Independent review also found that attaching an unlabeled `value_definition` directly to a generic Entity instance would bypass SOL Property semantics.

Supporting artifacts:

- `docs/research/sol-v0.1-minimal-thermal-reference-model-v0.1.md`
- `docs/validation/sol-v0.1-minimal-thermal-reference-model-independent-review-v0.1.md`
- `docs/research/sol-v0.1-resolved-model-snapshot-and-thermal-reference-environment-proposal-v0.2.md`
- `docs/validation/sol-v0.1-resolved-model-snapshot-reference-environment-focused-final-review.md`

## Decision

### 1. Focused closed `ResolvedModelSnapshot`

SOL v0.1 design-stage reference fixtures use a normalized closed snapshot containing:

```text
snapshot_state = closed
ontology_environment[] = exact package identities
entities[]
relations[]
```

The snapshot is immutable validation input. Validators do not search external model fragments for additional model entities or relation occurrences.

This explicit closure satisfies the closed-snapshot requirement needed by QRC evaluation.

### 2. Model-instance identity

Each focused Entity instance has one nonempty opaque model-instance ID unique within the snapshot.

The final globally unique model-instance URI syntax remains deferred under ADR-0009.

Model-instance identity is distinct from:

- schema-resource canonical identity;
- backend-local tag/name/handle.

### 3. Entity instance shape

Focused normalized Entity instance:

```text
{
  id,
  type,
  properties[]
}
```

`type` is a canonical EntityTypeDefinition ID resolved in the exact ontology environment.

`properties` is required and may be empty.

### 4. Lightweight PropertyAssignment

Focused normalized PropertyAssignment:

```text
{
  property,
  value_definition
}
```

Rules:

1. `property` is a canonical PropertyDefinition ID;
2. `value_definition` is a normalized InlineValueDefinition under ADR-0026/0027;
3. the assignment has no independent model-instance identity;
4. one Entity instance may not contain duplicate assignments for the same PropertyDefinition;
5. assignment order is semantically irrelevant;
6. unknown PropertyDefinition identity fails before value evaluation;
7. backend parameter/property labels are not canonical PropertyDefinition identity.

General PropertyDefinition applicability, lifecycle, provenance, and reified property-assignment semantics remain deferred.

### 5. Relation edge shape

Focused relation instances remain lightweight graph statements:

```text
{
  relation,
  source,
  target
}
```

where:

- `relation` is a canonical RelationDefinition ID;
- `source` and `target` are model-instance IDs in the same snapshot.

The normalized snapshot rejects duplicate identical `(relation, source, target)` triples.

No edge-level identity/provenance/reification is introduced in this focused schema.

### 6. Semantic relation validation

Validators resolve each relation through the exact ontology environment and enforce:

- source/target existence;
- RelationDefinition kind;
- canonical type-or-subtype endpoint matching;
- authoritative `allowed_pairs` where applicable;
- accepted source-cardinality authority/projection semantics;
- QRC against canonical target identity/type in the closed snapshot when present.

### 7. Exact ontology environment

`ontology_environment` contains exact package identities `(name, SemVer version)`.

For focused reference validation, the supplied resolved package set must match the snapshot-declared environment exactly.

Dependency ranges and undeclared extra package providers do not participate in normalized model interpretation.

### 8. Reference package fixtures

The design stage may use small normalized ADR-0028-conforming package fixtures rather than requiring a production compiler or release package.

The first environment contains:

```text
@simulation-ontology/core-reference-fixture
  provides namespace: sol

@simulation-ontology/thermal-reference
  provides namespace: thermal-ref
  exact dependency: core-reference-fixture
```

Rules:

1. fixture package identity is not semantic identity;
2. canonical IDs assigned to included semantic resources are durable committed bindings;
3. a future production package/provider may reuse those canonical IDs;
4. every included Core resource preserves its complete accepted semantics;
5. an included relation is not narrowed merely because the reference case uses a subset of its valid domain/range/allowed pairs.

### 9. Design-stage validation order

```text
1. validate exact ontology environment
2. build ADR-0028 canonical resource index
3. validate snapshot structure/closure
4. resolve unique model-instance identities/types
5. resolve PropertyDefinition assignments
6. validate InlineValueDefinitions
7. resolve relation triples
8. validate endpoint/subtype/allowed-pair semantics
9. validate ordinary cardinality
10. validate QRC when present
11. evaluate focused semantic metrology evidence when supplied
```

Backend installation, license, solver runtime, and production Adapter state are outside this sequence.

## Consequences

### Positive

- Thermal and Plasma/QRC cases can be one deterministic machine-readable input.
- Reference validation reuses canonical package identities rather than ad hoc local names.
- QRC obtains explicit closed-graph evidence.
- Property values preserve semantic Property identity without reifying assignment objects.
- The design stage remains independent of backend runtime availability.

### Costs

- A focused model-snapshot schema and semantic validator are required.
- Durable canonical IDs must be committed for reference package resources.
- The reference package is deliberately not a full production Core release.

## Deferred

- production Core package compiler;
- global model-instance URI algorithm;
- general PropertyDefinition applicability/domain contract;
- reified relation instances;
- reified ValueDefinition mechanism-payload serialization;
- open/distributed model fragments;
- runtime/result/execution state;
- backend handles and Adapter execution.

## Decision summary

SOL v0.1 design-stage reference validation uses a closed `ResolvedModelSnapshot` bound to an exact ADR-0028 ontology environment. Model entities have opaque snapshot-local identity, canonical EntityType identity, explicit lightweight PropertyAssignments using canonical PropertyDefinitions, and canonical relation triples. Small normalized package fixtures with durable canonical IDs are sufficient for current Thermal and Plasma/QRC stress validation without requiring production backend or compiler infrastructure.
