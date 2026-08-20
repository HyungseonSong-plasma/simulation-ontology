# SOL v0.1 Simulation–Model–Task Composition Semantics Proposal v0.2

**Role:** Research revision  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`  
**Primary input:** `docs/validation/sol-v0.1-simulation-model-task-composition-independent-review-v0.1.md`

## 1. Revision scope

This revision preserves the independently accepted parts of v0.1:

```text
Simulation -> has_model -> exactly 1 SimulationModel
Simulation -> has_task  -> 1..* SimulationTask
SimulationTask -> produces -> 0..* Result
```

and the non-owning/reference semantics of `has_model` and `has_task`.

It changes only:

1. the semantic relation between `SimulationTask` and `Analysis`;
2. the task-to-model binding;
3. the authority/derivation rule for existing `SimulationModel -> analyzed_by -> Analysis`.

`Analysis is_a SimulationTask` is withdrawn.

## 2. Revised semantic roles

The four relevant identities remain orthogonal:

```text
SimulationModel
  = what is modeled

Analysis
  = what computational question is asked

SolverConfiguration
  = how the numerical solve is configured

SimulationTask
  = the identifiable application of one Analysis to one SimulationModel
```

A `SimulationTask` is therefore a justified reification of the model–analysis application relation. It may independently participate in provenance, result, scheduling, execution, mapping, or lifecycle relations without forcing those concerns onto the reusable `Analysis` definition.

## 3. Normative relation proposal

### 3.1 Simulation aggregation

```text
has_model
  domain: Simulation
  range: SimulationModel
  cardinality from Simulation: exactly 1

has_task
  domain: Simulation
  range: SimulationTask
  cardinality from Simulation: 1..*
```

Both are semantic reference/aggregation links, not exclusive ownership. No delete cascade or lifecycle ownership is implied.

### 3.2 Task model binding

```text
uses_model
  domain: SimulationTask
  range: SimulationModel
  cardinality from SimulationTask: exactly 1
```

Meaning: the semantic model to which this task applies.

The name `uses_model` is intentionally task-specific and does not reuse the generic condition relation `applied_to`.

### 3.3 Task analysis binding

```text
has_analysis
  domain: SimulationTask
  range: Analysis
  cardinality from SimulationTask: exactly 1
```

Meaning: the computational question performed by this task.

An `Analysis` instance MAY be reused by multiple SimulationTask instances, including tasks that use different SimulationModels.

### 3.4 Solver configuration remains separate

The existing relation remains:

```text
Analysis -> solved_by -> SolverConfiguration
```

No `is_a` edge is added between `Analysis` and `SimulationTask`, and no `is_a` edge is added from `SolverConfiguration` to `SimulationTask`.

Solver cardinality, defaults, and task-specific override semantics are explicitly outside this revision.

## 4. Derived `analyzed_by`

The existing relation:

```text
SimulationModel -> analyzed_by -> Analysis
```

is retained as a **derived semantic relation**, not an independent authoritative assertion.

For a complete model-instance graph:

```text
M analyzed_by A
IFF
exists SimulationTask T such that
  T uses_model M
  AND
  T has_analysis A
```

Consequences:

- one Analysis may be reused on several models through distinct tasks;
- one model may be analyzed by many Analyses;
- duplicate authoritative truth between task bindings and `analyzed_by` is avoided;
- implementations MAY materialize `analyzed_by` for traversal/indexing, but validators SHALL derive/check it from task bindings;
- a materialized `analyzed_by` edge that lacks supporting task bindings is stale/inconsistent derived data, not an alternative source of semantic truth.

## 5. Simulation consistency invariant

For every Simulation `S`:

```text
S has_model M
S has_task T
```

then:

```text
T uses_model M
```

must hold.

Therefore a Simulation is an explicit context grouping one model with one or more tasks that operate on that model.

A task may exist independently of a Simulation aggregate as long as it has exactly one model and exactly one Analysis. A task MAY be referenced by more than one Simulation only when all referencing Simulations select the same `uses_model` target; the Core relation itself remains non-owning.

## 6. Cardinality summary

| Relation | Domain | Range | Source cardinality |
|---|---|---|---|
| `has_model` | Simulation | SimulationModel | exactly 1 |
| `has_task` | Simulation | SimulationTask | 1..* |
| `uses_model` | SimulationTask | SimulationModel | exactly 1 |
| `has_analysis` | SimulationTask | Analysis | exactly 1 |
| `produces` | SimulationTask | Result | 0..* |
| `analyzed_by` | SimulationModel | Analysis | derived, set-valued |

No inverse ownership cardinality is imposed on `SimulationModel`, `SimulationTask`, or `Analysis` beyond the source-side requirements above.

## 7. Boundary cases

### R-ST-01 — model-only definition

```text
M : SimulationModel
```

No Simulation/task required.

**Valid.**

### R-ST-02 — Simulation with no task

```text
S has_model M
```

No `has_task`.

**Invalid: `has_task` minimum-cardinality conflict.**

### R-ST-03 — same model, same Analysis, two task identities

```text
T1 uses_model M
T1 has_analysis A

