# SOL v0.1 Model-Component Relations and `applied_to` Remediation v0.2

**Role:** Research  
**Date:** 2026-08-20  
**Revision scope:** MC-01 and MC-02 only

## 1. Preserved decisions

This revision does not reopen the accepted parts of v0.1:

- one generic `includes_component` relation;
- direct-only, non-transitive membership;
- non-owning and order-independent semantics;
- generic `includes_component` requiredness `0..*`;
- explicit `applied_to` source family `BoundaryCondition | InitialCondition | Source | Load`;
- `applied_to` target family `Field | Equation | Scope`;
- `applied_to` source cardinality `1..*`;
- no artificial `Condition` superclass;
- no one-use Interface introduced solely for `applied_to`.

## 2. Canonical endpoint type matching

All relation endpoint type checks in this remediation use canonical semantic type identity and the resolved canonical subtype closure.

For an actual endpoint instance `x` and an allowed endpoint type `T`:

```text
matches(x, T)
IFF
exists resolved consistent semantic type X of x:
  X = T OR X <: T
```

If an instance has multiple consistent semantic types, one matching type is sufficient. Type inconsistency is a prior validation failure; a validator SHALL NOT choose among inconsistent types.

Backend implementation inheritance, declaration order, file order, and local spelling do not participate in this match.

### Example

```text
TemperatureField is_a Field
T : TemperatureField
```

Then `T` satisfies an allowed endpoint type `Field`.

This rule applies equally to source and target endpoints for both `includes_component` and `applied_to`.

## 3. `includes_component` allowed-pair contract is authoritative

`includes_component` does not use `range: Entity` as a normative Core range declaration.

The authoritative validity contract is the canonical allowed-pair set:

```text
AllowedPair = (allowed_source_type, allowed_target_type)
```

An edge:

```text
s includes_component t
```

is valid iff there exists at least one allowed pair `(S,T)` such that:

```text
matches(s, S)
AND
matches(t, T)
```

The canonical Core allowed pairs are induced by this matrix:

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

A documentation/tooling layer MAY derive summary sets:

```text
domain_summary = all source types appearing in AllowedPair
range_summary  = all target types appearing in AllowedPair
```

but these summaries are not independent semantic authorities and SHALL NOT weaken or broaden the allowed-pair constraint.

No Core Entity Type named `Entity` is introduced by this decision.

## 4. Extension behavior

Domain/backend ontology extensions inherit compatibility through ordinary subtype semantics.

Example:

```text
TemperatureField is_a Field
ElectronTemperatureField is_a TemperatureField
```

Both may be direct targets of:

```text
MathematicalModel includes_component ...
```

because both satisfy the allowed target type `Field` through subtype closure.

An extension MAY introduce additional relation constraints or a different relation for new structural semantics, but SHALL NOT mutate the Core allowed-pair table by declaration order or silently weaken it.

A new source type unrelated by subtype to an allowed Core source type does not gain `includes_component` permission merely because it is an Entity.

## 5. `includes_component` relation semantics

```text
includes_component
  allowed_pairs: normative matrix above
  source cardinality: 0..*
  ownership: non-owning
  order semantics: none
  transitive: false
```

No transitive edge is inferred:

```text
SimulationModel includes_component MathematicalModel
MathematicalModel includes_component TemperatureField
```

does not assert:

```text
SimulationModel includes_component TemperatureField
```

Recursive traversal is a query/view operation only.

## 6. `applied_to` repaired contract

The canonical allowed source family is:

```text
BoundaryCondition
InitialCondition
Source
Load
```

The canonical allowed target family is:

```text
Field
Equation
Scope
```

Endpoint conformance uses the subtype-aware `matches()` rule from section 2.

Therefore:

```text
WallTemperatureBC is_a BoundaryCondition
TemperatureField is_a Field
WallBoundaryScope is_a Scope
```

allows:

```text
bc applied_to temperature
bc applied_to wall
```

without adding an artificial superclass.

Each conforming source instance SHALL have:

```text
applied_to cardinality = 1..*
```

The relation is unordered and does not encode backend handles or native selection IDs.

## 7. Counterexample closure

### MC-01A — subtype target

```text
TemperatureField is_a Field
math includes_component temperature
```

Expected: valid.

### MC-01B — subtype condition

```text
WallTemperatureBC is_a BoundaryCondition
wall-bc applied_to wall-scope
```

Expected: valid if the target is equal/subtype of `Field | Equation | Scope`.

### MC-01C — unrelated type

```text
MaterialModel includes_component WallTemperatureBC
```

Expected: invalid; no allowed pair is satisfied.

### MC-01D — multiple consistent typing

An endpoint with two consistent semantic types, one of which satisfies the allowed endpoint type, matches once. Type multiplicity does not create duplicate relation counts.

### MC-01E — inconsistent typing

An endpoint with a prior semantic type inconsistency is invalid before allowed-pair evaluation; validators SHALL NOT choose a favorable type.

### MC-02A — no Core `Entity` wildcard

A parser SHALL NOT look for a Core Entity Type named `Entity` to validate `includes_component`.

Expected: validity is determined exclusively by allowed pairs plus canonical subtype closure.

### MC-02B — summary cannot broaden

A tooling-generated `range_summary` containing `BoundaryCondition` does not permit:

```text
MaterialModel includes_component BoundaryCondition
```

unless an authoritative allowed pair matches.

## 8. Research verdict

MC-01 and MC-02 are resolved without changing the accepted v0.1 relation direction.

**Ready for focused final Validation.**
