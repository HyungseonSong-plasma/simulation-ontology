# Adapter Protocol 0.1 — Error, Idempotency, and State Semantics

**Status:** M0.3 Phase 4 normative baseline  
**Issue:** #43  
**Date:** 2026-08-21

## Purpose

This document defines the transport-independent failure algebra and state/idempotency semantics for Adapter Protocol 0.1. It preserves the separation between protocol operation failure, valid negative operation responses, and SOL semantic lifecycle evaluation.

It does not define JSON-RPC error codes, stdin/stdout framing, process exit codes, retry timing/backoff, reconnection, request IDs, deduplication, or replay mechanics. Those belong to M0.5.

## Logical operation result algebra

The logical operation surface is:

```text
describe_adapter -> AdapterDescription | ProtocolFailure
validate_plan    -> ValidatePlanResponse | ProtocolFailure
execute_plan     -> ExecutePlanResponse | ProtocolFailure
```

A valid negative operation-domain response remains a successful protocol response. In particular:

- `ValidatePlanResponse.preflight = rejected | unavailable` is not `ProtocolFailure`.
- `ExecutePlanResponse.execution = failed | rejected | unavailable` is not `ProtocolFailure`.
- `ProtocolFailure` means the operation did not produce its normal valid response type.

## ProtocolFailure

```text
ProtocolFailure
  category:
    compatibility
    invalid_request
    operational
  code: string
  detail: string
  side_effects:
    none
    may_have_occurred
```

`ProtocolFailure` deliberately has no `adapter_protocol_version` field. Compatibility failure can occur before peers establish a common Adapter Protocol version.

The initial stable failure codes are:

```text
protocol.compatibility_not_established
protocol.unsupported_adapter_protocol
protocol.unsupported_public_contract
protocol.malformed_bootstrap
protocol.invalid_request
adapter.operational_failure
```

Failure `code` is machine-readable. `detail` is human-readable and MUST NOT be parsed to infer lifecycle state, retryability, or idempotency.

### Category semantics

`compatibility` means interoperability was not established because required Adapter Protocol/Public Contract support is incompatible or required compatibility evidence is missing. Adapter execution side effects are prohibited, so `side_effects` MUST be `none`.

`invalid_request` means bootstrap/request syntax, versioning, or contract requirements are malformed, unsupported, or incoherent. The failure occurs before adapter execution side effects, so `side_effects` MUST be `none`.

`operational` means the adapter/runtime could not produce the normal valid operation result. For `describe_adapter` and `validate_plan`, `side_effects` MUST be `none`. For `execute_plan`, `none` is permitted only when the producer can prove execution side effects did not begin; otherwise the value MUST be `may_have_occurred`.

### Side-effect evidence is not retry policy

`side_effects = none` is a producer guarantee that no operation side effect occurred. `side_effects = may_have_occurred` is the conservative state when that guarantee cannot be made.

Protocol 0.1 defines no `retryable`, `safe_to_retry`, idempotency key, deduplication token, validation token, lease, resume token, or automatic replay directive. A future transport layer may use the evidence, but it must not reinterpret the evidence as a retry instruction.

## Lifecycle separation

Neither `ProtocolFailure` nor preflight/execution-domain outcomes are automatically any of:

```text
PASS
FAIL
BLOCKED
INDETERMINATE
```

Raw compatibility/bootstrap/request/operational failure is not SOL semantic evaluation. Core lifecycle classification may occur only from canonical semantic evidence defined for Core comparison, such as Public Contract `RealizationEffectDto` data returned in a valid execution response.

## validate_plan idempotency

For the same normalized `ValidatePlanRequest` and equivalent relevant adapter state/evidence, repeated validation MUST produce the same normalized `ValidatePlanResponse` or the same logical failure classification.

Validation creates no durable execution authorization and no execution side effect. A different result is allowed when relevant evidence changes, including descriptor scope, capability evidence, availability, or preconditions. This is state-dependent determinism, not a violation of idempotency.

Protocol 0.1 does not define transport retry timing or replay even for validation.

## execute_plan non-idempotency

`execute_plan` is non-idempotent by default. Submitting the same canonical `ExecutePlanRequest` again is a new execution attempt.

Protocol 0.1 does not standardize a generic stronger idempotency declaration. Adapter package version, ordinary capability declaration, prior validation result, and repeated canonical request equality do not imply replay safety.

If real-adapter evidence later requires a portable stronger guarantee, it requires an explicit versioned protocol extension rather than a silent interpretation of 0.1 metadata.

## Current-state re-check for repeated execution

Each execute attempt re-checks current state before any new side effect. When evidence is available, Protocol 0.1 uses conservative deterministic precondition rejection:

- already-realized state: `execution = rejected`, all actions `not_started`, diagnostic `adapter.precondition_rejected`, context precondition `already_realized`;
- partial prior execution: reject before new side effects with precondition `partial_prior_execution` unless the current request can genuinely execute as a fresh request without hidden resume authority;
- unresolved current prerequisite: reject before side effects with precondition `unresolved_prerequisite`.

If failure or dependency loss occurs after execution begins, the Phase 3 partial/failed/skipped execution reporting rules apply rather than converting the result into a pre-side-effect rejection.

These precondition strings are namespaced protocol diagnostic context evidence. They do not create new canonical semantic identity.

## Ambiguous execute-response boundary

If an execute request may have reached execution and the caller observes neither a valid `ExecutePlanResponse` nor a `ProtocolFailure` that proves `side_effects = none`, the caller MUST assume side effects may have occurred.

```text
no execute response
!= no side effect
!= safe to retry
```

The caller MUST NOT synthesize an `ExecutePlanResponse`, SOL lifecycle state, or replay authorization from response absence. Automatic retry/replay remains prohibited in M0.3. M0.5 must preserve this constraint when transport behavior is defined.

## Executable evidence

Positive Phase 4 fixtures under `fixtures/adapter-protocol/0.1/` cover:

- Adapter Protocol incompatibility;
- Public Contract incompatibility;
- missing compatibility evidence;
- malformed bootstrap and invalid request failures;
- side-effect-free operational validation failure;
- execute operational failure before side effects;
- ambiguous execute failure after side effects may have occurred;
- repeated validation under equivalent relevant state;
- already-realized, partial-prior-execution, and unresolved-prerequisite execute rejection;
- valid rejected/unavailable preflight responses;
- existing valid exact/unavailable execution responses.

Counterexamples under `fixtures/counterexamples/` prove that lifecycle status and retryability cannot be smuggled into `ProtocolFailure`, and that response absence cannot be interpreted as proof of no side effect or safe replay.

The executable contract tests are in `crates/sol-adapter-protocol/tests/error_state_semantics.rs`.
