# SOL v0.1 Conditional / Predicate Schema Proposal v0.2

**Status:** Focused Research revision  
**Date:** 2026-08-20  
**Revision input:** `docs/validation/sol-v0.1-conditional-predicate-independent-review-v0.1.md`

## 1. Preserved contract

The following v0.1 direction is unchanged:

- Conditional is a meta-constraint;
- `then` contains only Cardinality/Type/Value/Dimension/Compatibility;
- nested Conditional consequents are forbidden;
- no `else` field;
- predicate vocabulary is Compare, Membership, Exists, Boolean(and/or/not);
- normalized numbers reuse ADR-0021 exact-decimal semantics;
- truth domain is TRUE/FALSE/INDETERMINATE;
- Boolean logic uses strong-Kleene semantics;
- backend runtime/license state does not affect Core predicate truth.

Only COND-01 through COND-05 and the Membership-order clarification are revised.

## 2. EvaluationReference binding contract

Normalized predicate references retain:

```yaml
ref:
  key: cfg:mode
```

The normalized model snapshot/evaluation context SHALL bind every referenced key to exactly one evaluation binding.

```text
0 matching bindings
    -> PREDICATE_REFERENCE_UNRESOLVED

1 matching binding
    -> valid EvaluationReference

>1 matching bindings
    -> PREDICATE_REFERENCE_AMBIGUOUS
```

Zero/multiple binding results are normalization/reference failures and no predicate truth value is produced.

The storage syntax of the binding table remains part of later canonical package integration. Its cardinality/ambiguity semantics are fixed here.

This resolves COND-01.

## 3. Lookup result after successful binding

A valid EvaluationReference lookup in a closed resolved configuration snapshot returns:

```text
PRESENT(value)
ABSENT
UNRESOLVED
```

- PRESENT: a definite scalar/presence value exists;
- ABSENT: the binding is valid but the current snapshot definitively has no value/target;
- UNRESOLVED: the binding exists but required semantic information is incomplete.

## 4. Compare semantics

Normalized Compare:

```yaml
predicate: compare
ref: { key: cfg:mode }
scalar_kind: string
op: eq
value: advanced
```

Operators:

```text
eq, ne                  -> number|string|boolean
lt, le, gt, ge          -> number only
```

No cross-kind coercion.

Lookup/evaluation:

```text
PRESENT(value of declared scalar_kind)
    -> evaluate operator

PRESENT(value of different scalar kind)
    -> evaluation failure
       PREDICATE_SCALAR_KIND_MISMATCH

ABSENT
    -> INDETERMINATE
       PREDICATE_VALUE_ABSENT

UNRESOLVED
    -> INDETERMINATE
       PREDICATE_VALUE_UNRESOLVED
```

Thus for missing x:

```text
eq(x,v) = INDETERMINATE
ne(x,v) = INDETERMINATE
NOT eq(x,v) = INDETERMINATE
```

Presence-sensitive rules should use `Exists` explicitly.

This resolves COND-02 and COND-04 for Compare.

## 5. Membership semantics

Authoring Membership SHALL include explicit `scalar_kind`:

```yaml
predicate: membership
ref: eedf_model
scalar_kind: string
values:
  - boltzmann_linear
  - boltzmann_quadratic
```

This field is required even when `values` is nonempty. The rule avoids a second authoring mode and makes an empty set unambiguous.

Normalized Membership uses the same field and normalized scalar representations.

```text
PRESENT(value of declared scalar_kind)
    -> TRUE iff value is in normalized set

PRESENT(value of different scalar kind)
    -> evaluation failure
       PREDICATE_SCALAR_KIND_MISMATCH

ABSENT
    -> INDETERMINATE
       PREDICATE_VALUE_ABSENT

UNRESOLVED
    -> INDETERMINATE
       PREDICATE_VALUE_UNRESOLVED
```

An empty normalized Membership set is valid. For a PRESENT value of the declared kind it evaluates FALSE.

Normalized Membership `values` has mathematical set semantics:

- duplicates are removed;
- array order is non-semantic;
- implementations MAY choose a stable presentation order, but predicate equality/membership SHALL NOT depend on order.

This resolves COND-03 and the serialization clarification.

## 6. Exists semantics

Exists remains the explicit presence predicate:

```text
PRESENT(_) -> TRUE
ABSENT     -> FALSE
UNRESOLVED -> INDETERMINATE / PREDICATE_REFERENCE_VALUE_UNRESOLVED
```

Present `false`, numeric zero, and empty string remain TRUE for Exists.

Therefore a common optional-slot guard can be written:

```text
Exists(x) AND Compare(x, eq, v)
```

If x is ABSENT:

```text
Exists(x) = FALSE
Compare(x,eq,v) = INDETERMINATE
FALSE AND INDETERMINATE = FALSE
```

so the Conditional is deterministically inactive rather than indeterminate.

## 7. Boolean semantics

Truth domain remains:

```text
TRUE | FALSE | INDETERMINATE
```

Strong-Kleene rules remain unchanged.

### NOT

```text
TRUE -> FALSE
FALSE -> TRUE
INDETERMINATE -> INDETERMINATE
```

### AND

```text
any FALSE -> FALSE
else any INDETERMINATE -> INDETERMINATE
else TRUE
```

### OR

