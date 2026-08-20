# Cross-Backend Semantic Mapping Matrix v0.1

**Status:** Research evidence  
**Reference backends:** MOOSE, COMSOL, Ansys Mechanical  
**Reference problem:** Steady-state heat conduction  
**Purpose:** Preserve cross-backend evidence used to refine Simulation Ontology Language (SOL) semantic boundaries, with the next focus on `Value` and `ValueRepresentation`.

## 1. Research question

This matrix tests whether native metadata from MOOSE, COMSOL, and Ansys can be normalized into a common solver-independent semantic graph.

It specifically provides evidence for three accepted parameter-semantics principles:

1. **Native Parameter Non-Equivalence** — a backend-native parameter/property classification does not by itself determine its SOL semantic type.
2. **Contextual Parameterization** — parameterization is a contextual assignment or reference binding between semantic concepts, values, and model contexts.
3. **Semantic Mapping Requirement** — backend metadata must undergo semantic classification before representation in the Core Simulation Ontology.

The matrix is also retained as evidence for the next unresolved semantic boundary: what SOL means by `Value`, and whether value representation requires explicit modeling.

## 2. Matrix A — Core heat-conduction semantics

| SOL semantic concept | MOOSE | COMSOL | Ansys Mechanical | Candidate SOL interpretation |
|---|---|---|---|---|
| Temperature field | nonlinear variable `T`; kernels reference `variable = T` | dependent temperature field of Heat Transfer physics | temperature degree of freedom in thermal analysis | `Field` Entity |
| Heat conduction | heat-conduction Kernel(s) | Heat Transfer in Solids / Solid heat-conduction feature | Steady-State Thermal physics | `PhysicsModel` + `MathematicalModel` |
| Thermal conductivity | material property; consuming kernel references a material-property name | material/physics property `k`, supporting user/material definitions and tensor forms | Engineering Data thermal conductivity, including isotropic/orthotropic and temperature-dependent definitions | `MaterialProperty` Entity with value/model representation |
| Domain scope | `block` / subdomain | domain geometric selection | body/geometry scoping | `Scope` Entity + Relation |
| Stationary analysis | steady execution configuration | Stationary Study | Steady-State Thermal analysis | `StationaryAnalysis` |
| Solver realization | Executioner / solver configuration | solver sequence associated with Study | solver and Analysis Settings | `SolverConfiguration`, separate from `Analysis` |

### Observation

The native representation differs substantially, while the semantic concepts remain stable. In particular, a MOOSE input parameter may contain the *name/reference* of a material property rather than the physical value itself. Native parameter identity therefore cannot be copied directly into the core semantic graph.

## 3. Matrix B — Fixed-temperature boundary condition

| Semantic role | MOOSE | COMSOL | Ansys Mechanical | Candidate SOL interpretation |
|---|---|---|---|---|
| BC identity | `DirichletBC` or equivalent | Temperature physics feature | Temperature boundary condition | `BoundaryCondition` Entity |
| Target field | `variable = T` | Heat Transfer temperature field | thermal temperature DOF/context | `targets -> TemperatureField` |
| Spatial target | `boundary = ...` | boundary selection | geometry/Named Selection through scoping | `defined_on -> Scope` |
| Prescribed value | `value = 300` or function-capable equivalent | `T0 = 300[K]` / expression | Magnitude with supported representations | prescribed value relation/property |
| Value representation | scalar or function depending on BC type | expression and parameter-capable value | constant, tabular, function/table-like forms depending on context | unresolved `ValueRepresentation` boundary |

### Normalized graph

```text
TemperatureBoundaryCondition
        ├── targets ─────────> TemperatureField
        ├── defined_on ──────> BoundaryScope
        └── prescribed_value → TemperatureValue
```

### Observation

A single backend object exposes native inputs that normalize into at least three different semantic categories:

```text
field reference
scope reference
value assignment
```

Therefore `variable`, `boundary`, `selection`, `Location`, `Magnitude`, and similar native metadata cannot all be modeled as a generic SOL `Parameter`.

For this simple case, a mandatory `ParameterBinding` Entity is not required. Relations plus a value assignment are sufficient.

## 4. Matrix C — Material coefficient

| Case | MOOSE | COMSOL | Ansys Mechanical | Candidate SOL interpretation |
|---|---|---|---|---|
| Constant conductivity | material property with constant value | material/property expression with constant value | constant Engineering Data conductivity | `MaterialProperty` + scalar value |
| Temperature-dependent conductivity | material property/model depending on temperature | function/expression-dependent material property | temperature-dependent Engineering Data | `MaterialProperty` + functional/property model |
| Anisotropic conductivity | tensor-capable material modeling | scalar, diagonal, symmetric, or full tensor representation where supported | orthotropic conductivity | `MaterialProperty` + tensor/property model |
| Consuming model | Kernel references material property | heat-conduction feature consumes `k` | thermal model consumes Engineering Data | `parameterized_by` relation |

### Normalized graph

```text
Material
   │
   └── has_material_property
              │
              ▼
     ThermalConductivity
              │
              └── represented_by
                    ├── ScalarValue
                    ├── Function / PropertyModel
                    └── TensorValue
```

### Observation

`ThermalConductivity` should not be reduced to the literal `400 W/(m K)` because the same semantic quantity may have constant, functional, temperature-dependent, anisotropic, or tensor representations.

This is direct evidence that the next SOL boundary must distinguish the **semantic quantity/property** from its **value representation**.

