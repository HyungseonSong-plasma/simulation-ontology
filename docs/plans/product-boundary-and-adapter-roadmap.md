# SOL Product Boundary and Adapter Roadmap

**Status:** Accepted roadmap, synchronized after GitHub Milestone adoption and M0.4 Phase 1  
**Date:** 2026-08-21  
**Scope:** Core product boundary, adapter ownership, protocol/transport sequencing, MockAdapter role, and reference-adapter roadmap

## Purpose

SOL keeps canonical simulation semantics in the Core platform while allowing real solver adapters to evolve as independent projects with separate dependencies and release cycles.

## Core repository responsibility

The `simulation-ontology` repository owns:

- Rust semantic Core;
- canonical Public Contract and schemas;
- CLI and future SDKs;
- versioned language-neutral Adapter Protocol;
- MockAdapter reference conformance behavior;
- reusable adapter conformance tooling;
- adapter authoring guidance and skeletons.

The Core repository SHALL NOT acquire solver-specific runtime dependencies solely to support a real backend adapter.

## Adapter repository boundary

Real solver adapters are separate repositories/projects, conceptually:

```text
simulation-ontology          # SOL platform
sol-adapter-moose            # first official real adapter
sol-adapter-zapdos           # later domain/reference adapter
sol-adapter-crane            # later domain/reference adapter
```

This preserves independent release cycles, prevents backend dependencies from contaminating Core, allows backend-native implementation languages, and makes compatibility a contract property rather than repository coupling.

## Support tiers

### Tier A — SOL platform

Core runtime, Public Contract, Adapter Protocol, CLI/SDK surfaces, MockAdapter, conformance tooling, and authoring documentation.

### Tier B — official reference adapters

Separate repositories maintained as reference implementations of the SOL adapter ecosystem.

### Tier C — community/external adapters

External implementations tested against the published contract where possible. Core does not guarantee access to the target solver or solver-native correctness.

## Published contract baseline

M0.2 published Public Contract 0.1. M0.3 published Adapter Protocol 0.1.

The Protocol 0.1 logical operation surface is:

```text
describe_adapter -> AdapterDescription | ProtocolFailure
validate_plan    -> ValidatePlanResponse | ProtocolFailure
execute_plan     -> ExecutePlanResponse | ProtocolFailure
```

These are transport-independent logical protocol operations, not JSON-RPC method definitions.

Adapter interoperability requires both:

```text
Adapter Protocol compatibility
AND
Public Contract compatibility
```

Adapter implementation/package version implies neither.

## MappingPlan and execution boundary

Core owns:

- MappingPlan DAG/dependency semantics;
- canonical deterministic representation and comparison order.

Adapters own backend scheduling for whole-plan execution. Independent actions may be reordered or parallelized when dependency edges are preserved.

Therefore canonical deterministic order is not a mandatory physical backend total order.

`validate_plan` is advisory preflight. `execute_plan` is authoritative and rechecks the full current request/state before new side effects.

## Failure, lifecycle, and retry boundary

Protocol/bootstrap/request/operational failures are not SOL lifecycle states. Preflight and execution status do not directly become `PASS`, `FAIL`, `BLOCKED`, or `INDETERMINATE`.

`execute_plan` is non-idempotent by default. Ambiguous response loss does not imply that no backend side effect occurred and does not imply safe replay.

Transport-layer retry/reconnection policy therefore must preserve M0.3 side-effect semantics rather than redefining them.

## Backend-native data boundary

Solver-native object models, vendor API wrappers, and vendor-specific semantic identities SHALL NOT become canonical SOL semantics.

Opaque namespaced backend job/artifact references MAY appear only as provenance/evidence when semantic identity/equality does not depend on them.

## MockAdapter role

MockAdapter is the Core repository's executable reference implementation target for Adapter Protocol conformance. It is not evidence of solver-native physical correctness.

M0.4 SHALL promote MockAdapter to reference Protocol 0.1 behavior and exercise at least:

- adapter description and dual compatibility;
- target/capability evidence;
- advisory `validate_plan`;
- authoritative `execute_plan`;
- exact/degraded/unsupported realization evidence;
- dependency-preserving alternate/parallel scheduling;
- partial execution and prerequisite failure behavior;
- typed ProtocolFailure behavior;
- non-idempotent execute/replay counterexamples;
- opaque provenance without backend-native semantic leakage.

M0.4 Phase 0 (reference-conformance boundary) and Phase 1 (adapter description/dual compatibility) are complete. Phase 2 advisory `validate_plan` reference behavior is the current implementation phase.

M0.4 may build Core-local reusable test helpers as needed for MockAdapter reference conformance, but general external-adapter test-runner/tooling productization belongs to M0.6.

