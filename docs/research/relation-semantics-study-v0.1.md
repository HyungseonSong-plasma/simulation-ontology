# Relation Semantics Study v0.1

**Status:** Research/design candidate  
**Date:** 2026-08-20  
**Reference systems:** MOOSE, COMSOL, Ansys  
**Purpose:** Define the semantic boundary between RelationDefinition, relation instances, relation endpoint typing, inheritance, and relation constraints in SOL.

## 1. Research question

SOL already uses Relations for semantic references such as `targets`, `defined_on`, `parameterized_by`, and `solved_by`. The unresolved issue is how Relation semantics should be represented at the schema level and how much validation information belongs to a RelationDefinition versus a separate Constraint system.

A second question is how RelationDefinition domain/range contracts interact with Entity inheritance.

## 2. Cross-backend evidence

### MOOSE

MOOSE input metadata frequently encodes semantic references through parameters. Examples include `variable`, `boundary`, `block`, and coupled-variable parameters. MOOSE class inheritance also propagates valid input parameters from a base class to derived MooseObjects. This supports an inheritance-compatible semantic model: a more specialized object remains valid wherever the base semantic type is accepted.

### COMSOL

COMSOL separates parameter/property assignment from selection assignment, and its API interfaces themselves form superinterface/subinterface hierarchies. A `PhysicsFeature`, for example, implements common model-entity, parameter, and selection capabilities. This supports validating relation endpoints against semantic supertypes rather than exact backend object classes.

### Ansys

Mechanical objects similarly combine value-bearing properties with scoping/selection relationships and object categories. Backend categories and implementation inheritance are not copied into SOL, but they reinforce the need for endpoint contracts that accept specialized semantic entities through their supertypes.

## 3. Definition versus instance

SOL should distinguish:

```text
RelationDefinition
    identifier = targets
    semantic meaning = target field/scope/etc.
    domain/range contract = ...

RelationInstance / Edge
    BC_1 ── targets ──> TemperatureField
```

The relation definition has reusable schema identity. The individual edge is normally a graph statement and does not require independent Entity identity.

## 4. Candidate RelationDefinition structure

A minimal relation definition SHOULD contain:

```yaml
relation:
  id: targets
  description: Connects a semantic source entity to the entity it targets.
  domain: BoundaryCondition
  range: Field
  inverse: targeted_by   # optional
```

`domain` and `range` are semantic typing contracts for endpoints.

## 5. Cardinality boundary

Cardinality is not part of the semantic meaning of an edge itself. It is a validity requirement over the graph.

Therefore:

```text
TemperatureBC ── targets ──> TemperatureField
```

is the Relation instance, while:

```text
TemperatureBC MUST have exactly one targets edge
```

is a Constraint.

A RelationDefinition MAY reference or expose common cardinality constraints for authoring convenience, but cardinality semantics belong normatively to the Constraint system.

## 6. Domain/range inheritance semantics

### 6.1 Subtype compatibility

A relation endpoint satisfies a domain or range contract when the endpoint's semantic type is the declared type **or any subtype of that type**.

Example:

```text
BoundaryCondition
    ↑
TemperatureBoundaryCondition

Field
    ↑
TemperatureField
```

Given:

```yaml
relation:
  id: targets
  domain: BoundaryCondition
  range: Field
```

this edge is valid:

```text
TemperatureBC ── targets ──> TemperatureField
```

because both endpoint types are subtype-compatible with the declared contract.

Conceptually:

```text
valid(A ─R→ B)
iff
  type(A) <= domain(R)
  and
  type(B) <= range(R)
```

where `<=` means "is the same type as or a subtype of".

### 6.2 Inherited availability

A subtype does not need to redeclare a RelationDefinition merely to use a relation whose domain accepts one of its supertypes.

```text
BoundaryCondition can use targets
→ TemperatureBoundaryCondition can use targets
```

This follows semantic substitutability and avoids duplicating relation declarations throughout an inheritance tree.

MOOSE provides a useful implementation analogy: derived objects initialize their valid parameters from the base class and thereby inherit common parameters such as `variable`, `block`, or `boundary`. SOL does not copy this mechanism, but the substitutability principle is compatible with it.

### 6.3 Context-specific narrowing belongs to Constraint

Suppose the general relation is:

```text
targets
  domain = BoundaryCondition
  range  = Field
```

but a particular semantic type requires:

```text
TemperatureBoundaryCondition.targets
  range = TemperatureField
```

SOL SHOULD NOT redefine or override the `targets` RelationDefinition merely to narrow the endpoint type.

Instead:

