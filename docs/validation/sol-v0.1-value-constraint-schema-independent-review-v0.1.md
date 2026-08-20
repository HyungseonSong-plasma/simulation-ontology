# SOL v0.1 Value Constraint Schema Independent Review v0.1

**Role:** Validation  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`

## Inputs

- `docs/research/sol-v0.1-value-constraint-schema-proposal-v0.1.md`
- ADR-0002, ADR-0007, ADR-0018, ADR-0020
- official MOOSE/COMSOL/Ansys range/allowed-value evidence

Research verdict was not treated as authority.

## Overall assessment

The proposed **numeric interval + finite scalar allowed set** decomposition is supported by the reference backends and preserves the accepted Value/ValueDefinition/Conditional boundaries. However, three contract-level ambiguities can still produce different results from independent validators.

## Findings

### VAL-V1 — Numeric equality/canonicalization is not implementation-independent

**Verdict: Unresolved.**

The proposal states mathematical JSON-number equality such as `1 == 1.0`, but it does not define a normalized numeric representation precise enough to survive different host-language parsers.

Counterexample:

```text
9007199254740992
9007199254740993
```

A binary64/JavaScript-oriented parser may lose their distinction if numeric input is materialized too early, while an arbitrary-precision/integer-aware implementation can preserve it. Similar issues arise for decimal literals, exponent syntax, signed zero, and exact set deduplication/order filtering.

This affects:

- allowed-set duplicate elimination;
- numeric set equality;
- interval ordering;
- interval/set filtering;
- boundary equality.

Required remediation:

Define one canonical **normalized numeric scalar** contract independent of host-language numeric type. Acceptable directions include an exact decimal coefficient/exponent representation or another precisely specified canonical numeric domain. Do not rely on Python/JavaScript native numeric identity as the semantic authority.

Authoring lexical numbers may remain ergonomic, but a conforming normalizer must preserve enough source numeric information to construct the canonical scalar deterministically.

### VAL-V2 — Empty Value payload conflict class is over-specified

**Verdict: Unresolved.**

The proposal says an empty allowed set SHALL produce a Schema Conflict. That is not always correct under ADR-0007 Conditional activation.

Counterexample:

```text
P -> allowed_set {}
```

If `P` is false, no active unsatisfiable Value constraint exists. If `P` is true, the effective configuration is unsatisfiable. This is a Configuration Conflict rather than an unconditional Schema Conflict.

The same issue applies to an intrinsically empty interval used as a Conditional consequent.

Required remediation:

Separate:

```text
Value normalization result = satisfiable | empty
```

from:

```text
conflict classification = Schema | Configuration
```

Classification must use contributor/activation context under ADR-0007, not the Value payload alone.

### VAL-V3 — Numeric comparison-space resolution lacks a deterministic gate

**Verdict: Unresolved.**

The proposal correctly says raw magnitudes from unresolved Unit contexts must not be compared, but does not define whether such input is:

- structurally invalid;
- semantically invalid;
- temporarily non-normalizable;
- valid but unevaluable.

Independent compilers could therefore diverge.

Required remediation:

Define an explicit normalization precondition:

```text
Numeric Value Constraint -> normalized payload
ONLY IF
its scalar operands are already in one resolved semantic comparison space.
```

If comparison-space resolution is required but unavailable, no normalized numeric Value payload may be emitted. Use one deterministic diagnostic such as:

```text
VALUE_COMPARISON_SPACE_UNRESOLVED
```

The diagnostic is a compiler/normalization boundary, not a backend representability failure and not evidence that the upstream Dimension contract is invalid.

This focused Value slice need not define Unit conversion itself.

## Accepted parts not to reopen

The following survive independent review:

- Value Constraint applies to evaluated scalar data, not ValueDefinition mechanism.
- Focused v0.1 forms are numeric interval and finite allowed set.
- scalar literal kinds are number/string/boolean and are not Entity taxonomic Types.
- interval supports open/closed/unbounded sides.
- string/boolean exact equality is appropriate for this slice.
- ordinary Value payloads contain no activation predicate; Conditional remains orthogonal.
- interval∩interval, set∩set, and numeric interval∩numeric set must be deterministic.
- vector/tensor/complex/regex/arbitrary expression constraints may remain deferred.
- Unit/metrology registry syntax does not belong in this Value payload.

## Defect classification

- Architecture defect: none requiring new primitive families.
- Contract defect: VAL-V1, VAL-V2, VAL-V3.
- Backend limitation: none; backend runtime is not required.
- Validation-tooling defect: none yet because implementation has not started.

## Verdict

**Revise.**

Revise only the three findings above. Do not reopen the accepted two-form Value architecture or the Value/ValueDefinition/Conditional boundaries.

After revision, perform a focused final contract review before drafting an ADR or schemas.
