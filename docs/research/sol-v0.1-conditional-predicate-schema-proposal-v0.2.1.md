# SOL v0.1 Conditional / Predicate Schema Proposal v0.2.1

**Status:** Focused Research revision  
**Date:** 2026-08-20  
**Base:** v0.2 proposal  
**Revision input:** COND-06 and COND-07 from `docs/validation/sol-v0.1-conditional-predicate-v0.2-focused-re-review.md`

## 1. Scope

All v0.2 contracts remain unchanged. This revision adds only:

1. deterministic precedence for predicate evaluation failure inside Boolean composition;
2. normative nonempty `then` cardinality.

## 2. Predicate evaluation result domain

A predicate node evaluation produces exactly one of:

```text
Truth(TRUE | FALSE | INDETERMINATE)
EvaluationFailure(code)
```

`EvaluationFailure` is not a fourth truth value.

Leaf failures include, for example:

```text
PREDICATE_SCALAR_KIND_MISMATCH
```

Reference binding normalization failures occur before truth evaluation and likewise do not produce a truth value.

## 3. Boolean evaluation failure precedence

Before a Boolean predicate finalizes its strong-Kleene truth value, every operand SHALL be evaluated sufficiently to determine whether it produced `EvaluationFailure`.

Normatively:

```text
if any operand -> EvaluationFailure(code):
    Boolean predicate -> EvaluationFailure(code/evidence)
else:
    apply strong-Kleene truth reduction
```

Therefore a decisive Boolean truth value SHALL NOT short-circuit-mask an evaluation failure.

Examples:

```text
FALSE AND INDETERMINATE -> FALSE
TRUE  OR  INDETERMINATE -> TRUE

FALSE AND EvaluationFailure(X) -> EvaluationFailure(X)
TRUE  OR  EvaluationFailure(X) -> EvaluationFailure(X)
NOT EvaluationFailure(X)       -> EvaluationFailure(X)
```

When multiple operands fail, the validator SHALL preserve all failure diagnostics/evidence as an order-independent set; declaration order SHALL NOT select one failure as the semantic result.

The overall Conditional validation rule remains:

```text
predicate EvaluationFailure -> overall FAIL
```

This closes COND-06 while preserving strong-Kleene truth semantics for successfully evaluated operands.

## 4. Conditional consequent cardinality

A v0.1 Conditional `then` list SHALL contain **one or more** ordinary constraints:

```text
then cardinality = 1..*
```

Allowed consequent families remain exactly:

```text
Cardinality
Type
Value
Dimension
Compatibility
```

Consequent rules:

- `then: []` -> structural failure;
- missing `then` -> structural failure;
- nested Conditional -> structural failure;
- one or more ordinary constraints -> structurally eligible, subject to each referenced family schema/semantic contract.

An empty Conditional is not represented as a valid no-op rule in SOL v0.1.

This closes COND-07.

## 5. Boundary cases added

```text
FALSE AND EvaluationFailure(X) -> EvaluationFailure(X)
TRUE OR EvaluationFailure(X)   -> EvaluationFailure(X)
NOT EvaluationFailure(X)       -> EvaluationFailure(X)
multiple Boolean child failures -> all diagnostics preserved, order-independent
then: []                        -> structural failure
then: [ordinary constraint]     -> valid Conditional shape
```

All v0.2 boundary cases remain required.

## 6. Finding closure

| Finding | Status |
|---|---|
| COND-01 | Resolved in v0.2 |
| COND-02 | Resolved in v0.2 |
| COND-03 | Resolved in v0.2 |
| COND-04 | Resolved in v0.2 |
| COND-05 | Resolved in v0.2 |
| COND-06 Boolean failure masking | Resolved in v0.2.1 |
| COND-07 empty consequent ambiguity | Resolved in v0.2.1 |

**Contract-level unresolved:** none proposed.  
**Regression:** none proposed.

## 7. Research verdict

**Ready for final focused contract re-review.**
