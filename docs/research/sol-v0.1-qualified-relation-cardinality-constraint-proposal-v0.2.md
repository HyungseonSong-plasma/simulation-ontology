# SOL v0.1 Qualified Relation Cardinality Constraint Proposal v0.2

**Status:** Research proposal revision; no accepted ADR modified  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`  
**Revises:** [`sol-v0.1-qualified-relation-cardinality-constraint-proposal-v0.1.md`](sol-v0.1-qualified-relation-cardinality-constraint-proposal-v0.1.md)  
**Review basis:** [`sol-v0.1-remediation-proposals-independent-review.md`](../validation/sol-v0.1-remediation-proposals-independent-review.md)  
**Architecture baseline:** [`sol-v0.1-ar01-ar04-architecture-reassessment-v0.1.md`](sol-v0.1-ar01-ar04-architecture-reassessment-v0.1.md)

## 1. Revision objective

This revision keeps **Qualified Relation Cardinality** as the preferred AR-03 remediation, but tightens its semantics so two conforming validators cannot legitimately disagree about:

- whether the counted relation snapshot is complete;
- whether duplicate edges count once or multiple times;
- how multiple typing and subtype closure are evaluated;
- what `min`, `max`, and `exact` imply under subtype narrowing;
- when constraints with different qualifiers may be normalized or reasoned about;
- whether universal `all` semantics are supported.

No new Constraint primitive family and no general `any/all/none` predicate language are added.

---

# 2. Canonical construct

Qualified Relation Cardinality remains an optional typed qualifier on the existing relation-cardinality constraint:

```text
CardinalityConstraint
├── relation
├── qualifier?       # v0.1: target_type only
├── min?
├── max?
└── exact?
```

Canonical authoring example:

```yaml
type: cardinality
relation: products
qualifier:
  target_type: NegativeIonSpecies
