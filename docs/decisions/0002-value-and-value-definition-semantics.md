# ADR 0002 — Value and ValueDefinition Semantics

**Status:** Accepted  
**Date:** 2026-08-20  
**Scope:** Simulation Ontology Language v0.1

## Context

Cross-backend analysis of MOOSE, COMSOL, and Ansys Mechanical showed that a semantic simulation quantity and the mechanism used to provide or evaluate its value are distinct concepts.

The same semantic slot can be represented by a literal constant, expression, function, table/interpolation, tensor-valued definition, or backend evaluator. Conversely, scalar/vector/tensor shape is independent of whether the value is literal, functional, or tabulated.

The primary evidence is recorded in:

- `docs/research/cross-backend-semantic-mapping-matrix-v0.1.md`

## Decision

The following principles are accepted for SOL v0.1.

### V1 — Value semantics

> **Value is an evaluated typed datum, not the semantic quantity itself.**

A physical or simulation concept such as `ThermalConductivity`, `Temperature`, or `ElectricField` is not identical to its concrete evaluated value.

### V2 — ValueDefinition semantics

> **ValueDefinition describes how a Value is obtained.**

A semantic concept may obtain its value from a literal, expression, function, table/interpolation, external source, or another evaluation mechanism. The precise taxonomy of `ValueDefinition` subtypes remains subject to further validation.

### V3 — Orthogonality of shape and evaluation mechanism

> **Value shape and value-evaluation mechanism are orthogonal concepts.**

Scalar, vector, and tensor describe the shape of an evaluated value. Literal, expression, function, tabular, interpolation, and similar categories describe how that value is defined or evaluated. These dimensions MUST NOT be collapsed into one flat taxonomy.

Conceptually:

```text
Semantic Concept
      │
      │ has_value_definition
      ▼
ValueDefinition
      │
      │ evaluates_to
      ▼
Value
      │
      ├── Scalar
      ├── Vector
      └── Tensor
```

A tensor-valued property, for example, may have a literal definition, functional definition, or tabular definition without changing its tensor value shape.

## Consequences

1. SOL MUST distinguish semantic quantities/properties from evaluated values.
2. SOL MUST provide a semantic representation for value definitions, although the final language-level form is not yet fixed.
3. `ScalarValue`, `VectorValue`, and `TensorValue` MUST NOT be placed in the same classification axis as `ExpressionDefinition`, `FunctionDefinition`, or `TabularDefinition`.
4. Backend-native constant/function/table/expression metadata must be normalized according to these two independent dimensions.
5. Functional or context-dependent values do not automatically require `ParameterBinding`; their dependence may be represented within `ValueDefinition` when the dependence is intrinsic to value evaluation.

## Deferred decisions

This ADR intentionally does not decide:

- whether `Value` is a first-class SOL language construct, typed data object, or graph Entity;
- the normative subtype taxonomy of `ValueDefinition`;
- whether `Reference` belongs under `ValueDefinition` or should remain a Relation;
- exact unit and physical-dimension ownership;
- provenance and data-source representation;
- uncertainty representation;
- runtime evaluation semantics.

In particular, the previously proposed V4 rule concerning physical dimensions and units remains provisional pending a dedicated unit/dimension study.

## Rationale

The decision preserves semantic identity independently of backend representation and avoids conflating two different questions:

1. **What is the shape/type of the evaluated datum?**
2. **How is that datum obtained?**

This distinction is required to represent equivalent simulation concepts consistently across MOOSE, COMSOL, Ansys, and future backends.
