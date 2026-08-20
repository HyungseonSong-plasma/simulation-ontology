# ADR-0023: Conditional Constraint and Predicate Evaluation Contract

**Status:** Accepted  
**Date:** 2026-08-20  
**Supplements:** ADR-0007, ADR-0018, ADR-0019, ADR-0020, ADR-0021, ADR-0022

## Context

ADR-0007 defines Conditional/Implication as the sixth SOL v0.1 Constraint family and fixes a minimal domain-neutral Predicate vocabulary. After the five ordinary Constraint families were consolidated, Conditional still required deterministic machine semantics for reference binding, missing versus unresolved values, scalar comparisons, three-valued logic, evaluation failures, consequent activation, and final validation aggregation.

Independent review of Research proposals v0.1 through v0.2.1 closed seven contract ambiguities before this ADR was accepted.

## Decision

### 1. Conditional is a meta-constraint

SOL v0.1 Conditional activates ordinary constraints; it does not duplicate their semantics.

```yaml
type: conditional
if: <Predicate>
then:
  - <ordinary Constraint>
```

`then` SHALL contain `1..*` constraints from exactly these five families:

```text
Cardinality
Type
Value
Dimension
Compatibility
```

Nested Conditional consequents are invalid in v0.1.

No `else` field is defined. An alternative branch may be authored as a separate Conditional with an appropriate negated predicate when its information state permits that logic.

### 2. Predicate vocabulary

SOL v0.1 supports only:

```text
Compare      eq, ne, lt, le, gt, ge
Membership
Exists
Boolean      and, or, not
```

Regex, tolerance/approximate comparison, collection quantifiers, aggregates, temporal predicates, and general graph-query predicates remain deferred.

### 3. EvaluationReference

Authoring may use compact local/domain references. Before predicate evaluation, the compiler SHALL resolve each authoring reference to a normalized EvaluationReference:

```yaml
ref:
  key: cfg:mode
```

`EvaluationReference` is a normalized validation handle, not a new ontology Entity and not backend-local identity.

Within the normalized model snapshot/evaluation context, every referenced key SHALL bind to exactly one evaluation binding:

```text
0 bindings  -> PREDICATE_REFERENCE_UNRESOLVED
1 binding   -> valid
>1 bindings -> PREDICATE_REFERENCE_AMBIGUOUS
```

Zero/multiple bindings are normalization/reference failures and do not produce predicate truth values.

The eventual package representation of the binding table is deferred to canonical package integration; its cardinality semantics are fixed here.

### 4. Lookup result domain

A valid EvaluationReference lookup in a closed resolved configuration snapshot produces exactly one of:

```text
PRESENT(value)
ABSENT
UNRESOLVED
```

`ABSENT` means the current snapshot definitively lacks a value/target for a valid binding. `UNRESOLVED` means semantic information required to decide presence/value is incomplete.

Backend installation, license, runtime, module, release, or adapter availability SHALL NOT create Core predicate `UNRESOLVED`.

### 5. Scalar domain

Predicate scalar kinds are:

```text
number | string | boolean
```

Normalized numeric values reuse ADR-0021 exact-decimal representation. Native binary floating-point values are not semantic authority. Cross-kind coercion is forbidden.

### 6. Compare

Normalized Compare carries:

```text
predicate, ref{key}, scalar_kind, op, value
```

Operators:

```text
eq, ne             -> number|string|boolean
lt, le, gt, ge     -> number only
```

Evaluation:

```text
PRESENT(correct kind) -> evaluate
PRESENT(wrong kind)   -> EvaluationFailure(PREDICATE_SCALAR_KIND_MISMATCH)
ABSENT                -> INDETERMINATE / PREDICATE_VALUE_ABSENT
UNRESOLVED            -> INDETERMINATE / PREDICATE_VALUE_UNRESOLVED
```

In particular, missing data is not a scalar value:

```text
eq(absent,v) = INDETERMINATE
ne(absent,v) = INDETERMINATE
```

Presence-sensitive rules SHOULD use Exists explicitly.

### 7. Membership

Authoring and normalized Membership SHALL include explicit `scalar_kind`, including when the value set is empty.

```yaml
predicate: membership
ref: mode
scalar_kind: string
values: [basic, advanced]
```

Rules:

- values are homogeneous according to `scalar_kind`;
- normalized numbers use ADR-0021 exact-decimal semantics;
- normalized values have mathematical set semantics;
- duplicates are removed;
- array order is non-semantic;
- empty Membership set is valid.

Evaluation:

```text
PRESENT(correct kind) -> TRUE iff value is in set
PRESENT(wrong kind)   -> EvaluationFailure(PREDICATE_SCALAR_KIND_MISMATCH)
ABSENT                -> INDETERMINATE / PREDICATE_VALUE_ABSENT
UNRESOLVED            -> INDETERMINATE / PREDICATE_VALUE_UNRESOLVED
```

For an empty set and PRESENT correct-kind value, result is FALSE.

### 8. Exists

Exists tests presence, not truthiness:

```text
PRESENT(_) -> TRUE
ABSENT     -> FALSE
UNRESOLVED -> INDETERMINATE / PREDICATE_REFERENCE_VALUE_UNRESOLVED
```

Therefore PRESENT `false`, numeric zero, and empty string all satisfy Exists.

