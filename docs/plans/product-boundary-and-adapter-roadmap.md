# SOL Product Boundary and Adapter Roadmap

**Status:** Planning decision  
**Date:** 2026-08-21  
**Scope:** Public product boundary, adapter ownership, protocol boundary, MockAdapter role, and support roadmap

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
- versioned adapter protocol contract
- MockAdapter reference conformance implementation
- adapter conformance tests/tooling
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

## 6. Adapter Protocol scope

The SOL Adapter Protocol is an official, language-neutral public contract owned by the Core repository. It defines what an adapter must expose to SOL, not how a backend must be implemented internally.

### 6.1 Normative protocol responsibilities

The protocol SHALL define versioned canonical request/response contracts for at least:

- adapter identity and protocol compatibility description;
- BackendTarget requirement resolution;
- capability discovery;
- realization normalization;
- RealizationEffect comparison;
- MappingPlan validation at the adapter boundary;
- PlanAction execution;
- realization/execution reporting.

A conceptual minimal method surface is:

```text
describe_adapter
resolve_target
discover_capabilities
normalize_realization
compare_effects
validate_plan
execute_plan
report_realization
```

Exact method names and payload schemas remain subject to the canonical public-contract/schema work. Solver-native object models, vendor API wrappers, and vendor-specific implementation details SHALL NOT become part of the Core protocol merely because one adapter requires them.

### 6.2 Language-neutral contract

Protocol payloads SHALL use the canonical SOL public representation. JSON is the baseline wire representation for v0.x so that Rust, Python, Java, C++, and other adapter implementations can interoperate without a language-specific FFI dependency.

Adapter-specific capability vocabularies MAY be owned by the adapter, provided their identity, version/provenance, and interpretation are exposed through the published protocol contract where required by SOL validation.

### 6.3 Default v0.x transport

The default local transport for v0.x SHALL be JSON-RPC over process standard input/output (`stdio`).

```text
SOL Core / CLI
      <-> JSON-RPC over stdio
Adapter process
```

This is the baseline because it:

- is language neutral;
- isolates solver/vendor runtime dependencies from the Core process;
- is straightforward to mock and test;
- supports Python, Java, C++, Rust, and other adapter implementation languages;
- avoids requiring a network service for local simulation workflows.

Remote/network transports such as gRPC are explicitly deferred until a demonstrated use case requires them. A future transport SHALL preserve the same semantic adapter contract rather than redefine adapter semantics around the transport.

## 7. MockAdapter role

MockAdapter is not merely a test double. It SHALL serve as the Core repository's reference conformance implementation of the Adapter Protocol.

It does not claim to validate solver-native behavior. Its purpose is to provide an executable specification for the protocol and deterministic Core/adapter interaction semantics without requiring a real solver installation.

MockAdapter SHOULD exercise at least:

- adapter description/protocol negotiation;
- BackendTarget resolution;
- capability discovery;
- realization normalization;
- RealizationEffect comparison;
- PlanAction/MappingPlan validation;
- deterministic PASS / FAIL / BLOCKED / INDETERMINATE behavior;
- exact / transformed / lossy / unsupported representability scenarios;
- resource alias/collision cases;
- duplicate producer detection;
- unresolved prerequisite behavior;
- dependency cycle detection;
- state-dependent idempotency cases;
- execution-protocol behavior without a solver;
- realization reporting.

The Core conformance suite SHALL be runnable against MockAdapter and SHOULD be reusable by external adapter projects. A future CLI workflow MAY expose this as a command such as:

```text
sol adapter test <adapter-command>
```

The exact CLI syntax is non-normative at this planning stage.

## 8. Adapter conformance boundary

Passing Core conformance tests means that an adapter satisfies the published SOL Adapter Protocol for the tested protocol version. It SHALL NOT be interpreted as proof that the adapter's solver-native mapping is physically correct or that the target solver itself has been validated.

Therefore validation responsibility is separated as follows:

