# ADR-0015: Simulation–Model–Task Composition Semantics

**Status:** Accepted  
**Date:** 2026-08-20  
**Amends:** ADR-0013/0014 frozen baseline for the relation scope below

## Context

ADR-0014 removed the ambiguous `children` shorthand from canonical machine-readable semantics. This left the top-level relationship among `Simulation`, `SimulationModel`, and `SimulationTask` intentionally open.

Independent Research/Validation compared candidate structures against the accepted model/task separation, analysis/solver separation, relation reification rules, and cross-backend evidence. Validation rejected `Analysis is_a SimulationTask` because it conflated a reusable computational question (`Analysis`) with the identifiable application of that question to a model (`SimulationTask`).

The accepted design therefore treats `SimulationTask` as a reified semantic application context between exactly one `SimulationModel` and exactly one `Analysis`.

## Decision

### 1. Simulation references exactly one model

SOL v0.1 defines:

```text
has_model
  domain: Simulation
  range: SimulationModel
  source cardinality: exactly 1
```

`has_model` is a semantic reference/aggregation relation, not exclusive ownership. A `SimulationModel` may be reused by multiple Simulation/task contexts.

Exactly one top-level model is used in v0.1 because the complete multiphysics semantic model is represented by one `SimulationModel`. Multi-model/co-simulation composition requires a future explicit contract and is not inferred here.

### 2. Simulation includes one or more tasks

SOL v0.1 defines:

```text
has_task
  domain: Simulation
  range: SimulationTask
  source cardinality: 1..*
```

A `SimulationModel` may exist independently. A `Simulation` represents a model-plus-computation context and therefore requires at least one task.

`has_task` is order-independent and non-owning. No delete/lifecycle cascade is implied.

### 3. SimulationTask is a reified application context

A `SimulationTask` is the independently identifiable application of one `Analysis` to one `SimulationModel`.

SOL v0.1 defines:

```text
uses_model
  domain: SimulationTask
  range: SimulationModel
  source cardinality: exactly 1

has_analysis
  domain: SimulationTask
  range: Analysis
  source cardinality: exactly 1
```

`Analysis` does **not** inherit from `SimulationTask`. `SolverConfiguration` does **not** inherit from `SimulationTask`.

The distinction is:

```text
SimulationModel      = what is modeled
Analysis             = what computational question is asked
SolverConfiguration  = how it is solved
SimulationTask       = application of that Analysis to that model
```

This preserves independent task identity/provenance/result relationships and permits one Analysis definition to be reused by multiple task/model bindings.

### 4. Analysis/solver relation remains separate

The existing relation remains:

```text
Analysis -> solved_by -> SolverConfiguration
```

This ADR does not define solver cardinality, backend defaults, or task-specific solver override semantics.

### 5. `analyzed_by` is derived from task bindings

The existing semantic relation:

```text
SimulationModel -> analyzed_by -> Analysis
```

is retained as a derived relation.

For a complete model-instance graph:

```text
M analyzed_by A
IFF
exists SimulationTask T such that
  T uses_model M
  AND
  T has_analysis A
```

Task bindings are authoritative. An implementation may materialize `analyzed_by` for traversal/indexing, but a materialized edge with no supporting task is stale/inconsistent derived data.

Multiple tasks sharing the same `(M,A)` pair produce one set-valued derived `analyzed_by` edge.

### 6. Simulation consistency invariant

For every Simulation `S`, if:

```text
S has_model M
S has_task T
```

then:

```text
T uses_model M
```

MUST hold.

A task may exist independently of a Simulation aggregate if its `uses_model` and `has_analysis` requirements are satisfied. A task may be referenced by multiple Simulations only when all of those Simulations select the same task model.

### 7. Result relation is optional before execution

The existing relation:

```text
SimulationTask -> produces -> Result
```

has source cardinality:

```text
0..*
```

A task definition is semantically valid before execution and before any Result exists. One task may later be associated with multiple Results.

This ADR does not introduce Result subtypes or runtime execution state.

## Cardinality summary

| Relation | Domain | Range | Source cardinality |
|---|---|---|---|
| `has_model` | Simulation | SimulationModel | exactly 1 |
| `has_task` | Simulation | SimulationTask | 1..* |
| `uses_model` | SimulationTask | SimulationModel | exactly 1 |
| `has_analysis` | SimulationTask | Analysis | exactly 1 |
| `produces` | SimulationTask | Result | 0..* |
| `analyzed_by` | SimulationModel | Analysis | derived set relation |

## Boundary cases

- Model-only `SimulationModel`: valid.
- Simulation with no `has_task`: invalid.
- One model with many tasks: valid.
- Same Analysis reused by different tasks and different models: valid.
- One task with zero/multiple `uses_model`: invalid.
- One task with zero/multiple `has_analysis`: invalid.
- Simulation task whose `uses_model` differs from `Simulation.has_model`: configuration conflict.
- SolverConfiguration supplied as a `has_task` target: type violation.
- Task with zero Results before execution: valid.
- Simulation with two top-level models: invalid in v0.1.

## Backend independence

MOOSE may lower separate SOL tasks to separate input/Executioner artifacts. COMSOL may lower several tasks to Study/step structures under one model. Ansys may use several analysis-system contexts sharing model data. These are backend transformations and do not alter this Core relation contract.

Backend installation, licensing, production Adapter implementation, and execution V&V are outside this design-stage relation decision.

## Deferred

This ADR does not define:

- multi-model/co-simulation top-level composition;
- task workflows/dependencies among multiple analyses;
- solver cardinality/override behavior;
- task execution/status lifecycle in Core;
- Result subtype taxonomy;
- ownership/deletion semantics for `has_model` or `has_task`;
- complete cardinalities for unrelated Core relations.

## Validation evidence

- `docs/research/sol-v0.1-simulation-model-task-composition-semantics-proposal-v0.1.md`
- `docs/validation/sol-v0.1-simulation-model-task-composition-independent-review-v0.1.md`
- `docs/research/sol-v0.1-simulation-model-task-composition-semantics-proposal-v0.2.md`
- `docs/validation/sol-v0.1-simulation-model-task-composition-v0.2-focused-final-review.md`

## Freeze amendment

ADR-0013 remains in force, as amended by ADR-0014 and this ADR. Only the explicit Simulation/Model/Task relation semantics above are added to the frozen baseline.

## Decision summary

SOL v0.1 represents `Simulation` as a non-owning context referencing exactly one `SimulationModel` and one or more `SimulationTask`s. A `SimulationTask` reifies the application of exactly one `Analysis` to exactly one model, `analyzed_by` is derived from those task bindings, and Results remain optional before execution.
