# SOL v0.1 ADR-0022 Compatibility Schema Implementation Review v0.1

**Status:** Validation finding  
**Date:** 2026-08-20  
**Inputs:** ADR-0022, Compatibility authoring/normalized schemas, semantic helper/test, smoke evidence

## Verdict

**Revise — Validation-tooling defect only.**

The structural schemas and obligation identity/composition/aggregation helper match the accepted contract. One diagnostic requirement is missing from the helper output.

## CIV-01 — INDETERMINATE diagnostic code is dropped

ADR-0022 requires unresolved semantic evaluation context to produce:

```text
state: INDETERMINATE
code: COMPATIBILITY_EVALUATION_CONTEXT_UNRESOLVED
```

The current `evaluate_obligation()` returns only:

```text
"INDETERMINATE"
```

Therefore two consumers cannot distinguish the accepted unresolved-context state from another future source of indeterminacy using the required diagnostic contract.

**Classification:** Validation-tooling defect  
**Architecture impact:** none

## Required revision

Update the helper/test so per-obligation evaluation preserves at least:

```text
state
optional/required diagnostic code when INDETERMINATE
```

Binary PASS/FAIL may omit the code. Family aggregation may continue to aggregate the `state` values only.

Add a regression test asserting the exact code.

Do not modify ADR-0022 or the structural schemas unless the focused fix reveals a genuine schema mismatch.

## Accepted implementation boundaries

- backend identity space is structurally rejected;
- schema/model-instance references remain distinct;
- symmetric multiset identity is correct;
- ordered identity is directional;
- duplicate/opposite expectation behavior is correct;
- empty-set `PASS` and `FAIL > INDETERMINATE > PASS` aggregation are correct;
- backend runtime/license metadata remains outside semantic input.

## Final verdict

**Revise** — CIV-01 only.
