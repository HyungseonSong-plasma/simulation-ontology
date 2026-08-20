# SOL v0.1 Conditional / Predicate Schema Proposal v0.1

**Status:** Research proposal  
**Date:** 2026-08-20  
**Scope:** Final SOL v0.1 Constraint-family consolidation  
**Depends on:** ADR-0007, ADR-0018, ADR-0019, ADR-0020, ADR-0021, ADR-0022

## 1. Problem

ADR-0007 accepts `Conditional / Implication` as the sixth and final v0.1 Constraint family and fixes the domain-neutral Predicate vocabulary:

```text
Compare (=, !=, <, <=, >, >=)
Membership (one_of)
Exists
Boolean (and, or, not)
```

The remaining design work is to define a machine-readable authoring/normalized contract that:

- does not duplicate Cardinality/Type/Value/Dimension/Compatibility semantics;
- does not require a general graph-query language in Core;
- handles missing/unresolved controlling values deterministically;
- preserves exact numeric comparison semantics from ADR-0021;
- activates ordinary constraints without nested conditional ambiguity;
- produces deterministic results under partial semantic information.

## 2. Conditional remains a meta-constraint

Canonical shape:

```yaml
type: conditional
if: <Predicate>
then:
  - <ordinary Constraint>
```

`then` MAY contain only the five ordinary v0.1 families:

```text
Cardinality
Type
Value
Dimension
Compatibility
```

A Conditional SHALL NOT directly contain another Conditional in v0.1.

No `else` branch is defined. `else C` can be expressed as a second Conditional with the logically negated predicate when the author needs that behavior.

## 3. Predicate reference boundary

### 3.1 Authoring reference

Authoring MAY use a compact local/domain reference string:

```yaml
ref: electron_transport_model
```

The exact human graph/path syntax is not standardized by this slice.

### 3.2 Normalized EvaluationReference

Before predicate evaluation, the compiler resolves the authoring reference to a normalized evaluation reference:

```yaml
ref:
  key: cfg:electron_transport_model
```

Conceptually:

```text
EvaluationReference = {
  key: stable evaluation-reference key within the normalized model snapshot
}
```

`EvaluationReference` is not a new ontology Entity and is not backend-local identity. It is a normalized validation handle whose lookup semantics are supplied by the normalized model/package evaluation context.

The canonical package-integration step may later define the storage table for these keys; this proposal defines only the predicate-facing contract.

## 4. Predicate lookup contract

A closed resolved configuration snapshot SHALL expose deterministic lookup:

```text
lookup(EvaluationReference)
    -> PRESENT(value)
     | ABSENT
     | UNRESOLVED
```

Meanings:

- `PRESENT(value)`: a controlling scalar/presence value is known;
- `ABSENT`: the snapshot definitively contains no value/target for this reference;
- `UNRESOLVED`: the snapshot cannot decide presence/value because required semantic information is incomplete.

`ABSENT != UNRESOLVED`.

Backend installation/license/runtime availability SHALL NOT be used to create `UNRESOLVED` in the Core predicate evaluation context.

## 5. Normalized scalar values

Predicate scalar comparison reuses ADR-0021 scalar semantics:

```text
scalar_kind = number | string | boolean
```

Normalized numbers use exact decimal form:

```yaml
coefficient: "15"
exponent10: 0
```

No binary floating-point value is semantic authority.

String and Boolean values remain exact JSON scalars.

No cross-kind coercion is allowed.

## 6. Compare predicate

### Authoring example

```yaml
predicate: compare
ref: mode
op: eq
value: advanced
```

### Normalized example

```yaml
predicate: compare
ref:
  key: cfg:mode
scalar_kind: string
op: eq
value: advanced
```

Operators:

```text
eq, ne, lt, le, gt, ge
```

Rules:

- `eq` and `ne` are allowed for number/string/boolean;
- `lt`, `le`, `gt`, `ge` are allowed only for number;
- both sides SHALL use the same scalar kind;
- numeric comparison uses ADR-0021 exact-decimal ordering;
- no string lexical ordering or Boolean ordering is introduced.

Lookup semantics:

```text
PRESENT(value) -> compare
ABSENT         -> FALSE
UNRESOLVED     -> INDETERMINATE
```

`ABSENT` produces FALSE for **all** Compare operators, including `ne`. Missing is not treated as a value unequal to every literal.

## 7. Membership predicate

### Normalized form

```yaml
predicate: membership
ref:
  key: cfg:eedf_model
scalar_kind: string
values:
  - boltzmann_linear
  - boltzmann_quadratic
```

Rules:

- values are homogeneous according to `scalar_kind`;
- numeric values use ADR-0021 exact-decimal canonicalization;
- normalized values have set semantics and duplicates are removed;
- an empty membership set is valid and always FALSE for a PRESENT value;
- no cross-kind coercion.

Lookup semantics:

```text
PRESENT(value) -> TRUE iff value belongs to normalized set
ABSENT         -> FALSE
UNRESOLVED     -> INDETERMINATE
```

## 8. Exists predicate

Normalized form:

```yaml
predicate: exists
ref:
  key: cfg:reduced_electric_field
```

Semantics:

```text
PRESENT(_) -> TRUE
ABSENT     -> FALSE
UNRESOLVED -> INDETERMINATE
```

`Exists` tests reference presence, not truthiness. Therefore present values `false`, `0`, and empty string still satisfy Exists.

Complex authoring paths, relation-target filters, and collection queries are compiler/domain concerns that must normalize to a deterministic EvaluationReference before this predicate algebra is applied.

## 9. Boolean predicates

Normalized forms:

```yaml
predicate: and
operands: [<Predicate>, <Predicate>, ...]
```

```yaml
predicate: or
operands: [<Predicate>, <Predicate>, ...]
```

```yaml
predicate: not
operand: <Predicate>
```

`and` and `or` require at least one operand. Empty Boolean operand lists are invalid in v0.1.

Predicate truth domain:

```text
TRUE
FALSE
INDETERMINATE
```

Use strong-Kleene three-valued semantics.

### NOT

```text
NOT TRUE          = FALSE
NOT FALSE         = TRUE
NOT INDETERMINATE = INDETERMINATE
```

### AND

```text
if any operand FALSE -> FALSE
else if any operand INDETERMINATE -> INDETERMINATE
else -> TRUE
```

### OR

```text
if any operand TRUE -> TRUE
else if any operand INDETERMINATE -> INDETERMINATE
else -> FALSE
```

Thus a decisive FALSE in `AND` or decisive TRUE in `OR` determines the result even if another operand is unresolved.

## 10. Conditional activation

Evaluate `if` first.

```text
TRUE
  -> activate every ordinary constraint in then

FALSE
  -> activate none of them

INDETERMINATE
  -> activate none; Conditional activation state = INDETERMINATE
```

An INDETERMINATE predicate SHALL NOT be guessed FALSE and SHALL NOT activate its consequent speculatively.

## 11. Active-constraint composition

After all Conditional predicates are evaluated:

1. collect all static ordinary constraints;
2. add every consequent from predicates that evaluated TRUE;
3. intersect/compose those ordinary constraints according to their accepted family contracts;
4. predicates that evaluated FALSE contribute nothing;
5. predicates that evaluated INDETERMINATE contribute no consequent but preserve incomplete activation state.

If active Conditional consequents make an otherwise satisfiable configuration contradictory, ADR-0007 classifies the contradiction as a **Configuration Conflict**.

## 12. Final validation-state precedence

Conditional activation incompleteness must not hide a definite failure.

For the design-stage validator:

```text
FAIL > INDETERMINATE > PASS
```

- if any static/active ordinary constraint definitely fails, overall validation is FAIL;
- otherwise, if any Conditional predicate is INDETERMINATE, overall validation is INDETERMINATE;
- otherwise, validation may be PASS if all ordinary constraints pass.

This is an evaluation-completeness rule, not a new logical Constraint family.

## 13. Conflict versus evaluation incompleteness

```text
Schema Conflict
    = static ontology/interface/profile semantic constraints are unsatisfiable

Configuration Conflict
    = constraints activated by the current configuration are unsatisfiable

INDETERMINATE
    = predicate/reference semantic information is insufficient to decide activation
```

