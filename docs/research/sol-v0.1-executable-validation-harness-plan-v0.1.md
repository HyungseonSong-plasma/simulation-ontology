# SOL v0.1 Executable Validation Harness Plan v0.1

**Status:** Research implementation plan  
**Date:** 2026-08-20

## Objective

Produce the minimum executable evidence required to test Proposed ADR-0011 and ADR-0012 before SOL v0.1 architecture freeze. Contract acceptance and ADR drafting are complete; implementation evidence is not.

## Repository baseline

The repository currently contains documentation plus scaffold directories for `schema/`, `tests/`, `examples/`, and ontology packages, but no executable validation harness. Therefore executable validation begins with a small non-normative reference implementation rather than assuming an existing runtime.

## Phase 1 — machine-readable contract harness

Implement:

1. JSON Schema for QRC authoring shape and normalized validation inputs.
2. JSON Schema for MappingRule five-field and MappingClaim four-field envelopes.
3. JSON Schema for RealizationEffect, PlanAction, comparator registry entries, BackendTargetRequirement/ResolvedBackendTarget, and evaluation records.
4. A non-normative reference validator implementing:
   - `FAIL > BLOCKED > INDETERMINATE > PASS` aggregation;
   - QRC bound normalization, closed-snapshot requirement, stable-identity counting, qualifier/type closure handling;
   - comparator exactly-one registry resolution;
   - local versus cross-component comparison context;
   - complete effect-pair enumeration and proof-only pruning;
   - producer/prerequisite/cycle checks;
   - component-keyed target resolution boundaries.
5. Deterministic fixtures proving declaration/serialization/component order independence.

Reference implementation language is tooling-only and SHALL NOT become part of the SOL language contract.

### Phase-1 PASS criteria

- MappingRule with any sixth top-level field fails structural validation when the envelope is configured closed.
- MappingClaim with any fifth top-level field fails structural validation when the envelope is configured closed.
- the same semantic fixture under different serialization/order permutations yields the same canonical decision/evidence set;
- comparator zero or multiple matches always FAIL;
- unproved-disjoint effects are retained;
- local/cross-component comparator context is unique;
- mixed `FAIL + BLOCKED` aggregates to FAIL;
- QRC open snapshot => `BLOCKED: QRC_CLOSED_SNAPSHOT_REQUIRED`;
- QRC `min:1,max:3,exact:2` normalizes to `[2,2]`;
- QRC `min:3,exact:2` => `FAIL: QRC_EMPTY_INTERVAL`;
- duplicate relation edges to the same stable identity count once.

## Phase 2 — Thermal MOOSE executable fixture

Create one concrete transient heat-conduction SOL instance and one MOOSE target Profile/Adapter fixture that exercises:

- variable/field realization;
- heat-conduction term;
- transient time-derivative term;
- material/property dependencies;
- boundary condition;
- transient execution configuration;
- PlanAction prerequisite DAG;
- effect coverage and idempotency evidence.

Generate a MOOSE input artifact from the MappingPlan. Execute it against a pinned MOOSE release when a compatible runtime is available. Record command, release/build identity, generated input checksum, exit status, and minimal result evidence.

### Thermal PASS criteria

- canonical plan is order-independent;
- prerequisite/producer graph is acyclic and complete;
- generated artifact contains the required transient equation contribution rather than only a transient execution controller;
- repeated planning is isomorphic;
- execution succeeds on the declared release;
- observed adapter mutation/artifact surface matches validated effects.

## Phase 3 — Plasma executable fixture

Use one COMSOL or Ansys plasma path with an available executable/API runtime. The fixture SHALL exercise:

- N:1 or otherwise nontrivial MappingClaim composition;
- selection/resource alias overlap;
- QRC negative-ion product constraint;
- backend formulation binding;
- release/module/capability resolution;
- where applicable, component/orchestration binding.

### Plasma PASS criteria

- four-field MappingClaim preserves stable source/provenance without hidden side channel;
- cross-claim collision is detected through normalized effects;
- QRC duplicate edge and multiple-typing cases remain deterministic;
- unsupported formulation/module and unresolved runtime evidence separate into `unsupported` versus `BLOCKED` as contracted;
- generated backend artifact/API operations execute successfully on the declared target.

## Phase 4 — independent execution validation

Validation role SHALL consume only the accepted/proposed ADR contracts, machine-readable implementation artifacts, immutable fixtures, generated backend artifacts, and execution evidence. Research narrative and prior PASS conclusions are excluded.

For each requirement classify:

```text
PASS / FAIL / BLOCKED / INDETERMINATE
```

and defect ownership:

```text
Architecture defect
Profile defect
Adapter defect
Backend limitation
Reference-model defect
Validation-tooling defect
```

Architecture freeze may be reconsidered only when the minimum Thermal and Plasma execution evidence is complete and no architecture-level unresolved finding remains.

## Current feasibility

Phase 1 is executable without proprietary backend installations. Phases 2 and 3 require actual backend runtimes for final execution evidence. Dry-run/generated artifacts are useful intermediate evidence but SHALL NOT be labeled backend execution validation.

## Research verdict

Proceed with Phase 1 immediately. Keep ADR-0011 and ADR-0012 `Proposed` and SOL v0.1 architecture freeze blocked until independent execution validation closes the remaining gate.
