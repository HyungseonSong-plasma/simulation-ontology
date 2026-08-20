# SOL v0.1 Compatibility Constraint v0.2 Focused Re-Review

**Status:** Validation finding  
**Date:** 2026-08-20  
**Target:** `docs/research/sol-v0.1-compatibility-constraint-schema-proposal-v0.2.md`

## Verdict

**Revise — one residual aggregation rule only.**

COMP-01 through COMP-04 are resolved without regression:

- normalized operands preserve `schema | model_instance` identity space;
- symmetric pair identity is an unordered two-member multiset and does not depend on lexical sorting;
- unresolved semantic evaluation prerequisites produce explicit `INDETERMINATE` rather than guessed compatibility;
- generic reducibility detection is correctly moved from mandatory validator behavior to design guidance.

## COMP-05 — Mixed FAIL and INDETERMINATE aggregation is unspecified

**Classification:** Contract determinism defect  
**Severity:** Required narrow revision

The proposal defines per-obligation states but not the plan/set-level verdict when multiple active Compatibility obligations have mixed results.

Counterexample:

```text
Obligation A -> completed binary evaluation -> FAIL
Obligation B -> semantic prerequisite unresolved -> INDETERMINATE
```

Validator A may report overall `FAIL` because conjunction is already false. Validator B may report overall `INDETERMINATE` because not every obligation completed. Both interpretations fit the current text.

For conjunctive semantics, the minimum deterministic aggregation should be explicit. A suitable v0.1 rule is:

```text
FAIL > INDETERMINATE > PASS
```

Meaning:

- any definite failed obligation makes the Compatibility set `FAIL`;
- otherwise, if at least one obligation is `INDETERMINATE`, the set is `INDETERMINATE`;
- otherwise all obligations pass and the set is `PASS`.

Static normalization/composition errors and explicit contradictory expectations remain errors/conflicts before this evaluation aggregation step.

## Required revision scope

Add only:

1. deterministic Compatibility-set aggregation precedence;
2. boundary cases for `FAIL + INDETERMINATE`, `PASS + INDETERMINATE`, and all-PASS.

Do not reopen criterion identity, operand representation, symmetric key semantics, primitive separation, or representability boundaries.

## Final verdict

**Revise** — limited COMP-05 closure only.
