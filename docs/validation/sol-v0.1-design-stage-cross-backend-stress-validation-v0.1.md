# SOL v0.1 Design-Stage Cross-Backend Stress Validation v0.1

**Role:** Independent Validation  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`  
**Stage:** Architecture / contract design validation  

## 1. Evaluation boundary

This review deliberately separates:

```text
architecture / contract validity
!= reference implementation readiness
!= adapter implementation completeness
!= installed backend execution validity
```

Backend installation, proprietary license availability, API credentials, local runtime availability, and full Adapter V&V are **out of scope for the current design stage**. Their absence SHALL NOT block an architecture decision. A backend runtime is used here only if already available and only for an optional minimal smoke case.

The validation question is narrower: can the accepted/proposed SOL semantic contracts represent the same small simulation meaning across MOOSE, COMSOL, and Ansys without changing Core semantics or introducing backend-native Core concepts?

## 2. Inputs

Normative / candidate contracts:

- ADR-0006 — Semantic preservation across backends
- ADR-0007 — Constraint architecture and composition
- ADR-0009 — Identity, namespace, package, and versioning
- ADR-0010 — Profile and Backend Mapping Contract
- Proposed ADR-0011 — MappingPlan Determinism and Executable Backend Target
- Proposed ADR-0012 — Qualified Relation Cardinality

Repository evidence:

- `docs/research/cross-backend-semantic-mapping-matrix-v0.1.md`
- `docs/validation/cross-backend-reference-validation-plan-v0.1.md`
- final contract validation artifacts under `docs/validation/`

External evidence was checked only against official backend documentation.

## 3. Minimal thermal reference semantics

Use one backend-independent semantic case:

```text
Material
  has ThermalConductivity = constant k

TemperatureField

HeatConductionModel
  acts_on TemperatureField
  parameterized_by ThermalConductivity

TemperatureBoundaryCondition
  targets TemperatureField
  defined_on BoundaryScope
  prescribed_value 300 K

