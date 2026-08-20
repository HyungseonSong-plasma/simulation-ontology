# Simulation Ontology Language

**Version:** 0.1  
**Status:** Frozen semantic baseline through ADR-0016; machine-readable consolidation in progress

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

SOL v0.1 has nine normative language constructs:

```text
Ontology
├── Namespace
├── Entity
├── Property
├── Relation
├── Constraint
├── Extension
├── Profile
└── Interface
```

`Value` and `ValueDefinition` have accepted semantic rules (ADR-0002), but their final language-level representation is not yet fixed. `PhysicalDimension` and `Unit` also have accepted semantic boundaries (ADR-0004/0005), while their final representation remains open. A Core `Action` language construct is deferred; the `PlanAction` of ADR-0011 is a MappingPlan/Adapter IR contract rather than a SOL Core ontology construct.

### 3.1 Namespace

A `Namespace` provides stable identity and prevents naming collisions across independently developed ontologies.

### 3.2 Entity

An `Entity` represents a semantic concept with independent identity in the ontology graph.

A concept SHOULD be modeled as an Entity when it needs independent semantic identity: it can be referenced by multiple other concepts, participate in relations of its own, carry non-trivial metadata, or evolve independently from the entity that uses it. A concept SHOULD NOT be promoted to an Entity merely because it has a value.

### 3.3 Property

A `Property` represents an intrinsic characteristic or value-bearing attribute of an Entity. It does not represent an independently meaningful semantic connection between two entities.

Use a Property when the information primarily answers what characteristic or value an Entity has. Use a Relation when the information primarily answers how one independently identifiable Entity is semantically connected to another.

### 3.4 Relation

A `Relation` is a first-class semantic link between independently identifiable ontology Entities.

A relation SHOULD specify its identifier, semantic description, endpoint constraints, optional inverse, and cardinality where applicable.

Use a Relation when both endpoints have independent semantic identity and the connection itself has domain meaning. A Relation SHOULD remain lightweight. If the connection itself needs substantial properties, provenance, version constraints, state, mappings, or relations of its own, the connection SHOULD be reified as an Entity.

Endpoint constraints may be expressed as explicit domain/range types or, where a single broad range would be ambiguous, as an authoritative allowed-pair matrix. ADR-0016 uses the latter for `includes_component`.

### 3.5 Constraint

A `Constraint` expresses a machine-verifiable requirement on an ontology or model graph, including required properties/relations, cardinality, allowed entity types, dimensional consistency, namespace rules, dependency restrictions, and graph invariants.

The SOL v0.1 Constraint architecture is governed by ADR-0007. Qualified Relation Cardinality is the accepted typed-target cardinality extension defined by ADR-0012.

### 3.6 Extension

An `Extension` allows an ontology to add specialized concepts while depending on a lower-level ontology. The primary extension classes are `DomainExtension` and `BackendExtension`.

Extensions MUST obey the inward dependency rule:

```text
specialized ontology -> simulation-ontology
```

The Core MUST NOT depend on an extension.

### 3.7 Profile

A `Profile` composes compatible ontology extensions for a concrete simulation environment.

```text
MOOSE Plasma Profile
    imports plasma-ontology
    imports moose-ontology
```

Backend realization bindings SHOULD normally live in profiles or backend mapping modules rather than polluting the domain ontology. The declarative Profile / executable BackendAdapter separation and MappingRule/MappingClaim contracts are governed by ADR-0010 and ADR-0011.

### 3.8 Interface

An `Interface` is a normative reusable semantic capability/shape contract under ADR-0008.

An Entity Type MAY have at most one direct taxonomic `is_a` parent and MAY implement zero or more Interfaces. An Interface MAY define Property requirements, Relation requirements, and Constraints, and MAY extend zero or more Interfaces. Interface contract composition is conjunctive under ADR-0007.

`Interface` is not a spatial/geometric entity. Under ADR-0014, the spatial concept is named `SpatialInterface`.

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

If multiple unrelated Entity types must satisfy the same reusable semantic shape or capability, use an `Interface` rather than artificial multiple taxonomic inheritance.

An Entity Type has at most one direct `is_a` parent in v0.1. It may implement multiple Interfaces. Tree formatting, YAML grouping, or a field named `children` SHALL NOT create taxonomic inheritance implicitly.

### Rule 6 — Value is not the semantic concept

`Value` is an evaluated typed datum and MUST NOT be identified with the semantic quantity or simulation concept whose value it represents.

### Rule 7 — ValueDefinition defines evaluation

`ValueDefinition` describes how a `Value` is obtained. Candidate mechanisms include literal, expression, function, tabular/interpolation, and external-data definitions. This subtype taxonomy is not yet normative.

### Rule 8 — Value shape and evaluation mechanism are orthogonal

