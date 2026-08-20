# SOL v0.1 Plasma/QRC Implementation Independent Review v0.1

**Role:** Validation  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`  
**Input artifacts:** committed plasma reference package, closed model snapshot, `tests/qrc_snapshot_semantics.py`, and `tests/test_qrc_snapshot_semantics.py`  
**Normative basis:** ADR-0012, ADR-0025, ADR-0029

## Verdict

**Revise**

The Plasma/QRC reference topology and the integration path are sound, but one validator-tooling defect prevents final acceptance.

## Confirmed

- The plasma fixture uses a closed snapshot with exact package environment.
- `DissociativeAttachmentReaction` implements an Interface whose relation requirement maps explicitly to canonical `products`.
- Two QRC ConstraintDefinitions independently require exactly one `NegativeIonSpecies` product and exactly one `NeutralSpecies` product.
- QRC counting uses distinct target model-instance identity and canonical subtype closure.
- Unknown qualifier target types fail as unresolved rather than being treated as a zero count.
- Duplicate relation triples fail at the snapshot precondition before QRC counting.
- Declaration/package order does not affect the expected QRC counts.
- Backend runtime/license state is not an input to QRC validation.

## Finding PQIV-01 — empty interval diagnostic is count-order dependent

`tests/qrc_snapshot_semantics.py::_contains()` evaluates `count < minimum` before checking whether a finite `minimum > maximum` interval is intrinsically empty.

Counterexample:

```text
min = 2
max = 1
count = 0
```

Current behavior can return ordinary unsatisfied cardinality and surface `QRC_CARDINALITY_VIOLATION`.

ADR-0012 requires malformed/empty QRC bounds to be rejected independently of the current target count. The same malformed constraint must therefore deterministically produce:

```text
QRC_EMPTY_INTERVAL
```

before count satisfaction is evaluated.

**Classification:** Validation-tooling defect.  
**Architecture impact:** none.  
**Required remediation:** validate the finite interval itself before comparing `count`; add a regression test whose count is below `min` so the prior ordering bug is observable.

## Next state

Return only PQIV-01 to implementation, then perform focused Validation. Do not reopen Plasma/QRC topology, ADR-0012 semantics, Interface mapping, or ADR-0029 snapshot design.
