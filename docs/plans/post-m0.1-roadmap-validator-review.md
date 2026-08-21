# Post-M0.1 Roadmap Validator Review

**Status:** Validator decision  
**Date:** 2026-08-21  
**Reviewed documents:**
- `docs/plans/product-boundary-and-adapter-roadmap.md`
- `docs/plans/versioning-and-compatibility-policy.md`

## Decision

**APPROVE WITH EXECUTION GATES**

The adjusted sequence

```text
M0.2 Canonical Public Contract 0.1
 -> M0.3 Adapter Protocol 0.1
 -> M0.4 MockAdapter Protocol Conformance
 -> M0.5 JSON-RPC over stdio
 -> M0.6 Adapter Conformance Tooling
 -> parallel SDK and external MOOSE-adapter tracks
```

is consistent with the existing product boundary and versioning/compatibility policy and is preferable to moving directly from M0.1 into transport or a TypeScript client.

## Validation dimensions

| Dimension | Result | Validator reasoning |
|---|---|---|
| Product-boundary consistency | PASS | Core remains responsible for public contracts, protocol, MockAdapter, SDKs, CLI, and conformance tooling while real solver dependencies remain outside the repository. |
| Version-axis consistency | PASS | Public Contract and Adapter Protocol are established as separate independently versioned surfaces before downstream SDK/adapter commitments. |
| Semantic/transport separation | PASS | JSON-RPC is sequenced after protocol semantics, so transport cannot accidentally define adapter meaning. |
| MockAdapter role | PASS | MockAdapter becomes executable reference conformance behavior before a real adapter is treated as authoritative feedback. |
| SDK timing | PASS WITH GATE | SDK experimentation may occur earlier, but stable SDK contracts must consume a declared Public Contract rather than internal Rust structures. |
| Real-adapter timing | PASS WITH GATE | MOOSE remains an external repository and should enter the official reference path only after Protocol 0.1 plus reusable conformance tooling exist. |
| Compatibility enforcement | PASS WITH GATE | Every milestone must promote compatibility uncertainty into executable fixtures instead of relying on inferred package-version compatibility. |
| Extensibility | PASS | The sequence supports later Zapdos/CRANE and other language adapters without changing the Core repository boundary. |
| Simplicity | PASS | Each milestone stabilizes one contract layer at a time and avoids prematurely coupling SDK, transport, and backend work. |

## Why this order is correct

### 1. Public semantics must precede SDK ergonomics

M0.1 stabilizes semantic behavior inside the Rust implementation, but internal Rust structs and crate layout are not the Public Contract. M0.2 therefore needs to identify and freeze the initial canonical public DTO boundary before TypeScript or Python APIs become supported surfaces.

### 2. Protocol semantics must precede transport

The Adapter Protocol defines interoperability meaning; JSON-RPC over stdio only transports that meaning. Implementing transport first would create a high risk that framing/envelope choices accidentally become normative protocol semantics. M0.3 before M0.5 prevents that inversion.

### 3. Mock conformance must precede real-backend authority

A real MOOSE adapter is valuable as architecture feedback, but it must not become the source of Core semantics. M0.4 and M0.6 establish an executable reference and reusable test boundary first; MOOSE can then validate and stress the published contract from a separate repository.

### 4. Compatibility must be explicit before execution

Because Runtime, Public Contract, Adapter Protocol, ontology packages, and adapter implementation versions are independent axes, M0.3 must include explicit compatibility declaration/negotiation before `execute_plan`. Matching package versions must never be treated as interoperability evidence.

## Required gates before each milestone closes

### M0.2 — Canonical Public Contract 0.1

Must provide:

- inventory/classification of public vs internal vs Experimental representations;
- explicit `Public Contract 0.1` version identity;
- canonical JSON shapes for the adopted DTO surface;
- normalization/equality rules where semantics depend on canonical form;
- unknown optional-field/extensibility behavior;
- golden and backward-compatibility fixtures.

Validator blocker: M0.3 must not freeze protocol payloads until these dependencies are explicit.

### M0.3 — Adapter Protocol 0.1

Must provide:

- versioned adapter identity/compatibility description;
- explicit supported protocol range semantics;
- canonical request/response/error payloads for the initial method surface;
- compatibility negotiation before execution;
- incompatible and missing-information counterexamples;
- rule that incompatible protocol changes require a new protocol version.

Validator blocker: M0.5 must not define transport-specific semantics that are absent from the protocol contract.

### M0.4 — MockAdapter Protocol Conformance

Must provide executable protocol-level cases for at least:

- exact, transformed, lossy, unsupported realization;
- PASS / FAIL / BLOCKED / INDETERMINATE behavior;
- alias/resource collision;
- duplicate producer;
- unresolved prerequisite;
- cycle rejection;
- state-dependent idempotency;
- realization reporting.

Validator blocker: passing MockAdapter conformance proves protocol behavior only, not solver-native physical correctness.

### M0.5 — JSON-RPC over stdio

Must demonstrate semantic parity with the in-process reference path and test malformed input, request/envelope errors, subprocess termination, stdout/stderr discipline, and deterministic error propagation.

Validator blocker: transport metadata must remain non-semantic unless explicitly promoted into a versioned protocol contract.

### M0.6 — Adapter Conformance Tooling

Must allow an external adapter project to run the Core conformance suite without importing solver dependencies into the Core workspace. The adapter authoring skeleton and guide should consume the same canonical protocol definitions and fixtures.

Validator blocker: the official MOOSE reference-adapter track should not depend on private or in-process-only Core implementation details.

## Risks to monitor

1. **Rust-structure leakage:** generated JSON or SDK APIs may mirror current Rust ownership/module layout instead of the canonical public model.
2. **Protocol/transport conflation:** JSON-RPC error codes or process behavior may accidentally redefine semantic adapter outcomes.
3. **Premature SDK freeze:** TypeScript/Python convenience APIs may become de facto contracts before Public Contract 0.1 is declared.
4. **MOOSE-driven overfitting:** first real-adapter requirements may introduce backend-native concepts into Core rather than adapter-owned capability vocabularies.
5. **Silent protocol mutation:** real-adapter feedback may tempt compatible-looking changes inside protocol 0.1 that are actually semantic breaking changes.
6. **Conformance overclaim:** protocol conformance must not be presented as physical/numerical validation of a solver mapping.

## Final validator conclusion

The adjusted roadmap is structurally sound, compatible with the two governing planning decisions, and improves sequencing by stabilizing semantic/public contracts before language SDKs, transport mechanics, and real-backend implementation.

No architecture ADR is required merely for this sequencing adjustment because it does not redefine M0.1 Core semantics. If M0.2 or later work changes canonical Core meaning, canonical identity, lifecycle semantics, MappingPlan semantics, or another architecture invariant, that semantic change must be handled through an ADR in addition to the applicable versioning/breaking-change process.
