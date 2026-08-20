# SOL v0.1 Minimal Thermal Reference Model v0.1

**Role:** Research  
**Date:** 2026-08-20  
**Stage:** Design-stage reference-model stress test  
**Scope:** Core semantic expressibility and official-document backend sanity only; no backend installation, license, runtime, or production Adapter validation

## 1. Objective

Test whether the accepted SOL v0.1 Core, Constraint, Value/Unit/Dimension, Interface, identity, and package contracts can describe one deliberately small steady heat-conduction problem without importing MOOSE, COMSOL, or Ansys object structure into Core.

The case is intentionally smaller than an Adapter integration test. The required question is:

> Can one stable SOL semantic model preserve the same thermal intent while each reference backend realizes it with its own native structure?

## 2. Reference problem

A homogeneous solid region has a temperature field `T` governed by steady Fourier heat conduction with constant scalar thermal conductivity:

```text
div(k grad(T)) = 0
```

Reference values:

```text
thermal conductivity k = 10 W/(m K)
left boundary temperature = 300 K
right boundary temperature = 400 K
```

The case contains no volumetric heat source, transient heat capacity term, radiation, convection, contact, phase change, or multiphysics coupling.

Only two named boundary scopes are semantically relevant to the fixture. Geometry tessellation, element family, nonlinear/linear solver choices, backend tags, and backend-native selection handles are not Core semantics of this reference case.

## 3. Official backend evidence

### 3.1 MOOSE

Official MOOSE heat-transfer examples show the same semantic ingredients as separate native constructs:

- a temperature variable;
- `HeatConduction` operating on that variable;
- a material exposing `thermal_conductivity`;
- `DirichletBC` binding a temperature value to a variable and named boundary;
- `Executioner type = Steady` for steady solution.

References:

- https://mooseframework.inl.gov/moose/source/executioners/Steady.html
- https://mooseframework.inl.gov/moose/modules/heat_transfer/tutorials/introduction/therm_step02.html

### 3.2 COMSOL

Official COMSOL documentation states that Heat Transfer in Solids models solid heat transfer with the heat equation/Fourier conduction, with thermal conductivity `k`; for steady state the time-dependent term disappears. A fixed temperature condition is a Temperature boundary feature, and the programming guide creates a `HeatTransfer` physics interface and `TemperatureBoundary` feature assigned to geometry/boundary selections.

References:

- https://doc.comsol.com/6.3/doc/com.comsol.help.comsol/comsol_ref_heattransfer.30.10.html
- https://doc.comsol.com/6.3/doc/com.comsol.help.comsol/comsol_ref_heattransfer.30.25.html
- https://doc.comsol.com/6.3/doc/com.comsol.help.comsol/comsol_ref_heattransfer.30.36.html
- https://doc.comsol.com/6.4/doc/com.comsol.help.comsol/application_programming_guide.15.25.html

### 3.3 Ansys Mechanical

Official Ansys documentation states that a Steady-State Thermal analysis determines temperatures/heat flow under steady thermal loads, requires Thermal Conductivity in Engineering Data, and supports Temperature boundary conditions. Temperature loads are explicitly scoped to geometry or named selections and given a magnitude.

References:

- https://ansyshelp.ansys.com/public/Views/Secured/corp/v252/en/wb_sim/ds_static_thermal_analysis_type.html
- https://ansyshelp.ansys.com/public/Views/Secured/corp/v242/en/wb_sim/ds_Given_Temperatures.html

The backend structures are therefore different, but the thermal semantics required by this fixture are common.

## 4. Minimal SOL semantic inventory

The reference model uses only accepted Core concepts plus reference-specific specializations where a generic Core type alone would not state enough intent.

### Core model instances

```text
Simulation
SimulationModel
SimulationTask
PhysicsModel
MathematicalModel
ConstitutiveModel
SpatialModel
MaterialModel
ConditionModel
Field
Equation
Material
MaterialProperty
Scope
BoundaryCondition
StationaryAnalysis
```

### Reference-specialized semantic types

The reference extension may define stable schema identities such as:

```text
ThermalConductionPhysics     is_a PhysicsModel
SteadyHeatEquation           is_a MathematicalModel or represented by contained Equation semantics
FourierConductionLaw         is_a ConstitutiveModel
TemperatureField             is_a Field
ThermalConductivityProperty  is_a MaterialProperty
FixedTemperatureCondition    is_a BoundaryCondition
```

Only one direct `is_a` parent is used. No backend object type is introduced into the Core/reference taxonomy.

## 5. Reference semantic graph

Conceptually:

```text
ThermalSimulation
  --has_model--> ThermalModel
  --has_task---> SteadyThermalTask

SteadyThermalTask
  --uses_model--> ThermalModel
  --has_analysis--> StationaryThermalAnalysis

ThermalModel
  --includes_component--> ThermalPhysics
  --includes_component--> HeatMathModel
  --includes_component--> FourierLaw
  --includes_component--> ThermalSpatialModel
  --includes_component--> ThermalMaterialModel
  --includes_component--> ThermalConditionModel

ThermalPhysics
  --represented_by--> HeatMathModel

HeatMathModel
  --closed_by--> FourierLaw
  --defined_on--> ThermalSpatialModel
  --includes_component--> TemperatureField_1
  --includes_component--> HeatEquation_1

FourierLaw
  --parameterized_by--> ThermalMaterialModel

ThermalMaterialModel
  --includes_component--> SolidMaterial_1
  --includes_component--> ThermalConductivity_1

ThermalSpatialModel
  --includes_component--> SolidDomainScope
  --includes_component--> LeftBoundaryScope
  --includes_component--> RightBoundaryScope

ThermalConditionModel
  --includes_component--> LeftFixedTemperature
  --includes_component--> RightFixedTemperature

LeftFixedTemperature
  --applied_to--> TemperatureField_1
  --applied_to--> LeftBoundaryScope

RightFixedTemperature
  --applied_to--> TemperatureField_1
  --applied_to--> RightBoundaryScope
```

