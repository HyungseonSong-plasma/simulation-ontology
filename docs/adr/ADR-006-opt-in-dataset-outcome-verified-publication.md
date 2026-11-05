# ADR-006 — Explicit Dataset Outcome Opt-In and Verified Publication

**Status:** User-approved architectural direction; implementation candidate under Validator gates
**Decision:** D-02 A, 2026-11-05
**Scope:** M0.10 Phase 2 (#156); Draft PR #173
**Baseline:** Accepted ADR-005 and completed Phase 1 (#155)

## Context

Public Contract 0.3 Phase 1 publishes inline scalar `produced` results
through `ObservationOutcomeDtoV03` and `SimulationRunRecordDtoV03`.
Phase 2 introduces a separate `DatasetObservationResultDtoV03` with
`DatasetManifest` and `DataReference`. Changing the existing
`Produced { result: ScalarObservationResultDtoV03 }` Rust variant would
break source consumers, while treating `produced` as merely metadata-validated
would silently promote potentially unreadable or corrupted field data.

## Decision

1. **Do not mutate the existing scalar contract.** The published scalar
   JSON/Rust DTO and scalar-only run schema keep the exact Phase 1 meaning.
   Older scalar-only parsers are not implicitly upgraded to Dataset-capable.
2. **Explicit opt-in:** publish a separately named
   `DatasetCapableSimulationRunRecordDtoV03` and
   `dataset-capable-run-record.schema.json`. A scalar produced outcome has
   the original `status: produced` and scalar result fields with no type tag.
   A dataset produced outcome uses `status: produced`,
   `result_type: dataset`, and the full Dataset result. `not_produced`
   remains the same terminal status with typed diagnostic evidence.
3. **One terminal result per requested observation** regardless of result
   category, with duplicate, missing, unrequested, mismatched-run and
   ambiguous/hybrid-result cases rejected.
4. **Verified publication is a distinct operation:** after canonical
   reference/meaning validation from an authoritative ontology/Physics
   binding, the SOL-managed resolver must authorize access, retrieve bytes,
   verify exact byte length and actual SHA-256, decode the versioned
   interchange payload and validate the values/topology/association
   against the manifest. Only then may the result be issued as `produced`.
   Parsing a DTO or checking digest *format* alone is insufficient.
5. **Historical result vs later retrieval:** after successful publication,
   later resolver failure (unavailable, denied, changed/corrupt bytes)
   yields a typed resolution error and does not retroactively rewrite
   the historical terminal outcome as `not_produced`.
6. This is a **bounded Phase 2 contract**. The in-memory resolver is
   conformance machinery; external object storage, Adapter Protocol 0.3
   transport and real-adapter integration belong to later phases.

## Rejected / deferred alternatives

- Replacing the legacy `Produced.result` field with a Rust union in place:
  rejected due to Rust source compatibility break.
- Adding `produced_dataset` as a new terminal status: rejected because
  result *category* must not redefine produced-versus-not-produced semantics.
- Implicitly parsing a Dataset as an ordinary scalar result: rejected.
- Elevating raw SHA-256 to solver-independent physical-content identity:
  rejected; byte integrity is not semantic equality.
- Automatic adoption by existing Public Contract / Protocol consumers:
  rejected; explicit capability/decoder selection is required.
- New Public Contract 0.4 axis: not adopted by D-02; reopen if an explicit
  0.3 opt-in cannot be guaranteed to fail closed for an affected consumer.

## Validator acceptance gates

1. Legacy scalar JSON and the original Rust types are unchanged and
   golden fixtures remain byte-equivalent after deterministic normalization.
2. Mixed scalar/dataset/non-produced examples have exact terminal coverage
   and fail closed for unknown/missing type markers, ambiguous hybrid
   payloads, duplicate outcomes, wrong run, missing observation, and
   backend-native field insertion.
3. Authenticated/authorized resolution checks actual bytes, length, SHA-256,
   exact v1 payload encoding, indexing, topology, support association and
   value cardinality. Unauthorized, missing and corrupted data block
   publication.
4. Only a trusted, explicit semantic authority can confirm canonical
   quantity/unit/scope/source/coordinates for real MOOSE evidence. Synthetic
   mappings or reference-syntax checks are insufficient for scientific PASS.
5. A later lookup error leaves the historically published result unchanged.
6. Confirm independent real 1D results, original thermal first slice,
   0.1/0.2 regressions and exact-head CI before Phase exit.

## Compatibility consequences

- This is an additive, **explicitly opted-into Public Contract 0.3 surface**;
  `Adapter Protocol 0.3` negotiation remains a later Phase 3 task.
- No reinterpretation of Public Contract 0.1/0.2 or Adapter Protocol 0.1/0.2.
- Dataset content equality/normalization across MOOSE, COMSOL and ANSYS
  remains deferred, as in D-01-R1.

## Status limitation

User-approved D-02 architecture is a direction decision, **not** a claim
that PR #173 is ready to merge or that Validator/Physics gates are closed.
Any contradiction between this implementation and accepted ADR-005
requires returning to Manager meeting.
