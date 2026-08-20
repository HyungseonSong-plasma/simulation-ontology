# Profile / Backend Mapping Contract — Thermal Reference Study v0.1

**Status:** Research/design candidate  
**Date:** 2026-08-20  
**Reference systems:** MOOSE, COMSOL, Ansys  
**Purpose:** Derive a backend-independent Profile / Backend Mapping contract by projecting the same small thermal semantic model into three simulation systems.

## 1. Reference semantic model

The reference model contains:

```text
Material
├── ThermalConductivity
TemperatureField
BoundaryCondition
├── prescribed temperature
└── target boundary/scope
Study
└── steady or transient analysis semantics
```

The goal is not to copy any backend's object tree. The goal is to preserve the same SOL semantic graph while generating a valid backend-specific representation.

## 2. MOOSE projection

Representative native realization:

```text
[Variables]
  [T]

[Kernels]
  HeatConduction(variable=T)

[Materials]
  HeatConductionMaterial(thermal_conductivity=...)

[BCs]
  DirichletBC(variable=T, value=..., boundary=...)
  FunctionDirichletBC(variable=T, function=..., boundary=...)

[Executioner]
  Steady | Transient
```

Observations:

- SOL `TemperatureField` maps to a MOOSE variable plus equation/kernel participation rather than to one self-contained backend object.
- SOL material conductivity may map to a parameter of a material object.
- SOL boundary-condition targeting maps through MOOSE `variable` and `boundary` references.
- `ValueDefinition` may be realized as a literal parameter or as a separate MOOSE Function.
- `Study` semantics are split between executioner selection and, for transient physics, equation/kernel content such as time-derivative terms.

This is strong evidence that mapping cardinality is not restricted to one SOL concept ↔ one backend object.

## 3. COMSOL projection

Representative native realization:

```text
Heat Transfer in Solids physics interface
├── Solid domain feature
│   └── thermal conductivity k (often From material)
├── Temperature boundary feature
│   ├── boundary selection
│   └── T0 expression/value
├── default or explicit boundary features
└── study / solver configuration
```

Observations:

- SOL `TemperatureField` is realized as the dependent variable owned by a physics interface rather than a standalone variable object in the same sense as MOOSE.
- `ThermalConductivity` may be supplied by material data or by a user-defined physics property/expression.
- Boundary scope is represented by COMSOL selection semantics, separate from the temperature expression.
- A prescribed value can be a literal or expression.
- Study semantics are represented by COMSOL study/solver features and interact with the physics formulation.

This reinforces the need to map semantic roles and bindings rather than backend tree positions.

## 4. Ansys projection

Representative Mechanical/Workbench realization:

```text
Engineering Data / Material
└── Thermal Conductivity

Steady-State Thermal or Transient Thermal analysis
├── geometry/material assignment
├── Temperature load/boundary condition
├── other thermal loads/conditions
└── Analysis Settings / solver context
```

Observations:

- Material properties are typically sourced from Engineering Data and assigned to bodies.
- Temperature conditions are scoped to geometry/selections.
- Analysis type and settings are represented by the Workbench/Mechanical analysis context rather than by the same object decomposition used by MOOSE or COMSOL.
- Temperature-dependent material behavior may be tabular or otherwise backend-specific.

The Ansys projection again requires semantic mapping across multiple backend object categories and scoping mechanisms.

## 5. Cross-backend semantic mapping matrix

| SOL semantic construct | MOOSE | COMSOL | Ansys | Mapping shape |
|---|---|---|---|---|
| `TemperatureField` | Variable + equation/kernel participation | Dependent variable of Heat Transfer physics | Temperature DOF/field of Thermal analysis | transformed / composite |
| `ThermalConductivity` | Material parameter/property | Material or Solid feature property | Engineering Data material property | direct role mapping, source differs |
| `Material` | Material block/object + block applicability | Material node + domain assignment | Engineering Data material + body assignment | transformed |
| prescribed Temperature BC | `DirichletBC` / `FunctionDirichletBC` | Temperature feature | Temperature condition/load | near-exact semantic role |
| BC scope | `boundary` sideset reference | geometric selection | geometry/named-selection scoping | transformed reference mapping |
| BC target field | `variable=T` | implicit/physics-owned temperature variable | implicit analysis DOF/context | explicit in MOOSE, contextual in others |
| literal value | numeric parameter | numeric expression | numeric property | exact or unit-transformed |
| expression/function | MOOSE Function or expression-capable parameter | expression/function system | tabular/expression/command capability depending on context | transformed; capability-sensitive |
| steady Study | `Executioner type=Steady` + steady equation formulation | Stationary study/solver | Steady-State Thermal analysis | transformed |
| transient Study | `Transient` + time-dependent equation terms | Time Dependent study + physics formulation | Transient Thermal analysis | transformed |

## 6. Main design finding: mapping is rule-based, not lookup-only

A Profile cannot be only:

```text
SOL concept -> backend type name
```

because a semantic construct may map to:

- one backend object;
- several backend objects;
- a backend object plus parameter bindings;
- a context-owned implicit feature;
- a generated expression/function/table;
- no faithful native representation.

Therefore the mapping unit must be a **MappingRule** with explicit source contract, backend realization, conditions, and representability classification.

## 7. Candidate Profile contract

A Profile minimally needs:

