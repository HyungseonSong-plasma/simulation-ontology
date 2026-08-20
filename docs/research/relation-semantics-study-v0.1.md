# Relation Semantics Study v0.1

**Status:** Research/design candidate  
**Date:** 2026-08-20  
**Reference systems:** MOOSE, COMSOL, Ansys  
**Purpose:** Define the semantic boundary between RelationDefinition, relation instances, relation endpoint typing, inheritance, relation characteristics, and relation constraints in SOL.

## 1. Research question

SOL uses Relations for semantic references such as `targets`, `defined_on`, `parameterized_by`, `depends_on`, and `solved_by`. The design question is how Relation semantics should be represented at schema level and how much validation information belongs to RelationDefinition versus Constraint.

## 2. Cross-backend evidence

### MOOSE

MOOSE input metadata frequently encodes semantic references through parameters. For example, `KokkosCoupledVarNeumannBC` has a `variable` identifying the variable the residual operates on, a `boundary` list identifying where the object applies, and `v` identifying a coupled variable. These are distinct reference roles even though the backend exposes them as input parameters.

This supports normalizing backend references into semantic relations while keeping occurrence requirements separate from relation identity.

### COMSOL

COMSOL explicitly separates parameter/property assignment from selection assignment. A physics interface or physics feature can be assigned to a named/local geometric selection. Physics Builder also distinguishes variable dependencies and necessary variables from ordinary user-input values. These provide evidence for semantic relations such as `defined_on`, `depends_on`, and `targets` independent of backend property syntax.

COMSOL selections can also be derived from source/destination selections or parent/physics selections. This is useful evidence that some relationships have stable directional meaning, but it does not imply generic symmetry or transitivity.

### Ansys

Ansys Mechanical exposes scoping through alternatives such as Geometry Selection and Named Selection. The scoped object and the selected geometry/selection play distinct roles. Contact and connection objects can additionally have source/target semantics. These patterns support directional semantic relations and use-site constraints rather than treating every backend reference as an untyped property.

## 3. Definition versus instance

SOL distinguishes:

```text
RelationDefinition
    identifier = targets
    semantic meaning = target field/scope/etc.
    domain/range contract = ...

Relation instance / edge
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

Cardinality is a validity requirement over graph usage, not intrinsic edge meaning.

```text
TemperatureBC ── targets ──> TemperatureField
```

is a Relation instance, while:

```text
TemperatureBC MUST have exactly one targets edge
```

is a Constraint.

A RelationDefinition MAY reference common cardinality constraints for authoring convenience, but cardinality semantics belong normatively to Constraint.

## 6. Domain/range inheritance semantics

### 6.1 Subtype compatibility

A relation endpoint satisfies a domain or range contract when the endpoint's semantic type is the declared type or any subtype.

```text
valid(A ─R→ B)
iff
  type(A) <= domain(R)
  and
  type(B) <= range(R)
```

where `<=` means same type or subtype.

### 6.2 Inherited availability

A subtype does not redeclare a RelationDefinition merely to use a relation whose domain accepts a supertype.

### 6.3 Context-specific narrowing belongs to Constraint

If `targets` generally maps `BoundaryCondition -> Field` but `TemperatureBoundaryCondition` must target `TemperatureField`, the specialized endpoint restriction is a Constraint rather than an override of `targets`.

### 6.4 Relation specialization is semantic

A specialized relation MAY be introduced when the relation itself has a more specific semantic meaning, not solely because its domain/range are narrower. A subrelation must preserve substitutability and may not widen parent endpoints.

## 7. Domain/range expressions

For v0.1, domain/range SHOULD initially be a single semantic type. Union/type-expression support is deferred until reference models demonstrate a real requirement. A meaningful common supertype is preferred over a union.

## 8. Inverse relation

An inverse relation is semantic metadata, not a graph occurrence constraint.

```text
targets ↔ targeted_by
solved_by ↔ solves
```

Inverse declaration is optional. If both directions declare endpoint contracts, they must be mutually compatible.

## 9. Relation identity and reification

A relation edge remains lightweight by default. If the connection itself needs provenance, version applicability, mapping parameters, state, or other relations, the connection may be reified according to the general semantic-identity boundary.

## 10. Relation characteristics stress test

### 10.1 Functional and inverse-functional

A functional relation means a source may have at most one target for that relation. This is equivalent to a maximum-cardinality rule:

```text
functional(R) ≡ max_count(source, R) = 1
```

Likewise inverse-functional is an incoming-cardinality rule. Because cardinality may differ by Entity subtype, Interface, Profile, or use site, SOL SHOULD model functional/inverse-functional behavior through Constraint rather than as intrinsic RelationDefinition characteristics.

Example:

```yaml
constraint:
  applies_to: TemperatureBoundaryCondition
  relation: targets
  max_count: 1
