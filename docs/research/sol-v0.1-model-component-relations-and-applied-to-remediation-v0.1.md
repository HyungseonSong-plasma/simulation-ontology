# SOL v0.1 Model-Component Relations and `applied_to` Remediation v0.1

**Role:** Research  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`

## 1. Objective

Following independent acceptance of the model-side tree classification, define the minimum explicit relation semantics needed to replace non-normative tree grouping and repair REL-01:

```text
applied_to domain = ConditionModel
```

which incorrectly relies on condition leaf concepts behaving like implicit ConditionModel subtypes.

This proposal does not complete every Core relation cardinality. It establishes structural component membership and repairs the concrete condition-target relation so later cardinality work has deterministic domains.

## 2. Constraints

Preserve:

- ADR-0007 explicit Relation/Constraint architecture;
- ADR-0008 no artificial multiple inheritance;
- ADR-0014 no implicit tree taxonomy;
- ADR-0015 explicit task/model relations;
- backend independence;
- component reuse and non-owning semantics;
- minimal Core vocabulary;
- no backend installation/runtime dependency.

## 3. Candidate component-relation strategies

### Candidate A — one relation per child role

Examples:

```text
has_field
has_equation
has_boundary
has_species
has_material_property
...
```

This is maximally explicit but introduces dozens of structural relations whose only distinction is the target type. It duplicates type information in relation names and increases schema/mapping surface without evidence that the structural membership semantics differ.

**Disposition: reject for v0.1 minimal Core.**

### Candidate B — one relation per aggregate category

Examples:

```text
has_mathematical_component
has_spatial_component
has_material_component
...
```

This reduces vocabulary but still creates parallel relations with identical non-owning aggregation semantics and requires a second relation for every future aggregate category.

**Disposition: acceptable but not minimal.**

### Candidate C — generic `includes_component` + source-type constraints

```text
includes_component
```

means:

> this aggregate/model context directly includes the referenced semantic component in its definition.

The relation is non-owning, non-transitive, and order-independent. Type-specific Constraints determine which target types are valid for each source type.

This preserves one stable structural relation while keeping semantic role information in canonical Entity Type identity.

**Disposition: preferred.**

## 4. Proposed `includes_component` contract

```text
includes_component
  domain:
    SimulationModel
    PhysicsModel
    MathematicalModel
    ConstitutiveModel
    SpatialModel
    MaterialModel
    ConditionModel
    NumericalModel
    ObservationModel
    SolverConfiguration
  range: Entity
  source cardinality: 0..*
  ownership: non-owning
  order semantics: none
  transitive: false
```

The broad `range: Entity` is constrained by a mandatory source-type compatibility table; it is not permission to link arbitrary Entity Types.

### 4.1 Allowed direct-component matrix

| Source type | Allowed direct `includes_component` targets |
|---|---|
| SimulationModel | PhysicsModel, MathematicalModel, ConstitutiveModel, SpatialModel, MaterialModel, ConditionModel, NumericalModel, ObservationModel |
| PhysicsModel | Phenomenon, Process, Interaction |
| MathematicalModel | Formulation, Equation, Field, Operator, MathematicalParameter |
| ConstitutiveModel | ClosureRelation, PropertyModel |
| SpatialModel | Geometry, Domain, Boundary, SpatialInterface, Scope |
| MaterialModel | Material, Species, MaterialProperty |
| ConditionModel | BoundaryCondition, InitialCondition, Source, Load |
| NumericalModel | Discretization, Mesh, NumericalApproximation |
| ObservationModel | Quantity, Probe, Integral, Dataset, Output |
| SolverConfiguration | NonlinearSolver, LinearSolver, Preconditioner, ConvergenceCriterion |

Any direct pair outside this table is a Core type/structure conflict unless another accepted extension/profile contract explicitly defines a different relation for that purpose.

### 4.2 No transitive inference

From:

```text
SimulationModel includes_component MathematicalModel
MathematicalModel includes_component Field
```

SOL v0.1 SHALL NOT automatically assert:

```text
SimulationModel includes_component Field
```

The direct relation is intentionally non-transitive. Tools MAY expose recursive traversal as a query/view, but it is not an additional stored semantic edge.

### 4.3 No ownership/lifecycle semantics

The same Field, MaterialProperty, Scope, or other component MAY be referenced from more than one compatible aggregate where semantic reuse is valid.

Deleting an aggregate does not semantically delete its components.

### 4.4 Requiredness remains orthogonal

General `includes_component` cardinality is:

```text
0..*
```

This relation alone does not say every MathematicalModel must contain an Equation or every MaterialModel must contain a Material. Requiredness belongs to explicit type/interface/domain/profile constraints.

This prevents structural membership from being confused with completeness rules.

## 5. REL-01 solution alternatives

### Alternative 1 — introduce `Condition` superclass

```text
BoundaryCondition is_a Condition
InitialCondition is_a Condition
Source is_a Condition
Load is_a Condition
```

Problem: `Source` and `Load` are external forcing concepts and are not necessarily semantically kinds of “condition”. This would create taxonomy mainly to simplify one relation domain.

**Reject.**

### Alternative 2 — introduce a new Interface solely for `applied_to`

This is extensible, but current evidence shows no unrelated Entity Type that needs exactly this capability contract. Introducing an Interface now adds language/serialization surface without solving another independent problem.

**Defer until reuse evidence appears.**

### Alternative 3 — explicit domain union

Define:

```text
applied_to
  domain:
    BoundaryCondition
    InitialCondition
    Source
    Load
  range:
    Field
    Equation
    Scope
  source cardinality: 1..*
