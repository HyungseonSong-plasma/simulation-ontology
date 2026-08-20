# Core Simulation Ontology Architecture

**Version:** 0.1  
**Status:** Design baseline

## 1. Purpose

The Core Simulation Ontology provides a solver-independent and domain-independent semantic framework for simulation models. Software-specific concepts such as a MOOSE `Kernel`, COMSOL `Physics Feature`, or Ansys analysis object are backend representations rather than core ontology concepts.

The ontology should answer:

> What is modeled, how is it represented mathematically, what material and spatial context applies, how is it discretized, and what computational task is performed on it?

The project is intended to act as a foundational framework rather than a complete ontology for any single physics domain or simulation package.

## 2. Framework layering

The framework uses four conceptual layers:

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

### Core Framework

`simulation-ontology` defines the solver- and domain-independent semantic primitives, relations, constraints, extension rules, and ontology language contracts.

### Domain Ontology

A domain ontology specializes the core for a physical or scientific domain without depending on a simulation backend.

Examples:

- `plasma-ontology`
- `thermal-ontology`
- `electromagnetics-ontology`
- `fluid-ontology`

A plasma ontology may define concepts such as electron transport, species, ionization, drift-diffusion flux, and Boltzmann formulations while remaining independent of MOOSE, COMSOL, or Ansys.

### Backend Ontology

A backend ontology specializes the core for the native concepts and metadata of a simulation framework without defining a particular physics domain.

Examples:

- `moose-ontology`
- `comsol-ontology`
- `ansys-ontology`

Backend ontologies may define native object categories, parameter metadata, selections/scoping, execution constructs, and mappings from core semantic concepts to software representations.

### Simulation Profile

A simulation profile composes one or more domain ontologies with a backend ontology to create an executable or generatable simulation vocabulary.

```text
plasma-ontology + moose-ontology
              │
              ▼
       MOOSE Plasma Profile
```

The profile is where a domain concept such as an electron diffusion operator may be bound to a backend realization such as a specific MOOSE object.

## 3. Dependency rule

Dependencies are strictly directed toward the core framework:

```text
Application
    ↓ depends on
Simulation Profile
    ↓
Domain Ontology + Backend Ontology
    ↓
simulation-ontology
```

The core framework MUST NOT depend on domain ontologies, backend ontologies, simulation profiles, or applications.

Domain and backend ontologies SHOULD remain independent of one another. Their composition belongs to the Simulation Profile layer.

This rule prevents MOOSE-, COMSOL-, Ansys-, plasma-, thermal-, or other specialized concepts from leaking into the core semantic model.

## 4. Top-level simulation separation

```text
Simulation
├── SimulationModel
└── SimulationTask
```

`SimulationModel` defines **what is modeled**. `SimulationTask` defines **what computation is performed on the model**.

```text
SimulationModel
      │ analyzed_by
      ▼
SimulationTask
      │ produces
      ▼
Result
```

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

Physics and mathematics are intentionally separated because one physical phenomenon may have multiple mathematical formulations, while the same mathematical operator may be reused across different physical domains.

```text
PhysicsModel
      │ represented_by
      ▼
MathematicalModel
```

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
├── Interface
└── Scope
```

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

Solver algorithms are not part of `NumericalModel`; they belong to `SimulationTask/SolverConfiguration`.

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

## 6. SimulationTask

```text
SimulationTask
├── Analysis
└── SolverConfiguration
```

### 6.1 Analysis

Defines the computational question being asked.

```text
Analysis
├── Stationary
├── Transient
├── FrequencyDomain
├── Eigenvalue
├── Parametric
└── Optimization
```

### 6.2 SolverConfiguration

Defines the numerical algorithms used to solve an analysis.

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

                  SimulationModel
                         │
                    analyzed_by
                         ▼
                      Analysis
                         │
                     solved_by
                         ▼
                SolverConfiguration
                         │
                      produces
                         ▼
                       Result
                         │
                    observed_by
                         ▼
                ObservationModel
```

## 8. Backend boundary

MOOSE, COMSOL, and Ansys concepts do not belong directly in the core ontology.

```text
Core Ontology
      │
      ▼
Simulation IR
      │
 ┌────┼─────┐
 ▼    ▼     ▼
MOOSE COMSOL ANSYS
```

Backend adapters and backend ontologies map core semantic concepts to native software representations.

## 9. Ontology language versus implementation technology

The ontology language is conceptually independent of its storage and implementation technologies.

The project may use technologies such as YAML for authoring, JSON-LD/RDF/OWL for graph representation, SHACL for semantic constraints, JSON Schema for structural validation, Python for a reference SDK, and TypeScript for frontend tooling. These technologies implement the ontology language; they do not define its semantics.

This separation allows the implementation stack to evolve without changing the conceptual ontology contract.

## 10. Design principles

1. Separate physics from mathematics.
2. Separate mathematical formulation from constitutive closure.
3. Separate model definition from analysis tasks.
4. Separate analysis from solver configuration.
5. Separate physics from numerical discretization.
6. Treat spatial scope as a first-class semantic concept.
7. Keep software-specific objects outside the core ontology.
8. Keep domain-specific concepts outside the core ontology unless they are genuinely universal simulation concepts.
9. Model semantic relations explicitly rather than relying only on a class hierarchy.
10. Allow one simulation model to be reused by multiple simulation tasks.
11. Treat MOOSE, COMSOL, Ansys, and future tools as backend implementations.
12. Keep domain and backend extension axes orthogonal and compose them through Simulation Profiles.
13. Enforce inward-only dependencies: specialized layers depend on the core, never the reverse.
14. Keep ontology semantics independent of implementation technologies.

## 11. Next design step

The next design artifact is the **Simulation Ontology Language v0.1**. It will define the minimum language constructs required to express the framework: Entity, Property, Relation, Constraint, Extension, Namespace, and Profile composition semantics. Only after this contract is stable should the project standardize the YAML authoring form and its JSON-LD/RDF/SHACL implementation pipeline.
