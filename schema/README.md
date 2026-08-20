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

All six ADR-0007 Constraint families now have accepted focused schema slices.

Cardinality/QRC semantic evaluation such as closed-world counting, stable identity, qualifier subtype closure, and interval normalization remains semantic-validator work rather than JSON Schema logic.

The ADR-0019 Type slice leaves canonical identity resolution, Entity-Type-versus-Interface identity, subtype closure, finite target-family/allowed-pair narrowing, Type intersection, and semantic-versus-representability axis separation to semantic validation.

The ADR-0020 Dimension slice uses sparse seven-axis authoring, full normalized vectors, explicit `vector: {}` DimensionOne, integer-valued exponents, and equality-based Dimension intersection. Unit conversion/metrology metadata is outside the Dimension payload.

The ADR-0021 Value slice uses numeric intervals and finite scalar allowed sets. Every normalized numeric operand uses an exact-decimal coefficient/exponent representation; empty Value results are separated from conflict classification; numeric normalization requires a resolved or not-required comparison space.

The ADR-0022 Compatibility slice uses canonical semantic criterion identity, typed `schema | model_instance` operands, ordered/symmetric binary obligation identity, explicit semantic `INDETERMINATE`, and deterministic empty-set / mixed-state aggregation. Backend identity and backend runtime/license state are outside the semantic payload.

The ADR-0023/0024 Conditional slice uses exact-one EvaluationReference bindings, `PRESENT | ABSENT | UNRESOLVED` lookup, Compare/Membership/Exists/Boolean predicates, three-valued truth plus separate EvaluationFailure, nonempty ordinary-only consequents, and cross-family `FAIL > INDETERMINATE > PASS` aggregation. Normalized Membership has set semantics and cannot retain duplicates.

The repository intentionally does not expose permissive opaque schemas for unfinished language areas.

Next planned schema work includes canonical entities/relations, identifiers/packages, **Interfaces**, accepted Value/Unit/PhysicalDimension graph representations, and Profile/mapping authoring contracts.
