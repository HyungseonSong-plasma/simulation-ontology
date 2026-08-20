# Simulation Ontology

A solver-independent simulation ontology for multiphysics backends.

## Goal

The project defines a common semantic model for describing simulations independently of a specific simulation package. MOOSE, COMSOL, and Ansys are used as reference backends to validate that the core ontology remains software-independent.

The core design separates:

- **SimulationModel** — what is being modeled.
- **SimulationTask** — what computation is performed on that model.
- **Backend mappings** — how ontology concepts are represented in MOOSE, COMSOL, Ansys, and future simulation systems.

## Core architecture

```text
Simulation
├── SimulationModel
│   ├── PhysicsModel
│   ├── MathematicalModel
│   ├── ConstitutiveModel
│   ├── SpatialModel
│   ├── MaterialModel
│   ├── ConditionModel
│   ├── NumericalModel
│   └── ObservationModel
│
└── SimulationTask
    ├── Analysis
    └── SolverConfiguration
```

See [`docs/architecture.md`](docs/architecture.md) for the current ontology specification.

## Repository layout

```text
docs/                  Human-readable specifications and design decisions
ontology/core/         Machine-readable core entities, relations, and constraints
ontology/backends/     Backend-specific mappings
schema/                 Validation schemas
examples/               Reference simulation models
tests/                  Ontology and mapping validation tests
```

## Status

**Core Simulation Ontology v0.1 — design phase.**

The immediate focus is defining the core entity vocabulary, semantic relations, cardinalities, and constraints before implementing detailed backend mappings.
