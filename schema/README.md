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

Value/Unit/Dimension schemas accepted under ADR-0026/0027:

- `dimension-vector-v0.1.schema.json`
- `unit-reference-v0.1.schema.json`
- `exact-decimal-v0.1.schema.json`
- `value-v0.1.schema.json`
- `value-definition-inline-v0.1.schema.json`

The existing normalized Dimension and Value Constraint schemas reuse the shared DimensionVector and exact-decimal primitives. Tensor component-count validation, semantic metrology resolution, missing-unit policy, nonliteral format-provider normalization, dependency-driven reification, and reified ValueDefinition graph consistency remain semantic-validator/compiler responsibilities.

The repository intentionally does not expose an unqualified opaque Core expression/function/tabular payload. Nonliteral InlineValueDefinition payloads are owned by an explicitly identified format provider under ADR-0027.

Canonical package/resource schemas accepted under ADR-0028:

- `entity-type-definition-v0.1.schema.json`
- `property-definition-v0.1.schema.json`
- `relation-definition-v0.1.schema.json`
- `constraint-definition-v0.1.schema.json`
- `ontology-package-normalized-v0.1.schema.json`

The normalized package boundary separates distribution metadata from semantic namespaces and canonical resource identity. It requires exact resolved package dependencies, explicit namespace export tables, closed resource collections, canonical references, and reusable ConstraintDefinition wrappers. Cross-resource identity uniqueness, namespace-provider uniqueness, reference kind resolution, relation endpoint resolution, cardinality projection authority, Interface referential completeness, normalized allowed-pair convergence, and resolved-environment dependency checks remain semantic-validator responsibilities.

The next schema work is driven only by the **Minimal Thermal and Plasma/QRC reference-model validation** if those cases expose a genuine missing normalized resource/model-document boundary. Production Profile/Adapter schemas and backend execution contracts remain outside the current design-stage closure gate.