This satisfies ADR-0015/0016 model/task and direct-membership semantics. `analyzed_by` need not be authored because it is derived from the task bindings.

## 6. Value / Unit / Dimension use

All three numerical definitions are local, literal, and dependency-free, so ADR-0026/0027 permit inline ValueDefinitions.

Conceptually:

```text
ThermalConductivity_1.value_definition
  mechanism: literal
  value: 10 W/(m K)

LeftFixedTemperature.value_definition
  mechanism: literal
  value: 300 K

RightFixedTemperature.value_definition
  mechanism: literal
  value: 400 K
```

The normalized number is ADR-0021 exact-decimal data. Unit is an explicit `UnitReference`; unit identity does not create a Core Unit Entity. Required dimensions are semantic Dimension Constraints:

```text
Temperature              -> Theta^1
ThermalConductivity      -> M L T^-3 Theta^-1
```

A design-stage metrology resolution fixture may supply the corresponding resolved `DimensionVector` evidence. Absence of a metrology service is `INDETERMINATE` semantic evidence under ADR-0026, not backend incompatibility and not an Adapter/runtime failure.

## 7. Cross-backend projection sanity matrix

| SOL semantic obligation | MOOSE official construct | COMSOL official construct | Ansys official construct | Design-stage judgment |
|---|---|---|---|---|
| Temperature field | Variable `T`/`temp` | Heat Transfer dependent temperature field | thermal temperature DOF/context | representable |
| Steady heat conduction | `HeatConduction` kernel | Heat Transfer in Solids / Solid heat equation | Steady-State Thermal physics | representable |
| Constant thermal conductivity | material property `thermal_conductivity` | Solid/material `k` | Engineering Data Thermal Conductivity | representable |
| Fixed temperature condition | `DirichletBC` | Temperature boundary feature | Temperature boundary condition/load | representable |
| Spatial target | `boundary` name/id | boundary selection | Geometry Selection / Named Selection | representable |
| Stationary computational question | `Executioner type=Steady` participates in realization | Stationary study | Steady-State Thermal analysis | representable |

The expected overall representability class is **transformed**, not because thermal semantics are lost, but because the same SOL separation is realized by different backend object decompositions. No evidence in this case requires `lossy` or `unsupported`.

## 8. Mapping/identity preservation observations

The reference case supports the existing architecture rather than adding backend-shaped Core objects:

1. one SOL `TemperatureField_1` model-instance identity can map to backend-local `T`, COMSOL field handles, or Ansys thermal DOF context without changing SOL identity;
2. one SOL fixed-temperature condition maps to different native feature/load objects while preserving field, scope, and value intent;
3. Analysis and SolverConfiguration remain separate Core concepts even when a backend native object partially collapses those concerns;
4. Material/constitutive semantics need not mirror a backend material tree;
5. backend boundary/selection identifiers remain backend-local identities.

## 9. Counterexample search against current Core

The simple thermal graph does **not** expose a new Core architecture contradiction in:

- Simulation/Model/Task composition;
- `includes_component` allowed pairs;
- `represented_by`, `closed_by`, `parameterized_by`, `defined_on`;
- `applied_to` leaf-condition targeting;
- Value/Unit/Dimension boundary;
- schema/model/backend identity separation;
- exact/transformed/lossy/unsupported representability vocabulary.

### TH-01 — normalized model-instance snapshot serialization remains absent

ADR-0028 deliberately normalized ontology-package **schema resources**, not model instances. The repository has no accepted canonical model-instance document schema for serializing:

```text
model-instance id + EntityType id
relation edges between model instances
local InlineValueDefinition on a value-bearing model instance
resolved ontology-package environment used by the model
```

This does not make the thermal semantics unrepresentable conceptually, and it is not a backend limitation. It does prevent the thermal reference case from becoming a single canonical machine-readable input that two independent validators can load without an additional fixture convention.

**Classification:** Language/schema contract gap exposed by reference-model validation.

The smallest possible remediation, if design-stage closure requires a machine-readable reference model rather than a human-readable semantic fixture, is a focused normalized model-snapshot envelope. It should not introduce new domain semantics or a generic runtime/execution model.

Candidate minimum only:

```text
ResolvedModelSnapshot
  ontology_environment: exact package identities
  entities:
    - id: opaque model-instance identity
      type: canonical EntityTypeDefinition id
      value_definition?: InlineValueDefinition
  relations:
    - relation: canonical RelationDefinition id
      source: model-instance id
      target: model-instance id
```

The candidate intentionally omits backend handles, solver runtime state, file paths, Adapter state, and generalized property-assignment machinery.

## 10. Research verdict

**Thermal semantic architecture: PASS at contract/conceptual level.**

**Machine-readable reference fixture: REVISE if canonical single-input validation is required for design-stage closure, due only to TH-01.**

Recommended next role: independent Validation should decide whether TH-01 is a genuine closure-gate language/schema gap or whether a documented non-normative fixture is sufficient for the intended v0.1 design-stage reference test.
