# SOL Product Boundary and Adapter Roadmap

**Status:** Planning decision  
**Date:** 2026-08-21  
**Scope:** Public product boundary, adapter ownership, and support roadmap

## Purpose

This document records the product-boundary decision for SOL after the Core architecture phase. The goal is to keep the `simulation-ontology` repository focused on the language-neutral SOL platform while allowing real solver adapters to evolve as independent projects with separate release cycles and dependencies.

## 1. Core repository responsibility

The `simulation-ontology` repository owns the SOL platform itself.

### Official platform scope

- Rust semantic Core
- canonical public API contract
- CLI
- TypeScript SDK/binding
- Python SDK/binding
- schema and ontology runtime support
- adapter protocol contract
- MockAdapter
- adapter conformance tests
- adapter authoring guide and reference skeleton

The Core repository SHALL NOT acquire solver-specific runtime dependencies solely to support a real backend adapter.

## 2. Adapter project boundary

Real adapters are separate repositories/projects.

Planned official/reference repositories are conceptually:

```text
simulation-ontology          # SOL Core platform
sol-adapter-moose            # official reference adapter
sol-adapter-zapdos           # official domain/reference adapter
sol-adapter-crane            # official domain/reference adapter
```

This separation is intentional so that:

- Core and adapters have independent release cycles;
- solver-specific dependencies do not contaminate the Core workspace;
- an adapter can use the native language/ecosystem best suited to its backend;
- adapter compatibility can be expressed through the SOL adapter protocol rather than repository coupling;
- additional adapters can be developed externally without requiring changes to the Core repository.

## 3. Support tiers

SOL distinguishes three support tiers.

### Tier A — Official SOL Platform

Owned and maintained in the `simulation-ontology` repository:

- Core runtime and validation
- public SDKs and CLI
- adapter protocol
- MockAdapter
- conformance tooling
- adapter development documentation

### Tier B — Official Reference Adapters

Maintained as separate repositories but treated as reference implementations of the SOL adapter ecosystem.

They exist both to support useful backends and to validate SOL against real simulation systems.

### Tier C — Community / External Adapters

Adapters developed by users, institutions, or companies against the published SOL adapter contract.

The Core project provides protocol, conformance tests, skeletons, and authoring guidance, but does not guarantee access to the target solver or perform official integration testing unless the adapter is promoted to an official reference adapter.

## 4. Reference adapter roadmap

### v0.1 — MOOSE real adapter

The first official real adapter SHALL target MOOSE and live in a separate repository.

Primary purposes:

- validate the complete SOL -> MappingPlan -> Adapter -> backend artifact path;
- exercise target resolution, capability declaration, effect normalization, plan execution, and realization reporting against a real open-source framework;
- provide an executable reference implementation for future adapter authors;
- validate baseline thermal and multiphysics mappings in an environment suitable for automated CI.

The Core repository continues to use MockAdapter for its own unit/contract tests even after the MOOSE adapter exists.

### v0.2 — Zapdos and CRANE

The next reference-adapter milestone SHALL extend validation into plasma and chemistry domains using Zapdos and CRANE as separate adapter/domain projects.

Primary purposes:

- species and reaction semantics;
- electron/heavy-species transport;
- coupled field mappings;
- plasma boundary conditions;
- chemistry and reaction-network mapping;
- complex constraint and MappingPlan stress testing.

Zapdos/CRANE integrations SHALL not be embedded into the Core repository.

### v0.3+ — Additional open-source backends

Later milestones SHOULD add selected open-source backends as independent adapter projects.

Selection criteria SHOULD include:

- open-source availability;
- CI-installable/headless execution where practical;
- stable API or input representation;
- a modeling/object paradigm materially different from MOOSE;
- ability to stress semantic areas not already covered by MOOSE/Zapdos/CRANE;
- sustainable maintenance cost.

Specific v0.3 targets are intentionally deferred until there is implementation evidence from v0.1 and v0.2.

## 5. Proprietary backends: COMSOL and Ansys

COMSOL and Ansys are valid target ecosystems but SHALL NOT be required official real adapters in the Core/reference roadmap because licensing, installation, and automated integration testing may not be available to the project.

The SOL project SHOULD instead provide enough infrastructure for licensed developers to build and validate these adapters independently:

```text
SOL provides
├── Adapter Protocol
├── BackendTarget contract
├── MockAdapter/reference patterns
├── Adapter skeleton
├── Authoring guide
└── Conformance test suite

External developer provides
├── solver license
├── solver installation
├── adapter implementation
├── solver-specific integration tests
└── adapter distribution/maintenance
```

A COMSOL or Ansys adapter MAY later be promoted to an official/reference adapter if maintainers have stable access to the required licensed environment and can support repeatable integration validation.

## 6. MockAdapter role

MockAdapter is not a substitute for every real-backend validation, but it is an official Core test asset and reference conformance implementation.

It SHOULD exercise at least:

- BackendTarget resolution;
- capability discovery;
- realization normalization;
- RealizationEffect comparison;
- PlanAction/MappingPlan generation and validation;
- deterministic PASS / FAIL / BLOCKED / INDETERMINATE behavior;
- execution-protocol behavior without a solver;
- realization reporting.

This keeps the Core test suite deterministic and free from external solver installations while real adapters validate backend-specific behavior in their own projects.

## 7. Release independence

Core and adapters SHALL be versioned independently.

Example:

```text
SOL Core            1.2
MOOSE Adapter       0.8
Zapdos Adapter      0.4
CRANE Adapter       0.3
Community COMSOL    0.2
```

Compatibility SHALL be expressed through published adapter-protocol and BackendTarget compatibility contracts rather than assuming matching package versions.

## 8. Current decision summary

The current canonical product boundary is:

```text
SOL Core repository
  = platform + SDKs + CLI + protocol + MockAdapter + conformance tooling

Real adapters
  = separate repositories/projects

Reference roadmap
  v0.1 -> MOOSE
  v0.2 -> Zapdos / CRANE
  v0.3+ -> selected open-source backends

COMSOL / Ansys
  = externally implementable target ecosystems;
    official implementation is not required while repeatable licensed testing is unavailable
```

This document is a roadmap/product-boundary decision, not an architecture ADR. If future work changes Core semantics or adapter protocol invariants, those changes should be handled through the appropriate ADR process.
