# Relation Semantics Study v0.1

**Status:** Research/design candidate  
**Date:** 2026-08-20  
**Reference systems:** MOOSE, COMSOL, Ansys  
**Purpose:** Define the semantic boundary between RelationDefinition, relation instances, and relation constraints in SOL.

## 1. Research question

SOL already uses Relations for semantic references such as `targets`, `defined_on`, `parameterized_by`, and `solved_by`. The unresolved issue is how Relation semantics should be represented at the schema level and how much validation information belongs to a RelationDefinition versus a separate Constraint system.

## 2. Cross-backend evidence

### MOOSE

MOOSE input metadata frequently encodes semantic references through parameters. Examples include:

- `variable` referencing the variable an object acts on;
- `boundary` or `block` selecting where an object applies;
- coupled-variable parameters referencing other variables.

These native parameter names normalize naturally to semantic edges rather than value-bearing Properties.

### COMSOL

COMSOL separates parameter/property assignment from selection assignment. A physics feature can receive values through `set(...)` and can independently be assigned to geometric entities through `selection()`. This supports separating value semantics from relation semantics.

### Ansys

Mechanical objects similarly combine value-bearing properties with scoping/selection relationships. Geometry or Named Selection scoping is semantically a relation between the simulation object and a target scope even when represented as an object property in the native API.

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

This avoids conflating:

```text
what the relation means
```

with:

```text
how many such relations are valid in a particular schema/context
```

## 6. Domain/range boundary

Domain and range are stronger candidates for RelationDefinition semantics because they define which semantic endpoint categories the relation is intended to connect.

Example:

```text
targets
  domain: BoundaryCondition
  range: Field
```

However, specialized domains may narrow these contracts. For example a domain ontology may permit `targets` from another model entity type. Therefore SOL should support either inheritance-compatible domain/range widening/narrowing or relation specialization rather than forcing all use-site restrictions into the Core relation definition.

## 7. Inverse relation

An inverse relation is semantic metadata, not a graph constraint.

```text
targets ↔ targeted_by
solved_by ↔ solves
```

Inverse declaration MAY be optional. A backend need not serialize the inverse edge explicitly if it is derivable from the canonical relation.

## 8. Relation identity and reification

A relation edge remains lightweight by default:

```text
A ── relation ──> B
```

If the connection itself needs provenance, version applicability, mapping parameters, state, or other relations, ADR 0001/0003-style reification rules apply:

```text
A
 ↓
ReifiedConnection
 ├── source → A
 ├── target → B
 └── metadata ...
```

The reified object is no longer merely an ordinary edge instance.

## 9. Candidate rules

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

## 10. Implication for SOL architecture

The likely structure is:

```text
Schema layer
  RelationDefinition
      ├── id
      ├── description
      ├── domain
      ├── range
      └── inverse?

Graph/model layer
  Entity ── relation edge ──> Entity

Validation layer
  Constraint
      ├── cardinality
      ├── requiredness
      └── context-specific endpoint restrictions
```

This keeps semantic meaning, graph data, and validation rules separate.

## 11. Remaining questions

1. Whether domain/range contracts are single types or unions/expressions.
2. How inheritance affects relation domain/range compatibility.
3. Whether relation specialization is needed in v0.1.
4. Whether symmetric/transitive/functional relation characteristics belong in RelationDefinition or Constraint semantics.
5. How profile/backend mappings refer to relation definitions.
