# ADR-002 — Adapter Protocol Boundary and Responsibility Ownership

**Status:** Accepted  
**Date:** 2026-08-21  
**Milestone:** M0.3 Adapter Protocol 0.1  
**Issue:** #39

## Context

M0.2 established Public Contract 0.1 as SOL's language-neutral semantic payload boundary. M0.3 now needs a separate Adapter Protocol 0.1 without serializing incidental Rust APIs, leaking backend-native object models, or prematurely coupling protocol semantics to JSON-RPC/process transport.

The accepted Manager review in `docs/plans/m0.3-adapter-protocol-manager-review.md` resolved the main architecture questions. This ADR promotes those accepted decisions into normative architecture before protocol DTO implementation begins.

## Decision

### 1. Public Contract and Adapter Protocol remain distinct

Public Contract 0.1 defines canonical solver-independent semantic payloads. Adapter Protocol 0.1 defines the interoperability operations and protocol-specific metadata needed to exchange and act on those payloads across the Core/adapter boundary.

Where semantics overlap, Adapter Protocol MUST reuse Public Contract 0.1 payloads rather than define parallel semantic JSON shapes.

Internal Rust traits, crate APIs, ownership models, and function decomposition are implementation details and are not the public protocol surface.

### 2. Responsibility ownership

Core-local responsibilities include:

- canonical model and Public Contract validation;
- MappingPlan DAG/dependency validity;
- canonical deterministic MappingPlan representation ordering;
- BackendTarget selection/resolution from declared capability evidence;
- semantic comparison/classification of canonical realization effects;
- final SOL lifecycle classification.

Adapter-internal responsibilities include:

- backend-native object/API interaction;
- backend scheduling of the whole validated plan;
- backend-side state/precondition inspection;
- normalization of backend-native realization into canonical Public Contract realization/effect reporting.

Shared Public Contract payloads include canonical MappingPlan, BackendTarget requirements where reused, realization/effect data, evaluation-related semantic data, and other already-published Public Contract 0.1 objects.

Opaque backend job/artifact references MAY cross as provenance/evidence metadata only. They MUST NOT define canonical semantic identity, equality, or solver-independent meaning.

### 3. Minimal public operation surface

Protocol 0.1 starts from the minimal candidate operation set:

- adapter description/bootstrap (`describe_adapter` candidate);
- advisory plan validation/preflight (`validate_plan` candidate);
- authoritative plan execution (`execute_plan` candidate).

These labels identify the intended logical operations. Exact wire names and payload schemas are defined by later M0.3 phases.

Any additional public operation requires independent justification. Conceptual responsibilities MUST NOT be converted one-for-one into remote methods merely because they exist in planning documents or Rust traits.

### 4. MappingPlan order is not backend physical total order

Core owns MappingPlan dependency semantics and canonical deterministic ordering for representation, comparison, fixtures, and reproducibility.

The adapter owns physical backend scheduling. It MAY execute independent actions in another valid topological order or in parallel if and only if:

- every dependency/prerequisite is preserved;
- execution remains consistent with the validated plan;
- required per-action and aggregate canonical reporting remains observable.

Therefore:

```text
canonical deterministic representation order != mandatory backend physical total order
```

### 5. Validation and execution authority

`validate_plan` is advisory preflight. It does not grant durable authorization for later execution.

`execute_plan` is the authoritative execution boundary and must re-check execution-critical compatibility, plan identity/integrity, capabilities, and current preconditions before side effects.

This prevents a successful earlier validation from becoming a TOCTOU assumption.

### 6. Compatibility and bootstrap

Interoperability requires both:

```text
Adapter Protocol compatibility
AND
Public Contract compatibility
```

Adapter implementation/package version implies neither.

Protocol 0.1 requires a version-safe description/bootstrap that can expose enough information to reject incompatible peers without requiring the caller to first assume compatibility with an incompatible full protocol payload.

Capability declarations in one adapter description are treated as stable for that description's lifetime/scope unless explicitly stated otherwise. Transient backend/environment availability belongs to validate/execute preconditions, not a general Protocol 0.1 capability event model.

### 7. Error and lifecycle separation

