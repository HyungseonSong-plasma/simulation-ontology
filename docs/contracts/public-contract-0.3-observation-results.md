# Public Contract 0.3 — Compact Observation Results

**Status:** M0.10 Phase 1 implementation contract  
**Parent:** #153  
**Phase:** #155  
**Architecture:** `docs/adr/ADR-005-canonical-run-result-boundary.md`

## Scope

This document defines the Public Contract 0.3 compact/scalar observation-result surface implemented in M0.10 Phase 1.

It does not define distributed field storage, dataset content identity/normalization, Adapter Protocol 0.3 transport, runtime routing, or backend extraction. Those remain later M0.10 phases.

Public Contract 0.1 and 0.2 retain their published meanings. A 0.1/0.2 document is not silently accepted as 0.3 and a 0.3 result document is not coerced into an older surface.

## Semantic ownership

`ObservationModel` remains the canonical statement of what is requested. Public Contract 0.3 records what happened for that request during one SOL run.

```text
ObservationModel/request
        |
        v
SimulationRunRecord
        |
        +-- Produced(ScalarObservationResult)
        |
        +-- NotProduced(typed diagnostic evidence)
```

`SimulationRunRecord` is an operational record. It is not a SimulationModel semantic entity and does not participate in model semantic equality.

## Scalar observation result

The Phase 1 compact result shape is:

```text
ScalarObservationResult
  public_contract_version = 0.3
  observation              canonical ObservationModel reference
  run_id                   canonical SOL run reference
  source                   canonical source quantity/field reference
  scope                    canonical observation scope reference
  value                    JSON number
  unit                     canonical unit reference
```

The result occurrence is anchored by the pair `(run_id, observation)`. This does not erase payload distinctions: changes in canonical source, scope, value, or unit remain distinguishable in the serialized result.

No backend postprocessor name, result handle, object identifier, file path, dataset tag, process/job identifier, or other solver-native locator is a canonical result identity field.

## Terminal observation outcome

For every entry in `requested_observations`, a `SimulationRunRecord` contains exactly one terminal observation outcome.

Phase 1 uses two typed outcomes:

- `produced` — contains one canonical scalar observation result;
- `not_produced` — contains the canonical observation reference plus non-empty typed diagnostic evidence.

The exact-coverage invariant is semantic validation, not merely JSON shape validation:

```text
set(requested_observations)
==
set(observation_outcomes.observation)
```

Duplicates, missing outcomes, and outcomes for unrequested observations are rejected. A produced result must reference the containing `run_id`.

This keeps observation production independent from top-level solver/execution completion. Adapter Protocol 0.3 will later transport execution and observation outcomes together; Phase 1 does not add or reinterpret execution status.

## Determinism and referential integrity

Canonical references use the existing SOL canonical-reference syntax. Requested observations and terminal outcomes are normalized into deterministic order. Non-production diagnostics are normalized deterministically after diagnostic shape validation.

The following are rejected by typed semantic validation even when their JSON shape is otherwise valid:

- a requested canonical observation replaced by a backend/vendor-named pseudo-observation;
- a produced result whose `run_id` differs from its containing run;
- missing or duplicate terminal outcomes;
- backend-native result/provenance fields inserted through extension data.

## Thermal Phase 1 fixture

The first positive scalar fixture is the accepted thermal observation:

```text
observation = observation.maximum_temperature
run         = run.thermal_reference_001
source      = thermal.temperature_field
scope       = scope.main_domain
value       = 400.0
unit        = unit.kelvin
```

A companion `not_produced` fixture proves that absence of the requested observation is represented explicitly rather than inferred from execution success or failure.

## Deferred boundaries

M0.10 Phase 1 deliberately does not choose:

- canonical field/dataset serialization format;
- dataset content digest/equality or normalization rules;
- `DatasetManifest` details;
- solver-neutral `DataReference` representation;
- Adapter Protocol 0.3 request/response layout;
- backend-native extraction/provenance mapping.

If implementation requires any of those choices, Operator must stop and return to Manager `meeting` rather than extending this Phase implicitly.
