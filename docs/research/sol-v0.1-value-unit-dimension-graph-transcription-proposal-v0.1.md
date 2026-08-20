# SOL v0.1 Value / Unit / PhysicalDimension Graph Transcription Proposal v0.1

**Status:** Research proposal  
**Date:** 2026-08-20  
**Scope:** Final language-level transcription of accepted Value, Unit, PhysicalDimension, and ValueDefinition boundaries  
**Depends on:** ADR-0002, ADR-0003, ADR-0004, ADR-0005, ADR-0007, ADR-0009, ADR-0020, ADR-0021

## 1. Problem

Earlier SOL documents used a conceptual picture such as:

```text
SemanticQuantity -> requires_dimension -> PhysicalDimension
SemanticQuantity -> has_value_definition -> ValueDefinition -> evaluates_to -> Value
Value -> expressed_in -> Unit
Unit -> has_dimension -> PhysicalDimension
```

The picture intentionally preceded the final Entity/typed-data boundary. After ADR-0020/0021 and the Entity-boundary studies, serializing every node and arrow above as a Core Entity/Relation would now contradict accepted SOL semantics.

This proposal decides which concepts are:

- model graph Entities;
- typed data/definition structures;
- external typed references;
- Constraint applications;
- semantic-service validation dependencies.

## 2. Evidence summary

### Accepted SOL evidence

ADR-0002 separates semantic concept, `ValueDefinition`, and evaluated `Value`, and makes value shape orthogonal to evaluation mechanism.

ADR-0003 requires independently meaningful object references and computational dependencies to remain semantic Relations rather than a generic `ReferenceDefinition`.

ADR-0004 requires a physical semantic quantity to declare or imply a PhysicalDimension, allows explicit Unit on a Value, and states that missing Unit SHALL NOT imply dimensionless.

ADR-0005 requires machine-comparable base-dimension semantics while excluding a mandatory Core unit registry.

ADR-0020 fixes the canonical seven-axis `DimensionVector` representation.

ADR-0021 fixes exact-decimal scalar normalization and comparison-space gating.

### Cross-backend evidence

MOOSE, COMSOL, and Ansys all distinguish semantic quantities/properties from concrete value representations. COMSOL and Ansys explicitly distinguish absolute temperature from temperature difference despite the same base physical dimension. COMSOL uses affine offsets for absolute Celsius/Fahrenheit contexts; Ansys exposes distinct Temperature and Temperature Difference quantity semantics. Ansys Workbench also permits numeric values with omitted units whose units are inherited from the project quantity context.

Therefore:

```text
physical dimension
    != semantic quantity/context
    != concrete unit identity
    != evaluated value datum
```

A unit symbol or missing unit field cannot carry all four meanings.

## 3. PhysicalDimension becomes a typed canonical structure

SOL v0.1 SHALL NOT add `PhysicalDimension` as a mandatory Core Entity Type.

Its canonical semantic representation is the ADR-0020 `DimensionVector`:

```yaml
time: 0
length: 1
mass: 0
electric_current: 0
thermodynamic_temperature: 0
amount_of_substance: 0
luminous_intensity: 0
```

All seven axes are present after normalization and exponents have ADR-0020 integer-valued semantics.

DimensionOne is the all-zero vector.

Semantic dimension equality is vector equality, not object identity, display label, unit symbol, or external vocabulary identity.

Named external dimension resources MAY be mapped to/from this vector, but their identity is non-authoritative to Core dimension equality.

### Consequence for conceptual `requires_dimension`

`requires_dimension` SHALL NOT become an Entity-to-Entity Core Relation.

A value-bearing semantic contract declares/implies its required dimension through an accepted **Dimension Constraint** whose semantic payload contains the canonical `DimensionVector`.

The exact package location of that Constraint application is part of canonical package integration, but the semantic authority is the Dimension Constraint rather than a duplicate relation or `PhysicalDimension` Entity.

## 4. Unit becomes UnitReference, not a Core Entity

SOL v0.1 SHALL NOT own a Core `Unit` Entity catalogue.

A Value MAY carry an external/metrology-resolvable typed reference:

```yaml
unit:
  namespace: qudt
  id: K
```

The normalized identity key is:

```text
UnitReferenceKey = (namespace, id)
```

`namespace` and `id` are nonempty opaque strings interpreted by a configured metrology provider/registry contract. QUDT is a valid reference implementation but not a hard Core dependency.

Authoring adapters MAY accept URI or backend-native unit syntax, but canonical Core normalization SHALL produce the two-field `UnitReference`. Backend-local unit tokens are not authoritative normalized unit identity.

A display symbol MAY be retained outside the semantic payload as presentation metadata and SHALL NOT define Unit identity.

### Consequence for conceptual `has_dimension`

`Unit -> has_dimension -> PhysicalDimension` SHALL NOT become a Core Relation.

Metrology resolution provides a descriptor conceptually containing:

```text
resolved UnitReference
  -> DimensionVector
  -> scale/conversion semantics
  -> optional affine offset semantics
```

Core compares the returned DimensionVector with the active Dimension Constraint. Scale/offset/catalogue semantics remain metrology-provider responsibilities.

## 5. Missing Unit has one narrow payload meaning

Absence of `Value.unit` means exactly:

> this Value payload carries no explicit UnitReference.

It SHALL NOT mean:

- DimensionOne;
- unit one;
- SI units;
- backend default units;
- unknown dimension;
- semantically valid omission.

Whether omission is valid is decided by the owning semantic/use-site contract. Omission is valid only when that contract provides an unambiguous unit policy or explicitly permits no explicit unit representation.

If unit semantics are required but neither an explicit UnitReference nor a resolvable contextual unit policy is available, semantic validation is incomplete; the validator SHALL NOT guess a unit or dimension.

A normalized Core Value may therefore omit `unit`, but acceptance of that omission requires separate validation evidence from its use-site context.

This preserves Ansys-style contextual unit inheritance without copying project unit systems into the Value payload.

## 6. Affine-unit semantic context remains outside UnitReference

`UnitReference(namespace,id)` identifies a unit definition only. It SHALL NOT carry an independent Core field such as:

```text
unit_context = absolute | difference
```

when that semantic distinction belongs to the expected semantic quantity/use site.

Absolute Temperature and Temperature Difference may share the same temperature DimensionVector while requiring different affine conversion semantics.

The owning semantic contract SHALL preserve the quantity/use-site distinction needed for metrology conversion. When dimension equality alone is insufficient, an accepted Compatibility criterion MAY express quantity-specific unit admissibility after Dimension validation.

Thus:

```text
Dimension equality
    is necessary for ordinary physical compatibility
    but does not erase quantity-specific affine semantics.
```

## 7. Value is typed evaluated data, not a Core Entity by default

A normalized `Value` is a typed datum:

```text
Value
  shape
  scalar_kind
  data
  unit? : UnitReference
```

`Value` SHALL NOT be added to the Core Entity taxonomy merely because it is semantically typed or unit-bearing.

### v0.1 shape contract

```text
shape = scalar | vector | tensor
scalar_kind = number | string | boolean
```

Numeric components use the ADR-0021 exact-decimal scalar representation.

For the focused normalized representation:

```yaml
# scalar
shape: scalar
scalar_kind: number
data:
  coefficient: "300"
  exponent10: 0
unit:
  namespace: qudt
  id: K
```

```yaml
# vector
shape: vector
scalar_kind: number
data:
  - {coefficient: "1", exponent10: 0}
  - {coefficient: "2", exponent10: 0}
unit:
  namespace: qudt
  id: M-PER-SEC
```

```yaml
# tensor, flattened row-major storage with explicit dimensions
shape: tensor
scalar_kind: number
tensor_shape: [3, 3]
data:
  - <9 exact-decimal components>
unit:
  namespace: qudt
  id: W-PER-M-K
```

Rules:

1. `vector` data is a nonempty homogeneous scalar array.
2. `tensor_shape` is present only for tensor, has at least two positive integer dimensions, and tensor data length SHALL equal the product of `tensor_shape`.
3. All components share one `scalar_kind`.
4. A UnitReference is permitted only for numeric Values in v0.1.
5. A single Value payload carries at most one UnitReference applying to all numeric components; heterogeneous-unit tuples are not a v0.1 Value shape.
6. Value shape is independent of ValueDefinition evaluation mechanism.

