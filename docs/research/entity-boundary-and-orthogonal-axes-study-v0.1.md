# Entity Boundary and Orthogonal Semantic Axes Study v0.1

**Status:** Research / candidate design principles  
**Date:** 2026-08-20  
**Scope:** SOL Core architecture

## 1. Motivation

Value and ValueDefinition analysis exposed a recurring modeling problem: semantic richness does not automatically imply that a construct needs graph identity. A complex typed datum may remain local data, while a simple named boundary or reusable model may require independent identity.

Palantir's ontology design provides a useful external analogy: object types represent entities/events, while property values and value types provide data and semantic typing/constraints without turning each value into an object. This supports separating semantic typing from independent object identity.

## 2. Candidate Entity Boundary Rule

### E1 — Semantic Identity Boundary

> A construct SHALL be modeled as an Entity when independent semantic identity is required for reference, relation participation, reuse, provenance, lifecycle, or independent constraint. Otherwise it SHOULD remain a typed construct when its semantics can be completely represented within its owning context.

Complexity alone is not an Entity criterion.

```text
complex data ≠ necessarily Entity
simple concept ≠ necessarily typed data

independent semantic identity → Entity
otherwise                    → typed construct
```

## 3. Candidate Orthogonality Principle

### E2 — Orthogonal Semantic Axes

> Semantic characteristics that can vary independently SHOULD be modeled as independent axes rather than encoded through combinatorial class hierarchies.

Observed independent axes include:

```text
Value shape
    ⟂ evaluation mechanism

Evaluation mechanism
    ⟂ semantic identity / reification

Physical dimension
    ⟂ semantic quantity identity
```

This avoids class explosion such as `IdentifiedVectorFunctionValue` and instead composes independent semantics.

## 4. Reverse application of E1

E1 was applied to existing and candidate SOL Core constructs to test whether it produces coherent classifications.

| Construct | E1 result | Rationale |
|---|---|---|
| Material | Entity | Independently referenced by regions, properties, models, mappings, and provenance. |
| Field | Entity | Equations, BCs, couplings, results, and backend mappings independently reference fields. |
| Equation / MathModel | Entity | Participates in independent relations, reuse, scope, coupling, and backend realization. |
| Domain / Boundary / Scope | Entity | Independently targeted by equations, BCs, materials, selections, and mappings. |
| Study | Entity | Owns solver/evaluation context, configuration, lifecycle, and backend realization; multiple studies may reference the same physics/model. |
| Value | Typed construct by default | Evaluated datum normally has no independent graph identity. Measurement/provenance should usually belong to a Measurement or other owning Entity. |
| ValueDefinition | Typed construct by default; Entity when identity is required | Evaluation mechanism and identity are orthogonal. Reuse/provenance/relations/lifecycle can justify an identified definition. |
| PhysicalDimension | Typed canonical structure by default | Machine-comparable exponent vector is sufficient for local dimension contracts. Named/external vocabulary resources may be referenced through adapters without forcing every dimension instance to be a Core Entity. |
| Unit | Reference/typed construct in Core boundary | Complete unit identity/catalogue belongs to metrology vocabulary/adapter per ADR 0005. SOL values need a resolvable unit reference, not ownership of every unit as a Core Entity. |
| Constraint | Typed construct by default; Entity when reusable/governed | Local cardinality/range/dimension constraints can be inline. Shared, versioned, independently referenced policies may justify identity. |
| Relation definition | Entity/schema resource | A semantic relation type has independent identity, domain/range/cardinality metadata, inheritance/versioning, and is reused across graph statements. |
| Relation instance / edge | Graph statement by default | The connection itself need not be reified unless it carries independent provenance, qualifiers, lifecycle, or other metadata. |
| Property definition | Entity/schema resource | Reusable semantic property definitions require stable identity and constraints. A concrete property value remains typed data. |
| Backend mapping | Typed construct by default; Entity when independently managed | Simple local mappings can be inline; versioned/reusable/provenance-bearing mappings may require identity. |
| Profile | Entity/schema resource | Composition, versioning, dependency, conflict policy, and backend/domain binding require independent identity. |
| Interface | Unresolved | E1 says it should be an Entity only if SOL needs independently reusable capability contracts. Its necessity must be established separately. |

## 5. Important distinctions found by the reverse test

### 5.1 Relation type vs relation instance

E1 suggests that a reusable relation definition such as `defined_on`, `depends_on`, or `targets` should have schema-level identity, while a particular edge is normally just a graph statement.

```text
RelationDefinition: targets     ← identified schema resource

BC_1 ── targets ──> Wall       ← graph statement
```

Only qualified/provenance-bearing edges need reification.

### 5.2 Property definition vs property value

The same distinction applies to properties:

```text
PropertyDefinition: thermal_conductivity   ← schema identity
Concrete value: 400 W/(m·K)                ← typed data
```

This is consistent with the Entity/Value boundary and with Palantir's distinction between ontology property definitions and property values.

### 5.3 Study passes E1 strongly

Study is not merely a bag of solver parameters. It can independently select/compose physics, define analysis/evaluation context, own execution configuration, participate in lifecycle/versioning, and map differently across MOOSE, COMSOL, and Ansys. It therefore has a strong Entity case.

### 5.4 PhysicalDimension does not need mandatory Core reification

ADR 0005 requires a canonical machine-comparable dimension representation, not mandatory graph identity. E1 therefore favors a canonical typed dimension vector in Core, while permitting mapping/reference to external identified dimension resources such as those in metrology vocabularies.

## 6. Palantir design evidence

Palantir distinguishes ontology object types from data/property values. Object types model real-world entities/events; properties characterize them; property values are data; value types add reusable semantic metadata and validation constraints without becoming object types. Link types separately define reusable relationships. This supports the broader SOL principle that semantic typing, constraints, and structural complexity do not by themselves require Entity identity.

Palantir also gives link types stable schema identity and metadata such as IDs and cardinality, which is consistent with SOL treating relation definitions as schema resources while keeping ordinary relation instances as graph statements.

## 7. Consequences if E1/E2 are accepted

1. `ReifiedValueDefinition` is unnecessary as a dedicated Core class.
2. Value remains typed evaluated data by default.
3. ValueDefinition mechanism and identity remain orthogonal.
4. PhysicalDimension can default to a canonical typed vector without contradicting ADR 0005.
5. Constraint and backend mapping can use inline forms until independent governance/identity is needed.
6. RelationDefinition and PropertyDefinition should be treated separately from relation/property instances and values.
7. SOL class hierarchies should not encode combinations of independent semantic axes.

## 8. Remaining questions

- Exact formal representation of schema resources versus model-instance Entities.
- Whether identified forms of typed constructs use a generic identity wrapper or native named declarations.
- Whether RelationDefinition and PropertyDefinition belong under a common SchemaResource abstraction.
- Whether Interface is required as an independently reusable capability contract.
- How qualified/reified relation instances are represented when edge-level provenance is required.
