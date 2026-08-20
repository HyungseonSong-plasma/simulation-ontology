# SOL v0.1 Compatibility Constraint Schema Proposal v0.2.1

**Status:** Focused Research revision  
**Date:** 2026-08-20  
**Base:** v0.2 proposal  
**Revision input:** COMP-05 from `docs/validation/sol-v0.1-compatibility-constraint-v0.2-focused-re-review.md`

## 1. Scope

All v0.2 contracts remain unchanged. This revision adds only deterministic aggregation for mixed per-obligation evaluation states.

## 2. Compatibility-set evaluation order

Evaluation proceeds in this order:

1. structural/identity normalization;
2. criterion resolution;
3. obligation-key construction and duplicate/conflict composition;
4. per-obligation semantic evaluation-context resolution;
5. binary evaluation where context is complete;
6. Compatibility-set aggregation.

A normalization or composition conflict is reported before set-level evaluation aggregation.

## 3. Per-obligation state

After successful normalization/composition, each active obligation is exactly one of:

```text
PASS
FAIL
INDETERMINATE
```

- `PASS`: binary compatibility result satisfies `expect`;
- `FAIL`: binary compatibility result contradicts `expect`;
- `INDETERMINATE`: a criterion-required semantic evaluation prerequisite is unresolved and no binary result is produced.

Backend runtime/license/module/release/adapter availability is not a semantic prerequisite and does not create `INDETERMINATE` here.

## 4. Deterministic Compatibility-set aggregation

Because active Compatibility obligations compose conjunctively, aggregate in the following strict precedence:

```text
FAIL > INDETERMINATE > PASS
```

Normatively:

```text
if any obligation == FAIL:
    set verdict = FAIL
else if any obligation == INDETERMINATE:
    set verdict = INDETERMINATE
else:
    set verdict = PASS
```

Therefore an already-false conjunction remains false even when another conjunct cannot yet be evaluated.

## 5. Boundary cases

```text
{PASS, PASS} -> PASS
{PASS, INDETERMINATE} -> INDETERMINATE
{FAIL, INDETERMINATE} -> FAIL
{FAIL, PASS} -> FAIL
```

An empty active Compatibility obligation set is outside this family-specific aggregation question and imposes no Compatibility failure; the surrounding validator may treat it as vacuously satisfied.

## 6. Finding closure

| Finding | Status |
|---|---|
| COMP-01 | Resolved in v0.2 |
| COMP-02 | Resolved in v0.2 |
| COMP-03 | Resolved in v0.2 |
| COMP-04 | Resolved in v0.2 |
| COMP-05 mixed-state aggregation | Resolved in v0.2.1 |

**Contract-level unresolved:** none proposed.  
**Regression:** none proposed.

## 7. Research verdict

**Ready for final focused contract re-review.**
