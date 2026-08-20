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
docs/                  Human-readable specifications, decisions, validation and implementation plans
docs/implementation/   Post-design implementation plans and implementation decisions
ontology/core/         Machine-readable core entities, relations, and constraints
ontology/backends/     Backend-specific mappings
schema/                 Validation schemas
examples/               Reference simulation models
tests/                  Ontology and mapping validation tests
rust/                   Rust semantic Core workspace (next implementation unit)
protocol/               Future JSON-RPC/IPC protocol contracts
```

## Development provenance

The initial architecture, ontology structure, documentation, and repository scaffold for this project were developed with **OpenAI ChatGPT using GPT-5.6 Sol**.

The project was conducted as an AI-assisted research and engineering workflow in which **direct human intervention in drafting and implementation was intentionally kept minimal**. Human involvement primarily consisted of defining the research direction, evaluating key conceptual choices, and approving major design decisions, while ChatGPT performed most of the ontology structuring, technical drafting, and initial repository construction.

This statement describes the development process and does not imply that generated ontology definitions are authoritative or validated solely by model output. The ontology is progressively verified against independent contract review, reference models, and official simulation-framework documentation.

## Status

**SOL v0.1 DESIGN-STAGE CLOSED — ADR-0030.**

**Post-design Core implementation planning has started.** The implementation baseline is Rust semantic Core + TypeScript/React client + native backend adapters, with a future JSON-RPC/IPC protocol boundary and CI-first Mock Adapter bootstrap.

The closed semantic baseline consists of Core architecture frozen through ADR-0017, focused language/schema/package/model-snapshot consolidation through ADR-0029, accepted Minimal Thermal and Plasma/QRC reference gates, and the final independent closure readback.

[`ADR-0030`](docs/decisions/0030-sol-v0.1-design-stage-closure.md) is the authoritative semantic closure decision.

The post-design implementation baseline is recorded in:

- [`Core Implementation Plan v0.1`](docs/implementation/sol-v0.1-core-implementation-plan-v0.1.md)
- [`IDR-0001`](docs/implementation/decisions/0001-polyglot-core-protocol-and-ci-first-bootstrap.md) — Rust/TypeScript/native-adapter polyglot architecture, protocol boundary, and CI-first bootstrap.

Architecture/freeze amendments:

- [`ADR-0013`](docs/decisions/0013-sol-v0.1-design-stage-architecture-freeze.md) records the design-stage freeze.
- [`ADR-0014`](docs/decisions/0014-interface-disambiguation-and-machine-readable-taxonomy-semantics.md) separates capability `Interface` from `SpatialInterface` and makes taxonomy explicit.
- [`ADR-0015`](docs/decisions/0015-simulation-model-task-composition-semantics.md) defines explicit Simulation/Model/Task relations and task reification.
- [`ADR-0016`](docs/decisions/0016-model-component-membership-and-condition-target-semantics.md) defines direct model-component membership and repairs condition/forcing `applied_to` semantics.
- [`ADR-0017`](docs/decisions/0017-core-relation-cardinality-requiredness-baseline.md) fixes the remaining generic Core relation cardinality baseline and the Constraint-authority / relation-projection rule.

Language/schema/package/model-snapshot consolidation:

- [`ADR-0018`](docs/decisions/0018-constraint-authoring-normalization-and-cardinality-schema-boundary.md) through [`ADR-0024`](docs/decisions/0024-cross-family-validation-state-aggregation.md) consolidate the six Constraint families and common validation-state behavior.
- [`ADR-0025`](docs/decisions/0025-interface-serialization-and-inherited-capability-conformance.md) defines Interface serialization and inherited conformance.
- [`ADR-0026`](docs/decisions/0026-value-unit-dimension-and-valuedefinition-transcription-boundary.md) and [`ADR-0027`](docs/decisions/0027-inline-valuedefinition-format-provider-boundary.md) fix Value/Unit/Dimension/ValueDefinition representation boundaries.
- [`ADR-0028`](docs/decisions/0028-canonical-ontology-package-and-resource-integration.md) defines normalized ontology-package/resource integration.
- [`ADR-0029`](docs/decisions/0029-resolved-model-snapshot-and-reference-validation-environment.md) defines the closed `ResolvedModelSnapshot` reference-validation boundary.
- [`ADR-0030`](docs/decisions/0030-sol-v0.1-design-stage-closure.md) closes the design stage.

## Reference gates

- **Minimal Thermal reference model — PASS.** Exercises task/model semantics, component membership, condition targeting, PropertyDefinition assignment, Value/UnitReference, Interface-targeted Dimension Constraints, and metrology boundaries.
- **Minimal Plasma/QRC reference model — PASS.** Exercises subtype-specialized Reaction/Species semantics, explicit reaction relations, Interface relation mapping, independent QRC obligations, closed-snapshot distinct-identity counting, subtype qualification, and counterexamples.

## Post-design implementation architecture

```text
TypeScript / React GUI + primary client
                |
         future JSON-RPC / IPC
                |
          Rust SOL Core
                |
         protocol-shaped DTOs
       /          |          \
      v           v           v
 MOOSE Adapter  COMSOL Adapter  Ansys Adapter
 Python/C++       Java           Python
```

The first implementation uses a deterministic **Mock Adapter** and does not require backend installation or licenses.

Core implementation order:

```text
data model / serialization
→ canonical identity & resolver
→ Constraint / QRC engine
→ MappingRule / MappingClaim
→ RealizationEffect / comparator
→ PlanAction / MappingPlan DAG
→ PASS/FAIL/BLOCKED/INDETERMINATE lifecycle
→ Mock Adapter
→ BackendTarget resolver
→ sol-cli
→ TypeScript protocol/client binding
```

The first vertical slice is the accepted **Thermal reference model**. Validation Lab positive/negative counterexamples are migrated into executable contract/golden/property tests.

### Initial implementation milestone

The bootstrap milestone is complete only when:

1. Rust Core builds/tests/formats/lints in GitHub Actions;
2. architecture counterexample tests execute in CI;
3. `sol-cli` emits deterministic canonical validation and MappingPlan results for Thermal through MockAdapter.

Before this gate is stable, the project does **not** begin production MOOSE/COMSOL/Ansys adapters or TypeScript Node/native bindings.

## Post-design boundary

Design-stage closure does **not** mean production implementation is complete. The following remain post-design work unless they expose a genuine semantic counterexample:

- Rust semantic Core and CLI implementation;
- TypeScript/React protocol client and GUI integration;
- Profile/BackendAdapter implementation;
- MOOSE/COMSOL/Ansys executable integration and V&V;
- installation/license/runtime concerns;
- richer domain packages and application/UI authoring;
- future namespace federation or multi-model/co-simulation design.

Any change to the closed v0.1 semantic baseline must satisfy the reopen criteria in ADR-0030 through a new focused evidence/validation cycle.

## Current implementation checkpoint

```text
Current State: CORE_IMPLEMENTATION_PLANNING_ACCEPTED
Next Atomic Unit: Rust workspace + GitHub Actions CI bootstrap
Primary Vertical Slice: Thermal
Real Backend Requirement: none
Design Reopen: no
```
