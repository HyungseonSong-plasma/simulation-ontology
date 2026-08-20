# ADR-0026 — Value, Unit, Dimension, and ValueDefinition Transcription Boundary

**Status:** Accepted  
**Date:** 2026-08-20  
**Scope:** SOL v0.1 language/schema consolidation

## Context

ADR-0002/0003/0004/0005 established the semantic separation between semantic concepts, evaluated Value, ValueDefinition, PhysicalDimension, Unit, and semantic dependencies. ADR-0020 fixed canonical DimensionVector semantics and ADR-0021 fixed exact-decimal scalar Value semantics. Earlier architecture diagrams intentionally left final graph-versus-typed-data transcription open.

Independent Validation of the focused transcription proposal accepted the final boundary after resolving metrology-state, dependency-resolution, and reified-identity gaps.

## Decision

### 1. PhysicalDimension

`PhysicalDimension` is not a mandatory Core Entity Type in v0.1. Its canonical machine-comparable representation is the ADR-0020 seven-axis `DimensionVector`.

DimensionOne is the all-zero vector. Dimension equality is vector equality.

`requires_dimension` and `has_dimension` are not introduced as duplicate Core Entity-to-Entity Relations. Required dimension is expressed through the accepted Dimension Constraint; Unit dimension is obtained through metrology resolution.

### 2. Unit

SOL Core does not own a unit catalogue. A normalized Value MAY carry:

```yaml
unit:
  namespace: <nonempty external/metrology namespace>
  id: <nonempty unit id>
```

The semantic UnitReference key is `(namespace,id)`. Display symbols and backend-local tokens are non-authoritative.

Missing `unit` means only that the Value payload carries no explicit UnitReference. It does not imply DimensionOne, unit one, SI, backend defaults, or semantic validity.

### 3. Semantic metrology validation

When unit/dimension evidence is required:

- resolved compatible evidence -> `PASS`;
- definite dimensional or quantity-specific incompatibility -> `FAIL`;
- required but unresolved metrology/context evidence -> `INDETERMINATE`.

The common aggregation order is ADR-0024:

```text
FAIL > INDETERMINATE > PASS
```

Backend runtime, installation, release, and license state do not participate in this semantic verdict.

### 4. Value

`Value` is typed evaluated data by default, not a Core Entity.

The focused v0.1 shape axis is:

```text
shape = scalar | vector | tensor
scalar_kind = number | string | boolean
```

Numeric components use ADR-0021 exact-decimal representation. Vector and tensor values are homogeneous. Tensor storage is flattened row-major with explicit positive `tensor_shape`; component count equals the product of tensor dimensions. UnitReference is permitted only for numeric Values in the focused v0.1 slice.

Value shape remains independent of ValueDefinition evaluation mechanism.

### 5. Inline ValueDefinition

The focused mechanism vocabulary is:

```text
literal | expression | function | tabular
```

A local definition MAY remain an inline typed structure only when it has no independent graph identity requirement and no compiler-resolved canonical SOL semantic dependencies.

Semantic validators SHALL NOT infer graph dependencies by scanning raw expression/function/tabular strings. Dependency recognition occurs in the authoring/compiler/name-resolution phase.

A normalized inline ValueDefinition has no graph identity and SHALL NOT be a Relation endpoint.

### 6. Reified ValueDefinition

`ValueDefinition` is a Core Entity Type for identified/relation-participating definition instances.

Reification is required when independent identity is needed for one or more of:

- canonical semantic dependency Relations;
- reuse by multiple owners;
- independent provenance or lifecycle;
- independent Constraints;
- independent backend mapping;
- explicit identified declaration.

Mechanism does not determine reification.

Each reified ValueDefinition is one resolved model-instance Entity. All owner and dependency graph statements referring to it SHALL use the same resolved model-instance identity. Equal payloads under distinct model-instance identities remain distinct instances.

### 7. Relations

The graph-level reified form uses:

```text
owner --has_value_definition--> ValueDefinition
ValueDefinition --depends_on--> semantic Entity
```

`has_value_definition` is only for reified definitions. Inline definitions use a local typed field instead.

`ReferenceDefinition` is not introduced. Pure aliases remain value-dependency Relations under ADR-0003.

`evaluates_to -> Value` and `expressed_in -> Unit` are not stored Core Entity Relations because Value and UnitReference are typed payloads rather than Core Entity instances.

### 8. Validation order

A normalized validator performs, as applicable:

1. canonical identity/name resolution;
2. inline/reified ValueDefinition consistency;
3. ValueDefinition graph endpoint consistency;
4. Value structural validation;
5. Dimension Constraint resolution;
6. UnitReference/contextual-unit semantic resolution;
7. dimension comparison;
8. quantity-specific Compatibility validation;
9. ADR-0024 aggregation.

## Machine-readable transcription

The focused accepted slice SHALL add:

- `schema/dimension-vector-v0.1.schema.json`
- `schema/unit-reference-v0.1.schema.json`
- `schema/value-v0.1.schema.json`
- `schema/value-definition-inline-v0.1.schema.json`

The existing Dimension Constraint schema SHOULD reuse the shared DimensionVector schema without semantic change.

The Core registry SHALL add `ValueDefinition` as an Entity Type and focused declarations for `has_value_definition` and `depends_on`. Final package placement and broader domain/range refinements remain canonical package integration work.

## Deferred

- universal SemanticQuantity superclass;
- QuantityKind layer;
- hard QUDT dependency;
- backend-native unit vocabulary in Core;
- heterogeneous-unit tuple Values;
- distributions/random fields/complex scalar kinds;
- interpolation versus tabular split;
- ExternalData versus provenance boundary;
- generic reification syntax for all typed constructs;
- full PropertyDefinition serialization;
- package placement of value-bearing fields and Constraint applications.

## Consequences

SOL now has one deterministic boundary between graph identity and value/metrology typed data while preserving dimensional semantics independently of backend implementations and unit registries.
