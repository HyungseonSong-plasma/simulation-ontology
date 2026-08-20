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

Implemented and independently accepted for design-stage use:

- `constraint-cardinality-authoring-v0.1.schema.json`
- `constraint-cardinality-normalized-v0.1.schema.json`
- `qrc-v0.1.schema.json` as a Cardinality authoring compatibility entry point
- `constraint-type-authoring-v0.1.schema.json`
- `constraint-type-normalized-v0.1.schema.json`

Cardinality/QRC semantic evaluation such as closed-world counting, stable identity, qualifier subtype closure, and interval normalization remains semantic-validator work rather than JSON Schema logic.

The ADR-0019 Type slice likewise leaves canonical identity resolution, Entity-Type-versus-Interface identity, subtype closure, finite target-family/allowed-pair narrowing, Type intersection, and semantic-versus-representability axis separation to semantic validation.

Still under focused consolidation:

- Value
- Dimension
- Compatibility
- Conditional / predicate syntax

The repository intentionally does not expose permissive opaque schemas for unfinished Constraint families.

Other planned schema work includes canonical entities/relations, identifiers/packages, Interfaces, and Profile/mapping authoring contracts.
