# ADR-005 — Canonical Simulation Run and Result Boundary

**Status:** Accepted  
**Date:** 2026-08-23  
**Milestone:** M0.10 Canonical Simulation Run & Result Contract  
**Issues:** #153, #154

## Context

SOL already has an accepted solver-independent realization boundary:

```text
MappingPlan
  -> action identity + dependency DAG

RealizationSpec
  -> solver-independent realization intent

Adapter Protocol 0.2
  -> target + plan + realization_spec
  -> backend execution
```

M0.9 proved this path through the solver-neutral Adapter Runtime and an external adapter process. That path deliberately stops at execution/effect/provenance evidence. It does not yet define a canonical public representation for numerical/scientific results produced by a solver.

A real solver may produce scalar observations, distributed fields, native files, postprocessor values, dataset objects, result handles, or other backend-specific artifacts. Promoting those native objects or identifiers into canonical SOL identity would violate the established Core/backend separation. At the same time, treating execution success as sufficient scientific-result evidence would conflate two different concerns: whether execution completed and whether each requested observation was actually produced.

M0.10 therefore introduces an explicit run/result boundary without changing Public Contract 0.1/0.2 or Adapter Protocol 0.1/0.2 semantics.

## Decision

### 1. Public Contract 0.3 is the run/result semantic boundary

Canonical run/result semantics are added through **Public Contract 0.3**.

SOL does not introduce a separate `Result Contract` version axis. Public Contract already owns canonical public semantic representations; adding an independent result-contract axis would create unnecessary compatibility combinations among Runtime, Public Contract, Result Contract, Adapter Protocol, ontology packages, and adapter implementations.

Public Contract 0.3 is an explicit later contract boundary. It preserves the accepted Public Contract 0.2 realization semantics, including:

- `MappingPlan` / `RealizationSpec` separation;
- action-ID non-semantic behavior;
- canonical solver-independent realization identity;
- backend-native leakage prohibition;
- deterministic normalization and reference-integrity rules.

Public Contract 0.1 and 0.2 remain behaviorally frozen.

### 2. Adapter Protocol 0.3 is the result-capable interoperability boundary

Result-capable external adapter interoperability uses **Adapter Protocol 0.3**.

This is not an optional reinterpretation or relabeling of Adapter Protocol 0.2. Existing 0.1/0.2 request/response meanings remain unchanged.

A result-capable interoperability path requires explicit compatibility with both:

```text
Adapter Protocol 0.3
AND
Public Contract 0.3
```

Old adapters are not silently promoted, upgraded, downgraded, or coerced into the 0.3 path. Existing validate/execute authority, non-idempotent execution, ambiguous response-loss, and no-automatic-replay semantics remain preserved unless a later accepted decision explicitly versions a change.

### 3. SimulationRunRecord is a canonical operational record

`SimulationRunRecord` represents one canonical record of an execution occurrence and its result coverage.

It is **not** a SimulationModel ontology/model-semantic entity and does not participate in model semantic equality.

Conceptually:

```text
Semantic/model layer
  SimulationModel
  SimulationTask
  ObservationModel
  RealizationSpec
          |
          v
Operational record layer
  SimulationRunRecord
          |
          +-- execution evidence
          +-- observation outcomes
          +-- provenance
```

A SOL run identity must not be derived from a backend-native job ID, file name, object handle, COMSOL tag, MOOSE output path, ANSYS result ID, or equivalent vendor object.

### 4. ObservationModel owns requested meaning; ObservationResult owns produced scientific value

`ObservationModel` continues to define the canonical meaning of what is requested or observed.

`ObservationResult` represents what was actually produced for a requested canonical observation during a specific run.

The run/result boundary therefore preserves:

```text
ObservationModel != ObservationResult
execution outcome != observation production
```

Execution completion MUST NOT imply that a requested scientific observation was produced.

Every requested observation must have exactly one terminal observation outcome associated with the run:

```text
requested observation
  -> Produced(result)
  OR
  -> typed non-production evidence / diagnostic
```

Exact serialized outcome names are downstream DTO design details, but exact terminal coverage is normative.

### 5. Compact scalar observations are inline

The initial compact result category is scalar observation data.

Conceptually:

```text
ScalarObservationResult
  observation reference
  run reference
  value
  canonical unit
```

Scalar representation is canonical semantic data, not a backend postprocessor name or file locator.

The first M0.10 scalar vertical slice is `MaximumTemperature` in the steady thermal case.

### 6. Distributed/field observations use manifest + referenced payload

