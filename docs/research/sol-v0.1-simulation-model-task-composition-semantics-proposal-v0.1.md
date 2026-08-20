# SOL v0.1 Simulation–Model–Task Composition Semantics Proposal v0.1

**Role:** Research  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`  
**Scope:** explicit top-level Simulation / SimulationModel / SimulationTask relations and minimum cardinalities

## 1. Problem

ADR-0014 correctly removed `children` as machine-readable semantics. The top-level diagram:

```text
Simulation
├── SimulationModel
└── SimulationTask
```

therefore has no canonical machine-readable relation yet.

A valid v0.1 contract must answer, without relying on tree indentation:

1. how a `Simulation` references its semantic model;
2. how it references the computational tasks performed in that simulation context;
3. how one `SimulationModel` is reused by multiple tasks;
4. whether `Analysis` is itself a task or a component owned by another task container;
5. what minimum cardinalities are required without importing backend execution structure.

## 2. Constraints

The design SHALL preserve:

- model/task separation;
- analysis/solver separation;
- model reuse across tasks;
- explicit relations instead of structural inference;
- single direct taxonomic inheritance from ADR-0008;
- backend independence;
- no ownership/lifecycle semantics unless explicitly justified;
- no dependency on installed backend runtimes or licenses.

Do not redesign MappingPlan, BackendAdapter, QRC, or result execution lifecycle in this task.

## 3. External reference evidence

### MOOSE

A basic MOOSE input file combines model-definition sections (`Mesh`, `Variables`, `Kernels`, `BCs`) with an `Executioner`; the Executioner defines the problem type and solving method. This is evidence that a backend artifact may bind one concrete task/execution configuration closely to one serialized model, but it is not evidence that SOL Core must make model and task the same identity.

Official evidence:
- https://mooseframework.inl.gov/moose/getting_started/examples_and_tutorials/examples/ex01_inputfile.html

### COMSOL

The COMSOL API states that `model.study` stores a **list of studies**, with each study containing study steps and each study step defining a solver-ready problem. A model can therefore support multiple computational studies without duplicating the semantic model object.

Official evidence:
- https://doc.comsol.com/6.3/doc/com.comsol.help.comsol/comsol_api_general.47.60.html

### Ansys Workbench

Workbench distinguishes a shared Model cell from analysis-system Setup/Solution/Results cells. Official documentation states that a second system can be generated linked at the Model cell of a first system, supporting reuse of model definition across analysis systems.

Official evidence:
- https://ansyshelp.ansys.com/public/Views/Secured/corp/v252/en/wb2_help/wb2h_typesofcells.html

### Palantir Ontology conceptual reference

Palantir recommends semantically meaningful links between independently identified objects and supports one-to-one, one-to-many, and many-to-many link types. This supports modeling Simulation/Model/Task as explicit semantic links rather than using containment syntax or inheritance when no subtype relationship exists.

Conceptual evidence:
- https://www.palantir.com/docs/foundry/ontology/ontology-structural-guidance
- https://www.palantir.com/docs/foundry/object-link-types/link-types-overview

Palantir is conceptual evidence only, not normative SOL authority.

## 4. Candidate structures

### Candidate A — `SimulationTask` as a container owning `Analysis`

```text
Simulation
  has_model -> SimulationModel
  has_task  -> SimulationTask

SimulationTask
  has_analysis -> Analysis
```

Advantages:
- preserves the old visual grouping literally.

Problems:
- introduces an otherwise semantically thin task container around the concept that already answers the task question (`Analysis`);
- creates duplicate identity/provenance/lifecycle questions between SimulationTask and Analysis;
- requires another relation and cardinality solely to recover meaning lost by the old diagram;
- does not improve backend independence.

**Research disposition: reject as unnecessary indirection.**

### Candidate B — `Analysis is_a SimulationTask`

```text
SimulationTask
    ▲
    │ is_a
 Analysis
    ▲
    ├── StationaryAnalysis
    ├── TransientAnalysis
    ├── FrequencyDomainAnalysis
    ├── EigenvalueAnalysis
    ├── ParametricAnalysis
    └── OptimizationAnalysis

