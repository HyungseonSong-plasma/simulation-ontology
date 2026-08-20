# SOL v0.1 Machine-Readable Core Consolidation Gap Analysis v0.1

**Role:** Research  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`  
**Scope:** Frozen ADR → machine-readable core transcription/consolidation only

## 1. Objective

Align `ontology/core/*.yaml`, `docs/architecture.md`, and `docs/ontology-language.md` with the design-stage baseline frozen by ADR-0013 without reopening resolved backend mapping, QRC, or Adapter contracts.

A freeze may be reopened narrowly only when consolidation exposes a normative contradiction or missing semantic contract. Implementation convenience is not evidence.

## 2. Sources

Primary repository sources:

- ADR-0007 — Constraint Architecture and Composition
- ADR-0008 — Inheritance and Interface Composition
- ADR-0009 — Identity, Namespace, Package, and Versioning
- ADR-0011 — MappingPlan Determinism and Executable Backend Target
- ADR-0012 — Qualified Relation Cardinality
- ADR-0013 — SOL v0.1 Design-Stage Architecture Freeze
- `docs/architecture.md`
- `docs/ontology-language.md`
- `ontology/core/entities.yaml`
- `ontology/core/relations.yaml`
- `ontology/core/constraints.yaml`

Conceptual comparison evidence:

- Palantir Ontology Interfaces documentation: https://www.palantir.com/docs/foundry/interfaces/interface-overview

Palantir is used only as conceptual evidence for separating abstract capability interfaces from concrete domain/spatial object types; it is not normative for SOL.

## 3. Findings

### MC-01 — `Interface` has two incompatible canonical meanings

`docs/architecture.md` and `ontology/core/entities.yaml` currently place:

```text
SpatialModel
├── Geometry
├── Domain
├── Boundary
├── Interface
└── Scope
```

where `Interface` means a spatial/geometric interface.

ADR-0008 separately accepts `Interface` as a language-level reusable capability contract:

```text
EntityType
  └── implements -> 0..* Interface
```

with Interface extension and conjunctive contract composition.

These cannot share one canonical SOL identifier without making an independent parser/validator unable to determine whether `Interface` denotes a spatial entity or an abstract capability contract.

**Classification:** Architecture naming defect / normative identity collision.

**Minimal proposal:**

- retain `Interface` for the ADR-0008 capability-contract construct;
- rename the spatial concept to `SpatialInterface`;
- update the frozen architecture diagram and machine-readable core accordingly;
- do not introduce aliases in v0.1 unless migration evidence later requires them.

Rationale: ADR-0008 gives the capability construct formal language semantics. `SpatialInterface` preserves the spatial meaning while eliminating collision. Palantir's Ontology similarly treats Interface as an abstract shape/capability contract implemented by concrete object types, supporting this lexical separation as a design pattern rather than a backend-specific choice.

### MC-02 — `children` conflates diagram grouping with taxonomic inheritance

`ontology/core/entities.yaml` uses nested `children` throughout. After ADR-0008, taxonomic inheritance has a precise contract:

```text
EntityType.is_a -> 0..1 EntityType
EntityType.implements -> 0..* Interface
```

The architecture tree diagrams do not consistently prove that every displayed parent/child edge is an `is_a` edge. In particular:

```text
Simulation
├── SimulationModel
└── SimulationTask
```

is described semantically as model/task separation (components of a simulation), not explicitly as `SimulationModel is_a Simulation` and `SimulationTask is_a Simulation`.

A machine-readable `children` field therefore leaves two conforming readers free to interpret the same edge as composition/grouping or inheritance.

**Classification:** Language/schema contract gap exposed by transcription.

**Minimal proposal:**

1. deprecate `children` as normative machine semantics;
2. represent taxonomic inheritance only through explicit `is_a` fields governed by ADR-0008;
3. represent composition/association only through explicit Relation instances/types;
4. do not invent missing composition relations during transcription—where the architecture only has diagrammatic grouping, record the relation as `TBD` and resolve it through one focused language ADR before declaring the machine-readable core canonical.

This avoids silently converting diagrams into inheritance.

### MC-03 — `Result` is referenced normatively but not declared in the entity registry

`docs/architecture.md` and `ontology/core/relations.yaml` use:

```text
SimulationTask -> produces -> Result
Result -> observed_by -> ObservationModel
```

but `ontology/core/entities.yaml` does not declare `Result`.

Independent schema generation cannot resolve the range of `produces` or domain of `observed_by` from the current machine-readable core.

**Classification:** Architecture completeness / machine-readable declaration gap.

**Minimal proposal:**

- declare `Result` as a Core semantic Entity Type with no inferred `is_a` parent in v0.1;
- preserve the existing `produces` and `observed_by` semantics;
- defer subtype taxonomy such as FieldResult, ScalarResult, DatasetResult until evidence exists.

This adds no new meaning beyond a construct already present in the architecture graph; it only makes the existing contract referentially complete.

### MC-04 — `docs/ontology-language.md` still marks `Interface` provisional

The language document says `Interface [provisional]`, while ADR-0008 has already accepted Interface composition for SOL v0.1.

**Classification:** Documentation transcription drift.

**Proposal:** update the language document to make `Interface` normative and cite ADR-0008. Keep executable action/method requirements deferred as ADR-0008 specifies.

### MC-05 — machine-readable core status and pending list are stale

All three `ontology/core/*.yaml` files still say `status: draft`. `constraints.yaml` still lists pending work for identifiers/namespaces and unit/dimension constraints even though the corresponding semantic boundaries were accepted in ADR-0004/0005/0009.

Some pending items remain genuinely open at machine-language level, especially relation cardinalities and explicit composition relations.

**Classification:** Documentation/schema transcription drift.

**Proposal:** replace undifferentiated `pending` with two explicit categories:

```text
accepted_semantics_not_yet_transcribed
open_language/schema_decisions
```

Do not mark the machine-readable core `stable` until MC-02 composition/inheritance semantics and referential completeness are validated.

### MC-06 — `produces` placement is inconsistent in architecture prose/graph

`relations.yaml` declares:

```text
produces.domain = SimulationTask
```

while the architecture relationship graph visually places `produces` below `SolverConfiguration`. The top-level architecture text instead shows `SimulationTask -> produces -> Result`.

A reader can therefore infer either:

```text
SimulationTask produces Result
```

or:

```text
SolverConfiguration produces Result
```

**Classification:** Architecture relation-domain ambiguity.

**Research proposal:** keep the broader semantic contract:

```text
SimulationTask -> produces -> Result
```

because `Analysis` and `SolverConfiguration` are task-layer constructs and backend execution details must not determine result ownership. Treat the graph indentation under SolverConfiguration as documentation drift, not a new narrower domain rule.

This must be independently validated before transcription.

## 4. Proposed focused remediation

Do not reopen ADR-0010/0011/0012. Limit remediation to:

1. **Naming disambiguation:** `SpatialModel.Interface` → `SpatialInterface`; reserve `Interface` for ADR-0008 capability contracts.
2. **Inheritance representation:** forbid normative `children`; use explicit `is_a` only for proven taxonomic inheritance.
3. **Result completeness:** declare `Result` as an existing Core concept with no additional subtype design.
4. **Relation clarification:** retain `SimulationTask -> produces -> Result`; correct misleading architecture graph placement.
5. **Language drift:** promote Interface from provisional to normative in `docs/ontology-language.md`.
6. **Status drift:** distinguish accepted-but-untranscribed semantics from genuinely open language/schema decisions.

## 5. What is intentionally not decided here

This proposal does not define:

- complete Core relation cardinalities;
- new top-level simulation composition relations;
- serialization syntax for every Entity Type;
- Result subtype taxonomy;
- Adapter runtime schemas;
- backend-specific scope/interface objects;
- migration/alias policy for `Interface` → `SpatialInterface`.

Those require separate evidence if needed.

## 6. Evaluation contract for Validation

Validation should independently test:

1. whether MC-01 is a real canonical-identity collision;
2. whether `SpatialInterface` is the minimum non-breaking disambiguation;
3. whether `children` can be interpreted deterministically under ADR-0008 without an explicit rule;
4. whether adding bare `Result` merely completes an existing contract or introduces new architecture;
5. whether `SimulationTask -> produces -> Result` is better supported by current normative text than `SolverConfiguration -> produces -> Result`;
6. whether any proposed fix imports backend-native semantics or expands frozen scope unnecessarily.

Verdict domain:

```text
Accept / Revise / Reject / Blocked
```

## 7. Research verdict

**Ready for focused independent validation.**

The design freeze remains valid for all unaffected ADRs. Only the naming/referential/language gaps identified above are candidates for a narrowly scoped freeze amendment.