## 5. Matrix D — Analysis and solver semantics

| Semantic concept | MOOSE | COMSOL | Ansys Mechanical | Candidate SOL interpretation |
|---|---|---|---|---|
| Steady-state computational question | steady execution | Stationary Study | Steady-State Thermal | `StationaryAnalysis` |
| Solver realization | Executioner / nonlinear-linear solver configuration | Stationary Solver / solver sequence | solver controls and Analysis Settings | `SolverConfiguration` |
| Convergence/tolerance control | solver parameters | solver/study tolerance settings | convergence controls | `ConvergenceCriterion` and associated values |
| Execution | application/executioner run | study/solver run | Solve | runtime action, not necessarily model ontology |

### Observation

The three backends support the existing SOL separation:

```text
Analysis
    │ solved_by
    ▼
SolverConfiguration
```

Solver-control values also provide a useful future stress test because some values can vary by load step, study step, nonlinear stage, or other execution context. Such cases may require relationship reification or a future Binding construct.

## 6. Matrix E — Native metadata to semantic role

| Native metadata meaning | MOOSE example | COMSOL example | Ansys example | SOL semantic classification |
|---|---|---|---|---|
| Literal value | `value = 300` | `T0 = 300[K]` | temperature Magnitude | `ValueAssignment` candidate |
| Field reference | `variable = T` | dependent field / expression reference | field implicit in thermal object/context | `FieldReference` relation |
| Material-property reference | material-property name | material property / expression source | Engineering Data property | `MaterialPropertyReference` relation |
| Spatial reference | `boundary`, `block` | geometric selection | Geometry/Named Selection/Location | `ScopeReference` relation |
| Representation choice | enum/type-like parameter | material vs user-defined, representation options | constant/tabular/function-style definition | representation/configuration semantics |
| Analysis control | Executioner settings | Study settings | Analysis Settings | `Analysis` configuration semantics |
| Solver control | nonlinear/linear/PETSc options | solver feature settings | convergence/solver controls | `SolverConfiguration` semantics |
| Expression/function | function-capable input | expression/function | tabular/function-like input | candidate `ExpressionValue` / `FunctionValue` |

## 7. Cross-backend findings

### Finding 1 — Native metadata is not semantic ontology

All three systems expose generic parameter/property mechanisms that carry semantically heterogeneous information. Native metadata must therefore be classified before entering the Core SOL graph.

```text
Native Metadata
      │
      ▼
Backend Ontology
      │
      ▼
Semantic Classification
      │
      ▼
SOL Graph
```

### Finding 2 — Parameterization is contextual

A literal such as `300 K` has insufficient simulation meaning by itself. Its meaning is determined by the semantic relationship and context in which it is used:

```text
TemperatureBoundaryCondition ── prescribed_value ──> 300 K
InitialCondition              ── initial_value ─────> 300 K
```

### Finding 3 — Mandatory ParameterBinding is not yet justified

For ordinary cases, the graph can remain simple:

```text
Entity ── Relation ──> Entity / Value
```

A relationship should be reified only when the relationship itself needs context, provenance, state, version constraints, mapping rules, or other rich metadata:

```text
Entity
   │
   ▼
Reified Binding Entity
   │
   ▼
Entity / Value
```

Current cross-backend evidence therefore favors **Relation + optional reification** over a mandatory first-class `ParameterBinding` construct.

## 8. Evidence motivating the `Value` semantic-boundary study

The matrix exposes a distinction that cannot be resolved by the current Entity/Property/Relation rules alone:

```text
Semantic concept
      ≠
Value
      ≠
Value representation
```

For example:

```text
ThermalConductivity
        │
        ├── 400 W/(m K)                 scalar constant
        ├── k(T)                        functional dependence
        └── [[kxx,kxy,...], ...]        tensor representation
```

Similarly, a prescribed boundary temperature can be represented as a constant, expression, function, table, or backend-specific evaluable form while retaining the same semantic role.

This raises the following questions for the next SOL design step:

1. Is `Value` a language-level construct, an Entity, a Property payload, or a typed data object?
2. Should scalar, vector, tensor, expression, function, distribution, and tabular forms share a common `Value` abstraction?
3. How are units and physical dimensions attached: to the semantic Quantity, the Value, or both?
4. How should a value reference another semantic entity or parameter?
5. How should context-dependent values such as time-, temperature-, frequency-, load-step-, or parameter-dependent definitions be represented?
6. Where is provenance attached when a value comes from material data, a user expression, an experiment, or a backend database?
7. Should `ValueRepresentation` be explicit, or inferred from a typed value object?

## 9. Working hypothesis for the next study

The matrix supports investigating a model of the form:

```text
Semantic Entity / Quantity
          │
          │ has_value or represented_by
          ▼
        Value
          │
          └── representation/type/context
```

Possible value forms to test include:

```text
ScalarValue
VectorValue
TensorValue
ExpressionValue
FunctionValue
TabularValue
DistributionValue
ReferenceValue
```

This taxonomy is **not yet normative**. It is recorded only as a hypothesis to be tested against the same MOOSE, COMSOL, and Ansys reference backends.

## 10. Decision status

This document is evidence, not an Architecture Decision Record. It preserves the cross-backend observations used to make later normative decisions.

The accepted parameter-semantics principles remain separate ADR material. The next intended semantic-boundary decision is `Value` and `ValueRepresentation`, which should be validated against all three reference backends before being promoted into the SOL v0.1 specification.
