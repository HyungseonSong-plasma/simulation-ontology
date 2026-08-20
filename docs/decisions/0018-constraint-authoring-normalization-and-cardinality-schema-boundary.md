# ADR-0018: Constraint Authoring, Normalization, and Cardinality Schema Boundary

**Status:** Accepted  
**Date:** 2026-08-20  
**Supplements:** ADR-0007, ADR-0012, ADR-0017

## Context

SOL v0.1 has accepted semantic Constraint families and composition rules, but repository artifacts currently mix compact authoring syntax and normalized package evidence. QRC authoring permits `min/max/exact` shorthand and omitted bounds, while Core cardinality evidence uses explicit `direction`, normalized `min/max`, canonical identity metadata, and `unbounded`.

Independent Validation concluded that one undifferentiated schema would create ambiguity over normalization, provenance placement, and cardinality authority.

## Decision

### 1. Separate authoring from normalized Constraint evidence

SOL v0.1 machine-readable tooling SHALL distinguish:

```text
AuthoringConstraint
      │ normalize
      ▼
NormalizedConstraintPayload
      │ + contributor/context evidence
      ▼
NormalizedConstraintEvidence
```

This is a serialization/compiler boundary, not a new SOL Core semantic primitive.

Accepted ADR semantics remain authoritative over JSON Schema and implementation code.

### 2. Authoring form may contain semantic shorthand

Authoring syntax MAY contain forms that normalize to one canonical payload. For Cardinality this includes `min`, `max`, and `exact`.

Zero supplied Cardinality bounds is valid and normalizes to the unconstrained interval:

```text
[0, unbounded]
```

A linter may report the form as redundant but SHALL NOT make it structurally or semantically invalid solely for having no explicit bound.

### 3. Cardinality normalization

For source/outgoing Cardinality authoring:

```text
L = max(min if present, exact if present), default 0
U = min(max if present, exact if present), default unbounded
```

- supplied negative/non-integer bound => `FAIL: QRC_BOUND_INVALID`;
- `L > U` => `FAIL: QRC_EMPTY_INTERVAL`;
- otherwise normalized interval is `[L,U]`.

### 4. Normalized Cardinality payload

The current v0.1 normalized Cardinality payload SHALL contain:

```text
type = cardinality
relation = canonical relation identity
direction = source
qualifier? = canonical target semantic type identity
min = non-negative integer
max = non-negative integer | unbounded
```

`exact` SHALL NOT appear in normalized payload.

Current v0.1 canonical schema covers source/outgoing Cardinality, which is the form exercised by ADR-0012 and ADR-0015/0016/0017. Incoming canonical syntax is deferred until required by a focused contract.

### 5. QRC remains Cardinality

Qualified Relation Cardinality is Cardinality with optional:

```text
qualifier.target_type
```

It is not a seventh Constraint family.

JSON Schema validates authoring/normalized structure but does not decide ADR-0012 semantic requirements such as closed-snapshot completeness, canonical qualifier resolution, stable target identity, subtype closure, or qualified counting.

### 6. Separate semantic payload from provenance evidence

`NormalizedConstraintPayload` contains the fields used for semantic equality, normalization, grouping, and primitive-specific intersection.

Provenance/context evidence SHALL be available to the normalized composition pipeline, but need not be stored as a literal field inside every payload.

Conceptually:

```text
NormalizedConstraintEvidence
├── constraint payload
└── stable contributor/context evidence
```

Payload equality SHALL NOT depend on provenance placement.

A current Core entry may use its stable package/canonical identity as contributor evidence.

### 7. Inline constraints need not become Entities

A reified ConstraintDefinition preserves stable semantic identity. Inline constraints remain inline unless independent identity is justified.

If normalization derives an evidence identity for an inline constraint, that identity SHALL be stable under sibling declaration reordering. Declaration order alone is not semantic identity.

### 8. Cardinality authority/projection remains ADR-0017

Canonical Core Cardinality Constraints normalize to the payload in section 4.

Relation-side `source_cardinality` remains only a projection/cache and is never composed as a second Constraint contributor.

Projection missing/mismatch remains:

```text
CARDINALITY_PROJECTION_MISSING
CARDINALITY_PROJECTION_MISMATCH
```

### 9. Other Constraint family payloads remain focused follow-up work

SOL v0.1 retains the six accepted families:

```text
Cardinality
Type
Value
Dimension
Compatibility
Conditional
```

This ADR does not invent a universal locator/path syntax or opaque field payloads for the other five families merely to publish a permissive union schema.

Each family shall receive focused machine-readable consolidation based on its accepted semantics. Conditional/predicate syntax follows only after the property/relation/value locator boundary is sufficiently defined.

### 10. Schema implementation slice

The immediate schema slice SHALL include:

- `constraint-cardinality-authoring-v0.1.schema.json`;
- `constraint-cardinality-normalized-v0.1.schema.json`;
- `qrc-v0.1.schema.json` as an authoring compatibility entry point aligned with the common authoring schema;
- minimal structural/normalization tests.

Backend runtime, installation, licensing, and production Adapter execution are not required for this schema work.

## Consequences

### Positive

- authoring shorthand cannot become a second semantic form during composition;
- `exact` and omitted bounds normalize deterministically;
- ADR-0017 Core cardinality evidence has a canonical schema target;
- provenance can be preserved without contaminating semantic payload equality;
- schema work does not pretend unfinished Constraint families are machine-defined.

### Costs

- tooling must run a normalization step between authoring and semantic composition;
- normalized evidence must retain stable contributor context outside or around the semantic payload;
- unified six-family schema completion remains incremental.

## Validation evidence

- `docs/research/sol-v0.1-constraint-schema-normalization-architecture-proposal-v0.1.md`
- `docs/validation/sol-v0.1-constraint-schema-normalization-architecture-independent-review-v0.1.md`
- `docs/research/sol-v0.1-constraint-schema-normalization-architecture-proposal-v0.2.md`
- `docs/validation/sol-v0.1-constraint-schema-normalization-architecture-focused-final-review-v0.2.md`

## Decision summary

SOL v0.1 separates Constraint authoring from canonical normalized evidence, consolidates Cardinality/QRC first, normalizes `min/max/exact` into explicit source `[min,max]` intervals, keeps provenance outside semantic payload equality, and defers unfinished Constraint-family field schemas rather than inventing opaque syntax.
