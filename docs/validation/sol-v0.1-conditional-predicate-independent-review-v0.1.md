# SOL v0.1 Conditional / Predicate Independent Review v0.1

**Status:** Validation finding  
**Date:** 2026-08-20  
**Target:** `docs/research/sol-v0.1-conditional-predicate-schema-proposal-v0.1.md`  
**Normative basis:** ADR-0007, ADR-0018 through ADR-0022

## Validation method

The proposal was reviewed independently as a logical/evaluation contract. Official MOOSE, COMSOL, and Ansys documentation was used only to confirm that condition-dependent validity and activation are real cross-backend concerns; native condition syntax was not treated as SOL semantics.

The review tested whether two validators given the same normalized model snapshot are forced to produce the same predicate truth, activation set, and validation state.

## Overall verdict

**Revise**

The four-predicate vocabulary, Conditional-as-meta-constraint model, three-valued Boolean direction, no-nested-Conditional boundary, and semantic/backend separation are viable. Five determinism gaps remain.

## COND-01 — EvaluationReference binding cardinality is unspecified

**Classification:** Contract determinism defect  
**Severity:** Required revision

The proposal defines normalized:

```yaml
ref:
  key: cfg:mode
```

and assumes the evaluation context supplies deterministic lookup, but does not require each key to bind to exactly one evaluation access contract in the normalized snapshot.

Counterexample:

```text
key = cfg:mode
```

Validator A resolves the key to one local parameter slot. Validator B sees two candidate bindings and chooses one by storage order. Both can claim to implement `lookup(key)`.

Required contract:

```text
EvaluationReference key -> exactly one evaluation binding
zero matches     -> normalization/reference failure
multiple matches -> normalization/reference ambiguity failure
```

The binding table/storage syntax may remain deferred to canonical package integration, but cardinality and ambiguity behavior cannot be deferred.

## COND-02 — ABSENT Compare semantics breaks `ne = NOT eq`

**Classification:** Predicate logic defect  
**Severity:** Required revision

The proposal assigns:

```text
ABSENT -> FALSE
```

for every Compare operator, including both `eq` and `ne`.

Therefore for absent `x`:

```text
eq(x, v)      = FALSE
ne(x, v)      = FALSE
NOT eq(x, v)  = TRUE
```

so:

```text
ne(x,v) != NOT eq(x,v)
```

This conflicts with the accepted Boolean vocabulary and with the proposal's statement that an `else` can be represented using a logically negated predicate.

A smaller consistent rule is:

```text
PRESENT    -> evaluate
ABSENT     -> INDETERMINATE for Compare/Membership
UNRESOLVED -> INDETERMINATE
```

Presence-sensitive authoring should use `Exists`. Then:

```text
Exists(x) AND eq(x,v)
```

is FALSE when x is absent because strong-Kleene `FALSE AND INDETERMINATE = FALSE`.

This preserves explicit presence semantics without making missing data a scalar value.

## COND-03 — Empty Membership authoring cannot infer scalar_kind

**Classification:** Authoring/normalization contract defect  
**Severity:** Required revision

The proposal permits an empty Membership set and says the authoring compiler infers scalar kind from authored values.

For:

```yaml
predicate: membership
ref: mode
values: []
```

there is no value from which to infer `number | string | boolean`.

Two validators can normalize the same input to different scalar kinds while all runtime PRESENT values still yield FALSE.

Required fix: authoring Membership must carry an explicit `scalar_kind`, or at minimum require one whenever `values` is empty. Requiring it uniformly is simpler and aligns authoring/normalized semantics.

## COND-04 — PRESENT scalar-kind mismatch outcome is undefined

**Classification:** Evaluation contract defect  
**Severity:** Required revision

Normalized Compare/Membership carries `scalar_kind`, but lookup may return a PRESENT value whose actual kind differs.

Example:

```text
predicate scalar_kind = number
lookup(ref) = PRESENT("1")  # string
```

The proposal prohibits cross-kind coercion but does not say whether this is FALSE, INDETERMINATE, or validation failure.

The outcome must be unique. Recommended minimal rule:

```text
PRESENT value kind != declared scalar_kind
    -> predicate evaluation failure
       PREDICATE_SCALAR_KIND_MISMATCH
```

This is not a Boolean FALSE and not unresolved information; the snapshot contains a definite value violating the normalized predicate contract.

## COND-05 — Final PASS wording remains optional

**Classification:** Verdict determinism defect  
**Severity:** Required revision

Section 12 states:

> otherwise, validation **may be PASS** if all ordinary constraints pass.

Given the already-declared precedence:

```text
FAIL > INDETERMINATE > PASS
```

this should be normative rather than optional.

Required rule:

```text
if any definite ordinary failure -> FAIL
else if any Conditional activation INDETERMINATE -> INDETERMINATE
else if all active/static ordinary constraints PASS -> PASS
```

An empty active Conditional set and no ordinary failures therefore does not create a separate Conditional state.

## Accepted parts / no regression

Do not reopen the following during revision:

1. Conditional is a meta-constraint rather than a seventh ordinary semantic axis.
2. `then` contains only Cardinality/Type/Value/Dimension/Compatibility in v0.1.
3. nested Conditional consequents are excluded in v0.1.
4. no `else` field is required.
5. Predicate vocabulary remains Compare, Membership, Exists, Boolean(and/or/not).
6. normalized numeric predicate values reuse ADR-0021 exact-decimal semantics.
7. no cross-kind scalar coercion.
8. Exists tests presence rather than truthiness.
9. truth domain is TRUE/FALSE/INDETERMINATE.
10. strong-Kleene NOT/AND/OR direction is accepted.
11. TRUE activates consequents; FALSE activates none; INDETERMINATE activates none and preserves incomplete activation state.
12. active consequent contradictions are Configuration Conflicts.
13. definite FAIL must dominate activation INDETERMINATE.
14. backend installation/license/runtime availability is outside predicate truth.
15. a universal graph-path language remains out of scope.

## Additional serialization clarification

Membership `values` has mathematical set semantics. Array order should be declared non-semantic in the normalized payload; implementations may choose a stable presentation order but semantic equality and membership must not depend on it.

This clarification can be included in the focused revision without adding a new finding category.

## Required revision scope

Research should revise only:

- EvaluationReference zero/one/many binding behavior;
- ABSENT Compare/Membership semantics;
- Membership `scalar_kind` authoring rule;
- PRESENT scalar-kind mismatch diagnostic;
- mandatory final PASS rule;
- normalized Membership array order as non-semantic.

No new predicate family, path language, backend condition model, collection quantifier, or nested Conditional support is required.

## Final verdict

**Revise** — focused contract revision only.
