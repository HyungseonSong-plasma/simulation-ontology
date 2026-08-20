# ADR 0004 — Unit and Physical-Dimension Semantics

**Status:** Accepted  
**Date:** 2026-08-20  
**Scope:** Simulation Ontology Language v0.1

## Context

Cross-backend validation against MOOSE, COMSOL, and Ansys, together with SI dimensional analysis, showed that SOL must distinguish semantic quantities, physical dimensions, units, and concrete values.

The supporting research is recorded in:

- `docs/research/dimensionless-unit-dimension-study-v0.1.md`
- `docs/research/cross-backend-semantic-mapping-matrix-v0.1.md`

A temporary `QuantityKind` abstraction was considered but rejected because it duplicated the identity already carried by the semantic quantity itself.

## Decision

### U1 — PhysicalDimension as semantic contract

> **A physical semantic quantity SHOULD declare or imply a `PhysicalDimension`. Dimension-one quantities SHALL be represented explicitly as having physical dimension one rather than being inferred from missing unit metadata.**

Examples:

```text
Pressure              -> M L^-1 T^-2
ThermalConductivity   -> M L T^-3 Θ^-1
RelativePermittivity  -> 1
PoissonRatio          -> 1
PlaneAngle            -> 1
```

`PhysicalDimension` is independent of the semantic identity of the quantity.

### U2 — Unit as concrete value representation

> **A concrete `Value` MAY carry an explicit `Unit`. When a Unit is present, it SHALL be compatible with the PhysicalDimension required by the semantic quantity or evaluation context.**

A unit may be omitted when the surrounding backend/model context resolves the unit contract or when the chosen serialization omits the coherent unit one.

### U3 — Unit absence is not dimensionlessness

> **The absence of an explicit Unit SHALL NOT by itself imply physical dimension one.**

Missing unit metadata may mean:

- the backend supplies a unit context;
- the serialization omits the unit;
- the quantity is dimension-one;
- the metadata is incomplete.

These cases are semantically distinct.

### U4 — Semantic quantity identity is independent of PhysicalDimension

> **Two semantic quantities MAY share the same PhysicalDimension while remaining distinct ontology concepts.**

Examples:

```text
Stress          -> Pressure dimension
YoungsModulus   -> Pressure dimension
Pressure        -> Pressure dimension
```

and:

```text
RelativePermittivity -> dimension one
PoissonRatio         -> dimension one
PlaneAngle           -> dimension one
```

Shared dimension does not imply semantic equivalence.

### U5 — Quantity-specific unit constraints

> **Physical-dimension compatibility is necessary but may not be sufficient for semantic unit compatibility. A semantic quantity MAY impose additional constraints on the units used to represent its Values.**

For example, `PlaneAngle` may permit angular units such as `rad` or `deg`, while `RelativePermittivity` may require the coherent unit one / no explicit symbol in the canonical representation.

This additional constraint is attached directly to the semantic quantity. SOL does not introduce a separate `QuantityKind` layer.

## Core semantic separation

The following concepts are distinct:

```text
SemanticQuantity
      !=
PhysicalDimension
      !=
Unit
      !=
Value
```

Conceptual graph:

```text
SemanticQuantity
      │
      ├── requires_dimension ──> PhysicalDimension
      │
      ├── unit_constraint ─────> Unit / UnitConstraint   [optional]
      │
      └── has_value_definition
                    │
                    ▼
             ValueDefinition
                    │
               evaluates_to
                    ▼
                  Value
                    │
                    └── expressed_in ──> Unit   [optional]
                                             │
                                             └── has_dimension
                                                      │
                                                      ▼
                                              PhysicalDimension
```

## Rejected abstraction — QuantityKind

A separate `QuantityKind` layer was considered:

```text
PlaneAngle -> QuantityKind(PlaneAngle)
```

but rejected because the semantic quantity Entity already carries that identity. The extra layer would duplicate information without adding independent semantics.

The preferred model is therefore:

```text
PlaneAngle
    ├── requires_dimension -> DimensionOne
    └── unit_constraint -> {rad, deg}
```

rather than:

```text
PlaneAngle
    └── quantity_kind -> PlaneAngle
```

## Consequences

1. SOL must model semantic quantity identity separately from physical dimension.
2. SOL must support explicit dimension-one quantities.
3. Values may carry units, but unit presence is not required in every serialization/backend context.
4. Unit validation must include dimensional compatibility.
5. Some quantities may require stricter unit validation than dimensional compatibility alone.
6. `QuantityKind` is not part of the SOL v0.1 core model.
7. Backend unit metadata must be normalized into the semantic quantity/value/unit model rather than copied directly.

## Interaction with ADR 0002

ADR 0002 remains unchanged:

- `Value` is an evaluated typed datum.
- `ValueDefinition` describes how a Value is obtained.
- value shape and evaluation mechanism are orthogonal.

ADR 0004 adds that physical-dimension requirements belong to the semantic quantity contract, while concrete unit representation belongs to the evaluated Value or its resolved context.

## Deferred decisions

This ADR does not decide:

- the canonical representation of PhysicalDimension vectors;
- the canonical Unit ontology or registry;
- whether Unit is a Core Entity or an external vocabulary reference;
- affine units and offset handling (for example Celsius/Kelvin conversions);
- logarithmic units;
- uncertainty propagation;
- exact canonicalization and unit-conversion policy;
- whether `unit_constraint` is represented as a Relation, Constraint, or dedicated validation construct.