```text
Profile
├── identity / version
├── imports / required ontology packages
├── backend adapter binding
├── backend release compatibility
├── MappingRules
├── profile constraints/refinements
└── generation policy / defaults
```

A candidate mapping rule:

```yaml
mapping:
  id: map-temperature-dirichlet

  source:
    type: BoundaryCondition
    constraints:
      - kind: semantic-role
        equals: prescribed_temperature

  target:
    backend: moose
    construct: DirichletBC

  bindings:
    target_field:
      to: variable
    scope:
      to: boundary
    value_definition:
      to: value-or-function

  representability: exact_or_transformed
```

The concrete target/binding vocabulary is adapter-defined, while the Profile contract remains backend-neutral.

## 8. Mapping status vocabulary

The following four statuses are sufficient for v0.1 candidate design:

### exact

The backend can represent the SOL semantics without semantic transformation beyond ordinary identifier/unit serialization.

### transformed

The semantics are preserved, but realization requires structural transformation, decomposition, synthesis, or contextual binding.

Examples:

- one SOL field becoming MOOSE Variable + Kernels;
- one SOL Study becoming executioner plus formulation-dependent backend settings;
- SOL scope relation becoming COMSOL selection or Ansys scoping.

### lossy

A backend representation can be generated, but some declared SOL semantics cannot be preserved faithfully.

Loss MUST be explicit and diagnostic. The SOL model remains unchanged.

### unsupported

No valid backend realization exists under the selected adapter/profile/capability context.

Unsupported is not an SOL semantic-validation failure.

## 9. Exact vs transformed

`transformed` is not a failure state.

This distinction is important because most useful cross-backend mappings are structurally transformed while semantically faithful.

```text
semantic preservation
    exact       -> yes
    transformed -> yes
    lossy       -> partial
    unsupported -> no backend realization
```

## 10. Candidate MappingRule contract

A MappingRule SHOULD be able to carry at least:

```text
MappingRule
├── id
├── source selector / semantic contract
├── applicability predicate
├── target backend construct or generation operation
├── bindings
│   ├── property binding
│   ├── relation/scoping binding
│   ├── value/value-definition binding
│   └── identity/backend-name binding
├── required backend capabilities
├── representability status or status evaluator
├── diagnostics
└── optional priority only for rule selection, never semantic override
```

Priority, if ever used, applies only to selecting among equivalent adapter mapping implementations. It MUST NOT override SOL semantic constraints or resolve ontology ambiguity.

## 11. Mapping cardinality

The contract SHALL allow:

```text
1 SOL construct -> 1 backend construct
1 SOL construct -> N backend constructs
N SOL constructs -> 1 contextual backend construct
N SOL constructs -> N backend constructs
```

A generated backend artifact may therefore need provenance back to one or more SOL model-instance identities.

## 12. Semantic vs generation bindings

Profile mapping should distinguish two concerns:

```text
Semantic mapping
  What backend concept/capability corresponds to this SOL meaning?

Generation binding
  Which backend property/tag/selection/object is written, and how?
```

This separation prevents backend serialization details from becoming semantic ontology relations.

## 13. Identity preservation

Under ADR-0009:

```text
SOL schema identity
SOL model-instance identity
        remain stable

backend-local identity
        is generated/bound by adapter/profile
```

A MappingRule MAY generate backend-local names/tags, but those identifiers SHALL NOT replace SOL identity.

## 14. Profile constraints

A Profile MAY add monotonic representability refinements, such as restricting a value definition to a backend-supported subset.

It SHALL NOT weaken Core/domain semantic constraints.

If the model is semantically valid but outside the profile/backend capability set, the result is `lossy` or `unsupported`, not silent semantic mutation.

## 15. Proposed v0.1 principles

### PBM1 — Rule-based mapping

Backend mapping SHALL be expressed as rules/contracts rather than a mandatory one-to-one type lookup.

### PBM2 — Arbitrary structural cardinality

A mapping MAY synthesize or consume multiple backend constructs while preserving one or more SOL semantic identities.

### PBM3 — Explicit representability status

Mapping evaluation SHALL distinguish `exact`, `transformed`, `lossy`, and `unsupported`.

### PBM4 — Transformed may be lossless

Structural transformation SHALL NOT be classified as semantic loss when all declared SOL semantics remain preserved.

### PBM5 — Semantic/generation separation

Semantic mapping and concrete backend generation bindings SHALL be separable concerns.

### PBM6 — Identity preservation

Backend-local identity SHALL remain a binding/projection artifact and SHALL NOT replace SOL schema or model-instance identity.

### PBM7 — No semantic weakening

Profiles/adapters SHALL NOT silently weaken SOL semantics to obtain a backend representation.

### PBM8 — Adapter-owned backend vocabulary

Backend target constructs, native property paths, scoping mechanisms, release-specific capabilities, and serialization operations SHALL remain adapter-defined.

## 16. Remaining questions before ADR

1. Whether `Profile` and `BackendAdapter` are both first-class architectural resources or whether Profile is data loaded by an adapter runtime.
2. Whether MappingRule belongs to Core schema, Profile schema, or a dedicated mapping package.
3. How backend capabilities are declared and queried without creating a heavy capability ontology.
4. Whether `exact/transformed/lossy/unsupported` is attached to a MappingRule statically or computed per model instance.
5. How multi-rule composition and rule-selection conflicts are resolved.
6. Plasma-domain stress test: species/reactions/electron-energy models/functions provide a harder test of the same contract.
