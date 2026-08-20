# SOL v0.1 Simulation–Model–Task Composition — Independent Review v0.1

**Role:** Independent Validation  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`

## 1. Inputs and evaluation contract

Validation used:

- frozen model/task and analysis/solver separation principles;
- ADR-0007 relation/reification rules;
- ADR-0008 explicit inheritance rules;
- ADR-0014 explicit-relation / no-implicit-children rule;
- current `docs/architecture.md` and `docs/ontology-language.md`;
- `docs/research/sol-v0.1-simulation-model-task-composition-semantics-proposal-v0.1.md` as the proposal under test;
- official MOOSE, COMSOL, Ansys reference documentation and Palantir structural guidance as evidence, not authority over SOL.

Verdict domain: `Accept / Revise / Reject / Blocked`.

Primary criterion: the contract must preserve distinct semantic identity for **model**, **computational question**, **solver configuration**, **application of the question to a model**, and **result**, without needing backend object structure to disambiguate them.

## 2. Executive verdict

**REVise.**

The proposal is correct on the need for explicit top-level links and on the minimum top-level cardinalities, but `Analysis is_a SimulationTask` collapses two distinct semantic roles. This is a proposal-level architecture defect, not a backend limitation.

| Proposal item | Verdict |
|---|---|
| `Simulation -> has_model -> exactly 1 SimulationModel` | **Accept** |
| `Simulation -> has_task -> 1..* SimulationTask` | **Accept** |
| relations are non-owning/reusable references | **Accept** |
| `Analysis is_a SimulationTask` | **Reject** |
| `SolverConfiguration` remains separate | **Accept** |
| model reuse across tasks | **Accept** |
| `SimulationTask -> produces -> 0..* Result` | **Accept** |
| one Analysis must have exactly one incoming `analyzed_by` model | **Reject / overconstrained** |
| proposal ready for ADR | **No — focused revision required** |

## 3. Main counterexample — Analysis definition reuse

Consider one reusable stationary computational question:

```text
steady-default : StationaryAnalysis
```

and two independent semantic models:

```text
thermal-A : SimulationModel
thermal-B : SimulationModel
```

It is semantically coherent to define two task instances:

```text
task-A = apply steady-default to thermal-A
task-B = apply steady-default to thermal-B
```

with different task identities/results/provenance even though the same Analysis definition is reused.

The Research proposal instead makes the Analysis instance itself the task and requires each Analysis to be associated with exactly one SimulationModel. A conforming author must therefore duplicate `steady-default` merely to bind it to another model:

```text
steady-default-A
steady-default-B
```

This conflates **what computational question is asked** with **the task/application of that question to a particular model**.

The distinction is especially material once a task carries or acquires:

- model binding;
- task provenance;
- execution status/revision;
- task-specific overrides;
- produced Result identities;
- scheduling/orchestration identity in a later layer.

Those properties belong naturally to the reified task/application context, not to the reusable `Analysis` definition.

**Classification: proposed Architecture defect — semantic-role conflation.**

## 4. Reification test

SOL's existing relation rule says a relationship should be reified when the relationship itself needs substantial provenance, state, constraints, mappings, or relations.

The semantic relationship:

```text
SimulationModel <--- task/application ---> Analysis
```

already has an independent relation to `Result` through `SimulationTask -> produces -> Result` and can reasonably require provenance/lifecycle identity. `SimulationTask` therefore has a justified role as a reified application of an Analysis to a model; it is not a content-free wrapper.

Palantir's structural guidance is consistent supporting evidence: direct links are suitable for simple relationships, while object-backed links are appropriate when the relationship carries its own metadata such as role/status/allocation. SOL should apply its own equivalent reification principle here.

## 5. Backend evidence does not require Analysis-as-task inheritance

### COMSOL

A COMSOL model stores a list of Studies; each Study contains solver-ready study steps. This supports multiple task/application contexts over one model, but does not imply that SOL Analysis identity must be the same as task identity.

### Ansys Workbench

Model data can be linked/shared into another analysis system. This supports model reuse across task/system contexts and again favors separate model/task identities.

### MOOSE

A basic input artifact combines model blocks with one Executioner configuration. If a Profile emits one backend artifact per SOL task, that is a transformed realization. It gives no reason to identify reusable Analysis semantics with task instance identity.

**Backend verdict: no backend constraint forces the rejected inheritance.**

## 6. Accepted top-level structure

The following remains deterministic and minimal:

```text
Simulation
  has_model -> exactly 1 SimulationModel
  has_task  -> 1..* SimulationTask
