# Adapter Authoring Guide — Protocol 0.1

Status: **reference authoring material for M0.6**. This guide and the accompanying Rust/stdin-stdout project are reference scaffolding, not a new normative Protocol surface.

The normative interoperability baseline remains:

- `docs/contracts/adapter-protocol-0.1.md` and its referenced contract documents;
- `schemas/adapter-protocol/0.1/`;
- Public Contract 0.1 and its schemas;
- the published versioned fixtures and counterexamples.

This guide explains one implementation pattern. It does not add operations, fields, lifecycle states, retry policy, backend identity, or transport semantics to those contracts.

## 1. Reference project structure

The checked reference is under `examples/adapter-authoring/rust-stdio/`:

```text
rust-stdio/
├── Cargo.toml
├── src/
│   ├── lib.rs      # adapter semantics: description, preflight, execution
│   └── main.rs     # JSON-RPC/stdin-stdout transport boundary
└── tests/
    └── conformance.rs
```

Keep the same responsibility split in an external adapter even if the implementation language or transport host differs:

```text
backend/native API
        ↑
backend translation layer
        ↑
Adapter Protocol operations
        ↑
transport/framing process
```

The backend/native API must not become the canonical SOL semantic model. Backend-specific object handles, job IDs, mesh handles, solver objects, and vendor API types stay below the Protocol boundary.

## 2. Description and dual compatibility

`describe_adapter` returns `AdapterDescription` or `ProtocolFailure`.

An adapter description declares:

- implementation identity (`adapter_id`, `adapter_version`);
- supported **Adapter Protocol** versions;
- supported **Public Contract** versions;
- addressed targets;
- capability declarations for those targets.

Interoperability requires both Protocol and Public Contract compatibility. Adapter package version is not a substitute for either compatibility axis.

The reference skeleton deliberately uses the published fixture target `mock` and thermal fixture capabilities so that the generic Protocol 0.1 positive suite can exercise it. A real adapter replaces those sample target/capability declarations with its own declared backend-facing capabilities; it must not rename or reinterpret published Protocol/Public Contract fields to fit a solver API.

## 3. `validate_plan`: advisory preflight

`validate_plan` is side-effect free and advisory. The request reuses the canonical Public Contract `BackendTargetDto` and `MappingPlanDto`; do not create adapter-local semantic copies.

A typical implementation checks, in order:

1. target compatibility;
2. required capabilities;
3. action support;
4. adapter-specific prerequisites that can be observed without executing the plan.

Expected negative conditions such as target mismatch, missing capability, unsupported action, rejected prerequisite, or transient unavailability are valid Protocol responses when represented by the published `ValidatePlanResponse` semantics. They are not automatically `ProtocolFailure` and they are not SOL lifecycle results.

An accepted preflight is not an authority token. Do not create validation leases, acceptance IDs, plan hashes, or durable execution authorization from preflight.

## 4. `execute_plan`: authoritative current-state check

`execute_plan` receives the full current target and plan again. A conforming adapter must re-check the current request and relevant backend state before creating new side effects.

The reference skeleton demonstrates:

- dependency-safe execution batches;
- exactly one terminal action report per plan action;
- aggregate effects equal to the normalized union of action effects;
- realization evidence using Public Contract subjects and qualities;
- opaque execution provenance separated from semantic identity.

Canonical MappingPlan order is deterministic representation, not a mandatory physical total order. A real backend may reorder or parallelize independent actions if all dependency edges and published execution invariants remain satisfied.

## 5. `ProtocolFailure` and side-effect evidence

`ProtocolFailure` is the logical Protocol failure channel. It is distinct from:

- a valid negative `ValidatePlanResponse`;
- a valid rejected/partial/unavailable `ExecutePlanResponse`;
- JSON-RPC/framing/process failure;
- SOL lifecycle classification.

Malformed operation payloads can become `protocol.invalid_request`. Operational failures use the published operational failure shape and conservative `side_effects` evidence.

For `describe_adapter` and `validate_plan`, operational failures must prove `side_effects=none`. For `execute_plan`, use `none` only when the adapter can establish that execution did not begin; otherwise preserve `may_have_occurred`.

