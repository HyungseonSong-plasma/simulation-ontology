# Adapter Protocol 0.1 — Execution and Realization Contract

**Status:** M0.3 Phase 3 normative baseline  
**Issue:** #42  
**Date:** 2026-08-21

## Purpose

This document defines the transport-independent `execute_plan` semantic boundary for Adapter Protocol 0.1. It preserves Core ownership of canonical `MappingPlan` DAG semantics and semantic effect comparison while allowing an adapter to choose any dependency-preserving backend schedule.

The execution contract reuses Public Contract 0.1 `BackendTargetDto`, `MappingPlanDto`, `RealizationEffectDto`, and `Diagnostic`. It does not define duplicate semantic payloads, JSON-RPC framing, stdio/process behavior, retries, or lifecycle evaluation.

## ExecutePlanRequest

```text
ExecutePlanRequest
  adapter_protocol_version: "0.1"
  target: BackendTargetDto
  plan: MappingPlanDto
```

The complete canonical target and plan are resent at execution time. Protocol 0.1 does not define a validation token, acceptance ID, lease, standardized plan hash/digest, or reference-only execution request.

`execute_plan` is the authoritative operation boundary. Before side effects, an adapter re-checks the Protocol/Public Contract compatibility needed by the request, current target/capability evidence, the full plan actually presented, and execution-critical preconditions/current availability. A prior successful `validate_plan` cannot authorize execution by itself.

## ActionExecutionReport

Every PlanAction in the request has exactly one terminal report:

```text
ActionExecutionReport
  action_id: string
  state:
    completed
    failed
    unavailable
    skipped_dependency
    not_started
  effects: RealizationEffectDto[]
  diagnostics: Diagnostic[]
  provenance?: ExecutionProvenance
```

The states mean:

- `completed`: the action executed to a terminal adapter result. It may report zero or more canonical realization effects.
- `failed`: the action started but encountered an execution-domain failure. It requires error diagnostic evidence and may retain realization effects produced before failure.
- `unavailable`: the action could not start because required current backend/environment state was unavailable. It reports no realization effects and carries `adapter.transient_unavailable` evidence.
- `skipped_dependency`: the action did not start because at least one prerequisite action did not complete. It reports no realization effects and carries `adapter.dependency_skipped` evidence.
- `not_started`: no side effect for the action occurred, including authoritative rejection before execution.

A missing, duplicate, or unknown action report is malformed.

## Execution scheduling evidence

Physical/logical scheduling is reported as ordered batches:

```text
execution_batches: [
  [action_a, action_b],
  [action_c]
]
```

Batch order expresses execution precedence evidence. Actions in one batch may execute concurrently. Batch members are canonicalized as a set-like list for deterministic representation.

Only actions that actually started (`completed` or `failed`) appear in execution batches, exactly once. Empty batches are invalid. A dependency must have state `completed` and must occur in an earlier batch than a dependent started action. A dependency in the same or a later batch is invalid.

Canonical `MappingPlan` representation/topological order is not a mandatory physical backend total order. Independent actions may be reordered or parallelized.

## ExecutePlanResponse

```text
ExecutePlanResponse
  adapter_protocol_version: "0.1"
  execution:
    completed
    partial
    failed
    rejected
    unavailable
  execution_batches: string[][]
  action_reports: ActionExecutionReport[]
  effects: RealizationEffectDto[]
  diagnostics: Diagnostic[]
  provenance?: ExecutionProvenance
```

The top-level execution state is execution-domain information, not SOL lifecycle evaluation:

- `completed`: all plan actions completed.
- `partial`: at least one action completed but not every action completed.
- `failed`: execution began, no action completed, and at least one started action failed.
- `rejected`: authoritative pre-side-effect checking rejected execution; every action is `not_started`.
- `unavailable`: authoritative pre-side-effect current-state availability prevented execution; no action started, at least one action is `unavailable`, and remaining actions are `not_started`.

`PASS`, `FAIL`, `BLOCKED`, and `INDETERMINATE` are not Adapter Protocol execution states.

## Realization effects

Action-level effects reuse Public Contract 0.1 `RealizationEffectDto`. The top-level `effects` field is only a deterministic convenience aggregate and MUST equal the normalized union of all action-report effects.

For Protocol 0.1:

- every effect remains a Public Contract semantic realization statement;
- exact duplicate effect objects collapse;
- semantically distinct effect objects remain distinct;
- canonical ordering is by canonical serialized representation;
- `degraded`, `unsupported`, and other realization qualities do not themselves produce Core lifecycle status in the adapter protocol.

`compare_effects`, semantic comparison, and `EvaluationResult` remain Core responsibilities.

## Execution provenance

```text
ExecutionProvenance
  producer: string
  opaque_references?: [
    {
      namespace: string
      reference: string
    }
  ]
```

Opaque references are traceability/evidence only. They may identify a backend job, artifact, run, or other adapter-native trace in a namespaced opaque form, but they MUST NOT become canonical target identity, action identity, dependency identity, MappingSubject identity, RealizationEffect subject identity, or semantic equality keys.

Backend/vendor object structures and native selection/object identifiers remain forbidden as canonical protocol semantics. Opaque references are permitted only inside the explicit provenance container.

## Stable Phase 3 execution diagnostic codes

Protocol 0.1 adds:

- `adapter.execution_failed`
- `adapter.dependency_skipped`
- `adapter.execution_rejected`

Phase 2 codes remain reusable where their meaning still applies, including `adapter.precondition_rejected` and `adapter.transient_unavailable`.

Machine behavior must use diagnostic codes/states rather than parsing human-readable diagnostic detail.

## TOCTOU rule

The authoritative boundary is intentionally explicit:

```text
validate plan A -> accepted
state changes or execute request contains plan B
execute_plan -> inspect the full current request/state again
             -> reject, report unavailable, or execute the current valid plan
```

A prior preflight response cannot force execution to proceed. Protocol 0.1 contains no durable authorization token that bypasses execute-time re-checks.

## Canonical validation rules

A response is valid against a request only when all of these hold:

1. every request PlanAction has exactly one report and there are no unknown reports;
2. started actions appear exactly once in `execution_batches`, and non-started actions appear in none;
3. every started action has all dependencies completed in earlier batches;
4. `skipped_dependency` is used only when at least one prerequisite did not complete;
5. aggregate effects equal the normalized union of action effects;
6. the declared top-level execution outcome is derivable from action terminal states;
7. required rejection/unavailability diagnostics are present where applicable;
8. opaque provenance references are not reused as semantic identity;
9. transport, retry/replay authority, validation-token, plan-digest, and backend-native semantic fields are absent.

## Explicit non-goals

Phase 3 does not define:

- JSON-RPC request/response IDs or framing;
- stdio/process lifecycle;
- automatic retry, replay, reconnect, or backoff;
- execute idempotency guarantees;
- timestamp/event-log normalization;
- one remote protocol operation per PlanAction;
- Core effect comparison or lifecycle classification;
- solver-native physical correctness proof.

Operational error/idempotency/state semantics are finalized in M0.3 Phase 4. Transport mechanics remain M0.5.

## Compatibility rule

After Adapter Protocol 0.1 publication, silently changing action state meaning, execution outcome derivation, batch/dependency rules, aggregate-effect normalization, or opaque-provenance identity rules is a semantic breaking change and requires an explicit protocol version boundary.
