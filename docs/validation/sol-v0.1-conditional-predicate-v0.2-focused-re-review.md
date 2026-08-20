# SOL v0.1 Conditional / Predicate v0.2 Focused Re-Review

**Status:** Validation finding  
**Date:** 2026-08-20  
**Target:** `docs/research/sol-v0.1-conditional-predicate-schema-proposal-v0.2.md`  
**Normative basis:** ADR-0007, ADR-0018 through ADR-0022

## Verdict

**Revise — two narrow determinism issues.**

COND-01 through COND-05 are resolved without regression. The revised ABSENT semantics, explicit Membership scalar kind, scalar-kind mismatch failure, exact-one EvaluationReference binding, and mandatory final PASS branch are mutually consistent.

## Resolved findings

| Finding | Verdict |
|---|---|
| COND-01 EvaluationReference zero/one/many binding | Resolved |
| COND-02 ABSENT Compare/Membership logic | Resolved |
| COND-03 empty Membership scalar kind | Resolved |
| COND-04 PRESENT scalar-kind mismatch | Resolved |
| COND-05 final PASS determinism | Resolved |
| Membership array order clarification | Resolved |

## COND-06 — Boolean decisive truth versus predicate evaluation failure is unspecified

**Classification:** Predicate evaluation contract defect  
**Severity:** Required revision

The proposal defines strong-Kleene truth reduction and separately states that a predicate evaluation failure causes validation `FAIL`, but it does not define whether evaluation failure dominates a Boolean result that could otherwise be decided by short-circuit logic.

Counterexample:

```text
AND(
  Compare(a, eq, 0) -> FALSE,
  Compare(b, eq, 1) -> PREDICATE_SCALAR_KIND_MISMATCH
)
```

Validator A eagerly evaluates both operands and reports predicate evaluation failure / overall FAIL.
Validator B short-circuits after the first FALSE and returns predicate FALSE, so the Conditional is merely inactive.

Both interpretations fit the current text.

The contract must distinguish **truth values** from **evaluation failures**. A minimal deterministic rule is:

```text
predicate evaluation failure > Boolean truth reduction
```

All Boolean operands SHALL be validated/evaluated sufficiently to detect evaluation failures before a Boolean truth value is finalized. If any operand has an evaluation failure, the Boolean predicate itself has an evaluation failure and no TRUE/FALSE/INDETERMINATE truth value is produced.

This rule applies analogously to OR and NOT. It does not change strong-Kleene reduction when every operand successfully produces a truth value.

## COND-07 — Empty `then` consequent list is unspecified

**Classification:** Serialization/contract completeness defect  
**Severity:** Required revision

The proposal says `then` may contain only the five ordinary Constraint families but does not state whether:

```yaml
type: conditional
if: <Predicate>
then: []
```

is valid.

A schema author could reasonably choose either `minItems: 0` or `minItems: 1`, producing different accepted languages.

For the focused v0.1 contract, require:

```text
then contains 1..* ordinary constraints
```

An empty Conditional has no semantic effect and does not justify a separate valid construct. This rule also gives the future structural schema a unique outcome.

## No-regression checks

The following remain accepted and SHALL NOT be reopened in the revision:

- Conditional is a meta-constraint;
- only five ordinary families are allowed in `then`;
- nested Conditional is forbidden;
- no `else` field;
- Compare/Membership/Exists/Boolean predicate vocabulary;
- exact-decimal numeric semantics;
- exact-one EvaluationReference binding;
- PRESENT/ABSENT/UNRESOLVED lookup distinction;
- Compare/Membership ABSENT -> INDETERMINATE;
- Exists ABSENT -> FALSE;
- scalar-kind mismatch -> evaluation failure;
- strong-Kleene truth tables for successful operands;
- TRUE/FALSE/INDETERMINATE activation semantics;
- active consequent contradiction -> Configuration Conflict;
- final validation precedence `FAIL > INDETERMINATE > PASS`;
- backend runtime/license independence.

## Required revision scope

Research should change only:

1. define evaluation failure precedence over Boolean truth reduction and prohibit short-circuit masking of failures;
2. define `then` cardinality as `1..*` ordinary constraints.

No new predicate family, path language, nested Conditional, or backend-specific mechanism is required.

## Final verdict

**Revise** — COND-06 and COND-07 only.