### 9. Predicate result model

A predicate node evaluation produces exactly one of:

```text
Truth(TRUE | FALSE | INDETERMINATE)
EvaluationFailure(code/evidence)
```

`EvaluationFailure` is not a fourth truth value.

### 10. Boolean truth semantics

When every operand evaluates successfully to a truth value, SOL uses strong-Kleene semantics.

NOT:

```text
TRUE -> FALSE
FALSE -> TRUE
INDETERMINATE -> INDETERMINATE
```

AND:

```text
any FALSE -> FALSE
else any INDETERMINATE -> INDETERMINATE
else TRUE
```

OR:

```text
any TRUE -> TRUE
else any INDETERMINATE -> INDETERMINATE
else FALSE
```

`and` and `or` require at least one operand.

### 11. EvaluationFailure dominates Boolean truth reduction

Every Boolean operand SHALL be evaluated sufficiently to detect EvaluationFailure before a Boolean truth value is finalized.

```text
if any child -> EvaluationFailure:
    Boolean node -> EvaluationFailure
else:
    apply strong-Kleene truth reduction
```

Thus:

```text
FALSE AND INDETERMINATE       -> FALSE
TRUE OR INDETERMINATE         -> TRUE
FALSE AND EvaluationFailure X -> EvaluationFailure X
TRUE OR EvaluationFailure X   -> EvaluationFailure X
NOT EvaluationFailure X       -> EvaluationFailure X
```

Short-circuit evaluation SHALL NOT hide an evaluation failure.

When multiple child failures occur, all diagnostics/evidence SHALL be preserved as an order-independent set; declaration order does not choose one semantic failure.

### 12. Conditional activation

```text
predicate TRUE
    -> activate every ordinary constraint in then

predicate FALSE
    -> activate none

predicate INDETERMINATE
    -> activate none and preserve activation INDETERMINATE

predicate EvaluationFailure
    -> overall validation FAIL with predicate diagnostic/evidence
```

An INDETERMINATE predicate is neither guessed FALSE nor speculatively TRUE.

### 13. Active constraint composition

Validation proceeds conceptually:

1. resolve EvaluationReference bindings;
2. evaluate Conditional predicates;
3. collect static ordinary constraints;
4. add consequents only from TRUE predicates;
5. compose/intersect ordinary constraints according to their accepted family contracts;
6. classify contradictions introduced by active consequents as Configuration Conflicts;
7. aggregate final validation state.

### 14. Final validation-state precedence

The design-stage validator uses:

```text
FAIL > INDETERMINATE > PASS
```

Normatively:

```text
if any reference/predicate evaluation failure or definite ordinary-constraint failure:
    FAIL
else if any Conditional activation is INDETERMINATE:
    INDETERMINATE
else:
    PASS
```

`PASS` is the mandatory identity outcome when no failure or incompleteness remains.

### 15. Conflict versus incompleteness

```text
Schema Conflict
    = static semantic contracts are unsatisfiable

Configuration Conflict
    = active Conditional consequents make the current configuration unsatisfiable

INDETERMINATE
    = semantic information is insufficient to decide one or more activations
```

Backend/Profile representability remains a separate evaluation axis.

## Schema implementation slice

The design-stage slice SHALL include:

- `schema/predicate-authoring-v0.1.schema.json`;
- `schema/predicate-normalized-v0.1.schema.json`;
- `schema/constraint-conditional-authoring-v0.1.schema.json`;
- `schema/constraint-conditional-normalized-v0.1.schema.json`;
- semantic helper/tests for lookup states, exact numeric comparison, set semantics, Boolean truth/failure behavior, activation, and final state precedence.

Conditional schemas SHALL reference accepted ordinary Constraint schemas rather than duplicating their definitions.

No backend runtime/license or production Adapter is required.

## Consequences

### Positive

- all six ADR-0007 Constraint families now have deterministic contract boundaries;
- missing and unresolved values cannot be confused with scalar FALSE/inequality;
- presence semantics are explicit through Exists;
- Boolean logic is stable under partial information;
- short-circuit implementation cannot hide malformed predicate data;
- Conditional activation and ordinary Constraint semantics remain orthogonal.

### Limits / deferred

- EvaluationReference binding-table serialization remains part of canonical package integration;
- no nested Conditional or else field in v0.1;
- no general graph-path/query language;
- no regex, tolerance, temporal, aggregate, or collection-quantifier predicates.

## Validation evidence

- `docs/research/sol-v0.1-conditional-predicate-schema-proposal-v0.1.md`
- `docs/validation/sol-v0.1-conditional-predicate-independent-review-v0.1.md`
- `docs/research/sol-v0.1-conditional-predicate-schema-proposal-v0.2.md`
- `docs/validation/sol-v0.1-conditional-predicate-v0.2-focused-re-review.md`
- `docs/research/sol-v0.1-conditional-predicate-schema-proposal-v0.2.1.md`
- `docs/validation/sol-v0.1-conditional-predicate-final-contract-review.md`

## Decision summary

SOL v0.1 Conditional is a non-nested implication meta-constraint driven by a minimal four-family Predicate algebra over exact-one resolved EvaluationReferences, three-valued truth plus separate EvaluationFailure, strong-Kleene Boolean semantics, and deterministic `FAIL > INDETERMINATE > PASS` validation aggregation.
