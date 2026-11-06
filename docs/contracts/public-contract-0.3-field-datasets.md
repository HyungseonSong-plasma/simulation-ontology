# Public Contract 0.3 — Bounded Field Dataset Boundary (Phase 2 candidate)

**Status:** Implementation candidate; D-01-R1 and D-02 architecture directions user-approved, detailed Validator gates OPEN.
**Parent:** #153; **Phase:** #156; **Architecture:** ADR-005.
**Source of authority:** [#156 decision record](https://github.com/HyungseonSong-plasma/simulation-ontology/issues/156).

This document describes a proposed first **solver-neutral** field representation. It
does **not** assert that raw MOOSE evidence has canonical quantity/unit/scope
mapping, and it does not close the independent 1D run or result transport gates.

## Categories and separation

- **ScalarObservationResultDtoV03:** unchanged inline scalar value + canonical unit
  and unchanged terminal outcome representation.
- **DatasetObservationResultDtoV03:** independent bounded field DTO with
  `(run_id, observation)` occurrence identity, source, scope, manifest and data reference.
- **DatasetManifestDtoV03:** scientific meaning/geometry/association/value shape.
  A backend-native filename/object/variable ID is never canonical identity.
- **DataReferenceDtoV03:** SOL-managed opaque `object_id`, explicit
  `payload_profile`, byte length, byte integrity evidence.
- **Backend source provenance:** out-of-band execution evidence, not a dataset
  object ID, semantic quantity or equality key.

Public Contract 0.1/0.2 and existing Public Contract 0.3 scalar result
serialization remain unchanged. This phase deliberately does **not** add
Adapter Protocol 0.3 transport or reinterpret the 0.2 execution flow.

## Bounded v1 coverage

| Property | Supported |
| --- | --- |
| Coordinate embedding dimension | 1 or 2 |
| Field carrier/topological dimension | 1 or 2, no higher than embedding |
| Field entity association | `vertex` (entity dimension 0), `cell` (carrier dimension) |
| Cell types | `line2`, `tri3`, `quad4` |
| Field shape | scalar, rank 0, empty component array |
| Observation sampling | stationary, or one `time_point` with value and canonical time unit |
| Connectivity | zero-based, ordered vertex indices; no degenerate repeated indices |
| Values | one JSON number per support entity, positions given by support indices |
| Storage | explicitly versioned interchange `sol.field-json.v1` |

Not yet supported: 3D, higher-order elements/DoFs, vector/tensor, quadrature
fields, generic cell faces, complex-valued fields, multi-snapshot dataset in
one result, arbitrary index canonicalization, any general physical-unit
conversion, or cross-solver dataset content equality. Unsupported cases must
fail explicitly rather than be coerced into vertex/cell fields.

Embedding dimension describes the coordinate frame; carrier dimension is the
field-support mesh topology; entity dimension describes the associated mesh
entity; rank describes the value components. These axes MUST NOT be merged.

Examples:
- Independent 1D line-mesh LAGRANGE scalar: embedding=1, carrier=1,
  location=vertex, entity dimension=0.
- 2D volume LAGRANGE scalar: embedding=2, carrier=2,
  location=vertex, entity dimension=0.
- Lower-dimensional surface/interface MONOMIAL CONSTANT scalar in a 2D mesh:
  embedding=2, carrier=1, location=cell, entity dimension=1.

## Dataset manifest and referential checks

`DatasetObservationResultDtoV03` contains canonical
`observation`, `run_id`, `source`, and `scope`.
`DatasetManifestDtoV03` requires:
- canonical `quantity` and `unit`;
- `geometry.coordinate_system`, coordinate unit per embedding axis, embedding
  dimension and topological dimension;
- `support.scope` equal to result scope; carrier dimension, entity
  dimension, association `vertex|cell` and entity count;
- rank/components/value count;
- stationary or time-point sampling; and
- coordinate count, cell count, accepted element types and index base 0.

DTO validation checks **reference syntax**, cardinalities, dimension
relationships, supported cell types, index bounds and duplicate indices.
It does not independently prove ontology reference resolution or unit validity
for a real solver variable; that remains a separate validator/Physics-owner
gate. No backend-native or unknown fields are accepted in the bounded DTO.

## Data reference and bytes

`DataReferenceDtoV03` contains `object_id` (SOL-managed reference, not a MOOSE
path), `payload_profile`, `byte_length`, and `integrity.sha256` lowercase
hex digest.

The bounded v1 DataReference `object_id` must be a canonical-looking locator in the SOL-managed `data.` namespace; this namespace is not physical-result identity. Syntactic namespace validation alone is not authority: the resolver must independently confirm the object is actually SOL-managed and access is permitted.

The DTO validator checks integrity metadata shape. A **resolving transport/storage
component MUST** retrieve the named bytes, enforce access authorization,
require exact byte length, verify SHA-256 against actual bytes, deserialize the
declared payload profile, and then call structural payload validation. The
current Phase 2 DTO does not implement a storage resolver or verify a stored
payload digest by itself; no implementation may report fully resolved
`produced` from DTO metadata alone.

`sol.field-json.v1` has a standalone schema with arrays:
`coordinates`, `cells[{type, vertices}]`, `support_indices`, `values`.
`support_indices[i]` identifies the vertex or cell carrying `values[i]`,
per the manifest's `location`. `index_base=0`.
The explicit v1 byte representation emits compact JSON with recursively sorted
object keys; array order is preserved and numeric JSON types are not silently
converted. Canonical physical-content identity and topology/coordinate
permutation normalization are **deferred**. Equal byte digests do not imply
scientifically equivalent results, and changed digests do not imply different
observation intent.

## Thermal Temperature Field Observation — bounded opt-in candidate

**Status:** User-directed Phase 2 implementation candidate (2026-11-06); **Human Validator and Physics meaning gates OPEN**. The accepted existing Thermal reference fixture, Public Contract 0.1/0.2 and Phase 1 scalar-only 0.3 wire/Rust DTOs are **unchanged**. This section is not a new accepted universal ontology rule.

### Requested meaning versus source Field

- Mathematical/source Field: `thermal.temperature_field` (pre-existing mathematical-model entity).
- Existing compact ObservationModel: `observation.maximum_temperature` (`MaximumTemperature`), a scalar reduction over the requested domain.
- **New bounded candidate ObservationModel:** `observation.temperature_field` (`TemperatureFieldObservation`), requesting the distributed, spatially associated temperature values on an explicit scope. It is **not** the underlying mathematical Field itself and is not interchangeable with the maximum-temperature scalar.
- Both ObservationModels observe the **same** mathematical Field through existing `observed_by` vocabulary (source `thermal.temperature_field` to each ObservationModel); no new relation kind is introduced.
- Candidate source-binding: `quantity.temperature`, `unit.kelvin`, `scope.main_domain`, `coordinate.cartesian_1d`, `unit.meter`, vertex-associated scalar LINE2, `stationary` sampling. **Actual canonical reference authority/physical-unit mapping requires Physics/SOL human review**; names in this section are candidate bindings, not proof that a source-native Exodus variable is Kelvin.

### One Run, two independently requested terminal outcomes

A result-capable *opt-in* Public Contract 0.3 Run can contain exactly these two **distinct** requested observation references:

```
run.synthetic_thermal_1d
  requested_observations:
    - observation.maximum_temperature  -> Scalar result (inline)
    - observation.temperature_field     -> Dataset result (Manifest + DataReference)
```

The result occurrences are distinct `(run_id, observation)` pairs even though the run ID, Field source and scope can be shared. **One outcome per requested observation** remains mandatory. Neither producing the Scalar nor producing the Dataset implies the other was produced. If either requested result is absent, return its own typed `not_produced` evidence; do not invent a missing result from solver completion or from the other observation. No implicit request or automatic reduction/expansion is introduced.

An actual `MaximumTemperature` is a reduction/aggregation of the final temperature Field over the specified domain (not necessarily an arbitrary native postprocessor label). Here the synthetic scalar is 400 and synthetic nodal values are `[400,400,400]`. This is a **conformance example only**; it does **not** assert that an adapter has emitted a Public Contract 0.3 result from actual MOOSE output.

### Candidate fixtures and adversarial checks

- `fixtures/thermal/thermal-reference-field-observations-candidate.json` — separate ontology candidate with two ObservationModels and two `observed_by` edges; the original `fixtures/thermal/thermal-reference.json` remains intact.
- `fixtures/public-contract/0.3/thermal-mixed-run-synthetic-candidate.json` — opt-in mixed run, one Scalar `produced`, one Dataset `produced`, exact same run/source/scope.
- `fixtures/public-contract/0.3/thermal-field-payload-synthetic-candidate.json` — three-node/2-element **synthetic** field with `sol.field-json.v1` canonical byte-integrity metadata.
- `crates/sol-public-contract/tests/thermal_field_observation_candidate.rs` — actual typed DTO parsing and in-memory verified-publication tests under a **test-only, manually pinned FixtureAuthority**; rejects duplicate/missing/unrequested/wrong-run outcomes, wrong field meaning, scope, time sampling, value counts and absent dataset marker; tests independently typed `not_produced`.

These tests verify source-level structural semantics *under the explicitly artificial authority*, not scientific approval of actual MOOSE data, production resolver authorization, Phase 3 Adapter Protocol 0.3, or Phase 4 transport. A separate real `TemperatureFieldObservation` producer, authorized SOL-managed resolver and end-to-end Physics-signed mapping are later work.

**Gates:** Physics/Adapter sign-off [#39](https://github.com/HyungseonSong-plasma/sol-adapter-moose/issues/39); human independent Validator [SOL #175](https://github.com/HyungseonSong-plasma/simulation-ontology/issues/175); final user acceptance, CI at exact final head and existing GitHub review/merge rules. Do not reclassify this candidate as accepted simply because the synthetic tests pass.

## Actual-source evidence vs publishable SOL fixtures

The repository includes:
- `fixtures/source-evidence/moose/m010/field_2d_potential_exodus.json`
  (1,721 nodal values, 3,155 TRI3 elements);
- `fixtures/source-evidence/moose/m010/field_1d_interface_sigma_s_exodus.json`
  (53 elemental values, 53 EDGE2 elements, 54 extracted vertices); and
- `fixtures/source-evidence/moose/m010/source_provenance.json`.

Origin: `moose-test-repo` run 41739266674, artifact 13296715986, `run/plasma.e`,
source SHA-256 `ae62510bcff30d68759633e099be9fa75164cda69dd893c7ec2481885799223c`.
The stored state index is 460, time value 0.0001 **as stored**, without an
established unit. They are **source-preserving evidence only**. The native
Exodus field extraction is not an independent 1D simulation; `sigma_s` is
a one-dimensional embedded interface within a 2D simulation. The evidence
includes backend field/block names for traceability, and these MUST NOT be
copied into canonical DTO semantics.

The separate `fixtures/public-contract/0.3/dataset-*-synthetic.json` and
`field-payload-*-synthetic.json` contain explicit **synthetic**, known
quantities/units. These are useful for contract compatibility and schema
tests but MUST NOT be reported as MOOSE scientific output.

## D-02 — Explicit mixed-result opt-in and verified publication

**Decision:** D-02 A approved in Manager meeting on 2026-11-05.
**Implementation status:** Phase 2 candidate; Validator and Physics gates remain open.

- The old `ObservationOutcomeDtoV03`, `ScalarObservationResultDtoV03`,
  `SimulationRunRecordDtoV03` and `simulation-run-record.schema.json` keep
  their original scalar-only JSON/Rust semantics and public API.
- Consumers that explicitly opt in use `DatasetCapableSimulationRunRecordDtoV03`
  and `dataset-capable-run-record.schema.json`. Scalar outcomes remain
  `{"status":"produced","result":{...}}` without a `result_type` field.
  Dataset outcomes are
  `{"status":"produced","result_type":"dataset","result":{...}}`.
  `not_produced` retains its former diagnostic shape.
- The opt-in decoder rejects missing/unknown Dataset markers, ambiguous
  scalar/dataset hybrids, duplicates, missing/unrequested observation outcomes,
  nonmatching run IDs and backend-native leakage. The scalar-only decoder
  must reject Dataset results rather than silently change meaning.
- Parsing the DTO or computing a manifest hash is NOT publication. The explicit
  `publish_verified(resolver, semantic_authority)` operation checks exact
  requested-observation coverage, source/quantity/unit/scope coordinate
  semantic mappings supplied by a trusted authority, authorization to
  retrieve SOL-governed bytes, exact length, actual SHA-256, canonical
  `sol.field-json.v1` byte encoding, and payload shape/topology.
- `FieldSemanticAuthorityV03` is a dependency contract for an authoritative
  ontology/Physics reference resolver. The test-only `FixtureOntology`
  validates *synthetic* mappings; the real MOOSE field mapping remains
  unapproved. An arbitrary implementer returning success is not evidence of
  authoritative scientific mapping.
- `InMemoryFieldPayloadResolverV03` is a fail-closed, immutable-object
  conformance implementation (explicit grant, access denial, missing object,
  byte corruption, digest/shape errors). It is not production storage.
- Only `PublishedDatasetCapableRunRecordV03` is a verified publication
  artifact from the new API. `to_canonical_json_unverified` is intentionally
  a parser/inspection representation, not authority to publish.
- A **subsequent** `resolve_verified_field` error (`AccessDenied`,
  `ObjectNotFound`, `ByteLengthMismatch`, `IntegrityMismatch`, etc.) is a
  **DataResolutionErrorV03**. It does not mutate a previously published
  historical `produced` into `not_produced`.
- A SHA-256 digest validates *bytes*, not scientific content equality.
  The bounded SHA-256 implementation has published known-vector tests.
  Production storage security/integrity policy and reuse of audited
  cryptographic libraries remain downstream implementation review.

### Still-required gates

1. Independent real MOOSE 1D field and 1D thermal field fixture; do not
   substitute an embedded 1D interface.
2. Physics-authoritative coordinate system, quantity, unit, time and scope
   mapping for the real MOOSE source evidence.
3. Production resolver access/retention, scope-matching and immutable storage
   semantics, and integration with the Phase 3 0.3 transport boundary.
4. Exact-head CI, counterexample review and Validator final acceptance.
## Missing Validator and implementation gates

1. Confirm real source quantity/unit, coordinate frame/units, time units and
   canonical scope/reference resolution with Physics owner; retain explicit
   backend provenance elsewhere.
2. Obtain an **independent** 1D MOOSE simulation's real field output. Do not
   substitute the extracted 1D interface as independent 1D evidence.
3. Verify byte resolver authority/access policies, missing/corrupt/payload
   mismatch failures and hash verification against the actual resolved bytes.
4. Integrate `DatasetObservationResult` into the already accepted exact-coverage
   terminal `ObservationOutcome` path **without breaking existing scalar
   clients**. Current bounded DTO is independently parseable but not yet
   integrated into `SimulationRunRecordDtoV03`.
5. Validate additional backend leakage cases, topology/permutation cases,
   dataset-source and time-reference matching, and cross-version regression.
6. Prove exact-head CI, explicit Validator approval, merged `main` and
   Phase #156 acceptance; only then close the issue.

The first thermal field requirement from #153 / ADR-005 remains in force.
This implementation candidate should be reviewed rather than automatically
promoted to a published complete result contract.
