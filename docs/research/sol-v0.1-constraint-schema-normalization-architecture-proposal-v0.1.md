# SOL v0.1 Constraint Schema Normalization Architecture Proposal v0.1

**Role:** Research  
**Date:** 2026-08-20

## 1. Objective

Define the minimum machine-readable architecture needed to transcribe the accepted Constraint semantics from ADR-0007, ADR-0012, ADR-0015, ADR-0016, and ADR-0017 without making JSON Schema syntax itself the semantic authority.

This proposal focuses on the boundary between human/package authoring forms and canonical normalized Constraint evidence. It does not invent new Type/Value/Dimension/Compatibility semantics.

## 2. Current inconsistency

The repository currently contains two different Constraint-shaped representations:

### QRC authoring schema

`schema/qrc-v0.1.schema.json` accepts compact authoring such as:

```yaml
type: cardinality
relation: products
qualifier:
  target_type: NegativeIonSpecies
min: 1
```

It permits `min/max/exact` as authoring bounds and does not carry canonical identity/provenance or an explicit normalized direction.

### Core normalized package constraints

`ontology/core/constraints.yaml` now carries entries such as:

```yaml
type: cardinality
canonical_id: core.cardinality.solved_by.source
relation: solved_by
direction: source
min: 0
max: unbounded
```

Treating both as one undifferentiated syntax would allow validators to disagree over missing fields, `exact` versus normalized intervals, and whether `unbounded` is syntax or semantic state.

## 3. Decision candidate — two machine-readable layers

SOL v0.1 SHOULD distinguish:

```text
AuthoringConstraint
      │ normalize
      ▼
NormalizedConstraint
```

### AuthoringConstraint

Purpose:

- concise human/package authoring;
- may use accepted shorthand such as `exact`;
- context may be inherited from the containing Entity/Interface/Profile declaration;
- may omit values that have an unambiguous canonical normalization rule.

### NormalizedConstraint

Purpose:

- deterministic validation/composition input;
- one canonical form per semantic meaning within the supported family;
- preserves enough provenance to explain composition/conflicts;
- never relies on declaration order or backend-specific syntax;
- is the machine-readable form used for equality, deduplication, intersection grouping, and validation evidence.

This is a serialization/compiler boundary, not a new SOL Core semantic primitive.

## 4. Semantic authority

Authority order SHALL be:

```text
Accepted ADR semantics
        ↓
Normalization contract
        ↓
NormalizedConstraint
        ↓
JSON Schema structural validation / validator implementation
```

JSON Schema SHALL reject malformed structures but SHALL NOT redefine accepted semantic intersection rules.

An authoring form and its normalized form are not two independent Constraint contributors. The authoring form compiles into the normalized form.

## 5. Common normalized evidence envelope

Every NormalizedConstraint SHALL preserve:

```text
family/type discriminator
canonical semantic payload
provenance sufficient to identify its contributor/context
```

For a reified ConstraintDefinition, stable identity is preserved. For an inline constraint, the containing declaration plus deterministic local position/key MAY supply provenance/identity; v0.1 does not require every inline constraint to become an Entity.

The exact package locator syntax for inline context is deferred to package-schema consolidation. The Constraint schema SHALL NOT invent a universal graph path language merely to serialize context.

## 6. Cardinality is the first fully normalized family

Cardinality already has enough accepted semantics to define a complete normalized form.

### 6.1 Authoring cardinality

Accepted authoring surface:

```yaml
type: cardinality
relation: <relation-id>
qualifier?:
  target_type: <semantic-type-id>
min?: non-negative integer
max?: non-negative integer
exact?: non-negative integer
```

At least one bound SHALL be supplied for an authored Cardinality Constraint. `qualifier` is the ADR-0012 QRC extension.

The authoring schema may omit `max`; omission normalizes to unbounded.

### 6.2 Normalized source cardinality

Canonical normalized form:

```yaml
type: cardinality
relation: <canonical-relation-id>
direction: source
qualifier?:
  target_type: <canonical-semantic-type-id>
min: <non-negative integer>
max: <non-negative integer | unbounded>
provenance: <normalized provenance evidence>
```

`exact` SHALL NOT appear in normalized form. It is normalized by ADR-0012 interval intersection:

```text
L = max(min if present, exact if present), default 0
U = min(max if present, exact if present), default unbounded
```

Empty interval is:

```text
FAIL: QRC_EMPTY_INTERVAL
```

Invalid non-integer/negative bound is:

```text
FAIL: QRC_BOUND_INVALID
```

Current v0.1 normalized Cardinality schema covers outgoing/source cardinality, which is the form exercised by ADR-0012 and ADR-0015/16/17. Incoming/inverse-functional canonical syntax remains a later schema extension; generic Core incoming cardinality is currently unconstrained and does not require a stored Constraint.