Analysis
  solved_by -> SolverConfiguration
```

Interpretation:

- `SimulationTask` is the abstract semantic category “a computational task performed on a simulation model”.
- `Analysis` is the v0.1 task type that specifies the computational question.
- `SolverConfiguration` is **not** a task subtype; it remains a separate numerical configuration used to solve an Analysis.

Advantages:
- directly matches the accepted definitions: `Analysis` defines the computational question and `SolverConfiguration` defines how it is solved;
- removes the need for a content-free task container;
- preserves single direct inheritance;
- permits future task kinds without changing `Simulation` relations;
- makes `SimulationTask -> produces -> Result` naturally apply to Analysis instances through inheritance.

**Research disposition: preferred.**

### Candidate C — remove `SimulationTask` and link Simulation directly to Analysis

This would simplify one type but contradict the frozen top-level semantic distinction and would remove a useful future abstraction.

**Research disposition: reject.**

## 5. Proposed top-level relation contract

### 5.1 `has_model`

```text
has_model
  domain: Simulation
  range: SimulationModel
  source cardinality: exactly 1
```

Meaning: the semantic model used by this Simulation context.

This relation is a **reference/aggregation relation, not exclusive ownership**. A `SimulationModel` MAY be referenced by more than one `Simulation`, Profile/application context, or task graph. Deleting a Simulation therefore has no Core semantic implication that the model is deleted.

Why exactly one:
- the frozen architecture defines singular `SimulationModel` as the complete model-side aggregate;
- multiphysics belongs inside one `SimulationModel` through its physics/math/material/spatial/etc. semantic components;
- allowing multiple top-level models would require a new coupling/co-simulation composition contract that v0.1 has not designed.

### 5.2 `has_task`

```text
has_task
  domain: Simulation
  range: SimulationTask
  source cardinality: min 1
  maximum: unbounded
```

Meaning: the computational tasks included in this Simulation context.

The relation is order-independent and does not imply exclusive ownership. A model may therefore be reused with multiple tasks without duplication.

A `SimulationModel` may exist independently without being wrapped in a `Simulation`. A `Simulation` instance, however, represents model + computation context and therefore requires at least one task.

### 5.3 `Analysis is_a SimulationTask`

Add one explicit taxonomic edge:

```text
Analysis
  is_a: SimulationTask
```

The existing Analysis specializations continue to inherit from Analysis.

Do **not** add:

```text
SolverConfiguration is_a SimulationTask
```

because this would collapse the accepted analysis/solver separation.

## 6. Model-to-task consistency

The existing relation remains:

```text
SimulationModel -> analyzed_by -> Analysis
```

For v0.1, give it these instance-level semantics:

```text
one SimulationModel may be analyzed_by 0..* Analysis instances
one Analysis instance SHALL be associated with exactly 1 SimulationModel
```

The inverse need not introduce a second authoritative relation name in v0.1; an implementation MAY expose an inverse view such as `analyzes_model`, but the canonical semantic edge remains `analyzed_by` unless a later relation-identity decision says otherwise.

### Simulation consistency invariant

For every `Simulation S`:

```text
S has_model M
S has_task  T
```

then, for every task `T` in v0.1:

```text
T is Analysis (or subtype of Analysis)
AND
M analyzed_by T
```

This forces every task contained by a Simulation to operate on that Simulation's model without inventing a duplicate `uses_model` truth.

A model may have additional `analyzed_by` tasks not included in a particular Simulation context; this is how the same model can be reused across multiple Simulation contexts/task sets.

## 7. Result cardinality boundary

Do not make `produces` required in the static authoring graph.

```text
SimulationTask -> produces -> Result
cardinality: 0..*
```

Rationale:
- a valid Simulation/task definition exists before execution and therefore before a Result exists;
- one task may yield multiple result objects/observations;
- runtime result existence is lifecycle evidence, not a prerequisite for semantic validity.

This is a narrow cardinality clarification, not a Result subtype design.

## 8. Minimal thermal reference case

```text
simulation-1 : Simulation
model-1      : SimulationModel
steady-1     : StationaryAnalysis

