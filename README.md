# simulation-ontology

SOL is a solver-independent semantic platform for describing simulation intent, validating that intent, planning how it may be realized, and connecting the same canonical model to multiple simulation backends through versioned adapter contracts.

The project is not intended to replace MOOSE, COMSOL, Ansys, or other solvers. Its purpose is to provide a common semantic layer above them so that the meaning of a simulation model is not defined by one solver's object model, file format, or API.

## Why SOL exists

Simulation workflows usually mix several concerns that are difficult to separate once they are encoded directly in a solver-specific model:

- physical intent;
- mathematical representation;
- constitutive assumptions;
- spatial applicability;
- numerical configuration;
- backend capability and realization details.

SOL separates those concerns into canonical semantic concepts. A model can therefore be validated before a real solver is selected, compared with a backend realization without redefining the original intent, and mapped through an explicit deterministic `MappingPlan` rather than hidden solver-specific logic.

Conceptually:

```text
Canonical simulation intent
          |
          v
Validation / constraints
          |
          v
Mapping claims + MappingPlan
          |
          v
BackendTarget / Adapter Protocol
          |
     +----+----+
     |         |
     v         v
   MOOSE     other adapters
```

## What SOL is intended to enable

SOL is designed for workflows such as:

- defining and exchanging simulation models independently of a particular solver;
- checking semantic consistency and reference integrity before backend execution;
- expressing where physics, equations, materials, and conditions apply through first-class spatial scopes;
- generating deterministic solver-independent mapping plans;
- distinguishing semantic intent from backend realization quality;
- reporting `PASS`, `FAIL`, `BLOCKED`, and `INDETERMINATE` without conflating unsupported execution with failed physics;
- developing solver adapters in the language and dependency environment appropriate to each backend;
- exposing one canonical public contract to Rust, CLI, future TypeScript/Python SDKs, and external adapter implementations.

Longer term, the same contract is intended to support multiphysics and domain-specific ontology packages without making any one backend the source of semantic truth.

## Current status

**M0.1 — Semantic Core Bootstrap is complete and merged to `main`.** The current implementation includes:

- solver-independent Core entity and relation semantics;
- canonical identity and deterministic graph resolution;
- first-class `SpatialScope` distinct from `SpatialModel` and backend-native selections;
- QRC/type/relation/cardinality/reference constraints;
- `MappingRule` / `MappingClaim` semantics;
- `RealizationEffect` comparison;
- deterministic DAG-based `MappingPlan` with cycle rejection;
- evaluation lifecycle semantics;
- MockAdapter behavior and BackendTarget resolution;
- Thermal reference/counterexample fixtures;
- `sol-cli validate` and `sol-cli plan --target mock` executable golden coverage.

The active milestone is **[M0.2 — Canonical Public Contract 0.1](../../issues/28)**. M0.2 does not add a real solver adapter. It turns the stabilized M0.1 semantics into an explicit, versioned, language-neutral public JSON/API contract before protocol transport or SDK ergonomics are allowed to define external behavior.

## Post-M0.1 roadmap

The accepted execution order is:

```text
M0.1  Semantic Core Bootstrap                  complete
  |
  v
M0.2  Canonical Public Contract 0.1            current
  |
  v
M0.3  Adapter Protocol 0.1
  |
  v
M0.4  MockAdapter Protocol Conformance
  |
  v
M0.5  JSON-RPC over stdio transport
  |
  v
M0.6  Adapter Conformance Tooling
  |
  +-------------------------+
  |                         |
  v                         v
SDK track               Real-adapter track
TypeScript / Python     sol-adapter-moose
                          separate repository
```

This ordering is deliberate: the Public Contract precedes language-specific SDKs, Adapter Protocol semantics precede JSON-RPC transport, and reusable conformance tooling precedes the first real MOOSE adapter.

## Public contract and versioning

The normative external boundary is the **Canonical Public Contract**, represented through versioned canonical JSON / JSON Schema rather than internal Rust struct layout.

SOL keeps several version axes intentionally independent:

- SOL Runtime Release;
- Public Contract;
- Adapter Protocol;
- ontology package versions;
- adapter implementation versions.

Matching package versions do not imply compatibility. Adapter interoperability will be established through explicit Adapter Protocol compatibility declarations/ranges, and public-contract compatibility includes both structural and semantic behavior.

Internal Rust crate decomposition may change without being a public breaking change when the supported facade and canonical behavior remain compatible.

## Adapter boundary

This repository owns the SOL platform boundary:

- Rust semantic Core and public facade;
- CLI;
- canonical public contract and schemas;
- versioned language-neutral Adapter Protocol;
- MockAdapter reference conformance implementation;
- reusable conformance tests/tooling;
- adapter authoring guidance and reference skeletons;
- future official SDK distributions.

Real solver adapters remain separate repositories/projects so their solver-specific dependencies and release cycles do not contaminate the Core workspace.

Reference adapter direction:

```text
first  -> MOOSE
next   -> Zapdos / CRANE
later  -> selected additional open-source backends
```

COMSOL and Ansys remain valid externally implementable adapter targets, but are not required official integrations while repeatable licensed CI environments are unavailable.

For v0.x, the planned default local adapter transport is canonical JSON using JSON-RPC over process `stdio`. The transport carries the Adapter Protocol; it must not redefine protocol semantics.

