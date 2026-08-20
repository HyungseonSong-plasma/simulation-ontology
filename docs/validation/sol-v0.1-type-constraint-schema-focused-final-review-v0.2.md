# SOL v0.1 Type Constraint Schema — Focused Final Review v0.2

**Role:** Validation  
**Date:** 2026-08-20

## Scope

Re-review only TC-V3 and TC-V4 from the prior independent review against `sol-v0.1-type-constraint-schema-proposal-v0.2.md`. The three-field payload, subtype intersection, Interface orthogonality, QRC orthogonality, and container-owned context were not reopened.

## TC-V3 — semantic versus representability evaluation axis

**Verdict: Resolved.**

The revised proposal now keeps upstream semantic Type contributors separate from Backend/Profile representability/applicability restrictions. A Profile restriction may reuse the same Type payload shape but is not inserted into the semantic Type intersection.

A semantically valid model that violates only a selected backend/profile Type restriction therefore remains semantically valid and is handled by representability/applicability diagnostics rather than schema invalidity.

The evaluation axis is evidence/context metadata and does not contaminate the three-field semantic payload.

## TC-V4 — finite target family and allowed-pair narrowing

**Verdict: Resolved.**

For a canonical target family `{R1...Rn}`, Type target `T` is valid iff at least one allowed family member satisfies `T = Ri` or `T <: Ri`.

For ADR-0016 `includes_component`, the validator first resolves the applicable authoritative source allowed pair(s), derives the allowed target family, then applies the same subtype narrowing. The Type Constraint cannot add a new allowed pair.

This produces deterministic results for union-family and source-dependent allowed-pair relations.

## Regression check

No regression found in:

- payload `type + relation + target_type`;
- authoring/normalized identity distinction;
- subtype intersection order independence;
- no implicit widening override;
- Interface identity not treated as Entity target type;
- QRC qualifier not treated as universal relation target restriction;
- semantic/Profile evaluation-dimension separation from accepted constraint composition rules.

## Final verdict

| Dimension | Verdict |
|---|---|
| TC-V3 | Resolved |
| TC-V4 | Resolved |
| Regression | None |
| Architecture defect remaining | None |
| Schema implementation readiness | Yes, relation-target Type slice |

**Overall: Accept.**

## Next state

Operating Desk may promote the accepted Type Constraint slice to a focused ADR and implement authoring/normalized schemas plus a minimal semantic intersection/compatibility smoke harness. No backend runtime is required.
