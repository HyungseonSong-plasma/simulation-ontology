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
semantic quantity identity
use-site quantity expectation
unit-source resolution
```

`UnitReference` identifies a unit. Conversion semantics are provided by a metrology adapter. Absolute-vs-difference semantics are derived from the semantic quantity required at the use site rather than stored as an independent `unit_context` field on the Value or UnitReference.

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

A display symbol MAY be retained as non-authoritative metadata. The symbol MUST NOT be used as canonical identity.

## 4. Semantic quantity and binding ownership

SOL does not introduce an independent `unit_context: absolute | difference | ordinary` field in Core.

The primary semantic source is `SemanticQuantity`. Distinct quantities may share the same physical dimension while imposing different conversion semantics.

```text
Temperature
  dimension = Θ
  affine role = absolute/point semantics

TemperatureDifference
  dimension = Θ
  affine role = difference/interval semantics
```

The exact serialization of quantity-level affine semantics remains an implementation detail; it may be intrinsic to a standard SemanticQuantity definition or expressed through a quantity-specific constraint.

A Binding/use-site does not duplicate the affine role. Instead, it declares which SemanticQuantity is expected at that site.

```text
InitialTemperature binding
    expects → Temperature

TemperatureRise binding
    expects → TemperatureDifference
```

Thus the same syntactic unit token can be interpreted correctly through the expected semantic quantity:

```text
20 degC bound as Temperature
→ 293.15 K

20 degC bound as TemperatureDifference
→ 20 K difference
```

This preserves the separation between semantic identity and contextual use while avoiding duplicated unit-context metadata.

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
conversion capabilities
```

The exact canonical base unit system is adapter-defined but MUST be internally consistent.

### compatible

```text
compatible(UnitReference, SemanticQuantity) -> boolean
```

The adapter resolves the unit dimension and conversion capabilities. SOL supplies the SemanticQuantity contract, including required DimensionVector and any additional quantity-specific unit constraints.

### convert

```text
convert(value, from_unit, to_unit, SemanticQuantity) -> converted_value
```

For ordinary multiplicative quantities, conversion uses scale. For affine/point quantities such as absolute temperature, the relevant offset is applied. For difference/interval quantities, additive offsets are not applied.

The adapter MUST NOT infer absolute-vs-difference semantics solely from a unit symbol when the SemanticQuantity contract is available.

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
Binding.expected_quantity
          │
          ▼
   SemanticQuantity
          │
          ├── dimension ────────> DimensionVector
          │
          └── quantity-specific conversion/unit semantics

Value.unit ──resolve────────────> UnitDescriptor
                                  └── DimensionVector
```

SOL first validates dimensional compatibility. It then applies quantity-specific unit/conversion constraints where dimension equality alone is insufficient.

## 8. Backend mapping examples

### MOOSE

A MOOSE input may provide a value/unit token while `MooseUnits` applies affine shifts for stand-alone Celsius/Fahrenheit and suppresses shifts in compound-unit operations. A SOL adapter maps the use site to the expected SemanticQuantity and preserves that meaning independently of the native token behavior.

### COMSOL

`degC` may represent an absolute temperature in a temperature-valued expression and differential temperature in compound expressions. SOL represents this distinction through the expected SemanticQuantity rather than by assigning two intrinsic meanings to the `degC` UnitReference.

### Ansys

Ansys exposes distinct Temperature and Temperature Difference semantics and may inherit project units for numeric-only assignments. The SOL backend/profile mapping therefore supplies the expected SemanticQuantity and any inherited unit policy at the binding/use site.

## 9. Candidate normative rules

### UR1 — UnitReference identity

A UnitReference SHALL identify an external or adapter-resolvable unit definition. Display symbols are non-authoritative.

### UR2 — Adapter-owned metrology semantics

Dimension, scale, offset, aliases, and conversion behavior SHALL be resolved by a metrology adapter rather than duplicated in SOL Core.

### UR3 — SemanticQuantity-owned conversion semantics

Where unit conversion depends on point/interval or equivalent semantic distinctions, the primary semantic contract SHALL be provided by the SemanticQuantity rather than by the UnitReference or Value.

### UR4 — Binding quantity expectation

A Binding/use-site SHALL express the SemanticQuantity expected at that site when such information is required for validation or conversion. The Binding SHOULD NOT duplicate quantity-level conversion semantics.

### UR5 — Contextual unit inheritance

An explicit unit MAY be omitted only when the surrounding model/profile/backend context supplies an unambiguous unit policy. Omission MUST NOT imply dimensionless.

### UR6 — Core independence

SOL Core SHALL remain valid without a particular metrology registry. QUDT MAY be the reference adapter but SHALL NOT be a hard dependency.

## 10. Resolved design question

The earlier candidate `unit_context: ordinary | absolute | difference` is removed from the Core model.

Resolution:

```text
SemanticQuantity = primary semantic source
Binding/use-site = declares expected SemanticQuantity
Value = typed datum
UnitReference = unit identity
MetrologyAdapter = dimension/scale/offset/conversion resolution
```

This avoids encoding the same semantics independently on Value, UnitReference, ValueDefinition, and Binding.

## 11. Remaining open questions

1. Whether quantity-level affine semantics require an explicit generic constraint vocabulary or can initially be defined by standard SemanticQuantity definitions.
2. Whether backend-native unresolved UnitReferences are permitted in a fully validated Core graph or only in an import/intermediate representation.
3. Whether quantity-specific unit constraints are represented directly as Constraint constructs or delegated partly to the metrology adapter.
4. How logarithmic units and other nonlinear conversion systems are represented.
