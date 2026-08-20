# SOL v0.1 Qualified Relation Cardinality Constraint Proposal v0.2.1

**Status:** Limited contract revision; no accepted ADR modified  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`  
**Supersedes for contract review:** `sol-v0.1-qualified-relation-cardinality-constraint-proposal-v0.2.md`  
**Primary review input:** `docs/validation/sol-v0.1-remediation-proposals-v0.2-contract-re-review.md`

## 1. Revision scope

This revision preserves the v0.2 Qualified Relation Cardinality design and changes only the two remaining contract ambiguities and the directly related qualifier-resolution boundary identified by Validation Lab.

The architectural decision remains:

```text
Constraint families remain six.
Cardinality gains an optional relation-target type qualifier.
Predicate remains Compare / Membership / Exists / Boolean.
Arbitrary any/all/none remains deferred.
```

No new Constraint family, generic collection query language, backend vocabulary, or new Core concept is introduced.

---

# 2. Canonical Qualified Relation Cardinality form

Conceptual form remains:

```text
Cardinality
├── relation
├── qualifier?
│   └── target_type
├── min?
├── max?
└── exact?
```

For a closed model-instance snapshot, a qualified constraint denotes:

```text
Q = {
  canonical_target_identity(t)
  | source --relation--> t
  AND t matches qualifier.target_type under canonical subtype closure
}

q = |Q|
```

Duplicate serialized edges to the same canonical target identity do not increase `q`.

---

# 3. Closed-world invocation boundary

## 3.1 Normative rule QRC21-I1 — model-instance evaluation requires a closed snapshot

Qualified Relation Cardinality PASS/FAIL evaluation is defined only over a **complete immutable model-instance relation snapshot** together with the resolved ontology/package version set and canonical subtype closure.

If the validator is asked to perform model-instance QRC evaluation while the relation snapshot is open-world, incomplete, schema-only, or otherwise not asserted complete, the canonical result is:

```text
BLOCKED
code = QRC_CLOSED_SNAPSHOT_REQUIRED
```

The validator SHALL NOT emit model-instance `PASS` or `FAIL` from partial observations.

Schema-level satisfiability reasoning is a separate validation operation. It MAY reason over the normalized QRC contract, but it SHALL NOT reuse model-instance count results or claim model-instance PASS/FAIL without a closed snapshot.

Therefore the same QRC object can be valid schema input while its instance evaluation is `BLOCKED`.

## 3.2 Positive boundary case

Closed immutable snapshot:

```text
products = {O2, O-}
qualifier = NegativeIonSpecies
min = 1
```

Subtype closure is complete and `O-` matches.

```text
q = 1
=> PASS
```

## 3.3 Negative boundary case

Observed open-world graph currently contains only:

```text
products = {O2}
```

but the validator has no completeness assertion for the `products` relation snapshot.

```text
no closed snapshot
=> BLOCKED: QRC_CLOSED_SNAPSHOT_REQUIRED
```

It is non-conforming to return FAIL merely because no negative ion has yet been observed.

---

# 4. Qualifier canonical identity and missing evidence boundary

## 4.1 Normative rule QRC21-I2 — qualifier resolution

`qualifier.target_type` SHALL resolve to exactly one canonical semantic type identity in the resolved ontology/package environment.

The result boundary follows provenance:

- syntactically malformed qualifier reference => `FAIL: QRC_QUALIFIER_MALFORMED`;
- multiple canonical candidates after the declared environment is resolved => `FAIL: QRC_QUALIFIER_AMBIGUOUS`;
- exactly one canonical identity that is not a semantic type => `FAIL: QRC_QUALIFIER_NOT_TYPE`;
- required ontology/package/type provider is declared but not yet available/resolved => `BLOCKED: QRC_QUALIFIER_EVIDENCE_MISSING`;
- all required evidence is present but the validator implementation cannot compute the required canonical type/subtype closure => `FAIL: QRC_SUBTYPE_RESOLVER_UNAVAILABLE`.

A complete canonical subtype closure has no legitimate `INDETERMINATE` target-type membership result in v0.1.

## 4.2 Positive boundary case

`NegativeIonSpecies` resolves uniquely to canonical identity `Tneg` in the declared plasma ontology version set.

```text
unique type identity => continue evaluation
```

## 4.3 Negative boundary case

Two imported namespaces expose two distinct canonical `NegativeIonSpecies` identities and the author uses an ambiguous bare reference.

```text
multiple canonical candidates
=> FAIL: QRC_QUALIFIER_AMBIGUOUS
```

No namespace/import-order preference is allowed.

---

# 5. Stable identity counting and multiple typing

The v0.2 rules remain normative.

## QRC21-I3 — stable identity set semantics

The qualified count is the cardinality of unique canonical model-instance target identities after ADR-0009 alias resolution.

```text
multiple serialized edges -> same canonical target identity
=> count once
```

Multiplicity that is semantically meaningful must be represented by distinct/reified relation instances; ordinary relation-edge duplication does not create multiset semantics.

## QRC21-I4 — multiple typing

A canonical target identity matches `qualifier.target_type = T` iff **at least one consistent asserted or inferred semantic type** of the target is equal to or a subtype of `T` in the versioned canonical subtype closure.

A target that matches through multiple types still counts once.

Type inconsistency that violates the ontology/type-validation contract is a prior validation `FAIL`; QRC SHALL NOT choose one inconsistent type arbitrarily.

---

# 6. Canonical min/max/exact authoring semantics

## 6.1 Normative rule QRC21-I5 — bounds normalize by intersection

`min`, `max`, and `exact` MAY appear together. They are **not mutually exclusive authoring forms**.

All present fields normalize into one closed integer interval.

Let:

```text
lower candidates = { min if present, exact if present }
upper candidates = { max if present, exact if present }

