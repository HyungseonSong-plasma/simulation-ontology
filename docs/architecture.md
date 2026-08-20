# Core Simulation Ontology Architecture

**Version:** 0.1  
**Status:** Frozen design baseline, amended through ADR-0015

## 1. Purpose

The Core Simulation Ontology provides a solver-independent and domain-independent semantic framework for simulation models. Software-specific concepts such as a MOOSE `Kernel`, COMSOL `Physics Feature`, or Ansys analysis object are backend representations rather than core ontology concepts.

The ontology should answer:

> What is modeled, how is it represented mathematically, what material and spatial context applies, how is it discretized, and what computational task is performed on it?

The project is intended to act as a foundational framework rather than a complete ontology for any single physics domain or simulation package.

## 2. Framework layering

```text
Application / Product
        │
        ▼
Simulation Profile
      ╱     ╲
     ▼       ▼
Domain     Backend
Ontology   Ontology
     ╲       ╱
      ▼     ▼
simulation-ontology
   Core Framework
```

The Core Framework defines solver- and domain-independent semantic primitives, relations, constraints, extension rules, and language contracts. Domain and Backend ontologies specialize toward the Core and are composed through Simulation Profiles. The Core MUST NOT depend on domain ontologies, backend ontologies, profiles, or applications.

## 3. Dependency rule

```text
Application
    ↓ depends on
Simulation Profile
    ↓
Domain Ontology + Backend Ontology
    ↓
simulation-ontology
```

Domain and backend ontologies SHOULD remain independent of one another. Their composition belongs to the Simulation Profile layer. This prevents MOOSE-, COMSOL-, Ansys-, plasma-, thermal-, or other specialized concepts from leaking into the Core semantic model.

## 4. Top-level Simulation semantics

`SimulationModel` defines **what is modeled**. `Analysis` defines **what computational question is asked**. `SimulationTask` is the identifiable application of one Analysis to one model. `SolverConfiguration` defines **how the Analysis is solved**.

ADR-0015 makes the top-level structure explicit:

```text
Simulation
   │
   ├── has_model ─────────> exactly 1 SimulationModel
   │
   └── has_task ──────────> 1..* SimulationTask
                                  │
                                  ├── uses_model ───> exactly 1 SimulationModel
                                  └── has_analysis ─> exactly 1 Analysis
```

`has_model` and `has_task` are non-owning semantic reference/aggregation relations. A SimulationModel may exist independently and may be reused by multiple task/Simulation contexts.

For every Simulation `S`:

```text
S has_model M
S has_task T
=> T uses_model M
```

Tree formatting elsewhere in this document is descriptive grouping unless an explicit Relation or `is_a` rule is stated. Diagram indentation alone is not inheritance or ownership.

## 5. SimulationModel

```text
SimulationModel
├── PhysicsModel
├── MathematicalModel
├── ConstitutiveModel
├── SpatialModel
├── MaterialModel
├── ConditionModel
├── NumericalModel
└── ObservationModel
```

This tree presents conceptual model-side categories. It does not assert `is_a` edges unless a separate normative rule explicitly does so.

### 5.1 PhysicsModel

Represents the physical meaning of the model, independent of mathematical or software implementation.

```text
PhysicsModel
├── Phenomenon
├── Process
└── Interaction
```

### 5.2 MathematicalModel

Defines the mathematical formulation used to represent the physical model.

```text
MathematicalModel
├── Formulation
├── Equation
├── Field
├── Operator
└── MathematicalParameter
```

```text
PhysicsModel
      │ represented_by
      ▼
MathematicalModel
```

Physics and mathematics are intentionally separated because one physical phenomenon may have multiple mathematical formulations, while the same mathematical operator may be reused across different physical domains.

### 5.3 ConstitutiveModel

Defines closure relations and property models required to close the mathematical system.

```text
ConstitutiveModel
├── ClosureRelation
└── PropertyModel
```

```text
MathematicalModel
      │ closed_by
      ▼
ConstitutiveModel
```

### 5.4 MaterialModel

Represents materials, species, and material properties.

```text
MaterialModel
├── Material
├── Species
└── MaterialProperty
```

```text
ConstitutiveModel
      │ parameterized_by
      ▼
MaterialModel
```

### 5.5 SpatialModel

Represents geometry and the spatial scope on which model entities are defined or applied.

```text
SpatialModel
├── Geometry
├── Domain
├── Boundary
├── SpatialInterface
└── Scope
```

`SpatialInterface` denotes an interface between spatial regions/domains. The unqualified language term `Interface` is reserved for the reusable capability-contract construct defined by ADR-0008 and disambiguated by ADR-0014.

`Scope` is a first-class concept. It generalizes concepts such as MOOSE block/boundary identifiers, COMSOL selections, and Ansys geometry scoping or Named Selections.

### 5.6 ConditionModel

Defines conditions and external forcing.

```text
ConditionModel
├── BoundaryCondition
├── InitialCondition
├── Source
└── Load
```

### 5.7 NumericalModel

Defines how the mathematical model is converted into a numerical representation.

```text
NumericalModel
├── Discretization
├── Mesh
└── NumericalApproximation
```

```text
MathematicalModel
      │ discretized_by
      ▼
NumericalModel
```

Solver algorithms are not part of `NumericalModel`; they belong to `SolverConfiguration` in the task/analysis layer.

### 5.8 ObservationModel

Defines what quantities are observed, derived, or exported from a simulation.

```text
ObservationModel
├── Quantity
├── Probe
├── Integral
├── Dataset
└── Output
```

## 6. SimulationTask, Analysis, SolverConfiguration, and Result

### 6.1 SimulationTask

`SimulationTask` is a Core Entity Type representing the identifiable application of exactly one Analysis to exactly one SimulationModel:

