# SOL v0.1 ADR-0018 Cardinality Schema Slice — Independent Validation

**Role:** Validation  
**Date:** 2026-08-20

## Inputs

Validation consumed only:

- `docs/decisions/0018-constraint-authoring-normalization-and-cardinality-schema-boundary.md`
- `schema/constraint-cardinality-authoring-v0.1.schema.json`
- `schema/constraint-cardinality-normalized-v0.1.schema.json`
- `schema/qrc-v0.1.schema.json`
- `tests/constraint_cardinality_normalizer.py`
- `tests/test_constraint_cardinality_schema.py`
- `tests/constraint-cardinality-test-results.txt`

Research narrative and prior PASS claims were not used as authority.

## Findings

### A18-01 — authoring schema preserves accepted QRC surface

**PASS.**

The authoring schema requires only `type` and `relation`; `qualifier`, `min`, `max`, and `exact` remain optional. Boundless authoring therefore remains structurally valid as required by ADR-0012/0018.

Negative/non-integer bounds are structurally rejected. Unknown fields are rejected.

### A18-02 — normalized schema is canonical for current source Cardinality slice

**PASS.**

The normalized schema requires:

```text
type
relation
direction = source
min
max
```

and permits `max` only as a non-negative integer or `unbounded`. `exact` and arbitrary extra fields are rejected.

This is consistent with ADR-0012 interval normalization and ADR-0017 Core cardinality evidence.

### A18-03 — QRC compatibility entry point is not a second authority

**PASS.**

`qrc-v0.1.schema.json` delegates by `$ref` to the common Cardinality authoring schema rather than duplicating a parallel field contract. QRC remains Cardinality plus optional qualifier.

### A18-04 — semantic validation boundary is preserved

**PASS.**

The structural schema does not attempt to decide:

- empty interval after mixed-bound intersection;
- canonical relation/type identity;
- closed snapshot completeness;
- stable target identity;
- subtype closure;
- QRC qualified-set count.

The minimal normalizer exercises interval normalization and requires canonical qualifier identity to be supplied by the external identity-resolution layer.

### A18-05 — provenance does not contaminate payload equality

**PASS.**

Neither Cardinality payload schema requires an inline provenance field. ADR-0018's requirement that normalized composition evidence remain traceable is therefore left to the containing evidence/package layer rather than incorrectly becoming part of semantic payload equality.

### A18-06 — smoke evidence

**PASS for design-stage schema evidence, with environment note.**

The recorded smoke harness reports 9 PASS / 0 FAIL covering boundless authoring, mixed exact normalization, empty-interval semantic failure, structural bound rejection, normalized required fields, no `exact`, canonical qualifier boundary, QRC `$ref`, and ADR-0017 projection fixture.

Direct repository cloning in the execution container was blocked by DNS resolution. This is an external tooling-connectivity limitation, not a schema/architecture defect. The current design-stage gate does not require remote checkout or backend runtime execution.

### A18-07 — unfinished families remain honestly unimplemented

**PASS.**

No permissive opaque Type/Value/Dimension/Compatibility/Conditional schema was introduced. The repository therefore does not falsely claim complete six-family machine readability.

## Verdict

| Dimension | Verdict |
|---|---|
| Authoring Cardinality schema | PASS |
| Normalized Cardinality schema | PASS |
| QRC compatibility entry point | PASS |
| Semantic/structural boundary | PASS |
| Cardinality normalization smoke evidence | PASS |
| Architecture regression | None |
| Backend/runtime dependency | None |

**Overall: Accept.**

## Next state

ADR-0018 Cardinality/QRC schema slice is complete for design-stage purposes. Operating Desk should next choose a focused Constraint family whose payload can be defined from already accepted semantics without inventing a universal locator/path language. The dependency-preferred next candidate is **Type Constraint**, because subtype closure and endpoint narrowing are already normative and are prerequisites for Interface and domain/profile refinement validation.
