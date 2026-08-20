# Simulation Ontology Language

**Version:** 0.1  
**Status:** Frozen Core semantic baseline through ADR-0017; focused language/schema/package/model-snapshot representation accepted through ADR-0029.

## 1. Purpose

The Simulation Ontology Language (SOL) defines the minimum semantic constructs required to build, extend, compose, validate, and map simulation ontologies independently of a particular simulation backend.

Serialization and implementation technologies are not themselves the language. YAML/JSON, JSON Schema, Python, TypeScript, JSON-LD/RDF, SHACL, and future tooling may implement accepted SOL contracts without redefining their semantics.

## 2. Minimum language constructs

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

In addition, SOL has accepted typed-value/model contracts for `Value`, `UnitReference`, `DimensionVector`, `InlineValueDefinition`, canonical ontology packages, and closed `ResolvedModelSnapshot` reference inputs. These are not additional Core Entity Types merely because they are machine-readable structures.

### Namespace

A Namespace provides authoring/API naming and collision control. Namespace names are distinct from package identity and from canonical persistent semantic identity under ADR-0009/0028.

### Entity

An Entity represents a semantic concept or model instance with independent identity. Promote a concept to Entity only when independent reference, relation participation, provenance/state, reuse, or independent evolution justifies identity.

### Property

A Property represents an intrinsic characteristic/value-bearing attribute of an Entity. A property assignment in a resolved model snapshot references a canonical PropertyDefinition and carries a ValueDefinition payload.

### Relation

A Relation is a first-class semantic link between independently identifiable Entities. Relation definitions use canonical endpoint contracts; direct domain/range and authoritative allowed-pair forms are both supported. Cardinality remains Constraint authority even when a RelationDefinition carries a matching projection.

### Constraint

A Constraint expresses a machine-verifiable requirement. SOL v0.1 defines six minimum families:

```text
Cardinality / QRC
Type
Value
Dimension
Compatibility
Conditional
```

Constraint composition is governed by ADR-0007 and focused serialization/evaluation by ADR-0018..0024.

### Extension

An Extension specializes the Core without causing Core to depend outward on domain/backend concepts. DomainExtension and BackendExtension are the primary axes.

### Profile

A Profile composes compatible domain and backend extensions and declarative realization bindings. Executable lowering remains a MappingPlan/BackendAdapter responsibility under ADR-0010/0011.

### Interface

An Interface is an abstract reusable capability/shape contract. It may declare Property requirements, Relation requirements, targeted Constraint applications, and extension of other Interfaces. Concrete Entity Types implement Interfaces through explicit canonical mappings. `Interface` is distinct from the spatial Entity Type `SpatialInterface`.

## 3. Normative semantic rules

### Rule 1 — Explicit identity layers

SOL distinguishes display labels, authoring/API qualified names, and canonical persistent identity. Canonical identity does not depend on package version, repository path, file location, backend handle, or semantic category spelling.

### Rule 2 — Explicit taxonomy

Entity Type inheritance is represented only by explicit `is_a`. An Entity Type has at most one direct `is_a` parent in v0.1. Diagram indentation, file hierarchy, declaration order, or generic grouping never creates inheritance.

### Rule 3 — Interface composition for orthogonal capabilities

Use Interface composition instead of artificial multiple Entity inheritance. Interfaces may extend multiple Interfaces. Effective implementation mappings must resolve deterministically and inherited Entity-type guarantees may not silently disappear.

### Rule 4 — Model/task separation

`SimulationModel` defines what is modeled. `Analysis` defines the computational question. `SimulationTask` reifies application of exactly one Analysis to exactly one SimulationModel. `SolverConfiguration` defines how an Analysis is solved.

```text
Simulation --has_model--> exactly 1 SimulationModel
Simulation --has_task--> 1..* SimulationTask
SimulationTask --uses_model--> exactly 1 SimulationModel
SimulationTask --has_analysis--> exactly 1 Analysis
SimulationTask --produces--> 0..* Result
```

`Analysis` and `SolverConfiguration` do not inherit from SimulationTask.

### Rule 5 — Task binding is authoritative

```text
M analyzed_by A
IFF
exists T:
  T uses_model M
  AND T has_analysis A
```

`analyzed_by` is derived data; task bindings are the source of truth.

### Rule 6 — Direct component membership

`includes_component` is direct-only, non-owning, non-transitive, unordered structural membership. Valid source/target combinations are the ADR-0016 allowed-pair matrix and endpoint matching is canonical equal-or-subtype matching.

### Rule 7 — Semantic relations remain distinct from membership

`includes_component` does not replace `represented_by`, `closed_by`, `parameterized_by`, `defined_on`, `discretized_by`, `applied_to`, `solved_by`, `produces`, `observed_by`, or value-dependency relations.

### Rule 8 — Condition targeting

`BoundaryCondition | InitialCondition | Source | Load` are direct `applied_to` sources and each requires `1..*` targets from `Field | Equation | Scope` after subtype-aware matching. `ConditionModel` is an aggregate and is not itself a direct condition target source.

### Rule 9 — Cardinality authority

Cardinality is a Constraint. Relation-side cardinality is a projection/cache that must match the authoritative canonical Constraint and does not independently intersect with it.

Generic Core source interval `0..*` is accepted for:

```text
represented_by
closed_by
parameterized_by
defined_on
discretized_by
solved_by
observed_by
```

Domain/Interface/Profile constraints may narrow these intervals conjunctively.

### Rule 10 — QRC

Qualified Relation Cardinality filters targets by canonical target Entity Type/subtype and counts distinct target identity over a complete immutable/closed snapshot. Intrinsically invalid finite bounds fail before count satisfaction is evaluated.