The tensor component-count rule is semantic/helper validation when JSON Schema cannot express the product directly.

## 8. ValueDefinition remains mechanism, not value shape

The focused v0.1 mechanism vocabulary is intentionally small:

```text
literal
expression
function
tabular
```

`ReferenceDefinition` is excluded by ADR-0003.

`Interpolation` and `ExternalData` remain deferred until their semantic boundary from tabular/function/provenance is independently justified.

A ValueDefinition mechanism SHALL NOT determine whether the definition is inline or reified.

## 9. InlineValueDefinition

A dependency-free local evaluation definition MAY remain an inline typed structure when no independent semantic identity, relation participation, reuse, provenance, lifecycle, or independent mapping is required.

Focused normalized form:

```yaml
mechanism: literal
output:
  <Value>
```

or a dependency-free expression/function/tabular payload whose complete semantics are local to the owner.

### v0.1 conservative boundary

An inline ValueDefinition SHALL NOT carry semantic dependency references to independently identifiable SOL concepts.

If evaluation depends on another semantic Entity, ADR-0003 requires the dependency to be a semantic Relation. The definition must therefore use the reified form in Section 10.

This avoids hidden generated graph identities and avoids encoding semantic dependencies as opaque strings inside a typed payload.

## 10. Reified ValueDefinition Entity

SOL v0.1 SHALL add `ValueDefinition` as an Entity Type **only for identified/relation-participating definition instances**.

This does not make every ValueDefinition an Entity; inline definitions remain typed data under Section 9.

A reified ValueDefinition has:

```text
model-instance identity
mechanism payload
output Value contract/value as applicable
semantic Relations such as depends_on
optional provenance/lifecycle/mapping metadata
```

Reification is required when independent identity is needed for any of:

- semantic dependency Relations;
- reuse by multiple owning concepts;
- independent provenance;
- lifecycle/version state;
- independent Constraints;
- independent backend mapping.

The mechanism remains orthogonal: literal, expression, function, or tabular definitions may all be reified when semantic identity requires it.

### Ownership/reference connection

A value-bearing Entity may use:

- a local inline `value_definition` field; or
- an explicit `has_value_definition` Relation to a reified `ValueDefinition` Entity.

`has_value_definition` is needed only for the reified graph case. Because many different Entity Types may be value-bearing, its domain need not be over-constrained to a synthetic common superclass in v0.1; the use-site/schema capability determines where it is admissible.

ADR-0003 `depends_on` Relations originate from a reified `ValueDefinition` when computational dependencies must be preserved.

The exact broad range contract for `depends_on` and final package serialization of these relations remain canonical relation/package integration work; both endpoints remain semantic Entities.

## 11. `evaluates_to` is not a stored graph Relation by default

A runtime/evaluated `Value` is typed data rather than an Entity. Therefore:

```text
ValueDefinition -> evaluates_to -> Value
```

SHALL NOT be serialized as an Entity-to-Entity Relation by default.

Instead the evaluation result is a typed output/result field or runtime evaluation result associated with the ValueDefinition/use site.

If future provenance requires an independently identified measurement/result object, that is a separate Entity chosen by the semantic-identity rule rather than reifying every Value.

## 12. `expressed_in` is a Value field, not a graph Relation

Because UnitReference is typed external identity rather than Core Entity identity:

```text
Value -> expressed_in -> Unit
```

is normalized as:

```yaml
Value:
  unit: UnitReference
```

not as a Core relation edge.

## 13. Dimension and unit validation sequence

For a numeric physical Value in a resolved use site:

```text
1. obtain expected semantic/use-site contract
2. obtain active Dimension Constraint -> expected DimensionVector
3. inspect explicit UnitReference, if any
4. if explicit UnitReference exists:
     resolve through metrology provider
     compare resolved DimensionVector to expected DimensionVector
     apply any quantity-specific unit Compatibility criteria
5. if UnitReference is absent:
     validate that the use-site contract supplies an unambiguous contextual unit policy or permits omission
6. perform conversion only with the semantic quantity/use-site context required for affine semantics
```