simulation-1 has_model model-1
simulation-1 has_task  steady-1
model-1 analyzed_by steady-1
steady-1 solved_by solver-config-1   # if explicitly represented
```

The same `model-1` may later be reused:

```text
transient-1 : TransientAnalysis
model-1 analyzed_by transient-1

simulation-2 has_model model-1
simulation-2 has_task transient-1
```

or one Simulation may include both tasks:

```text
simulation-3 has_model model-1
simulation-3 has_task steady-1
simulation-3 has_task transient-1
```

No model duplication is required.

## 9. Backend stress assessment

### MOOSE

A profile may lower each SOL Analysis/task to a separate MOOSE artifact/Executioner configuration if the backend serialization naturally binds one Executioner per input. That is a transformed backend realization, not a reason to constrain Core `has_task` to one.

### COMSOL

Multiple SOL Analysis tasks can map naturally to multiple `Study` objects/steps associated with one COMSOL model.

### Ansys

Multiple SOL task contexts can map to multiple Workbench analysis systems that share/derive from model data, while backend system-link mechanics stay outside Core.

No backend requires a new Core field or ownership relation.

## 10. Proposed machine-readable additions

Conceptually:

```yaml
entities:
  Simulation: {}
  SimulationModel: {}
  SimulationTask: {}
  Analysis:
    is_a: SimulationTask

relations:
  has_model:
    domain: Simulation
    range: SimulationModel
    cardinality:
      min: 1
      max: 1

  has_task:
    domain: Simulation
    range: SimulationTask
    cardinality:
      min: 1

  analyzed_by:
    domain: SimulationModel
    range: Analysis
    cardinality:
      min: 0
    inverse_cardinality:
      min: 1
      max: 1

  produces:
    domain: SimulationTask
    range: Result
    cardinality:
      min: 0
```

The exact YAML field names for general cardinality serialization remain non-normative until the language/schema transcription contract is finalized. The semantic cardinalities above are the proposal under review.

## 11. Counterexamples the proposal must survive

### C-ST-01 — model-only artifact

A `SimulationModel` exists without a Simulation/task wrapper.

Expected: valid. `has_model` is a Simulation constraint, not a SimulationModel existence constraint.

### C-ST-02 — Simulation with no task

`Simulation S has_model M` but no `has_task` edge.

Expected: invalid Simulation instance.

### C-ST-03 — task points at the wrong model through `analyzed_by`

```text
S has_model M1
S has_task A1
M2 analyzed_by A1
```

Expected: configuration conflict for S.

### C-ST-04 — one model, multiple tasks

```text
M analyzed_by A1
M analyzed_by A2
S has_model M
S has_task A1
S has_task A2
```

Expected: valid.

### C-ST-05 — multiple models under one Simulation

```text
S has_model M1
S has_model M2
```

Expected: cardinality conflict in v0.1. A future co-simulation/multi-model contract would require explicit new evidence.

### C-ST-06 — SolverConfiguration treated as task

```text
S has_task solver-config-1
```

Expected: type violation. SolverConfiguration is not a SimulationTask subtype.

### C-ST-07 — task definition before results

A valid Analysis has no `produces` edge before execution.

Expected: valid; result cardinality minimum is zero.

## 12. Research verdict

**Recommend Candidate B.**

Adopt:

```text
Simulation --has_model--> exactly 1 SimulationModel
Simulation --has_task--> 1..* SimulationTask
Analysis is_a SimulationTask
SimulationModel --analyzed_by--> 0..* Analysis
one Analysis associated with exactly 1 SimulationModel
SimulationTask --produces--> 0..* Result
```

with the Simulation consistency invariant that every included Analysis task analyzes the Simulation's selected model.

This is the minimum explicit relation model that restores the semantics previously hidden by tree grouping while preserving model reuse, analysis/solver separation, and backend independence.

**Ready for independent Validation.**
