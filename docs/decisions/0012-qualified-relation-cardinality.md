# ADR-0012: Qualified Relation Cardinality

**Status:** Accepted  
**Date:** 2026-08-20  
**Supplements:** ADR-0007 Constraint Architecture and Composition

## Context

ADR-0007 defines six constraint families and deliberately deferred collection quantifiers. Plasma reference-model validation exposed a narrower requirement that is not expressible unambiguously by the current unqualified Cardinality and Type contracts:

```text
reaction.products contains at least one NegativeIonSpecies
```

A general `any/all/none` predicate language would expand Core more than required. The validated remediation instead qualifies an existing relation cardinality by target semantic type.

## Decision

### 1. Constraint taxonomy remains unchanged

SOL v0.1 SHALL retain the six ADR-0007 constraint families. No new constraint primitive or generic collection-query language is introduced.

`Cardinality` MAY carry an optional relation-target type qualifier:

```text
Cardinality
├── relation
├── qualifier?
│   └── target_type
├── min?
├── max?
└── exact?
```

This construct is called **Qualified Relation Cardinality (QRC)**.

### 2. QRC denotes a typed target-identity set

For one constrained source and a complete immutable model-instance relation snapshot:

```text
Q = {
  canonical_target_identity(t)
  | source --relation--> t
  AND t matches qualifier.target_type
}

q = |Q|
```

Target-type matching SHALL use the resolved ontology/package version set and canonical subtype closure.

Duplicate serialized relation edges to the same canonical target identity SHALL count once. Semantic multiplicity that must count multiple times requires distinct/reified relation instances rather than duplicate ordinary edges.

### 3. Closed-world model-instance evaluation

QRC model-instance PASS/FAIL evaluation SHALL require a complete immutable relation snapshot.

If the invocation is open-world, incomplete, schema-only, or otherwise lacks a completeness assertion for the instance relation snapshot, the canonical model-instance result SHALL be:

```text
BLOCKED
code = QRC_CLOSED_SNAPSHOT_REQUIRED
```

Partial observation SHALL NOT justify model-instance FAIL merely because a qualifying target has not yet been observed.

Schema-level satisfiability reasoning is a separate validation operation and SHALL NOT be reported as model-instance QRC PASS/FAIL.

### 4. Qualifier identity resolution

`qualifier.target_type` SHALL resolve to exactly one canonical semantic type identity in the declared ontology/package environment.

Canonical boundaries:

- malformed qualifier reference => `FAIL: QRC_QUALIFIER_MALFORMED`;
- multiple canonical candidates => `FAIL: QRC_QUALIFIER_AMBIGUOUS`;
- resolved identity is not a semantic type => `FAIL: QRC_QUALIFIER_NOT_TYPE`;
- declared required ontology/package/type evidence is not yet resolved => `BLOCKED: QRC_QUALIFIER_EVIDENCE_MISSING`;
- required canonical subtype resolver implementation unavailable => `FAIL: QRC_SUBTYPE_RESOLVER_UNAVAILABLE`.

With complete canonical subtype closure, v0.1 does not define an `INDETERMINATE` target-type-membership result.

### 5. Stable identity and multiple typing

A target matches qualifier `T` iff at least one consistent asserted or inferred semantic type of that target is equal to or a subtype of `T` in the canonical subtype closure.

A multiply typed target that matches through several consistent types SHALL contribute one canonical target identity to `Q`.

Type inconsistency is a prior validation failure; QRC SHALL NOT choose among inconsistent types.

### 6. `min`, `max`, and `exact` normalize by interval intersection

`min`, `max`, and `exact` MAY appear together. Every supplied bound SHALL be a non-negative integer. Any negative or non-integer bound is:

```text
FAIL
code = QRC_BOUND_INVALID
```

Normalize valid supplied bounds to one interval:

```text
L = max(min if present, exact if present), default 0
U = min(max if present, exact if present), default +infinity
```

If `L <= U`, the canonical interval is `[L,U]`.

If `L > U`:

```text
FAIL
code = QRC_EMPTY_INTERVAL
```

Validators SHALL NOT treat the same mixed authoring form as mutually exclusive syntax and SHALL NOT silently discard a field.

Examples:

```yaml
min: 1
max: 3
exact: 2
```

normalizes to `[2,2]`.

```yaml
min: 3
exact: 2
```