### Rule 11 — Validation-state separation

For the common semantic state slice:

```text
FAIL > INDETERMINATE > PASS
```

Operational resource interruption, backend installation/license absence, and other invocation preconditions are separate axes and do not silently become semantic FAIL or PASS.

### Rule 12 — Value is not the semantic concept

`Value` is an evaluated typed datum and is not the identity of the semantic quantity/entity whose value it represents.

### Rule 13 — Canonical DimensionVector

`DimensionVector` is a typed seven-axis physical-dimension payload. Dimension-one is explicit as the all-zero vector. Missing UnitReference does not imply dimension-one.

### Rule 14 — UnitReference and metrology boundary

`UnitReference` identifies a unit in an external metrology namespace/registry. When unit/dimension comparison requires metrology resolution and that resolution is unavailable, semantic evaluation is `INDETERMINATE`, not an invented default.

### Rule 15 — Value representation

`Value` supports accepted scalar/vector/tensor shapes and scalar kinds. Normalized numeric values use exact-decimal representation rather than host binary floating-point identity.

### Rule 16 — ValueDefinition boundary

A dependency-free local value definition may remain `InlineValueDefinition`. A definition requiring independent identity, semantic dependency, reuse, or provenance is reified as a `ValueDefinition` Entity and may participate in `has_value_definition` / `depends_on` relations.

### Rule 17 — Nonliteral format providers

Core does not own a universal expression/function/tabular DSL. A nonliteral InlineValueDefinition uses an explicitly identified format provider that returns normalized local payload plus resolved semantic dependencies. Any semantic dependency forces reification; raw payload text is not scanned heuristically by Core.

### Rule 18 — Property assignment identifies semantic value role

Resolved model instances do not carry unlabeled generic value payloads. A property assignment references a canonical PropertyDefinition, which identifies what the ValueDefinition means for that entity.

### Rule 19 — Package/namespace/canonical identity separation

A distribution package, semantic namespace, and canonical semantic identity are distinct concepts. A normalized package has exact resolved dependencies, explicit namespace export tables, canonical resource IDs, and closed resource collections.

Within one resolved immutable environment, one canonical ID has one active normalized resource definition. Across package versions, the same semantic concept may retain its canonical ID while definition content evolves under version policy.

### Rule 20 — One active namespace provider in v0.1

A resolved environment permits at most one active provider for a visible semantic namespace. Namespace federation/augmentation is deferred.

### Rule 21 — Canonical normalized references

After normalization, schema-resource references use canonical semantic IDs. Model-instance references use resolved model-instance IDs. Backend-local handles remain backend/Profile/Adapter concerns.

### Rule 22 — Resolved model snapshot

ADR-0029 defines the focused closed reference-model boundary:

```text
ResolvedModelSnapshot
├── snapshot_state = closed
├── exact ontology_environment
├── entities[]
│   ├── model-instance id
│   ├── canonical EntityType id
│   └── properties[]
│       ├── canonical PropertyDefinition id
│       └── InlineValueDefinition
└── relations[]
    ├── canonical RelationDefinition id
    ├── source model-instance id
    └── target model-instance id
```

The snapshot is sufficient for design-stage reference validation but is not a claim that every future application/model-document feature is standardized in v0.1.

## 4. Package model

A normalized ontology package contains:

```text
package identity/version
semantic namespace export table(s)
exact resolved dependencies
entity_types
properties
relations
constraint_definitions
interfaces
interface_implementations
```

Package/resource schemas are governed by ADR-0028. Model-instance ValueDefinition entities belong to model documents/snapshots rather than ontology-package resource collections.

## 5. Dependency model

```text
Application
    ↓
Profile
    ↓
Domain / Backend Ontology
    ↓
Core Ontology
```

Core-to-backend, Core-to-domain, and direct domain-to-backend dependency are forbidden by default. Domain/backend composition belongs to a Profile.

## 6. Accepted machine-readable coverage

Focused schema/semantic-validator coverage exists for:

- all six Constraint families and Predicate evaluation;
- Interface definitions/implementations and inherited conformance;
- DimensionVector, UnitReference, exact-decimal Value, and InlineValueDefinition;
- EntityTypeDefinition, PropertyDefinition, RelationDefinition, ConstraintDefinition;
- normalized ontology packages and exact resolved environments;
- closed `ResolvedModelSnapshot` reference inputs;
- Thermal Dimension/metrology reference validation;
- Plasma/QRC closed-snapshot reference validation.

Schemas implement accepted semantics; they are not independent semantic authorities.

## 7. Accepted reference gates

### Thermal — PASS

The reference fixture exercises model/task semantics, direct component membership, condition targeting, Property assignments, Values/Units, Interface-targeted Dimension Constraints, and metrology resolution states.

### Plasma/QRC — PASS

The reference fixture exercises Reaction/Species subtype semantics, explicit reaction relations, Interface relation mapping, independent qualified cardinality obligations, distinct identity counting, subtype matching, and counterexamples for missing/extra/wrong-type/duplicate/unresolved/open-snapshot states.

## 8. Explicitly deferred / outside design-stage closure

The following are not unresolved v0.1 Core architecture questions:

- production BackendAdapter implementation and executable backend V&V;
- backend installation and commercial license availability;
- namespace federation/augmentation;
- multi-model/co-simulation semantics;
- richer future PropertyDefinition/Result/domain sub-taxonomies;
- complete general application/model-document syntax beyond the focused ADR-0029 snapshot;
- production Profile/backend package authoring beyond accepted mapping contracts.

Any future work that changes an accepted semantic boundary requires a new focused decision/validation cycle rather than silent mutation of these rules.
