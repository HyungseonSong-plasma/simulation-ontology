# Simulation Ontology

A solver-independent simulation ontology for multiphysics backends.

## Goal

The project defines a common semantic model for describing simulations independently of a specific simulation package. MOOSE, COMSOL, and Ansys are used as reference backends to validate that the Core ontology remains software-independent.

The design separates:

- **SimulationModel** — what is being modeled.
- **Analysis** — what computational question is asked.
- **SimulationTask** — the identifiable application of one Analysis to one model.
- **SolverConfiguration** — how the Analysis is solved.
- **Backend mappings** — how ontology concepts are represented in MOOSE, COMSOL, Ansys, and future simulation systems.

## Core architecture

```text
Simulation
  ├── has_model -> exactly 1 SimulationModel
  └── has_task  -> 1..* SimulationTask
                         ├── uses_model -> exactly 1 SimulationModel
                         ├── has_analysis -> exactly 1 Analysis
                         │                    └── solved_by -> SolverConfiguration
                         └── produces -> 0..* Result
```

`SimulationModel` directly groups compatible model-side components through non-owning, non-transitive `includes_component` relations governed by ADR-0016. `SimulationModel -> analyzed_by -> Analysis` is derived from task bindings rather than a second source of truth.

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

This statement describes the development process and does not imply that generated ontology definitions are authoritative or validated solely by model output. The ontology is progressively verified against independent contract review, reference models, and official simulation-framework documentation.

## Status

**Core Simulation Ontology v0.1 — design-stage architecture frozen through ADR-0017; language/schema consolidation and canonical ontology-package integration accepted through ADR-0028.**

Architecture/freeze amendments:

- [`ADR-0013`](docs/decisions/0013-sol-v0.1-design-stage-architecture-freeze.md) records the design-stage freeze.
- [`ADR-0014`](docs/decisions/0014-interface-disambiguation-and-machine-readable-taxonomy-semantics.md) separates capability `Interface` from `SpatialInterface` and makes taxonomy explicit.
- [`ADR-0015`](docs/decisions/0015-simulation-model-task-composition-semantics.md) defines explicit Simulation/Model/Task relations and task reification.
- [`ADR-0016`](docs/decisions/0016-model-component-membership-and-condition-target-semantics.md) defines direct model-component membership and repairs condition/forcing `applied_to` semantics.
- [`ADR-0017`](docs/decisions/0017-core-relation-cardinality-requiredness-baseline.md) fixes the remaining generic Core relation cardinality baseline and the Constraint-authority / relation-projection rule.

Language/schema consolidation:

- [`ADR-0018`](docs/decisions/0018-constraint-authoring-normalization-and-cardinality-schema-boundary.md) separates Constraint authoring from normalized semantic payloads and consolidates Cardinality/QRC schema boundaries.
- [`ADR-0019`](docs/decisions/0019-relation-target-type-constraint-semantics-and-schema.md) defines the focused relation-target Type Constraint payload and semantic-versus-representability evaluation-axis separation.
- [`ADR-0020`](docs/decisions/0020-dimension-constraint-and-canonical-dimensionvector.md) defines the canonical DimensionVector and Unit/metrology boundary.
- [`ADR-0021`](docs/decisions/0021-scalar-value-constraint-and-exact-decimal-normalization.md) defines scalar Value intervals/allowed sets, exact-decimal normalized numbers, Value satisfiability, and comparison-space gating.
- [`ADR-0022`](docs/decisions/0022-compatibility-constraint-and-semantic-criterion-contract.md) defines canonical semantic Compatibility criteria, typed operands, ordered/symmetric obligation identity, semantic `INDETERMINATE`, and deterministic family aggregation.
- [`ADR-0023`](docs/decisions/0023-conditional-constraint-and-predicate-evaluation-contract.md) defines Conditional activation, EvaluationReference lookup, Compare/Membership/Exists/Boolean predicates, strong-Kleene truth, and EvaluationFailure semantics.
- [`ADR-0024`](docs/decisions/0024-cross-family-validation-state-aggregation.md) clarifies cross-family `FAIL > INDETERMINATE > PASS` propagation for the common design-stage validation-state slice.
- [`ADR-0025`](docs/decisions/0025-interface-serialization-and-inherited-capability-conformance.md) defines InterfaceDefinition/InterfaceImplementation serialization, targeted reusable ConstraintApplications, mapping convergence, and inherited Interface guarantees through Entity specialization.
- [`ADR-0026`](docs/decisions/0026-value-unit-dimension-and-valuedefinition-transcription-boundary.md) fixes the graph-versus-typed-data boundary for DimensionVector, UnitReference, Value, and reified ValueDefinition.
- [`ADR-0027`](docs/decisions/0027-inline-valuedefinition-format-provider-boundary.md) defines the format-provider normalization boundary for nonliteral inline ValueDefinitions and deterministic semantic-dependency extraction.
- [`ADR-0028`](docs/decisions/0028-canonical-ontology-package-and-resource-integration.md) defines normalized ontology-package resources, namespace exports, canonical identity continuity, resolved dependencies, referential completeness, and the package/schema integration boundary.

All six Constraint families, the focused Interface slice, Value/Unit/Dimension/ValueDefinition transcription, and canonical normalized ontology-package/resource integration now have accepted design-stage contracts and independent Validation readback. Backend installation, licensing, production Adapter implementation, and full backend execution V&V are intentionally outside this design-stage closure path unless they later expose a genuine architecture counterexample.

The immediate focus is **Minimal Thermal Reference Model validation**, followed by a minimal Plasma/QRC stress model and final independent design-stage audit.
