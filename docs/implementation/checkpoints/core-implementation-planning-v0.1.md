# Core Implementation Planning Checkpoint v0.1

**Date:** 2026-08-20  
**Operating state:** post-design implementation  
**Semantic baseline:** ADR-0030 closed

## Current State

`CORE_IMPLEMENTATION_PLANNING_ACCEPTED`

## Completed artifacts

- `docs/implementation/sol-v0.1-core-implementation-plan-v0.1.md`
- `docs/implementation/decisions/0001-polyglot-core-protocol-and-ci-first-bootstrap.md`
- `docs/implementation/README.md`
- top-level README and docs navigation updated for post-design implementation state

## Accepted implementation strategy

```text
Rust semantic Core
+ TypeScript/React primary client
+ native backend adapters
  - MOOSE: Python/C++
  - COMSOL: Java
  - Ansys: Python
+ future JSON-RPC/IPC Core/Adapter boundary
+ Mock Adapter before real backend installation
+ CI-first Rust development
```

## Ordered implementation sequence

```text
data model/serialization
→ canonical identity & resolver
→ Constraint/QRC engine
→ MappingRule/MappingClaim
→ RealizationEffect/comparator
→ PlanAction/MappingPlan DAG
→ lifecycle states
→ Mock Adapter
→ BackendTarget resolver
→ CLI
→ TypeScript protocol/client binding
```

## First vertical slice

Accepted Thermal reference model.

Validation Lab positive and negative fixtures become executable Rust contract/golden tests; property tests are added after deterministic unit behavior exists.

## Initial milestone

1. GitHub Actions builds/tests/formats/lints Rust workspace.
2. Architecture counterexample tests run in CI.
3. `sol-cli` emits deterministic canonical Thermal validation and MappingPlan JSON via MockAdapter.

## Explicit deferrals

Until the milestone is stable:

- no production MOOSE adapter;
- no production COMSOL adapter;
- no production Ansys adapter;
- no backend installation/license CI requirement;
- no TypeScript Node/native FFI binding;
- no GUI-owned semantic engine.

## Verdict

`ACCEPT`

No semantic design reopen is required.

## Next State

`CORE_IMPLEMENTATION_BOOTSTRAP`

## Next Atomic Unit

```text
Create rust/ Cargo workspace
→ sol-core
→ sol-mock-adapter
→ sol-cli
→ GitHub Actions build/test/fmt/clippy
→ fixture loader smoke using accepted Thermal JSON
```