```

This allows another valid use of the same `targets` relation to have different cardinality without redefining relation meaning.

### 10.2 Symmetric

Symmetry changes the logical meaning of the relation:

```text
A R B => B R A
```

It is therefore a RelationDefinition characteristic when SOL explicitly needs this inference semantics.

A symmetric relation requires compatible endpoint contracts. In the simplest v0.1 form, domain and range should be the same semantic type or otherwise mutually compatible.

Backend examples reviewed in MOOSE, COMSOL, and Ansys are predominantly directional (`variable`, `boundary`, `selection`, source/target, dependency). They do not provide a strong requirement for a generic symmetric relation in the initial Core vocabulary. Therefore symmetry support may exist in the RelationDefinition model while no Core relation is required to use it initially.

### 10.3 Transitive

Transitivity also changes logical/inference semantics:

```text
A R B and B R C => A R C
```

It belongs to RelationDefinition if supported, not Constraint, because it defines what additional relation statements follow from existing statements rather than whether a local graph occurrence is valid.

The reviewed simulation-backend relations do not establish a strong need for transitivity in v0.1. In particular, dependency, geometric scoping, variable coupling, and source/target relations should not be assumed transitive without explicit semantic evidence.

Therefore SOL SHOULD NOT mark relations transitive by default.

### 10.4 Reflexive, irreflexive, asymmetric

These characteristics describe relation logic but are not currently required by the MOOSE/COMSOL/Ansys reference cases. They SHOULD be deferred from the minimal v0.1 schema unless a reference model requires them.

Self-reference prohibitions such as `A must not depend_on itself` can initially be represented as Constraints without introducing a full relation-algebra vocabulary.

## 11. Cross-backend stress-test matrix

| Backend pattern | SOL normalization | Relation characteristic required? | Constraint required? |
|---|---|---|---|
| MOOSE BC `variable = u` | `operates_on -> Field` | No | Type/cardinality may be constrained by BC type |
| MOOSE `boundary = ...` | `defined_on -> Boundary/Scope` | No | Requiredness/cardinality use-site specific |
| MOOSE coupled variable `v` | `depends_on` / `coupled_to -> Field` | No generic transitivity | Allowed target quantity/type may be constrained |
| COMSOL physics/feature `selection()` | `defined_on -> Scope` | No | Geometry level/applicability constraints |
| COMSOL necessary variable | `depends_on -> Variable/Field` | Do not assume transitive | Existence/type constraints |
| Ansys Geometry/Named Selection scoping | `defined_on -> Scope` | No | Allowed scope type and occurrence constraints |
| Ansys source/target connection roles | directional source/target relations | Not symmetric by default | Endpoint/type constraints |

The test shows that simulation-backend relations are mostly directional semantic references. Cardinality and applicability vary by object/use site and therefore fit Constraint. Symmetry/transitivity are genuine relation semantics but are not required for the ordinary backend references examined.

## 12. Candidate normative rules

### RLD1 — Relation Definition / Edge Separation
A reusable semantic relation SHALL have a stable RelationDefinition. Individual connections SHALL normally be graph edges.

### RLD2 — Endpoint Contract
A RelationDefinition SHOULD declare semantic domain and range contracts when these can be stated without over-constraining extensions.

### RLD3 — Cardinality as Constraint
Cardinality, requiredness, functional behavior, and inverse-functional behavior SHALL be modeled as Constraints.

### RLD4 — Inverse as Relation Metadata
An inverse relation MAY be declared when it has stable semantic meaning and is unambiguous.

### RLD5 — Reify Only Rich Connections
A relation instance SHALL be reified only when the connection itself requires independent semantic identity or metadata.

### RLD6 — Inheritance-Compatible Endpoints
An Entity satisfies a relation endpoint contract when its type is the declared type or a subtype.

### RLD7 — Constraint-Based Narrowing
Subtype/use-site narrowing of permitted endpoints SHALL normally be expressed as Constraint rather than RelationDefinition override.

### RLD8 — Semantic Relation Specialization
Relation specialization MAY be used only for genuinely specialized relation meaning and SHALL not widen parent endpoint contracts.

### RLD9 — Logical Relation Characteristics
Symmetry and transitivity, when explicitly supported, SHALL be properties of RelationDefinition because they define inference semantics rather than local occurrence validity. Neither characteristic SHALL be inferred from backend structure or enabled by default.

### RLD10 — Minimal v0.1 Relation Algebra
The minimal v0.1 Core need not require reflexive, irreflexive, asymmetric, or other advanced relation characteristics until demonstrated by reference models. Equivalent local validity requirements MAY initially be expressed through Constraints.

## 13. Implication for SOL architecture

```text
Schema layer
  RelationDefinition
      ├── id
      ├── description
      ├── domain
      ├── range
      ├── inverse?
      └── logical characteristics?   # symmetric/transitive only if needed

Graph/model layer
  Entity ── relation edge ──> Entity

Validation layer
  Constraint
      ├── cardinality / requiredness
      ├── functional / inverse-functional
      ├── subtype-specific endpoint narrowing
      ├── scope/applicability
      └── context-specific restrictions
```

## 14. Remaining questions

1. Profile/backend mapping contract for RelationDefinitions.
2. Whether relation specialization is needed in v0.1 or should remain deferred.
3. Whether union/type-expression domain/range is required by the first reference models.
4. Exact Constraint representation and composition semantics.

