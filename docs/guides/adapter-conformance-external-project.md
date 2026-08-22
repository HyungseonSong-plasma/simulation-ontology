# External-project Adapter Conformance Workflow — M0.6

Status: **M0.6 reference workflow**. The executable example is `examples/external-project-conformance/`.

This workflow demonstrates that a project outside the SOL Core Cargo workspace can consume the conformance model, published fixture runner, and external-process harness while treating the adapter itself as an executable boundary.

It does **not** stabilize a public conformance CLI, exit-code contract, package-distribution contract, or serialized report schema. Those remain subject to an explicit public-interface review under the accepted Phase 0 boundary.

## Boundary

The conformance consumer depends on the conformance libraries only. It does not link:

- the reference MockAdapter implementation;
- the adapter-authoring skeleton implementation;
- MOOSE, COMSOL, Ansys, or another solver runtime.

Adapter programs are passed as executable paths and are observed through the published JSON-RPC/stdin-stdout transport.

This separation models the intended ownership boundary:

```text
external adapter project / backend dependencies
                 |
                 | executable transport boundary
                 v
        conformance consumer
                 |
                 v
published Protocol/Public Contract fixtures + runner
```

## Published fixture reuse

The CI reference copies `fixtures/adapter-protocol/0.1` to the runner temporary directory before invoking the standalone consumer. The consumer therefore receives an explicit fixture directory path and does not require fixtures to remain under a Core-repository-relative path at runtime.

The copied JSON remains the published versioned fixture material. The consumer does not rewrite it into a second semantic representation.

## Executed evidence

The standalone workflow produces four observations:

1. `positive` — the Phase 4 authoring skeleton must establish `Conformant` against the positive Protocol/Public Contract baseline;
2. `adversarial_valid_negative` — expected target/capability/preflight/execution/ProtocolFailure negative behavior must still establish `Conformant`;
3. `semantic_violation_detection` — parsable dependency/scheduling, aggregate-effect, and provenance-identity violations must be detected as `NonConformant`;
4. `execute_response_loss` — ambiguous execute response loss must remain `NotEstablished` through harness/transport evidence rather than becoming adapter non-conformance or replay authority.

The adversarial reference profile arguments are repository test controls. They are not Adapter Protocol fields, semantic identities, or stable external CLI names.

## Local usage in this repository

First build the adapter/probe executables:

```text
cargo build --workspace --all-targets --locked
```

Then set the executable and fixture paths and run the standalone package. The exact environment-variable names below belong to this reference example and are not a stabilized public runner interface.

```text
SOL_CONFORMANCE_FIXTURES="$PWD/fixtures/adapter-protocol/0.1" \
SOL_REFERENCE_ADAPTER="$PWD/target/debug/sol-adapter-authoring-skeleton" \
SOL_MOCK_ADAPTER="$PWD/target/debug/sol-mock-adapter-stdio" \
SOL_TRANSPORT_PROBE="$PWD/target/debug/sol-transport-error-probe" \
SOL_SEMANTIC_ADVERSARY="$PWD/target/debug/sol-conformance-adversary" \
cargo test --manifest-path examples/external-project-conformance/Cargo.toml --locked
```

The package is its own Cargo workspace. Running it this way therefore verifies that the conformance consumer is not implicitly relying on root-workspace membership.

## CI-oriented machine-readable evidence

The example binary can emit a JSON observation containing the deterministic Phase 0 case IDs/results. The observation contains:

```text
"stability": "provisional_not_a_public_cli_or_report_contract"
```

This output exists only to prove that deterministic machine-readable CI evidence can be derived without changing Protocol/Public Contract semantics. M0.6 does not promise that this exact JSON field layout, binary invocation, environment-variable naming, or process exit behavior is a stable consumer contract.

Rust Core CI runs the standalone workflow after the root workspace has built the reference executables. It then:

- copies the published fixtures to a temporary path;
- checks formatting for the standalone workspace;
- runs its locked tests;
- generates the provisional JSON observation twice;
- requires byte-for-byte equality;
- parses and validates the milestone-local expected determinations.

## Physical and numerical validation

A `Conformant` result means the adapter behavior observed at the published Protocol/Public Contract boundary conforms to the tested interoperability requirements. It does not establish solver-native physical or numerical correctness.

A real adapter repository must separately own backend-facing evidence such as translation tests, solver-version compatibility, conservation/error benchmarks, mesh/material checks, and domain-specific V&V. Those results must not be collapsed into the conformance determination or SOL lifecycle classification.

## Real-adapter handoff

Completing M0.6 makes a real-adapter repository track implementation-eligible; it does not place a real MOOSE/COMSOL/Ansys adapter inside SOL Core. The first real adapter remains a separate repository/project using this contract and conformance boundary.
