# SOL v0.1 Thermal Machine Fixture Revision v0.2

**Role:** Research  
**Date:** 2026-08-20  
**Input findings:** TRV-01, TRV-02  
**Base:** ADR-0029 + committed Thermal machine fixture v0.1

## Scope

No architecture, package, snapshot, Interface, Value, Dimension, or backend-mapping contract is reopened.

The revision removes only two under-specified model instances from the deliberately minimal Thermal fixture.

## TRV-01

Remove:

```text
SolidMaterial_1 : Material
ThermalMaterialModel --includes_component--> SolidMaterial_1
```

Reason: the fixture has no accepted relation binding that Material instance to `ThermalConductivity_1`. Shared aggregate membership shall not be interpreted as property ownership.

The retained parameter context is:

```text
FourierLaw --parameterized_by--> ThermalMaterialModel
ThermalMaterialModel --includes_component--> ThermalConductivity_1
```

which is sufficient for the chosen minimal stress case.

## TRV-02

Remove:

```text
HeatEquation_1 : Equation
HeatMathModel --includes_component--> HeatEquation_1
```

Reason: `SteadyHeatMathModel` already supplies the reference-specific mathematical-model identity and the generic Equation instance had no accepted relation to the field or independent equation payload.

The retained mathematical context is:

```text
ThermalPhysics --represented_by--> HeatMathModel
HeatMathModel --closed_by--> FourierLaw
HeatMathModel --defined_on--> ThermalSpatialModel
HeatMathModel --includes_component--> TemperatureField_1
```

## Preserved stress dimensions

Unchanged:

- Simulation/Model/Task identity and cardinality;
- `includes_component` allowed-pair validation;
- `represented_by`, `closed_by`, `parameterized_by`, `defined_on`;
- condition `applied_to` cardinality and field/scope targets;
- explicit PropertyAssignments;
- Thermal Property Interfaces;
- targeted Dimension Constraints;
- exact-decimal values and UnitReferences;
- focused metrology evidence;
- official-document MOOSE/COMSOL/Ansys sanity verdict;
- backend runtime/license exclusion.

## Research verdict

**Ready for focused fixture readback after the two entities/edges are removed.**
