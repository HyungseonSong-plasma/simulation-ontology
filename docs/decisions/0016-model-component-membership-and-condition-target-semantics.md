# ADR-0016: Model-Component Membership and Condition-Target Semantics

**Status:** Accepted  
**Date:** 2026-08-20  
**Amends:** frozen SOL v0.1 baseline through ADR-0015 for the narrow scope below

## Context

ADR-0014 removed implicit `children` inheritance semantics from the machine-readable Core. Subsequent consolidation showed that the architecture tree still carried useful structural grouping that needed an explicit relation, and that the existing relation:

```text
ConditionModel -> applied_to -> Field | Equation | Scope
```

incorrectly depended on `BoundaryCondition`, `InitialCondition`, `Source`, and `Load` behaving like implicit `ConditionModel` subtypes.

Independent Validation accepted a minimal remediation after requiring deterministic subtype-aware endpoint matching and removal of an ambiguous `range: Entity` metatype/Core-type interpretation.

## Decision

### 1. Introduce `includes_component`

SOL v0.1 defines one generic direct structural-membership relation:

```text
includes_component
```

It means that an aggregate/model context directly includes the referenced semantic component in its definition.

The relation is:

- non-owning;
- non-transitive;
- order-independent;
- generic source cardinality `0..*`.

It does not imply deletion/lifecycle cascade and does not replace semantic relations such as `represented_by`, `closed_by`, `parameterized_by`, `defined_on`, `discretized_by`, or `solved_by`.

### 2. Allowed-pair matrix is the semantic authority

`includes_component` validity SHALL be determined by this canonical allowed-pair matrix:

| Allowed source type | Allowed target types |
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

A documentation or tooling layer MAY derive domain/range summaries from the matrix, but those summaries are not an independent semantic authority and SHALL NOT broaden the allowed pairs.

No Core Entity Type named `Entity` is introduced by this ADR.

### 3. Endpoint matching is subtype-aware

For actual endpoint instance `x` and allowed endpoint type `T`:

```text
matches(x,T)
IFF
exists resolved consistent semantic type X of x:
  X = T OR X <: T
```

Canonical semantic identity and the canonical subtype closure SHALL be used.

If an endpoint has multiple consistent semantic types, one matching type is sufficient. Type inconsistency is a prior validation failure; a validator SHALL NOT choose among inconsistent types.

Backend inheritance, declaration order, serialization order, and source-file location are non-normative.

### 4. `includes_component` is direct-only

From:

```text
SimulationModel includes_component MathematicalModel
MathematicalModel includes_component TemperatureField
```

SOL SHALL NOT infer:

```text
SimulationModel includes_component TemperatureField
```

Recursive component traversal MAY be exposed as a query/view but is not an additional stored semantic edge.

### 5. Component reuse is permitted

Because `includes_component` is non-owning, one compatible semantic component MAY be referenced by more than one aggregate/model context unless another accepted constraint forbids that particular reuse.

General membership does not itself impose completeness. A zero-component aggregate remains structurally valid at this generic Core layer unless another type/domain/profile constraint requires content.

### 6. Repair `applied_to`

The canonical source family for `applied_to` is:

```text
BoundaryCondition
InitialCondition
Source
Load
```

The canonical target family is:

```text
Field
Equation
Scope
```

Subtype-aware endpoint matching from section 3 applies to both source and target.

Each conforming source instance SHALL have:

```text
applied_to cardinality = 1..*
```

The relation is unordered and SHALL NOT encode backend-native selection IDs, sideset names, feature tags, or solver handles.

The aggregate `ConditionModel` is not itself a valid `applied_to` source merely because it includes condition/forcing components.

### 7. No artificial superclass or one-use Interface

SOL v0.1 does not introduce a generic `Condition` superclass solely to simplify `applied_to`, and does not introduce an Interface used only for this relation.

If future evidence demonstrates an independently reusable condition-target capability contract, that may be handled by a separate ADR.

### 8. Backend boundary remains unchanged

MOOSE/COMSOL/Ansys object trees and ownership models remain backend representations. This ADR captures only Core semantic grouping and condition/forcing targets.

Backend installation, licensing, runtime availability, production Adapter behavior, and execution V&V are outside this design-stage decision.

## Consequences

### Positive

- architecture-tree grouping has an explicit non-taxonomic semantic representation;
- Core no longer depends on implicit `ConditionModel` inheritance for condition targeting;
- subtype-specialized domain entities such as `TemperatureField` and specialized BCs remain valid through canonical subtype closure;
- backend tree ownership does not leak into Core;
- structural membership and semantic relations remain orthogonal.

### Costs

- validators must implement allowed-pair matching with canonical subtype closure;
- the machine-readable relation schema must distinguish authoritative allowed-pair constraints from derived summaries;
- remaining Core relation cardinalities still require separate evidence.

## Validation evidence

- `docs/research/sol-v0.1-model-component-relations-and-applied-to-remediation-v0.1.md`
- `docs/validation/sol-v0.1-model-component-relations-and-applied-to-independent-review-v0.1.md`
- `docs/research/sol-v0.1-model-component-relations-and-applied-to-remediation-v0.2.md`
- `docs/validation/sol-v0.1-model-component-relations-and-applied-to-focused-final-review-v0.2.md`

## Freeze amendment

The existing design-stage freeze remains in force. This ADR amends only structural component membership, endpoint type matching for this contract, and `applied_to` source/cardinality semantics.

## Decision summary

SOL v0.1 uses one non-owning, non-transitive, unordered `includes_component` relation governed by a canonical allowed-pair matrix with subtype-aware endpoint matching, and repairs `applied_to` to operate directly on `BoundaryCondition | InitialCondition | Source | Load` with `1..*` targets in `Field | Equation | Scope`.
