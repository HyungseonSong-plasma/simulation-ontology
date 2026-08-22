# ADR-004 — Canonical RealizationSpec Boundary

**Status:** Accepted  
**Date:** 2026-08-22  
**Milestone:** M0.8 Canonical Realization Contract 0.2  
**Issues:** #110, #124, #125

## Context

The first real SOL adapter exposed a gap that MockAdapter conformance could not reveal. Public Contract 0.1 `PlanActionDto` carries only `id` and `dependencies[]`; Adapter Protocol 0.1 `validate_plan` and `execute_plan` carry only `BackendTargetDto + MappingPlanDto`. That is enough to validate DAG/scheduling/effect/failure semantics, but not enough for an independent real adapter to construct a deterministic backend model.

Two canonically different thermal models may legitimately produce the same plan action IDs and dependency graph while differing in conductivity, boundary values, spatial geometry, constitutive choice, or observation intent. If an adapter receives only the plan, those models are indistinguishable at the adapter boundary.

Hard-coding meaning into action-ID spelling, interpreting opaque extension fields as hidden physics, or reading private Core state would violate the language-neutral external adapter boundary.

## Decision

### 1. MappingPlan remains planning/dependency structure

`MappingPlan` continues to define:

- stable action identity;
- dependency edges / DAG validity;
- deterministic canonical representation;
- canonical planning/topological information.

It does **not** define the physical, mathematical, constitutive, material, spatial, analysis, observation, or parameter meaning required to realize an action.

```text
MappingPlan != RealizationSpec
```

The spelling of an action ID is non-semantic. An adapter MUST NOT infer realization meaning from identifiers such as `thermal.material` or `thermal.solve`.

### 2. RealizationSpec is a separate canonical solver-independent payload

Public Contract 0.2 introduces `RealizationSpec` as the immutable canonical realization projection supplied to an adapter alongside a plan.

The initial 0.2 realization boundary carries enough explicit information for the thermal vertical slice:

- canonical realization entities with kind + semantic type;
- explicit scalar parameter values;
- canonical unit references;
- canonical spatial scopes/membership;
- canonical semantic relations;
- explicit action-to-semantic-subject/scope bindings;
- source model and ontology-version provenance needed to interpret the projection.

Backend-native object types, IDs, mesh-selection handles, solver input block names, executable handles, or backend IR structures are forbidden as canonical semantic identity.

### 3. PlanAction opaque extensions cannot substitute for RealizationSpec

Public Contract 0.1 extension behavior remains unchanged: unknown optional fields remain opaque and cannot acquire new canonical meaning.

Public Contract 0.2 additionally reserves the realization boundary explicitly. PlanAction extensions MUST NOT be interpreted as authoritative physics/realization meaning. Known attempts to hide realization data in PlanAction extensions are rejected by the 0.2 realization validation path; unknown opaque fields remain non-semantic.

### 4. Values and units are explicit

The initial RealizationSpec scalar quantity representation is:

```text
Quantity
  value: JSON number
  unit: canonical reference
```

A parameter binding names an explicit canonical semantic parameter symbol/reference and carries a quantity. Units are not free-form prose. They are canonical references such as:

```text
unit.kelvin
unit.meter
unit.watt_per_meter_kelvin
```

0.2 does not claim a complete unit-conversion algebra. It fixes explicit quantity/unit identity and deterministic serialization; richer dimensional analysis may evolve separately.

### 5. Spatial applicability remains first-class and solver-independent

`SpatialScope` remains distinct from `SpatialModel` and backend-native selections.

RealizationSpec may project spatial entities and their explicit scalar parameters (for example length or position) plus canonical scopes/memberships. An adapter maps these into backend mesh/subdomain/boundary constructs without making native selections canonical.

### 6. Every plan action is explicitly bound

A RealizationSpec used with a MappingPlan must contain exactly one action binding for every plan action and no binding for an unknown action.

Each binding references explicit canonical semantic subjects and/or SpatialScopes. Every referenced subject/scope must resolve inside the RealizationSpec projection.

This prevents an adapter from reconstructing missing meaning from action spelling or private state.

### 7. Public Contract 0.1 remains frozen; 0.2 is an explicit version boundary

Public Contract 0.1 semantics and schemas remain unchanged.

