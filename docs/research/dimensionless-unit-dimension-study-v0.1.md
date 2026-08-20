# Dimensionless Unit and Physical-Dimension Study v0.1

**Status:** Research evidence  
**Date:** 2026-08-20  
**Reference backends:** MOOSE, COMSOL, Ansys  
**Purpose:** Refine the SOL boundary among `SemanticQuantity`, `PhysicalDimension`, `Unit`, and `Value`, with special attention to dimensionless quantities.

## 1. Research question

A provisional SOL rule proposed:

- semantic quantities require or imply a physical dimension;
- concrete values may carry units compatible with that dimension.

The unresolved case is a dimensionless quantity such as Poisson ratio, relative permittivity, Mach number, emissivity, or a nondimensional group.

The key questions are:

1. How should SOL represent a dimensionless quantity?
2. Is missing unit metadata equivalent to dimensionlessness?
3. Is physical-dimension compatibility sufficient for semantic unit compatibility?
4. Can these concerns be modeled without introducing a redundant `QuantityKind` abstraction?

## 2. MOOSE evidence

MOOSE solid-mechanics documentation describes Poisson ratio as unitless while other parameters such as Young's modulus, mesh length, and time require consistent units.

This supports treating Poisson ratio as a semantic quantity whose physical-dimension contract is dimension one/dimensionless, even when no explicit runtime unit is supplied.

```text
PoissonRatio
    requires_dimension -> DimensionOne

Value
    magnitude = 0.3
    explicit unit = optional / typically omitted
```

The absence of an explicit unit therefore does not remove the semantic dimension contract.

## 3. COMSOL evidence

COMSOL explicitly treats quantities such as relative permittivity as dimensionless. It also distinguishes plane angle from ordinary dimensionless ratios by using angular units such as `rad` and allowing alternative angular representations such as degrees where appropriate.

This yields two important observations:

```text
RelativePermittivity
    physical dimension -> DimensionOne

PlaneAngle
    physical dimension -> DimensionOne
    permitted representation -> rad / deg
```

The quantities remain semantically distinct even though their SI physical dimension is one.

The earlier draft incorrectly connected COMSOL relative permittivity directly with `%` and `ppm` representations. That claim has been removed; it is not required for the SOL conclusion.

## 4. Ansys evidence

Ansys documentation identifies quantities such as Mach number and relative permittivity as dimensionless. User interfaces may display such values without a visible unit symbol.

This supports the distinction:

```text
MachNumber
    semantic identity = MachNumber
    physical dimension = DimensionOne
    explicit unit symbol = commonly omitted
```

A missing display unit is therefore a representation choice, not the definition of dimensionlessness.

## 5. Cross-backend matrix

| Case | MOOSE | COMSOL | Ansys | SOL implication |
|---|---|---|---|---|
| Poisson ratio | explicitly unitless | dimensionless ratio | dimensionless material quantity | explicit `DimensionOne` contract |
| Relative permittivity | backend-dependent metadata | explicitly dimensionless | explicitly dimensionless | semantic quantity distinct from dimension/value |
| Mach number | not primary reference case | dimensionless in CFD contexts | explicitly dimensionless | missing visible unit does not define dimension |
| Plane angle | unit handling depends on model | represented with `rad`/`deg` while SI dimension is one | typically degree/radian representations | dimension compatibility alone may be insufficient for semantic unit validation |

## 6. Main findings

### Finding D1 — Dimensionless is an explicit dimension contract

A dimensionless quantity should be represented as having physical dimension one rather than as having no dimension.

Conceptually:

```text
PhysicalDimension
    DimensionOne = [0,0,0,0,0,0,0,...]
```

The exact base-dimension vector convention remains an implementation choice.

### Finding D2 — Unit absence is not dimensionlessness

A dimensionless value may have no visible unit symbol, but the absence of an explicit Unit cannot by itself determine the PhysicalDimension.

```text
unit = absent
    !=
dimension = DimensionOne
```

### Finding D3 — Semantic quantity remains distinct from physical dimension

Different quantities may share the same PhysicalDimension while remaining semantically distinct.

```text
MachNumber
PoissonRatio
RelativePermittivity
PlaneAngle
```

may all map to `DimensionOne`, but they are not interchangeable semantic concepts.

This generalizes the same pattern for dimensional quantities, for example Stress and Young's modulus sharing the same physical dimension while retaining distinct semantics.

### Finding D4 — Dimension compatibility may be insufficient for unit compatibility

Because plane angle and ordinary dimensionless ratios both have dimension one, dimension checking alone could incorrectly accept semantically inappropriate units.

Therefore a semantic quantity MAY impose additional unit constraints beyond physical-dimension compatibility.

This does **not** require a separate `QuantityKind` object. The semantic quantity Entity itself already carries the needed identity.

## 7. Refined candidate rules

### U1 — Dimension as semantic contract

> A physical semantic quantity SHOULD declare or imply a `PhysicalDimension`, including `DimensionOne` where applicable.

### U2 — Unit as value representation

> A concrete `Value` MAY carry an explicit `Unit`; when present, that Unit SHALL be dimensionally compatible with the semantic quantity's required PhysicalDimension.

A unit may be omitted where the surrounding model/backend context supplies the unit contract or where serialization convention suppresses an explicit unit symbol.

### U3 — Unit absence is not semantic dimension

> The absence of an explicit Unit SHALL NOT by itself be used to infer `DimensionOne` or any other PhysicalDimension.

### U4 — Semantic quantity is independent of physical dimension

> Semantic quantities with identical PhysicalDimensions SHALL remain distinct ontology concepts when their domain meanings differ.

### U5 — Semantic unit compatibility

> Physical-dimension compatibility is necessary but may not always be sufficient for unit compatibility. A semantic quantity MAY impose additional constraints on units used to represent its Values.

`U5` is expressed as a constraint on the existing SemanticQuantity Entity. No `QuantityKind` abstraction is introduced.

## 8. Candidate core graph

```text
SemanticQuantity
      │
      ├── requires_dimension ──> PhysicalDimension
      │
      ├── may constrain ───────> Unit / UnitSet
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
                    └── expressed_in ──> Unit   [optional where context resolves it]
                                             │
                                             └── has_dimension
                                                      │
                                                      ▼
                                              PhysicalDimension
```

Examples:

```text
RelativePermittivity
    ├── requires_dimension -> DimensionOne
    └── allowed representation -> ordinary dimension-one value

PlaneAngle
    ├── requires_dimension -> DimensionOne
    └── allowed_units -> {rad, deg}
```

No intermediate `QuantityKind` node is required because the SemanticQuantity itself already distinguishes `RelativePermittivity` from `PlaneAngle`.

## 9. Important modeling implication

`SemanticQuantity`, `PhysicalDimension`, `Unit`, and `Value` remain distinct concepts:

```text
SemanticQuantity
      !=
PhysicalDimension
      !=
Unit
      !=
Value
```

Dimension compatibility and semantic unit compatibility are validation relations between these concepts, not identity relations.

## 10. Minimal-abstraction principle

This study provides a concrete application of the SOL Entity boundary rule:

> Do not introduce a new Entity when existing semantic identity plus Relations and Constraints already express the required meaning.

The proposed `QuantityKind` abstraction duplicated SemanticQuantity identity and has therefore been rejected.

## 11. Decision status

This document remains research evidence, not yet an ADR.

The corrected cross-backend evidence supports U1-U5 as candidates for a dedicated Unit/PhysicalDimension ADR. `QuantityKind` is explicitly excluded from the candidate core model.
