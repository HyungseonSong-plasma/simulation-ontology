# Schema

Machine-readable structural schemas for the SOL v0.1 language live here.

## Authority boundary

Schemas implement accepted ADR semantics; they do not define or override semantic truth.

ADR-0018 distinguishes:

```text
AuthoringConstraint -> normalization -> NormalizedConstraintPayload
```

Normalized composition evidence retains contributor/context provenance outside or around the semantic payload.

## Current Constraint coverage

Implemented:

- `constraint-cardinality-authoring-v0.1.schema.json`
- `constraint-cardinality-normalized-v0.1.schema.json`
- `qrc-v0.1.schema.json` as a Cardinality authoring compatibility entry point

The Cardinality/QRC slice is validated for design-stage use. QRC closed-world/type/identity/counting semantics remain semantic-validator responsibilities rather than JSON Schema rules.

Still under focused consolidation:

- Type
- Value
- Dimension
- Compatibility
- Conditional / predicate syntax

The repository intentionally does not expose permissive opaque schemas for unfinished Constraint families.

Other planned schema work includes canonical entities/relations, identifiers/packages, Interfaces, and Profile/mapping authoring contracts.