Protocol/bootstrap incompatibility, request-contract failure, and operational execution failure are protocol-domain failures. They are not themselves SOL lifecycle values.

They MUST NOT be directly relabeled as:

```text
PASS
FAIL
BLOCKED
INDETERMINATE
```

Only a successful canonical semantic outcome/evidence may later be interpreted by Core under SOL lifecycle semantics.

### 8. Idempotency and transport boundary

Equivalent `validate_plan` evaluation should be idempotent for the same canonical input and relevant declared state/evidence.

`execute_plan` is non-idempotent by default unless explicitly guaranteed.

JSON-RPC request IDs, stdio framing, process lifecycle, reconnect/retry/backoff/replay, and transport-error mechanics are M0.5 concerns and are not normative Adapter Protocol 0.1 semantics.

### 9. Publication boundary

M0.3 publishes a versioned Adapter Protocol 0.1 baseline. It does not claim mature long-term stability before real-adapter evidence, but once published, existing 0.1 semantics cannot be silently redefined. Incompatible later feedback requires an explicit protocol version boundary.

## Responsibility matrix

| Concern | Ownership / classification | Crosses protocol boundary? |
|---|---|---|
| Canonical simulation/model semantics | Public Contract / Core | As Public Contract payload |
| MappingPlan DAG/dependency validity | Core-local | Plan payload crosses; validity ownership does not |
| Canonical plan representation order | Core-local semantic representation | Yes as canonical payload ordering |
| Backend physical scheduling | Adapter-internal | Reported outcome/evidence only |
| Adapter identity/version | Protocol metadata | Yes |
| Supported Adapter Protocol versions | Protocol bootstrap metadata | Yes |
| Supported Public Contract versions | Protocol bootstrap metadata | Yes |
| Capability declaration/evidence | Protocol metadata | Yes |
| BackendTarget selection/resolution | Core-local | No standalone remote operation |
| Plan preflight validation | Public protocol operation | Yes |
| Plan execution | Public protocol operation | Yes |
| Backend-native realization normalization | Adapter-internal | Canonical normalized result crosses |
| RealizationEffect semantic comparison | Core-local | No standalone remote operation |
| Backend-native object/API structure | Adapter-internal only | Forbidden as canonical semantic payload |
| Opaque backend artifact/job reference | Provenance/evidence metadata | Permitted, non-semantic only |
| JSON-RPC / stdio / process lifecycle | M0.5 transport | Not normative in M0.3 |

## Rejected alternatives

The following designs are rejected:

- serializing the existing Rust Adapter trait as the protocol;
- one remote method per conceptual responsibility;
- duplicating Public Contract semantic payload shapes inside Adapter Protocol;
- requiring independent actions to execute in canonical representation order;
- making `validate_plan` a durable authorization token;
- assuming `execute_plan` is idempotent/retry-safe by default;
- treating raw protocol/operational errors as SOL lifecycle states;
- using backend-native object/vendor API structures as canonical semantic identity;
- banning all backend references, including opaque provenance/evidence handles;
- defining JSON-RPC/stdio/process semantics in M0.3.

## Consequences

This boundary keeps the protocol small and language-neutral, protects the canonical semantic contract from backend leakage, permits legitimate backend optimization/parallel scheduling, and leaves transport mechanics replaceable.

The cost is that Core and adapter implementations must maintain explicit conversion/reporting boundaries rather than sharing solver-native types or relying on one in-process Rust interface.

## Validation obligations

Phase 0 architecture tests MUST reject at least:

- duplicate protocol-specific semantic payload definitions for Public Contract concepts;
- backend-native semantic payload leakage;
- opaque provenance used as semantic identity;
- canonical-order overconstraint on independent backend actions;
- one-method-per-responsibility overexposure.

Later M0.3 phases must add executable protocol fixtures for dual compatibility, version-safe bootstrap, advisory validation/authoritative execution, error/lifecycle separation, idempotency boundaries, and transport exclusion.

## Supersession

This ADR is normative for Adapter Protocol 0.1 architecture. Changes to these invariants require a new Manager/Researcher/Validator decision process and an ADR update/supersession rather than an incidental implementation change.
