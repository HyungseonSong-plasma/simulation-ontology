# SOL v0.1 Qualified Relation Cardinality Constraint Proposal v0.1

**Status:** Research proposal; no accepted ADR modified  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`  
**Baseline:** [`sol-v0.1-ar01-ar04-architecture-reassessment-v0.1.md`](sol-v0.1-ar01-ar04-architecture-reassessment-v0.1.md)

## 1. Purpose

This proposal addresses AR-03, the collection-constraint counterexample identified by Validation Lab and confirmed by Research Lab.

The target semantic requirement is representative:

> For an attachment reaction, at least one `products` relation target must be a `NegativeIonSpecies`.

ADR-0007 currently defines `Cardinality`, `Type`, `Value`, `Dimension`, `Compatibility`, and `Conditional`, with predicate families `Compare`, `Membership`, `Exists`, and Boolean composition. Collection quantifiers were explicitly deferred. Therefore the current architecture cannot express the requirement unambiguously.

This proposal compares three approaches and recommends **Qualified Relation Cardinality** as the smallest Core extension that preserves precise semantics without turning Predicate into a general collection query language.

---

# 2. Counterexample and exact semantic requirement

Assume:

```text
Reaction R
  products -> {e, O2, O-}
```

The constraint is not:

- all products are negative ions;
- the relation range merely permits negative ions;
- the reaction has at least one product of any type.

It is exactly:

```text
count({ p | R products p AND p is NegativeIonSpecies }) >= 1
```

This is a cardinality constraint over a typed subset of relation targets.

---

# 3. Option A — General any/all/none quantifiers

## Form

```yaml
predicate:
  any:
    relation: products
    where:
      type: NegativeIonSpecies
```

Equivalent operators could include:

```text
any
all
none
```

## Advantages

- highly expressive;
- familiar collection semantics;
- future rules involving arbitrary collection predicates can reuse it.

## Costs

It expands the minimal Predicate language into a collection query language and immediately raises additional Core questions:

- empty collection semantics for `all` and `none`;
- nested quantifiers;
- quantification over property paths versus relations;
- unknown/indeterminate target values;
- quantifier interaction with Conditional constraints;
- normalization/intersection semantics;
- validator implementation complexity.

## Assessment

**Too broad for the observed v0.1 requirement.**

The architecture currently values small primitive vocabularies and adds expressiveness only when semantic behavior actually requires it. AR-03 proves existential typed cardinality is needed, but does not prove arbitrary collection predicates are needed.

---

# 4. Option B — Qualified Relation Cardinality

## Form

Extend the existing `Cardinality` constraint with an optional target qualifier.

Canonical conceptual shape:

```text
QualifiedRelationCardinality
├── relation
├── qualifier
├── min?
└── max?
```

The qualifier for v0.1 SHOULD initially be restricted to semantic target type/subtype matching.

Example:

```yaml
type: cardinality
relation: products
qualifier:
  target_type: NegativeIonSpecies