The new realization payload is published under Public Contract 0.2. The existing 0.1 Rust facade/DTO behavior must remain backward compatible.

0.2 may reuse unchanged nested semantic concepts, but top-level 0.2 realization documents declare `public_contract_version: "0.2"` explicitly.

### 8. Adapter Protocol 0.1 remains frozen; Protocol 0.2 carries RealizationSpec

Adapter Protocol 0.1 remains unchanged.

Adapter Protocol 0.2 request semantics are:

```text
ValidatePlanRequest 0.2
  adapter_protocol_version: "0.2"
  target: Public Contract 0.2 BackendTarget
  plan: Public Contract 0.2 MappingPlan
  realization_spec: Public Contract 0.2 RealizationSpec

ExecutePlanRequest 0.2
  adapter_protocol_version: "0.2"
  target: Public Contract 0.2 BackendTarget
  plan: Public Contract 0.2 MappingPlan
  realization_spec: Public Contract 0.2 RealizationSpec
```

`validate_plan` remains advisory and side-effect free. `execute_plan` remains authoritative and rechecks the complete current request/state before side effects. 0.2 creates no validation token, lease, implicit replay authority, or automatic execute replay.

### 9. Compatibility remains dual-axis and explicit

An adapter claiming the 0.2 realization path must explicitly establish both:

```text
Adapter Protocol 0.2 compatibility
AND
Public Contract 0.2 compatibility
```

Adapter implementation version implies neither. Supporting Protocol 0.1/Public Contract 0.1 does not imply support for the 0.2 realization path.

### 10. RealizationSpec is input intent; RealizationEffect remains output evidence

The boundary is:

```text
canonical model
   -> MappingPlan          planning/dependencies
   +  RealizationSpec      explicit realization intent
   -> adapter-local IR     non-canonical
   -> backend execution
   -> RealizationEffect    canonical realization evidence
```

RealizationSpec does not report whether backend realization was exact/degraded/unsupported. That remains the responsibility of RealizationEffect and later Core comparison/evaluation.

## Required distinguishability invariant

The same MappingPlan may be valid for two different models:

```text
Plan(A) == Plan(B)
```

while their explicit realization intent differs:

```text
RealizationSpec(A) != RealizationSpec(B)
```

For the thermal reference case, changing conductivity or boundary temperature while keeping the plan DAG unchanged MUST produce distinct canonical RealizationSpec representations.

## Rejected alternatives

M0.8 rejects:

- adding physics meaning to action-ID spelling;
- enriching `PlanActionDto` with backend-specific objects;
- assigning new canonical meaning to opaque 0.1 extensions;
- passing a private Core graph/state handle to an external adapter;
- making a solver-native IR part of the Public Contract;
- silently adding mandatory realization data to Protocol/Public Contract 0.1;
- using prior `validate_plan` acceptance as execution authorization;
- inferring 0.2 compatibility from adapter package version.

## Consequences

Real adapters gain enough canonical information to build deterministic backend-native IR while Core preserves a clean separation among ontology truth, planning dependencies, realization intent, backend realization, and realization evidence.

The explicit version boundary means 0.2-aware adapters must advertise new compatibility. Existing 0.1 adapters remain valid for the published 0.1 contract and are not retroactively required to understand RealizationSpec.

## Validation obligations

M0.8 implementation must include:

- a thermal positive RealizationSpec fixture;
- a same-plan/different-spec distinguishability counterexample;
- missing/unknown/duplicate action-binding rejection;
- unresolved semantic subject/scope rejection;
- malformed canonical unit-reference rejection;
- backend-native leakage rejection;
- PlanAction hidden-realization-extension rejection;
- mixed 0.1/0.2 Protocol/Public Contract request rejection;
- explicit dual-axis 0.2 compatibility evidence;
- preservation of all existing 0.1 schema, protocol, transport, and conformance tests.

## Cross-team handoff rule

The MOOSE adapter may continue meaning-neutral host/process/backend/IR/workspace work independently. It may start canonical SOL thermal semantics -> MOOSE IR mapping only after this 0.2 realization boundary is merged and the adapter can establish Protocol 0.2 + Public Contract 0.2 compatibility.

MOOSE-native mapping remains adapter-team authority; canonical RealizationSpec semantics remain SOL Platform authority.