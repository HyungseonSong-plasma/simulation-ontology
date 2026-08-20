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

## Development provenance

The initial architecture, ontology structure, documentation, and repository scaffold for this project were developed with **OpenAI ChatGPT using GPT-5.6 Sol**.

The project was conducted as an AI-assisted research and engineering workflow in which **direct human intervention in drafting and implementation was intentionally kept minimal**. Human involvement primarily consisted of defining the research direction, evaluating key conceptual choices, and approving major design decisions, while ChatGPT performed most of the ontology structuring, technical drafting, and initial repository construction.

This statement describes the development process and does not imply that generated ontology definitions are authoritative or validated solely by model output. The ontology is intended to be progressively verified against simulation frameworks and reference implementations.

## Status

**Core Simulation Ontology v0.1 — design-stage architecture frozen through ADR-0012.**

The freeze is recorded in [`ADR-0013`](docs/decisions/0013-sol-v0.1-design-stage-architecture-freeze.md). Backend installation, licensing, production Adapter implementation, and full backend execution V&V are intentionally outside this design-stage freeze.

The immediate focus is **language/schema consolidation and minimal reference-model implementation** against the frozen architecture baseline. Production BackendAdapter development should proceed in separate Adapter workstreams and return to the architecture process only when it produces a genuine architecture counterexample.
