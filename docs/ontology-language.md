# Simulation Ontology Language

**Version:** 0.1-draft  
**Status:** Language design

## 1. Purpose

The Simulation Ontology Language (SOL) defines the minimum semantic constructs required to build, extend, compose, validate, and map simulation ontologies.

The language is independent of serialization and implementation technologies. YAML, JSON-LD, RDF/OWL, SHACL, Python, and TypeScript are candidate representations and tooling layers, not the language itself.

## 2. Design goals

The language MUST support:

1. A solver-independent core ontology.
2. Domain-specific ontology extensions.
3. Backend-specific ontology extensions.
4. Independent evolution of domain and backend ontologies.
5. Composition through Simulation Profiles.
6. Explicit semantic relations.
7. Machine-verifiable constraints.
8. Namespaced identifiers and versioned dependencies.
9. Mapping from semantic concepts to backend realizations.
10. Multiple serialization and SDK implementations.

## 3. Minimum language constructs

Version 0.1 starts with seven normative constructs and one provisional capability construct:

```text
Ontology
├── Namespace
├── Entity
├── Property
├── Relation
├── Constraint
├── Extension
├── Profile
└── Interface   [provisional]
```

`Value` and `ValueDefinition` now have accepted semantic rules (ADR 0002), but their final language-level representation is intentionally not yet classified as a normative construct. `Action` is also deferred.

### 3.1 Namespace

A `Namespace` provides stable identity and prevents naming collisions across independently developed ontologies.

Conceptual example:

```text
sim:Equation
plasma:ElectronTransport
moose:ADDiffusion
```

A namespace MUST identify the ontology that owns a term.

### 3.2 Entity

An `Entity` represents a semantic concept with independent identity in the ontology graph.

An entity MAY specialize another entity, declare properties, participate in relations, be constrained by validation rules, or be extended by another ontology.

#### Entity boundary rule

A concept SHOULD be modeled as an Entity when it needs independent semantic identity: it can be referenced by multiple other concepts, participate in relations of its own, carry non-trivial metadata, or evolve independently from the entity that uses it.

A concept SHOULD NOT be promoted to an Entity merely because it has a value.

### 3.3 Property

A `Property` represents an intrinsic characteristic or value-bearing attribute of an Entity. It does not represent an independently meaningful semantic connection between two entities.

#### Property boundary rule

Use a Property when the information primarily answers what characteristic or value an Entity has. Use a Relation when the information primarily answers how one independently identifiable Entity is semantically connected to another.

A value-bearing concept MAY be promoted from Property to Entity when it requires its own identity, relations, provenance, functional dependence, uncertainty model, or reusable metadata.

### 3.4 Relation

A `Relation` is a first-class semantic link between independently identifiable ontology Entities.

A relation SHOULD specify its identifier, semantic description, allowed domain/range, optional inverse, and cardinality where applicable.

Core examples include `represented_by`, `closed_by`, `parameterized_by`, `defined_on`, `discretized_by`, `applied_to`, `analyzed_by`, `solved_by`, `produces`, and `observed_by`.

#### Relation boundary rule

Use a Relation when both endpoints have independent semantic identity and the connection itself has domain meaning.

A Relation SHOULD remain lightweight. If the connection itself needs substantial properties, provenance, version constraints, parameter mappings, state, or relations of its own, the connection SHOULD be reified as an Entity.

### 3.5 Constraint

A `Constraint` expresses a machine-verifiable requirement on an ontology or model graph, including required properties/relations, cardinality, allowed entity types, dimensional consistency, namespace rules, dependency restrictions, and graph invariants.

### 3.6 Extension

An `Extension` allows an ontology to add specialized concepts while depending on a lower-level ontology. The primary extension classes are `DomainExtension` and `BackendExtension`.

Extensions MUST obey the inward dependency rule:

```text
specialized ontology → simulation-ontology
```

The core MUST NOT depend on an extension.

### 3.7 Profile

A `Profile` composes compatible ontology extensions for a concrete simulation environment.

```text
MOOSE Plasma Profile
    imports plasma-ontology
    imports moose-ontology
```

Backend realization bindings SHOULD normally live in profiles or backend mapping modules rather than polluting the domain ontology.

### 3.8 Interface [provisional]

An `Interface` defines a reusable semantic contract or capability that multiple Entity types can satisfy without forcing them into a single inheritance branch.

`Interface` remains provisional until reference models demonstrate that capability-based polymorphism cannot be represented cleanly through Entity inheritance and Constraints alone.

## 4. Normative semantic boundary rules

### Rule 1 — Identity

If a concept has independent semantic identity, model it as an `Entity`.

### Rule 2 — Intrinsic characteristic

If information is primarily an intrinsic characteristic or literal/value-bearing attribute of one Entity, model it as a `Property`.

### Rule 3 — Semantic connection

If two independently identifiable Entities are connected in a way that has domain meaning, model the connection as a `Relation`.

### Rule 4 — Reification

If a Relation itself requires substantial properties, provenance, state, constraints, mappings, or relations, reify it as an `Entity` rather than overloading the Relation.

