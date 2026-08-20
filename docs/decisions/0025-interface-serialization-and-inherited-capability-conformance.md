# ADR-0025: Interface Serialization and Inherited Capability Conformance

**Status:** Accepted  
**Date:** 2026-08-20  
**Supplements:** ADR-0007, ADR-0008, ADR-0009, ADR-0014, ADR-0018 through ADR-0024

## Context

ADR-0008 established SOL `Interface` as a reusable language-level capability contract, separate from Entity taxonomy and from the spatial `SpatialInterface` Entity. Machine-readable consolidation still required deterministic serialization for Interface extension, Property/Relation requirements, reusable Constraint applications, Entity Type implementation mappings, overlapping Interface contracts, and Entity-taxonomy inheritance of Interface guarantees.

Independent review of Interface serialization proposals v0.1 through v0.2.1 closed four contract gaps before this ADR was accepted.

## Decision

### 1. Interface remains a schema resource, not an Entity Type

`Interface` is a language-level capability/shape contract with stable canonical semantic identity under ADR-0009.

It SHALL NOT be inserted into the Entity Type taxonomy and SHALL remain distinct from `SpatialInterface` under ADR-0014.

Backend implementation-language interfaces/classes SHALL NOT determine SOL Interface identity or Entity `is_a` taxonomy.

### 2. InterfaceDefinition normalized shape

A focused normalized InterfaceDefinition contains:

```yaml
id: <canonical-interface-id>
extends: []
property_requirements: []
relation_requirements: []
constraint_applications: []
```

Only direct `extends` edges are serialized. Requirement/application collections are order-independent after canonical resolution.

### 3. Property and Relation requirements

Property and Relation requirements use canonical semantic identities:

```yaml
property_requirements:
  - property: <canonical-property-id>

relation_requirements:
  - relation: <canonical-relation-id>
```

Requirement identity is:

```text
(requirement_kind, canonical_requirement_id)
```

No Interface-local name-matching or second requirement-identity layer is introduced in v0.1.

Each listed requirement is a required **schema capability** of an implementing Entity Type. This does not itself require a model instance to contain a value or relation edge; instance requiredness/cardinality remains Constraint semantics.

Optional Interface members are deferred in v0.1.

### 4. Interface extension

An Interface MAY directly extend zero or more Interfaces.

The effective contract is the transitive conjunctive closure of local requirements/ConstraintApplications and direct-parent contracts.

The extension graph SHALL be acyclic:

```text
cycle -> INTERFACE_EXTENSION_CYCLE
```

Duplicate effective Property/Relation requirements collapse by canonical requirement key. Declaration order and extension-path order are non-semantic.

Contributor provenance SHALL be preservable even when semantic duplicates collapse.

### 5. ConstraintApplication

An Interface applies reusable reified ConstraintDefinitions through an explicit application wrapper:

```yaml
constraint: <canonical-constraint-definition-id>
target:
  kind: interface
```

or:

```yaml
constraint: <canonical-constraint-definition-id>
target:
  kind: property_requirement
  requirement: <canonical-property-id>
```

or:

```yaml
constraint: <canonical-constraint-definition-id>
target:
  kind: relation_requirement
  requirement: <canonical-relation-id>
```

This wrapper provides applicability/provenance context only. It does not introduce a seventh Constraint family or a graph-query language.

A Property/Relation target SHALL belong to the Interface's effective requirement contract.

Deterministic diagnostics include:

```text
INTERFACE_CONSTRAINT_TARGET_UNKNOWN
INTERFACE_CONSTRAINT_TARGET_KIND_MISMATCH
INTERFACE_CONSTRAINT_TARGET_INCOMPATIBLE
```

Duplicate ConstraintApplications collapse only when canonical ConstraintDefinition identity and normalized target are both equal.

### 6. ConstraintDefinition target-admissibility metadata

Every reified ConstraintDefinition used by an Interface ConstraintApplication SHALL expose a nonempty finite application-target contract:

```yaml
interface_application_target_kinds:
  - property_requirement
```

Allowed values are exactly:

```text
interface
property_requirement
relation_requirement
```

The set is application/context metadata around an existing six-family semantic ConstraintDefinition. It is not part of primitive-family payload identity or intersection algebra.

Outcomes:

```text
missing contract
  -> INTERFACE_CONSTRAINT_TARGET_CONTRACT_MISSING

empty/unknown-kind contract
  -> INTERFACE_CONSTRAINT_TARGET_CONTRACT_INVALID

target kind not allowed
  -> INTERFACE_CONSTRAINT_TARGET_INCOMPATIBLE
```

No target kind is inferred from family name, file location, payload appearance, backend metadata, or declaration order.

### 7. InterfaceImplementation direct declaration

Entity Type conformance uses explicit type-level declarations:

```yaml
entity_type: <canonical-entity-type-id>
interface: <canonical-interface-id>
property_mappings:
  - requirement: <required-property-id>
    concrete: <concrete-property-id>
relation_mappings:
  - requirement: <required-relation-id>
    concrete: <concrete-relation-id>
```

An InterfaceImplementation is schema metadata, not a model-instance Entity and not a backend MappingRule.

The direct declaration key is:

```text
(entity_type_id, interface_id)
```

