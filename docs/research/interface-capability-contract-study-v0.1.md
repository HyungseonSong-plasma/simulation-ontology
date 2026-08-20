# Interface / Capability Contract Study v0.1

**Status:** Research/design candidate  
**Date:** 2026-08-20  
**Reference systems:** MOOSE, COMSOL, Ansys; ontology-design comparison with Palantir  
**Purpose:** Test whether a SOL Interface needs Property, Relation, and Constraint contracts, and whether additional Core capabilities are required.

## 1. Question

After preferring single taxonomic `is_a` inheritance plus capability composition, SOL needs to determine the minimum contract expressible by an Interface.

Candidate:

```text
Interface
 ├── Property requirements
 ├── Relation requirements
 └── Constraints
```

## 2. Evidence

### COMSOL

COMSOL's public Java API decomposes model capabilities through interfaces. `ParameterEntity` is a base interface for entities with parameters and extends selection/container capabilities. `PhysicsFeature` combines `ParameterEntity`, plotting/export, and equation-view capabilities. This is evidence that simulation objects frequently need orthogonal contracts rather than a single implementation hierarchy.

### MOOSE

MOOSE objects expose common capabilities through reusable framework interfaces/mixins and parameter contracts: variable coupling, block/boundary restriction, material-property access, controllable parameters, etc. Native implementation inheritance is backend-specific, but the recurring semantic pattern is that an object may satisfy several independent contracts.

### Ansys

Ansys simulation objects similarly combine value-bearing properties, scoping/selection, material/geometry references, and analysis-specific validity requirements. The exact API hierarchy is product-specific, so SOL should capture the semantic capabilities rather than reproduce an Ansys class hierarchy.

### Palantir comparison

Palantir interfaces explicitly define required/optional properties, link-type constraints, and action-type constraints. Implementing object types map concrete properties/links/actions to those interface contracts. This is strong evidence for separating reusable capability contracts from concrete object taxonomy.

## 3. Stress test

### Property contract

Example:

```text
ValueBearing
  requires property/value role: value_definition
```

A concrete type implementing `ValueBearing` must provide a compatible property/role. Therefore Property requirements are necessary.

### Relation contract

Example:

```text
SpatiallyScoped
  requires relation: defined_on
  range: Scope
```

A concrete implementation may use a more specialized compatible relation or endpoint. Therefore Relation requirements are necessary.

### Constraint contract

Presence of a property/relation alone is insufficient. Examples include:

```text
requires exactly one defined_on relation
value dimension must match expected SemanticQuantity
relation target must satisfy a subtype restriction
```

These are validation rules rather than Property or Relation identity. Therefore Interface must be able to attach or reference Constraints.

## 4. Do Interfaces need Actions/Operations in v0.1?

Palantir includes action-type constraints, and COMSOL exposes behavioral capabilities such as plotting/export through interfaces. However SOL v0.1 currently models simulation semantics and backend generation, not a general operational/action ontology.

Adding Action/Operation requirements now would pull execution/lifecycle behavior into the Core before the Action/Transformation boundary has been designed.

Decision candidate: defer operation/action contracts. The Interface model should be extensible so they can be added later without changing Property/Relation/Constraint semantics.

## 5. Proposed SOL Interface

An Interface is an abstract schema/capability resource. It is not directly instantiated as a simulation Entity.

```yaml
interface:
  id: SpatiallyScoped
  description: Requires a simulation entity to expose spatial scope semantics.

  relations:
    defined_on:
      range: Scope

  constraints:
    - relation: defined_on
      min_count: 1
```

A concrete Entity type may then declare:

```yaml
TemperatureBoundaryCondition:
  is_a: BoundaryCondition
  implements:
    - SpatiallyScoped
    - ValueBearing
```

## 6. Mapping rather than name equality

Interface conformance should be semantic, not based only on identical local names.

Conceptually:

```text
Interface requirement
  defined_on -> Scope

Concrete type
  applied_to -> Boundary

implementation mapping
  defined_on := applied_to
```

provided `Boundary` is compatible with `Scope` and all attached constraints are satisfied.

This is important for backend/domain ontologies whose local vocabulary differs while semantics agree.

## 7. Interface inheritance

Interfaces MAY extend other interfaces because capability composition is itself orthogonal. Multiple interface extension can be permitted if contracts are compatible.

Unlike Entity taxonomic inheritance, interface extension is contract composition rather than assertion that a concrete simulation entity has multiple ontological identities.

Conflict handling must be deterministic: incompatible requirements inherited for the same semantic role make the composed interface invalid unless an explicit compatible refinement exists.

## 8. Candidate rules

### IF1 — Abstract Capability Contract

An Interface SHALL define an abstract semantic capability and SHALL NOT be instantiated directly as a simulation Entity.

### IF2 — Minimum Contract Surface

SOL v0.1 Interfaces SHALL be able to declare or reference Property requirements, Relation requirements, and Constraints.

### IF3 — Multiple Interface Implementation

An Entity type MAY implement multiple Interfaces while retaining a single direct taxonomic `is_a` parent in SOL v0.1.

### IF4 — Semantic Implementation Mapping

Interface conformance SHALL permit explicit mapping from interface roles to compatible concrete Property/Relation definitions; identical local names SHALL NOT be required.

### IF5 — Constraint Satisfaction

An implementation SHALL satisfy all required Interface constraints after inheritance and implementation mappings are resolved.

### IF6 — Interface Extension

An Interface MAY extend one or more Interfaces when their contracts are mutually compatible. Interface extension is contract composition, not Entity taxonomic multiple inheritance.

### IF7 — Operation Contracts Deferred

Action/Operation/Transformation requirements are outside the minimum SOL v0.1 Interface contract and SHALL be deferred until the operational semantics boundary is designed.

## 9. Result

The stress test supports the minimum v0.1 Interface surface:

```text
Interface
 ├── Property requirements
 ├── Relation requirements
 └── Constraints
```

No fourth mandatory contract category is currently required for Core v0.1. Action/operation capabilities are a credible future extension but are not necessary to model the current MOOSE/COMSOL/Ansys semantic layer.

## 10. Architectural consequence

The preferred inheritance/composition model becomes:

```text
Entity type
  ├── is_a: at most one direct taxonomic parent
  └── implements: zero or more Interfaces

Interface
  ├── Property requirements
  ├── Relation requirements
  ├── Constraints
  └── extends: zero or more Interfaces
```

This preserves E2 (orthogonal semantic axes), avoids backend implementation hierarchy leakage, and provides a reusable contract mechanism for Profile and backend mapping work.

## 11. Remaining questions

1. Exact syntax for Property requirements.
2. Exact syntax for Relation requirement mappings.
3. Conflict-resolution rules for multiple Interface extension.
4. Whether optional requirements are supported in v0.1 or only required contracts.
5. Whether Interface identity belongs to the same namespace/versioning mechanism as Entity and Relation definitions.