The shape/type of an evaluated datum and the mechanism by which it is obtained MUST be modeled as independent dimensions.

### Rule 9 — Reference as Relation

A reference to an independently identifiable semantic concept SHALL normally be represented by a semantic `Relation`, not by a generic `ReferenceDefinition`.

### Rule 10 — Value dependency

When one concept obtains its value from another concept, the dependency SHALL be represented semantically as a value-dependency Relation. A `ValueDefinition` is required only when an evaluation or transformation mechanism itself must be represented.

### Rule 11 — PhysicalDimension as semantic contract

A physical semantic quantity SHOULD declare or imply a `PhysicalDimension`. Dimension-one quantities SHALL be represented explicitly as having physical dimension one rather than inferred from missing unit metadata.

Examples:

```text
Pressure             -> M L^-1 T^-2
ThermalConductivity  -> M L T^-3 Theta^-1
RelativePermittivity -> 1
PoissonRatio         -> 1
PlaneAngle           -> 1
```

### Rule 12 — Unit as value representation

A concrete `Value` MAY carry an explicit `Unit`. When a Unit is present, it SHALL be compatible with the `PhysicalDimension` required by the semantic quantity or evaluation context. Unit absence SHALL NOT by itself imply physical dimension one.

### Rule 13 — Semantic quantity identity is independent of dimension

Two semantic quantities MAY share the same `PhysicalDimension` while remaining distinct ontology concepts.

```text
Pressure      != Stress       != YoungsModulus
PoissonRatio  != PlaneAngle   != RelativePermittivity
```

### Rule 14 — Quantity-specific unit constraints

Physical-dimension compatibility is necessary but may not always be sufficient for semantic unit compatibility. A semantic quantity MAY impose additional constraints on units used to represent its Values. SOL does not introduce a separate `QuantityKind` layer for this purpose; the semantic quantity Entity already carries the relevant identity.

### Rule 15 — Explicit taxonomy

Taxonomic inheritance MUST be represented by explicit `is_a` semantics and SHALL NOT be inferred from diagram indentation, file hierarchy, declaration order, or generic `children` grouping. Composition and association are represented through Relations, not taxonomic inference.

### Rule 16 — Canonical Interface identity

The unqualified language identifier `Interface` denotes the ADR-0008 capability contract. The spatial entity concept is `SpatialInterface`. The two meanings SHALL NOT share one canonical identifier.

### Rule 17 — SimulationTask reifies model–Analysis application

`SimulationTask` is an Entity Type representing the identifiable application of exactly one `Analysis` to exactly one `SimulationModel`.

```text
Simulation --has_model--> exactly 1 SimulationModel
Simulation --has_task--> 1..* SimulationTask
SimulationTask --uses_model--> exactly 1 SimulationModel
SimulationTask --has_analysis--> exactly 1 Analysis
SimulationTask --produces--> 0..* Result
```

`Analysis` and `SolverConfiguration` do not inherit from SimulationTask. Analysis defines the computational question; SolverConfiguration defines how it is solved.

For every Simulation `S`, task `T`, and model `M`:

```text
S has_model M
AND S has_task T
=> T uses_model M
```

`has_model` and `has_task` are non-owning references.

### Rule 18 — `analyzed_by` is derived

`SimulationModel -> analyzed_by -> Analysis` is derived from SimulationTask bindings:

```text
M analyzed_by A
IFF
exists T:
  T uses_model M
  AND T has_analysis A
```

Task bindings are authoritative. A materialized `analyzed_by` edge without a supporting task is inconsistent derived data.

### Rule 19 — Direct structural membership uses `includes_component`

SOL v0.1 represents direct aggregate/model membership with the non-owning, non-transitive, unordered relation:

```text
includes_component
```

Its generic cardinality is `0..*`. Valid source-target combinations are governed by the authoritative allowed-pair matrix in ADR-0016. Derived domain/range summaries cannot broaden that matrix.

The relation does not replace semantic relations such as `represented_by`, `closed_by`, `parameterized_by`, `defined_on`, `discretized_by`, `applied_to`, or `solved_by`.

### Rule 20 — ADR-0016 endpoint matching and condition targeting

For `includes_component` and `applied_to`, endpoint matching uses canonical semantic identity and subtype closure:

```text
matches(x,T)
IFF
exists consistent canonical type X of x:
  X = T OR X <: T
```

Type inconsistency fails before endpoint matching; backend inheritance and declaration order are irrelevant.

`applied_to` has source family:

```text
BoundaryCondition | InitialCondition | Source | Load
```

and target family:

```text
Field | Equation | Scope
```

Each conforming source requires `1..*` `applied_to` targets. `ConditionModel` itself is an aggregate and is not a direct `applied_to` source.