L = max(lower candidates), default 0
U = min(upper candidates), default +infinity
```

`exact = n` therefore contributes both `L >= n` and `U <= n`.

All supplied bounds SHALL be non-negative integers.

If:

```text
L <= U
```

normalization succeeds and the canonical interval is `[L,U]`.

If:

```text
L > U
```

the constraint object is contradictory and validation returns:

```text
FAIL
code = QRC_EMPTY_INTERVAL
```

This is one canonical result; validators SHALL NOT alternatively treat the same combination as mutually-exclusive syntax or silently discard one field.

## 6.2 Positive boundary case — exact inside min/max

```yaml
min: 1
max: 3
exact: 2
```

Normalization:

```text
L = max(1,2) = 2
U = min(3,2) = 2
=> [2,2]
```

The constraint is valid. Instance PASS/FAIL then depends only on whether the qualified count equals 2.

## 6.3 Negative boundary case — contradictory exact/min

```yaml
min: 3
exact: 2
```

Normalization:

```text
L = 3
U = 2
=> L > U
=> FAIL: QRC_EMPTY_INTERVAL
```

The validator SHALL NOT reinterpret this as `[2,2]` and SHALL NOT classify it as an arbitrary syntax error.

## 6.4 Positive boundary case — exact/max agreement

```yaml
max: 2
exact: 2
```

normalizes to `[2,2]`.

## 6.5 Negative boundary case — invalid scalar

```yaml
exact: -1
```

```text
negative cardinality bound
=> FAIL: QRC_BOUND_INVALID
```

---

# 7. Qualified count evaluation

## 7.1 Normative rule QRC21-I6 — evaluation after normalization

After qualifier resolution, snapshot completeness, stable-identity deduplication, type matching, and bound normalization:

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

Serialization order of edges or asserted types SHALL NOT affect `Q`, `q`, or the result.

---

# 8. Subtype implication algebra

The corrected v0.2 algebra remains unchanged and replaces any broad monotonic-refinement claim.

For qualifier types:

```text
A <: B
```

we have:

```text
Q_A subseteq Q_B
count(Q_A) <= count(Q_B)
```

Therefore only the following one-way implications are generally valid:

```text
min n on A  => min n on B
max n on B  => max n on A
```

The reverse directions do not follow:

```text
min n on B  !=> min n on A
max n on A  !=> max n on B
```

`exact n` on one qualifier does not imply `exact n` on a strict supertype or subtype unless equality of the two qualified target sets is independently proven.

The term “monotonic refinement” SHALL NOT be used as a blanket rule for arbitrary qualified `min/max/exact` intervals.

---

# 9. Equivalent and distinct qualifier composition

The v0.2 bounded reasoning scope remains unchanged.

## QRC21-I7 — equivalent qualifier

Constraints over the same relation and canonically equivalent qualifier normalize by interval intersection.

Example:

```text
QRC(products, NegativeIon, min=1)
AND
QRC(products, NegativeIon, max=2)
=> QRC(products, NegativeIon, [1,2])
```

An empty intersection is `FAIL` with the normal constraint-conflict provenance.

## QRC21-I8 — distinct qualifier

Distinct qualifiers remain separate conjunctive obligations unless canonical subtype reasoning proves a specific implication required by Section 8 or proves equivalence/disjointness within the supported bounded reasoning rules.

SOL v0.1 does not require a general solver for overlapping arbitrary qualified sets.

---

# 10. Universal semantics remain deferred

Qualified Relation Cardinality in v0.1 supports typed subset counts such as:

```text
any target of type T      -> qualified min 1
no target of type T       -> qualified max 0
exactly N targets of T    -> qualified exact N
```

It does **not** define an arbitrary `all(targets, predicate)` operator.

“All relation targets are T” is expressible through relation endpoint narrowing only when excluding every non-T target is genuinely the relation contract. No equality between total and qualified counts is introduced by this proposal.

---

# 11. Thermal boundary example

Constraint:

```yaml
type: cardinality
relation: coupled_scope
qualifier:
  target_type: ThermalBoundaryScope
