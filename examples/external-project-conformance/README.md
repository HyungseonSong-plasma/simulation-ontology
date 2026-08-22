# Standalone external-project conformance consumer

This directory is intentionally its **own Cargo workspace**. It demonstrates how a project outside the SOL Core workspace can consume the conformance model, fixture runner, and external-process harness while invoking adapter executables only through the published transport boundary.

The example does not link `sol-mock-adapter`, the authoring skeleton implementation, MOOSE, COMSOL, Ansys, or another solver runtime. Adapter commands are provided as executable paths through environment variables.

## Inputs

The workflow expects:

- `SOL_CONFORMANCE_FIXTURES` — a directory containing the published Adapter Protocol 0.1 fixtures;
- `SOL_REFERENCE_ADAPTER` — a positive reference adapter executable;
- `SOL_MOCK_ADAPTER` — the reference profile worker used for valid-negative adversarial cases;
- `SOL_TRANSPORT_PROBE` — the transport response-loss probe;
- `SOL_SEMANTIC_ADVERSARY` — the parsable semantic-violation responder.

The reference profile arguments are test controls for this repository's evidence adapter. They are not Adapter Protocol fields or stable conformance CLI spelling.

## Local repository check

Build the repository executables first, then point the standalone consumer at them:

```text
cargo build --workspace --all-targets --locked

SOL_CONFORMANCE_FIXTURES="$PWD/fixtures/adapter-protocol/0.1" \
SOL_REFERENCE_ADAPTER="$PWD/target/debug/sol-adapter-authoring-skeleton" \
SOL_MOCK_ADAPTER="$PWD/target/debug/sol-mock-adapter-stdio" \
SOL_TRANSPORT_PROBE="$PWD/target/debug/sol-transport-error-probe" \
SOL_SEMANTIC_ADVERSARY="$PWD/target/debug/sol-conformance-adversary" \
cargo test --manifest-path examples/external-project-conformance/Cargo.toml --locked
```

Running the package binary with the same environment produces a deterministic JSON observation for CI demonstration.

That JSON is explicitly marked `provisional_not_a_public_cli_or_report_contract`. It is milestone evidence only. Exact command spelling, exit-code semantics, and a stable serialized conformance report remain subject to a future explicit public-interface review under the Phase 0 decision.

Protocol conformance remains interoperability evidence. Backend physical/numerical V&V stays in the adapter/backend project and is not represented by this observation.