At most one direct declaration for a key is valid in one resolved package environment:

```text
INTERFACE_IMPLEMENTATION_DUPLICATE_DIRECT
```

### 8. Explicit and complete requirement mappings

Every effective Property/Relation requirement in the implemented Interface closure SHALL have exactly one mapping in each direct declaration.

Even identity mappings remain explicit.

Diagnostics:

```text
0 mappings
  -> INTERFACE_REQUIREMENT_MAPPING_MISSING

>1 mappings
  -> INTERFACE_REQUIREMENT_MAPPING_AMBIGUOUS

mapping for non-effective requirement
  -> INTERFACE_REQUIREMENT_MAPPING_UNKNOWN

Property/Relation kind mismatch
  -> INTERFACE_REQUIREMENT_KIND_MISMATCH
```

Backend-local handles/paths are not valid Core concrete mapping identities.

### 9. Overlapping effective mapping convergence

An Entity Type may receive the same canonical requirement from multiple Interface paths/declarations.

For each:

```text
EffectiveRequirementKey
  = (requirement_kind, canonical_requirement_id)
```

collect all direct and inherited concrete mapping contributors.

v0.1 requires exact canonical convergence:

```text
one distinct concrete canonical ID
  -> one effective mapping; preserve contributor provenance

more than one distinct concrete canonical ID
  -> INTERFACE_EFFECTIVE_MAPPING_CONFLICT
```

No declaration/import priority resolves conflicts.

A future accepted Property/Relation refinement contract may permit proof of safe remapping between different concrete identities. v0.1 does not guess such refinement.

### 10. Entity taxonomic inheritance preserves Interface guarantees

Entity `is_a` specialization SHALL preserve Interface guarantees declared on Entity ancestors.

Let `Ancestors(E)` be the resolved Entity `is_a` closure including E.

```text
EffectiveInterfaces(E)
  = Interface-extension closure of
    union of direct InterfaceImplementation declarations
    over every Entity Type in Ancestors(E)
```

Ancestor declarations contribute their concrete mappings unchanged to subtype conformance.

A local overlapping mapping on a subtype must converge to the inherited concrete canonical identity under Section 9. Distinct remapping fails with `INTERFACE_EFFECTIVE_MAPPING_CONFLICT`.

Thus specialization may add Interface guarantees but SHALL NOT silently remove or reinterpret inherited guarantees.

### 11. Effective Interface Constraints on Entity subtypes

Interface ConstraintApplications are inherited through Interface extension and Entity `is_a` effective-Interface closure.

For a requirement-scoped ConstraintApplication, the canonical requirement target resolves through the Entity Type's converged effective mapping before the accepted Constraint-family validator checks the concrete semantic definition/refinement.

All effective Interface constraints compose conjunctively under ADR-0007.

### 12. Deterministic validation order

A focused Interface conformance pass conceptually:

1. resolves canonical Interface/Entity/Property/Relation/ConstraintDefinition references;
2. validates Entity taxonomy needed for ancestor closure;
3. validates Interface extension graph and cycles;
4. computes effective Interface contracts;
5. validates direct implementation-key uniqueness;
6. validates each direct declaration's mapping completeness;
7. collects direct + inherited effective mapping contributors;
8. requires exact canonical mapping convergence;
9. validates ConstraintApplication target existence/kind/admissibility;
10. applies effective Interface ConstraintDefinitions conjunctively through accepted Constraint-family semantics.

Backend runtime, license, release, adapter capability, and backend inheritance metadata are not Interface semantic inputs.

## Schema implementation slice

Design-stage consolidation SHALL add:

- `schema/interface-definition-v0.1.schema.json`;
- `schema/interface-implementation-v0.1.schema.json`;
- minimal semantic helper/tests for extension closure/cycles, ConstraintApplication target resolution, direct declaration uniqueness, mapping completeness/convergence, Entity-taxonomy inherited Interface guarantees, and declaration-order independence.

Final package placement of Interface/ConstraintDefinition registries and final PropertyDefinition serialization remain canonical-package integration work.

## Deferred

- Action/Operation requirements;
- optional Interface members;
- backend hierarchy mirroring;
- name-based implicit mapping;
- runtime duck typing;
- production SDK generation;
- safe remapping to a distinct concrete Property/Relation identity based on a future refinement proof;
- final package storage layout;
- universal validator-state envelope.

## Validation evidence

- `docs/research/sol-v0.1-interface-serialization-proposal-v0.1.md`
- `docs/validation/sol-v0.1-interface-serialization-independent-review-v0.1.md`
- `docs/research/sol-v0.1-interface-serialization-proposal-v0.2.md`
- `docs/validation/sol-v0.1-interface-serialization-v0.2-focused-review.md`
- `docs/research/sol-v0.1-interface-serialization-proposal-v0.2.1.md`
- `docs/validation/sol-v0.1-interface-serialization-final-contract-review.md`

## Decision summary

SOL v0.1 serializes Interface as a canonical reusable schema capability contract with explicit Property/Relation requirements, targeted reusable ConstraintApplications, acyclic multiple Interface extension, explicit complete Entity Type implementation mappings, exact canonical convergence for overlapping mappings, and monotonic inheritance of Interface guarantees through Entity taxonomic specialization.
