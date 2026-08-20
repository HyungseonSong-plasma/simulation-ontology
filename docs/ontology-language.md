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

Version 0.1 starts with seven constructs:

```text
Ontology
├── Namespace
├── Entity
├── Property
├── Relation
├── Constraint
├── Extension
└── Profile
```

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

An `Entity` represents a semantic type or concept.

An entity MAY:

- specialize another entity;
- declare properties;
- participate in relations;
- be constrained by validation rules;
- be extended by another ontology.

Conceptual example:

```text
plasma:ElectronTransport
    is_a sim:PhysicsModel
```

### 3.3 Property

A `Property` represents an intrinsic attribute of an entity rather than a semantic link to another entity.

Examples include labels, scalar parameters, units, dimensions, numerical values, configuration values, or metadata.

The distinction between `Property` and `Relation` is semantic:

```text
thermal_conductivity = 400 W/(m K)   → Property/value
ElectronTransport represented_by X   → Relation
```

The exact boundary between properties and relations will be refined during v0.1 validation.

### 3.4 Relation

A `Relation` is a first-class semantic link between ontology entities.

A relation definition SHOULD specify:

- identifier;
- semantic description;
- allowed domain;
- allowed range;
- optional inverse relation;
- cardinality constraints where applicable.

Core examples include:

```text
represented_by
closed_by
parameterized_by
defined_on
discretized_by
applied_to
analyzed_by
solved_by
produces
observed_by
```

Relations are not implementation pointers. They express semantic meaning in the simulation graph.

### 3.5 Constraint

A `Constraint` expresses a machine-verifiable requirement on an ontology or model graph.

Constraints MAY express:

- required properties;
- required relations;
- cardinality;
- allowed entity types;
- dimensional consistency;
- namespace rules;
- dependency restrictions;
- graph invariants.

Example:

```text
BoundaryCondition
    requires target Field
    requires Scope where spatial scoping is applicable
```

The language defines constraint semantics independently of the validation technology. SHACL is a candidate implementation for graph constraints.

### 3.6 Extension

An `Extension` allows an ontology to add specialized concepts while depending on a lower-level ontology.

Two primary extension classes are expected:

```text
DomainExtension
BackendExtension
```

A domain extension adds scientific or physical semantics. A backend extension adds simulation-software semantics.

Extensions MUST obey the inward dependency rule:

```text
specialized ontology → simulation-ontology
```

The core MUST NOT depend on an extension.

### 3.7 Profile

A `Profile` composes compatible ontology extensions for a concrete simulation environment.

Example:

```text
MOOSE Plasma Profile
    imports plasma-ontology
    imports moose-ontology
```

A profile MAY define bindings such as:

```text
plasma:DiffusionOperator
    realized_by
moose:ADDiffusion
```

Backend realization bindings SHOULD normally live in profiles or backend mapping modules rather than polluting the domain ontology.

## 4. Ontology package model

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

A domain or backend extension additionally declares its extension type and parent/core dependency.

A profile declares the ontology packages it composes and the bindings required to make that composition executable or generatable.

## 5. Dependency model

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

Forbidden examples include:

```text
Core → MOOSE
Core → Plasma
Plasma Ontology → MOOSE Ontology
MOOSE Ontology → Plasma Ontology
```

The final two dependencies are forbidden by default because domain and backend ontologies are intended to remain orthogonal. Their integration belongs to a profile.

## 6. Identity and references

Every exported ontology term MUST have a stable namespaced identifier.

References between ontology packages MUST use semantic identifiers rather than source-file paths or implementation class names.

Conceptually:

```text
sim:Equation
plasma:ElectronContinuityEquation
moose:Kernel
```

The concrete URI and namespace syntax remains an implementation decision for the next language iteration.

## 7. Semantic graph model

An instantiated simulation ontology forms a graph:

```text
Entity ──Relation──> Entity
  │                    │
Property             Property
  │                    │
Value                Value
```

This graph model is the canonical conceptual representation regardless of authoring syntax.

It allows traversal such as:

```text
ElectronTransport
    represented_by
ElectronContinuityEquation
    closed_by
DriftDiffusionFlux
    parameterized_by
ElectronMobility
```

## 8. Candidate implementation stack

The following stack is provisional and subordinate to the language specification:

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

## 9. Proposed processing pipeline

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

## 10. Open questions for v0.1

The following questions must be resolved before the language is considered stable:

1. Exact distinction between Entity, Property, and value-bearing Parameter.
2. Whether `Result` is a core Entity type or a runtime artifact type.
3. Formal inheritance semantics.
4. Relation domain/range and cardinality semantics.
5. Required versus optional constraints.
6. Namespace and URI convention.
7. Ontology package version compatibility rules.
8. Extension conflict-resolution rules.
9. Profile binding semantics.
10. Unit and physical-dimension representation.
11. Whether actions/transformations should become a first-class construct in a later version.

## 11. v0.1 success criterion

The language is sufficiently defined when the same core language can independently express:

1. a small domain ontology such as thermal transport;
2. a backend ontology such as MOOSE;
3. a profile composing the two;
4. enough constraints to reject an invalid composition; and
5. enough mapping information to generate or construct a minimal backend simulation model.

The first reference validation should therefore use a small steady-state heat-conduction model before expanding to plasma and multiphysics domains.