normalizes to an empty interval and fails with `QRC_EMPTY_INTERVAL`.

### 7. Instance evaluation

After qualifier resolution, snapshot completeness, stable-identity deduplication, canonical subtype matching, and bound normalization:

```text
PASS iff L <= q <= U
FAIL otherwise
```

Failure diagnostics SHALL include:

- constrained source identity;
- relation canonical identity;
- qualifier canonical type identity;
- normalized interval `[L,U]`;
- observed canonical qualified count `q`;
- canonical target identities counted;
- ontology/package version context.

Serialization order of relation edges or asserted types SHALL NOT change `Q`, `q`, or the result.

### 8. Subtype implication algebra is directional

For semantic types:

```text
A <: B
```

then:

```text
Q_A subseteq Q_B
count(Q_A) <= count(Q_B)
```

Therefore the generally valid implications are:

```text
min n on A => min n on B
max n on B => max n on A
```

The reverse implications do not generally hold:

```text
min n on B !=> min n on A
max n on A !=> max n on B
```

`exact n` on one qualifier SHALL NOT imply `exact n` on a strict supertype/subtype unless equality of the qualified sets is independently established.

A blanket "monotonic refinement" rule SHALL NOT be applied across arbitrary qualified `min/max/exact` constraints.

### 9. Constraint composition

QRC remains conjunctive under ADR-0007.

Constraints over the same relation and canonically equivalent qualifier SHALL compose by interval intersection. An empty interval is an explicit constraint conflict.

Constraints over distinct qualifiers remain distinct conjunctive obligations unless canonical type reasoning proves equivalence or a supported one-way implication. SOL v0.1 does not require a general solver for arbitrary overlaps among qualified sets.

### 10. General universal collection predicates remain deferred

QRC supports typed subset counts such as:

```text
at least one T     -> qualified min 1
no T               -> qualified max 0
exactly N T        -> qualified exact N
```

It does not introduce arbitrary:

```text
all(targets, predicate)
any(targets, arbitrary predicate)
none(targets, arbitrary predicate)
```

An "all relation targets are T" contract is not inferred from QRC unless it is independently expressible through the relation/type contract itself.

## Examples

### Plasma

```yaml
type: cardinality
relation: products
qualifier:
  target_type: NegativeIonSpecies
min: 1
```

For a closed snapshot:

```text
products = {O2, O-}
```

with `O- <: NegativeIonSpecies`, the qualified set has one canonical target and the constraint passes.

For a closed snapshot `{O2, O}`, it fails. For an incomplete/open snapshot containing only `{O2}`, it is `BLOCKED: QRC_CLOSED_SNAPSHOT_REQUIRED`, not FAIL.

### Thermal

```yaml
type: cardinality
relation: coupled_scope
qualifier:
  target_type: ThermalBoundaryScope
min: 1
```

A closed snapshot with one `ThermalBoundaryScope` and one unrelated `ContactPair` has qualified count one and passes.

## Consequences

### Positive

- AR-03 is expressible without adding a seventh constraint family.
- Core gains typed subset cardinality without a generic collection query language.
- Stable-identity counting avoids duplicate-edge inflation.
- Subtype reasoning and mixed cardinality bounds have deterministic algebra.

### Costs

- Validators require canonical identity resolution, subtype closure, closed-snapshot evidence, and interval normalization.
- Open-world model-instance graphs cannot produce QRC PASS/FAIL until relation completeness is established.

## Deferred

- arbitrary `all/any/none` predicates;
- general satisfiability solving over overlapping arbitrary qualifiers;
- multiset cardinality for ordinary relation-edge duplication;
- backend-specific collection semantics.

## Validation requirement

Design-stage acceptance requires independent contract validation plus reference-model and official-document stress testing showing that QRC is necessary, deterministic, and backend-independent. A minimal schema/validator smoke case MAY be retained as supplementary implementability evidence.

Full backend installation/licensing, Adapter implementation, and backend execution V&V are outside the QRC architecture gate and SHALL NOT block SOL v0.1 design-stage architecture freeze unless they later reveal a genuine architecture counterexample.

## Decision summary

SOL v0.1 extends the existing Cardinality family with an optional semantic target-type qualifier. QRC is evaluated over closed model-instance snapshots using canonical target identity and subtype closure, normalizes `min/max/exact` by interval intersection, preserves directional subtype implication algebra, and leaves general collection quantifiers deferred.
