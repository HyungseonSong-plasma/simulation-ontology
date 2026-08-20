# ADR-0010: Profile and Backend Mapping Contract

**Status:** Accepted for SOL v0.1 architecture  
**Date:** 2026-08-20

## Context

SOL needs to project one backend-independent semantic model into heterogeneous simulation systems such as MOOSE, COMSOL, and Ansys without forcing Core concepts to mirror backend object models. Cross-backend thermal and plasma stress tests showed that mappings are often structural transformations rather than 1:1 type correspondences, and that backend capability, creation order, runtime API calls, local identifiers, and representability must remain separate from semantic mapping intent.

Relevant evidence:

- [Thermal profile/backend mapping reference](../research/profile-backend-mapping-thermal-reference-v0.1.md)
- [Profile vs BackendAdapter boundary validation](../research/profile-vs-backend-adapter-boundary-validation-v0.1.md)
- [MappingRule minimal schema stress test](../research/mapping-rule-minimal-schema-stress-test-v0.1.md)
- [Constraint architecture and composition](0007-constraint-architecture-and-composition.md)
- [Identity, namespace, package, and versioning](0009-identity-namespace-package-and-versioning.md)
- [Semantic preservation across backends](0006-semantic-preservation-across-backends.md)

## Decision

### 1. Profile and BackendAdapter are separate layers

SOL v0.1 SHALL separate declarative mapping policy from executable backend integration.

```text
Profile
  = declarative mapping specification

BackendAdapter
  = executable backend integration runtime
```

A Profile MAY declare:

- its own identity and version;
- required ontology packages;
- target backend adapter binding;
- backend release compatibility requirements;
- MappingRules;
- profile-specific constraint refinements;
- generation defaults/policies.

A BackendAdapter is responsible for:

- backend version/release inspection;
- capability discovery;
- backend-local identifier allocation;
- serialization and/or backend API invocation;
- scoping/selection application;
- artifact creation/update;
- runtime diagnostics;
- realization reporting.

### 2. MappingPlan is the explicit intermediate representation

Mapping SHALL proceed through an inspectable intermediate `MappingPlan`.

```text
SOL Model
   -> semantic validation
Profile
   -> MappingRule evaluation/composition
MappingPlan
   -> representability/capability preflight
BackendAdapter
   -> execution
Backend Artifacts
   -> Realization Report
```

The MappingPlan SHOULD support dry-run validation and explainability before backend execution.

### 3. Minimal MappingRule schema

`MappingRule` SHALL use the following five top-level semantic fields in v0.1:

```text
MappingRule
├── source
├── applicability
├── realization
├── bindings
└── capabilities
```

Their roles are:

- `source`: the SOL semantic construct(s) to which the rule applies;
- `applicability`: the predicate under which the rule is active;
- `realization`: the backend realization pattern to produce/attach/configure;
- `bindings`: property/relation/value mappings between SOL and backend constructs;
- `capabilities`: adapter/backend capabilities required by the rule.

### 4. Realization dependencies remain inside realization

Generation ordering and dependencies SHALL NOT introduce a sixth top-level MappingRule field.

`realization` MAY declare dependency/order metadata such as:

```text
create / attach / configure
produces
requires / depends_on
```

MappingPlan construction SHALL derive an execution DAG from these dependencies and reject cycles or unresolved prerequisites before backend execution.

### 5. Mapping cardinality is unrestricted by 1:1 assumptions

A MappingRule MAY realize mappings with cardinalities such as:

```text
1 SOL construct -> 1 backend construct
1 SOL construct -> N backend constructs
N SOL constructs -> 1 backend context
N SOL constructs -> N backend constructs
```

Structural transformation is expected and does not imply semantic loss.

### 6. Representability classification is evaluated, not statically declared

The effective mapping result SHALL be classified as one of:

```text
exact
transformed
lossy
unsupported
```

These classifications are MappingPlan/realization outcomes, not fixed intrinsic labels on MappingRule.

The final classification depends on:

- Profile semantic intent;
- the concrete SOL model/ValueDefinition;
- Adapter capabilities;
- backend release/runtime conditions.

`transformed` is a successful lossless mapping whose backend structure differs from the SOL semantic structure.

A `lossy` mapping MUST surface the lost semantic information explicitly. `unsupported` means the selected backend/profile cannot realize the construct faithfully enough to proceed under the current policy.