## Transport roadmap

The planned default local v0.x transport is JSON-RPC over process `stdio`.

```text
SOL Core / CLI
      <-> JSON-RPC over stdio
Adapter process
```

M0.5 owns JSON-RPC framing, method mapping, request IDs, subprocess lifecycle, malformed transport input, process failure propagation, reconnect behavior, and replay constraints. Transport SHALL carry Adapter Protocol 0.1 without changing its semantics.

M0.5 is planned under parent tracker #74 with Phase issues #75–#80. It becomes implementation-eligible only after M0.4 closes successfully and its completion state is synchronized.

Remote/network transports remain deferred until demonstrated requirements justify them.

## Conformance boundary

Passing SOL protocol conformance means the tested adapter satisfies the published Adapter Protocol/Public Contract behavior for the declared versions. It does not prove solver-native physical correctness.

```text
Core repository
  -> protocol/schema conformance
  -> deterministic contract behavior
  -> MockAdapter reference behavior

Adapter repository
  -> backend-native mapping correctness
  -> solver API integration
  -> solver-specific regression tests
  -> physical/numerical validation where applicable
```

M0.6 is planned under parent tracker #81 with Phase issues #82–#87. It productizes reusable external-adapter invocation, positive/adversarial conformance fixture execution, authoring scaffolding, and external-project workflow only after M0.5 closes successfully.

## Milestone sequence

```text
M0.1  Semantic Core Bootstrap                  COMPLETE
  |
  v
M0.2  Canonical Public Contract 0.1            COMPLETE
  |
  v
M0.3  Adapter Protocol 0.1                     COMPLETE
  |
  v
M0.4  MockAdapter Protocol Conformance         ACTIVE (Phase 2 current)
  |
  v
M0.5  JSON-RPC / stdio Transport               PLANNED
  |
  v
M0.6  Adapter Conformance Tooling              PLANNED
  |
  +-------------------------+
  |                         |
  v                         v
SDK track               Real-adapter track
TypeScript / Python     sol-adapter-moose
                        separate repository
```

The sequencing invariants are:

1. canonical Public Contract before stable SDK ergonomics;
2. Adapter Protocol semantics before transport;
3. MockAdapter reference conformance before transport integration;
4. reusable external-adapter conformance tooling before official real-adapter development.

Experimental SDK or real-adapter spikes may occur earlier for research, but they cannot redefine or silently mutate the published Public Contract or Adapter Protocol baseline.

## GitHub Milestone projection

GitHub Milestones are the repository execution projection of these roadmap milestones; they are not version axes or acceptance authority.

Canonical progress units are Phase issues, while parent tracker issues remain normative acceptance/gate records and PRs remain implementation evidence. GitHub percentage is informational; Validator exit audit, parent completion, exact-head/main evidence, and milestone closure rules remain authoritative.

Current repository groupings are:

```text
M0.1: parent #2,  Phase #6–#16   historical complete
M0.2: parent #28, Phase #29–#35  historical complete
M0.3: parent #38, Phase #39–#44  historical complete
M0.4: parent #65, Phase #66–#71  active
M0.5: parent #74, Phase #75–#80  planned
M0.6: parent #81, Phase #82–#87  planned
```

See `docs/operations/github-milestone-convention.md` for naming, membership, progress, due-date, closure, backfill, and Operator `resume`/`update` rules.

## Reference adapter roadmap

### First — MOOSE

The first official real adapter SHALL target MOOSE in a separate repository after reusable Core-side conformance foundations are available. It should validate the complete SOL -> MappingPlan -> Adapter -> backend-artifact path and act as the first real-system feedback source for later contract hardening.

### Next — Zapdos / CRANE

Later reference adapters should stress plasma, chemistry, species/reaction, coupled-field, transport, and domain-specific mapping semantics while remaining outside the Core repository.

### Proprietary ecosystems

COMSOL and Ansys remain valid external adapter targets. Official support is not required while repeatable licensed CI environments are unavailable.

## Release independence

The following version axes remain independent:

- SOL Runtime Release;
- Public Contract;
- Adapter Protocol;
- ontology package versions;
- adapter implementation versions.

Compatibility is established through explicit contract/version/capability evidence, not matching package numbers.

## Change control

This document is a product/roadmap decision, not a semantic ADR. Changes to Core semantics, Public Contract invariants, or Adapter Protocol invariants require the normal Manager -> Researcher -> Validator flow and an ADR when architecture is fixed or changed. GitHub Milestone operating-convention changes that alter roadmap/acceptance semantics require Manager `meeting` before adoption.
