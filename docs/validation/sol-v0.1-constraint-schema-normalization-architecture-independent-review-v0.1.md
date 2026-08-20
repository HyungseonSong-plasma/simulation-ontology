# SOL v0.1 Constraint Schema Normalization Architecture — Independent Review v0.1

**Role:** Validation  
**Date:** 2026-08-20

## Inputs

- `docs/research/sol-v0.1-constraint-schema-normalization-architecture-proposal-v0.1.md`
- ADR-0007 Constraint architecture/composition
- ADR-0012 Qualified Relation Cardinality
- ADR-0017 Cardinality authority/projection contract
- existing `schema/qrc-v0.1.schema.json`
- current `ontology/core/constraints.yaml`

Research conclusions were not treated as authoritative.

## Evaluation contract

Determine whether the proposal can produce deterministic machine-readable Constraint evidence without inventing semantics beyond accepted ADRs or making serialization syntax the semantic authority.

Verdict domain: `Accept / Revise / Reject / Blocked`.

## Findings

### CS-V1 — AuthoringConstraint / NormalizedConstraint separation

**Verdict: Accept.**

Separating authoring shorthand from canonical normalized evidence resolves a real repository-state mismatch: QRC authoring permits `exact` and omitted bounds, while Core normalized cardinality evidence uses explicit `direction`, `min`, and `max: unbounded`.

The separation is a serialization/compiler boundary and does not add a new Core semantic primitive. It also preserves the authority chain:

```text
ADR semantics -> normalization -> normalized evidence -> structural tooling
```

### CS-V2 — Cardinality normalized interval form

**Verdict: Accept.**

Normalizing source/outgoing Cardinality to explicit:

```text
relation
direction = source
qualifier?
min
max = finite | unbounded
```

and eliminating `exact` from normalized form is consistent with ADR-0012 interval normalization and ADR-0017 Core cardinality evidence.

The proposal correctly leaves closed-snapshot/type/identity QRC evaluation to semantic validation rather than JSON Schema.

### CS-V3 — Authoring “at least one bound” is an unjustified strengthening

**Classification:** Language/schema contract regression.  
**Verdict: Revise.**

ADR-0012 defines `min?`, `max?`, and `exact?` as optional and does not require at least one authoring bound. An authored cardinality with no explicit bound deterministically normalizes to the unconstrained interval `[0,unbounded]` under the accepted default rules.

The proposal newly states:

```text
At least one bound SHALL be supplied
```

which would reject an input not rejected by the accepted QRC contract.

Required correction: preserve the accepted optional-bound syntax. A boundless authored Cardinality is semantically redundant but valid and normalizes to `[0,unbounded]`. If a future style/lint rule wants to reject no-op constraints, that is non-semantic linting and must not be structural validity in v0.1.

### CS-V4 — Provenance preservation versus inline `provenance` field is ambiguous

**Classification:** Serialization-contract ambiguity.  
**Verdict: Revise.**

ADR-0007 requires enough provenance to identify contributing sources during conflict diagnostics. It does not require that every inline Constraint object contain a literal field named `provenance`.

The proposal both says container context may supply provenance/identity and shows `provenance` as a mandatory field of normalized Cardinality. Independent implementations could therefore disagree whether:

```yaml
type: cardinality
canonical_id: core.cardinality.solved_by.source
relation: solved_by
direction: source
min: 0
max: unbounded
```

is a complete normalized Constraint when its containing package/key already supplies stable contributor context.

Required correction: distinguish:

1. canonical **constraint payload**, and
2. normalized **evidence record/envelope** carrying provenance/context.

Provenance SHALL be available in the normalized evidence record, but this review does not require an inline payload field until the package/evidence-envelope serialization contract is accepted. Payload equality/intersection must not depend on incidental provenance placement.

### CS-V5 — Deferral of unfinished family payloads

**Verdict: Accept.**

It is correct not to publish a permissive `oneOf` accepting opaque Type/Value/Dimension/Compatibility objects merely to claim six-family schema completeness. Accepted family semantics should be transcribed through focused payload contracts rather than a generic invented path language.

### CS-V6 — QRC remains Cardinality family

**Verdict: Accept.**

The proposal correctly keeps QRC as Cardinality + optional target-type qualifier and avoids a seventh primitive family.

## Final verdict

| Dimension | Verdict |
|---|---|
| Authoring/normalized separation | Accept |
| Cardinality normalized interval | Accept |
| QRC family placement | Accept |
| Semantic-vs-JSON-Schema boundary | Accept |
| Boundless authoring compatibility | Revise |
| Provenance carrier contract | Revise |
| Other family deferral | Accept |
| Architecture redesign required | No |

**Overall: Revise.**

## Next state

Research SHALL revise only CS-V3 and CS-V4. Preserve the two-layer architecture, Cardinality normalized interval, QRC semantics, and deferral of unfinished family payloads. Then perform a focused final contract review before implementing schemas.