```text
Core repository
  -> protocol/schema conformance
  -> deterministic contract behavior
  -> MockAdapter reference behavior

Adapter repository
  -> backend-native realization correctness
  -> solver API integration
  -> solver-specific regression tests
  -> physical/numerical validation where applicable
```

This distinction is particularly important for proprietary adapters that the Core maintainers cannot execute because of licensing or installation constraints.

## 9. Protocol roadmap

### v0.1

- canonical JSON protocol payloads;
- JSON-RPC over stdio as the default transport;
- MockAdapter reference conformance implementation;
- reusable adapter conformance tests;
- MOOSE adapter developed in its external reference repository.

### v0.2

- harden protocol contracts using evidence from MOOSE and initial Zapdos/CRANE work;
- expand adapter authoring documentation and reference skeletons;
- strengthen compatibility/conformance diagnostics without introducing solver-native concepts into Core.

### v0.3+

- validate the protocol against additional open-source adapters;
- consider remote transport only if real deployment requirements justify it;
- preserve transport-independent semantic contracts.

## 10. Release independence

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

## 11. Current decision summary

The current canonical product boundary is:

```text
SOL Core repository
  = platform + SDKs + CLI
  + language-neutral Adapter Protocol
  + MockAdapter reference conformance implementation
  + conformance tooling

Adapter contract
  = canonical JSON payloads
  + JSON-RPC over stdio as the v0.x default local transport

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

## 12. Adjusted post-M0.1 execution sequence

M0.1 stabilizes the semantic Core but does not by itself stabilize the public wire/API contract or the independently versioned adapter protocol. Post-M0.1 implementation SHALL therefore proceed in the following order:

```text
M0.2  Canonical Public Contract 0.1
  -> define canonical public DTO boundaries and JSON shapes
  -> version the public contract independently from the runtime
  -> establish compatibility fixtures and normalization rules

M0.3  Adapter Protocol 0.1
  -> define versioned request/response/error contracts
  -> define protocol compatibility declaration and negotiation
  -> freeze the initial normative adapter method surface

M0.4  MockAdapter Protocol Conformance
  -> promote MockAdapter to executable protocol reference behavior
  -> exercise exact/transformed/lossy/unsupported and lifecycle counterexamples
  -> create reusable protocol conformance fixtures

M0.5  JSON-RPC over stdio Transport
  -> transport the already-defined protocol without redefining semantics
  -> verify in-process and subprocess MockAdapter parity
  -> validate framing, malformed input, process failure, and error propagation

M0.6  Adapter Conformance Tooling
  -> reusable adapter test runner
  -> adapter authoring skeleton and guide
  -> external-project conformance workflow
```

Only after M0.6 should product-development work split into parallel tracks:

```text
                     M0.6
                      |
          +-----------+-----------+
          |                       |
          v                       v
      SDK track              Adapter track
      TypeScript             sol-adapter-moose
      Python                 separate repository
          |                       |
          +-------- feedback -----+
                      |
          Public Contract / Protocol hardening
                      |
                 Zapdos / CRANE
```

This ordering preserves three product-boundary invariants:

1. the canonical semantic/public contract is defined before language-specific SDK ergonomics;
2. the Adapter Protocol is defined before its JSON-RPC transport;
3. the real MOOSE adapter begins only after reusable Core-side protocol conformance tooling exists.

TypeScript and Python SDK work MAY begin experimentally earlier for ergonomics research, but SHALL NOT define or freeze canonical semantics ahead of the Public Contract. The MOOSE adapter SHALL remain a separate repository and SHALL act as the first real-system feedback source for protocol hardening rather than as a dependency of the Core workspace.

### Validator acceptance criteria for this sequence

The post-M0.1 sequence is considered valid only while all of the following remain true:

- Public Contract and Adapter Protocol remain independently versioned surfaces.
- Transport implementation does not redefine adapter semantics.
- MockAdapter remains the Core reference conformance implementation even after real adapters exist.
- Real solver dependencies remain outside the Core repository.
- SDKs preserve canonical SOL semantics rather than exposing incidental Rust crate structure.
- Real-adapter feedback may harden a later protocol version, but SHALL NOT silently redefine an already-declared protocol version.
