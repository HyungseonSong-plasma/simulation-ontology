# SOL v0.1 Type Constraint Schema — Independent Review v0.1

**Role:** Validation  
**Date:** 2026-08-20

## Inputs

- `docs/research/sol-v0.1-type-constraint-schema-proposal-v0.1.md`
- ADR-0007 Constraint composition
- ADR-0008 inheritance/Interface separation
- ADR-0016 subtype-aware relation endpoint semantics
- ADR-0018 authoring/normalized Constraint boundary
- accepted relation-semantics and constraint-intersection studies

## Evaluation contract

Determine whether `type + relation + target_type` is sufficient for deterministic relation-target Type Constraint authoring/normalization/composition without conflating taxonomic type, Interface capability, or backend/Profile representability.

## Findings

### TC-V1 — minimal relation-target payload

**Verdict: Accept.**

A context-local payload:

```text
type = type
relation
target_type
```

is sufficient for the currently evidenced v0.1 Type use-case. The containing Entity/Interface/local declaration can supply the constrained source/use-site context through the ADR-0018 evidence layer, avoiding an invented universal path language.

Authoring and normalized forms may share structural fields while normalization canonicalizes relation/type identities.

### TC-V2 — deterministic subtype intersection

**Verdict: Accept.**

The proposed intersection:

```text
A = B -> A
A <: B -> A
B <: A -> B
unrelated -> conflict
```

is consistent with v0.1 single taxonomic inheritance. Interface implementation remains a separate axis and does not create synthetic multiple taxonomic inheritance.

### TC-V3 — Profile contribution is semantically ambiguous

**Classification:** Evaluation-dimension mixing.  
**Verdict: Revise.**

The proposal lists Profile/local refinement together as contributors to Type constraints and later describes Profile widening/narrowing using the same effective Type intersection. Accepted Constraint composition work qualifies Profile behavior: a backend Profile may narrow **representability/applicability**, but must not mutate upstream semantic truth or turn a semantically valid SOL model into an invalid semantic model merely to match backend capability.

Counterexample:

```text
Domain semantic Type: target Field
Valid model target: ElectricField <: Field
Backend Profile only represents TemperatureField
```

If the Profile Type restriction is intersected as upstream semantic truth, the valid ElectricField model becomes schema-invalid instead of semantically valid + backend unsupported.

Required correction: distinguish semantic Type contributors:

```text
inherited Entity type
Interface semantic contract
local/domain semantic constraint
active semantic Conditional
```

from Profile/backend representability Type restrictions. The latter may reuse the same payload shape/normalizer but SHALL be evaluated in the representability/applicability contract and must not rewrite the upstream semantic effective Type constraint.

### TC-V4 — union / allowed-family compatibility rule needs deterministic wording

**Classification:** Contract ambiguity.  
**Verdict: Revise.**

The proposal defines compatibility for simple range `R` and mentions ADR-0016 allowed-pair matrices, but does not state one canonical rule for a relation whose valid target contract is a finite allowed family such as:

```text
applied_to -> Field | Equation | Scope
```

Required correction: for a target contract represented as canonical allowed types `{R1...Rn}`, Type Constraint target `T` is a valid narrowing iff:

```text
exists Ri: T = Ri OR T <: Ri
```

For ADR-0016 `includes_component`, the allowed target family must first be selected from the authoritative allowed pair(s) compatible with the contributing source context; then the same existential narrowing rule applies. A Type Constraint never adds a new base allowed pair.

### TC-V5 — QRC orthogonality

**Verdict: Accept.**

The proposal correctly distinguishes QRC subset qualification from universal relation-target Type restriction. A QRC qualifier does not imply every relation target must be of that type.

### TC-V6 — Interface/type orthogonality

**Verdict: Accept.**

Rejecting an Interface identifier as `target_type` in this Type slice is consistent with ADR-0008. Interface capability requirements remain separate from Entity taxonomic Type constraints.

## Final verdict

| Dimension | Verdict |
|---|---|
| Minimal payload | Accept |
| Authoring/normalized identity boundary | Accept |
| Type intersection | Accept |
| Interface orthogonality | Accept |
| QRC orthogonality | Accept |
| Profile semantic/representability separation | Revise |
| Union/allowed-pair compatibility | Revise |
| Architecture redesign required | No |

**Overall: Revise.**

## Next state

Research SHALL revise only TC-V3 and TC-V4. Preserve the three-field payload, subtype intersection, QRC/Interface orthogonality, and container-owned context. Then perform focused final Validation before schema implementation.