min: 1
```

Normative meaning:

```text
count({ t in targets(products) | type(t) <: NegativeIonSpecies }) >= 1
```

This is not a seventh Constraint primitive. It is a qualified form of the existing `Cardinality` primitive using existing Type semantics.

## Why this is minimal

It reuses two already accepted ideas:

```text
Cardinality
+ Type/subtype matching
```

rather than adding a new generic collection-expression evaluator.

## Recommended v0.1 qualifier scope

The qualifier SHOULD remain intentionally narrow:

```text
target_type
```

Future extensions MAY allow a restricted predicate qualifier, but only after additional evidence.

## Normative invariants

### QRC-I1 — Base relation validity

The referenced relation MUST be valid for the constrained source type under normal RelationDefinition domain/range rules.

### QRC-I2 — Qualified subset

The validator SHALL construct the subset of relation targets whose semantic type is equal to or a subtype of `qualifier.target_type`.

### QRC-I3 — Cardinality evaluation

`min` and `max` apply to the qualified subset, not the total relation target set.

### QRC-I4 — Ordinary cardinality remains independent

A qualified cardinality does not replace unqualified relation cardinality.

Example:

```text
products total >= 2
AND
negative-ion products >= 1
```

are two independent conjunctive obligations.

### QRC-I5 — Monotonic refinement

A more specific subtype qualifier MAY narrow an inherited qualified-cardinality constraint only if the resulting obligations remain satisfiable under ADR-0007 conjunctive composition.

### QRC-I6 — Deterministic intersection

For constraints with the same relation and semantically equivalent qualifier:

```text
[minA,maxA] ∩ [minB,maxB]
```

uses the existing Cardinality interval intersection.

If qualifiers differ, constraints compose conjunctively rather than being silently merged unless subtype reasoning proves one qualified set is contained in the other and a deterministic normalization rule is implemented.

### QRC-I7 — No implicit derivation relation

The validator MUST evaluate the qualifier over the canonical relation targets. It SHALL NOT require a synthetic derived relation such as `has_negative_ion_product` unless a domain ontology independently defines one for semantic reasons.

---

# 5. Option C — Domain-specific relation redesign

## Form

A plasma ontology could define:

```text
Reaction
  has_negative_ion_product -> NegativeIonSpecies
```

and then use ordinary cardinality:

```text
has_negative_ion_product min 1
```

## Advantages

- no Core syntax change;
- very readable domain model;
- useful if the relation itself has durable independent semantic meaning.

## Costs

The relation is usually derived from `products` plus target type. Unless that derived meaning is first-class for other reasons, this approach:

- duplicates existing reaction-product semantics;
- requires derivation/synchronization logic;
- risks inconsistent graphs (`products` says one thing, derived relation another);
- pushes a generic existential cardinality need into every domain ontology;
- violates the preference not to create new semantic constructs solely to compensate for Core validation limitations.

## Assessment

**Not recommended as the generic AR-03 solution.**

A domain-specific relation remains valid when it has independent semantic meaning, but it should not be required to express a typed subset cardinality.

---

# 6. Recommendation

Adopt **Qualified Relation Cardinality** as a follow-up Constraint ADR proposal.

Recommended architecture:

```text
Constraint
├── Cardinality
│   └── optional relation target qualifier
├── Type
├── Value
├── Dimension
├── Compatibility
└── Conditional
```

Predicate vocabulary remains unchanged:

```text
Compare
Membership
Exists
Boolean
```

Therefore AR-03 can be fixed without adding `any/all/none` to Predicate v0.1.

---

# 7. Thermal reference-model walkthrough

Qualified cardinality is not plasma-specific.

Example thermal interface constraint:

> A coupled thermal contact construct must have at least one target that is a `ThermalBoundaryScope`.

```yaml
type: cardinality
relation: coupled_scope
qualifier:
  target_type: ThermalBoundaryScope
min: 1
```

Suppose targets are:

```text
{boundary-1: ThermalBoundaryScope,
 contact-pair-1: ContactPair,
 metadata-scope: GenericScope}
```

The qualified count is 1, so the constraint passes even though not all relation targets have the qualified type.

This demonstrates that the construct is a general relation-cardinality capability rather than a plasma-specific rule.

A simpler thermal model without heterogeneous relation targets may not need the qualified form, which is acceptable; Core features need not appear in every reference model.

---

# 8. Plasma reference-model walkthrough

## 8.1 Attachment reaction

```text
reaction-attach-1
  type = AttachmentReaction
  products = {O2, O-}
```

Constraint:

```yaml
type: cardinality
relation: products
qualifier:
  target_type: NegativeIonSpecies
min: 1
```

Evaluation:

```text
products qualified by NegativeIonSpecies = {O-}
count = 1
PASS
```

## 8.2 Invalid reaction

```text
reaction-attach-2
  products = {O2, O}
```

Qualified subset:

```text
{}
```

Result:

```text
count = 0
min = 1
FAIL
```

The failure is unambiguous.

## 8.3 Multiple negative-ion products

```text
products = {O-, O2-, O2}
```

Constraint:

```text
min = 1
max = 2
```

Qualified count is 2 and passes. The ordinary total product cardinality remains independent.

---

# 9. Interaction with inheritance and interfaces

The existing ADR-0007 and ADR-0008 rules remain applicable.

Example:

```text
Reaction
  products -> Species