`INDETERMINATE` is not a conflict.

Backend/Profile non-representability remains outside this axis.

## 14. Proposed authoring predicate syntax

```text
Compare:
{
  predicate: compare,
  ref: string,
  op: eq|ne|lt|le|gt|ge,
  value: number|string|boolean
}

Membership:
{
  predicate: membership,
  ref: string,
  values: [homogeneous scalar...]
}

Exists:
{
  predicate: exists,
  ref: string
}

And/Or:
{
  predicate: and|or,
  operands: [Predicate, ...]
}

Not:
{
  predicate: not,
  operand: Predicate
}
```

The authoring compiler infers scalar kind from the authored literal/set and captures numeric lexemes losslessly before normalized exact-decimal construction.

## 15. Proposed normalized predicate syntax

Normalized Compare/Membership explicitly carries `scalar_kind` and `EvaluationReference`.

```text
Compare:
predicate, ref{key}, scalar_kind, op, value

Membership:
predicate, ref{key}, scalar_kind, values

Exists:
predicate, ref{key}

Boolean:
recursive normalized Predicate operands
```

The normalized syntax does not carry backend identifiers or native UI activation metadata.

## 16. Minimal schema slice after acceptance

Add:

- `schema/predicate-authoring-v0.1.schema.json`
- `schema/predicate-normalized-v0.1.schema.json`
- `schema/constraint-conditional-authoring-v0.1.schema.json`
- `schema/constraint-conditional-normalized-v0.1.schema.json`

The Conditional schema should reference the already accepted ordinary Constraint schemas for `then` rather than embedding duplicate family definitions.

A semantic helper/test should cover:

- exact numeric comparison;
- missing versus unresolved lookup;
- Exists false/zero truthiness boundary;
- Membership set semantics;
- three-valued Boolean truth tables;
- Conditional TRUE/FALSE/INDETERMINATE activation;
- no nested Conditional in `then`;
- active consequent -> Configuration Conflict boundary;
- `FAIL > INDETERMINATE > PASS` precedence;
- backend-runtime independence.

## 17. Backend evidence

- MOOSE exposes `isParamValid` and `isParamSetByUser`, demonstrating real validity logic dependent on whether controlling inputs are present/set.
- COMSOL Physics Builder provides Activation Condition, Additional Requirement, Boolean expressions, and `isActive`-style conditions on user inputs.
- Ansys System Coupling expressions support conditional and Boolean/logical statements.

SOL normalizes only the semantic applicability logic; backend UI/runtime expression languages are not copied into Core.

## 18. Non-goals / deferred

Not included in v0.1:

- `else` branch;
- nested Conditional consequents;
- regex/string-pattern predicates;
- approximate/tolerance compare;
- collection quantifiers `all/any/none`;
- aggregate predicates count/sum/average;
- temporal predicates;
- transitive graph-path predicates;
- a universal graph selector language;
- backend-native activation metadata.

## 19. Required boundary cases

1. `0` present -> Exists TRUE;
2. `false` present -> Exists TRUE;
3. ABSENT -> Exists FALSE;
4. UNRESOLVED -> Exists INDETERMINATE;
5. ABSENT `ne x` -> FALSE;
6. unresolved compare -> INDETERMINATE;
7. numeric `1`, `1.0`, `1e0` normalize equal;
8. string `lt` -> normalization/type failure;
9. empty Membership set + PRESENT -> FALSE;
10. FALSE AND INDETERMINATE -> FALSE;
11. TRUE OR INDETERMINATE -> TRUE;
12. NOT INDETERMINATE -> INDETERMINATE;
13. predicate TRUE -> consequent active;
14. predicate FALSE -> consequent inactive;
15. predicate INDETERMINATE -> no consequent + activation INDETERMINATE;
16. nested Conditional in `then` -> structural failure;
17. active consequent contradiction -> Configuration Conflict;
18. definite ordinary FAIL + predicate INDETERMINATE -> overall FAIL;
19. no FAIL + predicate INDETERMINATE -> overall INDETERMINATE;
20. backend runtime/license absent -> no effect on predicate truth.

## 20. Research verdict

**Ready for independent contract Validation.**
