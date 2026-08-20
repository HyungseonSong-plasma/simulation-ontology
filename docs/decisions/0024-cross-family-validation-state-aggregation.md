# ADR-0024: Cross-Family Validation-State Aggregation

**Status:** Accepted  
**Date:** 2026-08-20  
**Supplements:** ADR-0007, ADR-0022, ADR-0023

## Context

ADR-0022 permits an ordinary Compatibility Constraint family to return `INDETERMINATE` when a required semantic evaluation context is unresolved. ADR-0023 defines Conditional activation `INDETERMINATE` and final validation precedence, but its initial text did not explicitly propagate an ordinary-family `INDETERMINATE` into the overall result.

Implementation preflight exposed this cross-family integration gap before the ADR-0023 semantic helper was completed.

## Decision

### 1. Common three-state evaluation slice

For design-stage validation results represented in the common state slice:

```text
PASS | FAIL | INDETERMINATE
```

SOL aggregates conjunctively using:

```text
FAIL > INDETERMINATE > PASS
```

### 2. Inputs

This aggregation includes:

- static ordinary-family validation states;
- ordinary-family validation states activated by TRUE Conditional predicates;
- Conditional activation states;
- predicate/reference EvaluationFailure projected to `FAIL` while preserving diagnostics/evidence.

### 3. Normative aggregation

```text
if any predicate/reference EvaluationFailure
   or any ordinary-family state == FAIL:
    overall = FAIL
else if any ordinary-family state == INDETERMINATE
     or any Conditional activation == INDETERMINATE:
    overall = INDETERMINATE
else:
    overall = PASS
```

The empty input set has identity `PASS` for this three-state slice.

### 4. Conflict classification

A Schema Conflict or Configuration Conflict is a definite unsatisfiable semantic result and therefore contributes `FAIL` in this slice.

`INDETERMINATE` remains semantic evaluation incompleteness, not a conflict.

### 5. Other operational/invocation states remain separate

This ADR does not define a universal validator-state lattice.

A family or invocation contract that exposes another state such as `BLOCKED` retains that accepted meaning. Such a state SHALL NOT be silently coerced into `PASS`, `FAIL`, or `INDETERMINATE` by this ADR.

Integration of additional state axes belongs to later canonical validator-envelope work.

## Boundary cases

```text
ordinary [INDETERMINATE], otherwise PASS -> INDETERMINATE
ordinary [FAIL, INDETERMINATE]           -> FAIL
ordinary [PASS] + activation U           -> INDETERMINATE
predicate EvaluationFailure + ordinary U -> FAIL
empty three-state input                   -> PASS
```

## Consequences

- static Compatibility incompleteness cannot be lost when Conditional logic is also present;
- ADR-0022 and ADR-0023 share one deterministic three-state aggregation rule;
- separate invocation/precondition states are not prematurely collapsed into a global state lattice.

## Validation evidence

- `docs/validation/sol-v0.1-adr0023-cross-family-indeterminate-preflight-review.md`
- `docs/research/sol-v0.1-cross-family-validation-state-aggregation-remediation-v0.1.md`
- `docs/validation/sol-v0.1-cross-family-validation-state-aggregation-focused-review.md`

## Decision summary

Within the common `PASS | FAIL | INDETERMINATE` design-stage evaluation slice, SOL uses conjunctive cross-family aggregation with strict precedence `FAIL > INDETERMINATE > PASS`, while leaving distinct invocation/precondition states for later validator-envelope integration.
