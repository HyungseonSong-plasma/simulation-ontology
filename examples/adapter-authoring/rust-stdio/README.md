# Rust stdio adapter authoring skeleton

This directory is a compile-checked reference implementation for SOL Adapter Protocol 0.1 authoring.

It is intentionally solver-independent. The sample `mock` target and thermal actions match the published Protocol 0.1 fixture baseline so the generic conformance suite can exercise the skeleton. They are not a prescribed target vocabulary for real adapters.

Start with `docs/guides/adapter-authoring-0.1.md` before adapting this project. In particular:

- keep backend/vendor API types behind a translation boundary;
- reuse published Protocol/Public Contract DTO semantics;
- keep preflight advisory and execution authoritative;
- treat ProtocolFailure, transport failure, and SOL lifecycle as distinct;
- keep backend identifiers opaque and non-semantic;
- run backend physics/numerics V&V separately from Protocol conformance.

Repository-local check:

```text
cargo test -p sol-adapter-authoring-skeleton --locked
```

The exact conformance CLI/output contract is intentionally not defined by this skeleton; the current executable evidence is the Rust conformance integration test.
