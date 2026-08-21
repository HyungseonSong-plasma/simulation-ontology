# ADR-001 — First-Class SpatialScope

**Status:** Accepted  
**Date:** 2026-08-21  
**Milestone:** M0.1 Semantic Core Bootstrap  
**Related:** #2, #7

## Context

The Core Simulation Ontology v0.1 implementation plan requires Spatial Scope to be a first-class semantic concept. The current implementation models spatial structure with `SpatialModel`, but does not separately represent the solver-independent applicability set used by equations, conditions, and other semantic objects.

Conflating spatial structure with applicability would make it difficult to preserve semantic intent across backends. Conversely, introducing backend-native selection objects such as MOOSE side sets, COMSOL selections, Ansys named selections, or Gmsh physical groups into Core would violate the architecture boundary.

## Decision

`SpatialModel` and `SpatialScope` are distinct Core concepts.

- `SpatialModel` represents spatial structure, for example a domain or boundary.
- `SpatialScope` represents the solver-independent spatial applicability set of semantic objects.
- Every `SpatialScope` has solver-independent canonical identity.
- M0.1 supports explicit static membership only.
- Each scope member is a reference to a Core `SpatialModel` entity.
- Scope membership is serialized compactly as references, but the canonical resolver resolves and validates those references.
- A valid M0.1 scope is non-empty and duplicate-free.
- Unresolved members are invalid.
- Members resolving to non-spatial semantic entities are invalid.
- Backend-native selections are adapter realizations of `SpatialScope`; they do not become Core identity or Core ontology concepts.

Conceptually:

```text
SpatialModel = what spatial structure exists
SpatialScope = where a semantic object applies
Backend selection = how an adapter realizes that scope
```

Example:

```text
scope.main_domain
  members:
    - domain.main

scope.hot_wall
  members:
    - boundary.hot_wall
```

Semantic use remains distinct from quantity targeting. For example, a boundary condition may target a temperature field and also a spatial scope; introducing Scope does not erase the semantic target quantity.

## M0.1 Representation

The serialization shape is intentionally minimal:

```rust
pub struct SpatialScope {
    pub id: String,
    pub members: Vec<String>,
}
```

The resolved representation uses canonical references conceptually equivalent to:

```rust
pub struct ResolvedSpatialScope {
    pub id: CanonicalId,
    pub members: Vec<CanonicalId>,
}
```

No new ontology relation is required for scope membership in M0.1. Membership is a first-class resolved structure rather than a backend selection or an inferred relation.

## Validation Rules

M0.1 validators must enforce:

1. scope id is a valid canonical id;
2. scope id is unique across the semantic graph;
3. scope membership is non-empty;
4. members are duplicate-free;
5. every member resolves;
6. every member resolves to `SpatialModel`;
7. backend-native identifiers are not used as Core scope identity.

Required executable cases include:

```text
PASS: scope.hot_wall -> boundary.hot_wall
FAIL: empty scope
FAIL: duplicate boundary.hot_wall member
FAIL: unresolved spatial member
FAIL: member resolves to solver/material/non-spatial entity
```

## Consequences

### Connectivity

Scope becomes an explicit canonical node with resolved membership rather than an opaque string collection. Semantic consumers can reference Scope while the resolver preserves graph connectivity to spatial entities.

### Extensibility

The contract can later grow to composite scopes (`union`, `intersection`, `difference`) or predicate/query scopes without changing the distinction between spatial meaning and backend realization.

### Simplicity

M0.1 deliberately excludes query languages, geometry predicates, scope algebra, and backend-native selection metadata. Explicit membership is sufficient for the Thermal vertical slice and architecture gate.

## Rejected Alternatives

### Treat `SpatialModel` itself as Scope

Rejected because spatial structure and semantic applicability are different concepts.

### Introduce backend-native selections into Core

Rejected because this would make canonical semantics backend-dependent.

### Add a new `contains` semantic relation immediately

Rejected for M0.1 because explicit resolved membership provides the required first-class connectivity without expanding the approved relation vocabulary.

### Add predicate/query scopes in M0.1

Rejected as unnecessary complexity before the explicit membership contract is stable.

## Follow-up

Phase 1 issue #7 must implement this contract, add Thermal positive coverage plus counterexamples, and close only after executable CI evidence confirms the architecture gate.