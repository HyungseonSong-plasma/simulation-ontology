# SOL v0.1 ADR-0017 Transcription Readback Validation v0.1

**Role:** Validation  
**Date:** 2026-08-20

## Inputs

- ADR-0017
- `ontology/core/relations.yaml`
- `ontology/core/constraints.yaml`
- `docs/architecture.md`
- `docs/ontology-language.md`
- `README.md`

## Findings

### R17-01 — Machine relation projections

**PASS.**

All seven ADR-0017 relations explicitly project `source_cardinality: 0..unbounded`.

### R17-02 — Canonical Cardinality Constraint authority

**PASS.**

`constraints.yaml` contains one canonical structured outgoing Cardinality Constraint for each of the seven relations and a projection-consistency rule. Relation-side values are documented as projections rather than independent constraint contributors.

### R17-03 — README state recovery

**REVise — documentation transcription drift.**

README still identifies the frozen baseline through ADR-0016 and does not list ADR-0017.

### R17-04 — Architecture state recovery

**REVise — documentation transcription drift.**

`docs/architecture.md` still reports the baseline through ADR-0016 and still describes remaining relation cardinality work as open. ADR-0017's generic `[0,∞]` matrix is not summarized.

### R17-05 — SOL language state recovery

**REVise — documentation transcription drift.**

`docs/ontology-language.md` still reports the baseline through ADR-0016 and lists remaining Core relation cardinality as an open design question rather than accepted semantics with schema enforcement pending.

## Verdict

| Dimension | Verdict |
|---|---|
| ADR-0017 semantic contract | PASS |
| Machine-readable relation projection | PASS |
| Machine-readable canonical Constraint authority | PASS |
| Human-readable transcription | Revise |
| Architecture defect | None |

**Overall transcription verdict: Revise.**

## Required correction

Do not reopen ADR-0017. Update README, architecture, and ontology-language only so that:

- baseline status is frozen through ADR-0017;
- the seven generic Core source intervals are recorded as `0..*`;
- cardinality remains a Constraint and relation-side values are projections;
- remaining work is schema enforcement and domain/interface/profile refinement rather than an unresolved generic Core cardinality decision.

Then perform focused readback Validation.
