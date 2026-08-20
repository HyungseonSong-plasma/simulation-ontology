# SOL v0.1 Conditional / Predicate Final Contract Review

**Status:** Final Validation  
**Date:** 2026-08-20  
**Targets:** Conditional/Predicate proposals v0.2 and v0.2.1  
**Normative basis:** ADR-0007, ADR-0018 through ADR-0022

## Verdict

**Accept**

The Conditional/Predicate contract is sufficiently deterministic for ADR drafting and design-stage schema implementation.

## Finding closure

| Finding | Verdict |
|---|---|
| COND-01 EvaluationReference binding cardinality | Resolved |
| COND-02 ABSENT Compare/Membership semantics | Resolved |
| COND-03 Membership scalar-kind ambiguity | Resolved |
| COND-04 PRESENT scalar-kind mismatch | Resolved |
| COND-05 final PASS determinism | Resolved |
| COND-06 Boolean failure masking | Resolved |
| COND-07 empty consequent ambiguity | Resolved |

No regression was found in previously accepted boundaries.

## Accepted contract summary

1. Conditional remains a meta-constraint over the five ordinary v0.1 Constraint families.
2. `then` contains `1..*` ordinary constraints; nested Conditional consequents are invalid.
3. No `else` field is required in v0.1.
4. Predicate vocabulary remains Compare, Membership, Exists, and Boolean (`and`, `or`, `not`).
5. Every normalized EvaluationReference key binds to exactly one evaluation binding; zero/multiple bindings are failures.
6. Lookup distinguishes `PRESENT`, `ABSENT`, and `UNRESOLVED`.
7. Compare/Membership on ABSENT or UNRESOLVED is `INDETERMINATE`; presence-sensitive logic uses Exists.
8. Exists tests presence, not truthiness.
9. Numeric predicates reuse ADR-0021 exact-decimal semantics and no scalar-kind coercion occurs.
10. Membership authoring carries explicit `scalar_kind`; normalized values have order-independent set semantics.
11. Predicate truth is strong-Kleene `TRUE | FALSE | INDETERMINATE`.
12. Predicate `EvaluationFailure` is not a truth value and dominates Boolean truth reduction; failures cannot be short-circuit masked.
13. Multiple Boolean-child failures preserve an order-independent diagnostic/evidence set.
14. TRUE activates all consequents; FALSE activates none; INDETERMINATE activates none and preserves incomplete activation.
15. Predicate evaluation failure makes overall validation FAIL.
16. Active consequent contradictions are Configuration Conflicts.
17. Overall design-stage validation precedence is `FAIL > INDETERMINATE > PASS`.
18. Backend installation/license/runtime state is outside Core predicate truth.

## Defect classification

No remaining Architecture, Profile, Adapter, Backend limitation, Reference-model, or Validation-tooling defect is identified at the contract level for this family.

## Next state

**Decision -> ADR drafting / predicate + Conditional schema smoke slice.**

Acceptance completes the contract design for all six minimum ADR-0007 Constraint families, subject to implementation/readback validation of this final slice.
