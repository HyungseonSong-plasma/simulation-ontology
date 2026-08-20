# SOL v0.1 ADR-0020 Dimension Implementation Review v0.1

**Role:** Validation  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`

## Inputs

- `docs/decisions/0020-dimension-constraint-and-canonical-dimensionvector.md`
- `schema/constraint-dimension-authoring-v0.1.schema.json`
- `schema/constraint-dimension-normalized-v0.1.schema.json`
- `tests/constraint_dimension_semantics.py`
- `tests/test_constraint_dimension_schema.py`
- `tests/constraint-dimension-test-results.txt`

Research process/conclusion was not used as authority.

## Findings

### DIV-01 — Sparse/full vector contract

**Resolved.** Authoring and normalized schemas match ADR-0020: sparse authoring, explicit required `vector`, full seven-axis normalized representation, and unknown-axis rejection are represented deterministically.

### DIV-02 — Explicit DimensionOne

**Resolved.** `vector: {}` is valid authoring and expands to seven zeros. Missing `vector` is structurally invalid and is not interpreted as DimensionOne.

### DIV-03 — Equality/intersection

**Resolved.** The helper normalizes both vectors before equality comparison, so sparse form and key order cannot change the result.

### DIV-04 — Backend/metrology independence

**Resolved.** Unit/backend metadata is absent from the semantic payload/helper interface and cannot silently alter DimensionVector equality.

### DIV-05 — JSON Schema integer / helper mismatch

**Unresolved — Validation-tooling defect.**

Draft 2020-12 JSON Schema `type: integer` accepts a JSON number whose mathematical value is integral, including lexical/decoded values such as `1.0` and `-2.0`. The current helper instead requires Python `int` and rejects any `float`:

```python
if isinstance(exponent, bool) or not isinstance(exponent, int):
    raise DimensionConstraintError("DIMENSION_EXPONENT_INVALID")
```

Therefore an input can be structurally schema-valid but fail semantic normalization solely because one implementation parsed an integral JSON number as floating-point.

Counterexample:

```yaml
type: dimension
vector:
  length: 1.0
```

Expected under the structural schema: integer-valued number, PASS.
Current helper: `DIMENSION_EXPONENT_INVALID`.

This violates independent-validator agreement.

## Defect classification

- Architecture defect: none.
- Contract defect: none.
- Validation-tooling defect: **DIV-05**.
- Backend/runtime limitation: out of scope.

## Verdict

**Revise.**

Do not reopen ADR-0020. Align the helper with JSON Schema integer-value semantics and add regression tests for integral floating representations (`1.0`, `-2.0`) versus genuinely non-integer numbers (`1.5`, `-0.5`) and booleans.

After that focused fix, perform a short implementation re-review.
