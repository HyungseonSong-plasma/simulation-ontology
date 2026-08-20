# SOL v0.1 Thermal Machine Fixture — Focused Final Review

**Role:** Validation  
**Date:** 2026-08-20  
**Inputs:** ADR-0029; committed normalized Core/Thermal package fixtures; closed Thermal snapshot; snapshot, Interface/Dimension/metrology semantic helpers and counterexample tests  
**Previous findings:** TRV-01, TRV-02

## Verdict

**ACCEPT for SOL v0.1 design-stage reference validation.**

## Finding closure

### TRV-01

`SolidMaterial_1` and its membership edge have been removed. The fixture no longer implies an unmodeled Material-to-MaterialProperty ownership relation by shared aggregate membership.

### TRV-02

`HeatEquation_1` and its membership edge have been removed. The reference-specific `SteadyHeatMathModel` remains the mathematical-model identity and no generic Equation instance relies on its local name for unmodeled semantics.

No regression resulted from either subtraction.

## Canonical environment readback

The reference environment now contains:

```text
@simulation-ontology/core-reference-fixture@0.1.0
@simulation-ontology/thermal-reference@0.1.0
```

The Thermal package has an exact dependency on the Core fixture. Both use ADR-0028 normalized package/resource forms and durable committed canonical IDs. The Core fixture preserves complete accepted semantics for each relation it includes, including the full ADR-0016 `includes_component` allowed-pair matrix rather than a Thermal-only narrowing.

The Thermal extension introduces only solver-independent specializations and semantic resources. No MOOSE block type, COMSOL feature tag, Ansys object identifier, backend release, or backend-local handle appears as Core/reference semantic identity.

## Closed model snapshot readback

The final Thermal snapshot contains:

- 17 model Entity instances;
- 25 unique Relation triples;
- exact package environment;
- explicit PropertyAssignments for conductivity and two prescribed temperatures;
- no duplicate model-instance ID;
- no duplicate relation triple;
- no backend-local identity field.

The graph exercises:

```text
has_model
has_task
uses_model
has_analysis
includes_component
represented_by
closed_by
parameterized_by
defined_on
applied_to
```

Subtype-aware endpoint matching is necessary for Thermal specializations and StationaryAnalysis, and the committed validator performs that closure before endpoint/cardinality evaluation.

Closed-snapshot source-cardinality validation evaluates applicable sources even when an edge is absent, so missing `has_model`, duplicate `has_model`, and a FixedTemperatureCondition with no `applied_to` target are explicit counterexamples rather than silently ignored omissions.

## Value / Unit / Dimension integration

The Thermal extension uses ADR-0025 rather than adding a new PropertyDefinition constraint field:

```text
ThermalConductivityProperty
  implements ThermalConductivityValueInterface
  -> thermal_conductivity_value Property requirement
  -> Dimension Constraint M L T^-3 Theta^-1

FixedTemperatureCondition
  implements PrescribedTemperatureInterface
  -> prescribed_temperature Property requirement
  -> Dimension Constraint Theta^1
```

The model assigns normalized literal Values through those PropertyDefinitions:

```text
10 W/(m K)
300 K
400 K
```

Focused metrology evidence resolves the UnitReferences to canonical DimensionVectors for the stress test. The reference metrology helper preserves the accepted state distinction:

- complete matching evidence -> `PASS`;
- unavailable required metrology evidence -> `INDETERMINATE`;
- resolved dimension contradiction -> definite semantic failure.

The evidence table is explicitly not a Core Unit registry.

## Counterexample coverage

Committed tests cover at least:

- ontology-environment mismatch;
- duplicate model Entity identity;
- unresolved EntityType;
- duplicate/unknown PropertyDefinition assignment;
- duplicate relation triple;
- illegal `includes_component` pair;
- missing and excessive `has_model` cardinality;
- missing `applied_to` for an applicable condition even when no edge exists;
- backend-local relation token rejection;
- declaration-order invariance;
- missing metrology evidence -> `INDETERMINATE`;
- resolved Unit/Dimension mismatch -> failure;
- normalized snapshot structural closure and canonical-reference shape.

A supplemental semantic-equivalent local smoke over the committed fixture shape recorded 17 entities / 25 triples with endpoint, allowed-pair, cardinality, Interface, and metrology checks passing.

## Backend official-document verdict

The previous independent official-document sanity conclusion remains unchanged:

| Backend | Thermal semantic support | Design-stage projection judgment |
|---|---|---|
| MOOSE | supported | transformed |
| COMSOL | supported | transformed |
| Ansys Mechanical | supported | transformed |

No demonstrated semantic loss or unsupported obligation is present in this minimal case.

## Operational execution boundary

A repository-root test run from a separately cloned execution workspace is not treated as a design-stage domain prerequisite. GitHub artifact access and persistence are available through the connected GitHub integration; external checkout/network restrictions, backend installation, license state, and solver execution remain operational/integration concerns.

This does not weaken the artifact-level contract verdict. Production execution V&V remains deferred to Adapter/backend projects.

## Final Thermal verdict

| Dimension | Verdict |
|---|---|
| Core semantic expressibility | **Accept** |
| Canonical package environment | **Accept** |
| Closed model snapshot contract | **Accept** |
| Relation/cardinality semantics | **Accept** |
| Value/Unit/Dimension integration | **Accept** |
| MOOSE/COMSOL/Ansys official-doc sanity | **Accept** |
| New Architecture defect | **None** |

**Thermal reference gate: PASS.**

## Next state

Proceed to the planned **Minimal Plasma/QRC stress model** using the same ADR-0028 package environment model and ADR-0029 closed snapshot contract. The Plasma case should specifically stress QRC, multiple subtype targets, semantic dependencies/couplings, and representability classification without expanding into backend runtime/Adapter validation.
