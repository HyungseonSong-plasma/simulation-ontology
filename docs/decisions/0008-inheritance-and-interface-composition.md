# ADR-0008: Inheritance and Interface Composition

**Status:** Accepted for SOL v0.1 architecture  
**Date:** 2026-08-20

## Context

SOL needs a stable way to express semantic specialization without copying backend implementation hierarchies or creating combinatorial class trees for orthogonal capabilities.

Evidence from MOOSE, COMSOL, Ansys, and Palantir-style ontology/interface design showed two distinct concerns:

1. **Taxonomic identity** — what a semantic type fundamentally *is*.
2. **Capability/contract composition** — which Properties, Relations, and Constraints a type must support.

These concerns should not be encoded through unrestricted multiple `is_a` inheritance.

Relevant evidence:

- [Entity boundary and orthogonal axes study](../research/entity-boundary-and-orthogonal-axes-study-v0.1.md)
- [Relation semantics study](../research/relation-semantics-study-v0.1.md)
- [Interface/capability contract study](../research/interface-capability-contract-study-v0.1.md)
- [Constraint composition principles validation](../research/constraint-composition-principles-validation-v0.1.md)
- [ADR-0007: Constraint architecture and composition](0007-constraint-architecture-and-composition.md)

## Decision

### 1. Single direct taxonomic inheritance

An SOL Entity Type SHALL have at most one direct `is_a` parent in v0.1.

```text
EntityType
  └── is_a -> 0..1 EntityType
```

This keeps taxonomic identity explicit and avoids ambiguous inheritance conflict resolution.

### 2. Capability composition through Interface

Orthogonal reusable semantic contracts SHALL be expressed through `Interface` rather than additional `is_a` parents.

```text
EntityType
  ├── is_a       -> 0..1 EntityType
  └── implements -> 0..* Interface
```

An Interface MAY define:

- Property requirements;
- Relation requirements;
- Constraints.

Interfaces SHALL NOT redefine the taxonomic identity of the implementing Entity Type.

### 3. Interface extension

An Interface MAY extend zero or more Interfaces.

```text
Interface
  └── extends -> 0..* Interface
```

Multiple Interface extension is contract composition, not Entity multiple inheritance.

All inherited Interface contracts compose conjunctively and are validated using ADR-0007.

### 4. Implementation mapping

An implementing Entity Type MAY satisfy an Interface contract through a compatible concrete Property or Relation whose local name differs from the Interface requirement.

Example:

```text
Interface requirement:
  defined_on -> Scope

Concrete ontology relation:
  applied_to -> Boundary

Implementation mapping:
  defined_on := applied_to
```

The concrete construct MUST satisfy the Interface semantic contract after subtype and constraint validation.

### 5. Backend inheritance independence

Backend implementation hierarchies SHALL NOT determine SOL `is_a` hierarchy.

```text
MOOSE C++ inheritance
COMSOL Java interfaces
Ansys API/object hierarchy
       !=
SOL semantic taxonomy
```

Backend metadata may provide evidence for capability contracts or mapping, but implementation-language inheritance is not normative semantic inheritance.

### 6. Orthogonal semantic axes remain separate

Characteristics such as value shape, evaluation mechanism, physical dimension, scoping capability, value-bearing capability, and equation participation SHOULD remain independent semantic axes when they can vary independently.

SOL SHALL avoid creating combinatorial taxonomy classes solely to encode independent capability combinations.

### 7. Interface conflicts are validation conflicts

If multiple implemented Interfaces impose incompatible requirements, SOL SHALL NOT choose one by declaration order or priority.

Their effective constraints are composed conjunctively under ADR-0007. Unsatisfiable composition is a schema conflict.

## Consequences

### Positive

- Entity taxonomy remains simple and explainable.
- Orthogonal capabilities can be reused without deep or multiple taxonomic inheritance.
- Interface composition is compatible with the established Constraint intersection model.
- Backend implementation details do not leak into Core semantics.
- Future domain/backend ontologies can add capabilities without modifying Core Entity inheritance.

### Costs

- A separate Interface construct and implementation mapping mechanism are required.
- Validators must resolve Interface extension and mapping before checking constraints.
- Some backend class/interface hierarchies cannot be represented by direct structural copying and require semantic normalization.

## Deferred

This ADR does not define:

- action/operation requirements on Interfaces;
- generic method signatures or executable behavior;
- arbitrary multiple Entity inheritance;
- union/intersection Entity types;
- runtime duck typing.

These require separate evidence before addition to Core.

## Decision summary

SOL v0.1 adopts **single direct Entity inheritance plus multi-Interface capability composition**. Interfaces carry reusable Property, Relation, and Constraint contracts; Interface extension may be multiple; and backend implementation inheritance remains non-normative to SOL taxonomy.
