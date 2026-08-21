# Adapter Protocol 0.1 — Target Evidence and `validate_plan` Preflight

**Status:** M0.3 Phase 2 normative baseline  
**Issue:** #41  
**Date:** 2026-08-21

## Purpose

This contract defines how Core consumes adapter-description target/capability evidence and how an adapter performs deterministic advisory plan preflight before execution. It preserves ADR-002 and the Phase 1 bootstrap/compatibility contract.

`validate_plan` is advisory. A successful response is not a durable authorization token, lease, acceptance ID, plan hash, or execution permit.

## Core target resolution remains Core-local

Core resolves a desired Public Contract `BackendTargetDto` against `AdapterDescription.targets` and their declared capabilities. Target matching and required-capability subset checks select compatible adapter evidence; they do not authorize execution.

`sol-target-resolver` consumes canonical Adapter Protocol description data rather than MockAdapter-specific capability types. Adapter target confirmation during preflight remains a separate decision from Core target resolution.

Capability declarations are descriptor-scope claimed representability evidence. They are not proof of physical correctness and are not current backend availability.

## `ValidatePlanRequest`

```text
ValidatePlanRequest
  adapter_protocol_version: "0.1"
  target: BackendTargetDto
  plan: MappingPlanDto
```

`target` and `plan` are the existing Public Contract 0.1 DTOs. Adapter Protocol does not duplicate their fields or redefine MappingPlan action/dependency semantics.

Both reused payloads must declare one coherent supported Public Contract version. For Protocol 0.1 in this repository that version is Public Contract `0.1`.

The full canonical `MappingPlanDto` is the Protocol 0.1 identity/integrity carrier. There is no standardized plan digest, hash algorithm, server-generated plan ID, validation token, lease, or acceptance token.

A local implementation may cache or hash canonical JSON internally, but such values are not Protocol 0.1 semantics.

## `ValidatePlanResponse`

```text
ValidatePlanResponse
  adapter_protocol_version: "0.1"
  target_compatible: boolean
  capabilities_satisfied: boolean
  preflight: accepted | rejected | unavailable
  diagnostics: Diagnostic[]
```

The response keeps three decisions distinct:

- `target_compatible`: the adapter confirms that it addresses the requested target symbol;
- `capabilities_satisfied`: the requested target requirements are satisfied by declared capability evidence;
- `preflight`: current adapter-side inspectable acceptance/precondition state.

These fields are protocol/preflight outcomes and are not SOL lifecycle values.

## Consistency rules

- target mismatch -> `target_compatible=false`, `preflight=rejected`, `adapter.target_mismatch` error diagnostic;
- missing required capability -> `capabilities_satisfied=false`, `preflight=rejected`, `adapter.missing_capability` error diagnostic;
- unsupported plan action -> target/capability checks may remain true, `preflight=rejected`, `adapter.unsupported_action`;
- invalid adapter-specific prerequisite -> target/capability checks may remain true, `preflight=rejected`, `adapter.precondition_rejected`;
- transient backend/environment unavailability -> target/capability checks remain true, `preflight=unavailable`, `adapter.transient_unavailable`;
- accepted -> target and capability booleans true, `preflight=accepted`, and no error diagnostic.

Rejected/unavailable states require machine-stable error diagnostics. Human-readable `detail` is evidence for people and must not be parsed as machine identity.

## Diagnostic reuse and protocol context

Adapter Protocol reuses Public Contract `Diagnostic` rather than defining a parallel diagnostic structure.

Initial stable codes:

- `adapter.target_mismatch`
- `adapter.missing_capability`
- `adapter.unsupported_action`
- `adapter.precondition_rejected`
- `adapter.transient_unavailable`

Plan action IDs and adapter-local precondition symbols are stable protocol symbols but are not necessarily canonical simulation references. They therefore must not be forced into `Diagnostic.subject`.

Machine-readable protocol context uses the Public Contract extension mechanism:

```text
sol_adapter_protocol_context:
  target?: string
  capability?: string
  plan_action_id?: string
  precondition?: string
```

This context is evidence for the diagnostic condition. It does not become canonical simulation identity.

## TOCTOU and execution handoff

Protocol 0.1 explicitly permits this sequence:

```text
validate plan A -> accepted
state or plan changes
execute later
```

The previous acceptance is insufficient authorization. Phase 3 `execute_plan` must carry the full canonical plan intended for execution again and must authoritatively re-check compatibility, plan identity/integrity, current capability/precondition evidence, and execution-critical state before side effects.

A changed plan has a different canonical plan representation. No prior validation token can bypass the authoritative execution checks.

## MappingPlan ownership

Adapter preflight may inspect plan actions and dependencies, but it may not mutate action identity, dependency edges, or canonical DAG meaning.

Canonical deterministic representation/topological ordering exists for reproducibility and semantic comparison. It does not promise that a backend will physically execute independent actions in that exact total order.

## Backend-native and transport exclusions

The request/response must not use backend-native object/type/selection IDs as canonical target, plan, or preflight semantics. Opaque backend references are permitted only in separately allowed provenance/evidence roles where semantic identity/equality does not depend on them.

Protocol 0.1 preflight also contains no JSON-RPC request IDs, stdio frames, subprocess lifecycle state, retry policy, reconnect state, timeout transport semantics, or other M0.5 transport concerns.

The following fields are explicitly forbidden from acquiring Protocol 0.1 preflight semantics:

```text
validation_token
acceptance_id
lease_id
plan_hash
plan_digest
backend_native_id
backend_object
native_object
solver_object
```

## Executable fixtures

Positive fixtures:

- `fixtures/adapter-protocol/0.1/validate-plan-accepted-request.json`
- `fixtures/adapter-protocol/0.1/validate-plan-accepted-response.json`

Counterexamples cover target mismatch, missing capability, unsupported action, prerequisite rejection, transient unavailability, inconsistent accepted response, durable-token authority, backend-native semantic leakage, and a changed-plan TOCTOU case.

## Compatibility rule

The meaning of `target_compatible`, `capabilities_satisfied`, `accepted/rejected/unavailable`, the five stable diagnostic codes above, and the rule that preflight is advisory are Adapter Protocol 0.1 semantics. They must not be silently redefined under the same protocol version.