Side-effect evidence is evidence, not retry policy. The Protocol and transport layers grant no automatic retry or replay authority. In particular, a lost `execute_plan` response is ambiguous and must not be silently replayed.

## 6. Provenance versus semantic identity

Canonical semantic subjects come from the Public Contract. Backend-native identifiers belong only in opaque provenance when they are useful as evidence.

For example, a backend job handle may be represented as a namespaced opaque reference. It must not replace a canonical entity/relation ID, participate in canonical semantic equality, or leak as a vendor-specific semantic object.

The reference skeleton uses an opaque namespaced execution reference solely to demonstrate that separation. Real adapters should choose backend-specific provenance namespaces without changing canonical SOL subject identity.

## 7. Transport integration

M0.5 maps the three logical operations to the repository's JSON-RPC/stdin-stdout transport profile. `src/main.rs` demonstrates a thin worker:

1. decode one transport request;
2. dispatch by the published transport method mapping;
3. parse method parameters into canonical Protocol DTOs;
4. invoke adapter logic;
5. return either Protocol success payload or `ProtocolFailure` in the Protocol result channel;
6. keep diagnostics on stderr and Protocol frames on stdout.

Do not move process IDs, JSON-RPC request IDs, reconnect state, stderr text, or framing data into Protocol payloads.

## 8. Local conformance check

In this repository the reference skeleton is compiled as a workspace member so normal Rust Core CI catches drift. Its integration test launches the skeleton executable as an external command and passes it to `PublishedPositiveFixtureSuite`.

Repository-local check:

```text
cargo test -p sol-adapter-authoring-skeleton --locked
```

The important boundary is the subprocess invocation: the conformance harness observes the adapter through the published transport, not by linking its implementation into the runner.

The exact conformance CLI and serialized report format remain provisional. Phase 0 intentionally did not stabilize command spelling, exit codes, or a machine-readable report schema. External-project packaging/workflow is the Phase 5 milestone gate; this guide therefore does not invent a CLI contract in advance.

## 9. CI pattern

Until the Phase 5 external-project workflow is published, an adapter project should treat the current library-level runner as the executable source of conformance semantics. A CI job should:

1. build the adapter executable;
2. obtain the matching published Protocol/Public Contract fixtures;
3. launch the adapter through the conformance harness;
4. require the expected positive and adversarial cases to establish conformance;
5. preserve harness/transport failures separately from adapter non-conformance;
6. run backend-native physics/numerics validation as a separate job owned by the adapter project.

The reference repository CI runs formatting, build, tests, Clippy, architecture counterexamples, and both JSON Schema publication gates in addition to the skeleton conformance test.

## 10. What conformance establishes

Protocol conformance establishes interoperability evidence against published Adapter Protocol/Public Contract behavior. It does not prove backend physical or numerical correctness.

Examples of evidence that belong in a real adapter repository, separately from Protocol conformance, include solver regression tests, mesh/material translation tests, conservation/error benchmarks, solver-version compatibility tests, and domain-specific V&V against analytical or trusted reference solutions.

A solver adapter may be Protocol-conformant while its backend translation or numerical model is physically wrong. Conversely, a physically validated solver integration is not SOL-interoperable unless it also satisfies the published Protocol/Public Contract boundary.

## 11. Adaptation checklist

Before turning the skeleton into a real adapter:

- replace the sample adapter identity, target, and capabilities;
- isolate vendor/solver API calls behind a backend translation layer;
- map Public Contract semantic subjects explicitly rather than exposing backend object identity;
- keep `validate_plan` advisory and side-effect free;
- re-check current state in `execute_plan`;
- preserve dependency-safe execution and aggregate-effect invariants;
- use `ProtocolFailure` only for the published failure categories;
- make execute side-effect evidence conservative;
- keep backend-native identifiers opaque and non-semantic;
- add no retry/replay authority to Protocol results;
- run Protocol conformance and backend physical/numerical V&V as separate CI evidence.

The first official real solver adapter remains a separate project/repository track; this skeleton does not introduce MOOSE, COMSOL, Ansys, or any other solver runtime into SOL Core.