T2 uses_model M
T2 has_analysis A
```

**Valid.** Task identity/provenance/result identity may differ independently.

Derived relation contains one set edge:

```text
M analyzed_by A
```

### R-ST-04 — same Analysis reused across two models

```text
T1 uses_model M1
T1 has_analysis A

T2 uses_model M2
T2 has_analysis A
```

**Valid.** No duplication of Analysis is required.

Derived:

```text
M1 analyzed_by A
M2 analyzed_by A
```

### R-ST-05 — Simulation/task model mismatch

```text
S has_model M1
S has_task T
T uses_model M2
```

where `M1 != M2`.

**Invalid: Simulation task-model consistency conflict.**

### R-ST-06 — missing task analysis

```text
T uses_model M
```

with no `has_analysis`.

**Invalid: task cardinality conflict.**

### R-ST-07 — multiple analyses on one task

```text
T has_analysis A1
T has_analysis A2
```

**Invalid in v0.1.** A multi-analysis workflow is represented by multiple SimulationTask identities or requires future workflow/orchestration evidence.

### R-ST-08 — SolverConfiguration supplied as task

```text
S has_task solver-config-1
```

**Invalid: type violation.**

### R-ST-09 — task before execution

A valid `T` has zero `produces` edges.

**Valid.** Result creation is not a semantic authoring prerequisite.

### R-ST-10 — stale materialized `analyzed_by`

```text
M analyzed_by A
```

but no task has `(uses_model=M, has_analysis=A)`.

**Invalid derived-state consistency if `analyzed_by` is materialized.**

## 8. Minimal thermal example

```text
M  : SimulationModel
A1 : StationaryAnalysis
A2 : TransientAnalysis
T1 : SimulationTask
T2 : SimulationTask
S  : Simulation

T1 uses_model M
T1 has_analysis A1
T2 uses_model M
T2 has_analysis A2

S has_model M
S has_task T1
S has_task T2
```

Derived:

```text
M analyzed_by A1
M analyzed_by A2
```

A Profile may map these tasks differently:

- MOOSE: potentially one generated input/Executioner realization per task;
- COMSOL: multiple Study/step realizations inside one backend model;
- Ansys: multiple analysis-system/task contexts sharing model data.

These are transformed backend realizations and do not alter Core task semantics.

## 9. Machine-readable impact

The revision requires only:

1. add `has_model`, `has_task`, `uses_model`, and `has_analysis` Relation definitions;
2. attach their minimum/maximum cardinality constraints;
3. mark `analyzed_by` as derived with the task-based derivation invariant;
4. attach `produces` minimum zero semantics;
5. add a Simulation consistency constraint.

It does **not** add:

- a new Entity Type;
- any new inheritance edge;
- backend vocabulary;
- task execution state;
- Result subtypes;
- solver override rules;
- multi-model/co-simulation semantics.

## 10. Finding resolution matrix

| Validation finding | v0.2 disposition |
|---|---|
| task container considered unnecessary | **Resolved** — SimulationTask explicitly reifies model–Analysis application |
| `Analysis is_a SimulationTask` conflates roles | **Resolved** — inheritance removed |
| Analysis reuse across models blocked | **Resolved** — Analysis has no inverse model cardinality |
| model/task binding needs authority | **Resolved** — `uses_model` exactly 1 |
| task/Analysis binding needs authority | **Resolved** — `has_analysis` exactly 1 |
| duplicate truth in `analyzed_by` | **Resolved** — relation is derived from tasks |
| Simulation model mismatch | **Resolved** — explicit consistency invariant |
| SolverConfiguration separation | **Preserved** |
| zero-result pre-execution validity | **Preserved** |

## 11. Research verdict

**Ready for focused final Validation.**

The revised contract keeps the independently accepted top-level cardinalities while making `SimulationTask` a semantically justified reified application entity rather than a taxonomic alias of `Analysis`.