AttachmentReaction <: Reaction
```

A parent constraint may state:

```text
products min 1
```

while `AttachmentReaction` adds:

```text
products qualified NegativeIonSpecies min 1
```

These constraints compose conjunctively. No relation override is required.

An Interface may likewise require a qualified target count if that capability is reusable across domains.

---

# 10. Interaction with Conditional

Qualified cardinality may appear as the consequent of Conditional without changing predicate semantics.

Example:

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

`Conditional` decides applicability; Cardinality performs the typed subset count.

This preserves the architecture rule that Conditional does not duplicate the semantics of the ordinary primitive it activates.

---

# 11. Why `any/all/none` remains deferred

The following can be expressed with qualified cardinality:

```text
any target of type T
=> qualified min 1

no target of type T
=> qualified max 0

exactly N targets of type T
=> qualified min N + max N
```

A universal rule such as:

```text
all products are NegativeIonSpecies
```

can often be expressed as endpoint type narrowing if the entire relation must have that type, or as an equality between total and qualified counts if future constraint expressions support cross-count comparison. The current AR-03 evidence does not require that general capability.

Therefore `all` should not be added merely for symmetry with `any`.

---

# 12. Core/schema change vs normative clarification

| Item | Category | Proposed change |
|---|---|---|
| Constraint primitive count | No change | Remains six families |
| Predicate vocabulary | No change | Remains Compare/Membership/Exists/Boolean |
| Cardinality schema | Core/schema extension | Add optional relation-target qualifier |
| Qualifier semantics | Normative addition | Initially semantic target type/subtype only |
| Intersection algebra | Normative extension | Interval intersection for same qualified target set; conjunctive composition otherwise |
| Domain-specific derived relation | No requirement | Allowed only when independently meaningful |

This is more than a textual clarification because machine-readable Cardinality schema and validator behavior must be extended.

---

# 13. Validator contract required before Validation Lab rerun

The Constraint validator MUST implement and expose evidence for:

1. relation-target collection resolution;
2. semantic target type/subtype testing;
3. qualified subset construction;
4. qualified `min/max/exact` evaluation;
5. normalization of equivalent qualified constraints;
6. deterministic intersection for equivalent qualifiers;
7. conjunctive evaluation for distinct qualifiers;
8. provenance-preserving diagnostics showing relation, qualifier, observed qualified count, and required range;
9. Conditional activation of qualified Cardinality;
10. regression tests for empty, single-match, multiple-match, and no-match collections.

Validation Lab SHALL not treat the design walkthrough alone as execution evidence.

---

# 14. Proposed follow-up ADR scope

Recommended ADR scope:

```text
Follow-up Constraint ADR
├── Qualified Relation Cardinality semantics
├── target-type qualifier contract
├── normalization/intersection rules
├── inheritance/interface/conditional composition
└── validator diagnostic requirements
```

It SHOULD explicitly state that arbitrary collection quantifiers remain deferred.

---

# 15. SOL v0.1 architecture impact

Canonical change:

```text
Before:
Cardinality(relation total count)

After:
Cardinality(relation total count)
OR
Cardinality(relation targets qualified by semantic target type)
```

No backend-native concept is introduced. No new predicate family is introduced. No plasma-specific Core construct is introduced.

---

# 16. Gate for next validation stage

AR-03 may be considered remediated for architecture purposes only after:

1. the follow-up Constraint ADR or an equivalent normative decision is accepted;
2. the machine-readable Constraint schema supports qualified relation cardinality;
3. the validator implementation passes the required regression tests;
4. the concrete Plasma reference model contains at least one attachment-reaction rule that exercises the qualifier;
5. Validation Lab observes deterministic PASS/FAIL behavior from the executable validator.

Only then should the plasma collection-constraint blocker be removed from the SOL v0.1 freeze checklist.