```text
any TRUE -> TRUE
else any INDETERMINATE -> INDETERMINATE
else FALSE
```

`and`/`or` authoring and normalized arrays require at least one operand.

## 8. Conditional activation

```text
predicate TRUE
    -> activate all ordinary constraints in then

predicate FALSE
    -> activate none

predicate INDETERMINATE
    -> activate none; preserve activation INDETERMINATE

predicate evaluation failure
    -> validation FAIL with predicate diagnostic
```

An INDETERMINATE predicate is neither guessed FALSE nor speculatively TRUE.

## 9. Active constraint composition

Evaluation order:

1. resolve every EvaluationReference binding;
2. evaluate Conditional predicates;
3. collect static ordinary constraints;
4. add consequents only from TRUE predicates;
5. compose/intersect ordinary constraints using their accepted family contracts;
6. classify contradictions caused by active consequents as Configuration Conflict;
7. aggregate validation state.

Nested Conditional in `then` remains structurally invalid.

## 10. Final validation-state rule

The v0.1 design-stage validator uses:

```text
FAIL > INDETERMINATE > PASS
```

Normatively:

```text
if any predicate evaluation failure or definite ordinary-constraint failure:
    overall = FAIL
else if any Conditional activation == INDETERMINATE:
    overall = INDETERMINATE
else:
    overall = PASS
```

`PASS` is mandatory in the final branch; it is not optional wording.

This resolves COND-05.

## 11. Conflict and incompleteness remain orthogonal

```text
Schema Conflict
    = static semantic contracts unsatisfiable

Configuration Conflict
    = active Conditional consequents make current configuration unsatisfiable

INDETERMINATE
    = semantic information insufficient to decide one or more predicate activations
```

A predicate evaluation failure such as `PREDICATE_SCALAR_KIND_MISMATCH` is a validation failure, not an INDETERMINATE truth value.

Backend/Profile representability remains outside this axis.

## 12. Authoring syntax

### Compare

```yaml
predicate: compare
ref: mode
op: eq
value: advanced
```

Compiler infers `scalar_kind` from the single authored value.

### Membership

```yaml
predicate: membership
ref: mode
scalar_kind: string
values: [basic, advanced]
```

`scalar_kind` is mandatory.

### Exists

```yaml
predicate: exists
ref: auxiliary_field
```

### Boolean

```yaml
predicate: and
operands: [<Predicate>, ...]
```

```yaml
predicate: or
operands: [<Predicate>, ...]
```

```yaml
predicate: not
operand: <Predicate>
```

## 13. Normalized syntax

Normalized Compare and Membership include:

```text
ref: { key }
scalar_kind
normalized scalar value(s)
```

Normalized numeric values use ADR-0021 exact decimal.

Normalized Exists includes only `predicate` and resolved `ref`.

Boolean nodes recursively contain normalized Predicate nodes.

## 14. Boundary cases

1. zero EvaluationReference bindings -> `PREDICATE_REFERENCE_UNRESOLVED` failure;
2. multiple bindings -> `PREDICATE_REFERENCE_AMBIGUOUS` failure;
3. PRESENT 0 -> Exists TRUE;
4. PRESENT false -> Exists TRUE;
5. ABSENT -> Exists FALSE;
6. UNRESOLVED -> Exists INDETERMINATE;
7. ABSENT Compare eq/ne -> INDETERMINATE;
8. ABSENT Membership -> INDETERMINATE;
9. PRESENT wrong scalar kind -> `PREDICATE_SCALAR_KIND_MISMATCH` failure;
10. empty authored Membership with explicit scalar_kind -> valid;
11. empty Membership + PRESENT correct-kind value -> FALSE;
12. Membership array permutations -> semantically equal;
13. numeric `1`, `1.0`, `1e0` -> same normalized number;
14. string `lt` -> normalization/type failure;
15. FALSE AND INDETERMINATE -> FALSE;
16. TRUE OR INDETERMINATE -> TRUE;
17. NOT INDETERMINATE -> INDETERMINATE;
18. TRUE Conditional -> consequents active;
19. FALSE Conditional -> consequents inactive;
20. INDETERMINATE Conditional -> no consequents + activation INDETERMINATE;
21. predicate evaluation failure -> overall FAIL;
22. active consequent contradiction -> Configuration Conflict;
23. definite ordinary FAIL + activation INDETERMINATE -> overall FAIL;
24. no FAIL + activation INDETERMINATE -> overall INDETERMINATE;
25. all predicates determinate + all ordinary constraints PASS -> overall PASS;
26. backend runtime/license absent -> no effect on predicate truth.

## 15. Finding closure matrix

| Finding | Status | Resolution |
|---|---|---|
| COND-01 EvaluationReference binding | Resolved | exact-one binding contract |
| COND-02 ABSENT Compare logic | Resolved | Compare/Membership ABSENT -> INDETERMINATE |
| COND-03 empty Membership scalar kind | Resolved | authoring scalar_kind mandatory |
| COND-04 PRESENT kind mismatch | Resolved | deterministic evaluation failure diagnostic |
| COND-05 optional PASS wording | Resolved | mandatory final PASS branch |
| Membership array order clarification | Resolved | set semantics, order non-semantic |

**Contract-level unresolved:** none proposed.  
**Regression:** none proposed.

## 16. Research verdict

**Ready for focused contract re-review.**