Distributed fields are not embedded indiscriminately into public JSON and are not represented merely by a backend-native artifact path.

Conceptually:

```text
DatasetObservationResult
  observation reference
  run reference
  DatasetManifest
  DataReference
```

`DatasetManifest` carries the solver-independent semantic metadata required to interpret the result, such as accepted quantity/unit/scope/support/location/component/index/coordinate metadata.

`DataReference` identifies the associated solver-neutral dataset payload according to the representation accepted in Phase 2.

Representation is category-driven, not selected by arbitrary payload byte size.

The concrete canonical dataset storage/serialization format, content-identity rule, and normalization strategy remain deliberately deferred to M0.10 Phase 2 (#156).

### 7. Backend-native artifacts are provenance only

Backend-native artifacts and identifiers may be carried as opaque provenance/evidence but cannot define canonical scientific-result identity or equality.

Examples include:

- MOOSE Exodus files, Postprocessor names, CSV paths, job/workspace IDs;
- COMSOL Dataset, Derived Value, solution/result tags;
- ANSYS result/DPF objects, native handles, job IDs;
- other vendor-native file paths, object identifiers, handles, or API wrappers.

The following invariants hold:

```text
backend locator A != backend locator B
```

does not imply canonically different observations when the canonical meaning/data are otherwise the same, and:

```text
same backend locator
```

does not collapse canonically different quantities, scopes, or observations.

A backend artifact reference alone is insufficient to constitute a canonical scientific result.

## Initial Public Contract 0.3 conceptual surface

Exact DTO names remain implementation-reviewable, but the accepted semantic surface is:

```text
Public Contract 0.3
  -> preserved 0.2 realization semantics
  -> SimulationRunRecord
  -> terminal ObservationOutcome
  -> ObservationResult
  -> ScalarObservationResult
  -> DatasetManifest
  -> solver-neutral DataReference
```

## Initial Adapter Protocol 0.3 conceptual surface

```text
Adapter Protocol 0.3
  -> explicit Public Contract 0.3 pair
  -> existing execution/no-replay invariants preserved
  -> run/result transport
  -> exact requested-observation coverage
```

## Required counterexamples

Implementation and conformance must preserve at least these distinctions:

1. Execution may complete successfully while one or more requested observations are not produced.
2. Two different backend-native result identifiers may represent the same canonical observation meaning; native identifiers cannot determine canonical equality.
3. One backend artifact identifier cannot substitute for canonically different quantities/scopes.
4. A field result is not canonical merely because a native artifact exists.
5. A 0.1/0.2 adapter cannot be treated as 0.3-compatible without explicit 0.3 Protocol + Public Contract evidence.
6. Public Contract 0.3 must not reinterpret Public Contract 0.2 realization semantics.

## Rejected alternatives

M0.10 Phase 0 rejects:

- introducing a separate `Result Contract 0.1` version axis;
- silently extending or relabeling Adapter Protocol 0.2 with mandatory result semantics;
- making `SimulationRunRecord` a model-semantic ontology entity or part of model semantic equality;
- inferring observation production from top-level execution success;
- making a MOOSE/COMSOL/ANSYS result object, path, tag, handle, or file canonical SOL identity;
- using backend artifact reference alone as a canonical distributed scientific result;
- choosing scalar-vs-dataset representation by arbitrary byte-size threshold.

## Consequences

SOL gains a solver-independent boundary from canonical observation intent to canonical scientific output while preserving independently versioned external adapters and backend-local result mechanisms.

MOOSE A0.1 remains unblocked and may continue against Public Contract/Adapter Protocol 0.2 for RealizationSpec -> MOOSE IR -> `.i` -> execution/artifact capture. A later MOOSE result-integration milestone can implement explicit 0.3 compatibility after the M0.10 result contract/protocol surface is published.

Stable SDK/application surfaces should consume the completed canonical result boundary rather than invent result semantics independently.

## Validator gates

Validator verdict for the Phase 0 decision is **APPROVE WITH GATES**.

Implementation must prove:

1. Public Contract 0.3 does not redefine 0.2 realization semantics.
2. Adapter Protocol 0.3 is an explicit compatibility boundary and does not auto-promote old adapters.
3. Distributed field results cannot become canonical solely through backend-native artifact references.
4. Every requested observation has exact terminal coverage independent of top-level execution outcome.

If implementation exposes a new semantic decision concerning canonical dataset content identity/normalization, concrete field storage format, or existing 0.1/0.2 compatibility meaning, Operator must reopen Manager `meeting` rather than invent the decision during implementation.
