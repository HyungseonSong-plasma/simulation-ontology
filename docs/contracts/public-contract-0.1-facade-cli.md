# Public Contract 0.1 — Facade and CLI Machine Boundary

**Status:** M0.2 Phase 6 accepted and merged  
**Issue:** #35  
**Date:** 2026-08-21

## Purpose

This document identifies the intentional supported access boundary for Public Contract 0.1 and distinguishes canonical machine-facing CLI output from human-readable CLI text.

## Rust facade boundary

The crate root:

```text
sol_public_contract
```

is the intentional Rust facade for Public Contract 0.1 DTOs and canonical JSON behavior. Consumers that need the canonical contract do not need to import `sol-core-*`, `sol-mock-adapter`, `sol-target-resolver`, or `sol-cli` implementation crates.

The facade exposes the adopted Public Contract DTOs, including:

```text
SimulationDto
ValidationReport / Diagnostic
MappingClaimsDto
MappingPlanDto / PlanActionDto
BackendTargetDto
RealizationEffectDocumentDto
EvaluationResultDto
CanonicalDocument / ContractVersion
```

This Rust facade is a supported implementation access point, but Rust type names and ownership/layout do not replace the language-neutral contract. The normative cross-language semantic representation remains the versioned Public Contract JSON rules, checked-in schemas, and fixtures.

Future TypeScript/Python SDKs must consume or reproduce those canonical semantics without depending on internal Rust ownership, resolver graph storage, adapter classes, or crate decomposition.

## CLI human-readable mode

The existing commands remain human-readable operational output:

```text
sol-cli validate <simulation.json>
sol-cli plan <simulation.json> --target mock
```

Their text output is intentionally not declared to be a Public Contract JSON document. Human-readable formatting may evolve independently unless a separate CLI compatibility promise is adopted.

## CLI machine-facing Public Contract mode

The `--json` form is the M0.2 machine-facing contract path:

```text
sol-cli validate <public-contract-simulation.json> --json
sol-cli plan <public-contract-simulation.json> --target mock --json
```

For Public Contract 0.1:

- `validate ... --json` requires a Public Contract 0.1 `SimulationDto` input and emits canonical `ValidationReport` JSON.
- `plan ... --target mock --json` requires a Public Contract 0.1 `SimulationDto` input, performs the current Thermal target compatibility check, and emits canonical solver-independent `MappingPlanDto` JSON.
- adapter/native identity is not inserted into the canonical `MappingPlanDto` merely because target resolution was performed.
- output is generated through Public Contract DTO canonical serialization rather than Rust `Debug`/`Display` formatting.

The `--json` path does not define Adapter Protocol operations or transport envelopes. It is a CLI consumer/producer of Public Contract payloads only.

## Thermal end-to-end evidence

Binary integration tests execute the actual `sol-cli` binary against:

```text
fixtures/public-contract/0.1/thermal-simulation.json
```

and assert exact canonical equivalence with:

```text
fixtures/public-contract/0.1/thermal-validation-report.json
fixtures/public-contract/0.1/thermal-mapping-plan.json
```

The same test suite keeps the original human-readable output distinct, preventing accidental replacement of one surface by the other.

## SDK-consumption evidence

`crates/sol-public-contract/tests/public_facade.rs` behaves like a minimal SDK/client consumer: every supported top-level Public Contract fixture is parsed using imports from the single `sol_public_contract` crate root. The same test verifies that the facade manifest has no dependency on internal Core, MockAdapter, target-resolver, or CLI crates.

The checked-in JSON Schemas under `schemas/public-contract/0.1/` remain the language-neutral artifact suitable for non-Rust consumers.

## Adapter Protocol leakage boundary

M0.2 Public Contract schemas must not define M0.3 operations or transport fields. The facade regression tests reject operation/transport markers such as:

```text
describe_adapter
validate_plan
execute_plan
jsonrpc
stdio
```

inside Public Contract schema artifacts.

This check is intentionally scoped to the canonical schema payload surface; documentation may mention future protocol concepts when explaining exclusions.

## Milestone completion

Phase 6 completed through PR #54. The final exact PR head `ccc306e933382291c3012e6ca58cac37a66c7b0e` passed Rust Core CI #320 (`32490108445`) across formatting, build, tests, Clippy, architecture counterexamples, and Public Contract JSON Schema validation.

The M0.2 Validator exit audit verdict was **APPROVE**. PR #54 then merged to `main` as `44e458cb530ea214227661a0f1589115ccb9d5f1`.

M0.2 is therefore closed. Public Contract 0.1 is the accepted canonical external semantic payload basis for M0.3 Adapter Protocol 0.1. Adapter Protocol operation semantics and transport remain separate future layers and are not retroactively added to this contract.
