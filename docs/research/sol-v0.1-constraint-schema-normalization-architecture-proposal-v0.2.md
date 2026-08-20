# SOL v0.1 Constraint Schema Normalization Architecture Proposal v0.2

**Role:** Research  
**Date:** 2026-08-20  
**Revision scope:** CS-V3 and CS-V4 only

## 1. Preserved architecture

This revision preserves:

- `AuthoringConstraint -> normalize -> NormalizedConstraint` separation;
- accepted ADR semantics as authority over schemas/tooling;
- Cardinality/QRC as the first fully consolidated family;
- normalized Cardinality interval with explicit `direction: source`, `min`, `max`, optional qualifier, and no `exact`;
- QRC as Cardinality rather than a seventh family;
- semantic QRC evaluation outside JSON Schema;
- deferral of unfinished Type/Value/Dimension/Compatibility/Conditional payload syntax rather than accepting opaque arbitrary objects.

## 2. Boundless Cardinality authoring remains valid

Authoring Cardinality preserves ADR-0012 optional bounds:

```yaml
type: cardinality
relation: <relation-id>
qualifier?:
  target_type: <semantic-type-id>
min?: non-negative integer
max?: non-negative integer
exact?: non-negative integer
```

Zero supplied bounds is valid and normalizes to:

```text
[0, unbounded]
```

Such a constraint is semantically redundant/no-op at generic level but is not structurally invalid.

A style checker MAY warn about redundant boundless authoring, but linting SHALL NOT be confused with SOL semantic/schema validity.

## 3. Cardinality normalization

For authored bounds:

```text
L = max(min if present, exact if present), default 0
U = min(max if present, exact if present), default unbounded
```

- negative/non-integer supplied bound -> `FAIL: QRC_BOUND_INVALID`;
- `L > U` -> `FAIL: QRC_EMPTY_INTERVAL`;
- otherwise canonical normalized interval is `[L,U]`.

Normalized Cardinality payload:

```yaml
type: cardinality
relation: <canonical-relation-id>
direction: source
qualifier?:
  target_type: <canonical-semantic-type-id>
min: <non-negative integer>
max: <non-negative integer | unbounded>
```

`exact` is forbidden in normalized payload.

## 4. Separate semantic payload from evidence envelope

The canonical semantic Constraint payload SHALL NOT embed provenance solely to make normalization evidence self-contained.

Instead distinguish:

```text
NormalizedConstraintEvidence
├── constraint: NormalizedConstraintPayload
└── provenance/context evidence
```

### NormalizedConstraintPayload

Contains only semantic fields required for normalization/equality/intersection of that family.

For Cardinality:

```text
type
relation
direction
qualifier?
min
max
```

Payload equality and primitive-specific intersection SHALL NOT depend on provenance metadata.

### NormalizedConstraintEvidence

Wraps or associates the payload with enough contributor/context evidence to satisfy ADR-0007 diagnostics.

Conceptually:

```yaml
constraint: <NormalizedConstraintPayload>
provenance:
  source_id: <stable contributor/context identity>
  source_kind?: <diagnostic category>
```

The exact `provenance` object field spelling and package locator syntax are not made normative here. A containing package declaration MAY provide equivalent stable contributor/context evidence.

The invariant is semantic:

```text
Every effective normalized constraint used in composition
MUST be traceable to at least one stable contributing context/source.
```

## 5. Reified versus inline constraints

A reified `ConstraintDefinition` preserves its stable semantic identity.

An inline Constraint need not be promoted to an Entity. Its evidence identity may be derived deterministically from its containing declaration plus an unambiguous local key/index supplied by the package normalization layer.

Declaration order alone SHALL NOT be used as semantic identity if reordering would change the identifier.

## 6. Current Core cardinality evidence compatibility

Entries such as:

```yaml
core_cardinality_solved_by_source:
  type: cardinality
  canonical_id: core.cardinality.solved_by.source
  relation: solved_by
  direction: source
  min: 0
  max: unbounded
```

contain semantic payload plus package-level identity metadata in one current YAML entry.

During canonical normalization:

```text
payload = {
  type: cardinality,
  relation: solved_by,
  direction: source,
  min: 0,
  max: unbounded
}

evidence contributor identity = core.cardinality.solved_by.source
```

Therefore current package artifacts can be normalized without requiring a new inline `provenance` field.

## 7. Schema implementation boundary

After contract acceptance, implement three structural schemas:

1. `constraint-cardinality-authoring-v0.1.schema.json`
   - accepts optional `min/max/exact` including zero supplied bounds;
   - rejects negative/non-integer supplied bounds;
   - authoring `max` is finite integer when present;
   - accepts optional QRC `qualifier.target_type`.

2. `constraint-cardinality-normalized-v0.1.schema.json`
   - requires `type`, `relation`, `direction`, `min`, `max`;
   - `direction` is `source` for current v0.1 normalized contract;
   - `max` is non-negative integer or literal `unbounded`;
   - `exact` is forbidden;
   - qualifier, when present, requires canonical target type string structurally.

3. a small evidence-envelope schema MAY be introduced only if the package normalization contract can define contributor identity without inventing a generic path language. Otherwise evidence-envelope serialization remains the next package-schema task while the invariant is tested in the normalizer.

Existing `qrc-v0.1.schema.json` becomes a compatibility/authoring entry point aligned with schema (1), not a parallel semantic authority.

## 8. Deterministic checks

### CS-V3-A — no bounds

```yaml
type: cardinality
relation: solved_by
```

Expected authoring structural result: PASS.
Expected normalization: `[0,unbounded]`.

### CS-V3-B — empty mixed bounds

```yaml
min: 3
exact: 2
```

Expected authoring structure: PASS.
Expected semantic normalization: `FAIL: QRC_EMPTY_INTERVAL`.

### CS-V4-A — payload without inline provenance

A normalized Cardinality payload is structurally valid even when provenance is carried by its containing normalized evidence record/package context.

Expected: payload PASS; composition pipeline must still require traceable contributor evidence.

### CS-V4-B — provenance changes do not change payload equality

Two evidence records with identical normalized Cardinality payload but different contributing sources have equal semantic payloads but preserve two provenance contributors for diagnostics/composition evidence.

### CS-V4-C — order-derived inline identity

If an inline constraint identity changes solely because sibling declaration order changes, the normalization is invalid/non-canonical.

## 9. Research verdict

CS-V3 and CS-V4 are resolved without changing accepted Constraint semantics.

**Ready for focused final Validation.**
