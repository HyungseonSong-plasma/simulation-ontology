# SOL Adapter Protocol 0.1

Status: **published v0.x interoperability baseline**. Published by M0.3 Phase 5 after exact-head CI and Validator exit audit, then merged to `main` in PR #63.

Adapter Protocol 0.1 is the language-neutral, transport-independent interoperability contract between SOL Core and an adapter. It consumes Public Contract 0.1 payload semantics where those semantics overlap; it does not redefine them.

## Normative components

- `docs/adr/ADR-002-adapter-protocol-boundary.md` — ownership and architecture boundary.
- `docs/contracts/adapter-protocol-0.1-bootstrap-description.md` — version-safe bootstrap, dual compatibility, targets, and capability declaration.
- `docs/contracts/adapter-protocol-0.1-plan-preflight.md` — advisory `validate_plan` contract.
- `docs/contracts/adapter-protocol-0.1-execution-realization.md` — authoritative `execute_plan`, scheduling, realization evidence, and provenance.
- `docs/contracts/adapter-protocol-0.1-error-idempotency-state.md` — failure algebra, idempotency, state, and ambiguous side-effect evidence.
- `schemas/adapter-protocol/0.1/` — canonical structural JSON Schema publication.
- `fixtures/adapter-protocol/0.1/` and Protocol counterexamples under `fixtures/counterexamples/` — executable structural and semantic examples.

## Published logical operation surface

```text
describe_adapter -> AdapterDescription | ProtocolFailure
validate_plan    -> ValidatePlanResponse | ProtocolFailure
execute_plan     -> ExecutePlanResponse | ProtocolFailure
```

`describe_adapter`, `validate_plan`, and `execute_plan` are logical responsibilities/operations, not JSON-RPC method definitions. M0.5 owns transport mapping.

## Public Contract reuse

Protocol request/response schemas reference Public Contract 0.1 schemas for:

- `BackendTargetDto`;
- `MappingPlanDto`;
- `Diagnostic`;
- `RealizationEffectDto`.

A protocol-local duplicate representation of those semantics is not part of Protocol 0.1.

## Compatibility boundary

Interoperability requires both:

```text
Adapter Protocol compatibility
AND
Public Contract compatibility
```

Adapter package version implies neither axis. Missing compatibility evidence means interoperability is not established; it is not automatically a SOL lifecycle state.

## Execution and lifecycle boundary

- `validate_plan` is advisory and side-effect free. Equivalent canonical input plus equivalent relevant state/evidence has an idempotent normalized result.
- `execute_plan` is authoritative and non-idempotent by default. It re-checks the full current request/state before new side effects.
- Canonical MappingPlan order is deterministic representation; it is not a mandatory physical backend total order. Independent actions may be reordered or parallelized while dependency edges are preserved.
- `ProtocolFailure`, preflight state, and execution state do not directly become `PASS`, `FAIL`, `BLOCKED`, or `INDETERMINATE`.
- Only appropriate canonical semantic evidence may later participate in Core lifecycle evaluation.

## Backend-data boundary

Backend-native object models, vendor API structures, and solver-native semantic identities are forbidden from canonical Protocol payload semantics. Opaque namespaced job/artifact references may appear only as provenance/evidence, and semantic equality or identity must not depend on them.

## Transport exclusion

Protocol 0.1 does not define JSON-RPC framing or error numbers, request IDs, stdio frames, subprocess lifecycle, network behavior, reconnect/backoff timing, or automatic request replay. The ambiguous execute-response-loss counterexample constrains future transport behavior but does not itself define a transport.

## Evolution rule

Protocol 0.1 is a published v0.x interoperability baseline, not a claim of mature long-term stability. Its declared structural and normative meanings cannot be silently redefined. An incompatible structural or semantic change requires an explicit protocol-version boundary such as 0.2 and migration evidence.