## 7. QRC is not a seventh family

`qualifier.target_type` remains an optional field of Cardinality.

Therefore the existing `qrc-v0.1.schema.json` SHOULD be treated as a focused authoring schema/compatibility entry point and eventually reference or be generated from the common Cardinality authoring definition rather than defining a parallel semantic family.

QRC semantic evaluation still requires ADR-0012 rules that JSON Schema alone cannot prove:

- closed immutable relation snapshot;
- canonical qualifier resolution;
- canonical stable target identity;
- subtype closure;
- multiple typing consistency;
- qualified-set counting.

## 8. Other five families — structural boundary without invented payloads

ADR-0007 fixes these families:

```text
Type
Value
Dimension
Compatibility
Conditional
```

and fixes their composition behavior at semantic level. However the repository has not yet accepted one canonical field-level locator/payload syntax for all of them.

Therefore this proposal SHALL NOT make arbitrary fields such as a universal `path`, `subject`, or `target` normative merely to complete a JSON Schema union.

Instead, follow-up family-specific consolidation SHALL define only the minimum payload already justified by accepted studies:

- Type: semantic type/subtype restriction on an explicitly identified use-site;
- Value: normalized allowed set or numeric interval where representable;
- Dimension: canonical DimensionVector compatibility/equality requirement;
- Compatibility: conjunctive compatibility obligation with diagnostic discriminator where needed;
- Conditional: predicate plus one-or-more ordinary consequent Constraints.

Until those payloads are accepted, a unified `oneOf` schema SHALL NOT accept opaque arbitrary objects for these families because doing so would create false machine-readability.

## 9. Predicate schema boundary

ADR-0007 accepts predicate families:

```text
Compare
Membership
Exists
Boolean(and/or/not)
```

Conditional schema work SHOULD be performed only after a canonical value/property/relation locator representation is chosen. The Boolean tree structure can be canonicalized then without inventing a generic path language now.

No `else` branch is required by v0.1.

## 10. Composition boundary

JSON Schema validates structure only.

The semantic validator remains responsible for:

- conjunctive accumulation;
- normalization before composition;
- primitive-specific intersection;
- conditional activation before consequent composition;
- type/subtype closure;
- DimensionVector equality;
- explicit Schema/Configuration conflict diagnostics;
- separation from backend representability.

No declaration-order override is introduced.

## 11. Cardinality authority/projection integration

ADR-0017 canonical Core Cardinality Constraints SHALL normalize into the Cardinality NormalizedConstraint form.

A relation-side `source_cardinality` remains only a projection/cache. Core package validation SHALL compare it against the normalized canonical Cardinality Constraint and report:

```text
CARDINALITY_PROJECTION_MISSING
CARDINALITY_PROJECTION_MISMATCH
```

as already accepted by ADR-0017.

## 12. Minimal implementation artifacts after contract acceptance

After independent Validation accepts this architecture, the next implementation slice SHOULD be limited to:

1. `schema/constraint-cardinality-authoring-v0.1.schema.json`;
2. `schema/constraint-cardinality-normalized-v0.1.schema.json`;
3. update `schema/qrc-v0.1.schema.json` to reference/align with the authoring definition without changing ADR-0012 semantics;
4. small structural tests for:
   - valid QRC authoring;
   - mixed `min/max/exact` authoring;
   - normalized form rejects `exact`;
   - normalized form requires explicit `min/max/direction/provenance`;
   - `max: unbounded` allowed only in normalized form;
   - negative/non-integer authoring bounds rejected structurally where JSON Schema can decide;
   - semantic empty-interval case remains a validator-level failure rather than being falsely claimed as pure JSON Schema validation;
5. Core cardinality projection consistency test using ADR-0017 fixtures.

No backend runtime is required.

## 13. Counterexamples

### CS-01 — authoring and normalized forms treated as two constraints

Expected: invalid workflow. Authoring compiles to normalized evidence; it is not composed with its own normalized output.

### CS-02 — normalized QRC retains `exact`

Expected: structural failure. `exact` must normalize to the canonical interval.

### CS-03 — normalized cardinality omits `max`

Expected: structural failure. Canonical normalized form must explicitly state finite maximum or `unbounded`.

### CS-04 — arbitrary opaque Type/Value payload accepted by a premature union schema

Expected: rejected design. A schema must not claim deterministic machine readability before the family payload contract exists.

### CS-05 — JSON Schema claims closed-world QRC PASS/FAIL

Expected: invalid validator design. Closed-snapshot/type/identity evaluation belongs to semantic validation under ADR-0012.

## 14. Research verdict

Adopt explicit AuthoringConstraint → NormalizedConstraint separation and implement Cardinality/QRC as the first fully consolidated family. Defer the other family payload schemas to focused, evidence-based follow-up work rather than inventing a generic locator syntax.

**Ready for independent Validation.**