Analysis = Stationary
```

A transient variant may add density, specific heat, and a time-derivative obligation, but transient execution is not required for this design-stage gate.

### 3.1 MOOSE projection

Official MOOSE Heat Transfer documentation represents the same semantics through distinct native constructs: a temperature nonlinear variable, `HeatConduction` kernel, optional `HeatConductionTimeDerivative`, material properties such as thermal conductivity/density/specific heat, `DirichletBC` or `FunctionDirichletBC`, and an `Executioner` for transient execution. The `DirichletBC` contract separately names a variable, boundary, and value.

Design-stage assessment:

- one SOL `TemperatureField` does not require one universal backend object;
- the heat equation may lower to multiple executable objects/kernels;
- scope, field reference, material-property reference, and prescribed value remain distinct semantic roles;
- this is a normal **transformed** realization, not semantic loss;
- no sixth MappingRule field or fifth MappingClaim field is required by this case.

**Verdict: PASS.**

Official evidence:

- https://mooseframework.inl.gov/source/bcs/DirichletBC.html
- https://mooseframework.inl.gov/moose/source/kernels/HeatConductionTimeDerivative.html
- https://mooseframework.inl.gov/modules/heat_transfer/tutorials/introduction/

### 3.2 COMSOL projection

Official COMSOL documentation represents stationary thermal semantics through a `Heat Transfer in Solids` physics interface, geometry-scoped physics features, a `Temperature` boundary feature, and a `Stationary` Study. The programming guide exposes separate creation/binding operations for the physics interface and boundary feature. The Solid heat-transfer contract carries density, heat capacity, thermal conductivity, and heat-source semantics.

Design-stage assessment:

- SOL `PhysicsModel`, `BoundaryCondition`, `Scope`, `MaterialProperty`, and `Analysis` can map to separate COMSOL feature nodes without changing SOL identities;
- backend feature tags are backend-local handles, not SOL semantic identities;
- COMSOL's physics-feature tree supports the Profile/Adapter separation rather than requiring Core to mirror the tree;
- exact/transformed classification can be made from semantic preservation independently of native object count.

**Verdict: PASS.**

Official evidence:

- https://doc.comsol.com/6.4/doc/com.comsol.help.comsol/application_programming_guide.15.25.html
- https://doc.comsol.com/6.4/doc/com.comsol.help.heat/heat_ug_interfaces.08.31.html
- https://doc.comsol.com/6.4/doc/com.comsol.help.heat/heat_ug_ht_features.09.099.html
- https://doc.comsol.com/6.4/doc/com.comsol.help.heat/heat_ug_interfaces.08.16.html

### 3.3 Ansys projection

Official Ansys documentation defines Steady-State Thermal as the equilibrium thermal analysis, requires thermal conductivity for assigned materials, and supports Temperature, Convection, Heat Flux, Heat Generation, and related thermal boundary/load types. Temperature boundaries are scoped to selected faces.

Design-stage assessment:

- `StationaryAnalysis`, `ThermalConductivity`, `BoundaryScope`, and `TemperatureBoundaryCondition` remain stable semantic concepts despite Ansys using Analysis/Engineering Data/load objects;
- scoping remains backend-local realization of SOL Scope semantics;
- solver/analysis object structure does not justify importing Ansys-native taxonomy into Core.

**Verdict: PASS.**

Official evidence:

- https://ansyshelp.ansys.com/public/Views/Secured/corp/v251/en/wb_sim/ds_static_thermal_analysis_type.html
- https://ansyshelp.ansys.com/public/Views/Secured/Electronics/v251/en/Subsystems/Mechanical/Content/Mechanical/ThermalSolutions.htm
- https://ansyshelp.ansys.com/public/Views/Secured/Electronics/v242/en/Subsystems/Mechanical/Content/Mechanical/TemperatureBoundary.htm

## 4. Minimal plasma/QRC stress case

Use the semantic constraint:

```text
Reaction.products contains at least one NegativeIonSpecies
```

expressed by QRC as:

```yaml
type: cardinality
relation: products
qualifier:
  target_type: NegativeIonSpecies
