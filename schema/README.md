# Schema

Machine-readable structural schemas for the SOL v0.1 language live here.

## Authority boundary

Schemas implement accepted ADR semantics; they do not define or override semantic truth. Semantic validators handle invariants that JSON Schema cannot express deterministically.

ADR-0018 distinguishes:

```text
AuthoringConstraint -> normalization -> NormalizedConstraintPayload
```

Normalized composition evidence retains contributor/context provenance outside or around the semantic payload.

## Current accepted design-stage coverage

### Constraint and Predicate schemas

- `constraint-cardinality-authoring-v0.1.schema.json`
- `constraint-cardinality-normalized-v0.1.schema.json`
- `qrc-v0.1.schema.json`
- `constraint-type-authoring-v0.1.schema.json`
- `constraint-type-normalized-v0.1.schema.json`
- `constraint-dimension-authoring-v0.1.schema.json`
- `constraint-dimension-normalized-v0.1.schema.json`
- `constraint-value-authoring-v0.1.schema.json`
- `constraint-value-normalized-v0.1.schema.json`
- `constraint-compatibility-authoring-v0.1.schema.json`
- `constraint-compatibility-normalized-v0.1.schema.json`
- `predicate-authoring-v0.1.schema.json`
- `predicate-normalized-v0.1.schema.json`
- `constraint-conditional-authoring-v0.1.schema.json`
- `constraint-conditional-normalized-v0.1.schema.json`

All six ADR-0007 Constraint families have accepted focused schema slices.

### Interface schemas — ADR-0025

- `interface-definition-v0.1.schema.json`
- `interface-implementation-v0.1.schema.json`

Semantic validation owns extension closure, canonical reference resolution, target admissibility, mapping completeness/convergence, and inherited Entity-type Interface guarantees.

### Value / Unit / Dimension / ValueDefinition schemas — ADR-0026/0027

- `dimension-vector-v0.1.schema.json`
- `unit-reference-v0.1.schema.json`
- `exact-decimal-v0.1.schema.json`
- `value-v0.1.schema.json`
- `value-definition-inline-v0.1.schema.json`

Semantic validation/compiler logic owns tensor component-count validation, metrology resolution, missing-unit policy, nonliteral format-provider normalization, dependency-driven reification, and reified ValueDefinition graph consistency.

### Canonical package/resource schemas — ADR-0028

- `entity-type-definition-v0.1.schema.json`
- `property-definition-v0.1.schema.json`
- `relation-definition-v0.1.schema.json`
- `constraint-definition-v0.1.schema.json`
- `ontology-package-normalized-v0.1.schema.json`

Semantic validation owns cross-resource identity uniqueness, namespace-provider uniqueness, exact resolved-dependency checks, reference kind resolution, relation endpoint resolution, cardinality projection authority, Interface referential completeness, and normalized allowed-pair convergence.

### Resolved model snapshot — ADR-0029

- `resolved-model-snapshot-v0.1.schema.json`

A design-stage reference snapshot is closed, names an exact normalized ontology environment, contains typed model-instance entities with canonical PropertyDefinition assignments, and stores explicit canonical RelationDefinition edges. Instance uniqueness, endpoint/subtype validation, cardinality, Interface applications, Dimension/metrology, and QRC evaluation remain semantic-validator responsibilities.

## Accepted reference gates

- **Minimal Thermal reference model — PASS**
- **Minimal Plasma/QRC reference model — PASS**

These fixtures validate the focused design-stage package/model boundary; they are not production Adapter or backend runtime schemas.

## Explicitly deferred / outside current closure gate

- production Profile/Adapter package authoring;
- backend execution/runtime schemas;
- namespace federation/augmentation;
- complete general-purpose model-document/application syntax beyond ADR-0029 reference snapshots;
- richer future PropertyDefinition metadata where not required by current reference cases.