## MockAdapter and validation responsibility

MockAdapter is the Core repository's reference conformance implementation, not evidence of solver-native physical correctness.

```text
Core repository
  -> semantic/public-contract validation
  -> protocol/schema conformance
  -> deterministic contract behavior
  -> MockAdapter reference behavior

Real adapter repository
  -> backend-native realization correctness
  -> solver API integration
  -> solver-specific regression tests
  -> physical/numerical validation where applicable
```

This separation allows Core CI to remain deterministic and solver-installation independent while still making external adapters testable against the same published contract.

## Development method and logical agents

Development is CI-first and counterexample-driven. The canonical logical agents are development/reasoning roles, not runtime SOL agents:

| Logical agent | Responsibility | Primary output / work mode |
|---|---|---|
| **Manager** | orchestrate decision-heavy discussions, track decision state, prevent premature convergence, coordinate user confirmation and handoff | `meeting` |
| **Planner** | objectives, sequencing, dependencies, alternatives, milestone/phase shape | plans, options, decision criteria, execution order |
| **Researcher** | ontology/architecture decisions, semantic boundaries, trade-offs | semantic decisions, ADRs, invariants |
| **Validator** | challenge decisions for connectivity, extensibility, simplicity, consistency, compatibility, and counterexamples | verdicts, counterexamples, acceptance gates |
| **Operator** | execute accepted work in GitHub and synchronize guides with accepted state | `resume`, `update` |

For architecture, Public Contract, Adapter Protocol, roadmap, or other decision-heavy work, the normal flow is:

```text
Manager: meeting
   -> Planner
   -> Researcher
   -> ADR when semantic architecture is fixed or changed
   -> Validator
   -> Planner critical revision
   -> Manager checks unresolved/rejected alternatives and decision state
   -> user confirmation when required
   -> ACCEPTED
   -> Operator: resume
   -> fixture/test -> implementation -> CI -> issue evidence
```

The named work modes are:

- `meeting` — Manager coordinates Planner, Researcher, Validator, and user confirmation until a decision is accepted, rejected, deferred, or revised;
- `resume` — Operator continues implementation from the current repository/issue/PR/CI state until a real merge, decision, permission, or architecture gate is reached;
- `update` — Operator synchronizes user/developer-facing guides with accepted current behavior, including the root README and future API, SDK, adapter, CLI, schema, examples, and onboarding documentation.

`meeting` does not give Manager semantic authority: Researcher owns semantic investigation, Validator independently challenges the proposal, Planner critically incorporates feedback, and compatibility-sensitive acceptance normally requires user confirmation.

`update` is documentation synchronization, not semantic authority. If guide maintenance exposes an unresolved architecture contradiction, Operator must surface it to Manager/Planner/Researcher instead of inventing a new contract in documentation.

Manager is normally used for architecture/public-contract/protocol/roadmap decisions or other work with multiple reasonable alternatives. Already accepted implementation tasks, CI fixes, mechanical maintenance, and routine guide synchronization do not need a meeting.

For implementation work, a failed CI gate blocks the next task. The operating loop is `push -> wait/check CI -> inspect failure -> minimal fix -> re-check -> continue only after success`.

For compatibility-sensitive milestone transitions, the preferred handoff is `implementation complete -> Validator exit audit -> Operator resume for required fixes -> milestone close -> Operator update -> Manager/Planner preparation of the next milestone`.

See [`docs/operations/logical-agent-workflow.md`](docs/operations/logical-agent-workflow.md) for the detailed role, decision-state, and work-mode conventions.

## Repository and project baselines

- Core architecture / M0.1 implementation plan: [`docs/plans/core-simulation-ontology-v0.1-implementation-plan.md`](docs/plans/core-simulation-ontology-v0.1-implementation-plan.md)
- Product boundary and adapter roadmap: [`docs/plans/product-boundary-and-adapter-roadmap.md`](docs/plans/product-boundary-and-adapter-roadmap.md)
- Versioning and compatibility policy: [`docs/plans/versioning-and-compatibility-policy.md`](docs/plans/versioning-and-compatibility-policy.md)
- Post-M0.1 validator review: [`docs/plans/post-m0.1-roadmap-validator-review.md`](docs/plans/post-m0.1-roadmap-validator-review.md)
- Logical agent workflow: [`docs/operations/logical-agent-workflow.md`](docs/operations/logical-agent-workflow.md)
- Spatial Scope ADR: [`docs/adr/ADR-001-first-class-spatial-scope.md`](docs/adr/ADR-001-first-class-spatial-scope.md)
- Implementation notes: [`docs/implementation/`](docs/implementation/)
- M0.1 completed tracker: [Issue #2](../../issues/2)
- M0.2 active tracker: [Issue #28](../../issues/28)
- M0.3 planned tracker: [Issue #38](../../issues/38)

## Planned public distributions

SOL is intended to remain language-neutral even though the semantic Core is currently implemented in Rust. Planned distribution surfaces are conceptually:

```text
Cargo   -> Rust SOL facade/Core distribution
binary  -> sol
npm     -> TypeScript SDK after Public Contract/protocol stabilization
PyPI    -> Python SDK after Public Contract/protocol stabilization
```

SDKs may be explored experimentally earlier, but stable SDK interfaces must consume and preserve the canonical SOL Public Contract rather than freeze incidental Rust internals.