min: 1
```

Canonical semantics are defined below.

---

# 3. Evaluation domain — closed-world model-instance snapshot

## QRC-I1 — Closed-world requirement

Qualified relation cardinality SHALL be evaluated only against a **closed-world model-instance relation snapshot** for the constrained source instance and relation.

For source instance `x`, relation `R`, and qualifier type `Q`, define:

```text
Targets_R(x)
```

as the complete set of resolved relation target identities in the current immutable model snapshot.

The validator MUST have evidence that this target set is complete for that snapshot.

### QRC-I2 — Open/incomplete snapshot handling

If target collection is still being resolved or completeness evidence is unavailable, the check is `BLOCKED`; it SHALL NOT infer zero missing targets.

If the validator is explicitly operating in an open-world/schema-only mode with no closed relation instance snapshot, Qualified Relation Cardinality SHALL NOT be evaluated as a model-instance PASS/FAIL. The validator SHALL report that the check is not evaluable in that mode rather than assume closed-world semantics.

Schema-level satisfiability is a separate procedure defined in Section 8.

### Boundary examples

| Input | Result |
|---|---|
| `products` snapshot declared complete and contains one negative-ion target | proceed to count |
| `products` snapshot not yet resolved | BLOCKED |
| open-world ontology query with no model-instance completeness contract | not a model-instance PASS/FAIL; route to schema-level reasoning only |

---

# 4. Counting semantics — stable identities, not serialized edges

## QRC-I3 — Unique stable target identity

The qualified count SHALL count unique canonical **model-instance target identities** after alias/name resolution under ADR-0009.

Duplicate serialized edges to the same canonical target identity count once.

Define:

```text
UniqueTargets_R(x) = distinct_by_canonical_identity(Targets_R(x))
```

A separately reified relation-instance model MAY define multiplicity as semantic data, but plain relation-edge duplication SHALL NOT create cardinality multiplicity.

### QRC-I4 — Identity resolution prerequisite

If a target reference cannot yet resolve to a stable model-instance identity, evaluation is `BLOCKED`.

If the model violates an already-required identity uniqueness invariant, validation is `FAIL` before qualified counting.

### Boundary examples

```text
products edges = [Ominus#1, alias-of-Ominus#1, O2#1]
```

After canonical identity resolution:

```text
unique targets = {Ominus#1, O2#1}
```

Qualified count for `NegativeIonSpecies` is 1, not 2.

---

# 5. Type qualification — multiple typing and subtype closure

## QRC-I5 — Versioned canonical subtype closure

Type qualification SHALL use the canonical type graph and subtype closure of the **resolved ontology/package version set** used by the model validation run.

For target `t` and qualifier type `Q`, define:

```text
qualifies(t,Q)
=
exists T in ValidTypes(t) such that T == Q or T <: Q
```

where `<:` is computed by the selected versioned canonical subtype closure.

## QRC-I6 — Multiple typing

A target MAY have multiple asserted or inferred semantic types. It qualifies if at least one consistent type is equal to or a subtype of `Q`.

The target identity is still counted at most once.

Example:

```text
Ominus#1 types = {NegativeIonSpecies, OxygenSpecies}
```

For qualifier `NegativeIonSpecies`, `Ominus#1` contributes exactly 1.

## QRC-I7 — Type inconsistency

Type consistency validation precedes qualified cardinality.

- If target typing is determinately inconsistent with ontology constraints, model validation is `FAIL`.
- If required ontology/type closure data are not resolved, QRC evaluation is `BLOCKED`.
- Core subtype matching in v0.1 SHALL NOT return `INDETERMINATE` once the versioned type graph is complete. An implementation that cannot compute the required closure is a validator/tooling `FAIL`, not semantic indeterminacy.

This prevents multiple typing from introducing implementation-dependent counting.

---

# 6. Qualified set and canonical count

For qualifier `Q`, define:

```text
Qualified_R,Q(x)
=
{ t in UniqueTargets_R(x) | qualifies(t,Q) }
```

and:

```text
count_R,Q(x) = |Qualified_R,Q(x)|
```

`min`, `max`, and `exact` apply to this count only.

Ordinary unqualified cardinality over `UniqueTargets_R(x)` remains a separate conjunctive obligation.

---

# 7. `min` / `max` / `exact` canonical algebra

## QRC-I8 — Canonical interval form

Normalize every cardinality constraint to a closed integer interval with an optional unbounded upper side:

```text
min = 0            if omitted
max = +infinity    if omitted
exact = n          normalizes to min=n, max=n
```

A constraint is locally malformed if:

```text
min < 0
max < 0
or min > max
```

Malformed interval => `FAIL`.

For the same relation and semantically equivalent qualifier, conjunctive composition is:

```text
[minA,maxA] ∩ [minB,maxB]
=
[max(minA,minB), min(maxA,maxB)]
```

If lower > upper => schema/model constraint conflict `FAIL`.

## QRC-I9 — Subtype set inclusion

If qualifier types satisfy:

```text
A <: B
```

then for every closed model snapshot:

```text
Qualified_R,A(x) ⊆ Qualified_R,B(x)
count_A <= count_B
```

This set inclusion is the only general subtype monotonicity assumed.

## QRC-I10 — Correct implication directions

For `A <: B`:

```text
min n on A  => min n on B
max n on B  => max n on A
```

The reverse implications do **not** generally hold:

```text
min n on B  -/-> min n on A
max n on A  -/-> max n on B
```

`exact n` on A does not imply `exact n` on B, and `exact n` on B does not imply `exact n` on A, unless independent reasoning proves the two qualified sets are equal.

### Validation Lab counterexample reflected

If:

```text
NegativeOxygenIon <: NegativeIonSpecies
```

then:

```text
min 1 NegativeOxygenIon
```

implies:

```text
min 1 NegativeIonSpecies
```

but:

```text
max 1 NegativeOxygenIon
```

does **not** imply:

```text
max 1 NegativeIonSpecies
```

because other negative-ion subtypes may also be present.

Conversely:

```text
max 1 NegativeIonSpecies
```

implies:

```text
max 1 NegativeOxygenIon
```

because the narrower qualified set cannot contain more members than the broader set.

## QRC-I11 — No broad monotonic-refinement claim

The v0.1 proposal SHALL NOT state that “a more specific subtype qualifier is a monotonic refinement” for arbitrary qualified cardinality intervals.

Subtype-based refinement is valid only when the specific implication being claimed follows from QRC-I9 and QRC-I10, or when satisfiability is independently proven.

---

# 8. Normalization and schema-level satisfiability scope

## QRC-I12 — Equivalent qualifiers

Two qualifiers are equivalent for normalization only when they resolve to the same canonical type identity or the versioned type reasoner proves mutual subtype equivalence.

Equivalent qualifiers use direct interval intersection.

## QRC-I13 — Distinct qualifiers

Constraints with distinct non-equivalent qualifiers remain separate conjunctive obligations unless a proven subtype/disjointness rule justifies a derived implication or conflict.

No silent merging is allowed.

## QRC-I14 — Required v0.1 subtype satisfiability rule

For `A <: B`, the validator SHALL detect at least this contradiction:

```text
min(A) > max(B)
=> unsatisfiable
```

because `count(A) <= count(B)`.

It MAY also remove a logically redundant obligation when QRC-I10 proves implication and provenance is preserved.

## QRC-I15 — Disjointness and multiple typing

If ontology semantics declare `A` and `B` disjoint, any concrete target consistently typed as both is a type-consistency `FAIL` before cardinality counting.

The QRC validator is not required in v0.1 to solve general integer constraints across arbitrary overlapping qualifiers, union types, or inferred intersections.

When no supported subset/equivalence/disjointness proof applies, distinct constraints are retained and evaluated conjunctively on concrete closed snapshots.

This limits schema-level satisfiability to deterministic rules justified by the existing type system rather than introducing a general solver.

---

# 9. Relationship to `any / all / none`

Qualified Relation Cardinality intentionally covers only the evidence-backed typed-count cases.

The following equivalences are supported:

```text
any target of type T
<=> qualified min 1

none of type T
<=> qualified max 0

exactly N of type T
<=> qualified exact N
```

## QRC-I16 — Universal arbitrary qualified semantics remain deferred

A general statement:

```text
all R targets satisfy qualifier Q
```

is **not** expressible by QRC alone in v0.1 unless the relation's independent semantic range/type contract itself excludes all non-Q targets.

The proposal SHALL NOT claim endpoint narrowing as a generic equivalent workaround for arbitrary `all` predicates.

A future cross-count equality or general collection quantifier design would require separate evidence and ADR work.

---

# 10. Model-instance validation algorithm

For one immutable source instance `x`, relation `R`, qualifier `Q`, and interval `[m,M]`:

```text
1. Validate constraint syntax and interval.
2. Resolve ontology/package version context and canonical subtype closure.
3. Resolve complete closed-world relation snapshot Targets_R(x).
4. Resolve each target to canonical stable model-instance identity.
5. Deduplicate targets by canonical identity.
6. Validate target type consistency.
7. Build Qualified_R,Q(x) using equal-or-subtype matching.
8. Count unique qualified identities.
9. PASS iff m <= count <= M; otherwise FAIL.
10. Emit relation, qualifier canonical identity, observed count, required interval,
    target identities, ontology-version context, and provenance.
```

No declaration order influences the result.

---

# 11. Deterministic boundary examples

## 11.1 Plasma positive

```text
products complete snapshot:
{O2#1, Ominus#1}
Ominus#1 : NegativeOxygenIon
NegativeOxygenIon <: NegativeIonSpecies
constraint: NegativeIonSpecies min 1
```

Qualified set = `{Ominus#1}`. Count = 1. Result = `PASS`.

## 11.2 Plasma negative

```text
products = {O2#1, O#1}
constraint: NegativeIonSpecies min 1
```

Qualified set empty. Count = 0. Result = `FAIL`.

## 11.3 Duplicate-edge boundary

```text
serialized products = {Ominus#1, alias(Ominus#1), O2#1}
```

After stable-identity deduplication qualified count = 1. `exact 2` therefore `FAIL`; `exact 1` `PASS`.

## 11.4 Multiple-typing boundary

```text
X#1 types = {NegativeIonSpecies, MetastableSpecies}
```

If type combination is consistent, X#1 qualifies once for `NegativeIonSpecies`.

If the ontology declares those types disjoint, type validation `FAIL`s before QRC counting.

## 11.5 Closed-world boundary

Relation loader has not confirmed completion of all reaction products.

Result = `BLOCKED`; absence of negative ion is not inferred.

## 11.6 Subtype implication positive

```text
NegativeOxygenIon <: NegativeIonSpecies
constraint A: NegativeOxygenIon min 2
constraint B: NegativeIonSpecies min 2
```

A implies B. B may be marked redundant if provenance-preserving normalization is implemented.

## 11.7 Subtype implication negative

```text
constraint A: NegativeOxygenIon max 1
constraint B: NegativeIonSpecies max 1
```

A does not imply B. Both constraints remain separate.

## 11.8 Schema contradiction

```text
NegativeOxygenIon <: NegativeIonSpecies
min 2 on NegativeOxygenIon
max 1 on NegativeIonSpecies
```

Since count(narrow) <= count(broad), constraints are unsatisfiable. Result = `FAIL` schema conflict.

## 11.9 INDETERMINATE boundary

For Core v0.1 target-type QRC, a well-formed closed snapshot with complete canonical type closure has **no legitimate INDETERMINATE result**.

If an implementation returns indeterminate because it cannot compute subtype closure, this is a validator/tooling `FAIL`. Missing type/version data are `BLOCKED`.

This deliberate restriction improves determinism.

---

# 12. Thermal reference walkthrough

Suppose a heterogeneous relation `coupled_scope` has complete unique targets:

```text
boundary-1 : ThermalBoundaryScope
contact-pair-1 : ContactPair
metadata-scope : GenericScope
```

Constraint:

```yaml
type: cardinality
relation: coupled_scope
qualifier:
  target_type: ThermalBoundaryScope
min: 1
```

Qualified count = 1 => PASS.

If two aliases both resolve to `boundary-1`, they still contribute one target.

If `SpecificThermalBoundary <: ThermalBoundaryScope`, a `min 1 SpecificThermalBoundary` constraint implies `min 1 ThermalBoundaryScope`, but a `max 1 SpecificThermalBoundary` constraint does not imply the broad `max 1` rule.

---

# 13. Interaction with inheritance, interfaces, and Conditional

Qualified constraints compose conjunctively like other constraints.

A parent may require total products `min 1`, while an attachment subtype additionally requires `NegativeIonSpecies min 1`. These are separate obligations.

Conditional MAY activate QRC as its consequent without changing predicate vocabulary:

```yaml
type: conditional
if:
  path: reaction_kind
  equals: attachment
then:
  - type: cardinality
    relation: products
    qualifier:
      target_type: NegativeIonSpecies
    min: 1
```

The general ADR-0007 phrase “monotonic refinement” must not be applied mechanically to subtype-qualified min/max intervals. Only the explicit implication rules in this proposal justify simplification or refinement.

---

# 14. Core/schema impact

| Item | Revision |
|---|---|
| Constraint primitive families | unchanged: six |
| Predicate vocabulary | unchanged: Compare/Membership/Exists/Boolean |
| Relation Cardinality schema | add optional `qualifier.target_type` |
| `min/max/exact` | normalize to one interval algebra |
| counting semantics | unique stable target identities in closed model snapshot |
| subtype matching | versioned canonical subtype closure |
| monotonic refinement claim | remove broad claim; replace with directional implication rules |
| `any` | represented only for typed existential case via `min 1` |
| `none` | represented only for typed absence via `max 0` |
| general `all` | explicitly deferred |
| backend-native semantics | none introduced |

This remains a narrow justified Core/schema extension, not a seventh primitive or generic collection query language.

---

# 15. Validator/runtime contract before executable revalidation

Validation Lab SHALL require executable evidence for:

1. closed-world relation snapshot completeness handling;
2. stable-identity alias resolution and edge deduplication;
3. multiple consistent types counted once;
4. subtype closure against an explicit ontology-version set;
5. inconsistent typing failure;
6. `min/max/exact` canonicalization;
7. same-qualifier interval intersection;
8. subtype directional implication tests for both `min` and `max`;
9. `min(narrow) > max(broad)` schema contradiction detection;
10. distinct qualifier preservation when no proven reasoning rule applies;
11. Conditional activation;
12. provenance-preserving diagnostics.

---

# 16. Independent-review issue matrix

| Independent-review issue | Revision response | Status |
|---|---|---|
| QRC-01 open-world vs closed-world | explicit closed-world model-instance snapshot + BLOCKED on incomplete resolution | **resolved at contract level** |
| QRC-02 multiple typing/subtype closure | versioned canonical subtype closure; consistent multiple typing; inconsistency precheck | **resolved at contract level** |
| QRC-03 set vs multiset | count unique stable target identities; duplicate edges do not multiply count | **resolved** |
| QRC-04 incorrect monotonic refinement | broad claim removed; min/max subtype implication directions explicitly defined | **resolved** |
| QRC-05 distinct qualifier satisfiability | same/equivalent normalization, required subtype contradiction rule, otherwise preserve obligations | **resolved within stated v0.1 reasoning scope** |
| QRC-06 universal workaround incomplete | general `all` explicitly unsupported/deferred; endpoint narrowing no longer presented as generic equivalent | **resolved** |
| Executable evidence for QRC | schema/validator fixtures not yet implemented | **unresolved — executable validation required** |

---

# 17. Contract re-review gate

This revision is ready for a **short contract re-review** focused on:

- closed-world/identity/type determinism;
- correctness of subtype `min`/`max` implication directions;
- bounded schema-level reasoning scope;
- confirmation that no general collection query language has leaked into Core.

It is not sufficient for architecture freeze by itself. After contract re-review, AR-03 remains execution-blocking until the machine-readable schema and validator implement the rules above and a concrete Plasma reference model demonstrates deterministic PASS/FAIL behavior.

**Revision verdict:** Qualified Relation Cardinality remains the preferred minimal AR-03 remediation and may enter short contract re-review.
