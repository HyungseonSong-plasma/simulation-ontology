# SOL v0.1 Cross-Family Validation-State Aggregation Focused Review

**Status:** Final focused Validation  
**Date:** 2026-08-20  
**Target:** `docs/research/sol-v0.1-cross-family-validation-state-aggregation-remediation-v0.1.md`  
**Normative basis:** ADR-0007, ADR-0022, ADR-0023

## Verdict

**Accept**

The focused rule closes COND-08 without changing the accepted Predicate, Conditional, or Compatibility semantics.

## Accepted integration rule

For the common design-stage evaluation state slice:

```text
PASS | FAIL | INDETERMINATE
```

aggregate conjunctively as:

```text
FAIL > INDETERMINATE > PASS
```

Inputs include static and active ordinary-family states, Conditional activation states, and predicate/reference EvaluationFailure projected to FAIL with diagnostics.

Therefore:

```text
ordinary Compatibility INDETERMINATE + otherwise PASS -> INDETERMINATE
ordinary FAIL + INDETERMINATE                       -> FAIL
Conditional activation INDETERMINATE + ordinary PASS -> INDETERMINATE
predicate EvaluationFailure + INDETERMINATE         -> FAIL
empty three-state input                              -> PASS
```

## Scope boundary

This rule does not collapse separate invocation/precondition states such as `BLOCKED` into the three-state slice. Their integration remains a later canonical validator-envelope concern.

## Defect classification

COND-08: **Resolved contract-integration defect**.

No Architecture/Profile/Adapter/Backend limitation/Reference-model defect is introduced by the clarification.

## Next state

**Decision -> accepted ADR supplement/clarification -> resume ADR-0023 semantic-helper implementation.**
