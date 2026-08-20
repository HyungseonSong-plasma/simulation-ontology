# UnitReference Stress Test v0.1

**Status:** Research evidence  
**Date:** 2026-08-20  
**Reference systems:** MOOSE, COMSOL, Ansys  
**Purpose:** Test whether `UnitReference + DimensionVector` is sufficient for SOL across dimensionless quantities, compound units, and offset temperature units.

## 1. Cases

Three cases were tested:

1. dimensionless quantities such as relative permittivity;
2. compound units such as thermal conductivity `W/(m*K)`;
3. offset temperature units such as `degC` versus `K`.

## 2. Dimensionless case

COMSOL explicitly marks relative permittivity as dimensionless. Ansys likewise defines relative permittivity/permeability as dimensionless ratios. This supports representing dimension one as a zero exponent vector rather than by missing unit metadata.

```text
RelativePermittivity
  dimension = [0,0,0,0,0,0,0]
```

A unit may be absent or resolve to unit one depending on serialization and metrology policy.

## 3. Compound-unit case

MOOSE `MooseUnits` resolves parsed units to combinations of the seven SI base units plus exponents and a prefactor. It explicitly supports compound expressions such as `N*m`, `kg*(m/s)^2`, `1/s`, and derived units including `W` and `Pa`.

MOOSE heat-conduction material parameters themselves commonly state `Unit: (no unit assumed)`, showing again that backend metadata may be weaker than SOL semantics.

COMSOL and Ansys use explicit compound-unit representations for thermal quantities. Ansys unit tables distinguish thermal conductivity as watts per kelvin and meter.

Result: `UnitReference + DimensionVector` is sufficient for ordinary multiplicative compound units, provided the metrology adapter resolves the unit to a canonical dimension plus conversion metadata.

## 4. Offset temperature case

This case exposes a stronger requirement.

COMSOL distinguishes absolute temperature from differential temperature. `100[degC]` is interpreted as 373.15 K, while a Celsius unit occurring inside a dimension that is not pure temperature is treated as differential and no offset is applied.

MOOSE `MooseUnits` follows a closely related rule: `degC` applies its additive shift when the unit stands alone, but in compound expressions it behaves like kelvin.

Ansys likewise distinguishes temperature from temperature difference in its unit systems, and thermal/radiation workflows may require an explicit temperature offset when Celsius or Fahrenheit scales are used.

Therefore these two expressions cannot be treated as identical merely because they share the same temperature base dimension:

```text
AbsoluteTemperature = 20 degC
TemperatureDifference = 20 degC-difference
```

The first maps to 293.15 K; the second maps to a difference of 20 K.

## 5. Main finding

`UnitReference + DimensionVector` is **necessary but not sufficient** for affine units such as Celsius and Fahrenheit.

The metrology resolution model must be able to distinguish at least:

```text
unit dimension
conversion scale
conversion offset
quantity/evaluation context
```

For compound expressions, offset units generally participate as differential units rather than as absolute affine values.

## 6. Candidate SOL rule

### T1 — Temperature Semantic Context

> A temperature-valued semantic concept SHALL distinguish absolute temperature from temperature difference when unit conversion semantics depend on that distinction.

This distinction SHOULD be expressed by the semantic quantity/context, not by introducing a duplicate physical dimension. Both remain temperature dimension, but unit conversion differs.

### T2 — Affine Unit Resolution

> A metrology adapter resolving an affine unit SHALL expose enough conversion semantics to distinguish scale-only conversion from scale-plus-offset conversion.

SOL Core need not own the conversion registry, but it must preserve the semantic context required to select the correct conversion.

## 7. Consequence for the candidate concrete representation

The existing candidate remains valid with one extension:

```text
SemanticQuantity
  ├── dimension: DimensionVector
  ├── optional semantic unit constraints/context
  └── value_definition

Value
  ├── data
  ├── shape
  └── unit: UnitReference

MetrologyAdapter.resolve(UnitReference, context)
  -> dimension
  -> scale
  -> offset semantics
```

No new `TemperatureDimension` subtype or `QuantityKind` layer is required. The semantic distinction belongs to the quantity/context and the metrology adapter contract.

## 8. Decision status

This document is research evidence. The stress test supports the concrete `DimensionVector` and `UnitReference` design, but adds a required affine-unit semantic-context constraint for the eventual ADR/specification update.
