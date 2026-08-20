# Core Simulation Ontology Architecture

**Version:** 0.1  
**Status:** Design baseline

## 1. Purpose

The Core Simulation Ontology provides a solver-independent semantic representation of simulation models. Software-specific concepts such as a MOOSE `Kernel`, COMSOL `Physics Feature`, or Ansys analysis object are backend representations rather than core ontology concepts.

The ontology should answer:

> What is modeled, how is it represented mathematically, what material and spatial context applies, how is it discretized, and what computational task is performed on it?

## 2. Top-level separation

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

## 3. SimulationModel

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

### 3.1 PhysicsModel

Represents the physical meaning of the model, independent of mathematical or software implementation.

```text
PhysicsModel
├── Phenomenon
├── Process
└── Interaction
```

Examples include thermal transport, fluid flow, electromagnetics, electron transport, species transport, and structural mechanics.

### 3.2 MathematicalModel

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

For example, electron transport may be represented by a fluid drift-diffusion formulation, a kinetic Boltzmann formulation, or a particle formulation.

### 3.3 ConstitutiveModel

Defines closure relations and property models required to close the mathematical system.

```text
ConstitutiveModel
├── ClosureRelation
└── PropertyModel
```

Examples include Fourier's law, Newtonian stress, and drift-diffusion flux relations.

```text
MathematicalModel
      │ closed_by
      ▼
ConstitutiveModel
```

### 3.4 MaterialModel

Represents materials, species, and material properties.

```text
MaterialModel
├── Material
├── Species
└── MaterialProperty
```

A constitutive model may be parameterized by material properties.

```text
ConstitutiveModel
      │ parameterized_by
      ▼
MaterialModel
```

### 3.5 SpatialModel

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

### 3.6 ConditionModel

Defines conditions and external forcing.

```text
ConditionModel
├── BoundaryCondition
├── InitialCondition
├── Source
└── Load
```

Conditions may target a field or equation and are applied on an appropriate scope.

### 3.7 NumericalModel

Defines how the mathematical model is converted into a numerical representation.

```text
NumericalModel
├── Discretization
├── Mesh
└── NumericalApproximation
```

Examples include finite element, finite volume, basis order, stabilization, and quadrature choices.

```text
MathematicalModel
      │ discretized_by
      ▼
NumericalModel
```

Solver algorithms are not part of `NumericalModel`; they belong to `SimulationTask/SolverConfiguration`.

### 3.8 ObservationModel

Defines what quantities are observed, derived, or exported from a simulation.

```text
ObservationModel
├── Quantity
├── Probe
├── Integral
├── Dataset
└── Output
```

## 4. SimulationTask

```text
SimulationTask
├── Analysis
└── SolverConfiguration
```

A task acts on an already-defined simulation model.

### 4.1 Analysis

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

The same `SimulationModel` may participate in multiple analyses.

### 4.2 SolverConfiguration

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

Analysis and solver configuration are separate: an analysis states *what computational problem is solved*; solver configuration states *how it is solved numerically*.

## 5. Core relationship graph

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

## 6. Backend boundary

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

Backend adapters map core semantic concepts to native software representations.

For example, a core `BoundaryCondition` targeting temperature may map to a MOOSE boundary-condition object, a COMSOL temperature physics feature, or an Ansys temperature boundary object.

## 7. Design principles

1. Separate physics from mathematics.
2. Separate mathematical formulation from constitutive closure.
3. Separate model definition from analysis tasks.
4. Separate analysis from solver configuration.
5. Separate physics from numerical discretization.
6. Treat spatial scope as a first-class semantic concept.
7. Keep software-specific objects outside the core ontology.
8. Model semantic relations explicitly rather than relying only on a class hierarchy.
9. Allow one simulation model to be reused by multiple simulation tasks.
10. Treat MOOSE, COMSOL, Ansys, and future tools as backend implementations.

## 8. Next design step

Version 0.1 defines the baseline taxonomy and architectural separation. The next step is to formalize the relation vocabulary — including `represented_by`, `closed_by`, `parameterized_by`, `defined_on`, `discretized_by`, `applied_to`, `analyzed_by`, `solved_by`, `produces`, and `observed_by` — with domain/range definitions, cardinalities, and validation constraints.