Failure to resolve a required semantic metrology service is evaluation incompleteness, not proof that the Value is dimensionally incompatible and not a backend representability failure.

Backend installation, solver runtime, release, and license state are not metrology semantic evidence.

## 14. Transcription matrix

| Earlier conceptual item | v0.1 transcription |
|---|---|
| `PhysicalDimension` node | typed canonical `DimensionVector` |
| `requires_dimension` | Dimension Constraint in owning semantic contract |
| `Unit` node | typed external `UnitReference` |
| `has_dimension` | metrology resolution result, not Core Relation |
| `Value` node | typed evaluated `Value` payload |
| `expressed_in` | optional `Value.unit` field |
| local `ValueDefinition` | inline typed `InlineValueDefinition` |
| identified/dependent `ValueDefinition` | reified `ValueDefinition` Entity |
| owner -> reified definition | `has_value_definition` Relation |
| ValueDefinition dependency | ADR-0003 semantic `depends_on` Relation |
| `evaluates_to -> Value` | typed output/evaluation result, not graph Relation |
| semantic unit restriction beyond dimension | ordinary Compatibility Constraint/criterion where required |

## 15. Proposed machine schema slice after acceptance

Add:

- `schema/dimension-vector-v0.1.schema.json`
- `schema/unit-reference-v0.1.schema.json`
- `schema/value-v0.1.schema.json`
- `schema/value-definition-inline-v0.1.schema.json`

Refactor the existing Dimension Constraint schema to reuse `dimension-vector-v0.1.schema.json` without changing accepted ADR-0020 semantics.

Add minimal semantic helper/tests for:

- DimensionVector equality/DimensionOne;
- UnitReference structural identity;
- missing-unit non-inference;
- scalar/vector/tensor shape validation;
- exact-decimal vector/tensor components;
- tensor component-count validation;
- unit only on numeric Values;
- inline dependency rejection;
- reification boundary cases;
- metrology-resolution state separated from backend runtime state.

Reified ValueDefinition Entity transcription should add the Entity Type and focused relation declarations only after contract Validation confirms the graph-boundary decision.

## 16. Required boundary cases

1. all-zero DimensionVector -> DimensionOne, not missing dimension;
2. two dimension resources/labels with equal vectors -> semantically same dimension;
3. omitted Value unit on a temperature Value -> does not imply Kelvin or dimensionless;
4. explicit UnitReference `{namespace,id}` -> stable typed unit identity;
5. display symbol mismatch -> does not change UnitReference identity;
6. Celsius absolute vs Celsius difference -> same base DimensionVector may still require different conversion semantics;
7. numeric scalar with explicit unit -> structurally valid;
8. string/Boolean Value with unit -> invalid in v0.1;
9. vector components of mixed scalar kind -> invalid;
10. tensor data length != product(tensor_shape) -> invalid;
11. inline literal with no dependencies -> valid typed definition;
12. inline expression with semantic dependency -> rejected from inline form/requires reification;
13. reified literal used by two owners -> allowed despite literal mechanism;
14. reified function with `depends_on` -> dependency preserved as Relation;
15. Value is not added to Entity taxonomy solely for provenance/unit metadata;
16. metrology service unavailable when explicit unit validation is required -> incomplete semantic evaluation, not backend incompatibility;
17. backend license/runtime absent -> no change to UnitReference/DimensionVector semantic contract.

## 17. Deferred

- a universal SemanticQuantity superclass/Entity;
- QuantityKind layer;
- hard QUDT dependency;
- backend-native unit vocabulary in Core;
- heterogeneous-unit tuple Values;
- distribution/random-field Value shape;
- complex-number Value scalar kind;
- interpolation versus tabular mechanism split;
- ExternalData mechanism versus provenance boundary;
- generic reification syntax for every typed construct;
- full PropertyDefinition serialization;
- final package placement of value-bearing fields/Constraint applications;
- broad `depends_on` range schema and generic value-bearing domain taxonomy.

## 18. Research verdict

**Ready for independent contract Validation.**
