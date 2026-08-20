# SOL v0.1 Thermal Machine Fixture — Independent Review v0.1

**Role:** Validation  
**Date:** 2026-08-20  
**Inputs:** ADR-0029, normalized Core/thermal reference packages, closed Thermal snapshot, snapshot/metrology validators and counterexample tests

## Overall verdict

**Revise — reference-model fixture only.**

No new Architecture defect, Constraint defect, package contract defect, or backend limitation is identified.

The ADR-0029 snapshot/schema direction and the committed normalized package environment are sufficient for deterministic validation. Two model instances in the current Thermal snapshot make semantic claims that are not connected by an accepted relation and are unnecessary for this intentionally minimal fixture.

## Accepted implementation boundaries

The following are accepted at readback level:

- `snapshot_state: closed` and exact ontology environment;
- unique model-instance identity contract;
- canonical EntityType/PropertyDefinition/RelationDefinition references;
- explicit PropertyAssignment rather than unlabeled Entity values;
- duplicate relation-triple rejection;
- canonical subtype-aware typed/allowed-pair endpoint validation;
- source-cardinality validation over applicable instances even when no edge is authored;
- normalized Core reference fixture with durable committed canonical IDs;
- Thermal extension types/properties with no backend-local identity;
- ADR-0020 Dimension Constraints applied to Thermal properties through ADR-0025 Interface Property requirements;
- focused metrology evidence remains external validation evidence, not a Core Unit registry;
- absence of backend/runtime/license state from all semantic inputs.

## TRV-01 — `SolidMaterial_1` has no accepted semantic association to conductivity

Current snapshot contains:

```text
ThermalMaterialModel --includes_component--> SolidMaterial_1
ThermalMaterialModel --includes_component--> ThermalConductivity_1
```

but no accepted Core relation states that `ThermalConductivity_1` is a property of `SolidMaterial_1`.

An independent validator must not infer that association merely because both happen to be the only Material and MaterialProperty in the same aggregate or because their local names suggest a connection.

For a multi-material case this inference would immediately become ambiguous.

**Classification:** Reference-model defect, not Architecture defect.

**Minimum remediation:** remove `SolidMaterial_1` from this minimal fixture. The reference question needs only a material/property parameter context, and `ThermalMaterialModel` containing the single specialized ThermalConductivityProperty is sufficient for the chosen stress case. A future domain model requiring explicit material-property ownership may justify a separate semantic relation through its own evidence; this fixture shall not invent one.

## TRV-02 — `HeatEquation_1` is under-specified and redundant in the minimal fixture

Current snapshot contains:

```text
HeatMathModel --includes_component--> TemperatureField_1
HeatMathModel --includes_component--> HeatEquation_1
```

but no accepted relation ties `HeatEquation_1` to the temperature field or encodes the Fourier equation semantics on that generic Equation instance.

The specialized `SteadyHeatMathModel` type already carries the reference-specific mathematical-model identity used by `represented_by` and `closed_by`. Adding a generic Equation instance with no further semantics does not strengthen the reference test and invites inference from its local name.

**Classification:** Reference-model defect, not Architecture defect.

**Minimum remediation:** remove `HeatEquation_1` and its membership edge from this deliberately small fixture. A future equation-AST/operator formulation layer can be tested separately if required.

## Dimension / metrology stress result

The Thermal extension correctly uses accepted mechanisms rather than new schema:

```text
ThermalConductivityProperty
  implements ThermalConductivityValueInterface
    property requirement -> thermal_conductivity_value
    Dimension Constraint -> M L T^-3 Theta^-1

FixedTemperatureCondition
  implements PrescribedTemperatureInterface
    property requirement -> prescribed_temperature
    Dimension Constraint -> Theta
```

The model values carry explicit UnitReferences (`si-ref:W_per_m_K`, `si-ref:K`). Focused metrology evidence resolves those references to DimensionVectors. Missing evidence is expected to produce `INDETERMINATE`; a resolved mismatched dimension is a definite semantic failure.

This provides an actual design-stage integration test of Property → ValueDefinition → Value → UnitReference → metrology DimensionVector against Interface-targeted Dimension Constraint without introducing a Core Unit entity.

## Backend sanity state

The previous official-document verdict remains unchanged:

- MOOSE: representable;
- COMSOL: representable;
- Ansys Mechanical: representable;
- aggregate realization classification: `transformed`, with no demonstrated semantic loss.

Installation, license and runtime absence remain out of scope.

## Next state

Return only TRV-01/TRV-02 to Research/reference-fixture revision.

Do not change ADR-0029 or introduce new Core relations for this minimal case.

After the two redundant instances/edges are removed, perform focused readback of:

1. snapshot structure;
2. endpoint/allowed-pair semantics;
3. cardinality;
4. Property/Interface/Dimension integration;
5. metrology PASS/INDETERMINATE/mismatch behavior.
