# SOL v0.1 ADR-0023 Cross-Family INDETERMINATE Preflight Review

**Status:** Validation finding  
**Date:** 2026-08-20  
**Trigger:** ADR-0023 semantic-helper implementation preflight  
**Normative basis:** ADR-0007, ADR-0022, ADR-0023

## Verdict

**Revise — narrow cross-family aggregation clarification required before semantic-helper completion.**

The Predicate/Conditional truth and activation contract remains accepted. One inconsistency exists between ADR-0022 and ADR-0023 final validation aggregation.

## COND-08 — Ordinary-family INDETERMINATE can be lost

ADR-0022 defines a valid Compatibility-family result:

```text
INDETERMINATE
```

when a criterion-required semantic evaluation context is unresolved and no definite Compatibility failure exists.

ADR-0023 currently states final validation as:

```text
if any reference/predicate evaluation failure or definite ordinary-constraint failure:
    FAIL
else if any Conditional activation is INDETERMINATE:
    INDETERMINATE
else:
    PASS
```

Counterexample:

```text
static Compatibility obligation -> INDETERMINATE
all predicates                  -> determinate
all other ordinary constraints  -> PASS
```

The literal ADR-0023 rule can produce `PASS`, which contradicts the accepted incomplete semantic evaluation from ADR-0022.

## Required clarification

Within the `PASS | FAIL | INDETERMINATE` evaluation slice, final validation SHALL aggregate both ordinary-family and Conditional activation states:

```text
FAIL > INDETERMINATE > PASS
```

Normatively:

```text
if any predicate/reference evaluation failure
   or any ordinary-family state == FAIL:
    overall = FAIL
else if any ordinary-family state == INDETERMINATE
     or any Conditional activation == INDETERMINATE:
    overall = INDETERMINATE
else:
    overall = PASS
```

This review does not standardize unrelated family states such as invocation/precondition `BLOCKED`; those remain governed by their own accepted contracts and later canonical validator-envelope integration.

## Scope preservation

Do not reopen:

- Predicate vocabulary;
- PRESENT/ABSENT/UNRESOLVED leaf semantics;
- strong-Kleene Boolean logic;
- EvaluationFailure precedence;
- `then` cardinality;
- active consequent Configuration Conflict semantics;
- Compatibility family semantics.

## Defect classification

**Contract integration defect** between two accepted design-stage slices. No backend, Profile, Adapter, runtime, or license issue is involved.

## Next state

Focused Research clarification -> focused Validation -> ADR supplement/amendment -> resume semantic-helper implementation.
