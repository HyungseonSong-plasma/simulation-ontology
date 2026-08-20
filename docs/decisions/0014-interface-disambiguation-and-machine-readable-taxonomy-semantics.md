# ADR-0014: Interface Disambiguation and Machine-Readable Taxonomy Semantics

**Status:** Accepted  
**Date:** 2026-08-20  
**Amends:** ADR-0013 freeze baseline for the narrow scope below

## Context

Post-freeze machine-readable consolidation exposed two independent-reader ambiguities in the existing baseline:

1. `Interface` was used both for a spatial/geometric concept under `SpatialModel` and for the normative capability-contract construct accepted by ADR-0008.
2. `ontology/core/entities.yaml` used `children` without defining whether those edges were taxonomic `is_a` inheritance or structural/grouping relationships.

The same consolidation pass also found that `Result` is referenced by accepted architecture relations but is not declared in the machine-readable entity registry, and that one architecture diagram visually places `produces` below `SolverConfiguration` despite the explicit relation contract declaring `SimulationTask -> produces -> Result`.

Independent Validation confirmed these as a narrow architecture naming/language-semantics gap plus transcription defects. No evidence requires reopening ADR-0010, ADR-0011, or ADR-0012.

## Decision

### 1. Reserve `Interface` for capability contracts

The unqualified SOL language term:

```text
Interface
```

SHALL denote the abstract reusable capability/shape contract defined by ADR-0008.

An Interface MAY define Property requirements, Relation requirements, and Constraints; Entity Types MAY implement zero or more Interfaces; Interfaces MAY extend zero or more Interfaces as specified by ADR-0008.

`Interface` is not a spatial/geometric entity type.

### 2. Rename the spatial concept

The spatial/geometric concept previously shown as:

```text
SpatialModel.Interface
```

is renamed:

```text
SpatialInterface
```

Therefore:

```text
SpatialModel
├── Geometry
├── Domain
├── Boundary
├── SpatialInterface
└── Scope
```

This is a naming disambiguation. It does not change the semantic meaning of an interface between spatial regions/domains.

No v0.1 alias from `Interface` to `SpatialInterface` is introduced because the prior name is semantically ambiguous. Migration aliases may be added later only with explicit provenance and versioning evidence.

### 3. `children` is non-normative shorthand

The field/name `children` SHALL NOT define taxonomic inheritance in the canonical machine-readable SOL v0.1 contract.

Architecture diagrams may continue to use tree formatting for readability, but diagram indentation or grouping SHALL NOT imply `is_a` unless an explicit semantic rule says so.

Canonical taxonomic inheritance SHALL be represented only by:

```text
is_a -> 0..1 EntityType
```

as defined by ADR-0008.

### 4. Composition must be explicit

Composition/grouping and taxonomic inheritance are distinct.

If a relationship such as:

```text
Simulation
├── SimulationModel
└── SimulationTask
```

is intended as structural composition rather than subtype inheritance, the canonical machine-readable model SHALL use explicit Relation semantics once those relation names/cardinalities are accepted.

Until that relation contract is accepted, the machine-readable core SHALL NOT invent an `is_a` edge from diagram grouping.

### 5. Declare `Result` as an existing Core type

`Result` is confirmed as a Core semantic Entity Type because accepted architecture already uses:

```text
SimulationTask -> produces -> Result
Result -> observed_by -> ObservationModel
```

`Result` has no required `is_a` parent and no v0.1 subtype taxonomy in this ADR.

This is referential completion of an existing contract, not introduction of a new result ontology.

### 6. Confirm `produces` domain

The v0.1 relation remains:

```text
SimulationTask -> produces -> Result
```

The visual placement of `produces` under `SolverConfiguration` in one architecture relationship diagram is non-normative documentation drift and SHALL be corrected.

No narrower `SolverConfiguration -> produces -> Result` rule is introduced.

### 7. Interface status is normative

`Interface` is no longer provisional in the SOL v0.1 language baseline. Its accepted semantics are those of ADR-0008 as amended only by the naming disambiguation in this ADR.

Executable method/action signatures remain deferred.

### 8. Machine-readable status discipline

Machine-readable artifacts SHALL distinguish:

```text
accepted_semantics_not_yet_transcribed
```

from:

```text
open_language_schema_decisions
```

A file SHALL NOT label already accepted identity, unit/dimension, Interface, or QRC semantics as generically pending.

The machine-readable package may remain `consolidating` until open relation composition/cardinality decisions are transcribed and independently validated.

## Consequences

### Positive

- every exported Core term has one canonical meaning;
- capability Interface and spatial interface semantics cannot collide;
- independent tools cannot infer inheritance from tree formatting;
- `Result` relation endpoints are referentially resolvable;
- frozen backend mapping/QRC contracts remain untouched.

### Costs

- current documentation and YAML artifacts using spatial `Interface` must be updated to `SpatialInterface`;
- machine-readable entity representation must move away from ambiguous `children` shorthand;
- explicit composition relations may require a later focused language/schema decision.

## Deferred

This ADR does not define:

- complete Simulation composition relation names/cardinalities;
- complete Core relation cardinalities;
- Result subtypes;
- migration aliases for the ambiguous old spatial `Interface` name;
- backend-native interface/selection objects;
- Adapter behavior.

## Validation evidence

- `docs/research/sol-v0.1-machine-readable-core-consolidation-gap-analysis-v0.1.md`
- `docs/validation/sol-v0.1-machine-readable-core-consolidation-independent-review-v0.1.md`
- ADR-0008 and ADR-0009

## Freeze amendment

ADR-0013 remains in force. This ADR amends only the canonical naming and machine-readable interpretation rules above. All unaffected frozen SOL v0.1 architecture decisions remain frozen.

## Decision summary

SOL v0.1 reserves `Interface` for reusable capability contracts, renames the spatial concept to `SpatialInterface`, forbids `children` from carrying normative inheritance meaning, requires explicit `is_a` for taxonomy, declares the already-referenced `Result` Core type, and confirms `SimulationTask -> produces -> Result`.