```text
SimulationTask
   ├── uses_model ─────> exactly 1 SimulationModel
   ├── has_analysis ───> exactly 1 Analysis
   └── produces ───────> 0..* Result
```

A task can be valid before execution and therefore before any Result exists. Task identity remains distinct from Analysis identity so one Analysis definition can be reused by multiple tasks/models.

`Analysis` does **not** inherit from `SimulationTask`. `SolverConfiguration` does **not** inherit from `SimulationTask`.

### 6.2 Analysis

Defines the computational question being asked.

```text
Analysis
├── StationaryAnalysis
├── TransientAnalysis
├── FrequencyDomainAnalysis
├── EigenvalueAnalysis
├── ParametricAnalysis
└── OptimizationAnalysis
```

The listed Analysis specializations are explicit taxonomic `is_a` relations in the current machine-readable registry.

### 6.3 SolverConfiguration

Defines the numerical algorithms used to solve an Analysis.

```text
SolverConfiguration
├── NonlinearSolver
├── LinearSolver
├── Preconditioner
└── ConvergenceCriterion
```

```text
Analysis
    │ solved_by
    ▼
SolverConfiguration
```

Solver cardinality, default selection, and task-specific solver overrides remain separate language/schema questions.

### 6.4 Derived `analyzed_by`

`SimulationModel -> analyzed_by -> Analysis` is a derived semantic relation:

```text
M analyzed_by A
IFF
exists SimulationTask T:
  T uses_model M
  AND
  T has_analysis A
```

Task bindings are authoritative. Implementations may materialize `analyzed_by` for indexing/traversal, but a materialized edge without a supporting task is inconsistent derived data.

### 6.5 Result

`Result` is the semantic output produced by a SimulationTask and may be associated with observation/output definitions.

```text
SimulationTask
      │ produces (0..*)
      ▼
    Result
      │ observed_by
      ▼
ObservationModel
```

SOL v0.1 does not define a mandatory Result subtype taxonomy.

## 7. Core relationship graph

```text
                    PhysicsModel
                         │
                  represented_by
                         ▼
                 MathematicalModel
                         │
                      closed_by
                         ▼
                 ConstitutiveModel
                         │
                   parameterized_by
                         ▼
                   MaterialModel

                 MathematicalModel
                         │
                    defined_on
                         ▼
                    SpatialModel

                 MathematicalModel
                         │
                   discretized_by
                         ▼
                   NumericalModel

                  ConditionModel
                         │
                     applied_to
                         ▼
              Field / Equation / Scope

                      Simulation
                    ╱            ╲
             has_model          has_task
                 │                │
                 ▼                ▼
          SimulationModel   SimulationTask
                 ▲             │       │
                 │        uses_model   has_analysis
                 └─────────────┘       │
                                       ▼
                                    Analysis
                                       │
                                    solved_by
                                       ▼
                              SolverConfiguration

SimulationModel -- analyzed_by (derived) --> Analysis

SimulationTask
      │ produces
      ▼
    Result
      │ observed_by
      ▼
ObservationModel
```

## 8. Backend boundary

MOOSE, COMSOL, and Ansys concepts do not belong directly in the Core ontology.

```text
Core Ontology
      │
      ▼
Simulation IR / MappingPlan
      │
 ┌────┼─────┐
 ▼    ▼     ▼
MOOSE COMSOL ANSYS
```

Backend adapters and backend ontologies map Core semantic concepts to native software representations. A backend may serialize one task per artifact or multiple tasks/studies per backend model; that is a transformed realization and does not change Core task identity.

## 9. Ontology language versus implementation technology

The ontology language is conceptually independent of its storage and implementation technologies.

The project may use YAML for authoring, JSON-LD/RDF/OWL for graph representation, SHACL for semantic constraints, JSON Schema for structural validation, Python for a reference SDK, and TypeScript for frontend tooling. These technologies implement the ontology language; they do not define its semantics.

## 10. Design principles

1. Separate physics from mathematics.
2. Separate mathematical formulation from constitutive closure.
3. Separate model definition from computational task/application identity.
4. Separate Analysis from SolverConfiguration.
5. Separate physics from numerical discretization.
6. Treat spatial scope as a first-class semantic concept.
7. Keep software-specific objects outside the Core ontology.
8. Keep domain-specific concepts outside the Core ontology unless genuinely universal.
9. Model semantic relations explicitly rather than relying only on a class hierarchy.
10. Allow one SimulationModel and one Analysis definition to be reused by multiple SimulationTasks.
11. Treat MOOSE, COMSOL, Ansys, and future tools as backend implementations.
12. Keep domain and backend extension axes orthogonal and compose them through Simulation Profiles.
13. Enforce inward-only dependencies: specialized layers depend on the Core, never the reverse.
14. Keep ontology semantics independent of implementation technologies.
15. Do not infer `is_a` inheritance from diagram indentation or machine-readable grouping shorthand.
16. Reserve `Interface` for reusable capability contracts; use `SpatialInterface` for the spatial entity concept.
17. Treat `has_model` and `has_task` as non-owning references; backend artifact lifecycle does not define Core ownership.
18. Treat task bindings as authoritative for model–Analysis application; `analyzed_by` is derived.

## 11. Consolidation state

The design-stage architecture is frozen through ADR-0015. Immediate machine-readable consolidation priorities are:

- transcribe accepted relation cardinalities and derived-relation invariants into the final structural/semantic schema;
- complete Interface serialization/validation;
- complete accepted Value/ValueDefinition/PhysicalDimension/Unit representation;
- define remaining Core relation cardinalities only where evidence requires them.

Backend installation, licensing, production Adapter implementation, and full execution V&V remain outside the design-stage architecture gate.
