# MappingRule Minimal Schema Stress Test v0.1

**Status:** Research/design candidate  
**Date:** 2026-08-20  
**Reference systems:** MOOSE, COMSOL, Ansys  
**Domains:** thermal conduction, low-temperature plasma

## Purpose

Test whether the proposed five-field `MappingRule` is sufficient across a simple thermal model and a plasma model, and identify any missing semantic axis before formalizing the Profile / Backend Mapping architecture.

Candidate schema:

```text
MappingRule
├── source
├── applicability
├── realization
├── bindings
└── capabilities
```

Representability classification (`exact`, `transformed`, `lossy`, `unsupported`) is evaluated in the MappingPlan rather than stored as a fixed source rule label.

## 1. Thermal reference model

Reference semantic model:

```text
Material
ThermalConductivity
TemperatureField
DirichletTemperatureBC
Scope/Boundary
Steady or Transient Study
```

### MOOSE

A thermal temperature field is realized through a Variable plus one or more Kernels; thermal conductivity is supplied by a Material object; Dirichlet conditions bind both a target variable and boundary sideset; and study/execution behavior is represented through the Executioner. This is naturally a transformed mapping rather than a universal 1:1 object mapping.

Representative rule:

```yaml
source:
  type: sol:TemperatureField

applicability:
  when:
    implements: sol:SpatialField

realization:
  create:
    - moose:Variable
    - moose:HeatConductionKernel
  depends_on:
    - thermal-material-realization

bindings:
  - source: quantity
    target: variable_semantics
  - source: defined_on
    target: block

capabilities:
  - variable
  - heat-conduction-kernel
```

The five fields are sufficient.

### COMSOL

The Heat Transfer interface owns/organizes the dependent temperature variable, domain physics feature, material properties, selections, boundary features, and study setup. A rule may therefore map one SOL construct into a backend-owned feature context rather than always creating an independent backend object.

Representative realization:

```yaml
realization:
  attach_to: comsol:HeatTransferInSolids
  create:
    - comsol:SolidFeature
  depends_on:
    - heat-transfer-interface
```

Again, no new top-level mapping axis is required.

### Ansys

Thermal behavior is organized by a thermal analysis context, material properties, loads/boundary conditions, scoping, and solution settings. The same source/applicability/realization/bindings/capabilities pattern remains sufficient.

## 2. Plasma stress test

Reference semantic model:

```text
ElectronDensityField
MeanElectronEnergyField
HeavySpecies
ElectronImpactReaction
HeavySpeciesReaction
SurfaceReaction
TransportCoefficient
ElectrostaticPotentialField
WallBoundaryCondition
PlasmaStudy
```

COMSOL Plasma provides a strong stress case because the Plasma interface couples electron drift-diffusion, heavy-species transport, and electrostatics. Its chemistry representation includes species, electron-impact reaction groups, heavy-species reactions, and surface reactions. Chemistry expressions may depend on electron temperature, electron density, gas temperature, reduced electric field, and scoped model variables.

### Example: electron-impact ionization reaction

```yaml
source:
  type: plasma:ElectronImpactReaction

applicability:
  when:
    and:
      - path: reaction_class
        one_of: [ionization, excitation, attachment]
      - path: reactants
        exists: true

realization:
  create:
    - backend:ElectronImpactReaction
  depends_on:
    - species-realizations
    - electron-transport-realization

bindings:
  - source: reactants
    target: reactants
  - source: products
    target: products
  - source: rate_definition
    target: rate_expression
  - source: energy_loss
    target: electron_energy_loss

capabilities:
  - electron-impact-chemistry
  - rate-expression
```

The five fields remain sufficient.

### Example: surface reaction

A surface reaction additionally depends on a surface/wall scope and species already realized in the backend.

```yaml
realization:
  create:
    - backend:SurfaceReaction
  depends_on:
    - wall-boundary-realization
    - species-realizations
```

No new semantic top-level field is needed; dependency/order is part of realization planning.

### Example: transport coefficient

Transport coefficients may be literals, tables, interpolation/function models, or computed expressions. These differences are already captured by existing SOL `ValueDefinition` semantics and map through `bindings` plus target capabilities. They do not require a new MappingRule family.

## 3. New finding: realization dependency/order

The stress test revealed one operational requirement that is weak in the original five-field sketch: generated backend artifacts frequently depend on artifacts created by other rules.

Examples:

```text
species before reactions
field/context before BCs
physics interface before physics features
material/property object before property binding
analysis/study context before solver feature configuration
```

This SHOULD NOT become a sixth semantic axis on `MappingRule`. It is part of backend realization planning.

Recommended structure:

```text
realization
├── create / attach / configure
├── produces symbolic outputs
├── depends_on other realization outputs
└── optional ordering hints
```

`MappingPlan` can then form a dependency DAG and topologically order backend actions before execution.

## 4. Representability classification

The test supports keeping representability as a plan/result property rather than a fixed declaration on the rule.

```text
Profile expectation
       +
source model details
       +
Adapter runtime capabilities
       ↓
MappingPlan evaluation
       ↓
exact | transformed | lossy | unsupported
```

A rule that is exact for a literal conductivity may be transformed or lossy for a temperature-dependent external property model on a backend/release with weaker support.

## 5. Minimal candidate schema

```text
MappingRule
├── source
│   └── semantic type/interface/pattern being mapped
├── applicability
│   └── domain-neutral Predicate over source/context
├── realization
│   ├── create / attach / configure backend constructs
│   ├── produced realization handles
│   └── dependency/order metadata
├── bindings
│   └── source Properties/Relations/ValueDefinitions -> backend slots/references
└── capabilities
    └── backend capabilities required for faithful realization
```

This schema supports 1:1, 1:N, N:1-context, and N:N realization patterns.

## 6. Candidate rules

### MR1 — Five-field MappingRule

SOL Profiles SHOULD define mapping rules using the five top-level concerns `source`, `applicability`, `realization`, `bindings`, and `capabilities`.

### MR2 — Realization dependency is internal to realization

Generation dependency and ordering SHALL be represented within `realization` / `MappingPlan`, not as a new semantic MappingRule axis.

### MR3 — MappingPlan as dependency DAG

A MappingPlan SHOULD materialize symbolic realization outputs and dependency edges so a BackendAdapter can topologically order execution or report cycles/unresolved dependencies before mutating a backend model.

### MR4 — Representability is evaluated

`exact`, `transformed`, `lossy`, and `unsupported` SHALL be MappingPlan/RealizationReport outcomes derived from Profile intent and Adapter capabilities rather than immutable labels on MappingRule definitions.

### MR5 — Backend execution remains adapter-owned

MappingRule declarations SHALL NOT embed backend API calls, filesystem side effects, object-handle allocation, or runtime mutation logic. These remain BackendAdapter responsibilities.

## 7. Conclusion

The five-field schema survived both thermal and plasma stress tests without requiring an additional top-level semantic field.

The only added requirement is an explicit realization dependency model inside `realization`/`MappingPlan` so multi-object backend generation can be planned deterministically.

This is sufficient evidence to proceed toward a Profile / Backend Mapping ADR after one final check of rule composition/selection when multiple MappingRules match the same source construct.