### 7. Applicable rules compose by default

For a given SOL source construct, all MappingRules whose `source` and `applicability` match SHALL be collected.

Compatible rules SHALL compose. Declaration order SHALL NOT create semantic priority, and numeric `priority` is not part of Core v0.1.

```text
all applicable rules
    -> collect claims
    -> compose compatible claims
    -> detect conflicts
    -> build MappingPlan
```

### 8. Generic MappingClaim contract

Core v0.1 SHALL provide a generic `MappingClaim` construct with exactly four required semantic fields:

```text
MappingClaim {
    obligation
    source
    realization
    provenance
}
```

- `obligation`: a Profile/Adapter-defined key naming the mapping responsibility being claimed;
- `source`: the SOL construct/model instance to which the claim applies;
- `realization`: the backend realization being claimed;
- `provenance`: the MappingRule/Profile origin of the claim.

The Core SHALL understand the generic conflict semantics of obligations, but SHALL NOT define backend-specific obligation vocabularies.

Example adapter/profile vocabulary MAY include concepts such as:

```text
primary-field-realization
transport-term
coefficient-binding
scope-binding
```

but such vocabulary remains Profile/Adapter-owned.

### 9. MappingClaim conflict rule

The canonical conflict invariant is:

```text
same source
+ same obligation
+ incompatible realization
=> MappingConflict
```

Claims with different obligations MAY compose when their realizations are compatible.

The Core SHALL NOT silently choose one claim by order or priority.

### 10. MappingClaim remains minimal

The following SHALL NOT be required MappingClaim fields in v0.1:

- numeric priority;
- representability status;
- capabilities;
- execution order;
- diagnostics.

Those belong respectively to no Core priority mechanism, MappingPlan evaluation, MappingRule/Adapter capability handling, realization dependency DAGs, and reporting/validation layers.

### 11. Capability responsibility is shared but distinct

Profile and Adapter SHALL cooperate without collapsing their responsibilities:

```text
Profile
  declares expected/required capabilities and semantic intent

BackendAdapter
  inspects actual backend/runtime capabilities

MappingPlan
  evaluates effective representability
```

Backend-specific capability taxonomies and compatibility scopes remain adapter-defined under ADR-0009.

### 12. Semantic preservation remains authoritative

Backend realization SHALL NOT mutate SOL semantic truth to fit a backend.

If the backend cannot faithfully represent a valid SOL model, the outcome SHALL be `lossy` or `unsupported` rather than silently weakening or rewriting the semantic model, consistent with ADR-0006 and ADR-0007.

## Consequences

### Positive

- Profiles are inspectable, testable, serializable mapping policy rather than executable backend code.
- Backend integrations can evolve independently while honoring the same Profile contract.
- Cross-backend structural differences are handled as transformations rather than forcing Core taxonomy changes.
- Dry-run MappingPlan generation enables explainability and preflight validation.
- Rule composition supports orthogonal mapping concerns without giant backend-specific monolithic rules.
- Core conflict semantics remain generic while obligation vocabulary stays extensible and backend-aware.
- Exact/transformed/lossy/unsupported classification can respond to real backend capabilities and release differences.

### Costs

- MappingPlan construction requires rule evaluation, claim composition, dependency DAG construction, and conflict checking.
- Adapters must implement capability inspection and realization compatibility checks.
- Profile authors must provide sufficient provenance and obligation vocabulary for meaningful diagnostics.

## Deferred

This ADR deliberately does not define:

- a standardized backend obligation vocabulary;
- numeric MappingRule priority;
- automatic winner-selection among conflicting claims;
- the concrete serialization syntax of MappingPlan;
- incremental/transactional backend update semantics;
- rollback semantics for failed backend execution;
- adapter-specific realization payload schemas;
- universal capability taxonomy across simulation backends.

These require implementation evidence before being added to Core.

## Decision summary

SOL v0.1 adopts a declarative `Profile` / executable `BackendAdapter` split with an explicit `MappingPlan` intermediate representation, a five-field `MappingRule`, realization-local dependency DAGs, evaluated `exact/transformed/lossy/unsupported` outcomes, additive rule composition, and a minimal four-field `MappingClaim` whose generic obligation conflict semantics are defined by Core while concrete obligation vocabularies remain Profile/Adapter-owned.