```

This directly represents the current v0.1 semantics without implicit inheritance or an artificial superclass.

**Preferred.**

## 6. `applied_to` semantics

An individual BoundaryCondition, InitialCondition, Source, or Load exists to constrain/force at least one semantic target. Therefore each such Entity instance SHALL have at least one `applied_to` edge.

Multiple targets are allowed. Examples:

```text
bc-1 applied_to temperature-field
bc-1 applied_to wall-scope
```

or:

```text
source-1 applied_to energy-equation
source-1 applied_to plasma-domain-scope
```

Target type gives semantic role; relation order has no meaning.

The relation does not encode backend selection IDs, sideset names, feature tags, or native solver handles.

## 7. Interaction with `includes_component`

Example thermal structure:

```text
sim-model includes_component math-model
sim-model includes_component spatial-model
sim-model includes_component material-model
sim-model includes_component condition-model

math-model includes_component temperature-field
spatial-model includes_component wall-scope
condition-model includes_component wall-temperature-bc

wall-temperature-bc applied_to temperature-field
wall-temperature-bc applied_to wall-scope
```

The structural relation and semantic target relation remain orthogonal:

```text
includes_component != applied_to
```

A condition's membership in a ConditionModel does not imply which field/scope it constrains.

## 8. Existing inter-model relations remain distinct

Do not replace:

```text
PhysicsModel represented_by MathematicalModel
MathematicalModel closed_by ConstitutiveModel
ConstitutiveModel parameterized_by MaterialModel
MathematicalModel defined_on SpatialModel
MathematicalModel discretized_by NumericalModel
Analysis solved_by SolverConfiguration
```

with `includes_component`.

Those relations answer different semantic questions between aggregate contexts. `includes_component` only replaces the structural meaning previously hidden in tree grouping.

## 9. Boundary counterexamples

### C-MC-01 — invalid cross-category component

```text
MaterialModel includes_component BoundaryCondition
```

Expected: type/structure conflict.

### C-MC-02 — no transitive stored edge

```text
SimulationModel includes_component MathematicalModel
MathematicalModel includes_component Field
```

Expected: valid; no automatic direct SimulationModel→Field edge.

### C-MC-03 — component reuse

```text
math-1 includes_component field-1
math-2 includes_component field-1
```

Expected: allowed unless another semantic constraint forbids the particular reuse.

### C-MC-04 — empty aggregate definition

```text
material-model-1 : MaterialModel
```

with zero components.

Expected at this generic Core relation layer: structurally valid. A domain/reference-model completeness constraint may later require content.

### C-MC-05 — condition with no target

```text
bc-1 : BoundaryCondition
```

with no `applied_to`.

Expected: invalid condition instance.

### C-MC-06 — aggregate ConditionModel used as condition

```text
condition-model-1 applied_to wall-scope
```

Expected: type violation. Only the explicit condition/forcing leaf union is in the v0.1 domain.

### C-MC-07 — artificial Condition inheritance

Validator assumes `Load is_a ConditionModel` to make old `applied_to` domain work.

Expected: non-conforming; no such inheritance exists.

## 10. Backend stress assessment

### MOOSE

Variables, Kernels, BCs, Mesh, and solver/execution configuration are distinct native objects/sections. `includes_component` expresses only Core structural membership; MOOSE block organization remains Adapter realization. A BC's `variable` and `boundary` bindings support direct condition-to-target semantics rather than aggregate ConditionModel application.

### COMSOL

Physics/model feature trees similarly contain fields/features/selections as distinct objects. Direct condition/scoping features support a condition-target relation while tree/node ownership remains backend-local.

### Ansys

Model/Setup structures contain geometry, mesh, loads, and BCs as separate objects. The Core relation can represent the semantic grouping without copying Workbench tree types.

No backend requires `includes_component` to become transitive or owning.

## 11. Proposed machine-readable shape

Conceptually:

```yaml
relations:
  includes_component:
    domain:
      - SimulationModel
      - PhysicsModel
      - MathematicalModel
      - ConstitutiveModel
      - SpatialModel
      - MaterialModel
      - ConditionModel
      - NumericalModel
      - ObservationModel
      - SolverConfiguration
    range: Entity
    source_cardinality: {min: 0, max: unbounded}
    ownership: non_owning_reference
    transitive: false

  applied_to:
    domain:
      - BoundaryCondition
      - InitialCondition
      - Source
      - Load
    range:
      - Field
      - Equation
      - Scope
    source_cardinality: {min: 1, max: unbounded}
```

A separate type-compatibility constraint table SHALL enforce the allowed `includes_component` pairs.

The exact final serialization field spelling remains a consolidation concern, not part of the semantic decision.

## 12. Research verdict

**Ready for independent Validation.**

Recommended v0.1 rule:

- use one non-owning, non-transitive `includes_component` relation for direct structural membership;
- constrain allowed source→target type pairs by the explicit matrix;
- keep generic component requiredness optional (`0..*`) unless another contract requires content;
- repair `applied_to` with the explicit domain union and `1..*` target cardinality;
- do not invent a common Condition superclass or a one-use Interface solely to repair REL-01.
