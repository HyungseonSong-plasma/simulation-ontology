# SOL v0.1 Remaining Core Relation Cardinality / Requiredness Matrix Proposal v0.1

**Role:** Research  
**Date:** 2026-08-20  
**Scope:** remaining Core relations after ADR-0015 and ADR-0016

## 1. Objective

Define the generic Core model-instance cardinality/requiredness contract for:

```text
represented_by
closed_by
parameterized_by
defined_on
discretized_by
solved_by
observed_by
```

without reopening cardinalities already fixed by ADR-0015/0016.

The goal is not to make every relation mandatory. The goal is to distinguish deliberately unconstrained/optional Core relations from genuinely unresolved cardinality semantics so independent validators do not invent requiredness.

## 2. Governing constraints

Preserve:

- ADR-0007 conjunctive Constraint composition and interval intersection;
- Relation study rule that cardinality/requiredness belongs normatively to Constraint rather than intrinsic edge meaning;
- ADR-0014 explicit taxonomy and no tree-inferred requiredness;
- ADR-0015 task/model cardinalities;
- ADR-0016 component and `applied_to` cardinalities;
- design-stage separation from backend installation/runtime requirements.

## 3. Generic Core versus completeness/executability constraints

SOL Core must distinguish:

```text
schema/model graph validity
```

from:

```text
domain completeness
profile executability
backend realization readiness
```

A semantic Entity may exist before all domain/profile choices are supplied. Therefore a generic Core relation SHOULD NOT be made required merely because one executable backend workflow requires it.

Examples:

- a PhysicsModel may be authored before a mathematical formulation is selected;
- a MathematicalModel may be non-spatial or not yet discretized;
- a ConstitutiveModel may use literal/functional values without a MaterialModel aggregate;
- an Analysis may exist before a SolverConfiguration is chosen;
- a Result may exist without an ObservationModel definition;
- a complete domain/profile may later require any of these through narrower Constraints.

## 4. Cardinality normalization rule

For the relations in this proposal, the generic Core source occurrence interval is:

```text
[0, +infinity]
```

This means:

- minimum zero: Core does not require an edge;
- maximum unbounded: Core does not impose a functional/max-one rule;
- incoming/inverse cardinality is also unconstrained by Core unless another accepted contract says otherwise.

Under ADR-0007, domain/interface/profile constraints MAY narrow this interval conjunctively.

Example:

```text
Core solved_by: [0, infinity]
ExecutableAnalysisProfile solved_by: [1, infinity]
Effective: [1, infinity]
```

No declaration-order override exists.

## 5. Proposed matrix

| Relation | Core source interval | Core incoming interval | Rationale |
|---|---:|---:|---|
| `represented_by` | 0..* | 0..* | Physics meaning can be authored independently; multiple mathematical formulations may represent the same physical model. |
| `closed_by` | 0..* | 0..* | Some mathematical systems need no separate closure; others may require several constitutive/closure models. |
| `parameterized_by` | 0..* | 0..* | A constitutive model may use zero, one, or multiple material contexts/properties; domain completeness may narrow this. |
| `defined_on` | 0..* | 0..* | Global/lumped/non-spatial mathematical models are possible; multiphysics/component contexts may require more than one spatial context. |
| `discretized_by` | 0..* | 0..* | Analytic/lumped models may have no numerical discretization; alternative/multiple numerical representations are allowed semantically. |
| `solved_by` | 0..* | 0..* | Analysis identity is independent of solver choice; multiple solver configurations may be alternatives or valid configurations. |
| `observed_by` | 0..* | 0..* | Results can exist without observation/output definitions and may support multiple observations. |

## 6. Why Core does not require `represented_by >= 1`

The architecture states the semantic distinction between physical meaning and mathematical representation, but that does not require every intermediate/partial PhysicsModel instance to already have a selected MathematicalModel.

A complete domain/reference model MAY impose:

```text
PhysicsModel represented_by min 1
```

when that completeness contract is semantically justified.

Making `min 1` universal in Core would prevent incremental model construction and abstract reusable PhysicsModel definitions.

## 7. Why Core does not require `solved_by >= 1`

`Analysis` defines the computational question and `SolverConfiguration` defines how it is solved. The separation itself implies solver selection is a distinct decision.

An executable profile MAY require:

```text
Analysis solved_by min 1
```

but a solver-independent semantic Analysis remains valid before that binding.

Backend evidence supports avoiding a Core max-one restriction: COMSOL permits one or more studies and exposes solver sequences as attachable/associable model entities rather than collapsing study identity into one globally fixed solver object.

## 8. Why maxima remain unbounded

No reviewed Core relation in this scope has evidence for a universal source maximum of one.

Examples:

- one PhysicsModel can have alternative mathematical representations;
- one MathematicalModel can use multiple constitutive or numerical contexts;
- one Analysis can be associated with alternative SolverConfigurations;
- one Result can be consumed by many observation/output definitions.

If a domain/profile requires exactly one, it can intersect with Core `[0,∞]`:

```text
[0, infinity] intersect [1,1] = [1,1]
```

## 9. Fixed relations excluded from revision

This proposal does not change:

```text
has_model       1..1
has_task        1..*
uses_model      1..1
has_analysis    1..1
produces        0..*
includes_component 0..*
applied_to      1..*
```

`analyzed_by` remains derived from task bindings and has no independent authoring cardinality.

## 10. Serialization rule

For this decision, explicit machine-readable:

```text
source_cardinality:
  min: 0
  max: unbounded
```

SHOULD be preferred during consolidation so validators can distinguish:

```text
deliberately unconstrained by Core
```

from:

```text
cardinality not yet transcribed / unknown
```

Normatively, cardinality remains a Constraint under ADR-0007. Embedding the interval next to a RelationDefinition is authoring/transcription convenience, not a reclassification of cardinality as intrinsic relation meaning.

## 11. Counterexamples

### RC-01 — backend-required solver leaks into Core

Validator rejects an Analysis with zero `solved_by` because MOOSE requires an Executioner to run.

Expected: Core-valid. Executability/profile validation may fail separately.

### RC-02 — artificial max-one mathematical representation

```text
physics-1 represented_by math-fluid
physics-1 represented_by math-reduced
```

Expected: Core-valid unless another constraint makes the representations mutually exclusive.

### RC-03 — closure-free mathematical model

A fully specified mathematical model has zero `closed_by` edges.

Expected: Core-valid.

### RC-04 — non-spatial mathematical model

A lumped/global MathematicalModel has zero `defined_on` edges.

Expected: Core-valid.

### RC-05 — multiple observations

One Result has several `observed_by` edges.

Expected: Core-valid.

### RC-06 — profile refinement

Core `solved_by [0,∞]` plus profile `solved_by [1,1]`.

Expected effective interval: `[1,1]`; no conflict.

## 12. Research verdict

Recommend explicit generic Core interval `[0,∞]` for all seven remaining relations in scope, while allowing domain/interface/profile contracts to narrow requiredness and maxima conjunctively.

**Ready for independent Validation.**
