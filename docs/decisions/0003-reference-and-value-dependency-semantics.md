# ADR 0003 — Reference and Value Dependency Semantics

**Status:** Accepted  
**Date:** 2026-08-20  
**Scope:** Simulation Ontology Language v0.1

## Context

ADR 0002 distinguishes semantic concepts, `ValueDefinition`, and evaluated `Value`. During that work, `ReferenceDefinition` was considered as a possible ValueDefinition subtype.

Cross-backend validation against MOOSE, COMSOL, and Ansys Mechanical, followed by a pure-alias stress test, showed that the term `reference` conflates different semantic concerns:

1. one semantic object referring to another;
2. one concept obtaining its value unchanged from another concept;
3. a ValueDefinition evaluating a transformation that depends on another concept.

The supporting research is recorded in:

- `docs/research/reference-vs-value-definition-study-v0.1.md`
- `docs/research/cross-backend-semantic-mapping-matrix-v0.1.md`

## Decision

### R1 — Reference as Relation

> **A reference to an independently identifiable semantic concept SHALL normally be represented by a semantic `Relation`, not by a `ReferenceDefinition`.**

Examples:

```text
TemperatureBC ── targets ──> TemperatureField
Equation ── defined_on ──> Domain
HeatConduction ── parameterized_by ──> ThermalConductivity
```

Backend names, identifiers, object handles, selection names, and similar native reference mechanisms do not determine a SOL value type.

### R2 — Value Dependency

> **When one concept obtains its value from another concept, the dependency SHALL be represented semantically as a value-dependency `Relation`. A `ValueDefinition` is required only when an evaluation or transformation mechanism itself must be represented.**

Pure alias:

```text
B = A

B ── derives_value_from ──> A
```

Transformation:

```text
B = f(A)

B
 └── has_value_definition
        └── FunctionDefinition
               └── depends_on ──> A
```

The same principle applies to expression, function, tabular, interpolation, and other evaluation mechanisms.

## Consequences

1. `ReferenceDefinition` is removed from the default SOL v0.1 `ValueDefinition` candidate taxonomy.
2. Object references remain within the existing `Relation` model.
3. Pure value aliases do not require an intermediate ValueDefinition node.
4. Computational dependencies are expressed as Relations from the relevant ValueDefinition to semantic dependency concepts.
5. Backend-native reference syntax must be normalized semantically rather than copied into Core SOL.
6. Graph complexity should increase only when semantic complexity increases.

## Semantic pattern

```text
Object dependency
A ── semantic_relation ──> B

Pure value dependency
A ── derives_value_from ──> B

Computational dependency
A
 └── has_value_definition
        └── ValueDefinition
               └── depends_on ──> B
```

## Rationale

A generic `ReferenceDefinition` would encode the mechanism by which a backend identifies another object rather than the semantic meaning of the connection. MOOSE coupled variables and material-property names, COMSOL expressions and material sourcing, and Ansys field/function/tabular dependencies all demonstrate that native reference mechanisms map to different semantic relations.

The pure-alias case also does not require a separate evaluation object when no transformation occurs. Representing `B = A` directly as `B derives_value_from A` is both sufficient and more consistent with SOL's existing Entity/Relation boundary.

## Interaction with ADR 0002

ADR 0002 remains unchanged:

- `Value` is an evaluated typed datum.
- `ValueDefinition` describes how a Value is obtained.
- value shape and evaluation mechanism are orthogonal.

ADR 0003 further constrains V2: merely obtaining a value unchanged from another semantic concept does not by itself require a ValueDefinition. A ValueDefinition represents an evaluation mechanism when that mechanism carries semantic information.

## Deferred decisions

This ADR does not decide:

- whether `depends_on` is a single generic relation or a super-relation with specialized subrelations;
- whether dependency relations are authored explicitly or inferred from expression/function ASTs;
- cyclic-dependency validation rules;
- the exact boundary between `TabularDefinition` and `InterpolationDefinition`;
- unit and physical-dimension semantics;
- provenance and external-data semantics.
