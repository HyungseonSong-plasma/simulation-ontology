# SOL v0.1 ADR-0011 / ADR-0012 — focused readback validation

**Status:** Independent Validation role readback  
**Date:** 2026-08-20

## ADR-0011

The draft now normatively includes comparator resolution context comprising comparison scope, component binding(s), comparison purpose, and normalized operand kinds; zero/multiple matches fail and no order/version heuristic may resolve ambiguity. It also restores immutable evaluation revisions, terminal transition rules, new-revision handling for resolved evidence, and runtime-drift invalidation.

**Verdict: Accept as faithful contract draft.**

## ADR-0012

The draft now restores non-negative integer bound validation with canonical `FAIL: QRC_BOUND_INVALID` and restores SHALL-level failure diagnostic evidence. Closed-snapshot evaluation, qualifier resolution, stable identity, multiple typing, interval intersection, subtype algebra, and deferred universal quantifiers remain consistent with the accepted QRC contract.

**Verdict: Accept as faithful contract draft.**

## Gate

- ADR-0011: contract-draft validation PASS.
- ADR-0012: contract-draft validation PASS.
- No architecture regression identified in readback scope.
- These ADRs remain `Proposed` until the Operating Desk decides the promotion gate.
- Architecture freeze remains blocked because executable validation evidence has not yet been produced.

The next workflow state is executable-validation preparation: determine repository implementation baseline and implement the minimum machine-readable schema/validator fixtures required by the accepted contract before freeze revalidation.