```

Both links are semantic aggregation/reference links, not exclusive ownership or lifecycle containment.

Consequences:

- a `SimulationModel` can exist independently;
- one model can be referenced by many tasks and multiple Simulation contexts;
- a Simulation with no task is invalid, while a model-only artifact is valid;
- multiple top-level models under one Simulation remain out of v0.1 scope unless a future co-simulation contract is accepted.

## 7. Required task reification contract

Research should revise to a minimal task structure such as:

```text
SimulationTask
  applies_to -> exactly 1 SimulationModel
  has_analysis -> exactly 1 Analysis
  produces -> 0..* Result
```

Names may be refined, but the semantic dimensions must stay orthogonal.

`SolverConfiguration` remains separate. The existing:

```text
Analysis -> solved_by -> SolverConfiguration
```

may remain unchanged in this focused revision; solver cardinality and task-specific solver override behavior are not required to close the current top-level composition gap.

### Simulation consistency invariant

For every Simulation `S`, model `M`, and task `T`:

```text
S has_model M
S has_task T
=> T applies_to M
```

This is deterministic and avoids deriving task/model binding from container position alone.

## 8. Effect on existing `analyzed_by`

Do not impose the Research proposal's inverse `exactly 1 model per Analysis` rule.

The current relation:

```text
SimulationModel -> analyzed_by -> Analysis
```

can remain as a semantic/derived relationship indicating that an Analysis is applicable/used on a model, but task binding is authoritative through the reified SimulationTask.

At minimum, the revised proposal must choose one of these deterministic contracts:

1. **Derived relation:** `M analyzed_by A` exists iff at least one SimulationTask applies `A` to `M`; or
2. **Declared consistency relation:** it may be authored, but every task `(M,A)` requires a matching `analyzed_by` edge and contradictory edges are invalid.

Option 1 is preferred because it avoids duplicate authoritative truth.

## 9. Boundary cases for revision

The revised contract must give one result for:

### V-ST-01 model-only definition
Valid.

### V-ST-02 Simulation with no task
Invalid.

### V-ST-03 one model + two task instances using the same Analysis
Valid.

### V-ST-04 two models + two task instances using the same Analysis definition
Valid across two Simulation contexts; no Analysis duplication required.

### V-ST-05 task included by Simulation S but `applies_to` a different model than `S.has_model`
Configuration conflict.

### V-ST-06 SolverConfiguration supplied as `has_task` target
Type violation.

### V-ST-07 valid task before execution with zero Results
Valid.

### V-ST-08 one Simulation with two `has_model` targets
Cardinality conflict in v0.1.

## 10. Scope preservation

The revision SHALL NOT:

- reopen ADR-0010/0011/0012;
- introduce backend execution state into Core;
- define production Adapter lifecycle;
- require installed MOOSE/COMSOL/Ansys runtimes;
- design multi-model/co-simulation in this cycle;
- define Result subtypes;
- solve complete solver-configuration cardinality/override semantics.

## 11. Final verdict

**REVise — focused semantic-role correction only.**

Return to Research with the accepted `has_model`, `has_task`, model-reuse, and zero-result boundaries intact. Replace `Analysis is_a SimulationTask` with a reified `SimulationTask` that binds exactly one SimulationModel and exactly one Analysis, then re-review only that delta and the treatment of `analyzed_by`.
