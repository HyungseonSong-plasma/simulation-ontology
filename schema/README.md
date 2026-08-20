# Schema

Machine-readable structural schemas for the SOL v0.1 language live here.

## Authority boundary

Schemas implement accepted ADR semantics; they do not define or override semantic truth.

ADR-0018 distinguishes:

```text
AuthoringConstraint -> normalization -> NormalizedConstraintPayload
```

Normalized composition evidence retains contributor/context provenance outside or around the semantic payload.

## Current accepted design-stage coverage

Constraint and Predicate schemas:

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

All six ADR-0007 Constraint families have accepted focused schema slices. Family-specific normalization/intersection semantics remain semantic-validator work where JSON Schema is insufficient.

Interface schemas accepted under ADR-0025:

- `interface-definition-v0.1.schema.json`
- `interface-implementation-v0.1.schema.json`

The Interface slice structurally represents direct extension, canonical Property/Relation requirements, targeted reusable ConstraintApplications, and direct Entity Type implementation mappings. Extension closure, canonical reference resolution, target-admissibility, mapping completeness/convergence, and inherited Entity-type Interface guarantees remain semantic-validation responsibilities.

The repository intentionally does not expose permissive opaque schemas for unfinished language areas.

Next planned schema work includes accepted Value/Unit/PhysicalDimension graph representations, canonical entities/relations and identifiers/packages, canonical package integration, and Profile/mapping authoring contracts.