min: 1
```

COMSOL's official Plasma documentation has explicit `Species`, `Reaction`, and reaction-group concepts, with reaction/species features scoped to plasma/heavy-species model selections. Species may be neutral, ion, or electron and reactions introduce participating species.

This validates the need to count semantic reaction-product targets independently of the backend's feature hierarchy. It does **not** justify a generic collection query language or backend reaction-node vocabulary in Core.

For MOOSE or Ansys installations that do not provide an equivalent generic plasma feature set, the correct design-stage outcome is `unsupported` for that Profile/target, not mutation of the valid SOL plasma model. Lack of a particular installed plasma module is a **Backend limitation / Adapter coverage question**, not an architecture defect.

**QRC architecture verdict: PASS.**

Official COMSOL evidence:

- https://doc.comsol.com/6.4/doc/com.comsol.help.plasma/plasma_ug_heavy_species.08.07.html
- https://doc.comsol.com/6.4/doc/com.comsol.help.plasma/plasma_ug_heavy_species.08.09.html
- https://doc.comsol.com/6.4/doc/com.comsol.help.plasma/plasma_ug_heavy_species.08.08.html

## 5. Multi-product / orchestration stress case

ADR-0011 requires component-keyed target identity and explicit cross-component transfer semantics. Official Ansys System Coupling documentation treats coupled analyses as multiple participants. A coupling interface connects participant regions on two sides, while each individual data transfer is directional and has source/target variables. System Coupling coordinates independent solver executions rather than collapsing them into one physics object.

This supports:

- explicit target components rather than a vendor umbrella target;
- component-local adapter/release/capability identity;
- explicit orchestration identity;
- directional transfer actions for source → target exchange;
- local versus cross-component comparison contexts.

No Core simulation concept needs to know `System Coupling`, participant-side names, or native transfer object types.

**ADR-0011 multi-component architecture verdict: PASS.**

Official evidence:

- https://ansys.synopsys.com/en-gb/products/system-coupling
- https://ansyshelp.ansys.com/public/Views/Secured/corp/v251/en/pdf/System_Coupling_Users_Guide.pdf

## 6. Counterexample-oriented findings

### C-DESIGN-01 — Backend object-count mismatch

MOOSE can realize one semantic heat equation through several kernels while COMSOL can place much of the same physics under one physics interface with feature nodes.

**Result:** not a defect. Mapping cardinality must remain unconstrained by 1:1 assumptions. Current architecture handles this as transformed realization.

### C-DESIGN-02 — Backend-local scope handles

MOOSE boundary/sideset names, COMSOL geometric selections, and Ansys face/named-selection scoping have incompatible native identities.

**Result:** not a defect. SOL Scope identity remains semantic; backend handles stay in Profile/Adapter realization evidence.

### C-DESIGN-03 — Plasma support absent on a target

A valid plasma model may have no faithful realization in a selected generic backend/profile.

**Result:** `unsupported`, classified as backend/profile coverage, not semantic invalidity. ADR-0006/0010 boundary is preserved.

### C-DESIGN-04 — Coupled target requires participant-specific transfer

Ansys System Coupling has participant/interface/transfer structure that cannot be represented safely by an unscoped vendor target.

**Result:** ADR-0011 component-keyed target and explicit transfer action are sufficient. No Core change required.

### C-DESIGN-05 — Typed subset reaction-product constraint

Ordinary unqualified relation cardinality cannot express "at least one product that is a NegativeIonSpecies" without overconstraining all products or adding a generic predicate language.

**Result:** ADR-0012 QRC is the minimal justified Core/schema extension. No seventh constraint family required.

## 7. Defect classification

| Finding | Classification | Architecture action |
|---|---|---|
| MOOSE multi-kernel lowering | Expected backend transformation | none |
| COMSOL feature hierarchy | Expected backend transformation | none |
| Ansys load/Engineering Data organization | Expected backend transformation | none |
| Backend-local scope IDs | Adapter/Profile concern | none |
| Generic target lacks plasma support | Backend limitation / Profile coverage | none |
| System Coupling participant transfer | Adapter/Profile orchestration concern | covered by ADR-0011 |
| QRC typed product count | Architecture requirement | covered by ADR-0012 |
| Installation/license/runtime availability | Out-of-scope external factor at design stage | none |

No new Architecture defect was found.

## 8. Contract verdict

| Contract | Design-stage verdict |
|---|---|
| ADR-0010 Profile/Backend mapping direction | **Accept** |
| ADR-0011 MappingPlan determinism / BackendTarget | **Accept** |
| ADR-0012 Qualified Relation Cardinality | **Accept** |
| MappingRule five-field public shape | **Accept at architecture level** |
| MappingClaim four-field public shape | **Accept at architecture level** |
| Core/backend separation | **Accept** |

The synthetic Phase-1 reference harness remains useful implementability evidence, but it is **supplementary** and is not an architecture-freeze prerequisite. Any remaining harness defect is Validation-tooling work and SHALL NOT block the design-stage gate unless it exposes a new architecture counterexample.

## 9. Architecture-freeze recommendation

The design-stage architecture gate is satisfied for the remediation scope reviewed here:

1. independent contract review has closed the known AR-01/02/03/04 ambiguities;
2. MOOSE, COMSOL, and Ansys official documentation supports the Core/Profile/Adapter separation;
3. the same minimal thermal semantics remain stable across all three backend structures;
4. the plasma/QRC case is expressible without a generic collection-query expansion;
5. multi-product orchestration is representable without vendor-native Core concepts;
6. no new Core primitive or MappingRule/MappingClaim top-level field is required by the stress cases.

**Validation recommendation: ADR-0011 and ADR-0012 are ready for acceptance, and the reviewed SOL v0.1 architecture scope is ready for design freeze.**

This recommendation does not claim Adapter implementation completeness or backend execution validation. Those belong to later implementation/adapter projects.
