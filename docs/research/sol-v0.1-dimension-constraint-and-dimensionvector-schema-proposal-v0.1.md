# SOL v0.1 Dimension Constraint and DimensionVector Schema Proposal v0.1

**Role:** Research  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`

## 1. Objective

Define the minimum machine-readable representation needed for the accepted SOL `Dimension` Constraint family without taking ownership of a complete unit/metrology system or prematurely deciding whether graph-level `PhysicalDimension` is ultimately an Entity or typed structure.

This proposal follows ADR-0004, ADR-0005, ADR-0007, and ADR-0018.

## 2. Evidence

### Accepted SOL contract

ADR-0004 requires:

- semantic quantity identity, PhysicalDimension, Unit, and Value to remain distinct;
- dimension-one to be explicit rather than inferred from missing Unit metadata;
- Unit compatibility to respect the semantic quantity's PhysicalDimension.

ADR-0005 requires:

- dimensional compatibility to remain a Core semantic contract;
- a canonical machine-comparable representation based on base-dimension exponents;
- no mandatory complete Unit registry or metrology dependency in Core.

ADR-0007 defines `Dimension` as one of the six Constraint primitive families and composes Dimension constraints by canonical vector equality.

ADR-0018 separates authoring representation from normalized semantic payload.

### External reference evidence

The current BIPM SI Brochure defines seven SI base dimensions and expresses derived dimensions as products of powers of those base dimensions. Dimension-one quantities have all dimensional exponents equal to zero.

Reference:
- BIPM, *The International System of Units (SI), 9th edition*, section 2.3.3: https://www.bipm.org/documents/20126/41483022/SI-Brochure-9.pdf

QUDT Dimension Vectors represent dimensional analysis using explicit exponents for base dimensions, supporting the same vector-normalization pattern without requiring QUDT as a Core dependency.

Reference:
- QUDT Dimension Vectors: https://qudt.org/doc/2026/01/DOC_VOCAB-DIMENSION-VECTORS.html

Backend evidence remains consistent with the separation:

- MOOSE exposes documented units and unit-conversion syntax while requiring a consistent physical unit system.
- COMSOL performs unit/dimension consistency checks while supporting multiple unit systems.
- Ansys similarly supports multiple consistent systems of units.

The backend unit syntax therefore should not become the Core Dimension representation.

## 3. Dependency decision

Dimension consolidation SHALL precede general Value Constraint consolidation.

Reason:

```text
numeric/value comparison
  may require unit normalization
      -> requires dimensional compatibility boundary
          -> requires canonical DimensionVector equality first
```

This does not mean every Value Constraint carries a Unit or Dimension. It means the Core dimensional contract must be machine-comparable before value/unit normalization can safely build on it.

## 4. Canonical DimensionVector axis set

SOL v0.1 uses the seven SI base dimensions as the canonical comparison basis:

| Canonical serialization key | BIPM dimension symbol |
|---|---|
| `time` | T |
| `length` | L |
| `mass` | M |
| `electric_current` | I |
| `thermodynamic_temperature` | Theta |
| `amount_of_substance` | N |
| `luminous_intensity` | J |

The long ASCII keys are serialization identifiers; they do not redefine BIPM terminology.

JSON object member order is non-semantic.

## 5. Exponent domain

For SOL v0.1, each base-dimension exponent is a signed integer.

This covers the accepted Core/reference cases and aligns with the BIPM/QUDT engineering dimension-vector representations used as evidence.

Fractional/rational dimensional exponents are **deferred**, not declared invalid for all future SOL versions. Adding exact rational exponents later would require an explicit normalization contract rather than floating-point comparison.

## 6. Authoring DimensionVector

Human authoring MAY omit zero axes:

```yaml
vector:
  mass: 1
  length: 1
  time: -3
  thermodynamic_temperature: -1
```

Omitted accepted axes normalize to exponent `0`.

Unknown axis keys are structurally invalid.

### Dimension one

Dimension one is explicitly authored by the presence of a Dimension contract whose vector may be empty:

```yaml
vector: {}
```

This normalizes to all seven exponents equal to zero.

Therefore:

```text
vector omitted entirely != DimensionOne
vector present as {}     == DimensionOne
```

The first is missing Dimension constraint data; the second is an explicit dimension-one contract.

## 7. Normalized DimensionVector

Canonical normalized representation contains all seven axes explicitly:

```yaml
vector:
  time: 0
  length: 1
  mass: 1
  electric_current: 0
  thermodynamic_temperature: -1
  amount_of_substance: 0
  luminous_intensity: 0
```

Thermal conductivity is represented as:

```text
M^1 L^1 T^-3 Theta^-1
```

Dimension one is:

```yaml
vector:
  time: 0
  length: 0
  mass: 0
  electric_current: 0
  thermodynamic_temperature: 0
  amount_of_substance: 0
  luminous_intensity: 0
