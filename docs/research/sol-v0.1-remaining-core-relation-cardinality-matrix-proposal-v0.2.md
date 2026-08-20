# SOL v0.1 Remaining Core Relation Cardinality / Requiredness Matrix Proposal v0.2

**Role:** Research  
**Date:** 2026-08-20  
**Revision scope:** RC-V2 and RC-V3 only

## 1. Preserved semantic matrix

The following generic Core source cardinalities remain unchanged:

```text
represented_by   0..*
closed_by        0..*
parameterized_by 0..*
defined_on       0..*
discretized_by   0..*
solved_by        0..*
observed_by      0..*
```

Incoming cardinality remains unconstrained by Core (`0..*`). Domain/interface/profile constraints may narrow these intervals conjunctively under ADR-0007.

No ADR-0015/0016 cardinality is reopened.

## 2. Canonical cardinality authority

Cardinality remains normatively a `Constraint` under ADR-0007.

For each relation in this decision, the Core SHALL have one canonical outgoing Cardinality Constraint equivalent to:

```text
relation: <relation-id>
direction: source/outgoing
interval:
  min: 0
  max: unbounded
```

Conceptually the canonical identities are:

```text
core.cardinality.represented_by.source
core.cardinality.closed_by.source
core.cardinality.parameterized_by.source
core.cardinality.defined_on.source
core.cardinality.discretized_by.source
core.cardinality.solved_by.source
core.cardinality.observed_by.source
```

Exact serialization of the constraint identity remains package/schema work, but there SHALL be exactly one Core semantic cardinality authority per relation/direction after normalization.

## 3. Relation-side `source_cardinality` is a projection only

A `source_cardinality` field written beside a RelationDefinition is not an independent constraint and SHALL NOT participate in Constraint intersection.

It is a machine-readable projection/cache of the canonical Core outgoing Cardinality Constraint for authoring and state recovery.

For the frozen Core registry in this scope:

```text
relation.source_cardinality
MUST equal
normalize(canonical Core outgoing Cardinality Constraint)
```

A validator SHALL NOT choose one representation over the other if they disagree.

Mismatch is:

```text
FAIL: CARDINALITY_PROJECTION_MISMATCH
```

and is classified as a transcription/validation-tooling defect.

## 4. Explicit projection is mandatory for this frozen matrix

For all seven relations in this decision, the Core `relations.yaml` transcription SHALL explicitly contain:

```yaml
source_cardinality:
  min: 0
  max: unbounded
```

Omission after this decision is accepted is not interpreted as `[0,∞]` and is not interpreted as a new semantic choice. It is:

```text
FAIL: CARDINALITY_PROJECTION_MISSING
```

for the frozen Core transcription.

This mandatory projection exists specifically so artifact-only state recovery can distinguish:

```text
accepted and intentionally unconstrained
```

from:

```text
not yet transcribed / unknown
```

## 5. Domain/interface/profile refinement does not rewrite Core projection

Example:

```text
Core solved_by constraint:        [0,∞]
Profile solved_by constraint:     [1,1]
Effective profile constraint:     [1,1]
Core relation projection remains: [0,∞]
```

The Core relation projection mirrors only the Core constraint. It SHALL NOT be rewritten to the effective profile interval.

Profile/domain/interface constraints remain separate contributors whose normalized intersection is computed under ADR-0007.

## 6. Conflict behavior

If two declarations both claim to be the canonical Core source cardinality for the same relation/direction and normalize differently, the Core package has a Schema Conflict.

If they normalize identically but are duplicate authorities, the package SHALL still fail canonical-authority validation unless the package schema explicitly treats them as one deduplicated declaration by stable identity. Declaration order SHALL NOT resolve the duplicate.

## 7. Counterexample closure

### RC-V2-A — omitted projection

Canonical Core constraint says:

```text
represented_by source [0,∞]
```

but `relations.yaml` omits `source_cardinality`.

Expected: `FAIL: CARDINALITY_PROJECTION_MISSING` in frozen Core transcription validation.

### RC-V2-B — explicit projection

Canonical Core constraint and relation projection both normalize to `[0,∞]`.

Expected: PASS.

### RC-V3-A — mismatched projection

```text
Core solved_by constraint: [0,∞]
relation projection:       [1,1]
```

Expected: `FAIL: CARDINALITY_PROJECTION_MISMATCH`.

No override or intersection is performed between the authority and its projection.

### RC-V3-B — profile narrowing

```text
Core constraint:    [0,∞]
Core projection:    [0,∞]
Profile constraint: [1,1]
```

Expected:

```text
Core transcription validation: PASS
Effective profile interval: [1,1]
```

The Core projection remains `[0,∞]`.

## 8. Research verdict

RC-V2 and RC-V3 are resolved while preserving ADR-0007's rule that cardinality semantics belong to Constraint.

**Ready for focused final Validation.**