### Rule 5 — Capability contract

If multiple unrelated Entity types must satisfy the same reusable semantic shape or capability, prefer an `Interface` if inheritance and constraints would otherwise create artificial taxonomy.

### Rule 6 — Value is not the semantic concept

`Value` is an evaluated typed datum and MUST NOT be identified with the semantic quantity or simulation concept whose value it represents.

```text
ThermalConductivity ≠ 400 W/(m K)
```

The former is a semantic concept; the latter is a concrete evaluated datum.

### Rule 7 — ValueDefinition defines evaluation

`ValueDefinition` describes how a `Value` is obtained. Candidate mechanisms include literal, expression, function, tabular/interpolation, and external-data definitions. This subtype taxonomy is not yet normative.

```text
Semantic Concept
      │ has_value_definition
      ▼
ValueDefinition
      │ evaluates_to
      ▼
Value
```

### Rule 8 — Value shape and evaluation mechanism are orthogonal

The shape/type of an evaluated datum and the mechanism by which it is obtained MUST be modeled as independent dimensions.

```text
Value shape:
  Scalar | Vector | Tensor

Evaluation mechanism:
  Literal | Expression | Function | Tabular | ...
```

A tensor-valued quantity may therefore be literal, functional, tabulated, or otherwise evaluated without changing its tensor shape.

These value rules are accepted in ADR 0002 and supported by the cross-backend semantic mapping evidence in `docs/research/cross-backend-semantic-mapping-matrix-v0.1.md`.

## 5. Ontology package model

Each ontology package SHOULD declare at least:

```text
identity
namespace
version
dependencies
entities
properties
relations
constraints
```

A domain or backend extension additionally declares its extension type and parent/core dependency. A profile declares the ontology packages it composes and the bindings required to make that composition executable or generatable.

## 6. Dependency model

Allowed dependency direction:

```text
Application
    ↓
Profile
    ↓
Domain / Backend Ontology
    ↓
Core Ontology
```

Core-to-backend, core-to-domain, and direct domain-to-backend dependencies are forbidden by default. Domain/backend integration belongs to a Profile.

## 7. Identity and references

Every exported ontology term MUST have a stable namespaced identifier. References between ontology packages MUST use semantic identifiers rather than source-file paths or implementation class names.

## 8. Semantic graph model

An instantiated simulation ontology forms a graph in which semantic Entities are connected by Relations and may carry intrinsic Properties. Values are explicitly distinguished from the semantic concepts they quantify.

```text
Entity ──Relation──> Entity
  │
  └── has_value_definition
              │
              ▼
       ValueDefinition
              │
          evaluates_to
              ▼
            Value
```

The final graph-level representation of `Value` and `ValueDefinition` remains an implementation/design question; the semantic distinction is normative.

## 9. Candidate implementation stack

| Concern | Candidate technology |
|---|---|
| Human authoring | YAML DSL |
| Structural validation | JSON Schema |
| Canonical graph interchange | JSON-LD |
| Semantic graph model | RDF / OWL |
| Graph constraints | SHACL |
| Reference SDK | Python |
| Python graph tooling | RDFLib / pySHACL |
| Typed configuration | Pydantic |
| Frontend SDK | TypeScript |
| UI | React |
| Versioning | Semantic Versioning |
| CI | GitHub Actions |

No technology in this table is part of the normative language contract at v0.1.

## 10. Proposed processing pipeline

```text
Human-authored ontology
        │
      YAML
        │
        ▼
Structural validation
        │
        ▼
Ontology compiler
        │
        ▼
Canonical semantic graph
   JSON-LD / RDF
        │
        ▼
Semantic validation
      SHACL
        │
        ▼
Profile composition
        │
        ▼
Backend IR / generator
```

## 11. Open questions for v0.1

The following questions remain unresolved:

1. Whether `Value` is a first-class language construct, typed data object, or graph Entity.
2. Normative subtype taxonomy of `ValueDefinition`.
3. Whether `Reference` is a `ValueDefinition` or remains a Relation.
4. Exact unit and physical-dimension ownership and validation semantics.
5. Provenance/data-source representation for value definitions.
6. Whether `Result` is a core Entity type or runtime artifact type.
7. Formal inheritance semantics.
8. Relation domain/range and cardinality semantics.
9. Required versus optional constraints.
10. Namespace and URI convention.
11. Ontology package version compatibility rules.
12. Extension conflict-resolution rules.
13. Profile binding semantics and when bindings must be reified as `BackendBinding` entities.
14. Whether `Interface` becomes normative in v0.1.
15. Whether actions/transformations should become a first-class construct in a later version.

## 12. v0.1 success criterion

The language is sufficiently defined when the same core language can independently express a small domain ontology, a backend ontology such as MOOSE, a profile composing the two, enough constraints to reject invalid composition, and enough mapping information to generate or construct a minimal backend simulation model.

The first reference validation should use a small steady-state heat-conduction model before expanding to plasma and multiphysics domains.
