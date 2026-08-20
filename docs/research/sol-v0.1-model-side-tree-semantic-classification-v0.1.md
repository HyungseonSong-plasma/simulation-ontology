# SOL v0.1 Model-Side Tree Semantic Classification v0.1

**Role:** Research  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`  
**Scope:** classify existing architecture tree edges before assigning remaining relation cardinalities

## 1. Problem

ADR-0014 established that architecture tree indentation and the old `children` field are not taxonomic semantics. ADR-0015 resolved only the top-level Simulation/Model/Task application structure.

The remaining architecture still displays trees such as:

```text
MathematicalModel
├── Formulation
├── Equation
├── Field
├── Operator
└── MathematicalParameter
```

and:

```text
ConditionModel
├── BoundaryCondition
├── InitialCondition
├── Source
└── Load
```

while `relations.yaml` uses broad domains such as `MathematicalModel` and `ConditionModel`. Because the leaf concepts currently have no `is_a` edge to those aggregate categories, a validator cannot infer that an `Equation` is eligible wherever a `MathematicalModel` is required.

The next cardinality matrix is therefore premature until each displayed edge is classified semantically.

## 2. Evaluation rule

Use a strict substitutability test for `is_a`:

```text
X is_a Y
```

is justified only if every valid instance of `X` is itself a semantic instance of `Y` and can satisfy the semantic contracts of `Y` without reinterpretation.

A diagram edge is **composition/association** when the child is a distinct semantic object that participates in or belongs to the parent model context but is not itself an instance of the parent type.

When neither is proven, mark `unresolved`; do not infer either relation.

## 3. Cross-backend evidence

### MOOSE

A basic MOOSE model separates mesh, variables, kernels/equations, boundary conditions, execution, and outputs into distinct input sections. A Variable is not a subtype of the whole mathematical/problem model and a boundary condition is not a subtype of the model container.

Official evidence:
- https://mooseframework.inl.gov/moose/getting_started/examples_and_tutorials/examples/ex01_inputfile.html

### COMSOL

A COMSOL model/physics interface contains dependent variables, physics features, material/constitutive settings, selections/scopes, studies, and solver sequences as distinct nodes/objects. A boundary feature or dependent field is not a subtype of the containing model/physics object.

Official evidence:
- https://doc.comsol.com/6.3/doc/com.comsol.help.comsol/comsol_api_general.47.60.html
- https://doc.comsol.com/6.4/doc/com.comsol.help.comsol/application_programming_guide.15.25.html

### Ansys Workbench / Mechanical

The Workbench Model cell contains geometry/model/mesh concerns while Setup contains loads, boundary conditions, and analysis configuration. These are structurally related objects rather than inheritance copies of the entire Model cell.

Official evidence:
- https://ansyshelp.ansys.com/public/Views/Secured/corp/v252/en/wb2_help/wb2h_typesofcells.html

Backend structure is not normative, but all three references supply counterevidence to interpreting every architecture tree edge as `is_a`.

## 4. Classification matrix

### 4.1 `SimulationModel` tree

| Displayed edge | Classification | Reason |
|---|---|---|
| SimulationModel → PhysicsModel | composition/association | Physics concern is part of the complete semantic model; a PhysicsModel is not itself the whole SimulationModel. |
| SimulationModel → MathematicalModel | composition/association | Mathematical representation is a model component/aspect, not the complete SimulationModel identity. |
| SimulationModel → ConstitutiveModel | composition/association | closure/property model participates in the simulation model but is not the whole model. |
| SimulationModel → SpatialModel | composition/association | spatial context is a distinct model component. |
| SimulationModel → MaterialModel | composition/association | material/species context is distinct. |
| SimulationModel → ConditionModel | composition/association | conditions/forcing are distinct from the complete model. |
| SimulationModel → NumericalModel | composition/association | discretization/numerical representation is distinct from semantic model identity. |
| SimulationModel → ObservationModel | composition/association | observation/output definition is a distinct concern. |

**No `is_a` edge is justified.**

### 4.2 `PhysicsModel` tree

| Edge | Classification | Reason |
|---|---|---|
| PhysicsModel → Phenomenon | composition/association | a physical phenomenon is what the physics model describes; it is not necessarily itself a model. |
| PhysicsModel → Process | composition/association | a process can be represented by a model but is not identical to a model container. |
| PhysicsModel → Interaction | composition/association | an interaction is a physical semantic object described/represented in a model. |

A future domain ontology MAY define domain-specific taxonomies below `Phenomenon`, `Process`, or `Interaction`; that does not make these generic concepts subclasses of `PhysicsModel`.

### 4.3 `MathematicalModel` tree

All five displayed edges are **composition/association**, not inheritance:

- Formulation
- Equation
- Field
- Operator
- MathematicalParameter

Counterexample: a `Field` instance is not a `MathematicalModel`; it is an independently identifiable mathematical object used by a model. Likewise an `Operator` or `Equation` can be reused across mathematical model definitions.

### 4.4 `ConstitutiveModel` tree

| Edge | Classification | Reason |
|---|---|---|
| ConstitutiveModel → ClosureRelation | composition/association | the aggregate model may contain several closure relations; the relation has separate identity. |
| ConstitutiveModel → PropertyModel | composition/association | property models participate in closure without being the entire constitutive-model aggregate. |

Although a particular domain could choose a specialized constitutive-model subtype taxonomy, the current generic tree does not prove such inheritance.

### 4.5 `SpatialModel` tree

All are **composition/association**:

- Geometry
- Domain
- Boundary
- SpatialInterface
- Scope

A Boundary or Scope is not substitutable for an entire SpatialModel.

### 4.6 `MaterialModel` tree

All are **composition/association**:

- Material
- Species
- MaterialProperty

A MaterialProperty is not a MaterialModel. Materials/species/properties are independently reusable semantic entities participating in the material context.

### 4.7 `ConditionModel` tree

All are **composition/association**:

- BoundaryCondition
- InitialCondition
- Source
- Load

A BoundaryCondition is not the whole ConditionModel. The current `applied_to` relation therefore cannot rely on implicit `BoundaryCondition is_a ConditionModel` inheritance.

### 4.8 `NumericalModel` tree

All are **composition/association**:

- Discretization
- Mesh
- NumericalApproximation

A Mesh is not a NumericalModel; it is a numerical/spatial artifact used by that model context.

### 4.9 `ObservationModel` tree

All are **composition/association**:

- Quantity
- Probe
- Integral
- Dataset
- Output

These are observation definitions/data concepts participating in an ObservationModel context, not subtypes of the aggregate.

### 4.10 `SolverConfiguration` tree

All are **composition/association**:

- NonlinearSolver
- LinearSolver
- Preconditioner
- ConvergenceCriterion

A LinearSolver is not substitutable for the entire SolverConfiguration; the configuration can combine several of these concerns.

### 4.11 `Analysis` tree

The following remain valid **taxonomic `is_a`** edges:

```text
StationaryAnalysis is_a Analysis
TransientAnalysis is_a Analysis
FrequencyDomainAnalysis is_a Analysis
EigenvalueAnalysis is_a Analysis
ParametricAnalysis is_a Analysis
OptimizationAnalysis is_a Analysis
```

Each names a specialized kind of computational question and is substitutable for the general Analysis contract.

### 4.12 SimulationTask

ADR-0015 is authoritative:

- Analysis is **not** a SimulationTask subtype;
- SolverConfiguration is **not** a SimulationTask subtype;
- SimulationTask reifies the application of one Analysis to one SimulationModel.

## 5. Consequence for current relation domains

The classification exposes that broad relation domains/ranges must not depend on the removed tree semantics.

Examples:

### `applied_to`

Current:

```text
domain: ConditionModel
range: Field | Equation | Scope
```

If the intended subject is a concrete `BoundaryCondition`, `InitialCondition`, `Source`, or `Load`, those objects are not ConditionModel subtypes. The relation contract must later either:

1. use an explicit Interface implemented by applicable condition entities;
2. define a union/domain set of concrete condition types; or
3. introduce a justified common semantic `Condition` type and explicit `is_a` edges.

This study does not choose among those alternatives.

### Other broad relations

The same check is required for `represented_by`, `closed_by`, `parameterized_by`, `defined_on`, and `discretized_by`: their current aggregate domains may be correct when the relation applies to the aggregate model object, but they SHALL NOT be assumed to apply automatically to child concepts through tree inheritance.

## 6. Proposed architecture rule

Add one general clarification:

> Model-category trees in the Core architecture are **semantic decomposition/grouping diagrams**, not taxonomic hierarchies, except where an explicit `is_a` relation is separately declared. In SOL v0.1, the Analysis specializations are the currently accepted Core taxonomy edges; other displayed model-category edges require explicit composition/association Relations before they become canonical machine-readable structure.

This rule does not prescribe one generic `has_component` relation or a large family of typed relations. Relation naming/cardinality should be designed in the next focused step after this classification is independently validated.

## 7. Counterexamples

### C-MT-01 — Field treated as MathematicalModel

If tree indentation implies inheritance, a `Field` can satisfy any relation requiring MathematicalModel. This is false: a field does not by itself define a mathematical representation/model.

Expected: reject implicit inheritance.

### C-MT-02 — BoundaryCondition treated as ConditionModel

Implicit inheritance would let a BC satisfy aggregate ConditionModel contracts, while an aggregate ConditionModel may contain several BCs, sources, and loads.

Expected: reject implicit inheritance.

### C-MT-03 — Mesh treated as NumericalModel

A Mesh cannot substitute for solver-independent discretization/numerical-model semantics.

Expected: reject implicit inheritance.

### C-MT-04 — StationaryAnalysis treated only as component

`StationaryAnalysis` does satisfy the general Analysis contract and should preserve explicit `is_a Analysis`.

Expected: retain inheritance.

### C-MT-05 — backend class hierarchy copied into Core

A backend may implement a BC, field, or solver as a subclass of a common backend object. That does not change the Core classification.

Expected: no Core taxonomy change.

## 8. Research verdict

**Recommend accepting the classification rule before designing remaining relation cardinalities.**

The next step after Validation should be a focused **model-component relation design**, deciding how aggregate model entities explicitly reference their component concepts. Only after those relations are accepted should the remaining cardinality/required-optional matrix be finalized.
