# M0.10 Thermal 1D source/result mapping — evidence and adversarial pre-review

**Status:** `SOURCE_TECHNICAL_PRECHECK_PASS / SCIENTIFIC_SIGNOFF_OPEN / INDEPENDENT_VALIDATOR_OPEN`  
**Authority:** Non-normative review packet only; **not** an accepted Physics mapping, Validator verdict, canonical result fixture, or approval to merge/release.
**Related:** SOL #153 / #156 / [Validator task #175](https://github.com/HyungseonSong-plasma/simulation-ontology/issues/175); [Adapter/Physics task #39](https://github.com/HyungseonSong-plasma/sol-adapter-moose/issues/39); SOL Draft PR #173; Adapter Draft PR #38.

## Pinned source evidence

| Component | Exact evidence | Status |
| --- | --- | --- |
| Adapter code/request | `sol-adapter-moose@28b15c351623b4ff05dd589e6bbf5624ecafae12`, `src/realization_v02.rs`, `tests/fixtures/sol/0.2/thermal-realization-request.json` | Source inspected |
| Actual MOOSE execution | [GitHub CI 42200001106](https://github.com/HyungseonSong-plasma/sol-adapter-moose/actions/runs/42200001106), real `moose-backend` job SUCCESS | Backend run PASS |
| Artifact | `13493163467`, `moose-backend-evidence`, ZIP 28,532 B, SHA-256 `fb629aac9f74b7479a511bd3b71c9926225375fea02974afacc1277d20695e38` | Native bytes read, ZIP CRC PASS |
| Exodus | `phase5-vv-evidence/runs/thermal-constant-field-vv/input_out.e`, 44,672 B, SHA-256 `8915ceb43c1c26a16d4dec537f26cf32c47c2cdaf3f4f750505cc5b0fdfcdcd6` | Native bytes read |
| Same-run scalar | `input_out.csv`, `temperature_max_vv=400.00000000001`, `temperature_min_vv=400` | Matches Field |
| Solver-numerical | `Steady` and `Solve Converged!`, 1D mesh of 5 nodes/4 LINE2 elements, first-order LAGRANGE `T` | Backend execution PASS |
| SOL candidate check | [Source-only provenance JSON](../../fixtures/source-evidence/moose/m010/thermal_1d_same_run_scalar_field_provenance.json), [structural metadata validation](../../scripts/validate_m010_source_evidence.py) | CI source-metadata PASS at prior exact head |

No native files, personal Drive addresses, or solver-native names become SOL semantic identity. The source ZIP was archived separately; this document does **not** verify an organization-owned official retention policy.

## Source mapping proposal / Physics-owner decision required

| Axis | Explicit upper-layer source | Observed backend state | Decision requirement |
| --- | --- | --- | --- |
| Physical equation | `ThermalTransport`, `SteadyHeatEquation`, `FourierLaw`, `ThermalConductivity` | MOOSE `HeatConduction` + constant conductivity and `Steady` | Physics owner confirm this exact bounded test, assumptions, applicability |
| Temperature quantity/unit | `thermal.temperature_field`, `thermal.temperature=400 unit.kelvin` | MOOSE LAGRANGE FIRST `T`; left Dirichlet value 400 | Explicitly approve native `T` -> SOL temperature/K mapping; no inferred physical unit from Exodus |
| Length/coordinate | `LineDomain1D` length 1 `unit.meter`; `BoundaryPoint1D` position 0 `unit.meter` | x=[0,0.25,0.5,0.75,1], y=z=0 | Confirm 1D Cartesian/metre coordinate frame and no silent transform |
| Thermal conductivity | `thermal.conductivity=45 unit.watt_per_meter_kelvin` | MOOSE `thermal_conductivity=45` property | Confirm W/(m K) and consistent unit conversion policy |
| Source scope | `scope.main_domain` contains `domain.main` | Full 1D line four segments | Approve complete-domain field support, not hot-wall-only support |
| Boundary scope | `scope.hot_wall` contains `boundary.hot_wall` | MOOSE left boundary x=0, natural right flux | Approve hot-wall mapping, right BC and physical assumptions |
| Association/topology | Candidate dataset v1 vertex/LINE2 scalar field | 5 nodal T values, 4 LINE2 cells | Confirm association=vertex, embedding/topological dimension=1, entity_dimension=0 |
| Sampling | `StationaryAnalysis` | MOOSE `Steady`; native Exodus output time records [0,1] | Approve `stationary`; **do not** claim measured time=1 second |
| Scalar observation | `observation.maximum_temperature` / `MaximumTemperature` | `temperature_max_vv` is **test-only** NodalExtremeValue, max over final nodal T values | Physics + SOL verify aggregate meaning and a real 0.3 result producer, not just test CSV |

At final state, T nodal values are [400.0, 400.00000000001097, 400.00000000001097, 400.00000000001097, 400.00000000001097]. Source-native Field maximum 400.00000000001097 and CSV maximum 400.00000000001 differ only by floating-point output precision. The absolute maximum deviation from the constant 400 reference is `1.0970779840135947e-11`, below the existing V&V `1e-8` tolerance. **No numerical conclusion for unrelated thermal/plasma cases is implied.**

MOOSE official framework references:
- HeatConduction: https://mooseframework.inl.gov/moose/source/kernels/HeatConduction.html
- NodalExtremeValue: https://mooseframework.inl.gov/source/postprocessors/NodalExtremeValue.html
- Official MOOSE materials have no automatic SI unit inference for arbitrary native variable values; the SOL `RealizationSpec` provides the unit commitment in this bounded case.

## Adversarial precheck — not an independent external review

An assistant-authored, **read-only** Python check against the original native ZIP performed **27 passing inspections**:
- 11 source/geometry/solver/field/scalar/analytic checks with actual source bytes;
- 15 deliberately invalid **candidate mapping-policy** cases rejected (wrong Kelvin/coordinate/conductivity unit, quantity, scope, support association/dimension, sampling, backend-native ID leakage, initial/final state, mismatched max/min, truncated field and missing terminal coverage);
- 1 explicit BLOCK for missing independent Temperature Field ObservationModel.

**Crucial limitation:** the injected negative cases challenge a separately written candidate policy, **not production Public Contract / Adapter Protocol 0.3 runtime**. These are helpful to a reviewer but do not fulfill the independent Validator criterion, no matter how many checks pass. Do not use this report as a GitHub `APPROVE` verdict.

## Unresolved semantic gates

1. `thermal.temperature_field` in the reference ontology is a Field model/source; the scalar run fixture only explicitly requests `observation.maximum_temperature`. Define and accept a distinct requested **Temperature Field ObservationModel** and identity before publishing two `produced` outcomes. Candidate names are not automatically normative.
2. `src/realization_v02.rs` currently verifies presence of `MaximumTemperature` but does not itself emit a Public Contract 0.3 scalar result. `temperature_max_vv` is test-owned V&V instrumentation.
3. Public Contract 0.3 mixed-result publication requires trusted canonical meaning authority and real authorized `data.*` resolver; raw native Exodus/artifact or source-only provenance is insufficient.
4. Scientific/Physics owner must submit distinct traceable `ACCEPT / REVISE / BLOCK` verdict to Adapter #39. Independent SOL Validator must then submit distinct `APPROVE / REQUEST_CHANGES / BLOCKED` verdict to SOL #175 at latest pinned HEAD, checking actual producer code and negative behavior. The same assistant's analysis is not an independent reviewer identity.
5. Existing 0.1/0.2 compatibility, original scalar wire, result terminal exact coverage, no-replay and no implicit v0.2→v0.3 promotion remain frozen.
6. User final acceptance comes **after** owner and independent Validator evidence. Draft PRs, merge rules, official reviews, project-owned retention remain separate formal gates.

## Explicit status

**Technical source check:** PASS.  
**Physical semantic mapping sign-off:** OPEN.  
**Independent Validator PR sign-off:** OPEN (zero submitted at handoff).  
**Canonical output publication/production resolver:** NOT IMPLEMENTED in this source evidence.  
**User final scientific/contract acceptance:** NOT REQUESTED.  
