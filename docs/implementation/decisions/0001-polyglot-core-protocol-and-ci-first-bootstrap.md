# IDR-0001 — Polyglot Core, Protocol Boundary, and CI-First Bootstrap

**Status:** Accepted  
**Date:** 2026-08-20  
**Type:** Post-design implementation decision  
**Semantic baseline:** ADR-0030 remains closed and authoritative

## Decision

SOL v0.1 implementation adopts a polyglot architecture:

- **Rust** — language-independent semantic Core and canonical planning/validation engine;
- **TypeScript/React** — GUI and primary client layer;
- **MOOSE Adapter** — Python/C++ native ecosystem;
- **COMSOL Adapter** — Java API ecosystem;
- **Ansys Adapter** — Python/PyAnsys ecosystem.

Core and Adapter implementations SHALL NOT be permanently coupled through direct language FFI. The target boundary is serialized request/response messages over a future JSON-RPC/IPC transport.

During bootstrap, a Rust MockAdapter MAY execute in process, but Core/Adapter interaction must use protocol-shaped serializable DTOs and MUST NOT expose backend-native runtime handles in Core semantic types.

## CI-first rule

Rust implementation begins with GitHub Actions running:

```text
cargo build --workspace --all-targets
cargo test --workspace --all-targets
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
architecture/reference counterexample tests
```

Real backend software is not required for the initial CI baseline.

## Ordered implementation sequence

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
→ CLI
→ TypeScript protocol/client binding
```

The detailed gates are defined in `../sol-v0.1-core-implementation-plan-v0.1.md`.

## First vertical slice

The accepted Thermal reference model is the first end-to-end implementation slice.

Validation Lab positive/negative counterexamples are migrated into deterministic unit, golden/contract, and later property tests.

## Initial milestone gate

The implementation bootstrap is considered stable only when:

1. Rust Core builds in CI;
2. architecture counterexample tests execute in CI;
3. `sol-cli` emits deterministic canonical validation and MappingPlan output for the Thermal slice through MockAdapter.

Before this gate is stable, the project SHALL NOT begin:

- production MOOSE/COMSOL/Ansys adapter implementation;
- TypeScript Node/native FFI binding;
- GUI-owned semantic validation.

## Rationale

This structure preserves the backend-independent SOL semantic source of truth while allowing each vendor integration to use its strongest supported native automation environment. A protocol boundary prevents backend lifecycle and language/runtime concerns from becoming Core semantic dependencies.

CI-first bootstrap reduces local Rust tooling burden and turns accepted architecture counterexamples into executable implementation contracts immediately.

## Official ecosystem sanity references

- MOOSE application development and Python tooling: https://mooseframework.inl.gov/application_development/
- COMSOL API for Java: https://www.comsol.com/support/learning-center/article/overview-of-the-comsol-api-107912
- Ansys PyAnsys ecosystem: https://docs.pyansys.com/

## Non-goal

This IDR does not reopen or modify any SOL semantic contract accepted by ADR-0001..ADR-0030.
