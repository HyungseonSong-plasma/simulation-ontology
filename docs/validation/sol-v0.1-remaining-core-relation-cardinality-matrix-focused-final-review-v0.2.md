# SOL v0.1 Remaining Core Relation Cardinality Matrix — Focused Final Review v0.2

**Role:** Validation  
**Date:** 2026-08-20

## Scope

Re-review only RC-V2 and RC-V3 against:

- `docs/research/sol-v0.1-remaining-core-relation-cardinality-matrix-proposal-v0.2.md`
- ADR-0007 Constraint authority/composition
- prior independent review v0.1.

The seven-relation `[0,∞]` semantic matrix and ADR-0015/0016 fixed cardinalities were not reopened.

## RC-V2 — explicit versus omitted cardinality

**Verdict: Resolved.**

The revised contract now requires an explicit Core relation projection for all seven frozen relations:

```text
source_cardinality = [0,∞]
```

Omission is deterministically `FAIL: CARDINALITY_PROJECTION_MISSING` in frozen Core transcription validation. It is no longer legal for one validator to interpret omission as unconstrained while another interprets it as unknown.

## RC-V3 — Constraint authority versus relation projection

**Verdict: Resolved.**

Cardinality remains normatively a Constraint under ADR-0007. Relation-side `source_cardinality` is explicitly a projection/cache only and does not participate as a second Constraint contributor.

The contract defines:

```text
projection == normalized canonical Core constraint  -> PASS
projection missing                                  -> FAIL
projection mismatch                                 -> FAIL
```

Profile/domain/interface refinements compose with the canonical Core Constraint and do not rewrite the Core projection.

Duplicate canonical Core authorities are also rejected rather than resolved by declaration order.

## Regression check

No regression found in:

- semantic matrix `represented_by/closed_by/parameterized_by/defined_on/discretized_by/solved_by/observed_by = 0..*`;
- incoming cardinality unconstrained by Core;
- conjunctive profile/domain/interface refinement;
- ADR-0015/0016 cardinalities;
- design-stage separation from backend execution requirements.

## Final verdict

| Dimension | Verdict |
|---|---|
| RC-V2 | Resolved |
| RC-V3 | Resolved |
| Regression | None |
| Architecture defect remaining | None |
| ADR drafting readiness | Yes |

**Overall: Accept.**

## Next state

Operating Desk may promote the accepted matrix and authority/projection contract to a focused ADR, transcribe explicit Core cardinality Constraints and relation projections, then perform readback Validation.
