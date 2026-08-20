# ADR-0017: Core Relation Cardinality / Requiredness Baseline

**Status:** Accepted  
**Date:** 2026-08-20  
**Amends:** frozen SOL v0.1 baseline through ADR-0016 for the relation-cardinality scope below

## Context

After ADR-0015 and ADR-0016 fixed the cardinalities of Simulation/Task and component/condition-target relations, seven Core semantic association relations still had no explicit requiredness state:

```text
represented_by
closed_by
parameterized_by
defined_on
discretized_by
solved_by
observed_by
```

The absence was ambiguous during machine-readable consolidation: one validator could interpret it as deliberately unconstrained, while another could treat it as not yet decided.

Independent Validation accepted a generic `[0,∞]` Core matrix and required an explicit serialization rule that preserves ADR-0007's principle that cardinality semantics belong normatively to Constraint.

## Decision

### 1. Generic Core source cardinality

The following relations have generic Core source interval:

```text
represented_by   0..*
closed_by        0..*
parameterized_by 0..*
defined_on       0..*
discretized_by   0..*
solved_by        0..*
observed_by      0..*
```

This means the Core imposes neither a minimum occurrence nor a universal maximum for these relations.

Incoming/inverse cardinality is likewise unconstrained by Core unless another accepted contract states otherwise.

### 2. Why requiredness remains outside generic Core

Generic Core validity is distinct from:

```text
domain completeness
profile executability
backend realization readiness
```

Therefore:

- PhysicsModel may exist before a MathematicalModel is selected;
- MathematicalModel may require no closure or no spatial/numerical realization;
- Analysis may exist before a SolverConfiguration is selected;
- Result may exist without an ObservationModel;
- multiple mathematical representations, numerical representations, solver configurations, or observations may be semantically valid.

A domain, Interface, or Profile MAY narrow these intervals conjunctively under ADR-0007.

Example:

```text
Core solved_by:    [0,∞]
Profile solved_by: [1,1]
Effective:         [1,1]
```

### 3. Cardinality remains a Constraint

For each relation in section 1, the Core SHALL contain one canonical outgoing Cardinality Constraint normalized to:

```text
relation: <relation-id>
direction: source/outgoing
min: 0
max: unbounded
```

Conceptual stable identities are:

```text
core.cardinality.represented_by.source
core.cardinality.closed_by.source
core.cardinality.parameterized_by.source
core.cardinality.defined_on.source
core.cardinality.discretized_by.source
core.cardinality.solved_by.source
core.cardinality.observed_by.source
```

Exact serialization syntax may evolve, but there SHALL be one canonical semantic authority per relation/direction after normalization.

### 4. Relation-side cardinality is a projection

A relation-side field such as:

```yaml
source_cardinality:
  min: 0
  max: unbounded
```

is a projection/cache of the canonical Core Cardinality Constraint for authoring and artifact state recovery.

It is not a second Constraint contributor and SHALL NOT be intersected with the canonical Constraint.

For the frozen Core registry in this scope:

```text
projection missing   -> FAIL: CARDINALITY_PROJECTION_MISSING
projection mismatch  -> FAIL: CARDINALITY_PROJECTION_MISMATCH
projection matches   -> PASS
```

### 5. Profile/domain/interface refinement does not rewrite Core projection

Later narrowing Constraints remain separate contributors.

Example:

```text
Core Constraint:       solved_by [0,∞]
Core relation projection:      [0,∞]
Profile Constraint:            [1,1]
Effective Profile interval:    [1,1]
```

The Core relation projection remains `[0,∞]`.

### 6. Duplicate authority is invalid

If multiple declarations claim to be the canonical Core source cardinality for the same relation/direction, validators SHALL NOT use declaration order, newest version, or file precedence to choose one.

Differing normalized values are a Schema Conflict. Duplicate authorities with the same normalized value are also invalid unless the package schema explicitly proves they are one declaration by stable identity.

### 7. Previously fixed cardinalities remain unchanged

This ADR does not change:

```text
has_model           1..1
has_task            1..*
uses_model          1..1
has_analysis        1..1
produces            0..*
includes_component  0..*
applied_to          1..*
analyzed_by          derived
```

## Consequences

### Positive

- every Core relation now has an explicit requiredness state or a previously accepted specialized rule;
- intentionally unconstrained relations cannot be confused with untranscribed decisions;
- cardinality authority remains in Constraint while relation registries remain easy to inspect;
- domain/profile completeness can narrow Core semantics without override behavior;
- backend execution requirements cannot silently become Core requiredness.

### Costs

- the Core package must preserve projection/Constraint consistency;
- validators need a transcription consistency check;
- final schema must encode canonical constraint identity and projection validation.

## Validation evidence

- `docs/research/sol-v0.1-remaining-core-relation-cardinality-matrix-proposal-v0.1.md`
- `docs/validation/sol-v0.1-remaining-core-relation-cardinality-matrix-independent-review-v0.1.md`
- `docs/research/sol-v0.1-remaining-core-relation-cardinality-matrix-proposal-v0.2.md`
- `docs/validation/sol-v0.1-remaining-core-relation-cardinality-matrix-focused-final-review-v0.2.md`

## Freeze amendment

The existing design-stage freeze remains in force. This ADR amends only the generic Core cardinality/requiredness state of the seven relations above and the authority/projection transcription rule.

## Decision summary

SOL v0.1 assigns `[0,∞]` generic Core source cardinality to `represented_by`, `closed_by`, `parameterized_by`, `defined_on`, `discretized_by`, `solved_by`, and `observed_by`; cardinality remains normatively a Constraint, while relation-side cardinality fields are mandatory matching projections for frozen Core state recovery.
