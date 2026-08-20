# Dimensionless Unit and Physical-Dimension Study v0.1

**Status:** Research evidence  
**Date:** 2026-08-20  
**Reference backends:** MOOSE, COMSOL, Ansys  
**Purpose:** Test whether SOL should equate dimensionless quantities with missing units, and refine the proposed Unit/PhysicalDimension boundary.

## 1. Research question

A provisional SOL rule proposed:

- semantic quantities require a physical dimension;
- concrete values carry units compatible with that dimension.

The unresolved case is a dimensionless quantity such as Poisson ratio, relative permittivity, Mach number, emissivity, or a nondimensional group.

The key question is:

> Does `dimensionless` mean `unit absent`, or is dimensionlessness an explicit physical-dimension state that may still have a unit-like representation?

## 2. MOOSE evidence

MOOSE solid-mechanics documentation explicitly describes Poisson ratio as unitless while other parameters such as Young's modulus, mesh length, and time require consistent units.

This supports treating Poisson ratio as a semantic quantity whose physical-dimension contract is dimensionless, even though no explicit runtime unit is required.

```text
PoissonRatio
    requires_dimension -> Dimensionless

Value
    magnitude = 0.3
    unit = optional / canonical-one
```

MOOSE therefore provides evidence that a missing explicit unit does not mean that the quantity lacks a dimension contract.

## 3. COMSOL evidence

COMSOL explicitly marks relative permittivity and many ratios/fractions as `dimensionless`. It also defines `%` and `ppm` as units for dimensionless values.

This is decisive evidence that:

```text
dimensionless != no representational unit
```

A dimensionless value may be represented as:

```text
0.5
50 %
500000 ppm
```

while retaining the same zero-dimensional physical-dimension vector.

COMSOL also treats plane angle using `rad`, while dimensional analysis still treats radians as dimensionless in SI. This shows that a dimensionless physical dimension does not erase semantic quantity kind or preferred unit representation.

Therefore:

```text
PlaneAngle
    requires_dimension -> Dimensionless
    preferred/compatible unit -> rad

RelativePermittivity
    requires_dimension -> Dimensionless
    ordinary representation -> unit one / no explicit symbol
```

These are not the same semantic quantity even though their base-dimension vectors are identical.

## 4. Ansys evidence

Ansys documentation identifies Mach number as dimensionless and commonly displays its unit field as empty (`[ ]` or `—`). Ansys also describes relative permittivity as a dimensionless ratio.

This supports the same distinction:

```text
MachNumber
    semantic identity = MachNumber
    physical dimension = Dimensionless
    explicit unit symbol = usually absent
```

The absence of a display unit is therefore a presentation choice, not evidence that the quantity has no physical-dimension classification.

## 5. Cross-backend matrix

| Case | MOOSE | COMSOL | Ansys | SOL implication |
|---|---|---|---|---|
| Poisson ratio | explicitly unitless | dimensionless material ratio | typically dimensionless Engineering Data quantity | explicit `Dimensionless` contract |
| Relative permittivity | backend-dependent metadata | explicitly dimensionless | explicitly dimensionless | semantic quantity distinct from its value/unit |
| Mach number | not primary reference case | dimensionless in CFD contexts | explicitly dimensionless, unit field empty | dimensionless does not require explicit unit symbol |
| Percent / ppm | may be represented numerically/conversion mechanisms | explicit units for dimensionless values | representation depends on product/context | dimensionless may still have representational units |
| Plane angle | unit handling depends on model | SI unit `rad`; zero-dimensional in base SI dimensions | often degree/radian representations | quantity kind must remain distinct from base dimension |

## 6. Main findings

### Finding D1 — Dimensionless is a real dimension contract

`Dimensionless` should be represented explicitly in SOL rather than inferred from the absence of a unit.

Conceptually:

```text
PhysicalDimension
    Dimensionless = [0,0,0,0,0,0,0,...]
```

The exact base-dimension vector convention remains an implementation choice.

### Finding D2 — Unit absence and dimensionlessness are not equivalent

A value can be dimensionless and have no visible unit, but a missing unit alone does not determine whether the quantity is dimensionless.

```text
unit = absent
    !=
dimension = Dimensionless
```

### Finding D3 — Dimensionless quantities may have representational units

Percent and ppm are representational units/scales for dimensionless values. Radian is another important case: its base physical dimension is dimensionless, while the semantic quantity remains plane angle.

Therefore a Unit must not be assumed to correspond one-to-one with a unique semantic quantity.

### Finding D4 — Semantic quantity remains necessary even when dimensions match

Poisson ratio, Mach number, relative permittivity, refractive index, emissivity, and plane angle may all share a dimensionless base-dimension contract while remaining semantically different quantities.

```text
SemanticQuantity != PhysicalDimension
```

This generalizes the earlier insight that stress and Young's modulus share dimensions but not semantic identity.

## 7. Refined candidate rules

### U1 — Dimension as semantic contract

> A physical semantic quantity SHOULD declare or imply a `PhysicalDimension`, including an explicit `Dimensionless` dimension where applicable.

### U2 — Unit as value representation

> A concrete dimensional or dimensionless `Value` MAY carry an explicit `Unit`; when present, the Unit SHALL be dimensionally compatible with the semantic quantity's required PhysicalDimension.

This refines the previous wording that implied every dimensional Value must necessarily carry an explicit unit. A unit may be omitted when the surrounding model/backend context supplies the unit contract or when the canonical representation is unit one.

### U3 — Unit absence is not semantic dimension

> The absence of an explicit Unit SHALL NOT be used by itself to infer `Dimensionless`.

### U4 — Quantity kind is independent of physical dimension

> Semantic quantities with identical PhysicalDimensions SHALL remain distinct ontology concepts when their domain meaning differs.

Examples:

```text
MachNumber          -> Dimensionless
PoissonRatio        -> Dimensionless
RelativePermittivity-> Dimensionless
PlaneAngle          -> Dimensionless
```

but:

```text
MachNumber != PoissonRatio != RelativePermittivity != PlaneAngle
```

## 8. Candidate core graph

```text
SemanticQuantity
      │
      ├── requires_dimension ──> PhysicalDimension
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

For a dimensionless quantity:

```text
MachNumber
    └── requires_dimension -> Dimensionless

Value(0.8)
    └── expressed_in -> UnitOne / omitted by serialization policy
```

For percentage representation:

```text
Porosity
    └── requires_dimension -> Dimensionless

Value(50)
    └── expressed_in -> Percent
           └── has_dimension -> Dimensionless
```

## 9. Important modeling implication

`PhysicalDimension`, `Unit`, `SemanticQuantity`, and `Value` are four distinct concepts:

```text
SemanticQuantity
      !=
PhysicalDimension
      !=
Unit
      !=
Value
```

Dimension compatibility is a validation relation between them, not an identity relation.

## 10. Decision status

This document is research evidence, not yet an ADR.

The cross-backend dimensionless test strengthens U1 and supports revised U2 plus new U3/U4. These rules are suitable candidates for a dedicated Unit/PhysicalDimension ADR after review.