```

A canonical serializer MAY emit keys in the table order for readability, but equality is by normalized axis/exponent mapping, not textual key order.

## 8. Dimension Constraint payload

### Authoring

```yaml
type: dimension
vector:
  mass: 1
  length: 1
  time: -3
  thermodynamic_temperature: -1
```

### Normalized semantic payload

```yaml
type: dimension
vector:
  time: 0
  length: 1
  mass: 1
  electric_current: 0
  thermodynamic_temperature: -1
  amount_of_substance: 0
  luminous_intensity: 0
```

The containing Entity/Property/Interface/local declaration plus ADR-0018 evidence identifies the semantic subject/use-site. This proposal does not introduce a universal graph path.

## 9. PhysicalDimension graph identity remains orthogonal

This DimensionVector is the canonical **machine-comparable semantic payload** for dimensional validation.

It does not decide whether a graph-level `PhysicalDimension` is serialized as:

- a reusable Entity carrying/referring to this vector;
- an inline typed structure;
- another canonical representation compiled to this vector.

If a graph-level `requires_dimension` relation or PhysicalDimension identifier is used, the compiler/normalizer must resolve it to exactly one canonical DimensionVector before Dimension Constraint composition.

A graph identifier and the normalized vector are not competing semantic authorities.

## 10. Dimension intersection

Active semantic Dimension constraints on the same semantic subject/use-site compose by normalized equality:

```text
D1 == D2 -> D1
D1 != D2 -> empty Dimension intersection
```

Conflict classification follows ADR-0007:

- intrinsically incompatible schema declarations -> Schema Conflict;
- incompatible constraints activated only for a configuration -> Configuration Conflict.

Declaration order does not participate.

## 11. Semantic versus backend representability boundary

A backend/Profile inability to express Units or dimensional metadata SHALL NOT weaken an upstream Dimension constraint.

```text
SOL semantic Dimension validation
        !=
backend unit/metadata representability
```

Backend/Profile limitations are handled on the mapping/representability axis under ADR-0006/0011.

No MOOSE, COMSOL, Ansys, QUDT, or other external runtime/registry is required to compare normalized DimensionVectors.

## 12. Unit boundary

The Dimension Constraint contains no Unit identifier, conversion factor, symbol, offset, or metrology-registry metadata.

Unit compatibility may consult the required DimensionVector, but Unit conversion/canonicalization belongs to later Unit/metrology representation work.

This preserves:

```text
SemanticQuantity != PhysicalDimension != Unit != Value
```

## 13. Proposed schemas

After contract acceptance:

### `constraint-dimension-authoring-v0.1.schema.json`

- requires `type: dimension` and `vector`;
- `vector` may contain any subset of the seven canonical axes;
- exponents are integers;
- unknown keys are rejected;
- empty `{}` is valid and explicitly means DimensionOne.

### `constraint-dimension-normalized-v0.1.schema.json`

- requires `type: dimension` and `vector`;
- requires all seven canonical axes;
- every exponent is an integer;
- unknown keys are rejected.

JSON Schema validates structure only. Context resolution, graph PhysicalDimension resolution, semantic conflict classification, and backend representability remain semantic/compiler responsibilities.

## 14. Counterexamples

### DC-01 — unit absence interpreted as dimension one

No Dimension contract exists and Unit is absent.

Expected: dimension is unresolved/missing; SHALL NOT infer DimensionOne.

### DC-02 — explicit DimensionOne

```yaml
type: dimension
vector: {}
```

Expected authoring: PASS; normalized vector = seven zeros.

### DC-03 — thermal conductivity

```yaml
vector: {mass: 1, length: 1, time: -3, thermodynamic_temperature: -1}
```

Expected normalized vector: complete seven-axis vector with omitted exponents zero.

### DC-04 — unknown axis

```yaml
vector: {currency: 1}
```

Expected: structural FAIL.

### DC-05 — non-integer exponent in v0.1

```yaml
vector: {time: -0.5}
```

Expected: structural FAIL for v0.1. Rational/fractional exponent support is deferred to an explicit future contract.

### DC-06 — unequal dimensions

Pressure-like vector intersected with Temperature vector.

Expected: empty Dimension intersection; no last-write-wins.

### DC-07 — backend omits unit metadata

SOL semantic quantity has a valid required DimensionVector; selected backend does not serialize units.

Expected: semantic Dimension contract remains unchanged. Backend representability/mapping handles the limitation separately.

## 15. Research verdict

**Ready for independent Validation.**

The proposal defines only the minimum machine-comparable Dimension Constraint representation required by already accepted ADRs and leaves Unit registry ownership, graph-level PhysicalDimension representation, fractional exponents, and backend runtime behavior outside this focused slice.
