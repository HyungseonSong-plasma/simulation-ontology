# SOL v0.1 ADR-0017 Transcription Focused Readback Validation

**Role:** Validation  
**Date:** 2026-08-20

## Scope

Re-review only the documentation transcription drift identified in `sol-v0.1-adr0017-transcription-readback-validation-v0.1.md`, while confirming the previously passing machine-readable authority/projection state did not regress.

## Inputs

- `docs/decisions/0017-core-relation-cardinality-requiredness-baseline.md`
- `ontology/core/relations.yaml`
- `ontology/core/constraints.yaml`
- `README.md`
- `docs/architecture.md`
- `docs/ontology-language.md`

## Findings

### FR17-01 — repository state recovery

**PASS.**

README now states that the design-stage architecture is frozen through ADR-0017, lists ADR-0017, records the seven generic Core `0..*` source intervals, and keeps backend installation/licensing/production Adapter execution outside the design-stage gate.

### FR17-02 — architecture transcription

**PASS.**

`docs/architecture.md` now reports the baseline through ADR-0017 and explicitly records:

```text
represented_by
closed_by
parameterized_by
defined_on
discretized_by
solved_by
observed_by
```

as generic Core source interval `0..*`.

It also preserves that cardinality is normatively a Constraint and relation-side `source_cardinality` is only a matching projection.

### FR17-03 — SOL language transcription

**PASS.**

`docs/ontology-language.md` now reports the baseline through ADR-0017 and includes a normative rule for the seven-relation Core cardinality baseline plus projection missing/mismatch failure semantics.

Generic Core cardinality is no longer listed as an open design question; remaining work is schema enforcement and narrower domain/Interface/Profile refinement.

### FR17-04 — machine authority/projection regression

**PASS.**

The seven relation projections remain explicit `0..*`, and `constraints.yaml` retains one canonical structured outgoing Core Cardinality Constraint for each relation plus projection-consistency semantics.

No relation-side projection is treated as an independent intersection contributor.

### FR17-05 — prior contract regression

**PASS.**

No regression was found to ADR-0015 or ADR-0016 cardinalities, `analyzed_by` derivation, `includes_component` allowed-pair semantics, or `applied_to 1..*`.

## Final verdict

| Dimension | Verdict |
|---|---|
| README state recovery | PASS |
| Architecture transcription | PASS |
| SOL language transcription | PASS |
| Machine cardinality authority | PASS |
| Projection consistency semantics | PASS |
| ADR-0015/0016 regression | None |
| Architecture defect | None |

**Overall: Accept.**

## Next state

ADR-0017 cycle is complete. The next dependency-ordered consolidation task should define the canonical machine-readable Constraint schema for already accepted ADR-0007/0012/0015/0016/0017 semantics before Interface and domain/profile refinement schemas depend on it.
