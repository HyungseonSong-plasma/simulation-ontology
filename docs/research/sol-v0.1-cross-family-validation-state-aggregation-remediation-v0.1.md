# SOL v0.1 Cross-Family Validation-State Aggregation Remediation v0.1

**Status:** Focused Research proposal  
**Date:** 2026-08-20  
**Input:** COND-08 from `docs/validation/sol-v0.1-adr0023-cross-family-indeterminate-preflight-review.md`  
**Scope:** Integration of ADR-0022 ordinary-family `INDETERMINATE` with ADR-0023 Conditional activation state

## 1. Problem

ADR-0022 permits Compatibility-family evaluation to produce `INDETERMINATE`. ADR-0023 final aggregation mentions definite ordinary failures and Conditional activation `INDETERMINATE`, but does not explicitly propagate ordinary-family `INDETERMINATE`.

No Predicate, Compatibility, or backend semantics need redesign.

## 2. Focused rule

For the common design-stage state slice:

```text
PASS | FAIL | INDETERMINATE
```

final validation aggregation is conjunctive and uses:

```text
FAIL > INDETERMINATE > PASS
```

Inputs to this aggregation include:

1. completed static ordinary-family validation states;
2. completed validation states of ordinary constraints activated by TRUE Conditional predicates;
3. Conditional activation states;
4. predicate/reference EvaluationFailure projected to `FAIL` with its diagnostics/evidence.

Normatively:

```text
if any predicate/reference EvaluationFailure
   or any ordinary-family state == FAIL:
    overall = FAIL
else if any ordinary-family state == INDETERMINATE
     or any Conditional activation == INDETERMINATE:
    overall = INDETERMINATE
else:
    overall = PASS
```

The empty input set therefore has identity `PASS` for this state slice.

## 3. Examples

```text
ordinary = [PASS, PASS]
conditional_activation = []
-> PASS
```

```text
ordinary = [INDETERMINATE]
conditional_activation = [TRUE/determinate]
-> INDETERMINATE
```

```text
ordinary = [FAIL, INDETERMINATE]
conditional_activation = [INDETERMINATE]
-> FAIL
```

```text
ordinary = [PASS]
conditional_activation = [INDETERMINATE]
-> INDETERMINATE
```

```text
predicate EvaluationFailure + ordinary INDETERMINATE
-> FAIL
```

## 4. Boundary to other states

This focused rule does **not** define a universal validator state lattice beyond the three-state evaluation slice.

In particular, family/invocation contracts that expose a distinct precondition state such as `BLOCKED` retain their existing meaning. Their integration into a future canonical validation envelope is deferred to package/validator integration and SHALL NOT be silently coerced to PASS/FAIL/INDETERMINATE by this rule.

This preserves the design-stage separation between semantic result completeness and invocation/environment prerequisites.

## 5. Conflict classification unchanged

`Schema Conflict` and `Configuration Conflict` remain diagnostic classifications associated with definite unsatisfiable semantic constraints and therefore contribute a definite `FAIL` in this three-state aggregation slice.

`INDETERMINATE` remains non-conflict incompleteness.

## 6. Scope preservation

No changes to:

- ADR-0022 Compatibility criterion/operand/evaluation semantics;
- ADR-0023 Predicate truth semantics;
- EvaluationReference lookup semantics;
- EvaluationFailure precedence;
- Conditional consequent activation;
- backend/Profile representability.

## 7. Required boundary cases

1. static ordinary Compatibility `INDETERMINATE` + all else PASS -> overall INDETERMINATE;
2. ordinary FAIL + ordinary INDETERMINATE -> FAIL;
3. ordinary PASS + Conditional activation INDETERMINATE -> INDETERMINATE;
4. predicate EvaluationFailure + ordinary INDETERMINATE -> FAIL;
5. no states/failures -> PASS;
6. `BLOCKED` from a separate invocation contract is not silently accepted as one of these three states.

## 8. Research verdict

**Ready for focused Validation.**
