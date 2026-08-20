# UnitReference and Metrology Adapter Contract v0.1

**Status:** Research/design candidate  
**Date:** 2026-08-20  
**Reference systems:** MOOSE, COMSOL, Ansys  
**Purpose:** Define the minimum SOL representation for units and the contract required of a metrology adapter.

## 1. Cross-backend observations

### MOOSE

`MooseUnits` parses unit expressions, normalizes them to seven SI base-unit exponents plus a multiplicative factor and additive shift, and supports dimensional-conformance checks and conversion. Celsius and Fahrenheit have additive shifts when used as stand-alone temperature units, while compound-unit operations zero the shift and treat them as temperature differences.

### COMSOL

COMSOL unit expressions carry scale and, for temperature units, offset behavior. The same token such as `degC` is interpreted as an absolute temperature when the expression has temperature dimension, but as a differential temperature when used inside a compound expression whose total dimension is not temperature.

### Ansys

Ansys associates units with quantity semantics and project/solver unit systems. It explicitly distinguishes Temperature from Temperature Difference in several APIs and unit systems. Numeric values without explicit units may inherit the active project unit for the relevant quantity.

## 2. Design consequence

The common contract cannot be represented by a unit symbol alone.

The following concerns are independent:

```text
unit identity
physical dimension
multiplicative scale
affine offset
quantity/evaluation context
unit-source resolution
```

`UnitReference` should identify a unit. Conversion semantics should be provided by a metrology adapter. Absolute-vs-difference semantics should be supplied by the semantic quantity/evaluation context, not encoded as an intrinsic property of every UnitReference.

## 3. UnitReference candidate

`UnitReference` is a typed reference, not a Core Entity.

Preferred canonical form:

```yaml
unit:
  namespace: qudt
  id: W-PER-M-K
```

URI form MAY also be supported:

```yaml
unit:
  uri: "http://qudt.org/vocab/unit/W-PER-M-K"
```

Backend-native unresolved units MAY be preserved temporarily during import:

```yaml
unit:
  namespace: comsol
  id: degC
```

but normalization SHOULD resolve them to a canonical metrology reference before Core validation when an adapter is available.

A display symbol MAY be retained as non-authoritative metadata:

```yaml
unit:
  namespace: qudt
  id: W-PER-M-K
  symbol: "W/(m·K)"
```

The symbol MUST NOT be used as the canonical identity.

## 4. Unit resolution context

Unit conversion may depend on semantic context. The minimum candidate context is:

```yaml
unit_context:
  role: absolute | difference | ordinary
```

Interpretation:

- `ordinary`: ordinary multiplicative unit semantics;
- `absolute`: affine/point semantics where an offset may apply;
- `difference`: interval semantics where additive offsets do not apply.

Examples:

```text
20 degC, role=absolute
→ 293.15 K

20 degC, role=difference
→ 20 K difference
```

This context is associated with the semantic quantity/value-definition use site, not with the unit identifier itself.

## 5. MetrologyAdapter contract

A conforming metrology adapter SHOULD provide at least the following operations.

### resolve

```text
resolve(UnitReference) -> UnitDescriptor
```

`UnitDescriptor` minimally contains:

```text
canonical_id
DimensionVector
scale_to_canonical
optional_offset_to_canonical
supported_context_roles
```

The exact canonical base unit system is adapter-defined but MUST be internally consistent.

### compatible

```text
compatible(UnitReference, DimensionVector, UnitContext) -> boolean
```

Checks dimensional compatibility and any additional quantity-specific constraints supplied by SOL.

### convert

```text
convert(value, from_unit, to_unit, UnitContext) -> converted_value
```

For multiplicative units:

```text
canonical = value * scale
```

For affine units in `absolute` context:

```text
canonical = (value + offset) * scale
```

For affine units in `difference` context, the offset is ignored.

### canonicalize

```text
canonicalize(UnitReference) -> UnitReference
```

Normalizes aliases/backend-native tokens to the adapter's canonical reference where possible.

## 6. Explicit versus contextual units

SOL values MAY omit an explicit UnitReference only when the owning semantic context provides a resolvable unit policy.

This is required because MOOSE and Ansys both allow numeric values whose unit semantics can be supplied by application/project conventions.

Conceptually:

```yaml
value:
  data: 400
  unit: null
```

is valid only if the enclosing Profile/backend context supplies an unambiguous unit contract.

Missing unit information MUST NOT be interpreted as dimensionless.

## 7. Interaction with DimensionVector

Validation follows:

```text
SemanticQuantity.dimension
           │
           ▼
    DimensionVector
           ▲
           │ resolve(unit)
Value.unit ─┘
```

A metrology adapter supplies the UnitReference's DimensionVector. SOL compares it against the semantic quantity's required dimension.

Dimension equality is necessary but may not be sufficient; semantic quantities may impose additional unit constraints.

## 8. Backend mapping examples

### MOOSE

```text
native: 600 degC
adapter descriptor:
  dimension = Temperature
  scale = 1
  offset = 273.15
context = absolute
```

Compound use such as `W/(m*degC)` resolves with `difference` semantics for the temperature factor.

### COMSOL

`100[degC]` in a temperature-valued expression uses absolute semantics; `degC` in a non-temperature compound expression uses differential semantics. The SOL importer must therefore recover context from the semantic quantity/expression rather than from the token alone.

### Ansys

Explicit `Quantity` strings preserve unit information. Numeric-only assignments may inherit the current project unit system. Ansys also exposes distinct Temperature and Temperature Difference semantics; therefore adapter resolution may depend on the quantity context supplied by the backend/profile.

## 9. Candidate normative rules

### UR1 — UnitReference identity

A UnitReference SHALL identify an external or adapter-resolvable unit definition. Display symbols are non-authoritative.

### UR2 — Adapter-owned metrology semantics

Dimension, scale, offset, aliases, and conversion behavior SHALL be resolved by a metrology adapter rather than duplicated in SOL Core.

### UR3 — Contextual affine conversion

For affine units, conversion semantics SHALL be resolved using semantic context that distinguishes absolute/point values from differences/intervals.

### UR4 — Contextual unit inheritance

An explicit unit MAY be omitted only when the surrounding model/profile/backend context supplies an unambiguous unit policy. Omission MUST NOT imply dimensionless.

### UR5 — Core independence

SOL Core SHALL remain valid without a particular metrology registry. QUDT MAY be the reference adapter but SHALL NOT be a hard dependency.

## 10. Open questions

1. Whether `ordinary | absolute | difference` should be generalized to a formal affine-space model.
2. Whether the unit-resolution context belongs on `SemanticQuantity`, `ValueDefinition`, `Value`, or a binding/use-site structure.
3. Whether backend-native unresolved UnitReferences are permitted in a fully validated Core graph or only in an import/intermediate representation.
4. Whether quantity-specific unit constraints are represented directly as Constraint objects or delegated to the metrology adapter.
5. How logarithmic units and other nonlinear conversion systems are represented.
