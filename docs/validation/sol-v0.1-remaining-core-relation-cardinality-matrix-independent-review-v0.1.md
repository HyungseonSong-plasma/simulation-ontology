# SOL v0.1 Remaining Core Relation Cardinality Matrix — Independent Review v0.1

**Role:** Validation  
**Date:** 2026-08-20

## Inputs

- `docs/research/sol-v0.1-remaining-core-relation-cardinality-matrix-proposal-v0.1.md`
- ADR-0007 Constraint architecture/composition
- accepted relation semantics study
- ADR-0015 and ADR-0016 fixed relation cardinalities
- current machine-readable relation/constraint registries

Research conclusions were not treated as authoritative.

## Evaluation contract

Determine whether independent validators can distinguish:

```text
required
optional/unconstrained
unknown/not-yet-transcribed
```

and compose later domain/interface/profile refinements without declaration-order behavior or a second cardinality authority.

## Findings

### RC-V1 — Semantic `[0,∞]` matrix is acceptable

**Verdict: Accept.**

For the seven relations in scope, no universal Core minimum or maximum is justified:

```text
represented_by
a closed_by
parameterized_by
defined_on
discretized_by
solved_by
observed_by
```

The proposal correctly separates generic Core graph validity from domain completeness, profile executability, and backend realization readiness.

The following counterexamples are valid at generic Core level:

- PhysicsModel with no selected MathematicalModel yet;
- closure-free MathematicalModel;
- non-spatial/lumped MathematicalModel;
- Analysis before solver selection;
- Result with no ObservationModel;
- multiple candidate mathematical representations or solver configurations.

Domain/interface/profile constraints may narrow `[0,∞]` conjunctively under ADR-0007.

### RC-V2 — Explicit-vs-omitted cardinality remains ambiguous

**Classification:** Language/schema transcription contract defect.  
**Verdict: Revise.**

The proposal says explicit machine-readable `0..*` **SHOULD** be preferred so tools can distinguish deliberate unconstrained semantics from unknown/not-yet-transcribed semantics. `SHOULD` does not force independent implementations to make the same distinction.

Counterexample:

```text
represented_by:
  domain: PhysicsModel
  range: MathematicalModel
  # no source_cardinality field
```

Validator A interprets omission as canonical `[0,∞]`; Validator B interprets omission as cardinality not yet transcribed. Both conform to the current wording.

Required correction: for the frozen Core relation registry, either:

1. require explicit canonical `source_cardinality: {min: 0, max: unbounded}` for all seven relations; or
2. require an equally explicit status marker meaning `unconstrained_by_core`.

The chosen v0.1 transcription must make omission non-authoritative/invalid for this frozen matrix.

### RC-V3 — Relation-side field versus Constraint authority is not fully normalized

**Classification:** Language/schema transcription contract defect.  
**Verdict: Revise.

ADR-0007 and the relation study place cardinality semantics normatively in Constraint. The current consolidation also writes `source_cardinality` next to Relation definitions. The proposal calls this authoring convenience but does not define what happens if relation-side and constraint-side representations disagree.

Counterexample:

```text
relations.yaml: solved_by source_cardinality = 0..*
constraints.yaml: solved_by cardinality = 1..1
```

One validator may treat the relation field as authoritative; another may treat the Constraint as authoritative.

Required correction:

- Constraint semantics remain normative;
- any relation-side `source_cardinality` is a canonical projection/mirror of the effective Core cardinality Constraint for authoring/readability;
- mismatch between the projection and canonical Core Constraint is a transcription/validation failure, never override or intersection;
- later domain/interface/profile constraints compose with the canonical Core Constraint under ADR-0007 and do not rewrite the Core projection.

### RC-V4 — Incoming cardinality remains safely unconstrained

**Verdict: Accept.**

No evidence in this scope requires inverse-functional/incoming maxima or minima. `[0,∞]` incoming cardinality is a valid generic Core boundary.

### RC-V5 — Fixed ADR-0015/0016 relations remain out of scope

**Verdict: Accept.**

No regression was found to:

```text
has_model 1..1
has_task 1..*
uses_model 1..1
has_analysis 1..1
produces 0..*
includes_component 0..*
applied_to 1..*
analyzed_by derived
```

## Final verdict

| Dimension | Verdict |
|---|---|
| Seven-relation semantic matrix `[0,∞]` | Accept |
| Incoming cardinality `[0,∞]` | Accept |
| Core/domain/profile refinement model | Accept |
| Explicit-vs-omitted serialization | Revise |
| Relation-side vs Constraint authority | Revise |
| Architecture redesign required | No |

**Overall: Revise.**

## Next state

Research SHALL revise only RC-V2 and RC-V3. Preserve the seven-relation `[0,∞]` semantic matrix and all prior fixed cardinalities. Then perform a focused final contract review before ADR drafting.