The value rules are accepted in ADR-0002, reference/dependency rules in ADR-0003, unit/dimension rules in ADR-0004/0005, Constraint composition in ADR-0007, Interface/inheritance rules in ADR-0008, identity/package rules in ADR-0009, Profile/backend mapping in ADR-0010/0011, QRC in ADR-0012, naming/taxonomy disambiguation in ADR-0014, Simulation/Model/Task relations in ADR-0015, and direct component/condition-target semantics in ADR-0016.

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

Language-level Interface definitions and implementation mappings must also be representable in packages that use them; the exact canonical serialization location is part of ongoing machine-readable consolidation.

## 6. Dependency model

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

Canonical identity, namespace, alias/rename/deprecation, package, and versioning rules are governed by ADR-0009. Machine-readable serialization SHALL preserve those semantics without using source-file location as identity.

## 8. Semantic graph model

### 8.1 Task/model graph

```text
Simulation
  ├── has_model -> SimulationModel
  └── has_task  -> SimulationTask
                        ├── uses_model -> SimulationModel
                        ├── has_analysis -> Analysis
                        │                    └── solved_by -> SolverConfiguration
                        └── produces -> Result
                                         └── observed_by -> ObservationModel
```

`analyzed_by` is the derived Model-to-Analysis traversal implied by the task bindings.

### 8.2 Direct model-component graph

```text
SimulationModel --includes_component--> MathematicalModel
MathematicalModel --includes_component--> Field
ConditionModel --includes_component--> BoundaryCondition
BoundaryCondition --applied_to--> Field / Scope
```

`includes_component` is direct-only. The first two edges do not imply `SimulationModel --includes_component--> Field`.

### 8.3 Value/dimension graph

```text
SemanticQuantity
      │
      ├── requires_dimension -> PhysicalDimension
      ├── unit_constraint ---> Unit / Constraint   [optional]
      └── has_value_definition
                    │
                    ▼
             ValueDefinition
                    │
               evaluates_to
                    ▼
                  Value
                    │
                    └── expressed_in -> Unit   [optional]
                                             │
                                             └── has_dimension
                                                      │
                                                      ▼
                                              PhysicalDimension
```

The final graph-level representation of `Value`, `ValueDefinition`, `PhysicalDimension`, and `Unit` remains an implementation/language-representation question; their semantic distinctions are normative.

`Result` is a Core semantic Entity Type; no Result subtype taxonomy is required in v0.1.

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
MappingPlan / BackendAdapter layer
```

`MappingPlan` and BackendAdapter runtime constructs are not Core ontology primitives merely because they participate in generation/execution.

## 11. Consolidation state

### Accepted semantics not yet fully transcribed

1. `Value` / `ValueDefinition` representation boundaries from ADR-0002/0003.
2. `PhysicalDimension` / `Unit` final serialization from ADR-0004/0005.
3. Complete six-family Constraint schema/composition from ADR-0007, including QRC from ADR-0012.
4. Interface definitions, Interface extension, `implements`, and implementation mappings from ADR-0008.
5. Full identity/namespace/package/version metadata from ADR-0009.
6. Profile/MappingRule/MappingClaim authoring schemas consistent with ADR-0010/0011.
7. Canonical structural/semantic schema enforcement for ADR-0015 relation cardinalities and derived `analyzed_by` consistency.
8. Canonical schema enforcement for ADR-0016 allowed-pair and subtype-aware relation endpoint semantics.

These are transcription/representation tasks unless implementation evidence exposes a new normative contradiction.

### Open language/schema decisions

1. Final language-level representation of `Value`, `ValueDefinition`, `PhysicalDimension`, and `Unit` within the canonical graph/authoring schema.
2. Normative subtype taxonomy of `ValueDefinition`, if any.
3. Provenance/data-source representation for ValueDefinitions beyond already accepted semantic boundaries.
4. Complete Core relation cardinalities and required/optional relation matrix beyond ADR-0015/0016.
5. Final canonical serialization layout for Interface definitions and implementation mappings.
6. SolverConfiguration cardinality/default/task-specific override semantics.
7. Whether additional generic actions/transformations become a first-class Core construct in a later SOL version; v0.1 does not require one.
8. Multi-model/co-simulation semantics if later justified by evidence.

Backend installation, licensing, production Adapter implementation, and backend execution V&V are not SOL language-design blockers.

## 12. v0.1 success criterion

The v0.1 semantic architecture is frozen through ADR-0016. Language/schema consolidation succeeds when accepted contracts can be represented and independently validated without adding semantics not present in the ADR baseline.

Small thermal and plasma/QRC models remain sufficient design-stage structural stress cases. Production-quality Adapter execution belongs to separate Adapter projects and may reopen the language/architecture only if it produces a genuine semantic counterexample.