min: 1
```

Closed targets after identity normalization:

```text
boundary-1 : ThermalBoundaryScope
contact-1  : ContactPair
```

Qualified set:

```text
{boundary-1}
q = 1
```

Result: `PASS`.

If the same relation snapshot is incomplete/open-world, result is `BLOCKED: QRC_CLOSED_SNAPSHOT_REQUIRED`, regardless of currently observed count.

---

# 12. Plasma boundary examples

## 12.1 Positive attachment example

```text
reaction-1.products = {O2, O-}
NegativeIonSpecies qualifier
min = 1
```

With a closed snapshot and canonical subtype evidence:

```text
Q = {O-}
q = 1
=> PASS
```

## 12.2 Negative attachment example

```text
reaction-2.products = {O2, O}
min = 1
```

Closed snapshot:

```text
Q = {}
q = 0
=> FAIL
```

## 12.3 Duplicate-edge example

Serialized edges contain `O-` twice but both edges resolve to the same model-instance identity.

```text
Q = {O-}
q = 1
```

The count is not 2.

## 12.4 Multiple-typing example

One target has consistent types:

```text
{OxygenSpecies, NegativeIonSpecies, ChargedSpecies}
```

It matches the negative-ion qualifier but contributes exactly one canonical target identity to `Q`.

---

# 13. Core/schema impact

| Item | v0.2.1 decision |
|---|---|
| Constraint primitive count | unchanged: six |
| Predicate vocabulary | unchanged |
| Cardinality schema | optional `qualifier.target_type` retained |
| `min/max/exact` | all allowed; canonical intersection normalization |
| closed-world requirement | explicit model-instance evaluation contract |
| stable identity count | unchanged from v0.2 |
| subtype algebra | unchanged corrected one-way rules |
| arbitrary `any/all/none` | deferred |
| backend-native vocabulary | none |

No new architecture structure is required.

---

# 14. Contract re-review finding matrix

| Re-review finding | v0.2.1 status | Revision disposition |
|---|---|---|
| QRC-01 open vs closed world | **Resolved** | open/schema-only model-instance invocation => canonical BLOCKED code |
| QRC-02 multiple typing and subtype closure | **Resolved** | v0.2 rule unchanged |
| QRC-03 set vs multiset | **Resolved** | v0.2 stable-identity set semantics unchanged |
| QRC-04 subtype/min/max algebra | **Resolved** | corrected one-way implication retained; no blanket monotonic claim |
| QRC-05 distinct qualifier satisfiability | **Resolved** | v0.2 bounded reasoning scope unchanged |
| QRC-06 universal workaround | **Resolved** | arbitrary universal semantics explicitly deferred |
| C-QRC-01 mixed `exact/min/max` | **Resolved** | all fields intersect into one canonical interval; empty interval => FAIL |
| qualifier unknown/missing evidence boundary from §4.3 re-review requirement | **Resolved** | malformed/ambiguous/not-type => FAIL; missing declared provider evidence => BLOCKED |

**Unresolved contract findings:** none identified by this revision.  
**Regression:** none intentionally introduced.

---

# 15. Deterministic boundary summary

For identical immutable ontology/package context, model snapshot, and constraint input:

| Situation | Canonical result |
|---|---|
| closed complete snapshot; normalized count in interval | `PASS` |
| closed complete snapshot; count outside interval | `FAIL` |
| open/incomplete/schema-only instance invocation | `BLOCKED: QRC_CLOSED_SNAPSHOT_REQUIRED` |
| malformed/ambiguous qualifier | `FAIL` |
| qualifier provider declared but unresolved | `BLOCKED` |
| canonical subtype resolver implementation missing | `FAIL` |
| `min:1,max:3,exact:2` | valid normalized interval `[2,2]` |
| `min:3,exact:2` | `FAIL: QRC_EMPTY_INTERVAL` |
| duplicate edges to same stable target identity | count once |
| target matches qualifier through multiple consistent types | count once |

---

# 16. Final contract re-review gate

**Contract-only status proposed by Research Lab: READY FOR FINAL CONTRACT RE-REVIEW.**

The remaining work is execution-validation-only:

1. machine-readable Cardinality schema implementing optional `qualifier.target_type` and `min/max/exact` normalization;
2. validator mode/snapshot completeness input and `QRC_CLOSED_SNAPSHOT_REQUIRED` diagnostic;
3. canonical identity/alias resolution and versioned subtype-closure implementation;
4. deterministic bound-normalization tests including mixed `exact/min/max` fixtures;
5. duplicate-edge and multiple-typing fixtures;
6. Thermal and Plasma concrete model fixtures exercising PASS/FAIL/BLOCKED boundaries;
7. Conditional activation regression tests;
8. repeated runs proving edge/type serialization order does not alter canonical qualified sets or diagnostics.

A final contract re-review PASS may allow this proposal to proceed toward a follow-up Constraint ADR. It does **not** by itself satisfy the SOL v0.1 architecture-freeze execution gate.