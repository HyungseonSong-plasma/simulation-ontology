# ADR-0007: Constraint Architecture and Composition

**Status:** Accepted for SOL v0.1 architecture  
**Date:** 2026-08-20

## Context

SOL needs a backend-independent way to express validity rules that can be inherited, supplied by interfaces, refined locally, specialized by profiles, and activated conditionally. Research and stress tests against MOOSE, COMSOL, Ansys, and plasma-oriented scenarios showed that the model must support both ordinary validation constraints and implication-style conditional applicability without adopting backend-specific override behavior.

Relevant evidence:

- [Constraint primitive stress test](../research/constraint-primitive-stress-test-v0.1.md)
- [Plasma predicate vocabulary stress test](../research/plasma-predicate-vocabulary-stress-test-v0.1.md)
- [Constraint composition principles validation](../research/constraint-composition-principles-validation-v0.1.md)
- [Constraint intersection algebra](../research/constraint-intersection-algebra-v0.1.md)
- [Relation semantics study](../research/relation-semantics-study-v0.1.md)
- [Interface/capability contract study](../research/interface-capability-contract-study-v0.1.md)
- [ADR-0006: Semantic preservation across backends](0006-semantic-preservation-across-backends.md)

## Decision

### 1. Hybrid representation

Context-local constraints SHALL normally be represented as inline typed constructs.

A constraint MAY be reified as a reusable `ConstraintDefinition` only when independent identity, reuse, provenance, lifecycle, versioning, documentation, or mapping semantics justify it.

```text
Constraint
├── Inline Typed Constraint       # default
└── ConstraintDefinition          # optional reification
```

### 2. Six primitive families

SOL v0.1 SHALL support these minimum constraint families:

```text
Constraint
├── Cardinality
├── Type
├── Value
├── Dimension
├── Compatibility
└── Conditional / Implication
```

`Conditional` is a meta-constraint: it activates ordinary constraints when a predicate evaluates to true rather than duplicating cardinality/type/value/etc. semantics.

### 3. Predicate vocabulary

The v0.1 predicate vocabulary SHALL remain domain-neutral and minimal:

```text
Predicate
├── Compare        (=, !=, <, <=, >, >=)
├── Membership     (one_of)
├── Exists         (path existence)
└── Boolean        (and, or, not)
```

Plasma-specific concepts such as ionization, electron-energy models, surface reactions, and species semantics SHALL remain in domain ontology constructs rather than becoming predicate types.

### 4. Conjunctive composition

Constraints contributed by inheritance, implemented interfaces, local definitions, profiles, and active conditionals SHALL compose conjunctively unless a future ADR explicitly introduces another semantic operator.

```text
Effective Constraints
 = Inherited
 ∧ Interface
 ∧ Local
 ∧ Profile
 ∧ Active Conditional
```

There is no declaration-order or last-write-wins semantic override in SOL Core.

### 5. Monotonic refinement

Specialization MAY narrow an existing semantic contract but SHALL NOT weaken an inherited guarantee.

Examples:

```text
Field ∩ TemperatureField = TemperatureField
[1, ∞] ∩ [0, 1] = [1, 1]
```

A profile MAY narrow backend representability or serialization choices, but SHALL NOT alter semantic truth established by Core/domain ontology.

### 6. Deterministic intersection algebra

Each primitive family SHALL have deterministic composition/intersection semantics after normalization.

- **Cardinality:** interval intersection.
- **Type:** semantic type/subtype intersection; incompatible disjoint types conflict.
- **Value:** numeric interval and/or allowed-set intersection.
- **Dimension:** normalized `DimensionVector` equality/compatibility according to the dimension contract.
- **Compatibility:** accumulate compatibility obligations conjunctively.
- **Conditional:** evaluate predicates first, then intersect the constraints activated by true predicates.

Constraints SHOULD be normalized to canonical forms before intersection.

### 7. Generic Compatibility primitive

`Compatibility` SHALL remain a single generic primitive in v0.1.

It SHALL NOT initially be split into separate quantity-, unit-, or backend-compatibility primitive classes. A lightweight discriminator such as `kind` MAY be introduced if diagnostics or dispatch require it, without changing the primitive taxonomy.

The primitive SHALL only be split in a future version if evidence shows materially different semantic behavior, intersection algebra, or lifecycle requirements.

### 8. Backend representability is not semantic compatibility

Backend capability/representability checks SHALL NOT be modeled as a `BackendCompatibilityConstraint` in SOL Core.

```text
SOL semantic validation
        !=
backend representability validation
```

A valid SOL model that a target backend cannot faithfully represent SHALL remain semantically valid and SHALL instead produce an unsupported/lossy backend mapping result in accordance with ADR-0006.

### 9. Explicit conflict detection

SOL SHALL NOT silently override incompatible constraints. Unsatisfiable intersections SHALL produce explicit diagnostics.

At minimum diagnostics SHALL distinguish:

```text
Constraint Conflict
├── Schema Conflict
├── Configuration Conflict
└── Backend Representability Conflict
```

A Schema Conflict means the ontology/interface contracts themselves are unsatisfiable. A Configuration Conflict means a particular set of active conditional constraints is unsatisfiable. A Backend Representability Conflict means the SOL model is semantically valid but cannot be represented faithfully by the selected backend/profile.

### 10. Provenance preservation

Constraint composition SHALL preserve enough provenance to identify the contributing parent type, interface, local declaration, profile, or conditional rule when reporting a conflict.

## Consequences

### Positive

- Constraint semantics remain small and backend-independent.
- Inheritance and interface composition become predictable and mechanically testable.
- Backend limitations cannot silently weaken the ontology.
- Conditional simulation metadata from MOOSE, COMSOL, Ansys, and plasma models can be normalized without backend-specific constraint classes.
- The model supports future reusable constraint definitions without forcing every local rule to become an entity.

### Costs

- Validators must implement normalization and primitive-specific intersection logic.
- Interface/profile composition requires satisfiability checking rather than simple overwrite semantics.
- Provenance must survive normalization so diagnostics remain explainable.

## Deferred

The following are deliberately outside this ADR:

- an `else` branch for Conditional;
- regex/string-specific predicates;
- collection quantifiers;
- specialized Compatibility primitive subclasses;
- a generic solver for arbitrary logical formulas;
- backend-specific capability taxonomies.

These require evidence before being added to Core.

## Decision summary

SOL v0.1 adopts a hybrid, typed, conjunctively composed constraint model with six primitive families, four domain-neutral predicate families, deterministic intersection semantics, monotonic refinement, explicit conflict detection, and strict separation between semantic validity and backend representability.
