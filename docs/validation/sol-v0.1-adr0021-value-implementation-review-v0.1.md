# SOL v0.1 ADR-0021 Value Implementation Review v0.1

**Role:** Validation  
**Date:** 2026-08-20

## Inputs

- `docs/decisions/0021-scalar-value-constraint-and-exact-decimal-normalization.md`
- `schema/constraint-value-authoring-v0.1.schema.json`
- `schema/constraint-value-normalized-v0.1.schema.json`
- `tests/constraint_value_semantics.py`
- `tests/test_constraint_value_schema.py`

Research process/conclusion was not used as authority.

## Findings

### VIV-01 — Exact decimal lexical normalization

**Resolved.** The helper normalizes JSON-compatible finite decimal lexemes exactly and preserves large adjacent integers that binary64 may collapse. Equivalent decimal lexical forms and signed zero normalize canonically.

### VIV-02 — Interval/set semantics

**Resolved.** Open/closed interval emptiness, exact interval ordering, numeric set deduplication, set-kind separation, and interval/set filtering follow ADR-0021 without host floating equality.

### VIV-03 — Empty result / conflict class

**Resolved.** `empty` remains a satisfiability result and conflict classification is applied separately using contributor/activation context.

### VIV-04 — Comparison-space boundary

**Resolved.** Numeric normalization rejects `unresolved` comparison space before normalized payload emission and does not embed Unit/backend metadata.

### VIV-05 — normalized `exponent10` schema/helper mismatch

**Unresolved — Validation-tooling defect.**

Draft 2020-12 JSON Schema `type: integer` accepts mathematically integral JSON numbers such as `1.0`. The normalized schema therefore accepts:

```yaml
coefficient: "1"
exponent10: 1.0
```

but `_decimal_value()` currently requires Python `int` and rejects the same schema-valid semantic integer.

This can make independent validators disagree solely because of host numeric representation.

Required focused fix:

- accept integer-valued numeric representations for `exponent10` and canonicalize them to an integer before Decimal comparison;
- continue rejecting Boolean and non-integral numeric values;
- add regression tests for `1.0`, `-2.0`, non-integral values, and Boolean.

Do not reopen ADR-0021 or the exact-decimal coefficient/exponent model.

## Defect classification

- Architecture defect: none.
- Contract defect: none.
- Validation-tooling defect: VIV-05.
- Backend/runtime limitation: out of scope.

## Verdict

**Revise.**

Perform only the VIV-05 helper/test correction, then run a focused implementation re-review.
