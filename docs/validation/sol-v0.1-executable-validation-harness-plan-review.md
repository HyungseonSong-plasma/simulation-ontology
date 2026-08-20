# SOL v0.1 Executable Validation Harness Plan — independent review

**Status:** Validation role  
**Date:** 2026-08-20

## Verdict

**Revise.** The phase structure is sound, but Phase 1 is narrower than the executable-validation scope established by the accepted contract reviews.

## Finding EVH-01 — missing contract dimensions in Phase 1

The planned reference validator covers decision aggregation, QRC, comparator uniqueness/context, effect-pair enumeration, DAG checks, and target resolution, but does not explicitly require executable-descriptor/effect equivalence, adapter-authorized bookkeeping, state-dependent idempotency, immutable evaluation revisions, representability aggregation, or loss-policy/execution-permission evaluation.

These are part of the accepted ADR-0011 contract and were explicit execution-validation requirements in prior Validation reports. Deferring all of them to backend execution would permit Phase 1 to PASS while substantial deterministic contract logic remains unimplemented.

**Classification:** Validation-tooling scope defect, not Architecture defect.

## Required correction

Add Phase-1 machine/reference tests for:

- executable descriptor versus declared effect coverage using a deterministic synthetic adapter procedure;
- adapter-only bookkeeping classification;
- state-independent guarantee and state-dependent idempotency result boundaries;
- immutable lifecycle revision transitions;
- representability aggregation precondition (`validation PASS + complete`);
- canonical loss-key and conjunctive execution policy behavior.

Backend-specific truth still belongs to later Thermal/Plasma execution; synthetic Phase-1 procedures only validate the contract mechanics.

After this addition the plan is implementation-ready.