```text
Constraint on TemperatureBoundaryCondition:
  relation = targets
  range must satisfy TemperatureField
```

This preserves one stable meaning for `targets` while allowing specialized validation rules.

### 6.4 Relation specialization is semantic, not merely structural

A specialized relation MAY be introduced when the relation itself has a more specific semantic meaning, not solely because its domain/range are narrower.

Example:

```text
depends_on
    ↑
thermally_depends_on
```

If SOL later supports relation specialization, a subrelation MUST be compatible with its parent:

```text
domain(subrelation) <= domain(parent)
range(subrelation)  <= range(parent)
```

or use the same endpoint contracts.

A subrelation MUST NOT widen the parent's domain or range because that would violate substitutability of the relation definition.

Relation specialization is not required merely for:

```text
BoundaryCondition.targets -> Field
TemperatureBoundaryCondition.targets -> TemperatureField
```

The latter is a Constraint unless `targets` itself acquires a distinct semantic meaning.

## 7. Domain/range expressions

For v0.1, a domain/range contract may initially be a single semantic type. Union/type-expression support can be added when reference models demonstrate a real need.

Preferred progression:

```text
v0.1 baseline:
  domain: BoundaryCondition
  range: Field

future if required:
  range:
    any_of: [Field, Scope]
```

The Core SHOULD prefer a meaningful common supertype over a union whenever one exists.

## 8. Inverse relation

An inverse relation is semantic metadata, not a graph constraint.

```text
targets ↔ targeted_by
solved_by ↔ solves
```

Inverse declaration MAY be optional. A backend need not serialize the inverse edge explicitly if it is derivable from the canonical relation.

If both directions declare endpoint contracts, they MUST be mutually compatible:

```text
domain(R) = range(inverse(R))
range(R)  = domain(inverse(R))
```

subject to subtype-compatible equivalence where specialization exists.

## 9. Relation identity and reification

A relation edge remains lightweight by default:

```text
A ── relation ──> B
```

If the connection itself needs provenance, version applicability, mapping parameters, state, or other relations, ADR 0001/0003-style reification rules apply.

The reified object is no longer merely an ordinary edge instance.

## 10. Candidate rules

### RLD1 — Relation Definition / Edge Separation

A reusable semantic relation SHALL have a stable RelationDefinition. An individual connection between semantic entities SHALL normally be represented as a graph edge using that definition rather than as an Entity.

### RLD2 — Endpoint Contract

A RelationDefinition SHOULD declare semantic domain and range contracts when these can be stated without over-constraining extensions.

### RLD3 — Cardinality as Constraint

Cardinality, requiredness, and occurrence-count rules SHALL be modeled as Constraints, not as intrinsic semantics of a relation edge.

### RLD4 — Inverse as Relation Metadata

An inverse relation MAY be declared on a RelationDefinition when the inverse has stable semantic meaning and can be derived unambiguously.

### RLD5 — Reify Only Rich Connections

A relation instance SHALL be reified only when the connection itself requires independent semantic identity, metadata, provenance, lifecycle, constraints, or relations.

### RLD6 — Inheritance-Compatible Endpoints

An Entity SHALL satisfy a relation domain/range contract when its type is the declared endpoint type or a subtype of that type.

### RLD7 — Constraint-Based Narrowing

Subtype-specific narrowing of a relation's permitted endpoints SHALL normally be expressed as a Constraint rather than by overriding the RelationDefinition.

### RLD8 — Semantic Relation Specialization

Relation specialization MAY be used only when the specialized relation has distinct semantic meaning. A specialized relation SHALL preserve substitutability by keeping or narrowing, never widening, the parent relation's endpoint contracts.

## 11. Implication for SOL architecture

The resulting separation is:

```text
Schema layer
  RelationDefinition
      ├── id
      ├── description
      ├── domain
      ├── range
      └── inverse?

Type layer
  Entity inheritance
      └── subtype compatibility

Graph/model layer
  Entity ── relation edge ──> Entity

Validation layer
  Constraint
      ├── cardinality
      ├── requiredness
      ├── subtype-specific endpoint narrowing
      └── context-specific restrictions
```

This keeps relation meaning stable while allowing domain ontologies and profiles to specialize validation without redefining Core relations.

## 12. Remaining questions

1. Whether multiple inheritance is permitted and how subtype compatibility is calculated in that case.
2. Whether relation specialization is needed in v0.1 or can remain deferred.
3. Whether symmetric/transitive/functional relation characteristics belong in RelationDefinition or Constraint semantics.
4. How profile/backend mappings refer to relation definitions.
5. Whether union/type-expression domain/range support is needed by the first reference models.
